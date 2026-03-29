#exonware/xwsystem/src/exonware/xwsystem/io/indexing/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.23
Generation Date: January 2026
Indexing module - Unified indexing facade for line-oriented files and
pluggable general-purpose index backends (key-value, B-tree–like, full-text).
BTreeIndexBackend: ordered keys and range_scan. FullTextIndexBackend: term index (index/search/search_any). See REF_15 (Indexing module).
"""

from .facade import XWIndex
from .backend import (
    IIndexBackend,
    InMemoryIndexBackend,
    FileBackedIndexBackend,
    BTreeIndexBackend,
    FullTextIndexBackend,
    FileBackedFullTextIndexBackend,
)
__all__ = [
    "XWIndex",
    "IIndexBackend",
    "InMemoryIndexBackend",
    "FileBackedIndexBackend",
    "BTreeIndexBackend",
    "FullTextIndexBackend",
    "FileBackedFullTextIndexBackend",
]
