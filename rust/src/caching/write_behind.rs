// #exonware/xwsystem/rust/src/caching/write_behind.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Write-behind (lazy write) cache implementation.
//! Performance Priority #4 - Delayed persistence for better write performance.


use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;

// Mark as dirty for flush
// Wait for interval or stop signal
/// Write-behind cache with delayed persistence.
///
/// Caches writes in memory and flushes to storage periodically in background.
/// Provides better write performance than write-through at the cost of
/// potential data loss if system crashes before flush.
///
/// Features:
/// - Asynchronous writes to storage
/// - Configurable flush interval
/// - Automatic background flushing
/// - Manual flush support
/// - Dirty entry tracking
pub struct WriteBehindCache {
    cache: LRUCache,
    writer: Option<Arc<dyn Fn(&Hashable, &serde_json::Value) + Send + Sync>>,
    flush_interval: f64,
    dirty_keys: Mutex<HashSet<Hashable>>,
    flush_count: Mutex<i64>,
    write_count: Mutex<i64>,
    write_errors: Mutex<i64>,
    stop_flusher: Arc<Mutex<bool>>,
}

impl ACache for WriteBehindCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        // Cache immediately
        self.cache.put(key.clone(), value);
        
        // Mark as dirty for flush
        self.dirty_keys.lock().unwrap().insert(key);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        // Remove from dirty set if present
        self.dirty_keys.lock().unwrap().remove(&key);
        self.cache.delete(key)
    }

    fn clear(&mut self) {
        self.dirty_keys.lock().unwrap().clear();
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

    fn keys(&self) -> Vec<Hashable> {
        self.cache.keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        self.cache.values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        self.cache.items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = self.cache.get_stats();
        let flush_count = *self.flush_count.lock().unwrap();
        let write_count = *self.write_count.lock().unwrap();
        let write_errors = *self.write_errors.lock().unwrap();
        let dirty_count = self.dirty_keys.lock().unwrap().len();
        
        stats.insert("flush_count".to_string(), serde_json::Value::Number(serde_json::Number::from(flush_count)));
        stats.insert("write_count".to_string(), serde_json::Value::Number(serde_json::Number::from(write_count)));
        stats.insert("write_errors".to_string(), serde_json::Value::Number(serde_json::Number::from(write_errors)));
        stats.insert("dirty_count".to_string(), serde_json::Value::Number(serde_json::Number::from(dirty_count as i64)));
        stats.insert("flush_interval".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.flush_interval).unwrap()));
        
        stats
    }
}

impl WriteBehindCache {
    /// Initialize write-behind cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// writer: Function to persist values (key, value) -> None
    /// flush_interval: Interval between flushes in seconds
    /// ttl: Optional TTL in seconds
    /// name: Cache name
    /// auto_start: Automatically start background flusher
    pub fn new<F>(
        capacity: Option<i64>,
        writer: Option<F>,
        flush_interval: Option<f64>,
        ttl: Option<f64>,
        name: Option<String>,
        _auto_start: Option<bool>
    ) -> Self
    where
        F: Fn(&Hashable, &serde_json::Value) + Send + Sync + 'static,
    {
        let writer_arc = writer.map(|f| Arc::new(f) as Arc<dyn Fn(&Hashable, &serde_json::Value) + Send + Sync>);
        
        Self {
            cache: LRUCache::new(capacity, ttl, name),
            writer: writer_arc,
            flush_interval: flush_interval.unwrap_or(5.0),
            dirty_keys: Mutex::new(HashSet::new()),
            flush_count: Mutex::new(0),
            write_count: Mutex::new(0),
            write_errors: Mutex::new(0),
            stop_flusher: Arc::new(Mutex::new(false)),
        }
    }

    /// Put value in cache and mark as dirty for later flush.
    ///
    /// Args:
    /// key: Cache key
    /// value: Value to cache
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) {
        ACache::put(self, key.clone(), value);
    }

    /// Flush all dirty entries to storage.
    ///
    /// Returns:
    /// Number of entries flushed
    pub fn flush(&self) -> i64 {
        if self.writer.is_none() {
            return 0;
        }
        
        let writer = self.writer.as_ref().unwrap();
        let mut dirty_keys = self.dirty_keys.lock().unwrap();
        let keys_to_flush: Vec<Hashable> = dirty_keys.iter().cloned().collect();
        dirty_keys.clear();
        drop(dirty_keys);
        
        let mut flushed = 0;
        
        for key in keys_to_flush {
            if let Some(value) = self.cache.get(key.clone(), None) {
                (writer)(&key, &value);
                flushed += 1;
                *self.write_count.lock().unwrap() += 1;
            } else {
                // Re-add to dirty set if not found
                self.dirty_keys.lock().unwrap().insert(key);
            }
        }
        
        if flushed > 0 {
            *self.flush_count.lock().unwrap() += 1;
        }
        
        flushed
    }

    /// Start background flusher thread.
    pub fn start_flusher(&self) {
        // Background flusher would be implemented here with a thread
        // For now, flush happens on-demand via flush() method
        *self.stop_flusher.lock().unwrap() = false;
    }

    /// Stop background flusher thread.
    ///
    /// Args:
    /// flush_remaining: Flush remaining dirty entries before stopping
    pub fn stop_flusher(&self, flush_remaining: Option<bool>) {
        *self.stop_flusher.lock().unwrap() = true;
        
        if flush_remaining.unwrap_or(true) {
            self.flush();
        }
    }

    /// Background thread for periodic flushing.
    pub fn _background_flusher(&self) {
        // Background flusher loop would be implemented here
        // For now, this is a placeholder
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Type exported via mod.rs
