#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/data_structures/trie.py
Trie Node for xwsystem.
Generic Trie node implementation that can be used by any library.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.20
Generation Date: 26-Jan-2025
"""

from typing import Any


class TrieNode:
    """
    Internal node for Trie structure.
    Generic implementation that can be used by any library.
    """

    def __init__(self):
        """Initialize Trie node."""
        self.children: dict[str, TrieNode] = {}
        self.is_end_word: bool = False
        self.value: Any = None

    def __repr__(self) -> str:
        """String representation of Trie node."""
        return f"TrieNode(children={len(self.children)}, is_end={self.is_end_word})"
