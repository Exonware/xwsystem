#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/io/folder/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.1.0.10
Generation Date: 30-Oct-2025
Folder-specific implementations.
Following codec/ pattern.
"""

from ..contracts import IFolderSource
from ..defs import TraversalMode
from .base import AFolderSource
from ..errors import FolderError
from .folder import XWFolder
__all__ = [
    "IFolderSource",
    "TraversalMode",
    "AFolderSource",
    "FolderError",
    "XWFolder",
]
