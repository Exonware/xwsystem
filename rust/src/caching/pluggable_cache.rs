// #exonware/xwsystem/rust/src/caching/pluggable_cache.rs
//exonware/xwsystem/caching/pluggable_cache.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Pluggable cache with runtime-switchable eviction strategies.
//! Extensibility Priority #5 - Maximum flexibility for custom behaviors.


use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::caching::base::{ACache, Hashable};
use crate::caching::eviction_strategies::{AEvictionStrategy, LRUEvictionStrategy};

// Update strategy metadata
// Check if we need to evict
// Initialize or update metadata
// Rebuild metadata for new strategy
// Build cache items list with metadata
// Select victim using strategy
/// Cache with pluggable eviction strategy.
///
/// Allows runtime switching of eviction policies for maximum flexibility.
pub struct PluggableCache {
    capacity: i64,
    strategy: Mutex<Box<dyn AEvictionStrategy + Send + Sync>>,
    name: String,
    cache: Mutex<HashMap<Hashable, serde_json::Value>>,
    metadata: Mutex<HashMap<Hashable, HashMap<String, serde_json::Value>>>,
    hits: Mutex<i64>,
    misses: Mutex<i64>,
    evictions: Mutex<i64>,
    strategy_switches: Mutex<i64>,
}

impl PluggableCache {
    /// Initialize pluggable cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// strategy: Eviction strategy (default: LRU)
    /// name: Cache name for debugging
    pub fn new(
        capacity: Option<i64>,
        strategy: Option<Box<dyn AEvictionStrategy + Send + Sync>>,
        name: Option<String>
    ) -> Self {
        let cap = capacity.unwrap_or(128);
        if cap <= 0 {
            panic!("Cache capacity must be positive, got {}", cap);
        }
        
        let strategy_val = strategy.unwrap_or_else(|| Box::new(LRUEvictionStrategy::new()));
        let name_val = name.unwrap_or_else(|| format!("PluggableCache-{:p}", &strategy_val));
        
        Self {
            capacity: cap,
            strategy: Mutex::new(strategy_val),
            name: name_val,
            cache: Mutex::new(HashMap::new()),
            metadata: Mutex::new(HashMap::new()),
            hits: Mutex::new(0),
            misses: Mutex::new(0),
            evictions: Mutex::new(0),
            strategy_switches: Mutex::new(0),
        }
    }
}

impl ACache for PluggableCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        let mut cache = self.cache.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();
        let mut hits = self.hits.lock().unwrap();
        let mut misses = self.misses.lock().unwrap();
        
        if !cache.contains_key(&key) {
            *misses += 1;
            return default;
        }
        
        // Update strategy metadata
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let entry_metadata = metadata.entry(key.clone()).or_insert_with(HashMap::new);
        entry_metadata.insert("access_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
        
        let strategy = self.strategy.lock().unwrap();
        strategy.on_access(key.clone(), entry_metadata.clone());
        drop(strategy);
        
        *hits += 1;
        cache.get(&key).cloned()
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let mut cache = self.cache.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();
        
        // Check if we need to evict
        if !cache.contains_key(&key) && cache.len() >= self.capacity as usize {
            drop(cache);
            drop(metadata);
            self._evict_using_strategy();
            cache = self.cache.lock().unwrap();
            metadata = self.metadata.lock().unwrap();
        }
        
        // Store value
        cache.insert(key.clone(), value.clone());
        
        // Initialize or update metadata
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let entry_metadata = metadata.entry(key.clone()).or_insert_with(HashMap::new);
        
        let strategy = self.strategy.lock().unwrap();
        if !entry_metadata.contains_key("access_time") {
            entry_metadata.insert("access_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
            entry_metadata.insert("insert_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
            strategy.on_insert(key.clone(), value, entry_metadata.clone());
        } else {
            entry_metadata.insert("access_time".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
            strategy.on_access(key.clone(), entry_metadata.clone());
        }
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache = self.cache.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();
        
        if !cache.contains_key(&key) {
            return false;
        }
        
        cache.remove(&key);
        metadata.remove(&key);
        
        let strategy = self.strategy.lock().unwrap();
        strategy.on_delete(key);
        
        true
    }

    fn clear(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();
        cache.clear();
        metadata.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().len() as i64
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().len() >= self.capacity as usize
    }

    fn evict(&mut self) {
        if self.cache.lock().unwrap().len() > 0 {
            self._evict_using_strategy();
        }
    }

    fn keys(&self) -> Vec<Hashable> {
        self.cache.lock().unwrap().keys().cloned().collect()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.lock().unwrap().values().cloned().collect()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.lock().unwrap().iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().unwrap();
        let hits = *self.hits.lock().unwrap();
        let misses = *self.misses.lock().unwrap();
        let evictions = *self.evictions.lock().unwrap();
        let strategy_switches = *self.strategy_switches.lock().unwrap();
        let strategy = self.strategy.lock().unwrap();
        let strategy_name = strategy.get_strategy_name();
        drop(strategy);
        
        let total_requests = hits + misses;
        let hit_rate = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let mut stats = HashMap::new();
        stats.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        stats.insert("type".to_string(), serde_json::Value::String("Pluggable".to_string()));
        stats.insert("strategy".to_string(), serde_json::Value::String(strategy_name));
        stats.insert("capacity".to_string(), serde_json::Value::Number(serde_json::Number::from(self.capacity)));
        stats.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(cache.len())));
        stats.insert("hits".to_string(), serde_json::Value::Number(serde_json::Number::from(hits)));
        stats.insert("misses".to_string(), serde_json::Value::Number(serde_json::Number::from(misses)));
        stats.insert("evictions".to_string(), serde_json::Value::Number(serde_json::Number::from(evictions)));
        stats.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        stats.insert("strategy_switches".to_string(), serde_json::Value::Number(serde_json::Number::from(strategy_switches)));
        
        stats
    }
}

impl PluggableCache {
    /// Change eviction strategy at runtime.
    ///
    /// Args:
    /// strategy: New eviction strategy
    ///
    /// Note:
    /// Metadata may need to be rebuilt for the new strategy.
    pub fn set_strategy(&self, strategy: Box<dyn AEvictionStrategy + Send + Sync>) {
        let mut strategy_guard = self.strategy.lock().unwrap();
        
        // Rebuild metadata for new strategy
        let cache = self.cache.lock().unwrap();
        let mut metadata = self.metadata.lock().unwrap();
        
        for (key, value) in cache.iter() {
            let entry_metadata = metadata.entry(key.clone()).or_insert_with(HashMap::new);
            entry_metadata.clear();
            strategy.on_insert(key.clone(), value.clone(), entry_metadata.clone());
        }
        
        *strategy_guard = strategy;
        *self.strategy_switches.lock().unwrap() += 1;
    }

    /// Get current eviction strategy name.
    pub fn get_strategy_name(&self) -> String {
        self.strategy.lock().unwrap().get_strategy_name()
    }

    /// Evict one item using current strategy.
    fn _evict_using_strategy(&self) {
        // Build cache items list with metadata
        let cache = self.cache.lock().unwrap();
        let metadata = self.metadata.lock().unwrap();
        
        let cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)> = cache
            .iter()
            .map(|(key, value)| {
                let meta = metadata.get(key).cloned().unwrap_or_else(HashMap::new);
                (key.clone(), value.clone(), meta)
            })
            .collect();
        
        drop(cache);
        drop(metadata);
        
        // Select victim using strategy
        let strategy = self.strategy.lock().unwrap();
        let victim_key = strategy.select_victim(cache_items);
        drop(strategy);
        
        if let Some(key) = victim_key {
            let mut cache = self.cache.lock().unwrap();
            let mut metadata = self.metadata.lock().unwrap();
            
            cache.remove(&key);
            metadata.remove(&key);
            
            let strategy = self.strategy.lock().unwrap();
            strategy.on_delete(key.clone());
            drop(strategy);
            
            *self.evictions.lock().unwrap() += 1;
        }
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Type exported via mod.rs
