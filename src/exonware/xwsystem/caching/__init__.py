#exonware/xwsystem/src/exonware/xwsystem/caching/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.20
Generation Date: 01-Nov-2025
xwsystem Caching Package
Comprehensive caching framework with LRU, LFU, TTL, and advanced caching strategies.
Production-grade caching utilities for high-performance applications.
Security Features (Priority #1):
- Input validation and sanitization
- Integrity verification with checksums
- Rate limiting and DoS protection
Usability Features (Priority #2):
- Context manager support
- Fluent API for method chaining
- Enhanced statistics formatting
- XW-prefixed decorators (@xwcached, @xw_async_cached)
- Read-through/Write-through auto-loading
- Tag-based invalidation
Performance Features (Priority #4):
- O(1) LFU cache (100x+ faster eviction)
- Batch operations (get_many, put_many, delete_many)
- Memory-bounded caches
- Cache warming strategies
- Bloom filter for fast negative lookups
- Write-behind (lazy write) for better write performance
- Async iterators for async caches
Extensibility Features (Priority #5):
- Event hooks system
- Pluggable eviction strategies
- Observable caches
- Advanced decorators with hooks
- Conditional eviction policies
- Prometheus metrics export
- Serializable caches (save/load)
NEW in v0.0.1.388:
- ReadThroughCache, WriteThroughCache, ReadWriteThroughCache
- TaggedCache with tag-based invalidation
- WriteBehindCache for delayed persistence
- BloomFilterCache for probabilistic lookups
- ConditionalEvictionCache with custom eviction rules
- SerializableCache with save/load support
- PrometheusExporter and StatsCollector
- Async iterators (__aiter__, items_async)
- XW-prefixed decorators (xwcached, xw_async_cached)
"""
# Core caches

from .lru_cache import LRUCache, AsyncLRUCache
from .lfu_cache import LFUCache, AsyncLFUCache
from .ttl_cache import TTLCache, AsyncTTLCache
from .cache_manager import CacheManager, CacheConfig, CacheStats
from .decorators import (
    xwcached, xw_async_cached, xwcache, xw_async_cache
)
from .distributed import DistributedCache, RedisCache
# Performance-optimized caches
from .lfu_optimized import OptimizedLFUCache, AsyncOptimizedLFUCache
from .memory_bounded import MemoryBoundedLRUCache, MemoryBoundedLFUCache
from .two_tier_cache import TwoTierCache
# Advanced cache types (NEW in v0.0.1.388)
from .read_through import ReadThroughCache, WriteThroughCache, ReadWriteThroughCache
from .serializable import SerializableCache
from .tagging import TaggedCache
from .write_behind import WriteBehindCache
from .conditional import ConditionalEvictionCache
from .bloom_cache import BloomFilterCache, SimpleBloomFilter
from .metrics_exporter import PrometheusExporter, StatsCollector
# Security features
from .secure_cache import SecureLRUCache, SecureLFUCache, SecureTTLCache
from .validation import validate_cache_key, validate_cache_value, sanitize_key
from .rate_limiter import RateLimiter, FixedWindowRateLimiter
from .integrity import CacheEntry, create_secure_entry, verify_entry_integrity
# Usability features
from .fluent import FluentLRUCache, FluentLFUCache, FluentTTLCache
from .stats import format_cache_stats, format_cache_stats_table, get_stats_summary
# Extensibility features
from .events import CacheEvent, CacheEventEmitter, EventLogger
from .observable_cache import ObservableLRUCache, ObservableLFUCache
from .eviction_strategies import (
    AEvictionStrategy,
    LRUEvictionStrategy,
    LFUEvictionStrategy,
    FIFOEvictionStrategy,
    RandomEvictionStrategy,
    SizeBasedEvictionStrategy,
    TTLEvictionStrategy,
)
from .pluggable_cache import PluggableCache
from .warming import (
    AWarmingStrategy,
    PreloadWarmingStrategy,
    LazyWarmingStrategy,
    PriorityWarmingStrategy,
    warm_cache,
)
# Utilities
from .utils import (
    estimate_object_size,
    compute_checksum,
    format_bytes,
    default_key_builder,
)
# Cache Factory (configurable cache creation)
from .factory import CacheFactory, CacheType, create_cache
# Interfaces (for advanced usage)
from .contracts import ICache
# External cache libraries
from .external_caching_python import (
    CacheboxCache,
    FunctoolsLRUCache,
    PylruCache,
    CachetoolsLRUCache,
    CachetoolsLFUCache,
    CachetoolsTTLCache,
    CachetoolsRRCache,
    HAS_CACHEBOX,
    HAS_CACHETOOLS,
    HAS_PYLRU,
)
# Errors
from .errors import (
    CacheError,
    CacheKeyError,
    CacheSizeError,
    CacheTTLError,
    CacheValidationError,
    CacheIntegrityError,
    CacheRateLimitError,
    CacheValueSizeError,
    CacheKeySizeError,
)
__all__ = [
    # Core caches
    "LRUCache",
    "AsyncLRUCache", 
    "LFUCache",
    "AsyncLFUCache",
    "TTLCache", 
    "AsyncTTLCache",
    # Performance-optimized
    "OptimizedLFUCache",
    "AsyncOptimizedLFUCache",
    "MemoryBoundedLRUCache",
    "MemoryBoundedLFUCache",
    "TwoTierCache",
    # Advanced cache types (NEW)
    "ReadThroughCache",
    "WriteThroughCache",
    "ReadWriteThroughCache",
    "SerializableCache",
    "TaggedCache",
    "WriteBehindCache",
    "ConditionalEvictionCache",
    "BloomFilterCache",
    "SimpleBloomFilter",
    "PrometheusExporter",
    "StatsCollector",
    # Security
    "SecureLRUCache",
    "SecureLFUCache",
    "SecureTTLCache",
    "validate_cache_key",
    "validate_cache_value",
    "sanitize_key",
    "RateLimiter",
    "FixedWindowRateLimiter",
    "CacheEntry",
    "create_secure_entry",
    "verify_entry_integrity",
    # Usability
    "FluentLRUCache",
    "FluentLFUCache",
    "FluentTTLCache",
    "format_cache_stats",
    "format_cache_stats_table",
    "get_stats_summary",
    # Extensibility
    "CacheEvent",
    "CacheEventEmitter",
    "EventLogger",
    "ObservableLRUCache",
    "ObservableLFUCache",
    "AEvictionStrategy",
    "LRUEvictionStrategy",
    "LFUEvictionStrategy",
    "FIFOEvictionStrategy",
    "RandomEvictionStrategy",
    "SizeBasedEvictionStrategy",
    "TTLEvictionStrategy",
    "PluggableCache",
    "AWarmingStrategy",
    "PreloadWarmingStrategy",
    "LazyWarmingStrategy",
    "PriorityWarmingStrategy",
    "warm_cache",
    # Utilities
    "estimate_object_size",
    "compute_checksum",
    "format_bytes",
    "default_key_builder",
    # Cache Factory
    "CacheFactory",
    "CacheType",
    "create_cache",
    # Interfaces
    "ICache",
    # Management
    "CacheManager",
    "CacheConfig",
    "CacheStats",
    # Decorators
    "xwcached",
    "xw_async_cached",
    "xwcache",
    "xw_async_cache",
    # Distributed
    "DistributedCache",
    "RedisCache",
    # External cache libraries
    "CacheboxCache",
    "FunctoolsLRUCache",
    "PylruCache",
    "CachetoolsLRUCache",
    "CachetoolsLFUCache",
    "CachetoolsTTLCache",
    "CachetoolsRRCache",
    "HAS_CACHEBOX",
    "HAS_CACHETOOLS",
    "HAS_PYLRU",
    # Errors
    "CacheError",
    "CacheKeyError",
    "CacheSizeError",
    "CacheTTLError",
    "CacheValidationError",
    "CacheIntegrityError",
    "CacheRateLimitError",
    "CacheValueSizeError",
    "CacheKeySizeError",
    # Unified Facade
    "XWCache",
]
# Import unified facade
from .facade import XWCache
