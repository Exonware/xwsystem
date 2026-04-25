#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/security/kms.py

Lightweight Key-Management-Service abstraction.

Defines an :class:`IKMS` protocol that pluggable adapters implement, a
production-useful :class:`LocalFileKMS` that stores keys as files under
a directory root, and a :class:`RotationPolicy` + :func:`evaluate_rotation`
helper that tells the host when a key is due for rotation.

The host (hive-api, CI jobs) is responsible for wiring this to its
actual encryption backends — this module is the contract + a workable
default.
"""

from __future__ import annotations

import os
import secrets
import shutil
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Iterable, Protocol, runtime_checkable


@dataclass(slots=True)
class KeyMaterial:
    """Opaque key payload + bookkeeping."""

    id: str
    alias: str
    created_ts: float
    rotated_ts: float | None = None
    revoked_ts: float | None = None
    version: int = 1
    algo: str = "HIVE-HSK"
    size_bits: int = 4096
    material: bytes = b""
    meta: dict[str, Any] = field(default_factory=dict)

    def is_active(self) -> bool:
        return self.revoked_ts is None

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "alias": self.alias,
            "created_ts": self.created_ts,
            "rotated_ts": self.rotated_ts,
            "revoked_ts": self.revoked_ts,
            "version": self.version,
            "algo": self.algo,
            "size_bits": self.size_bits,
            "active": self.is_active(),
            "meta": self.meta,
            "has_material": bool(self.material),
        }


@runtime_checkable
class IKMS(Protocol):
    """Minimum surface every adapter must provide."""

    def list_keys(self) -> list[KeyMaterial]: ...
    def get(self, alias: str) -> KeyMaterial | None: ...
    def put(self, key: KeyMaterial) -> KeyMaterial: ...
    def delete(self, alias: str) -> bool: ...


class LocalFileKMS:
    """Directory-backed KMS.

    Stores one ``<alias>.key`` binary per alias and a sibling ``<alias>.json``
    with the metadata. Atomic by way of ``os.replace``.
    """

    def __init__(self, root: str | Path) -> None:
        self._root = Path(root)
        self._root.mkdir(parents=True, exist_ok=True)

    def list_keys(self) -> list[KeyMaterial]:
        out: list[KeyMaterial] = []
        for meta_path in sorted(self._root.glob("*.json")):
            key = self._load_meta(meta_path.stem)
            if key is not None:
                out.append(key)
        return out

    def get(self, alias: str) -> KeyMaterial | None:
        return self._load_meta(alias)

    def put(self, key: KeyMaterial) -> KeyMaterial:
        import json as _json
        tmp = self._root / f"{key.alias}.json.tmp"
        final_meta = self._root / f"{key.alias}.json"
        payload = key.to_dict()
        payload["has_material"] = bool(key.material)
        tmp.write_text(_json.dumps(payload, indent=2), encoding="utf-8")
        os.replace(tmp, final_meta)
        if key.material:
            tmp_key = self._root / f"{key.alias}.key.tmp"
            tmp_key.write_bytes(key.material)
            os.replace(tmp_key, self._root / f"{key.alias}.key")
        return key

    def delete(self, alias: str) -> bool:
        removed = False
        for suffix in (".json", ".key"):
            target = self._root / f"{alias}{suffix}"
            if target.exists():
                target.unlink()
                removed = True
        return removed

    # ---- private -------------------------------------------------------

    def _load_meta(self, alias: str) -> KeyMaterial | None:
        import json as _json
        meta_path = self._root / f"{alias}.json"
        if not meta_path.exists():
            return None
        try:
            raw = _json.loads(meta_path.read_text(encoding="utf-8"))
        except Exception:
            return None
        key_path = self._root / f"{alias}.key"
        material = key_path.read_bytes() if key_path.exists() else b""
        return KeyMaterial(
            id=str(raw.get("id", alias)),
            alias=str(raw.get("alias", alias)),
            created_ts=float(raw.get("created_ts", 0.0)),
            rotated_ts=raw.get("rotated_ts"),
            revoked_ts=raw.get("revoked_ts"),
            version=int(raw.get("version", 1)),
            algo=str(raw.get("algo", "HIVE-HSK")),
            size_bits=int(raw.get("size_bits", 4096)),
            material=material,
            meta=dict(raw.get("meta") or {}),
        )


# ---------------------------------------------------------------------------
# Rotation helpers
# ---------------------------------------------------------------------------


@dataclass(slots=True)
class RotationPolicy:
    """Declarative rotation policy — all thresholds optional."""

    max_age_days: int | None = None
    max_usage_count: int | None = None
    risk_rotate_on_events: tuple[str, ...] = ()


@dataclass(slots=True)
class RotationDecision:
    action: str         # 'none' | 'rotate' | 'emergency'
    reason: str
    age_days: float = 0.0
    usage_count: int = 0


def evaluate_rotation(
    *,
    key: KeyMaterial,
    policy: RotationPolicy,
    usage_count: int = 0,
    risk_event_kinds: Iterable[str] = (),
    now_ts: float | None = None,
) -> RotationDecision:
    now = now_ts if now_ts is not None else time.time()
    reference = key.rotated_ts or key.created_ts or now
    age_days = max(0.0, (now - reference) / 86400.0)

    if any(kind in risk_event_kinds for kind in policy.risk_rotate_on_events):
        return RotationDecision(
            action="emergency",
            reason="risk-triggered rotation",
            age_days=age_days, usage_count=usage_count,
        )
    if policy.max_usage_count is not None and usage_count >= policy.max_usage_count:
        return RotationDecision(
            action="rotate",
            reason=f"usage count {usage_count} >= {policy.max_usage_count}",
            age_days=age_days, usage_count=usage_count,
        )
    if policy.max_age_days is not None and age_days >= policy.max_age_days:
        return RotationDecision(
            action="rotate",
            reason=f"age {age_days:.1f}d >= {policy.max_age_days}d",
            age_days=age_days, usage_count=usage_count,
        )
    return RotationDecision(action="none", reason="within policy", age_days=age_days, usage_count=usage_count)


def new_random_material(bits: int = 4096) -> bytes:
    """Return ``bits``/8 random bytes — suitable for high-entropy seeds."""
    return secrets.token_bytes(max(8, bits // 8))


__all__ = [
    "IKMS", "KeyMaterial", "LocalFileKMS",
    "RotationPolicy", "RotationDecision", "evaluate_rotation",
    "new_random_material",
]
