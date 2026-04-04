"""
Outbound HTTP timeout defaults for auth and federation clients (plan: resilience-patterns).

Override with environment variables where the process runs (operators align with ops tier profile).
"""

from __future__ import annotations

import os


def outbound_http_timeout_tuple(
    *,
    default_connect: float = 5.0,
    default_read: float = 30.0,
) -> tuple[float, float]:
    """
    Return (connect_timeout, read_timeout) for ``requests`` / compatible clients.

    Env:
        XWAUTH_OUTBOUND_HTTP_CONNECT_TIMEOUT — seconds (default ``default_connect``)
        XWAUTH_OUTBOUND_HTTP_READ_TIMEOUT — seconds (default ``default_read``)
    """
    try:
        connect = float(os.environ.get("XWAUTH_OUTBOUND_HTTP_CONNECT_TIMEOUT", str(default_connect)))
    except ValueError:
        connect = default_connect
    try:
        read = float(os.environ.get("XWAUTH_OUTBOUND_HTTP_READ_TIMEOUT", str(default_read)))
    except ValueError:
        read = default_read
    return (max(0.1, connect), max(0.1, read))
