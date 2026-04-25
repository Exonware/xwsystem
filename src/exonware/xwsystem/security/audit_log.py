#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/security/audit_log.py

Append-only event audit log with optional HMAC signing.

Distinct from :mod:`xwsystem.security.audit` (which scans *code* for
security issues) — this module is a runtime **event log**. Every record
is a small JSON object ``{ts, actor, kind, resource, action, ok, ...}``
written to a newline-delimited file. When a signing secret is provided
each line carries an HMAC so tampering is detectable.

Used by higher layers (hive-api, approval inbox, sessions, key vault)
to get consistent audit behaviour without each re-inventing the file
format.
"""

from __future__ import annotations

import hmac
import json
import os
import threading
import time
import hashlib
from collections.abc import Callable, Iterable
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


AuditListener = Callable[["AuditEvent"], None]


@dataclass(slots=True)
class AuditEvent:
    ts: float
    actor: str
    kind: str          # e.g. "auth.login", "resource.api-keys", "query.run"
    resource: str = ""
    action: str = ""   # e.g. "create" / "update" / "delete" / "run"
    ok: bool = True
    meta: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        return {
            "ts": self.ts,
            "actor": self.actor,
            "kind": self.kind,
            "resource": self.resource,
            "action": self.action,
            "ok": self.ok,
            "meta": self.meta,
        }


class AuditLog:
    """Append-only NDJSON log with optional HMAC signatures.

    Instances are process-local and thread-safe. Pass a ``signing_secret``
    to enable line-level HMAC so :meth:`verify` can flag tampering.
    """

    def __init__(
        self,
        path: str | Path,
        *,
        signing_secret: str | bytes | None = None,
        max_bytes: int | None = None,
    ) -> None:
        self._path = Path(path)
        self._path.parent.mkdir(parents=True, exist_ok=True)
        if isinstance(signing_secret, str):
            signing_secret = signing_secret.encode()
        self._secret: bytes | None = signing_secret
        self._max_bytes = max_bytes
        self._lock = threading.Lock()
        self._listeners: list[AuditListener] = []

    # ---- configuration -------------------------------------------------

    def add_listener(self, fn: AuditListener) -> Callable[[], None]:
        self._listeners.append(fn)
        return lambda: self._listeners.remove(fn)

    @property
    def path(self) -> Path:
        return self._path

    # ---- writes --------------------------------------------------------

    def append(
        self,
        *,
        actor: str,
        kind: str,
        resource: str = "",
        action: str = "",
        ok: bool = True,
        meta: dict[str, Any] | None = None,
    ) -> AuditEvent:
        event = AuditEvent(
            ts=time.time(), actor=actor, kind=kind, resource=resource,
            action=action, ok=ok, meta=meta or {},
        )
        line = self._serialise(event)
        with self._lock:
            if self._max_bytes is not None and self._path.exists() and self._path.stat().st_size >= self._max_bytes:
                rotated = self._path.with_suffix(self._path.suffix + f".{int(time.time())}.bak")
                os.replace(self._path, rotated)
            with self._path.open("a", encoding="utf-8") as fh:
                fh.write(line + "\n")
        for listener in list(self._listeners):
            try:
                listener(event)
            except Exception:
                pass
        return event

    # ---- reads ---------------------------------------------------------

    def all(self, *, limit: int | None = None) -> list[AuditEvent]:
        out: list[AuditEvent] = []
        if not self._path.exists():
            return out
        with self._path.open("r", encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line:
                    continue
                try:
                    obj = json.loads(line)
                except json.JSONDecodeError:
                    continue
                out.append(_from_wire(obj))
        if limit is not None:
            out = out[-limit:]
        return out

    def search(
        self,
        *,
        actor: str | None = None,
        kind: str | None = None,
        resource: str | None = None,
        action: str | None = None,
        since_ts: float | None = None,
        until_ts: float | None = None,
        limit: int | None = None,
    ) -> list[AuditEvent]:
        results: list[AuditEvent] = []
        for ev in self.all():
            if actor is not None and ev.actor != actor:
                continue
            if kind is not None and not ev.kind.startswith(kind):
                continue
            if resource is not None and ev.resource != resource:
                continue
            if action is not None and ev.action != action:
                continue
            if since_ts is not None and ev.ts < since_ts:
                continue
            if until_ts is not None and ev.ts > until_ts:
                continue
            results.append(ev)
        if limit is not None:
            results = results[-limit:]
        return results

    # ---- integrity -----------------------------------------------------

    def verify(self) -> tuple[bool, list[int]]:
        """Return ``(all_ok, bad_line_numbers)``.

        Valid when every signed line round-trips to the same HMAC. If the
        log was written without a secret, verify always returns ``(True, [])``.
        """
        if self._secret is None or not self._path.exists():
            return True, []
        bad: list[int] = []
        with self._path.open("r", encoding="utf-8") as fh:
            for idx, raw in enumerate(fh, start=1):
                raw = raw.strip()
                if not raw:
                    continue
                try:
                    obj = json.loads(raw)
                except json.JSONDecodeError:
                    bad.append(idx)
                    continue
                sig = obj.pop("_sig", None)
                payload = _canonical_payload(obj)
                expected = hmac.new(self._secret, payload.encode(), hashlib.sha256).hexdigest()
                if sig != expected:
                    bad.append(idx)
        return len(bad) == 0, bad

    # ---- internals -----------------------------------------------------

    def _serialise(self, event: AuditEvent) -> str:
        obj = event.to_dict()
        if self._secret is not None:
            payload = _canonical_payload(obj)
            obj["_sig"] = hmac.new(self._secret, payload.encode(), hashlib.sha256).hexdigest()
        return json.dumps(obj, separators=(",", ":"), sort_keys=True)


def _canonical_payload(obj: dict[str, Any]) -> str:
    """Deterministic string for signing — drop ``_sig`` + sort keys."""
    data = {k: v for k, v in obj.items() if k != "_sig"}
    return json.dumps(data, separators=(",", ":"), sort_keys=True)


def _from_wire(obj: dict[str, Any]) -> AuditEvent:
    return AuditEvent(
        ts=float(obj.get("ts", 0.0)),
        actor=str(obj.get("actor", "")),
        kind=str(obj.get("kind", "")),
        resource=str(obj.get("resource", "")),
        action=str(obj.get("action", "")),
        ok=bool(obj.get("ok", True)),
        meta=dict(obj.get("meta") or {}),
    )


__all__ = ["AuditEvent", "AuditLog"]
