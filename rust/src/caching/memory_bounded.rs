// #exonware/xwsystem/rust/src/caching/memory_bounded.rs
//exonware/xwsystem/caching/memory_bounded.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Memory-bounded cache implementations.
//! Performance Priority #4 - Memory budget enforcement for controlled memory usage.


use std::collections::HashMap;
use std::sync::Mutex;
use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;
use crate::caching::lfu_cache::LFUCache;
use crate::caching::utils::{estimate_object_size, format_bytes};

// If single value exceeds budget, reject it
// If key exists, update memory accounting
// Evict LRU entries until we have enough memory
// Store value using parent method
// Update memory tracking
// Update memory tracking
// Delete using parent method
// Update memory tracking
/// LRU Cache with memory budget enforcement.
///
/// Evicts entries based on memory usage instead of entry count.
/// Useful for caching variable-size objects.
pub struct MemoryBoundedLRUCache {
    cache: Mutex<LRUCache>,
    memory_budget_mb: f64,
    memory_budget_bytes: i64,
    current_memory_bytes: Mutex<i64>,
    value_sizes: Mutex<HashMap<Hashable, i64>>,
}

impl ACache for MemoryBoundedLRUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.lock().unwrap().get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let value_size = estimate_object_size(&value);
        
        // If single value exceeds budget, reject it
        if value_size > self.memory_budget_bytes {
            return;
        }
        
        let mut current_memory = self.current_memory_bytes.lock().unwrap();
        let mut value_sizes = self.value_sizes.lock().unwrap();
        
        // If key exists, update memory accounting
        if let Some(old_size) = value_sizes.get(&key) {
            *current_memory -= old_size;
        }
        
        // Evict LRU entries until we have enough memory
        while *current_memory + value_size > self.memory_budget_bytes {
            let cache_size = self.cache.lock().unwrap().size();
            if cache_size == 0 {
                break;
            }
            self._evict_lru_with_memory_internal();
        }
        
        // Store value using parent method
        self.cache.get_mut().unwrap().put(key.clone(), value);
        
        // Update memory tracking
        value_sizes.insert(key, value_size);
        *current_memory += value_size;
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut value_sizes = self.value_sizes.lock().unwrap();
        if let Some(value_size) = value_sizes.remove(&key) {
            let mut current_memory = self.current_memory_bytes.lock().unwrap();
            *current_memory -= value_size;
        }
        self.cache.get_mut().unwrap().delete(key)
    }

    fn clear(&mut self) {
        self.cache.get_mut().unwrap().clear();
        self.value_sizes.lock().unwrap().clear();
        *self.current_memory_bytes.lock().unwrap() = 0;
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        let current_memory = self.current_memory_bytes.lock().unwrap();
        *current_memory >= self.memory_budget_bytes
    }

    fn evict(&mut self) {
        self._evict_lru_with_memory_internal();
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
        let memory_stats = self.get_memory_stats();
        for (k, v) in memory_stats {
            stats.insert(k, v);
        }
        stats
    }
}

impl MemoryBoundedLRUCache {
    /// Initialize memory-bounded LRU cache.
    ///
    /// Args:
    /// capacity: Maximum number of entries (fallback limit)
    /// memory_budget_mb: Memory budget in megabytes
    /// ttl: Optional TTL in seconds
    /// name: Cache name for debugging
    pub fn new(
        capacity: Option<i64>,
        memory_budget_mb: Option<f64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        let budget_mb = memory_budget_mb.unwrap_or(100.0);
        let budget_bytes = (budget_mb * 1024.0 * 1024.0) as i64;
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            memory_budget_mb: budget_mb,
            memory_budget_bytes: budget_bytes,
            current_memory_bytes: Mutex::new(0),
            value_sizes: Mutex::new(HashMap::new()),
        }
    }

    /// Evict LRU entry and update memory tracking.
    fn _evict_lru_with_memory_internal(&self) {
        let items = self.cache.lock().unwrap().items();
        if items.is_empty() {
            return;
        }
        
        // Get LRU key (first item in items list is LRU)
        let lru_key = items[0].0.clone();
        
        // Update memory tracking
        let mut value_sizes = self.value_sizes.lock().unwrap();
        let mut current_memory = self.current_memory_bytes.lock().unwrap();
        if let Some(value_size) = value_sizes.remove(&lru_key) {
            *current_memory -= value_size;
        }
        drop(value_sizes);
        drop(current_memory);
        
        // Evict from cache
        let mut cache = self.cache.lock().unwrap();
        cache.delete(lru_key);
    }

    /// Get memory-specific statistics.
    pub fn get_memory_stats(&self) -> HashMap<String, serde_json::Value> {
        let current_memory = *self.current_memory_bytes.lock().unwrap();
        let cache_size = self.cache.lock().unwrap().size();
        
        let memory_used_pct = if self.memory_budget_bytes > 0 {
            (current_memory as f64 / self.memory_budget_bytes as f64 * 100.0) as f64
        } else {
            0.0
        };
        
        let avg_value_size_bytes = if cache_size > 0 {
            current_memory as f64 / cache_size as f64
        } else {
            0.0
        };
        
        let mut stats = HashMap::new();
        stats.insert("memory_budget_mb".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.memory_budget_mb).unwrap()));
        stats.insert("memory_budget_bytes".to_string(), serde_json::Value::Number(serde_json::Number::from(self.memory_budget_bytes)));
        stats.insert("current_memory_bytes".to_string(), serde_json::Value::Number(serde_json::Number::from(current_memory)));
        stats.insert("memory_used_pct".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(memory_used_pct).unwrap()));
        stats.insert("avg_value_size_bytes".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(avg_value_size_bytes).unwrap()));
        stats
    }
}

// Update memory for existing key
// Evict until we have space
// Update memory tracking
// Update memory tracking
/// Optimized O(1) LFU Cache with memory budget enforcement.
///
/// Combines frequency-based eviction with memory limits.
/// Note: Uses LFUCache instead of OptimizedLFUCache since OptimizedLFUCache is not yet fully implemented.
pub struct MemoryBoundedLFUCache {
    cache: Mutex<LFUCache>,
    memory_budget_mb: f64,
    memory_budget_bytes: i64,
    current_memory_bytes: Mutex<i64>,
    value_sizes: Mutex<HashMap<Hashable, i64>>,
}

impl ACache for MemoryBoundedLFUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.lock().unwrap().get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let value_size = estimate_object_size(&value);
        
        // If single value exceeds budget, reject it
        if value_size > self.memory_budget_bytes {
            return;
        }
        
        let mut current_memory = self.current_memory_bytes.lock().unwrap();
        let mut value_sizes = self.value_sizes.lock().unwrap();
        
        // Update memory for existing key
        if let Some(old_size) = value_sizes.get(&key) {
            *current_memory -= old_size;
        }
        
        // Evict until we have space
        while *current_memory + value_size > self.memory_budget_bytes {
            let cache_size = self.cache.lock().unwrap().size();
            if cache_size == 0 {
                break;
            }
            self._evict_lfu_with_memory_internal();
        }
        
        // Store using parent
        self.cache.get_mut().unwrap().put(key.clone(), value);
        
        // Update memory tracking
        value_sizes.insert(key, value_size);
        *current_memory += value_size;
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut value_sizes = self.value_sizes.lock().unwrap();
        if let Some(value_size) = value_sizes.remove(&key) {
            let mut current_memory = self.current_memory_bytes.lock().unwrap();
            *current_memory -= value_size;
        }
        self.cache.get_mut().unwrap().delete(key)
    }

    fn clear(&mut self) {
        self.cache.get_mut().unwrap().clear();
        self.value_sizes.lock().unwrap().clear();
        *self.current_memory_bytes.lock().unwrap() = 0;
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        let current_memory = self.current_memory_bytes.lock().unwrap();
        *current_memory >= self.memory_budget_bytes
    }

    fn evict(&mut self) {
        self._evict_lfu_with_memory_internal();
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
        let current_memory = *self.current_memory_bytes.lock().unwrap();
        let memory_used_pct = if self.memory_budget_bytes > 0 {
            (current_memory as f64 / self.memory_budget_bytes as f64 * 100.0) as f64
        } else {
            0.0
        };
        stats.insert("memory_budget_mb".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.memory_budget_mb).unwrap()));
        stats.insert("current_memory_bytes".to_string(), serde_json::Value::Number(serde_json::Number::from(current_memory)));
        stats.insert("memory_used_pct".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(memory_used_pct).unwrap()));
        stats
    }
}

impl MemoryBoundedLFUCache {
    /// Initialize memory-bounded LFU cache.
    pub fn new(
        capacity: Option<i64>,
        memory_budget_mb: Option<f64>,
        name: Option<String>
    ) -> Self {
        let budget_mb = memory_budget_mb.unwrap_or(100.0);
        let budget_bytes = (budget_mb * 1024.0 * 1024.0) as i64;
        Self {
            cache: Mutex::new(LFUCache::new(capacity, name)),
            memory_budget_mb: budget_mb,
            memory_budget_bytes: budget_bytes,
            current_memory_bytes: Mutex::new(0),
            value_sizes: Mutex::new(HashMap::new()),
        }
    }

    /// Evict LFU entry and update memory tracking.
    fn _evict_lfu_with_memory_internal(&self) {
        let items = self.cache.lock().unwrap().items();
        if items.is_empty() {
            return;
        }
        
        // Get LFU key (first item in items list)
        let lfu_key = items[0].0.clone();
        
        // Update memory tracking
        let mut value_sizes = self.value_sizes.lock().unwrap();
        let mut current_memory = self.current_memory_bytes.lock().unwrap();
        if let Some(value_size) = value_sizes.remove(&lfu_key) {
            *current_memory -= value_size;
        }
        drop(value_sizes);
        drop(current_memory);
        
        // Evict from cache
        let mut cache = self.cache.lock().unwrap();
        cache.delete(lfu_key);
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
