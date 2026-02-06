// #exonware/xwsystem/rust/src/caching/mod.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! XSystem Caching Package
//! 
//! Comprehensive caching framework with LRU, LFU, TTL, and advanced caching strategies.
//! Production-grade caching utilities for high-performance applications.
//! 
//! Security Features (Priority #1):
//! - Input validation and sanitization
//! - Integrity verification with checksums
//! - Rate limiting and DoS protection
//! 
//! Usability Features (Priority #2):
//! - Context manager support
//! - Fluent API for method chaining
//! - Enhanced statistics formatting
//! - XW-prefixed decorators (@xwcached, @xw_async_cached)
//! - Read-through/Write-through auto-loading
//! - Tag-based invalidation
//! 
//! Performance Features (Priority #4):
//! - O(1) LFU cache (100x+ faster eviction)
//! - Batch operations (get_many, put_many, delete_many)
//! - Memory-bounded caches
//! - Cache warming strategies
//! - Bloom filter for fast negative lookups
//! - Write-behind (lazy write) for better write performance
//! - Async iterators for async caches
//! 
//! Extensibility Features (Priority #5):
//! - Event hooks system
//! - Pluggable eviction strategies
//! - Observable caches
//! - Advanced decorators with hooks
//! - Conditional eviction policies
//! - Prometheus metrics export
//! - Serializable caches (save/load)
//! 
//! NEW in v0.0.1.388:
//! - ReadThroughCache, WriteThroughCache, ReadWriteThroughCache
//! - TaggedCache with tag-based invalidation
//! - WriteBehindCache for delayed persistence
//! - BloomFilterCache for probabilistic lookups
//! - ConditionalEvictionCache with custom eviction rules
//! - SerializableCache with save/load support
//! - PrometheusExporter and StatsCollector
//! - Async iterators (__aiter__, items_async)
//! - XW-prefixed decorators (xwcached, xw_async_cached)

// Core caches
pub mod bloom_cache;
pub mod cache_manager;
pub mod conditional;
pub mod contracts;
pub mod decorators;
pub mod distributed;
pub mod errors;
pub mod events;
pub mod eviction_strategies;
pub mod fluent;
pub mod integrity;
pub mod lfu_cache;
pub mod lfu_optimized;
pub mod lru_cache;
pub mod memory_bounded;
pub mod metrics_exporter;
pub mod observable_cache;
pub mod pluggable_cache;
pub mod rate_limiter;
pub mod read_through;
pub mod secure_cache;
pub mod serializable;
pub mod stats;
pub mod tagging;
pub mod ttl_cache;
pub mod two_tier_cache;
pub mod utils;
pub mod validation;
pub mod warming;
pub mod write_behind;

// External Rust caching libraries (optional feature)
#[cfg(feature = "external-caches")]
pub mod external_caching_rust;

// Base module must be declared before use
pub mod base;
pub mod defs;

pub use base::Hashable;

pub use bloom_cache::{BloomFilterCache, SimpleBloomFilter};

pub use cache_manager::{CacheConfig, CacheManager, CacheStats};

pub use conditional::{ConditionalEvictionCache};

pub use contracts::{ICache};

pub use decorators::{async_cache, async_cache_result, async_cached, cache, cache_result, cached, xw_async_cache, xw_async_cached, xwcache, xwcached};

pub use distributed::{DistributedCache, RedisCache};

pub use errors::{CacheError, CacheIntegrityError, CacheKeyError, CacheKeySizeError, CacheRateLimitError, CacheSizeError, CacheTTLError, CacheValidationError, CacheValueSizeError};

pub use events::{CacheEvent, CacheEventEmitter, EventLogger};

pub use eviction_strategies::{AEvictionStrategy, FIFOEvictionStrategy, LFUEvictionStrategy, LRUEvictionStrategy, RandomEvictionStrategy, SizeBasedEvictionStrategy, TTLEvictionStrategy};

pub use fluent::{FluentLFUCache, FluentLRUCache, FluentTTLCache};

pub use integrity::{CacheEntry, create_secure_entry, verify_entry_integrity};

pub use lfu_cache::{AsyncLFUCache, LFUCache};

pub use lfu_optimized::{AsyncOptimizedLFUCache, OptimizedLFUCache};

pub use lru_cache::{AsyncLRUCache, LRUCache};

pub use memory_bounded::{MemoryBoundedLFUCache, MemoryBoundedLRUCache};

pub use metrics_exporter::{PrometheusExporter, StatsCollector};

pub use observable_cache::{ObservableLFUCache, ObservableLRUCache};

pub use pluggable_cache::{PluggableCache};

pub use rate_limiter::{FixedWindowRateLimiter, RateLimiter};

pub use read_through::{ReadThroughCache, ReadWriteThroughCache, WriteThroughCache};

pub use secure_cache::{SecureLFUCache, SecureLRUCache, SecureTTLCache};

pub use serializable::{SerializableCache};

pub use stats::{format_cache_stats, format_cache_stats_table, get_stats_summary};

pub use tagging::{TaggedCache};

pub use ttl_cache::{AsyncTTLCache, TTLCache};

pub use two_tier_cache::{TwoTierCache};

pub use utils::{compute_checksum, default_key_builder, estimate_object_size, format_bytes};

pub use validation::{sanitize_key, validate_cache_key, validate_cache_value};

pub use warming::{AWarmingStrategy, LazyWarmingStrategy, PreloadWarmingStrategy, PriorityWarmingStrategy, warm_cache};

pub use write_behind::{WriteBehindCache};
