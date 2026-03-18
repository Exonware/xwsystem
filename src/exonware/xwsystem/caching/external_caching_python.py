#exonware/xwsystem/src/exonware/xwsystem/caching/external_caching_python.py
"""
External Python caching library implementations.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.11
Generation Date: 01-Nov-2025
Wrappers for external caching libraries:
- cachebox: Rust-based Python cache (fastest)
- functools.lru_cache: Standard library LRU (C implementation)
- cachetools: Flexible caching with TTL, LFU, RR policies
"""

import time
from typing import Any

from collections.abc import Hashable
from functools import lru_cache as std_lru_cache
from collections import OrderedDict
from .base import ACache
from .contracts import ICache
# Optional dependencies: cachebox and cachetools
import importlib.util
# Check for cachebox
_cachebox_spec = importlib.util.find_spec('cachebox')
if _cachebox_spec is not None:
    import cachebox
    HAS_CACHEBOX = True
else:
    HAS_CACHEBOX = False
    cachebox = None
# Check for cachetools
_cachetools_spec = importlib.util.find_spec('cachetools')
if _cachetools_spec is not None:
    import cachetools
    from cachetools import (
        LRUCache as CachetoolsLRU,
        LFUCache as CachetoolsLFU,
        TTLCache as CachetoolsTTL,
        RRCache as CachetoolsRR,
    )
    HAS_CACHETOOLS = True
else:
    HAS_CACHETOOLS = False
    cachetools = None
    CachetoolsLRU = None
    CachetoolsLFU = None
    CachetoolsTTL = None
    CachetoolsRR = None
# Check for pylru (default cache - highest throughput in benchmarks)
_pylru_spec = importlib.util.find_spec('pylru')
if _pylru_spec is not None:
    import pylru
    HAS_PYLRU = True
else:
    HAS_PYLRU = False
    pylru = None
# ============================================================================
# CACHEBOX IMPLEMENTATION
# ============================================================================


class CacheboxCache(ACache):
    """
    Cachebox wrapper - Rust-based high-performance cache.
    This is a Python wrapper around Rust's caching primitives,
    providing excellent performance for large datasets.
    """

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """
        Initialize Cachebox cache.
        Args:
            capacity: Maximum cache size
            ttl: Time to live in seconds (not directly supported by cachebox)
        """
        if not HAS_CACHEBOX:
            raise ImportError(
                "cachebox not available. Install with: pip install cachebox"
            )
        super().__init__(capacity=capacity, ttl=int(ttl) if ttl else None)
        # cachebox uses maxsize parameter
        self._cache = cachebox.Cache(maxsize=capacity)
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        try:
            value = self._cache.get(key, default)
            if value is not default:
                self._hits += 1
            else:
                self._misses += 1
            return value
        except Exception:
            self._misses += 1
            return default

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        # Check if we need to evict (cachebox handles this automatically)
        if len(self._cache) >= self.capacity and key not in self._cache:
            self._evictions += 1
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict entries (handled automatically by cachebox)."""
        # cachebox handles eviction automatically
        pass

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys())

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values())

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items())

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
        }
    # ICache interface compatibility

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache
# ============================================================================
# FUNCTOOLS LRU_CACHE IMPLEMENTATION
# ============================================================================


class FunctoolsLRUCache(ACache):
    """
    functools.lru_cache wrapper - Standard library LRU cache.
    This is implemented in C (CPython core), making it extremely fast
    for simple function memoization. However, it's a decorator, so we
    implement a manual LRU cache that mimics its behavior.
    """

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """
        Initialize functools-style LRU cache.
        Args:
            capacity: Maximum cache size
            ttl: Time to live in seconds (not supported by functools)
        """
        super().__init__(capacity=capacity, ttl=int(ttl) if ttl else None)
        # Use OrderedDict for LRU behavior
        self._cache: OrderedDict[Hashable, Any] = OrderedDict()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        if key in self._cache:
            # Move to end (most recently used)
            value = self._cache.pop(key)
            self._cache[key] = value
            self._hits += 1
            return value
        self._misses += 1
        return default

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        if key in self._cache:
            # Update existing - move to end
            self._cache.pop(key)
        else:
            # Ensure capacity is int (handle case where it might be string from config)
            capacity = int(self.capacity) if isinstance(self.capacity, str) else self.capacity
            if len(self._cache) >= capacity:
                # Evict least recently used (first item)
                if self._cache:
                    self._cache.popitem(last=False)  # Remove first (oldest)
                self._evictions += 1
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        if key in self._cache:
            del self._cache[key]
            return True
        return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict least recently used entry."""
        if self._cache:
            self._cache.popitem(last=False)
            self._evictions += 1

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys())

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values())

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items())

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
        }
    # ICache interface compatibility

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache
# ============================================================================
# PYLRU IMPLEMENTATION (default - highest throughput in benchmarks)
# ============================================================================


class PylruCache(ACache):
    """
    pylru wrapper - External LRU cache with highest throughput in benchmarks.
    Benchmarks: ~2.7M get/s, ~2.4M put/s, ~2.4M mixed/s (vs FunctoolsLRUCache ~1.9M).
    Used as the default cache when pylru is installed.
    """

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """
        Initialize pylru cache.
        Args:
            capacity: Maximum cache size
            ttl: Time to live in seconds (not supported by pylru)
        """
        if not HAS_PYLRU:
            raise ImportError(
                "pylru not available. Install with: pip install pylru"
            )
        super().__init__(capacity=capacity, ttl=int(ttl) if ttl else None)
        self._cache = pylru.lrucache(int(capacity) if isinstance(capacity, str) else capacity)
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        try:
            value = self._cache.get(key, default)
            if value is not default:
                self._hits += 1
            else:
                self._misses += 1
            return value
        except Exception:
            self._misses += 1
            return default

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        if key in self._cache:
            del self._cache[key]
            return True
        return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict least recently used entry (remove one arbitrary key if any)."""
        if self._cache:
            try:
                first = next(iter(self._cache))
                del self._cache[first]
                self._evictions += 1
            except StopIteration:
                pass

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys()) if hasattr(self._cache, 'keys') else list(self._cache)

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values()) if hasattr(self._cache, 'values') else [self._cache[k] for k in self.keys()]

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items()) if hasattr(self._cache, 'items') else [(k, self._cache[k]) for k in self.keys()]

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
        }

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache
# ============================================================================
# CACHETOOLS IMPLEMENTATIONS
# ============================================================================


class CachetoolsLRUCache(ACache):
    """Cachetools LRUCache wrapper."""

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """Initialize Cachetools LRU cache."""
        if not HAS_CACHETOOLS:
            raise ImportError(
                "cachetools not available. Install with: pip install cachetools"
            )
        super().__init__(capacity=capacity, ttl=int(ttl) if ttl else None)
        self._cache = CachetoolsLRU(maxsize=capacity)
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        value = self._cache.get(key, default)
        if value is not default:
            self._hits += 1
        else:
            self._misses += 1
        return value

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        if len(self._cache) >= self.capacity and key not in self._cache:
            self._evictions += 1
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict entries (handled automatically)."""
        pass

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys())

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values())

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items())

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
        }
    # ICache interface compatibility

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache


class CachetoolsLFUCache(ACache):
    """Cachetools LFUCache wrapper."""

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """Initialize Cachetools LFU cache."""
        if not HAS_CACHETOOLS:
            raise ImportError(
                "cachetools not available. Install with: pip install cachetools"
            )
        super().__init__(capacity=capacity, ttl=int(ttl) if ttl else None)
        self._cache = CachetoolsLFU(maxsize=capacity)
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        value = self._cache.get(key, default)
        if value is not default:
            self._hits += 1
        else:
            self._misses += 1
        return value

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        if len(self._cache) >= self.capacity and key not in self._cache:
            self._evictions += 1
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict entries (handled automatically)."""
        pass

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys())

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values())

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items())

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
        }
    # ICache interface compatibility

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache


class CachetoolsTTLCache(ACache):
    """Cachetools TTLCache wrapper."""

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """
        Initialize Cachetools TTL cache.
        Args:
            capacity: Maximum cache size
            ttl: Time to live in seconds (default: 3600)
        """
        if not HAS_CACHETOOLS:
            raise ImportError(
                "cachetools not available. Install with: pip install cachetools"
            )
        # Use provided ttl or default to 3600 seconds
        ttl_value = float(ttl) if ttl else 3600.0
        super().__init__(capacity=capacity, ttl=int(ttl_value))
        self._cache = CachetoolsTTL(maxsize=capacity, ttl=ttl_value)
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        value = self._cache.get(key, default)
        if value is not default:
            self._hits += 1
        else:
            self._misses += 1
        return value

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        if len(self._cache) >= self.capacity and key not in self._cache:
            self._evictions += 1
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict entries (handled automatically by TTL)."""
        pass

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys())

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values())

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items())

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
            "ttl": self.ttl,
        }
    # ICache interface compatibility

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache


class CachetoolsRRCache(ACache):
    """Cachetools RRCache (Random Replacement) wrapper."""

    def __init__(self, capacity: int = 128, ttl: float | None = None, **kwargs):
        """Initialize Cachetools RR cache."""
        if not HAS_CACHETOOLS:
            raise ImportError(
                "cachetools not available. Install with: pip install cachetools"
            )
        super().__init__(capacity=capacity, ttl=int(ttl) if ttl else None)
        self._cache = CachetoolsRR(maxsize=capacity)
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def get(self, key: Hashable, default: Any = None) -> Any:
        """Get value from cache."""
        value = self._cache.get(key, default)
        if value is not default:
            self._hits += 1
        else:
            self._misses += 1
        return value

    def put(self, key: Hashable, value: Any) -> None:
        """Put value in cache."""
        if len(self._cache) >= self.capacity and key not in self._cache:
            self._evictions += 1
        self._cache[key] = value

    def delete(self, key: Hashable) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._hits = 0
        self._misses = 0
        self._evictions = 0

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)

    def is_full(self) -> bool:
        """Check if cache is full."""
        return len(self._cache) >= self.capacity

    def evict(self) -> None:
        """Evict entries (handled automatically)."""
        pass

    def keys(self) -> list[Hashable]:
        """Get list of all cache keys."""
        return list(self._cache.keys())

    def values(self) -> list[Any]:
        """Get list of all cache values."""
        return list(self._cache.values())

    def items(self) -> list[tuple[Hashable, Any]]:
        """Get list of all key-value pairs."""
        return list(self._cache.items())

    def get_stats(self) -> dict[str, Any]:
        """Get cache statistics."""
        total = self._hits + self._misses
        hit_rate = (self._hits / total) if total > 0 else 0.0
        return {
            "hits": self._hits,
            "misses": self._misses,
            "evictions": self._evictions,
            "hit_rate": hit_rate,
            "size": self.size(),
            "capacity": self.capacity,
        }
    # ICache interface compatibility

    def set(self, key: str, value: Any, ttl: int | None = None) -> bool:
        """Set value in cache (ICache interface)."""
        self.put(key, value)
        return True

    def exists(self, key: str) -> bool:
        """Check if key exists (ICache interface)."""
        return key in self._cache
__all__ = [
    'CacheboxCache',
    'FunctoolsLRUCache',
    'PylruCache',
    'CachetoolsLRUCache',
    'CachetoolsLFUCache',
    'CachetoolsTTLCache',
    'CachetoolsRRCache',
    'HAS_CACHEBOX',
    'HAS_CACHETOOLS',
    'HAS_PYLRU',
]
