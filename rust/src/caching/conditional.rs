// #exonware/xwsystem/rust/src/caching/conditional.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Conditional eviction policies for caching.
//! Extensibility Priority #5 - Customizable eviction behavior.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;

// Still add the new entry (exceeds capacity temporarily)
// Start from least recently used
// Check if this entry can be evicted
// No evictable entry found
/// LRU Cache with conditional eviction policies.
///
/// Allows custom logic to determine which entries can be evicted.
/// Useful for protecting important entries from eviction.
pub struct ConditionalEvictionCache {
    cache: Mutex<LRUCache>,
    eviction_policy: Option<Arc<dyn Fn(&Hashable, &serde_json::Value) -> bool + Send + Sync>>,
    eviction_rejections: Mutex<i64>,
}

impl ACache for ConditionalEvictionCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.lock().unwrap().get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let mut cache_guard = self.cache.lock().unwrap();
        
        if cache_guard.get(key.clone(), None).is_some() {
            // Update existing key
            cache_guard.put(key, value);
        } else {
            // Add new key
            if cache_guard.is_full() {
                // Find evictable entry
                let evicted = self._evict_conditional_internal(&mut cache_guard);
                if !evicted {
                    // Still add the new entry (exceeds capacity temporarily)
                }
            }
            cache_guard.put(key, value);
        }
    }

    fn delete(&mut self, key: Hashable) -> bool {
        self.cache.get_mut().unwrap().delete(key)
    }

    fn clear(&mut self) {
        self.cache.get_mut().unwrap().clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        self.cache.get_mut().unwrap().evict();
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
        let eviction_rejections = *self.eviction_rejections.lock().unwrap();
        stats.insert("eviction_rejections".to_string(), serde_json::Value::Number(serde_json::Number::from(eviction_rejections)));
        stats
    }
}

impl ConditionalEvictionCache {
    /// Initialize conditional eviction cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// eviction_policy: Function (key, value) -> bool returning True if entry can be evicted
    /// ttl: Optional TTL in seconds
    /// name: Cache name
    pub fn new<F>(
        capacity: Option<i64>,
        eviction_policy: Option<F>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self
    where
        F: Fn(&Hashable, &serde_json::Value) -> bool + Send + Sync + 'static,
    {
        let default_policy: Arc<dyn Fn(&Hashable, &serde_json::Value) -> bool + Send + Sync> = Arc::new(|_, _| true);
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            eviction_policy: eviction_policy.map(|f| Arc::new(f) as Arc<dyn Fn(&Hashable, &serde_json::Value) -> bool + Send + Sync>).or(Some(default_policy)),
            eviction_rejections: Mutex::new(0),
        }
    }

    /// Evict LRU entry that passes eviction policy.
    ///
    /// Returns:
    /// True if entry was evicted
    fn _evict_conditional_internal(&self, cache: &mut LRUCache) -> bool {
        let items = cache.items();
        let mut attempts = 0;
        let max_attempts = items.len();
        
        // Try to evict entries (iterate through all items)
        // Note: items() doesn't guarantee LRU order, but we try all entries
        for (key, value) in items.iter() {
            attempts += 1;
            if attempts > max_attempts {
                break;
            }
            
            // Check if this entry can be evicted
            if let Some(policy) = &self.eviction_policy {
                if policy(key, value) {
                    // Evict this entry
                    if cache.delete(key.clone()) {
                        return true;
                    }
                } else {
                    // Try next entry
                    *self.eviction_rejections.lock().unwrap() += 1;
                }
            }
        }
        
        // No evictable entry found
        false
    }

    /// Update eviction policy.
    ///
    /// Args:
    /// policy: New eviction policy function
    pub fn set_eviction_policy<F>(&mut self, policy: F)
    where
        F: Fn(&Hashable, &serde_json::Value) -> bool + Send + Sync + 'static,
    {
        self.eviction_policy = Some(Arc::new(policy) as Arc<dyn Fn(&Hashable, &serde_json::Value) -> bool + Send + Sync>);
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Type exported via mod.rs
