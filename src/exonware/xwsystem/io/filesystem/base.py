#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/filesystem/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.12
Generation Date: 30-Oct-2025
Base classes for virtual filesystem.
"""

from abc import ABC, abstractmethod
from ..contracts import IVirtualFS
__all__ = ['AFileSystem']


class AFileSystem(IVirtualFS, ABC):
    """Abstract base for filesystem implementations."""
    @property
    @abstractmethod

    def scheme(self) -> str:
        """URI scheme."""
        pass
