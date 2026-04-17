#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/data_structures/union_find.py
Union-Find (Disjoint Set) Data Structure for xwsystem.
Generic Union-Find implementation that can be used by any library.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.40
Generation Date: 26-Jan-2025
"""

from typing import Any


class UnionFind:
    """
    Union-Find (Disjoint Set) data structure.
    Generic implementation that can be used by any library.
    Time Complexity:
    - make_set: O(1)
    - find: α(n) ≈ O(1) with path compression
    - union: α(n) ≈ O(1) with union by rank
    - connected: α(n) ≈ O(1)
    """

    def __init__(self):
        """Initialize Union-Find structure."""
        self._parent: dict[Any, Any] = {}
        self._rank: dict[Any, int] = {}
        self._sets_count = 0

    def make_set(self, x: Any) -> None:
        """
        Create new set with element x.
        Time Complexity: O(1)
        Args:
            x: Element to add
        """
        if x not in self._parent:
            self._parent[x] = x
            self._rank[x] = 0
            self._sets_count += 1

    def find(self, x: Any) -> Any:
        """
        Find root of set containing x with path compression.
        Time Complexity: α(n) ≈ O(1) (inverse Ackermann function)
        Args:
            x: Element to find
        Returns:
            Root of set containing x
        Raises:
            ValueError: If element not found
        """
        if x not in self._parent:
            raise ValueError(f"Element {x} not found in union-find structure")
        # Path compression
        if self._parent[x] != x:
            self._parent[x] = self.find(self._parent[x])
        return self._parent[x]

    def union(self, x: Any, y: Any) -> None:
        """
        Union sets containing x and y by rank.
        Time Complexity: α(n) ≈ O(1)
        Args:
            x: First element
            y: Second element
        """
        # Ensure both elements exist
        self.make_set(x)
        self.make_set(y)
        root_x = self.find(x)
        root_y = self.find(y)
        if root_x == root_y:
            return  # Already in same set
        # Union by rank
        if self._rank[root_x] < self._rank[root_y]:
            root_x, root_y = root_y, root_x
        self._parent[root_y] = root_x
        if self._rank[root_x] == self._rank[root_y]:
            self._rank[root_x] += 1
        self._sets_count -= 1

    def connected(self, x: Any, y: Any) -> bool:
        """
        Check if x and y are in same set.
        Time Complexity: α(n) ≈ O(1)
        Args:
            x: First element
            y: Second element
        Returns:
            True if x and y are in the same set
        """
        try:
            return self.find(x) == self.find(y)
        except ValueError:
            return False

    def size(self) -> int:
        """
        Get number of elements.
        Time Complexity: O(1)
        Returns:
            Number of elements
        """
        return len(self._parent)

    def sets_count(self) -> int:
        """
        Get number of disjoint sets.
        Time Complexity: O(1)
        Returns:
            Number of disjoint sets
        """
        return self._sets_count
