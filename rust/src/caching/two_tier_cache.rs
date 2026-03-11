// #exonware/xwsystem/rust/src/caching/two_tier_cache.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 26, 2025
//! 
//! Two-tier cache implementation combining memory and disk caching.


use std::collections::HashMap;
use std::sync::Mutex;
use crate::caching::contracts::ICache;
use crate::caching::lru_cache::LRUCache;

// Root cause fixed: LRUCache uses 'capacity' parameter, not 'maxsize'
// Disk to memory promotions
// Try memory tier first
// Promote to memory tier
/// Two-tier cache combining memory LRU cache with disk persistence.
///
/// Features:
/// - Memory tier: Fast LRU cache for hot data
/// - Disk tier: Persistent storage for cold data
/// - Automatic promotion from disk to memory on hit
/// - Write-through to both tiers
/// - Namespace support for multiple cache instances
/// - Comprehensive statistics for both tiers
pub struct TwoTierCache {
    namespace: String,
    memory_cache: Mutex<LRUCache>,
    // Note: DiskCache is not yet implemented, so we'll use a placeholder
    // disk_cache: Mutex<DiskCache>,
    stats: Mutex<HashMap<String, i64>>,
}

impl ICache for TwoTierCache {
    fn get(&self, key: &str) -> Option<serde_json::Value> {
        // Try memory tier first
        let memory_value = self.memory_cache.lock().unwrap().get(key.to_string(), None);
        if memory_value.is_some() {
            let mut stats = self.stats.lock().unwrap();
            *stats.entry("memory_hits".to_string()).or_insert(0) += 1;
            return memory_value;
        }
        
        // Try disk tier (placeholder - DiskCache not yet implemented)
        // let disk_value = self.disk_cache.lock().unwrap().get(key);
        // if let Some(value) = disk_value {
        //     // Promote to memory tier
        //     self.memory_cache.lock().unwrap().put(key.to_string(), value.clone());
        //     let mut stats = self.stats.lock().unwrap();
        //     *stats.entry("disk_hits".to_string()).or_insert(0) += 1;
        //     *stats.entry("promotions".to_string()).or_insert(0) += 1;
        //     return Some(value);
        // }
        
        // Miss in both tiers
        let mut stats = self.stats.lock().unwrap();
        *stats.entry("misses".to_string()).or_insert(0) += 1;
        None
    }

    fn set(&mut self, key: &str, value: serde_json::Value, _ttl: Option<i64>) -> bool {
        // Set in memory tier
        self.memory_cache.get_mut().unwrap().put(key.to_string(), value.clone());
        
        // Set in disk tier (placeholder - DiskCache not yet implemented)
        // let disk_success = self.disk_cache.get_mut().unwrap().set(key, value, ttl);
        
        let mut stats = self.stats.lock().unwrap();
        *stats.entry("sets".to_string()).or_insert(0) += 1;
        true
    }

    fn delete(&mut self, key: &str) -> bool {
        let memory_deleted = self.memory_cache.get_mut().unwrap().delete(key.to_string());
        
        // Delete from disk tier (placeholder - DiskCache not yet implemented)
        // let disk_deleted = self.disk_cache.get_mut().unwrap().delete(key);
        
        if memory_deleted {
            let mut stats = self.stats.lock().unwrap();
            *stats.entry("deletes".to_string()).or_insert(0) += 1;
            return true;
        }
        false
    }

    fn clear(&mut self) -> bool {
        self.memory_cache.get_mut().unwrap().clear();
        // self.disk_cache.get_mut().unwrap().clear();
        let mut stats = self.stats.lock().unwrap();
        stats.clear();
        true
    }

    fn exists(&self, key: &str) -> bool {
        // Check memory tier
        if self.memory_cache.lock().unwrap().get(key.to_string(), None).is_some() {
            return true;
        }
        
        // Check disk tier (placeholder - DiskCache not yet implemented)
        // self.disk_cache.lock().unwrap().exists(key)
        false
    }

    fn size(&self) -> i64 {
        let memory_size = self.memory_cache.lock().unwrap().size();
        // let disk_size = self.disk_cache.lock().unwrap().size();
        memory_size // + disk_size
    }
}

impl TwoTierCache {
    /// Initialize two-tier cache.
    ///
    /// Args:
    /// namespace: Cache namespace for organization
    /// memory_size: Maximum entries in memory tier
    /// disk_size: Maximum entries in disk tier
    /// disk_cache_dir: Custom disk cache directory
    /// max_file_size: Maximum size per disk cache file
    pub fn new(
        namespace: Option<String>,
        memory_size: Option<i64>,
        _disk_size: Option<i64>,
        _disk_cache_dir: Option<String>,
        _max_file_size: Option<i64>
    ) -> Self {
        let ns = namespace.unwrap_or_else(|| "default".to_string());
        let mem_size = memory_size.unwrap_or(1000);
        
        // Initialize disk cache (placeholder - DiskCache not yet implemented)
        // let disk = DiskCache::new(
        //     Some(ns.clone()),
        //     disk_cache_dir,
        //     disk_size,
        //     max_file_size,
        // );
        
        let mut stats = HashMap::new();
        stats.insert("memory_hits".to_string(), 0);
        stats.insert("disk_hits".to_string(), 0);
        stats.insert("misses".to_string(), 0);
        stats.insert("sets".to_string(), 0);
        stats.insert("deletes".to_string(), 0);
        stats.insert("promotions".to_string(), 0);
        
        Self {
            namespace: ns,
            memory_cache: Mutex::new(LRUCache::new(Some(mem_size), None, None)),
            // disk_cache: Mutex::new(disk),
            stats: Mutex::new(stats),
        }
    }

    /// Get comprehensive statistics for both tiers.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let memory_stats = self.memory_cache.lock().unwrap().get_stats();
        // let disk_stats = self.disk_cache.lock().unwrap().get_stats();
        let stats = self.stats.lock().unwrap();
        
        let memory_hits = *stats.get("memory_hits").unwrap_or(&0);
        let disk_hits = *stats.get("disk_hits").unwrap_or(&0);
        let misses = *stats.get("misses").unwrap_or(&0);
        let total_hits = memory_hits + disk_hits;
        let total_requests = total_hits + misses;
        let overall_hit_rate = if total_requests > 0 {
            total_hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let memory_size = memory_stats.get("size")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        
        let mut result = HashMap::new();
        result.insert("namespace".to_string(), serde_json::Value::String(self.namespace.clone()));
        result.insert("total_size".to_string(), serde_json::Value::Number(serde_json::Number::from(self.size())));
        result.insert("memory_size".to_string(), serde_json::Value::Number(serde_json::Number::from(memory_size)));
        result.insert("memory_hits".to_string(), serde_json::Value::Number(serde_json::Number::from(memory_hits)));
        result.insert("disk_hits".to_string(), serde_json::Value::Number(serde_json::Number::from(disk_hits)));
        result.insert("misses".to_string(), serde_json::Value::Number(serde_json::Number::from(misses)));
        result.insert("overall_hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(overall_hit_rate).unwrap()));
        result.insert("promotions".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("promotions").unwrap_or(&0))));
        result.insert("sets".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("sets").unwrap_or(&0))));
        result.insert("deletes".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("deletes").unwrap_or(&0))));
        result.insert("memory_stats".to_string(), serde_json::Value::Object(serde_json::Map::from_iter(
            memory_stats.iter().map(|(k, v)| (k.clone(), v.clone()))
        )));
        // result.insert("disk_stats".to_string(), serde_json::Value::Object(serde_json::Map::from_iter(
        //     disk_stats.iter().map(|(k, v)| (k.clone(), v.clone()))
        // )));
        result
    }

    /// Get memory tier statistics.
    pub fn get_memory_stats(&self) -> HashMap<String, serde_json::Value> {
        self.memory_cache.lock().unwrap().get_stats()
    }

    /// Get disk tier statistics.
    pub fn get_disk_stats(&self) -> HashMap<String, serde_json::Value> {
        // self.disk_cache.lock().unwrap().get_stats()
        HashMap::new() // Placeholder until DiskCache is implemented
    }

    /// Preload specified keys from disk to memory.
    ///
    /// Args:
    /// keys: List of keys to preload
    ///
    /// Returns:
    /// Number of keys successfully preloaded
    pub fn preload_from_disk(&self, _keys: Vec<serde_json::Value>) -> i64 {
        let preloaded = 0;
        // for key in keys {
        //     if let Some(key_str) = key.as_str() {
        //         if let Some(value) = self.disk_cache.lock().unwrap().get(key_str) {
        //             self.memory_cache.lock().unwrap().put(key_str.to_string(), value);
        //             preloaded += 1;
        //         }
        //     }
        // }
        preloaded
    }

    /// Evict specified keys from memory tier.
    ///
    /// Args:
    /// keys: List of keys to evict from memory
    ///
    /// Returns:
    /// Number of keys successfully evicted
    pub fn evict_from_memory(&self, keys: Vec<serde_json::Value>) -> i64 {
        let mut evicted = 0;
        let mut memory_cache = self.memory_cache.lock().unwrap();
        for key in keys {
            if let Some(key_str) = key.as_str() {
                if memory_cache.delete(key_str.to_string()) {
                    evicted += 1;
                }
            }
        }
        evicted
    }

}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Type exported via mod.rs
