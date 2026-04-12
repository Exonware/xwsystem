"""
#exonware/xwsystem/src/exonware/xwsystem/contracts.py
High-level, cross-cutting interfaces for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.39
Last Updated: 29-Jan-2026
MANDATORY (GUIDE_31_DEV.md): interfaces in `contracts.py` MUST use `Protocol`.
"""

from __future__ import annotations
from typing import Protocol, runtime_checkable
@runtime_checkable

class IXWFacade(Protocol):
    """Marker interface for public xwsystem facades."""
__all__ = [
    "IXWFacade",
]
