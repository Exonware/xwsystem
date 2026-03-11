// #exonware/xwsystem/rust/src/caching/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Caching protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::caching::defs::{CacheEvent, CacheLevel, CachePolicy, CacheStatus};

// ============================================================================

// CACHE MANAGER INTERFACES

// ============================================================================

// ============================================================================

// CACHE STORAGE INTERFACES

// ============================================================================

// ============================================================================

// CACHE EVICTION INTERFACES

// ============================================================================

// ============================================================================

// CACHE MONITORING INTERFACES

// ============================================================================

// ============================================================================

// DISTRIBUTED CACHE INTERFACES

// ============================================================================

// ============================================================================

// CACHE DECORATOR INTERFACES

// ============================================================================

// ============================================================================

// CACHE PERSISTENCE INTERFACES

// ============================================================================

// ============================================================================

// BASIC CACHE INTERFACE

// ============================================================================

// ============================================================================

// CACHING PROTOCOLS

// ============================================================================

/// Interface for cacheable objects.
///
/// Enforces consistent caching behavior across XWSystem.
pub trait ICacheable {
    /// Cache a value with key.
    /// Args:
    /// key: Cache key
    /// value: Value to cache
    /// ttl: Time to live in seconds
    /// Returns:
    /// True if cached successfully
    fn cache(&mut self, key: &str, value: serde_json::Value, ttl: Option<i64>) -> bool;

    /// Get cached value by key.
    /// Args:
    /// key: Cache key
    /// default: Default value if not found
    /// Returns:
    /// Cached value or default
    fn get_cached(&self, key: &str, default: serde_json::Value) -> serde_json::Value;

    /// Clear all cached values.
    fn clear_cache(&mut self);

    /// Check if key is cached.
    /// Args:
    /// key: Cache key to check
    /// Returns:
    /// True if cached
    fn has_cached(&self, key: &str) -> bool;

    /// Remove cached value by key.
    /// Args:
    /// key: Cache key to remove
    /// Returns:
    /// True if removed
    fn remove_cached(&mut self, key: &str) -> bool;

    /// Get number of cached items.
    /// Returns:
    /// Number of cached items
    fn get_cache_size(&self) -> i64;

    /// Get cache information.
    /// Returns:
    /// Cache information dictionary
    fn get_cache_info(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for cache management.
///
/// Enforces consistent cache management across XWSystem.
pub trait ICacheManager {
    /// Create a new cache.
    /// Args:
    /// name: Cache name
    /// max_size: Maximum cache size
    /// policy: Eviction policy
    /// Returns:
    /// Cache instance
    fn create_cache(&mut self, name: &str, max_size: i64, policy: CachePolicy) -> Box<dyn ICacheable>;

    /// Get cache by name.
    /// Args:
    /// name: Cache name
    /// Returns:
    /// Cache instance or None
    fn get_cache(&self, name: &str) -> Option<&dyn ICacheable>;

    /// Remove cache by name.
    /// Args:
    /// name: Cache name to remove
    /// Returns:
    /// True if removed
    fn remove_cache(&mut self, name: &str) -> bool;

    /// List all cache names.
    /// Returns:
    /// List of cache names
    fn list_caches(&self) -> Vec<String>;

    /// Clear all caches.
    fn clear_all_caches(&mut self);

    /// Get statistics for all caches.
    /// Returns:
    /// Dictionary of cache statistics
    fn get_cache_stats(&self) -> HashMap<String, HashMap<String, serde_json::Value>>;

    /// Set global cache policy.
    /// Args:
    /// policy: Global eviction policy
    fn set_global_policy(&mut self, policy: CachePolicy);

    /// Get global cache policy.
    /// Returns:
    /// Global eviction policy
    fn get_global_policy(&self) -> CachePolicy;

}

/// Interface for cache storage backends.
///
/// Enforces consistent cache storage across XWSystem.
pub trait ICacheStorage {
    /// Store value in cache.
    /// Args:
    /// key: Cache key
    /// value: Value to store
    /// ttl: Time to live in seconds
    /// Returns:
    /// True if stored successfully
    fn store(&mut self, key: &str, value: serde_json::Value, ttl: Option<i64>) -> bool;

    /// Retrieve value from cache.
    /// Args:
    /// key: Cache key
    /// Returns:
    /// Cached value or None
    fn retrieve(&self, key: &str) -> Option<serde_json::Value>;

    /// Delete value from cache.
    /// Args:
    /// key: Cache key to delete
    /// Returns:
    /// True if deleted
    fn delete(&mut self, key: &str) -> bool;

    /// Check if key exists in cache.
    /// Args:
    /// key: Cache key
    /// Returns:
    /// True if exists
    fn exists(&self, key: &str) -> bool;

    /// Clear all cached values.
    fn clear(&mut self);

    /// Get number of cached items.
    /// Returns:
    /// Number of cached items
    fn size(&self) -> i64;

    /// Get iterator over cache keys.
    /// Yields:
    /// Cache keys
    fn keys(&self) -> Box<dyn Iterator<Item = String>>;

    /// Get iterator over cache values.
    /// Yields:
    /// Cache values
    fn values(&self) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Get iterator over cache items.
    /// Yields:
    /// Tuples of (key, value)
    fn items(&self) -> Box<dyn Iterator<Item = (String, serde_json::Value)>>;

}

/// Interface for cache eviction strategies.
///
/// Enforces consistent cache eviction across XWSystem.
pub trait ICacheEviction {
    /// Check if eviction is needed.
    /// Args:
    /// cache_size: Current cache size
    /// max_size: Maximum cache size
    /// Returns:
    /// True if eviction needed
    fn should_evict(&self, cache_size: i64, max_size: i64) -> bool;

    /// Select item to evict.
    /// Args:
    /// items: List of (key, value, metadata) tuples
    /// Returns:
    /// Key of item to evict
    fn select_eviction_candidate(&self, items: &[(String, serde_json::Value, f64)]) -> String;

    /// Update access information for key.
    /// Args:
    /// key: Cache key
    /// timestamp: Access timestamp
    fn update_access(&mut self, key: &str, timestamp: f64);

    /// Get eviction policy.
    /// Returns:
    /// Eviction policy
    fn get_eviction_policy(&self) -> CachePolicy;

    /// Set eviction policy.
    /// Args:
    /// policy: Eviction policy to set
    fn set_eviction_policy(&mut self, policy: CachePolicy);

    /// Get eviction statistics.
    /// Returns:
    /// Eviction statistics dictionary
    fn get_eviction_stats(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for cache monitoring.
///
/// Enforces consistent cache monitoring across XWSystem.
pub trait ICacheMonitor {
    /// Record cache hit.
    /// Args:
    /// key: Cache key that was hit
    fn record_hit(&mut self, key: &str);

    /// Record cache miss.
    /// Args:
    /// key: Cache key that was missed
    fn record_miss(&mut self, key: &str);

    /// Record cache set operation.
    /// Args:
    /// key: Cache key that was set
    /// size: Size of cached value
    fn record_set(&mut self, key: &str, size: i64);

    /// Record cache delete operation.
    /// Args:
    /// key: Cache key that was deleted
    fn record_delete(&mut self, key: &str);

    /// Record cache eviction.
    /// Args:
    /// key: Cache key that was evicted
    /// reason: Eviction reason
    fn record_eviction(&mut self, key: &str, reason: &str);

    /// Get cache hit rate.
    /// Returns:
    /// Hit rate as percentage (0.0 to 1.0)
    fn get_hit_rate(&self) -> f64;

    /// Get cache miss rate.
    /// Returns:
    /// Miss rate as percentage (0.0 to 1.0)
    fn get_miss_rate(&self) -> f64;

    /// Get cache statistics.
    /// Returns:
    /// Statistics dictionary
    fn get_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Reset cache statistics.
    fn reset_stats(&mut self);

}

/// Interface for distributed cache operations.
///
/// Enforces consistent distributed caching across XWSystem.
pub trait IDistributedCache {
    /// Get current node ID.
    /// Returns:
    /// Node identifier
    fn get_node_id(&self) -> String;

    /// Get list of cluster nodes.
    /// Returns:
    /// List of node IDs
    fn get_cluster_nodes(&self) -> Vec<String>;

    /// Replicate cache entry to nodes.
    /// Args:
    /// key: Cache key
    /// value: Cache value
    /// nodes: Target nodes
    /// Returns:
    /// True if replicated successfully
    fn replicate(&mut self, key: &str, value: &serde_json::Value, nodes: &[String]) -> bool;

    /// Invalidate cache entry on nodes.
    /// Args:
    /// key: Cache key to invalidate
    /// nodes: Target nodes
    /// Returns:
    /// True if invalidated successfully
    fn invalidate(&mut self, key: &str, nodes: &[String]) -> bool;

    /// Sync cache with specific node.
    /// Args:
    /// node_id: Node to sync with
    /// Returns:
    /// True if synced successfully
    fn sync_with_node(&mut self, node_id: &str) -> bool;

    /// Get cache consistency level.
    /// Returns:
    /// Consistency level (e.g., 'strong', 'eventual')
    fn get_consistency_level(&self) -> String;

    /// Set cache consistency level.
    /// Args:
    /// level: Consistency level
    fn set_consistency_level(&mut self, level: &str);

}

/// Interface for cache decorators.
///
/// Enforces consistent cache decoration across XWSystem.
pub trait ICacheDecorator {
    /// Decorate function to cache results.
    /// Args:
    /// func: Function to decorate
    /// ttl: Time to live in seconds
    /// key_func: Function to generate cache key
    /// Returns:
    /// Decorated function
    fn cache_result(&self, func: fn(), ttl: Option<i64>, key_func: Option<fn()>) -> fn();

    /// Decorate function to invalidate cache.
    /// Args:
    /// func: Function to decorate
    /// key_func: Function to generate cache key
    /// Returns:
    /// Decorated function
    fn cache_invalidate(&self, func: fn(), key_func: Option<fn()>) -> fn();

    /// Decorate function to clear cache.
    /// Args:
    /// func: Function to decorate
    /// Returns:
    /// Decorated function
    fn cache_clear(&self, func: fn()) -> fn();

    /// Decorate function with conditional caching.
    /// Args:
    /// func: Function to decorate
    /// condition: Condition function for caching
    /// Returns:
    /// Decorated function
    fn cache_conditional(&self, func: fn(), condition: fn()) -> fn();

}

/// Interface for cache persistence.
///
/// Enforces consistent cache persistence across XWSystem.
pub trait ICachePersistence {
    /// Save cache to file.
    /// Args:
    /// cache_name: Name of cache to save
    /// file_path: File path to save to
    /// Returns:
    /// True if saved successfully
    fn save_cache(&self, cache_name: &str, file_path: &str) -> bool;

    /// Load cache from file.
    /// Args:
    /// cache_name: Name of cache to load
    /// file_path: File path to load from
    /// Returns:
    /// True if loaded successfully
    fn load_cache(&mut self, cache_name: &str, file_path: &str) -> bool;

    /// Backup cache to file.
    /// Args:
    /// cache_name: Name of cache to backup
    /// backup_path: Backup file path
    /// Returns:
    /// True if backed up successfully
    fn backup_cache(&self, cache_name: &str, backup_path: &str) -> bool;

    /// Restore cache from backup.
    /// Args:
    /// cache_name: Name of cache to restore
    /// backup_path: Backup file path
    /// Returns:
    /// True if restored successfully
    fn restore_cache(&mut self, cache_name: &str, backup_path: &str) -> bool;

    /// Get persistence format.
    /// Returns:
    /// Format name (e.g., 'pickle', 'json')
    fn get_persistence_format(&self) -> String;

    /// Set persistence format.
    /// Args:
    /// format_name: Format name
    fn set_persistence_format(&mut self, format_name: &str);

}

/// Basic cache interface for disk-based and specialized caches.
///
/// Root cause fixed: Added missing ICache interface that was being used by
/// TwoTierCache and DiskCache but didn't exist in contracts.
///
/// This is a simpler interface than ICacheable, designed for caches that
/// use string keys (like disk caches and two-tier caches).
pub trait ICache {
    /// Get value from cache.
    fn get(&self, key: &str) -> Option<serde_json::Value>;

    /// Set value in cache.
    fn set(&mut self, key: &str, value: serde_json::Value, ttl: Option<i64>) -> bool;

    /// Delete value from cache.
    fn delete(&mut self, key: &str) -> bool;

    /// Clear all cached values.
    fn clear(&mut self) -> bool;

    /// Check if key exists in cache.
    fn exists(&self, key: &str) -> bool;

    /// Get number of cached items.
    fn size(&self) -> i64;

}

/// Protocol for objects that support caching (simpler interface than ICacheable).
pub trait ICacheableSimple {
    /// Get value from cache.
    fn get(&self, key: &str, default: serde_json::Value) -> serde_json::Value;

    /// Set value in cache.
    fn set(&mut self, key: &str, value: serde_json::Value, ttl: Option<i64>);

    /// Delete value from cache.
    fn delete(&mut self, key: &str) -> bool;

    /// Clear all cached values.
    fn clear(&mut self);

}
