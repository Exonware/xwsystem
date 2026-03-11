// #exonware/xwsystem/rust/src/caching/base.rs
//exonware/xwsystem/caching/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Caching module base classes - abstract classes for caching functionality.


use std::collections::HashMap;

use crate::caching::defs::CachePolicy;

/// Type alias for hashable cache keys.
/// In Rust, we use String for cache keys for simplicity and consistency.
pub type Hashable = String;

// Default implementation - subclasses can override for efficiency
// Continue with other items even if one fails
/// Abstract base class for all cache implementations.
pub trait ACache {
    /// Get value from cache.
    /// Args:
    /// key: Cache key (Hashable)
    /// default: Default value if key not found
    /// Returns:
    /// Cached value or default
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value>;

    /// Put value in cache.
    /// Args:
    /// key: Cache key (Hashable)
    /// value: Value to cache
    fn put(&mut self, key: Hashable, value: serde_json::Value);

    /// Delete value from cache.
    /// Args:
    /// key: Cache key to delete
    /// Returns:
    /// True if key was deleted, False if not found
    fn delete(&mut self, key: Hashable) -> bool;

    /// Clear all cache entries.
    fn clear(&mut self);

    /// Get cache size.
    fn size(&self) -> i64;

    /// Check if cache is full.
    fn is_full(&self) -> bool;

    /// Evict entries from cache.
    fn evict(&mut self);

    /// Get list of all cache keys.
    /// Returns:
    /// List of cache keys (Hashable objects)
    fn keys(&self) -> Vec<Hashable>;

    /// Get list of all cache values.
    /// Returns:
    /// List of cache values
    fn values(&self) -> Vec<serde_json::Value>;

    /// Get list of all key-value pairs.
    /// Returns:
    /// List of (key, value) tuples
    fn items(&self) -> Vec<(Hashable, serde_json::Value)>;

    /// Get cache statistics.
    /// Returns:
    /// Dictionary with statistics (hits, misses, hit_rate, etc.)
    fn get_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Get multiple values in a single operation.
    /// Args:
    /// keys: List of keys to retrieve
    /// Returns:
    /// Dictionary of key-value pairs found in cache
    /// Note:
    /// More efficient than individual gets for batch operations.
    fn get_many(&self, keys: &[Hashable]) -> HashMap<Hashable, serde_json::Value> {
        let mut results = HashMap::new();
        for key in keys {
            if let Some(value) = self.get(key.clone(), None) {
                results.insert(key.clone(), value);
            }
        }
        results
    }

    /// Put multiple key-value pairs in a single operation.
    /// Args:
    /// items: Dictionary of key-value pairs to cache
    /// Returns:
    /// Number of items successfully cached
    /// Note:
    /// More efficient than individual puts for batch operations.
    fn put_many(&mut self, items: &HashMap<Hashable, serde_json::Value>) -> i64 {
        let mut count = 0;
        for (key, value) in items {
            self.put(key.clone(), value.clone());
            count += 1;
        }
        count
    }

    /// Delete multiple keys in a single operation.
    /// Args:
    /// keys: List of keys to delete
    /// Returns:
    /// Number of keys successfully deleted
    /// Note:
    /// More efficient than individual deletes for batch operations.
    fn delete_many(&mut self, keys: &[Hashable]) -> i64 {
        let mut count = 0;
        for key in keys {
            if self.delete(key.clone()) {
                count += 1;
            }
        }
        count
    }

}

/// Abstract base class for cache management.
pub trait ACacheManager {
    /// Create a new cache instance.
    fn create_cache(&mut self, name: &str, cache_type: &str) -> Box<dyn ACache>;

    /// Get cache instance by name.
    fn get_cache(&self, name: &str) -> Option<&dyn ACache>;

    /// Remove cache instance.
    fn remove_cache(&mut self, name: &str) -> bool;

    /// List all cache names.
    fn list_caches(&self) -> Vec<String>;

    /// Clear all caches.
    fn clear_all_caches(&mut self);
}

/// Abstract base class for distributed cache implementations.
pub trait ADistributedCache {
    /// Connect to distributed cache nodes.
    fn connect(&mut self, nodes: &[String]);

    /// Disconnect from distributed cache.
    fn disconnect(&mut self);

    /// Check if connected to distributed cache.
    fn is_connected(&self) -> bool;

    /// Get distributed cache node information.
    fn get_node_info(&self) -> HashMap<String, serde_json::Value>;

    /// Synchronize cache across nodes.
    fn sync(&mut self);
}

/// Abstract base class for cache decorators.
pub trait ACacheDecorator {
    /// Invalidate cache for specific arguments.
    fn invalidate(&mut self);

    /// Clear all cached results.
    fn clear(&mut self);
}
