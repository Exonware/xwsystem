// #exonware/xwsystem/rust/src/caching/fluent.rs
//exonware/xwsystem/caching/fluent.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Fluent API wrappers for caching module.
//! Usability Priority #2 - Method chaining for better developer experience.


use std::collections::HashMap;
use crate::caching::base::{ACache, Hashable};
use crate::caching::lfu_cache::LFUCache;
use crate::caching::lru_cache::LRUCache;
use crate::caching::ttl_cache::TTLCache;

/// LRU Cache with fluent API for method chaining.
///
/// Example:
/// ```rust
/// let mut cache = FluentLRUCache::new(Some(100), None, None);
/// cache.put("k1".to_string(), serde_json::json!("v1"))
///      .put("k2".to_string(), serde_json::json!("v2"))
///      .put("k3".to_string(), serde_json::json!("v3"));
///
/// let value = cache.get("k1".to_string(), None);  // Still returns the value, not self
///
/// cache.delete("k1".to_string())
///      .delete("k2".to_string())
///      .clear();  // Chain operations
/// ```
pub struct FluentLRUCache {
    cache: LRUCache,
}

impl ACache for FluentLRUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Note: get() doesn't need mutability in ACache trait, but LRUCache.get() might
        // For now, we'll use a workaround - in real implementation, cache should be Arc<RwLock<>>
        self.cache.get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        self.cache.put(key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        self.cache.delete(key)
    }

    fn clear(&mut self) {
        self.cache.clear();
    }

    fn size(&self) -> i64 {
        self.cache.size()
    }

    fn is_full(&self) -> bool {
        self.cache.is_full()
    }

    fn evict(&mut self) {
        self.cache.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        self.cache.get_stats()
    }
}

impl FluentLRUCache {
    /// Create a new FluentLRUCache.
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: LRUCache::new(capacity, ttl, name),
        }
    }

    /// Get value by key (returns value, not self for chaining).
    pub fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.get(key, default)
    }

    /// Put value and return self for chaining.
    ///
    /// Args:
    /// key: Cache key
    /// value: Value to cache
    ///
    /// Returns:
    /// Self for method chaining
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) -> &mut Self {
        self.cache.put(key, value);
        self
    }

    /// Delete key and return self for chaining.
    ///
    /// Args:
    /// key: Cache key to delete
    ///
    /// Returns:
    /// Self for method chaining
    pub fn delete(&mut self, key: Hashable) -> &mut Self {
        self.cache.delete(key);
        self
    }

    /// Clear cache and return self for chaining.
    ///
    /// Returns:
    /// Self for method chaining
    pub fn clear(&mut self) -> &mut Self {
        self.cache.clear();
        self
    }
}

/// LFU Cache with fluent API for method chaining.
///
/// Example:
/// ```rust
/// let mut cache = FluentLFUCache::new(Some(100), None);
/// cache.put("k1".to_string(), serde_json::json!("v1"))
///      .put("k2".to_string(), serde_json::json!("v2"))
///      .put("k3".to_string(), serde_json::json!("v3"));
/// ```
pub struct FluentLFUCache {
    cache: LFUCache,
}

impl ACache for FluentLFUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        self.cache.put(key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        self.cache.delete(key)
    }

    fn clear(&mut self) {
        self.cache.clear();
    }

    fn size(&self) -> i64 {
        self.cache.size()
    }

    fn is_full(&self) -> bool {
        self.cache.is_full()
    }

    fn evict(&mut self) {
        self.cache.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        self.cache.get_stats()
    }
}

impl FluentLFUCache {
    /// Create a new FluentLFUCache.
    pub fn new(
        capacity: Option<i64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: LFUCache::new(capacity, name),
        }
    }

    /// Get value by key (returns value, not self for chaining).
    pub fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.get(key, default)
    }

    /// Put value and return self for chaining.
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) -> &mut Self {
        self.cache.put(key, value);
        self
    }

    /// Delete key and return self for chaining.
    pub fn delete(&mut self, key: Hashable) -> &mut Self {
        self.cache.delete(key);
        self
    }

    /// Clear cache and return self for chaining.
    pub fn clear(&mut self) -> &mut Self {
        self.cache.clear();
        self
    }
}

/// TTL Cache with fluent API for method chaining.
///
/// Example:
/// ```rust
/// let mut cache = FluentTTLCache::new(Some(100), Some(300.0), None, None);
/// cache.put("k1".to_string(), serde_json::json!("v1"), None)
///      .put("k2".to_string(), serde_json::json!("v2"), Some(60.0))
///      .put("k3".to_string(), serde_json::json!("v3"), None);
/// ```
pub struct FluentTTLCache {
    cache: TTLCache,
}

impl ACache for FluentTTLCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(&self.cache, key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        // Use default TTL from cache (None means use cache's default TTL)
        let _ = self.cache.put(key, value, None);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        self.cache.delete(key)
    }

    fn clear(&mut self) {
        self.cache.clear();
    }

    fn size(&self) -> i64 {
        self.cache.size()
    }

    fn is_full(&self) -> bool {
        self.cache.is_full()
    }

    fn evict(&mut self) {
        self.cache.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        self.cache.get_stats()
    }
}

impl FluentTTLCache {
    /// Create a new FluentTTLCache.
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        cleanup_interval: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: TTLCache::new(capacity, ttl, cleanup_interval, name),
        }
    }

    /// Get value by key (returns value, not self for chaining).
    pub fn get(&self, key: String, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(&self.cache, key, default)
    }

    /// Put value and return self for chaining.
    pub fn put(&mut self, key: String, value: serde_json::Value, ttl: Option<f64>) -> &mut Self {
        let _ = self.cache.put(key, value, ttl);
        self
    }

    /// Delete key and return self for chaining.
    pub fn delete(&mut self, key: String) -> &mut Self {
        self.cache.delete(key);
        self
    }

    /// Clear cache and return self for chaining.
    pub fn clear(&mut self) -> &mut Self {
        self.cache.clear();
        self
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
