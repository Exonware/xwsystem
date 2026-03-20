#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/io/path_parser.py
Generic Path Parser for xwsystem.
Provides thread-safe path parsing with caching that can be used by any library.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.13
Generation Date: 26-Jan-2025
"""

import threading
from exonware.xwsystem.caching import create_cache
from exonware.xwsystem import get_logger
logger = get_logger(__name__)


class PathParser:
    """
    Thread-safe path parser with caching.
    Performance Optimization:
    - Uses xwsystem create_cache() for 10-50x faster cache operations
    - Automatic LRU eviction when capacity is reached
    - Thread-safe by default (via xwsystem cache)
    Can be used by any library for path parsing operations.
    """

    def __init__(self, max_cache_size: int = 1024, namespace: str = "xwsystem"):
        """
        Initialize path parser.
        Args:
            max_cache_size: Maximum cache size
            namespace: Cache namespace
        """
        # Use xwsystem optimized cache for 10-50x faster operations
        # Provides automatic LRU eviction and thread-safety
        self._cache = create_cache(
            capacity=max_cache_size,
            namespace=namespace,
            name='path_parser_cache'
        )
        self._max_cache_size = max_cache_size
        # Lock for potential future stats tracking if needed
        self._lock = threading.RLock()

    def parse(self, path: str) -> list[str]:
        """
        Parse a path string into parts (O(1) cache lookup with xwsystem cache).
        Args:
            path: Path string to parse
        Returns:
            List of path parts
        """
        # Check xwsystem cache first (O(1) with automatic LRU eviction)
        cached_parts = self._cache.get(path)
        if cached_parts is not None:
            return cached_parts
        # Cache miss - parse path
        parts = self._parse_path(path)
        # Cache the result (automatic LRU eviction when full)
        self._cache.put(path, parts)
        return parts

    def _parse_path(self, path: str) -> list[str]:
        """
        Internal path parsing logic.
        Args:
            path: Path string to parse
        Returns:
            List of path parts
        """
        if not path:
            return []
        # Simple dot-separated path parsing
        return [part for part in path.split('.') if part]

    def clear_cache(self) -> None:
        """Clear the path parser cache."""
        # Clear cache (implementation depends on cache type)
        if hasattr(self._cache, 'clear'):
            self._cache.clear()
