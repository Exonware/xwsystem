# src/exonware/conf.py
"""
Public-facing configuration for all exonware packages.
This module is self-contained and can be imported without triggering
any library initialization. It provides lazy mode configuration that
works across all exonware packages (xwsystem, xwnode, xwdata, etc.).
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.42
Generation Date: 11-Nov-2025
"""

from __future__ import annotations

import io
import sys

# Windows: prefer UTF-8 for console I/O so emojis and non-ASCII data round-trip.
# WHY: default console encodings (e.g. cp1252) cannot represent full Unicode.
if sys.platform == "win32":
    try:
        if hasattr(sys.stdout, "reconfigure"):
            try:
                sys.stdout.reconfigure(encoding="utf-8")
            except (ValueError, OSError):
                pass
        if hasattr(sys.stderr, "reconfigure"):
            try:
                sys.stderr.reconfigure(encoding="utf-8")
            except (ValueError, OSError):
                pass
    except Exception:
        try:
            if hasattr(sys.stdout, "buffer"):
                sys.stdout = io.TextIOWrapper(
                    sys.stdout.buffer,
                    encoding="utf-8",
                    errors="replace",
                    line_buffering=True,
                )
            if hasattr(sys.stderr, "buffer"):
                sys.stderr = io.TextIOWrapper(
                    sys.stderr.buffer,
                    encoding="utf-8",
                    errors="replace",
                    line_buffering=True,
                )
        except Exception:
            pass
