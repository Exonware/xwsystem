#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/cli/encoding.py
"""
Console encoding utilities.
Best-effort UTF-8 configuration for Windows consoles (safe for uvicorn logging).
No-op on non-Windows platforms.
"""

from __future__ import annotations
import io
import sys


def ensure_utf8_console() -> None:
    """
    Configure UTF-8 encoding for Windows console output streams.
    - No-op on non-Windows platforms.
    - Best-effort: never raises.
    """
    if sys.platform != "win32":
        return
    try:
        # Prefer reconfigure when available (Python 3.7+)
        if hasattr(sys.stdout, "reconfigure"):
            try:
                sys.stdout.reconfigure(encoding="utf-8", errors="replace")
                sys.stderr.reconfigure(encoding="utf-8", errors="replace")
                return
            except (AttributeError, OSError):
                # Fall back to wrapper below
                pass
        class _UTF8Wrapper(io.TextIOWrapper):
            """Wrapper that delegates key methods to the original stream."""
            def __init__(self, buffer, **kwargs):
                super().__init__(buffer, **kwargs)
                self._original = buffer
            def isatty(self):
                return getattr(self._original, "isatty", lambda: False)()
            def fileno(self):
                return getattr(self._original, "fileno", lambda: -1)()
        sys.stdout = _UTF8Wrapper(
            sys.stdout.buffer,
            encoding="utf-8",
            errors="replace",
            line_buffering=True,
        )
        sys.stderr = _UTF8Wrapper(
            sys.stderr.buffer,
            encoding="utf-8",
            errors="replace",
            line_buffering=True,
        )
    except Exception:
        # Fallback on error
        pass
__all__ = [
    "ensure_utf8_console",
]
