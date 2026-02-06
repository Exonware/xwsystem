#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/data_structures/trie.py

Trie Node for xwsystem.

Generic Trie node implementation that can be used by any library.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.5
Generation Date: 26-Jan-2025
"""

from typing import Any, Dict


class TrieNode:
    """
    Internal node for Trie structure.
    
    Generic implementation that can be used by any library.
    """
    
    def __init__(self):
        """Initialize Trie node."""
        self.children: Dict[str, TrieNode] = {}
        self.is_end_word: bool = False
        self.value: Any = None
    
    def __repr__(self) -> str:
        """String representation of Trie node."""
        return f"TrieNode(children={len(self.children)}, is_end={self.is_end_word})"
