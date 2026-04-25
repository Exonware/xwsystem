#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/notifications/facade.py

Core notifier abstractions.
"""

from __future__ import annotations

import json
import threading
import time
from collections.abc import Iterable
from dataclasses import dataclass, field
from typing import Any, Literal, Protocol, runtime_checkable


Severity = Literal["info", "warning", "critical", "success"]


@dataclass(slots=True)
class Notification:
    channel: str
    severity: Severity
    subject: str
    body: str
    ts: float = 0.0
    meta: dict[str, Any] = field(default_factory=dict)
    delivered: bool = True
    error: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return {
            "channel": self.channel,
            "severity": self.severity,
            "subject": self.subject,
            "body": self.body,
            "ts": self.ts,
            "meta": self.meta,
            "delivered": self.delivered,
            "error": self.error,
        }


@runtime_checkable
class INotificationChannel(Protocol):
    """Each channel is responsible for actual delivery."""

    name: str

    def send(self, n: Notification) -> Notification: ...


class InMemoryChannel:
    """Keeps the last `capacity` notifications in memory. Great for tests
    and for a "recent notifications" UI panel."""

    def __init__(self, name: str = "memory", capacity: int = 500) -> None:
        self.name = name
        self._capacity = max(1, capacity)
        self._items: list[Notification] = []
        self._lock = threading.Lock()

    def send(self, n: Notification) -> Notification:
        n.channel = self.name
        n.ts = n.ts or time.time()
        n.delivered = True
        with self._lock:
            self._items.append(n)
            if len(self._items) > self._capacity:
                self._items = self._items[-self._capacity:]
        return n

    def all(self) -> list[Notification]:
        with self._lock:
            return list(self._items)


class WebhookChannel:
    """HTTP-POST-JSON channel — works with Slack incoming webhooks,
    Discord, Teams, or any ``application/json`` endpoint.

    Uses :mod:`urllib.request` so we stay dependency-free.
    """

    def __init__(self, name: str, url: str, *, timeout: float = 5.0,
                 default_headers: dict[str, str] | None = None) -> None:
        self.name = name
        self._url = url
        self._timeout = timeout
        self._headers = {"Content-Type": "application/json", **(default_headers or {})}

    def send(self, n: Notification) -> Notification:
        import urllib.error
        import urllib.request

        n.channel = self.name
        n.ts = n.ts or time.time()
        payload = json.dumps({
            "severity": n.severity,
            "subject": n.subject,
            "body": n.body,
            "meta": n.meta,
            "ts": n.ts,
        }).encode("utf-8")
        req = urllib.request.Request(self._url, data=payload, method="POST", headers=self._headers)
        try:
            with urllib.request.urlopen(req, timeout=self._timeout) as resp:
                code = resp.status
                if code >= 300:
                    n.delivered = False
                    n.error = f"HTTP {code}"
                else:
                    n.delivered = True
        except urllib.error.URLError as exc:
            n.delivered = False
            n.error = str(exc)
        return n


class Notifier:
    """Fan-out multiplexer. ``notify()`` delivers to every registered
    channel, capturing per-channel success/error."""

    def __init__(self) -> None:
        self._channels: dict[str, INotificationChannel] = {}

    def add(self, channel: INotificationChannel) -> None:
        self._channels[channel.name] = channel

    def remove(self, name: str) -> None:
        self._channels.pop(name, None)

    def channels(self) -> list[str]:
        return sorted(self._channels)

    def notify(
        self,
        subject: str,
        body: str,
        *,
        severity: Severity = "info",
        meta: dict[str, Any] | None = None,
        only: Iterable[str] | None = None,
    ) -> list[Notification]:
        results: list[Notification] = []
        targets = only or self._channels.keys()
        for name in targets:
            channel = self._channels.get(name)
            if channel is None:
                continue
            n = Notification(
                channel=channel.name, severity=severity,
                subject=subject, body=body, meta=dict(meta or {}),
            )
            try:
                results.append(channel.send(n))
            except Exception as exc:  # noqa: BLE001
                n.delivered = False
                n.error = str(exc)
                results.append(n)
        return results
