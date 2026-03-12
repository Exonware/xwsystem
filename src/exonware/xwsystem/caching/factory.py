#exonware/xwsystem/src/exonware/xwsystem/caching/factory.py
"""
Cache Factory - Configurable cache creation utility.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.2
Generation Date: January 2025
Provides configurable cache factory that allows choosing different caching engines
via settings/environment variables. Defaults to PylruCache when pylru is installed (highest throughput);
otherwise FunctoolsLRUCache.
"""

import os
from typing import Any, Optional, Type
from enum import Enum
from ..config.logging_setup import get_logger
from .base import ACache
from .lru_cache import LRUCache
from .lfu_cache import LFUCache
from .ttl_cache import TTLCache
from .lfu_optimized import OptimizedLFUCache
from .memory_bounded import MemoryBoundedLRUCache, MemoryBoundedLFUCache
from .secure_cache import SecureLRUCache, SecureLFUCache, SecureTTLCache
from .external_caching_python import FunctoolsLRUCache, PylruCache, HAS_PYLRU
logger = get_logger("xwsystem.caching.factory")
# Default cache type: pylru when available, else functools_lru
_DEFAULT_CACHE_TYPE = "pylru" if HAS_PYLRU else "functools_lru"


class CacheType(str, Enum):
    """Supported cache types."""
    PYLRU = "pylru"  # Default when installed - highest throughput (~2.4M mixed/s)
    FUNCTOOLS_LRU = "functools_lru"
    LRU = "lru"
    LFU = "lfu"
    TTL = "ttl"
    OPTIMIZED_LFU = "optimized_lfu"
    MEMORY_BOUNDED_LRU = "memory_bounded_lru"
    MEMORY_BOUNDED_LFU = "memory_bounded_lfu"
    SECURE_LRU = "secure_lru"
    SECURE_LFU = "secure_lfu"
    SECURE_TTL = "secure_ttl"


class CacheFactory:
    """
    Configurable cache factory.
    Allows choosing different caching engines via:
    - Environment variable: XWSYSTEM_CACHE_TYPE (default: 'pylru' when installed, else 'functools_lru')
    - Settings: 'xwsystem.cache.type'
    - Per-instance configuration
    Supported cache types:
    - 'pylru' (default when installed) - PylruCache - Highest throughput (~2.4M get/put/mixed ops/s)
    - 'functools_lru' - FunctoolsLRUCache - OrderedDict-based LRU
    - 'lfu' - LFUCache - Least Frequently Used
    - 'lru' - LRUCache - Least Recently Used
    - 'ttl' - TTLCache - Time-To-Live expiration
    - 'optimized_lfu' - OptimizedLFUCache - O(1) LFU eviction
    - 'memory_bounded_lru' - MemoryBoundedLRUCache - Memory-bounded LRU
    - 'memory_bounded_lfu' - MemoryBoundedLFUCache - Memory-bounded LFU
    - 'secure_lru' - SecureLRUCache - Secure LRU with rate limiting
    - 'secure_lfu' - SecureLFUCache - Secure LFU with rate limiting
    - 'secure_ttl' - SecureTTLCache - Secure TTL with rate limiting
    Example:
        # Use default (PylruCache when pylru installed, else FunctoolsLRUCache)
        cache = CacheFactory.create(capacity=1000)
        # Override via environment variable
        # export XWSYSTEM_CACHE_TYPE=lru
        cache = CacheFactory.create(capacity=1000)
        # Override programmatically
        cache = CacheFactory.create(capacity=1000, cache_type='ttl', ttl=3600)
        # Package-specific override
        cache = CacheFactory.create(capacity=1000, namespace='xwstorage')
    """
    # Global settings registry (can be set by packages)
    _settings: dict[str, Any] = {}
    # Cache type registry
    _cache_types: dict[str, Type[ACache]] = {
        CacheType.PYLRU: PylruCache,  # Default when installed - highest throughput
        CacheType.FUNCTOOLS_LRU: FunctoolsLRUCache,
        CacheType.LRU: LRUCache,
        CacheType.LFU: LFUCache,
        CacheType.TTL: TTLCache,
        CacheType.OPTIMIZED_LFU: OptimizedLFUCache,
        CacheType.MEMORY_BOUNDED_LRU: MemoryBoundedLRUCache,
        CacheType.MEMORY_BOUNDED_LFU: MemoryBoundedLFUCache,
        CacheType.SECURE_LRU: SecureLRUCache,
        CacheType.SECURE_LFU: SecureLFUCache,
        CacheType.SECURE_TTL: SecureTTLCache,
    }
    @classmethod

    def set_setting(cls, key: str, value: Any) -> None:
        """
        Set factory setting.
        Args:
            key: Setting key (e.g., 'cache.type', 'cache.default_capacity')
            value: Setting value
        """
        cls._settings[key] = value
        logger.debug(f"CacheFactory setting updated: {key}={value}")
    @classmethod

    def get_setting(cls, key: str, default: Any = None) -> Any:
        """
        Get factory setting.
        Args:
            key: Setting key
            default: Default value if not found
        Returns:
            Setting value or default
        """
        return cls._settings.get(key, default)
    @classmethod

    def get_cache_type(cls, namespace: Optional[str] = None) -> str:
        """
        Get cache type from configuration.
        Priority:
        1. Namespace-specific setting: 'cache.type.{namespace}'
        2. Global setting: 'cache.type' or 'xwsystem.cache.type'
        3. Environment variable: XWSYSTEM_CACHE_TYPE or XWSYSTEM_CACHE_TYPE_{NAMESPACE}
        4. Default: 'pylru' when installed, else 'functools_lru'
        Args:
            namespace: Optional namespace for namespace-specific configuration
        Returns:
            Cache type string
        """
        # Check namespace-specific setting first
        if namespace:
            # Namespace-specific setting
            ns_key = f'cache.type.{namespace}'
            cache_type = cls.get_setting(ns_key)
            if cache_type:
                return cache_type
            # Namespace-specific environment variable
            env_key = f'XWSYSTEM_CACHE_TYPE_{namespace.upper()}'
            cache_type = os.getenv(env_key)
            if cache_type:
                return cache_type.lower()
        # Check global setting
        cache_type = cls.get_setting('cache.type') or cls.get_setting('xwsystem.cache.type')
        if cache_type:
            return cache_type
        # Check environment variable
        cache_type = os.getenv('XWSYSTEM_CACHE_TYPE', _DEFAULT_CACHE_TYPE)
        return cache_type.lower()
    @classmethod

    def create(
        cls,
        capacity: int = 128,
        cache_type: Optional[str] = None,
        namespace: Optional[str] = None,
        name: Optional[str] = None,
        **kwargs
    ) -> ACache:
        """
        Create cache instance based on configuration.
        Args:
            capacity: Cache capacity (default: 128)
            cache_type: Override cache type (optional)
            namespace: Namespace for namespace-specific configuration (optional)
            name: Cache name for debugging (optional)
            **kwargs: Additional cache-specific parameters (e.g., ttl, memory_budget_mb)
        Returns:
            Cache instance
        Raises:
            ValueError: If cache type is invalid
        """
        # Determine cache type
        if cache_type is None:
            cache_type = cls.get_cache_type(namespace)
        else:
            cache_type = cache_type.lower()
        # Get cache class
        cache_class = cls._cache_types.get(cache_type)
        if cache_class is None:
            # Try to find by partial match
            cache_class = next(
                (cls._cache_types[ct] for ct in cls._cache_types.keys() 
                 if cache_type in ct.lower() or ct.lower() in cache_type),
                None
            )
            if cache_class is None:
                default_cls = PylruCache if HAS_PYLRU else FunctoolsLRUCache
                logger.warning(
                    f"Unknown cache type '{cache_type}', using default ({default_cls.__name__}). "
                    f"Available types: {list(cls._cache_types.keys())}"
                )
                cache_class = default_cls
                cache_type = CacheType.PYLRU if HAS_PYLRU else CacheType.FUNCTOOLS_LRU
        # Prepare cache parameters
        cache_params = {'capacity': capacity}
        if name:
            cache_params['name'] = name
        # Add cache-specific parameters
        if cache_type == CacheType.TTL or cache_type == CacheType.SECURE_TTL:
            if 'ttl' not in kwargs:
                # Get TTL from settings or use default
                ttl = cls.get_setting('cache.ttl') or cls.get_setting('xwsystem.cache.ttl') or 3600
                cache_params['ttl'] = float(ttl)
            else:
                cache_params['ttl'] = kwargs.pop('ttl')
        if cache_type in (CacheType.MEMORY_BOUNDED_LRU, CacheType.MEMORY_BOUNDED_LFU):
            if 'memory_budget_mb' not in kwargs:
                memory_budget = cls.get_setting('cache.memory_budget_mb') or 100.0
                cache_params['memory_budget_mb'] = float(memory_budget)
            else:
                cache_params['memory_budget_mb'] = kwargs.pop('memory_budget_mb')
        # Add any remaining kwargs
        cache_params.update(kwargs)
        # Create cache instance
        try:
            cache = cache_class(**cache_params)
            logger.debug(
                f"Created {cache_type} cache: capacity={capacity}, "
                f"namespace={namespace}, name={name}"
            )
            return cache
        except Exception as e:
            logger.error(f"Failed to create {cache_type} cache: {e}, falling back to default")
            if HAS_PYLRU:
                try:
                    return PylruCache(capacity=capacity, **kwargs)
                except Exception:
                    return FunctoolsLRUCache(capacity=capacity, name=name)
            return FunctoolsLRUCache(capacity=capacity, name=name)
    @classmethod

    def register_cache_type(cls, cache_type: str, cache_class: Type[ACache]) -> None:
        """
        Register custom cache type.
        Args:
            cache_type: Cache type identifier
            cache_class: Cache class
        """
        cls._cache_types[cache_type.lower()] = cache_class
        logger.info(f"Registered custom cache type: {cache_type}")
    @classmethod

    def get_available_types(cls) -> list[str]:
        """Get list of available cache types."""
        return list(cls._cache_types.keys())
# Convenience function for easy access

def create_cache(
    capacity: int = 128,
    cache_type: Optional[str] = None,
    namespace: Optional[str] = None,
    name: Optional[str] = None,
    **kwargs
) -> ACache:
    """
    Create cache instance (convenience function).
    Example:
        from exonware.xwsystem.caching import create_cache
        # Use default (PylruCache when installed, else FunctoolsLRUCache)
        cache = create_cache(capacity=1000)
        # Override type
        cache = create_cache(capacity=1000, cache_type='lru')
        # Namespace-specific
        cache = create_cache(capacity=1000, namespace='xwstorage')
        # Override via environment variable
        # export XWSYSTEM_CACHE_TYPE=lfu
        cache = create_cache(capacity=1000)
    """
    return CacheFactory.create(
        capacity=capacity,
        cache_type=cache_type,
        namespace=namespace,
        name=name,
        **kwargs
    )
