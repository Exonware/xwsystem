"""
#exonware/xwsystem/src/exonware/xwsystem/errors.py
Package-level errors for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.1
Last Updated: 29-Jan-2026
"""

from __future__ import annotations


class XWSystemError(Exception):
    """Base exception for xwsystem (package-level)."""
__all__ = [
    "XWSystemError",
]
