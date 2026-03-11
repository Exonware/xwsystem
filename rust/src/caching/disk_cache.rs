// #exonware/xwsystem/rust/src/caching/disk_cache.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 26, 2025
//! 
//! Disk cache implementation with pickle-based persistence.


use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};

use crate::caching::contracts::ICache;
use crate::caching::errors::CacheError;

// Setup cache directory
// Metadata file for tracking cache entries
// Remove expired entries
// Evict oldest entries if over limit
// Sort by access time and evict oldest
// Check file size limit
// Delete all cache files
/// Disk-based cache with JSON persistence.
///
/// Features:
/// - JSON-based serialization
/// - SHA256 key hashing for filesystem safety
/// - Size limits and eviction policies
/// - Thread-safe file operations
/// - Automatic cache directory management
pub struct DiskCache {
    namespace: String,
    cache_dir: PathBuf,
    max_size: i64,
    max_file_size: i64,
    cleanup_interval: i64,
    metadata: Mutex<HashMap<String, HashMap<String, serde_json::Value>>>,
    metadata_file: PathBuf,
    stats: Mutex<HashMap<String, i64>>,
    last_cleanup: Mutex<f64>,
}

impl DiskCache {
    /// Initialize disk cache.
    ///
    ///
    /// Args:
    /// namespace: Cache namespace for organization
    /// cache_dir: Custom cache directory (default: ~/.xwsystem/cache/{namespace}/)
    /// max_size: Maximum number of cache entries
    /// max_file_size: Maximum size per cache file in bytes
    /// cleanup_interval: Cleanup interval in seconds
    pub fn new(
        namespace: Option<String>,
        cache_dir: Option<String>,
        max_size: Option<i64>,
        max_file_size: Option<i64>,
        cleanup_interval: Option<i64>
    ) -> Self {
        let ns = namespace.unwrap_or_else(|| "default".to_string());
        let max_sz = max_size.unwrap_or(1000);
        let max_file_sz = max_file_size.unwrap_or(10 * 1024 * 1024); // 10MB
        let cleanup_int = cleanup_interval.unwrap_or(3600); // 1 hour
        
        // Setup cache directory
        let cache_path = if let Some(dir) = cache_dir {
            PathBuf::from(dir)
        } else {
            // Use environment variable or default to current directory
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            home.join(".xwsystem").join("cache").join(&ns)
        };
        
        // Create directory if it doesn't exist
        fs::create_dir_all(&cache_path).ok();
        
        let metadata_file = cache_path.join("metadata.json");
        
        let mut instance = Self {
            namespace: ns,
            cache_dir: cache_path,
            max_size: max_sz,
            max_file_size: max_file_sz,
            cleanup_interval: cleanup_int,
            metadata: Mutex::new(HashMap::new()),
            metadata_file,
            stats: Mutex::new({
                let mut s = HashMap::new();
                s.insert("hits".to_string(), 0);
                s.insert("misses".to_string(), 0);
                s.insert("sets".to_string(), 0);
                s.insert("deletes".to_string(), 0);
                s.insert("evictions".to_string(), 0);
                s.insert("errors".to_string(), 0);
                s
            }),
            last_cleanup: Mutex::new(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64()
            ),
        };
        
        instance._load_metadata();
        instance
    }

    /// Load cache metadata from disk.
    fn _load_metadata(&self) {
        if let Ok(mut file) = fs::File::open(&self.metadata_file) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(metadata) = serde_json::from_str::<HashMap<String, HashMap<String, serde_json::Value>>>(&contents) {
                    *self.metadata.lock().unwrap() = metadata;
                }
            }
        }
    }

    /// Save cache metadata to disk.
    fn _save_metadata(&self) {
        let metadata = self.metadata.lock().unwrap();
        if let Ok(json_str) = serde_json::to_string(&*metadata) {
            if let Ok(mut file) = fs::File::create(&self.metadata_file) {
                let _ = file.write_all(json_str.as_bytes());
            }
        }
    }

    /// Hash key for filesystem safety.
    fn _hash_key(&self, key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Get cache file path for key.
    fn _get_cache_file(&self, key: &str) -> PathBuf {
        let hashed_key = self._hash_key(key);
        self.cache_dir.join(format!("{}.json", hashed_key))
    }

    /// Cleanup old entries if needed.
    fn _cleanup_if_needed(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let mut last_cleanup = self.last_cleanup.lock().unwrap();
        if now - *last_cleanup < self.cleanup_interval as f64 {
            return;
        }
        
        let mut metadata = self.metadata.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // Remove expired entries
        let expired_keys: Vec<String> = metadata.iter()
            .filter(|(_, meta)| {
                meta.get("expires")
                    .and_then(|v| v.as_f64())
                    .map(|expires| expires < now)
                    .unwrap_or(false)
            })
            .map(|(k, _)| k.clone())
            .collect();
        
        for key in &expired_keys {
            self._delete_entry_internal(key, &mut metadata);
        }
        
        // Evict oldest entries if over limit
        if metadata.len() as i64 > self.max_size {
            let mut sorted_items: Vec<(String, f64)> = metadata.iter()
                .map(|(k, meta)| {
                    let last_access = meta.get("last_access")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    (k.clone(), last_access)
                })
                .collect();
            
            sorted_items.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            
            let evict_count = metadata.len() - self.max_size as usize;
            for (key, _) in sorted_items.iter().take(evict_count) {
                self._delete_entry_internal(key, &mut metadata);
                *stats.get_mut("evictions").unwrap() += 1;
            }
        }
        
        *last_cleanup = now;
        drop(metadata);
        self._save_metadata();
    }

    /// Delete cache entry (internal, assumes lock is held).
    fn _delete_entry_internal(&self, key: &str, metadata: &mut HashMap<String, HashMap<String, serde_json::Value>>) {
        let cache_file = self._get_cache_file(key);
        if cache_file.exists() {
            let _ = fs::remove_file(&cache_file);
        }
        metadata.remove(key);
    }

    /// Delete cache entry.
    fn _delete_entry(&self, key: &str) {
        let mut metadata = self.metadata.lock().unwrap();
        self._delete_entry_internal(key, &mut metadata);
        drop(metadata);
        self._save_metadata();
    }
}

impl ICache for DiskCache {
    fn get(&self, key: String) -> Option<serde_json::Value> {
        self._cleanup_if_needed();
        
        let mut metadata = self.metadata.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        if !metadata.contains_key(&key) {
            *stats.get_mut("misses").unwrap() += 1;
            return None;
        }
        
        let meta = metadata.get(&key).unwrap().clone();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        // Check expiration
        if let Some(expires) = meta.get("expires").and_then(|v| v.as_f64()) {
            if expires < now {
                drop(metadata);
                drop(stats);
                self._delete_entry(&key);
                *self.stats.lock().unwrap().get_mut("misses").unwrap() += 1;
                return None;
            }
        }
        
        drop(metadata);
        drop(stats);
        
        // Load from disk
        let cache_file = self._get_cache_file(&key);
        if !cache_file.exists() {
            self._delete_entry(&key);
            *self.stats.lock().unwrap().get_mut("misses").unwrap() += 1;
            return None;
        }
        
        if let Ok(mut file) = fs::File::open(&cache_file) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&contents) {
                    // Update access time
                    let mut metadata = self.metadata.lock().unwrap();
                    if let Some(meta) = metadata.get_mut(&key) {
                        meta.insert("last_access".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
                    }
                    drop(metadata);
                    self._save_metadata();
                    
                    *self.stats.lock().unwrap().get_mut("hits").unwrap() += 1;
                    return Some(value);
                }
            }
        }
        
        *self.stats.lock().unwrap().get_mut("misses").unwrap() += 1;
        None
    }

    fn set(&self, key: String, value: serde_json::Value, ttl: Option<i64>) -> bool {
        self._cleanup_if_needed();
        
        // Serialize value
        let json_str = match serde_json::to_string(&value) {
            Ok(s) => s,
            Err(_) => {
                *self.stats.lock().unwrap().get_mut("errors").unwrap() += 1;
                return false;
            }
        };
        
        // Check file size limit
        if json_str.len() as i64 > self.max_file_size {
            return false;
        }
        
        // Save to disk
        let cache_file = self._get_cache_file(&key);
        if let Ok(mut file) = fs::File::create(&cache_file) {
            if file.write_all(json_str.as_bytes()).is_ok() {
                // Update metadata
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                
                let mut metadata = self.metadata.lock().unwrap();
                let mut meta = HashMap::new();
                meta.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(json_str.len())));
                meta.insert("created".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
                meta.insert("last_access".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
                
                if let Some(ttl_val) = ttl {
                    meta.insert("expires".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now + ttl_val as f64).unwrap()));
                }
                
                metadata.insert(key, meta);
                drop(metadata);
                self._save_metadata();
                
                *self.stats.lock().unwrap().get_mut("sets").unwrap() += 1;
                return true;
            }
        }
        
        *self.stats.lock().unwrap().get_mut("errors").unwrap() += 1;
        false
    }

    fn delete(&self, key: String) -> bool {
        let mut metadata = self.metadata.lock().unwrap();
        if metadata.contains_key(&key) {
            drop(metadata);
            self._delete_entry(&key);
            *self.stats.lock().unwrap().get_mut("deletes").unwrap() += 1;
            true
        } else {
            false
        }
    }

    fn clear(&self) -> bool {
        // Delete all cache files
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.file_name().and_then(|n| n.to_str()).map(|s| s.ends_with(".json")).unwrap_or(false) {
                    let _ = fs::remove_file(&path);
                }
            }
        }
        
        // Clear metadata
        let mut metadata = self.metadata.lock().unwrap();
        metadata.clear();
        drop(metadata);
        self._save_metadata();
        
        true
    }

    fn exists(&self, key: String) -> bool {
        let metadata = self.metadata.lock().unwrap();
        if !metadata.contains_key(&key) {
            return false;
        }
        
        let meta = metadata.get(&key).unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        // Check expiration
        if let Some(expires) = meta.get("expires").and_then(|v| v.as_f64()) {
            if expires < now {
                drop(metadata);
                self._delete_entry(&key);
                return false;
            }
        }
        
        drop(metadata);
        
        // Check file exists
        let cache_file = self._get_cache_file(&key);
        cache_file.exists()
    }

    fn size(&self) -> i64 {
        self.metadata.lock().unwrap().len() as i64
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let metadata = self.metadata.lock().unwrap();
        let stats = self.stats.lock().unwrap();
        
        let hits = *stats.get("hits").unwrap_or(&0);
        let misses = *stats.get("misses").unwrap_or(&0);
        let total_requests = hits + misses;
        let hit_rate = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let mut result = HashMap::new();
        result.insert("namespace".to_string(), serde_json::Value::String(self.namespace.clone()));
        result.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(metadata.len() as i64)));
        result.insert("max_size".to_string(), serde_json::Value::Number(serde_json::Number::from(self.max_size)));
        result.insert("hits".to_string(), serde_json::Value::Number(serde_json::Number::from(hits)));
        result.insert("misses".to_string(), serde_json::Value::Number(serde_json::Number::from(misses)));
        result.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        result.insert("sets".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("sets").unwrap_or(&0))));
        result.insert("deletes".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("deletes").unwrap_or(&0))));
        result.insert("evictions".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("evictions").unwrap_or(&0))));
        result.insert("errors".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("errors").unwrap_or(&0))));
        result.insert("cache_dir".to_string(), serde_json::Value::String(self.cache_dir.to_string_lossy().to_string()));
        
        result
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use DiskCache;
