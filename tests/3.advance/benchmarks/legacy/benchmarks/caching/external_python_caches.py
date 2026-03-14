#exonware/xwsystem/tests/3.advance/benchmarks/legacy/benchmarks/caching/external_python_caches.py
"""
Python wrappers for external Python caching libraries.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.1.0.1
Wrappers for:
- cachebox: Rust-based Python cache
- functools.lru_cache: Standard library LRU
- cachetools: Flexible caching with TTL, LFU, RR
- diskcache: Disk-based caching with LRU
- cacheout: Modern caching with TTL, LRU, LFU, FIFO
- pylru: Simple LRU cache
"""

from typing import Any

from collections.abc import Callable
from functools import lru_cache as std_lru_cache
import sys
import time
# Try to import the new proper implementations
USE_NEW_IMPLEMENTATIONS = False
try:
    from exonware.xwsystem.caching.external_caching_python import (
        CacheboxCache,
        FunctoolsLRUCache,
        CachetoolsLRUCache,
        CachetoolsLFUCache,
        CachetoolsTTLCache,
        CachetoolsRRCache,
        HAS_CACHEBOX,
        HAS_CACHETOOLS,
    )
    USE_NEW_IMPLEMENTATIONS = True
except ImportError:
    HAS_CACHEBOX = False
    HAS_CACHETOOLS = False
# Import flags from new implementations if available
if USE_NEW_IMPLEMENTATIONS:
    # Use the new implementations directly
    CacheboxWrapper = CacheboxCache
    FunctoolsLRUWrapper = FunctoolsLRUCache
    CachetoolsLRUWrapper = CachetoolsLRUCache
    CachetoolsLFUWrapper = CachetoolsLFUCache
    CachetoolsTTLWrapper = CachetoolsTTLCache
    CachetoolsRRWrapper = CachetoolsRRCache
else:
    # Fallback to old wrapper implementations
    # Cachebox wrapper
    try:
        import cachebox
        HAS_CACHEBOX = True
    except ImportError:
        HAS_CACHEBOX = False
        cachebox = None
    # Cachetools wrapper
    try:
        import cachetools
        from cachetools import (
            LRUCache as CachetoolsLRU,
            LFUCache as CachetoolsLFU,
            TTLCache as CachetoolsTTL,
            RRCache as CachetoolsRR,
            cached,
            cachedmethod,
        )
        HAS_CACHETOOLS = True
    except ImportError:
        HAS_CACHETOOLS = False
        cachetools = None
        CachetoolsLRU = None
        CachetoolsLFU = None
        CachetoolsTTL = None
        CachetoolsRR = None
# Diskcache wrapper
try:
    import diskcache
    HAS_DISKCACHE = True
except ImportError:
    HAS_DISKCACHE = False
    diskcache = None
# Cacheout wrapper
try:
    import cacheout
    from cacheout import (
        LRUCache as CacheoutLRU,
        LFUCache as CacheoutLFU,
        TTLCache as CacheoutTTL,
        FIFOCache as CacheoutFIFO,
    )
    HAS_CACHEOUT = True
except ImportError:
    HAS_CACHEOUT = False
    cacheout = None
    CacheoutLRU = None
    CacheoutLFU = None
    CacheoutTTL = None
    CacheoutFIFO = None
# Pylru wrapper
try:
    import pylru
    HAS_PYLRU = True
except ImportError:
    HAS_PYLRU = False
    pylru = None


class CacheboxWrapper:
    """Wrapper for cachebox library."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHEBOX:
            raise ImportError("cachebox not available. Install with: pip install cachebox")
        # cachebox uses maxsize parameter
        self._cache = cachebox.Cache(maxsize=capacity)
        self._capacity = capacity

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        try:
            return self._cache.get(key, default)
        except Exception:
            return default

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._capacity


class FunctoolsLRUWrapper:
    """Wrapper for functools.lru_cache."""

    def __init__(self, capacity: int = 128, **kwargs):
        self._capacity = capacity
        self._cache = {}
        self._access_order = []
        # functools.lru_cache is a decorator, so we implement LRU manually
        # or use a simple dict with manual LRU tracking

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        if key in self._cache:
            # Move to end (most recently used)
            self._access_order.remove(key)
            self._access_order.append(key)
            return self._cache[key]
        return default

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        if key in self._cache:
            # Update existing
            self._access_order.remove(key)
        elif len(self._cache) >= self._capacity:
            # Evict least recently used
            if self._access_order:
                lru_key = self._access_order.pop(0)
                del self._cache[lru_key]
        self._cache[key] = value
        self._access_order.append(key)

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        if key in self._cache:
            del self._cache[key]
            if key in self._access_order:
                self._access_order.remove(key)
            return True
        return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()
        self._access_order.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._capacity


class CachetoolsLRUWrapper:
    """Wrapper for cachetools LRUCache."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHETOOLS:
            raise ImportError("cachetools not available. Install with: pip install cachetools")
        self._cache = CachetoolsLRU(maxsize=capacity)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class CachetoolsLFUWrapper:
    """Wrapper for cachetools LFUCache."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHETOOLS:
            raise ImportError("cachetools not available. Install with: pip install cachetools")
        self._cache = CachetoolsLFU(maxsize=capacity)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class CachetoolsTTLWrapper:
    """Wrapper for cachetools TTLCache."""

    def __init__(self, capacity: int = 128, ttl: float = 3600.0, **kwargs):
        if not HAS_CACHETOOLS:
            raise ImportError("cachetools not available. Install with: pip install cachetools")
        self._cache = CachetoolsTTL(maxsize=capacity, ttl=ttl)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class CachetoolsRRWrapper:
    """Wrapper for cachetools RRCache (Random Replacement)."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHETOOLS:
            raise ImportError("cachetools not available. Install with: pip install cachetools")
        self._cache = CachetoolsRR(maxsize=capacity)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class DiskcacheWrapper:
    """Wrapper for diskcache library."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_DISKCACHE:
            raise ImportError("diskcache not available. Install with: pip install diskcache")
        # diskcache uses size parameter for max entries
        self._cache = diskcache.Cache(size_limit=capacity)
        self._capacity = capacity

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        try:
            return self._cache.get(key, default)
        except Exception:
            return default

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            return self._cache.delete(key)
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._capacity


class CacheoutLRUWrapper:
    """Wrapper for cacheout LRUCache."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHEOUT:
            raise ImportError("cacheout not available. Install with: pip install cacheout")
        self._cache = CacheoutLRU(maxsize=capacity)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class CacheoutLFUWrapper:
    """Wrapper for cacheout LFUCache."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHEOUT:
            raise ImportError("cacheout not available. Install with: pip install cacheout")
        self._cache = CacheoutLFU(maxsize=capacity)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class CacheoutTTLWrapper:
    """Wrapper for cacheout TTLCache."""

    def __init__(self, capacity: int = 128, ttl: float = 3600.0, **kwargs):
        if not HAS_CACHEOUT:
            raise ImportError("cacheout not available. Install with: pip install cacheout")
        self._cache = CacheoutTTL(maxsize=capacity, ttl=ttl)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class CacheoutFIFOWrapper:
    """Wrapper for cacheout FIFOCache."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_CACHEOUT:
            raise ImportError("cacheout not available. Install with: pip install cacheout")
        self._cache = CacheoutFIFO(maxsize=capacity)

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._cache.maxsize or 0


class PylruWrapper:
    """Wrapper for pylru library."""

    def __init__(self, capacity: int = 128, **kwargs):
        if not HAS_PYLRU:
            raise ImportError("pylru not available. Install with: pip install pylru")
        self._cache = pylru.lrucache(capacity)
        self._capacity = capacity
        self._access_order = []

    def get(self, key: str, default: Any = None) -> Any:
        """Get value from cache."""
        try:
            return self._cache[key]
        except KeyError:
            return default

    def put(self, key: str, value: Any) -> None:
        """Put value in cache."""
        self._cache[key] = value

    def delete(self, key: str) -> bool:
        """Delete key from cache."""
        try:
            del self._cache[key]
            return True
        except KeyError:
            return False

    def clear(self) -> None:
        """Clear cache."""
        # pylru doesn't have clear, so we recreate
        self._cache = pylru.lrucache(self._capacity)

    def size(self) -> int:
        """Get cache size."""
        return len(self._cache)
    @property

    def capacity(self) -> int:
        """Get cache capacity."""
        return self._capacity
# Export available wrappers
__all__ = [
    'CacheboxWrapper',
    'FunctoolsLRUWrapper',
    'CachetoolsLRUWrapper',
    'CachetoolsLFUWrapper',
    'CachetoolsTTLWrapper',
    'CachetoolsRRWrapper',
    'DiskcacheWrapper',
    'CacheoutLRUWrapper',
    'CacheoutLFUWrapper',
    'CacheoutTTLWrapper',
    'CacheoutFIFOWrapper',
    'PylruWrapper',
    'HAS_CACHEBOX',
    'HAS_CACHETOOLS',
    'HAS_DISKCACHE',
    'HAS_CACHEOUT',
    'HAS_PYLRU',
]
