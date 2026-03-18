"""
#exonware/xwsystem/src/exonware/xwsystem/data_structures/__init__.py
Generic Data Structures for xwsystem.
Provides reusable data structures that can be used by any library:
- TrieNode: Internal node for Trie structures
- UnionFind: Union-Find (Disjoint Set) data structure
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.12
Generation Date: 26-Jan-2025
"""

from .trie import TrieNode
from .union_find import UnionFind
__all__ = [
    'TrieNode',
    'UnionFind',
]
