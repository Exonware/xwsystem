"""
#exonware/xwsystem/src/exonware/xwsystem/config.py
Package-level configuration entry points for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.14
Last Updated: 29-Jan-2026
This module exists to satisfy the canonical structure in GUIDE_31_DEV.md:
- `config.py` at package root for configuration surfaces.
The concrete configuration system lives under `exonware.xwsystem.config`.
"""

from __future__ import annotations
from .config.logging_setup import get_logger, setup_logging
__all__ = [
    "get_logger",
    "setup_logging",
]
