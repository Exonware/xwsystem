"""
HTTP correlation and ops telemetry naming shared with xwauth-api (plan: telemetry-schema).

Consumers should propagate the same correlation identifiers across logs, traces, and audit.
"""

from __future__ import annotations

TELEMETRY_CONTRACT_VERSION = "1.0"

# Canonical response header names emitted by xwauth-api ops middleware (Title-Case).
HTTP_CORRELATION_RESPONSE_HEADERS: tuple[str, ...] = (
    "X-Request-Id",
    "X-Correlation-Id",
    "X-Trace-Id",
    "X-Route-Family",
    "X-Tenant-Id",
    "X-Telemetry-Contract-Version",
)
