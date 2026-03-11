// #exonware/xwsystem/rust/src/caching/read_through.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Read-through and Write-through cache implementations.
//! Extensibility Priority #5 - Auto-loading and auto-writing patterns.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;

// Cache miss - try loader
// Cache the loaded value
// Fall through to default
/// Read-through cache that automatically loads missing values.
///
/// When a key is not found in cache, automatically calls the loader function
/// to fetch the value and cache it.
///
/// Example:
/// ```rust
/// let loader = |key: &Hashable| -> Option<serde_json::Value> {
///     // Load from database
///     Some(serde_json::json!("loaded_value"))
/// };
///
/// let mut cache = ReadThroughCache::new(Some(1000), Some(Arc::new(loader)), None, None);
/// // Automatically loads from DB on cache miss
/// let user = cache.get("user:123".to_string(), None);
/// ```
pub struct ReadThroughCache {
    cache: Mutex<LRUCache>,
    loader: Option<Arc<dyn Fn(&Hashable) -> Option<serde_json::Value> + Send + Sync>>,
    loader_calls: Mutex<i64>,
}

impl ACache for ReadThroughCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Try cache first
        let cache_guard = self.cache.lock().unwrap();
        if let Some(value) = cache_guard.get(key.clone(), None) {
            return Some(value);
        }
        drop(cache_guard);
        
        // Cache miss - try loader
        if let Some(loader) = &self.loader {
            if let Some(loaded_value) = loader(&key) {
                *self.loader_calls.lock().unwrap() += 1;
                // Cache the loaded value
                let mut cache_guard = self.cache.lock().unwrap();
                cache_guard.put(key, loaded_value.clone());
                return Some(loaded_value);
            }
        }
        
        // Fall through to default
        default
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.put(key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.delete(key)
    }

    fn clear(&mut self) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.lock().unwrap().keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.lock().unwrap().values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.lock().unwrap().items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = self.cache.lock().unwrap().get_stats();
        let loader_calls = *self.loader_calls.lock().unwrap();
        stats.insert("loader_calls".to_string(), serde_json::Value::Number(serde_json::Number::from(loader_calls)));
        stats
    }
}

impl ReadThroughCache {
    /// Initialize read-through cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// loader: Function to load missing values (key) -> value
    /// ttl: Optional TTL in seconds
    /// name: Cache name
    pub fn new<F>(
        capacity: Option<i64>,
        loader: Option<F>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self
    where
        F: Fn(&Hashable) -> Option<serde_json::Value> + Send + Sync + 'static,
    {
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            loader: loader.map(|f| Arc::new(f) as Arc<dyn Fn(&Hashable) -> Option<serde_json::Value> + Send + Sync>),
            loader_calls: Mutex::new(0),
        }
    }

    /// Get value with automatic loading on miss.
    ///
    /// Args:
    /// key: Cache key
    /// default: Default if loader fails or not provided
    ///
    /// Returns:
    /// Cached or loaded value
    pub fn get(&mut self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(self, key, default)
    }
}

/// Write-through cache that automatically persists writes.
///
/// When a value is put in cache, automatically calls the writer function
/// to persist it to storage.
pub struct WriteThroughCache {
    cache: Mutex<LRUCache>,
    writer: Option<Arc<dyn Fn(&Hashable, &serde_json::Value) + Send + Sync>>,
    writer_calls: Mutex<i64>,
    writer_errors: Mutex<i64>,
}

impl ACache for WriteThroughCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.lock().unwrap().get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        // Persist first (write-through)
        if let Some(writer) = &self.writer {
            (writer)(&key, &value);
            *self.writer_calls.lock().unwrap() += 1;
        }
        
        // Then cache
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.put(key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.delete(key)
    }

    fn clear(&mut self) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.lock().unwrap().keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.lock().unwrap().values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.lock().unwrap().items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = self.cache.lock().unwrap().get_stats();
        let writer_calls = *self.writer_calls.lock().unwrap();
        let writer_errors = *self.writer_errors.lock().unwrap();
        stats.insert("writer_calls".to_string(), serde_json::Value::Number(serde_json::Number::from(writer_calls)));
        stats.insert("writer_errors".to_string(), serde_json::Value::Number(serde_json::Number::from(writer_errors)));
        stats
    }
}

impl WriteThroughCache {
    /// Initialize write-through cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// writer: Function to persist values (key, value) -> None
    /// ttl: Optional TTL in seconds
    /// name: Cache name
    pub fn new<F>(
        capacity: Option<i64>,
        writer: Option<F>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self
    where
        F: Fn(&Hashable, &serde_json::Value) + Send + Sync + 'static,
    {
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            writer: writer.map(|f| Arc::new(f) as Arc<dyn Fn(&Hashable, &serde_json::Value) + Send + Sync>),
            writer_calls: Mutex::new(0),
            writer_errors: Mutex::new(0),
        }
    }

    /// Put value with automatic persistence.
    ///
    /// Args:
    /// key: Cache key
    /// value: Value to cache and persist
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) {
        ACache::put(self, key, value);
    }
}

/// Combined read-through and write-through cache.
///
/// Automatically loads on miss and persists on write.
pub struct ReadWriteThroughCache {
    cache: Mutex<LRUCache>,
    loader: Option<Arc<dyn Fn(&Hashable) -> Option<serde_json::Value> + Send + Sync>>,
    writer: Option<Arc<dyn Fn(&Hashable, &serde_json::Value) + Send + Sync>>,
    loader_calls: Mutex<i64>,
    writer_calls: Mutex<i64>,
    writer_errors: Mutex<i64>,
}

impl ACache for ReadWriteThroughCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Try cache first
        let cache_guard = self.cache.lock().unwrap();
        if let Some(value) = cache_guard.get(key.clone(), None) {
            return Some(value);
        }
        drop(cache_guard);
        
        // Cache miss - try loader
        if let Some(loader) = &self.loader {
            if let Some(loaded_value) = loader(&key) {
                *self.loader_calls.lock().unwrap() += 1;
                // Cache the loaded value
                let mut cache_guard = self.cache.lock().unwrap();
                cache_guard.put(key, loaded_value.clone());
                return Some(loaded_value);
            }
        }
        
        // Fall through to default
        default
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        // Persist first (write-through)
        if let Some(writer) = &self.writer {
            (writer)(&key, &value);
            *self.writer_calls.lock().unwrap() += 1;
        }
        
        // Then cache
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.put(key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.delete(key)
    }

    fn clear(&mut self) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        let mut cache_guard = self.cache.lock().unwrap();
        cache_guard.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.lock().unwrap().keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.lock().unwrap().values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.lock().unwrap().items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = self.cache.lock().unwrap().get_stats();
        let loader_calls = *self.loader_calls.lock().unwrap();
        let writer_calls = *self.writer_calls.lock().unwrap();
        let writer_errors = *self.writer_errors.lock().unwrap();
        stats.insert("loader_calls".to_string(), serde_json::Value::Number(serde_json::Number::from(loader_calls)));
        stats.insert("writer_calls".to_string(), serde_json::Value::Number(serde_json::Number::from(writer_calls)));
        stats.insert("writer_errors".to_string(), serde_json::Value::Number(serde_json::Number::from(writer_errors)));
        stats
    }
}

impl ReadWriteThroughCache {
    /// Initialize read-write-through cache.
    pub fn new<L, W>(
        capacity: Option<i64>,
        loader: Option<L>,
        writer: Option<W>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self
    where
        L: Fn(&Hashable) -> Option<serde_json::Value> + Send + Sync + 'static,
        W: Fn(&Hashable, &serde_json::Value) + Send + Sync + 'static,
    {
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            loader: loader.map(|f| Arc::new(f) as Arc<dyn Fn(&Hashable) -> Option<serde_json::Value> + Send + Sync>),
            writer: writer.map(|f| Arc::new(f) as Arc<dyn Fn(&Hashable, &serde_json::Value) + Send + Sync>),
            loader_calls: Mutex::new(0),
            writer_calls: Mutex::new(0),
            writer_errors: Mutex::new(0),
        }
    }

    /// Get with auto-loading.
    pub fn get(&mut self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(self, key, default)
    }

    /// Put with auto-persistence.
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) {
        ACache::put(self, key, value);
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
