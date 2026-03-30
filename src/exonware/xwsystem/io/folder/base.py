#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/folder/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.29
Generation Date: 30-Oct-2025
Base classes for folder operations.
"""

from abc import ABC, abstractmethod
from pathlib import Path
from ..contracts import IFolderSource
__all__ = ['AFolderSource']


class AFolderSource(IFolderSource, ABC):
    """Abstract base for folder sources."""

    def __init__(self, path: Path):
        """Initialize folder source."""
        self._path = Path(path)
    @abstractmethod

    def create(self, parents: bool = True, exist_ok: bool = True) -> bool:
        """Create directory."""
        pass
    @abstractmethod

    def delete(self, recursive: bool = False) -> bool:
        """Delete directory."""
        pass
