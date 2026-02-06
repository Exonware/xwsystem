// #exonware/xwsystem/rust/src/caching/serializable.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Cache serialization utilities.
//! Usability Priority #2 - Persist and restore cache state.


use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::caching::base::ACache;
use crate::caching::lru_cache::LRUCache;

/// LRU Cache with serialization support.
///
/// Allows saving cache state to disk and loading it back.
///
/// Example:
/// ```rust
/// let mut cache = SerializableCache::new(Some(1000), None, None);
/// cache.put("key1".to_string(), serde_json::json!("value1"));
/// cache.put("key2".to_string(), serde_json::json!("value2"));
///
/// // Save to disk
/// cache.save_to_file("cache_backup.json".to_string(), Some("json".to_string()));
///
/// // Later... load from disk
/// let cache2 = SerializableCache::load_from_file("cache_backup.json".to_string(), Some("json".to_string()));
/// ```
pub struct SerializableCache {
    cache: LRUCache,
}

impl ACache for SerializableCache {
    fn get(&self, key: crate::caching::base::Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.get(key, default)
    }

    fn put(&mut self, key: crate::caching::base::Hashable, value: serde_json::Value) {
        self.cache.put(key, value);
    }

    fn delete(&mut self, key: crate::caching::base::Hashable) -> bool {
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

    fn keys(&self) -> Vec<crate::caching::base::Hashable> {
        self.cache.keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.values()
    }

    fn items(&self) -> Vec<(crate::caching::base::Hashable, serde_json::Value)> {
        self.cache.items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        self.cache.get_stats()
    }
}

impl SerializableCache {
    /// Create a new SerializableCache.
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: LRUCache::new(capacity, ttl, name),
        }
    }

    /// Save cache to file.
    ///
    /// Args:
    /// file_path: Path to save cache
    /// format: Serialization format ('json' - pickle not supported in Rust)
    ///
    /// Returns:
    /// True if saved successfully
    pub fn save_to_file(&self, file_path: String, format: Option<String>) -> bool {
        let format_str = format.unwrap_or_else(|| "json".to_string());
        
        if format_str != "json" {
            eprintln!("Warning: Only 'json' format is supported in Rust. 'pickle' is Python-specific.");
            return false;
        }
        
        // Collect cache data
        let items: HashMap<String, serde_json::Value> = self.items()
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        
        let stats = self.get_stats();
        let capacity = stats.get("capacity")
            .and_then(|v| v.as_i64())
            .unwrap_or(128);
        let ttl = stats.get("ttl")
            .and_then(|v| v.as_f64());
        let name = stats.get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let cache_data = serde_json::json!({
            "capacity": capacity,
            "ttl": ttl,
            "name": name,
            "items": items,
            "stats": stats
        });
        
        match fs::write(&file_path, serde_json::to_string_pretty(&cache_data).unwrap_or_default()) {
            Ok(_) => {
                println!("Cache saved to {} (json format)", file_path);
                true
            }
            Err(e) => {
                eprintln!("Failed to save cache to {}: {}", file_path, e);
                false
            }
        }
    }

    /// Load cache from file.
    ///
    /// Args:
    /// file_path: Path to load cache from
    /// format: Serialization format ('json' - pickle not supported in Rust)
    ///
    /// Returns:
    /// Loaded cache instance
    pub fn load_from_file(file_path: String, format: Option<String>) -> Self {
        let format_str = format.unwrap_or_else(|| "json".to_string());
        
        if format_str != "json" {
            panic!("Only 'json' format is supported in Rust. 'pickle' is Python-specific.");
        }
        
        let content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                panic!("Failed to read cache file {}: {}", file_path, e);
            }
        };
        
        let cache_data: serde_json::Value = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => {
                panic!("Failed to parse cache file {}: {}", file_path, e);
            }
        };
        
        // Create cache instance
        let capacity = cache_data.get("capacity")
            .and_then(|v| v.as_i64())
            .unwrap_or(128);
        let ttl = cache_data.get("ttl")
            .and_then(|v| v.as_f64());
        let name = cache_data.get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let mut cache = Self::new(Some(capacity), ttl, name);
        
        // Restore items
        if let Some(items) = cache_data.get("items").and_then(|v| v.as_object()) {
            for (key, value) in items {
                cache.put(key.clone(), value.clone());
            }
        }
        
        println!("Cache loaded from {} with {} entries", file_path, cache.size());
        cache
    }

    /// Create backup of cache.
    ///
    /// Args:
    /// backup_path: Path for backup file
    ///
    /// Returns:
    /// True if backed up successfully
    pub fn backup(&self, backup_path: String) -> bool {
        self.save_to_file(backup_path, Some("json".to_string()))
    }

    /// Restore cache from backup.
    ///
    /// Args:
    /// backup_path: Path to backup file
    ///
    /// Returns:
    /// True if restored successfully
    pub fn restore(&mut self, backup_path: String) -> bool {
        match Self::load_from_file(backup_path.clone(), Some("json".to_string())) {
            loaded_cache => {
                // Clear current cache
                self.clear();
                
                // Restore items
                for (key, value) in loaded_cache.items() {
                    self.put(key, value);
                }
                
                println!("Cache restored from {}", backup_path);
                true
            }
        }
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Type exported via mod.rs
