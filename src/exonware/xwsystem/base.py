"""
#exonware/xwsystem/src/exonware/xwsystem/base.py
High-level abstract base classes for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.35
Last Updated: 29-Jan-2026
MANDATORY (GUIDE_31_DEV.md): abstract classes in `base.py` MUST start with `A`
and MUST use `ABC`.
"""

from __future__ import annotations
from abc import ABC
from .contracts import IXWFacade


class AXWFacade(ABC, IXWFacade):
    """Abstract base for xwsystem facades."""
__all__ = [
    "AXWFacade",
]
