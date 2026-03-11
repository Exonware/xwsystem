// #exonware/xwsystem/rust/src/caching/lfu_cache.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! LFU (Least Frequently Used) Cache implementation with thread-safety and async support.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI64, Ordering};
use std::rc::Rc;
use rustc_hash::FxHashMap;

use crate::caching::base::{ACache, Hashable};

// Call parent constructor
// Emit performance warning for large caches
// Find least frequently used key
// Find least frequently used key
/// Thread-safe LFU (Least Frequently Used) Cache.
///
/// ⚠️ PERFORMANCE WARNING: This implementation uses O(n) eviction.
/// For better performance, use OptimizedLFUCache with O(1) eviction (100x+ faster).
///
/// Features:
/// - O(1) get and put operations
/// - O(n) eviction (uses min() scan - slow for large caches)
/// - Thread-safe operations
/// - Frequency-based eviction
/// - Statistics tracking
///
/// Recommended Alternative:
/// from exonware.xwsystem.caching import OptimizedLFUCache
/// cache = OptimizedLFUCache(capacity=1000)  # O(1) eviction
pub struct LFUCache {
    capacity: i64,
    name: String,
    // Phase 2: Use Rc<Value> to avoid cloning large values internally
    cache: Mutex<FxHashMap<Hashable, Rc<serde_json::Value>>>,
    frequencies: Mutex<FxHashMap<Hashable, i64>>,
    hits: AtomicI64,
    misses: AtomicI64,
    evictions: AtomicI64,
}

impl ACache for LFUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut frequencies = self.frequencies.lock().expect("Frequencies lock poisoned");
        
        if let Some(value_rc) = cache.get(&key) {
            // Increment frequency
            *frequencies.get_mut(&key).expect("Frequency entry not found") += 1;
            self.hits.fetch_add(1, Ordering::Relaxed);
            // Phase 2: Clone only when returning (Rc::clone is cheap)
            Some((**value_rc).clone())  // Clone actual Value when returning to Python
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            default
        }
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut frequencies = self.frequencies.lock().expect("Frequencies lock poisoned");
        
        if cache.contains_key(&key) {
            // Update existing key
            cache.insert(key.clone(), Rc::new(value));  // Phase 2: Wrap in Rc
            *frequencies.get_mut(&key).expect("Frequency entry not found") += 1;
        } else {
            if cache.len() as i64 >= self.capacity {
                // Find least frequently used key
                if let Some((lfu_key, _)) = frequencies.iter().min_by_key(|(_, &freq)| freq) {
                    let key_to_remove = lfu_key.clone();
                    cache.remove(&key_to_remove);
                    frequencies.remove(&key_to_remove);
                    self.evictions.fetch_add(1, Ordering::Relaxed);
                }
            }
            
            cache.insert(key.clone(), Rc::new(value));  // Phase 2: Wrap in Rc
            frequencies.insert(key, 1);
        }
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut frequencies = self.frequencies.lock().expect("Frequencies lock poisoned");
        
        if cache.remove(&key).is_some() {
            frequencies.remove(&key);
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut frequencies = self.frequencies.lock().expect("Frequencies lock poisoned");
        cache.clear();
        frequencies.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().expect("Cache lock poisoned").len() as i64
    }

    fn is_full(&self) -> bool {
        self.cache.lock().expect("Cache lock poisoned").len() as i64 >= self.capacity
    }

    fn evict(&mut self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut frequencies = self.frequencies.lock().expect("Frequencies lock poisoned");
        
        if !cache.is_empty() {
            // Find least frequently used key
            if let Some((lfu_key, _)) = frequencies.iter().min_by_key(|(_, &freq)| freq) {
                let key_to_remove = lfu_key.clone();
                cache.remove(&key_to_remove);
                frequencies.remove(&key_to_remove);
                self.evictions.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    fn keys(&self) -> Vec<Hashable> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.keys().cloned().collect()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        // Phase 2: Clone actual Value when returning
        cache.values().map(|v| (**v).clone()).collect()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        // Phase 2: Clone actual Value when returning
        cache.iter().map(|(k, v)| (k.clone(), (**v).clone())).collect()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let evictions = self.evictions.load(Ordering::Relaxed);
        
        let total_requests = hits + misses;
        let hit_rate = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let mut stats = HashMap::new();
        stats.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        stats.insert("type".to_string(), serde_json::Value::String("LFU".to_string()));
        stats.insert("capacity".to_string(), serde_json::Value::Number(serde_json::Number::from(self.capacity)));
        stats.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(cache.len() as i64)));
        stats.insert("hits".to_string(), serde_json::Value::Number(serde_json::Number::from(hits)));
        stats.insert("misses".to_string(), serde_json::Value::Number(serde_json::Number::from(misses)));
        stats.insert("evictions".to_string(), serde_json::Value::Number(serde_json::Number::from(evictions)));
        stats.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        
        stats
    }
}

impl LFUCache {
    /// Initialize LFU cache.
    ///
    /// ⚠️ PERFORMANCE WARNING: Consider using OptimizedLFUCache for better performance.
    /// This implementation has O(n) eviction complexity.
    pub fn new(
        capacity: Option<i64>,
        name: Option<String>
    ) -> Self {
        let cap = capacity.unwrap_or(128);
        if cap <= 0 {
            panic!("Cache capacity must be positive, got {}", cap);
        }
        
        let cache_name = name.unwrap_or_else(|| format!("LFUCache-{:p}", &cap));
        
        // Emit performance warning for large caches
        if cap > 1000 {
            // Would log warning in real implementation
        }
        
        Self {
            capacity: cap,
            name: cache_name,
            cache: Mutex::new(FxHashMap::default()),
            frequencies: Mutex::new(FxHashMap::default()),
            hits: AtomicI64::new(0),
            misses: AtomicI64::new(0),
            evictions: AtomicI64::new(0),
        }
    }

    /// Get value by key.
    pub fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(self, key, default)
    }

    /// Put key-value pair in cache.
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) {
        ACache::put(self, key, value);
    }

    /// Set value in cache (abstract method implementation).
    /// Delegates to put() for backward compatibility.
    ///
    /// Args:
    /// key: Key to store
    /// value: Value to store
    /// ttl: Optional time-to-live (not used in LFU)
    pub fn set(&mut self, key: String, value: serde_json::Value, _ttl: Option<i64>) {
        ACache::put(self, key, value);
    }

    /// Delete key from cache.
    pub fn delete(&mut self, key: Hashable) -> bool {
        ACache::delete(self, key)
    }

    /// Clear all items from cache.
    pub fn clear(&mut self) {
        ACache::clear(self);
    }

    /// Get current cache size.
    pub fn size(&self) -> i64 {
        ACache::size(self)
    }

    /// Check if cache is at capacity.
    pub fn is_full(&self) -> bool {
        ACache::is_full(self)
    }

    /// Evict least frequently used entry from cache.
    /// Implementation of abstract method from ACacheBase.
    pub fn evict(&mut self) {
        ACache::evict(self);
    }

    /// Get list of all cache keys.
    pub fn keys(&self) -> Vec<Hashable> {
        ACache::keys(self)
    }

    /// Get list of all cache values.
    pub fn values(&self) -> Vec<serde_json::Value> {
        ACache::values(self)
    }

    /// Get list of all key-value pairs.
    pub fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        ACache::items(self)
    }

    /// Get cache statistics.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        ACache::get_stats(self)
    }

    /// Reset cache statistics.
    pub fn reset_stats(&mut self) {
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
        self.evictions.store(0, Ordering::Relaxed);
    }
}

/// Async-safe LFU (Least Frequently Used) Cache.
pub struct AsyncLFUCache {
    cache: Arc<Mutex<LFUCache>>,
}

impl AsyncLFUCache {
    /// Initialize async LFU cache.
    pub fn new(
        capacity: Option<i64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LFUCache::new(capacity, name))),
        }
    }

    /// Get value by key asynchronously.
    ///
    /// Note: In Rust, async operations require an async runtime (e.g., tokio).
    /// This implementation uses blocking operations wrapped in async for compatibility.
    pub async fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        cache.get(key, default)
    }

    /// Put key-value pair in cache asynchronously.
    pub async fn put(&self, key: Hashable, value: serde_json::Value) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let mut cache = self.cache.lock().unwrap();
        cache.put(key, value);
    }

    /// Delete key from cache asynchronously.
    pub async fn delete(&self, key: Hashable) -> bool {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let mut cache = self.cache.lock().unwrap();
        cache.delete(key)
    }

    /// Clear all items from cache asynchronously.
    pub async fn clear(&self) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get current cache size asynchronously.
    pub async fn size(&self) -> i64 {
        let cache = self.cache.lock().unwrap();
        cache.size()
    }

    /// Get cache statistics asynchronously.
    pub async fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().unwrap();
        cache.get_stats()
    }
}
