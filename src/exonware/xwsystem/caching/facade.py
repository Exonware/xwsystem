#exonware/xwsystem/src/exonware/xwsystem/caching/facade.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.4
Generation Date: January 2026
XWCache - Unified Caching Facade
Simplified API for all caching strategies:
- LRU, LFU, TTL
- Memory-bounded, Two-tier, Secure
- Async support
- Decorator support
"""

from typing import Any, Optional, Callable
from functools import wraps
from . import (
    LRUCache, AsyncLRUCache,
    LFUCache, AsyncLFUCache,
    TTLCache, AsyncTTLCache,
    OptimizedLFUCache, AsyncOptimizedLFUCache,
    MemoryBoundedLRUCache, MemoryBoundedLFUCache,
    TwoTierCache,
    SecureLRUCache, SecureLFUCache, SecureTTLCache,
    xwcached, xw_async_cached,
)
from .factory import CacheFactory, CacheType
from ..config.logging_setup import get_logger
logger = get_logger(__name__)


class XWCache:
    """
    Unified caching facade - simple API for all cache strategies.
    Examples:
        >>> # Simple LRU cache
        >>> cache = XWCache(strategy="LRU", capacity=1000)
        >>> cache.put("key", "value")
        >>> value = cache.get("key")
        >>> # Named cache instance
        >>> user_cache = XWCache(strategy="LRU", name="users", capacity=500)
        >>> # TTL cache
        >>> session_cache = XWCache(strategy="TTL", ttl=3600)
        >>> # Async cache
        >>> async_cache = XWCache(strategy="LRU", async=True, capacity=1000)
        >>> # Advanced features
        >>> cache = XWCache(
        ...     strategy="LRU",
        ...     capacity=1000,
        ...     memory_bounded=True,
        ...     two_tier=True,
        ... )
        >>> # Decorator
        >>> @XWCache.cached(strategy="LRU", ttl=3600)
        >>> def expensive_function(x):
        ...     return x * 2
    """
    # Registry for named caches
    _cache_registry: dict[str, Any] = {}

    def __init__(
        self,
        strategy: str = "LRU",
        *,
        name: Optional[str] = None,
        capacity: int = 1000,
        ttl: Optional[int] = None,
        async_mode: bool = False,
        memory_bounded: bool = False,
        two_tier: bool = False,
        secure: bool = False,
        **kwargs
    ):
        """
        Initialize unified cache.
        Args:
            strategy: Cache strategy ("LRU", "LFU", "TTL", "OPTIMIZED_LFU")
            name: Optional name for cache instance (for multiple caches)
            capacity: Maximum cache size
            ttl: Time to live in seconds (for TTL strategy)
            async_mode: Use async cache implementation
            memory_bounded: Use memory-bounded cache
            two_tier: Use two-tier cache (memory + disk)
            secure: Use secure cache (with rate limiting)
            **kwargs: Additional cache-specific options
        """
        self.strategy = strategy.upper()
        self.name = name
        self.capacity = capacity
        self.ttl = ttl
        self.async_mode = async_mode
        self.memory_bounded = memory_bounded
        self.two_tier = two_tier
        self.secure = secure
        self.kwargs = kwargs
        # Create cache instance
        self._cache = self._create_cache()
        # Register named cache
        if name:
            XWCache._cache_registry[name] = self._cache

    def _create_cache(self):
        """Create cache instance based on strategy and options."""
        strategy_map = {
            "LRU": (LRUCache, AsyncLRUCache),
            "LFU": (LFUCache, AsyncLFUCache),
            "TTL": (TTLCache, AsyncTTLCache),
            "OPTIMIZED_LFU": (OptimizedLFUCache, AsyncOptimizedLFUCache),
        }
        # Secure variants
        if self.secure:
            secure_map = {
                "LRU": SecureLRUCache,
                "LFU": SecureLFUCache,
                "TTL": SecureTTLCache,
            }
            if self.strategy in secure_map:
                cache_class = secure_map[self.strategy]
                return cache_class(capacity=self.capacity, ttl=self.ttl, **self.kwargs)
        # Memory-bounded variants
        if self.memory_bounded:
            if self.strategy == "LRU":
                return MemoryBoundedLRUCache(capacity=self.capacity, **self.kwargs)
            elif self.strategy == "LFU":
                return MemoryBoundedLFUCache(capacity=self.capacity, **self.kwargs)
        # Two-tier cache
        if self.two_tier:
            return TwoTierCache(
                namespace=self.name or "default",
                memory_capacity=self.capacity,
                **self.kwargs
            )
        # Standard cache
        if self.strategy not in strategy_map:
            raise ValueError(f"Unknown cache strategy: {self.strategy}")
        cache_class = strategy_map[self.strategy][1 if self.async_mode else 0]
        # TTL-specific handling
        if self.strategy == "TTL" and self.ttl:
            return cache_class(capacity=self.capacity, ttl=self.ttl, **self.kwargs)
        return cache_class(capacity=self.capacity, **self.kwargs)

    def get(self, key: Any, default: Any = None) -> Any:
        """Get value from cache."""
        return self._cache.get(key, default)

    def put(self, key: Any, value: Any) -> None:
        """Put value in cache."""
        self._cache.put(key, value)

    def delete(self, key: Any) -> bool:
        """Delete value from cache."""
        return self._cache.delete(key)

    def clear(self) -> None:
        """Clear all cache entries."""
        self._cache.clear()

    def size(self) -> int:
        """Get cache size."""
        return self._cache.size()

    def __getitem__(self, key: Any) -> Any:
        """Support dict-like access."""
        return self.get(key)

    def __setitem__(self, key: Any, value: Any) -> None:
        """Support dict-like access."""
        self.put(key, value)

    def __delitem__(self, key: Any) -> None:
        """Support dict-like access."""
        self.delete(key)
    @classmethod

    def get_cache(cls, name: str) -> Any:
        """Get named cache instance."""
        return cls._cache_registry.get(name)
    @classmethod

    def cached(
        cls,
        strategy: str = "LRU",
        capacity: int = 1000,
        ttl: Optional[int] = None,
        async_mode: bool = False,
        **kwargs
    ):
        """
        Decorator for caching function results.
        Examples:
            >>> @XWCache.cached(strategy="LRU", ttl=3600)
            >>> def expensive_function(x):
            ...     return x * 2
        """
        # Create cache instance based on strategy
        strategy_map = {
            "LRU": (LRUCache, AsyncLRUCache),
            "LFU": (LFUCache, AsyncLFUCache),
            "TTL": (TTLCache, AsyncTTLCache),
        }
        if strategy.upper() not in strategy_map:
            raise ValueError(f"Unknown cache strategy: {strategy}")
        cache_class = strategy_map[strategy.upper()][1 if async_mode else 0]
        if strategy.upper() == "TTL" and ttl:
            cache = cache_class(capacity=capacity, ttl=ttl, **kwargs)
        else:
            cache = cache_class(capacity=capacity, **kwargs)
        if async_mode:
            return xw_async_cached(cache=cache, ttl=ttl, **kwargs)
        else:
            return xwcached(cache=cache, ttl=ttl, **kwargs)
    @classmethod

    def create(
        cls,
        strategy: str = "LRU",
        capacity: int = 1000,
        **kwargs
    ) -> "XWCache":
        """
        Factory method for creating cache instances.
        Examples:
            >>> cache = XWCache.create(strategy="LRU", capacity=1000)
        """
        return cls(strategy=strategy, capacity=capacity, **kwargs)
