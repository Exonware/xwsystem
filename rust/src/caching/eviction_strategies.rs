// #exonware/xwsystem/rust/src/caching/eviction_strategies.rs
//exonware/xwsystem/caching/eviction_strategies.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Pluggable eviction strategies for caching module.
//! Extensibility Priority #5 - Strategy pattern for custom eviction policies.

/// Abstract base class for eviction strategies.
///
/// Implements Strategy pattern for pluggable eviction policies.
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::caching::base::Hashable;
use crate::caching::utils::estimate_object_size;

pub trait AEvictionStrategy {
    /// Select item to evict from cache.
    /// Args:
    /// cache_items: List of (key, value, metadata) tuples
    /// metadata includes: access_time, access_count, size, etc.
    /// Returns:
    /// Key of item to evict, or None if no eviction needed
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable>;

    /// Update strategy state on item access.
    /// Args:
    /// key: Key that was accessed
    /// metadata: Item metadata
    fn on_access(&self, key: Hashable, metadata: HashMap<String, serde_json::Value>) -> ();

    /// Update strategy state on item insertion.
    /// Args:
    /// key: Key that was inserted
    /// value: Value that was inserted
    /// metadata: Item metadata
    fn on_insert(&self, key: Hashable, value: serde_json::Value, metadata: HashMap<String, serde_json::Value>) -> ();

    /// Update strategy state on item deletion.
    /// Args:
    /// key: Key that was deleted
    fn on_delete(&self, key: Hashable) -> ();

    /// Get strategy name.
    fn get_strategy_name(&self) -> String;

}

/// Least Recently Used eviction strategy.
pub struct LRUEvictionStrategy;

impl AEvictionStrategy for LRUEvictionStrategy {
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable> {
        if cache_items.is_empty() {
            return None;
        }
        
        // Find item with oldest access time
        let lru_item = cache_items.iter()
            .min_by(|a, b| {
                let time_a = a.2.get("access_time")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let time_b = b.2.get("access_time")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
            });
        
        lru_item.map(|(key, _, _)| key.clone())
    }

    fn on_access(&self, _key: Hashable, _metadata: HashMap<String, serde_json::Value>) {
        // Update access time - metadata updates should be handled by cache implementation
        // This method is called to notify the strategy of access
    }

    fn on_insert(&self, _key: Hashable, _value: serde_json::Value, _metadata: HashMap<String, serde_json::Value>) {
        // Set initial access time - metadata updates should be handled by cache implementation
        // This method is called to notify the strategy of insertion
    }

    fn on_delete(&self, _key: Hashable) {
        // No action needed on delete
    }

    fn get_strategy_name(&self) -> String {
        "LRU".to_string()
    }
}

impl LRUEvictionStrategy {
    /// Create a new LRU eviction strategy.
    pub fn new() -> Self {
        Self
    }
}

/// Least Frequently Used eviction strategy.
pub struct LFUEvictionStrategy {
    access_counts: Mutex<HashMap<Hashable, i64>>,
}

impl AEvictionStrategy for LFUEvictionStrategy {
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable> {
        if cache_items.is_empty() {
            return None;
        }
        
        let access_counts = self.access_counts.lock().unwrap();
        
        // Find item with lowest access count
        let lfu_item = cache_items.iter()
            .min_by_key(|(key, _, _)| {
                access_counts.get(key).copied().unwrap_or(0)
            });
        
        lfu_item.map(|(key, _, _)| key.clone())
    }

    fn on_access(&self, key: Hashable, _metadata: HashMap<String, serde_json::Value>) {
        // Increment access count
        let mut access_counts = self.access_counts.lock().unwrap();
        let count = access_counts.entry(key).or_insert(0);
        *count += 1;
    }

    fn on_insert(&self, key: Hashable, _value: serde_json::Value, _metadata: HashMap<String, serde_json::Value>) {
        // Initialize access count
        let mut access_counts = self.access_counts.lock().unwrap();
        access_counts.insert(key, 1);
    }

    fn on_delete(&self, key: Hashable) {
        // Remove access count tracking
        let mut access_counts = self.access_counts.lock().unwrap();
        access_counts.remove(&key);
    }

    fn get_strategy_name(&self) -> String {
        "LFU".to_string()
    }
}

impl LFUEvictionStrategy {
    /// Initialize LFU strategy.
    pub fn new() -> Self {
        Self {
            access_counts: Mutex::new(HashMap::new()),
        }
    }
}

/// First In, First Out eviction strategy.
pub struct FIFOEvictionStrategy;

impl AEvictionStrategy for FIFOEvictionStrategy {
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable> {
        if cache_items.is_empty() {
            return None;
        }
        
        // Find item with oldest creation time
        let fifo_item = cache_items.iter()
            .min_by(|a, b| {
                let time_a = a.2.get("created_at")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                let time_b = b.2.get("created_at")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
            });
        
        fifo_item.map(|(key, _, _)| key.clone())
    }

    fn on_access(&self, _key: Hashable, _metadata: HashMap<String, serde_json::Value>) {
        // No action on access for FIFO
    }

    fn on_insert(&self, _key: Hashable, _value: serde_json::Value, _metadata: HashMap<String, serde_json::Value>) {
        // Set creation time - metadata updates should be handled by cache implementation
    }

    fn on_delete(&self, _key: Hashable) {
        // No action needed on delete
    }

    fn get_strategy_name(&self) -> String {
        "FIFO".to_string()
    }
}

impl FIFOEvictionStrategy {
    /// Create a new FIFO eviction strategy.
    pub fn new() -> Self {
        Self
    }
}

/// Random eviction strategy.
pub struct RandomEvictionStrategy;

impl AEvictionStrategy for RandomEvictionStrategy {
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable> {
        if cache_items.is_empty() {
            return None;
        }
        
        // Select random item to evict
        // Note: For true randomness, add 'rand' crate to dependencies
        // For now, use a simple pseudo-random approach based on system time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize;
        let index = now % cache_items.len();
        Some(cache_items[index].0.clone())
    }

    fn on_access(&self, _key: Hashable, _metadata: HashMap<String, serde_json::Value>) {
        // No action on access for random
    }

    fn on_insert(&self, _key: Hashable, _value: serde_json::Value, _metadata: HashMap<String, serde_json::Value>) {
        // No action on insert for random
    }

    fn on_delete(&self, _key: Hashable) {
        // No action on delete for random
    }

    fn get_strategy_name(&self) -> String {
        "RANDOM".to_string()
    }
}

impl RandomEvictionStrategy {
    /// Create a new random eviction strategy.
    pub fn new() -> Self {
        Self
    }
}

/// Evict largest items first to free maximum memory.
pub struct SizeBasedEvictionStrategy;

impl AEvictionStrategy for SizeBasedEvictionStrategy {
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable> {
        if cache_items.is_empty() {
            return None;
        }
        
        // Find item with largest size
        let largest_item = cache_items.iter()
            .max_by_key(|(_, _, metadata)| {
                metadata.get("size_bytes")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0)
            });
        
        largest_item.map(|(key, _, _)| key.clone())
    }

    fn on_access(&self, _key: Hashable, _metadata: HashMap<String, serde_json::Value>) {
        // No action on access
    }

    fn on_insert(&self, _key: Hashable, value: serde_json::Value, _metadata: HashMap<String, serde_json::Value>) {
        // Track value size - metadata updates should be handled by cache implementation
        // Size calculation: estimate_object_size(value)
    }

    fn on_delete(&self, _key: Hashable) {
        // No action on delete
    }

    fn get_strategy_name(&self) -> String {
        "SIZE_BASED".to_string()
    }
}

impl SizeBasedEvictionStrategy {
    /// Create a new size-based eviction strategy.
    pub fn new() -> Self {
        Self
    }
}

/// Evict items closest to expiration first.
pub struct TTLEvictionStrategy;

impl AEvictionStrategy for TTLEvictionStrategy {
    fn select_victim(&self, cache_items: Vec<(Hashable, serde_json::Value, HashMap<String, serde_json::Value>)>) -> Option<Hashable> {
        if cache_items.is_empty() {
            return None;
        }
        
        // Find item with earliest expiration
        let expired_item = cache_items.iter()
            .min_by(|a, b| {
                let time_a = a.2.get("expires_at")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(f64::INFINITY);
                let time_b = b.2.get("expires_at")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(f64::INFINITY);
                time_a.partial_cmp(&time_b).unwrap_or(std::cmp::Ordering::Equal)
            });
        
        expired_item.map(|(key, _, _)| key.clone())
    }

    fn on_access(&self, _key: Hashable, _metadata: HashMap<String, serde_json::Value>) {
        // No action on access for TTL
    }

    fn on_insert(&self, _key: Hashable, _value: serde_json::Value, metadata: HashMap<String, serde_json::Value>) {
        // Set expiration time - metadata updates should be handled by cache implementation
        // TTL is typically: time.time() + ttl (where ttl comes from metadata.get('ttl', 300))
    }

    fn on_delete(&self, _key: Hashable) {
        // No action on delete
    }

    fn get_strategy_name(&self) -> String {
        "TTL".to_string()
    }
}

impl TTLEvictionStrategy {
    /// Create a new TTL eviction strategy.
    pub fn new() -> Self {
        Self
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
