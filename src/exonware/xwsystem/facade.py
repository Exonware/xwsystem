"""
#exonware/xwsystem/src/exonware/xwsystem/facade.py
Package-level facade(s) for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.35
Last Updated: 29-Jan-2026
This module provides a stable, high-level entry point for composing major
subsystem facades (IO, caching, security, etc.).
"""

from __future__ import annotations
from dataclasses import dataclass
from .base import AXWFacade
from .caching.facade import XWCache
from .http_client import AsyncHttpClient, HttpClient
from .io.facade import XWIO
from .monitoring.facade import XWMonitor
from .security.facade import XWSecurity
from .threading.facade import XWConcurrency
from .validation.facade import XWValidator
@dataclass(frozen=True, slots=True)

class XWSystem(AXWFacade):
    """Top-level facade that bundles major subsystem facades."""
    io: XWIO
    caching: XWCache
    security: XWSecurity
    monitoring: XWMonitor
    threading: XWConcurrency
    validation: XWValidator
    http: HttpClient
    http_async: AsyncHttpClient
    @classmethod

    def create(cls) -> XWSystem:
        """Create a default `XWSystem` facade with sane defaults."""
        return cls(
            io=XWIO(),
            caching=XWCache(),
            security=XWSecurity(),
            monitoring=XWMonitor(),
            threading=XWConcurrency(),
            validation=XWValidator(),
            http=HttpClient(),
            http_async=AsyncHttpClient(),
        )
__all__ = [
    "XWSystem",
]
