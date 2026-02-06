// #exonware/xwsystem/rust/src/caching/observable_cache.rs
//exonware/xwsystem/caching/observable_cache.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Observable cache implementations with event emission.
//! Extensibility Priority #5 - Event-driven caching for custom behaviors.


use std::collections::HashMap;
use std::sync::Mutex;
use crate::caching::base::{ACache, Hashable};
use crate::caching::events::{CacheEvent, CacheEventEmitter};
use crate::caching::lru_cache::LRUCache;

// Get LRU key before evicting
/// LRU Cache with event emission.
///
/// Emits events for all cache operations: HIT, MISS, PUT, DELETE, EVICT.
pub struct ObservableLRUCache {
    cache: Mutex<LRUCache>,
    emitter: CacheEventEmitter,
}

impl ACache for ObservableLRUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let value = self.cache.lock().unwrap().get(key.clone(), default.clone());
        
        let mut kwargs = HashMap::new();
        kwargs.insert("key".to_string(), serde_json::Value::String(key.clone()));
        
        if value.is_some() && value != default {
            kwargs.insert("value".to_string(), value.clone().unwrap());
            self.emitter._emit(CacheEvent::Hit, kwargs);
        } else {
            self.emitter._emit(CacheEvent::Miss, kwargs);
        }
        
        value
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        self.cache.get_mut().unwrap().put(key.clone(), value.clone());
        
        let mut kwargs = HashMap::new();
        kwargs.insert("key".to_string(), serde_json::Value::String(key));
        kwargs.insert("value".to_string(), value);
        self.emitter._emit(CacheEvent::Put, kwargs);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let deleted = self.cache.get_mut().unwrap().delete(key.clone());
        if deleted {
            let mut kwargs = HashMap::new();
            kwargs.insert("key".to_string(), serde_json::Value::String(key));
            self.emitter._emit(CacheEvent::Delete, kwargs);
        }
        deleted
    }

    fn clear(&mut self) {
        self.cache.get_mut().unwrap().clear();
        self.emitter._emit(CacheEvent::Clear, HashMap::new());
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        // Get LRU key before evicting
        let items = self.cache.lock().unwrap().items();
        if !items.is_empty() {
            let lru_key = items[0].0.clone();
            self.cache.get_mut().unwrap().evict();
            let mut kwargs = HashMap::new();
            kwargs.insert("key".to_string(), serde_json::Value::String(lru_key));
            self.emitter._emit(CacheEvent::Evict, kwargs);
        } else {
            self.cache.get_mut().unwrap().evict();
        }
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
        let event_stats = self.emitter.get_event_stats();
        let mut event_stats_json = serde_json::Map::new();
        for (k, v) in event_stats {
            event_stats_json.insert(k, serde_json::Value::Number(serde_json::Number::from(v)));
        }
        stats.insert("events".to_string(), serde_json::Value::Object(event_stats_json));
        stats
    }
}

impl ObservableLRUCache {
    /// Initialize observable LRU cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// ttl: Optional TTL in seconds
    /// name: Cache name for debugging
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            emitter: CacheEventEmitter::new(),
        }
    }

    /// Register event callback.
    pub fn on<F>(&self, event: CacheEvent, callback: F)
    where
        F: Fn(&CacheEvent, &HashMap<String, serde_json::Value>) + Send + Sync + 'static,
    {
        self.emitter.on(event, callback);
    }
}

/// Optimized O(1) LFU Cache with event emission.
///
/// Combines performance and extensibility.
/// Note: This implementation uses LFUCache instead of OptimizedLFUCache
/// since OptimizedLFUCache is not yet fully implemented.
pub struct ObservableLFUCache {
    cache: Mutex<crate::caching::lfu_cache::LFUCache>,
    emitter: CacheEventEmitter,
}

impl ACache for ObservableLFUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let value = self.cache.lock().unwrap().get(key.clone(), default.clone());
        
        let mut kwargs = HashMap::new();
        kwargs.insert("key".to_string(), serde_json::Value::String(key.clone()));
        
        if value.is_some() && value != default {
            kwargs.insert("value".to_string(), value.clone().unwrap());
            self.emitter._emit(CacheEvent::Hit, kwargs);
        } else {
            self.emitter._emit(CacheEvent::Miss, kwargs);
        }
        
        value
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        self.cache.get_mut().unwrap().put(key.clone(), value.clone());
        
        let mut kwargs = HashMap::new();
        kwargs.insert("key".to_string(), serde_json::Value::String(key));
        kwargs.insert("value".to_string(), value);
        self.emitter._emit(CacheEvent::Put, kwargs);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let deleted = self.cache.get_mut().unwrap().delete(key.clone());
        if deleted {
            let mut kwargs = HashMap::new();
            kwargs.insert("key".to_string(), serde_json::Value::String(key));
            self.emitter._emit(CacheEvent::Delete, kwargs);
        }
        deleted
    }

    fn clear(&mut self) {
        self.cache.get_mut().unwrap().clear();
        self.emitter._emit(CacheEvent::Clear, HashMap::new());
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
        let event_stats = self.emitter.get_event_stats();
        let mut event_stats_json = serde_json::Map::new();
        for (k, v) in event_stats {
            event_stats_json.insert(k, serde_json::Value::Number(serde_json::Number::from(v)));
        }
        stats.insert("events".to_string(), serde_json::Value::Object(event_stats_json));
        stats
    }
}

impl ObservableLFUCache {
    /// Initialize observable LFU cache.
    pub fn new(
        capacity: Option<i64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: Mutex::new(crate::caching::lfu_cache::LFUCache::new(capacity, name)),
            emitter: CacheEventEmitter::new(),
        }
    }

    /// Register event callback.
    pub fn on<F>(&self, event: CacheEvent, callback: F)
    where
        F: Fn(&CacheEvent, &HashMap<String, serde_json::Value>) + Send + Sync + 'static,
    {
        self.emitter.on(event, callback);
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
