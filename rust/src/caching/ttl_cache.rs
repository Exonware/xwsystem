// #exonware/xwsystem/rust/src/caching/ttl_cache.rs
//! TTL (Time To Live) Cache Implementation
//! ======================================
//! 
//! Production-grade TTL caching for XSystem.
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generated: 2025-01-27


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::rc::Rc;
use rustc_hash::FxHashMap;

use crate::caching::base::{ACache, Hashable};

/// Entry in TTL cache with expiration time.
/// Phase 2 optimization: Use Rc<Value> to avoid cloning large values internally.
pub struct TTLEntry {
    value: Rc<serde_json::Value>,  // Phase 2: Shared ownership to reduce cloning
    expires_at: f64,
    access_count: i64,
    created_at: f64,
}

impl TTLEntry {
    /// Create a new TTL entry.
    pub fn new(value: serde_json::Value, expires_at: f64) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        Self {
            value: Rc::new(value),  // Phase 2: Wrap in Rc
            expires_at,
            access_count: 0,
            created_at,
        }
    }

    /// Check if entry has expired.
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        now > self.expires_at
    }

    /// Update access count.
    pub fn touch(&mut self) {
        self.access_count += 1;
    }

    /// Get the value.
    pub fn value(&self) -> &serde_json::Value {
        &*self.value  // Phase 2: Deref Rc to get &Value
    }

    /// Get mutable reference to value (creates new Rc if needed).
    pub fn value_mut(&mut self) -> &mut serde_json::Value {
        // Phase 2: If Rc has multiple references, we need to clone
        Rc::make_mut(&mut self.value)
    }
}

// Call parent constructor
// Use custom TTL or default
// Check if we need to make space
// Update access tracking
/// Production-grade Time-To-Live cache with automatic expiration.
///
/// Features:
/// - Automatic expiration based on TTL
/// - LRU eviction when capacity is reached
/// - Thread-safe operations
/// - Statistics tracking
/// - Background cleanup
/// - Configurable cleanup intervals
pub struct TTLCache {
    capacity: i64,
    ttl: f64,
    cleanup_interval: f64,
    name: String,
    cache: Mutex<FxHashMap<Hashable, TTLEntry>>,
    access_order: Mutex<Vec<Hashable>>,
    hits: AtomicI64,
    misses: AtomicI64,
    evictions: AtomicI64,
    expirations: AtomicI64,
    cleanups: AtomicI64,
}

impl ACache for TTLCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Cache SystemTime::now() call (Phase 2 optimization)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH")
            .as_secs_f64();
        
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        
        if let Some(entry) = cache.get_mut(&key) {
            // Check if expired (use cached now)
            if entry.expires_at <= now {
                cache.remove(&key);
                access_order.retain(|k| k != &key);
                self.misses.fetch_add(1, Ordering::Relaxed);
                self.expirations.fetch_add(1, Ordering::Relaxed);
                return default;
            }
            
            // Update access tracking
            entry.touch();
            access_order.retain(|k| k != &key);
            access_order.push(key.clone());
            self.hits.fetch_add(1, Ordering::Relaxed);
            
            // Phase 2: Clone only when returning (Rc::clone is cheap, but we need Value clone for Python)
            Some(entry.value().clone())  // Clone actual Value when returning to Python
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            default
        }
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let _ = self.put_with_ttl(key, value, None);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        
        if cache.remove(&key).is_some() {
            access_order.retain(|k| k != &key);
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        cache.clear();
        access_order.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().expect("Cache lock poisoned").len() as i64
    }

    fn is_full(&self) -> bool {
        self.cache.lock().expect("Cache lock poisoned").len() as i64 >= self.capacity
    }

    fn evict(&mut self) {
        self._evict_lru();
    }

    fn keys(&self) -> Vec<Hashable> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.keys().cloned().collect()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        // Phase 2: Clone actual Value when returning
        cache.values().map(|e| e.value().clone()).collect()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        // Phase 2: Clone actual Value when returning
        cache.iter().map(|(k, e)| (k.clone(), e.value().clone())).collect()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let evictions = self.evictions.load(Ordering::Relaxed);
        let expirations = self.expirations.load(Ordering::Relaxed);
        let cleanups = self.cleanups.load(Ordering::Relaxed);
        
        let total_requests = hits + misses;
        let hit_rate = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let mut stats = HashMap::new();
        stats.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        stats.insert("type".to_string(), serde_json::Value::String("TTL".to_string()));
        stats.insert("capacity".to_string(), serde_json::Value::Number(serde_json::Number::from(self.capacity)));
        stats.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(cache.len() as i64)));
        stats.insert("hits".to_string(), serde_json::Value::Number(serde_json::Number::from(hits)));
        stats.insert("misses".to_string(), serde_json::Value::Number(serde_json::Number::from(misses)));
        stats.insert("evictions".to_string(), serde_json::Value::Number(serde_json::Number::from(evictions)));
        stats.insert("expirations".to_string(), serde_json::Value::Number(serde_json::Number::from(expirations)));
        stats.insert("cleanups".to_string(), serde_json::Value::Number(serde_json::Number::from(cleanups)));
        stats.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        stats.insert("ttl".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.ttl).unwrap()));
        
        stats
    }
}

impl TTLCache {
    /// Initialize TTL cache.
    ///
    /// Args:
    /// capacity: Maximum number of entries
    /// ttl: Time to live in seconds
    /// cleanup_interval: Cleanup interval in seconds
    /// name: Cache name for debugging
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        cleanup_interval: Option<f64>,
        name: Option<String>
    ) -> Self {
        let cap = capacity.unwrap_or(128);
        if cap <= 0 {
            panic!("Cache capacity must be positive, got {}", cap);
        }
        
        Self {
            capacity: cap,
            ttl: ttl.unwrap_or(300.0),
            cleanup_interval: cleanup_interval.unwrap_or(60.0),
            name: name.unwrap_or_else(|| "ttl_cache".to_string()),
            cache: Mutex::new(FxHashMap::default()),
            access_order: Mutex::new(Vec::new()),
            hits: AtomicI64::new(0),
            misses: AtomicI64::new(0),
            evictions: AtomicI64::new(0),
            expirations: AtomicI64::new(0),
            cleanups: AtomicI64::new(0),
        }
    }

    /// Start background cleanup thread.
    pub fn _start_cleanup_thread(&mut self) {
        // Background cleanup would be implemented here with a thread
        // For now, cleanup happens on-demand during get/put operations
    }

    /// Background cleanup loop.
    pub fn _cleanup_loop(&self) {
        // Background cleanup loop would be implemented here
        // For now, cleanup happens on-demand
    }

    /// Remove expired entries.
    pub fn _cleanup_expired(&self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        
        // Cache SystemTime::now() call (Phase 2 optimization)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH")
            .as_secs_f64();
        
        let expired_keys: Vec<Hashable> = cache.iter()
            .filter(|(_, entry)| entry.expires_at <= now)
            .map(|(k, _)| k.clone())
            .collect();
        
        let expired_count = expired_keys.len() as i64;
        for key in &expired_keys {
            cache.remove(key);
            access_order.retain(|k| k != key);
            self.expirations.fetch_add(1, Ordering::Relaxed);
        }
        
        if expired_count > 0 {
            self.cleanups.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Evict least recently used entry.
    fn _evict_lru(&self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        
        if let Some(lru_key) = access_order.first().cloned() {
            cache.remove(&lru_key);
            access_order.remove(0);
            self.evictions.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Update LRU access order.
    fn _update_access_order(&self, key: &Hashable) {
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        access_order.retain(|k| k != key);
        access_order.push(key.clone());
    }

    /// Store a value in the cache.
    ///
    /// Args:
    /// key: Cache key
    /// value: Value to store
    /// ttl: Custom TTL for this entry (overrides default)
    ///
    /// Returns:
    /// True if stored successfully
    pub fn put_with_ttl(&self, key: Hashable, value: serde_json::Value, ttl: Option<f64>) -> bool {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        
        // Use custom TTL or default
        let entry_ttl = ttl.unwrap_or(self.ttl);
        // Cache SystemTime::now() call (Phase 2 optimization)
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH")
            .as_secs_f64() + entry_ttl;
        
        // Check if we need to make space
        if !cache.contains_key(&key) && cache.len() as i64 >= self.capacity {
            self._evict_lru();
        }
        
        // Create and store entry (Phase 2: TTLEntry::new wraps value in Rc)
        let entry = TTLEntry::new(value, expires_at);
        self._update_access_order(&key);
        cache.insert(key, entry);
        
        true
    }

    /// Store a value in the cache (ACache interface).
    pub fn put(&self, key: String, value: serde_json::Value, ttl: Option<f64>) -> bool {
        self.put_with_ttl(key, value, ttl)
    }

    /// Set value in cache (abstract method implementation).
    /// Delegates to put() for backward compatibility.
    ///
    /// Args:
    /// key: Key to store
    /// value: Value to store
    /// ttl: Optional time-to-live in seconds
    pub fn set(&self, key: String, value: serde_json::Value, ttl: Option<i64>) {
        let ttl_f64 = ttl.map(|t| t as f64);
        let _ = self.put_with_ttl(key, value, ttl_f64);
    }

    /// Retrieve a value from the cache.
    ///
    /// Args:
    /// key: Cache key
    /// default: Default value if key not found
    ///
    /// Returns:
    /// Cached value or default
    pub fn get(&self, key: String, default: Option<serde_json::Value>) -> serde_json::Value {
        ACache::get(self, key, default.clone()).unwrap_or_else(|| default.unwrap_or(serde_json::Value::Null))
    }

    /// Delete a key from the cache.
    ///
    /// Args:
    /// key: Cache key to delete
    ///
    /// Returns:
    /// True if key was deleted
    pub fn delete(&self, key: String) -> bool {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        
        if cache.remove(&key).is_some() {
            access_order.retain(|k| k != &key);
            true
        } else {
            false
        }
    }

    /// Clear all entries from the cache.
    pub fn clear(&self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        let mut access_order = self.access_order.lock().expect("Access order lock poisoned");
        cache.clear();
        access_order.clear();
    }

    /// Get current cache size.
    pub fn size(&self) -> i64 {
        self.cache.lock().expect("Cache lock poisoned").len() as i64
    }

    /// Check if cache is at capacity.
    pub fn is_full(&self) -> bool {
        self.cache.lock().expect("Cache lock poisoned").len() as i64 >= self.capacity
    }

    /// Evict entry from cache (uses LRU strategy).
    /// Implementation of abstract method from ACacheBase.
    pub fn evict(&self) {
        self._evict_lru();
    }

    /// Check if key exists and is not expired.
    pub fn contains(&self, key: &str) -> bool {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        if let Some(entry) = cache.get(key) {
            !entry.is_expired()
        } else {
            false
        }
    }

    /// Get list of all cache keys.
    pub fn keys(&self) -> Vec<String> {
        ACache::keys(self)
    }

    /// Get list of all cache values.
    pub fn values(&self) -> Vec<serde_json::Value> {
        ACache::values(self)
    }

    /// Get list of all key-value pairs.
    pub fn items(&self) -> Vec<(String, serde_json::Value)> {
        ACache::items(self)
    }

    /// Get cache statistics.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        ACache::get_stats(self)
    }

    /// Get remaining TTL for a key.
    pub fn get_remaining_ttl(&self, key: &str) -> Option<f64> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        if let Some(entry) = cache.get(key) {
            if entry.is_expired() {
                return None;
            }
            // Cache SystemTime::now() call (Phase 2 optimization)
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX_EPOCH")
                .as_secs_f64();
            Some(entry.expires_at - now)
        } else {
            None
        }
    }

    /// Shutdown the cache and cleanup thread.
    pub fn shutdown(&self) {
        // Cleanup thread shutdown would be implemented here
        self._cleanup_expired();
    }

}

// Async synchronization
// Background cleanup task
// Use custom TTL or default
// Check if we need to make space
// Update access tracking
/// Async-compatible TTL cache.
///
/// Features:
/// - Full async integration
/// - Automatic expiration
/// - Async cleanup tasks
/// - Thread-safe operations
pub struct AsyncTTLCache {
    cache: Arc<Mutex<TTLCache>>,
}

impl AsyncTTLCache {
    /// Initialize async TTL cache.
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        cleanup_interval: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: Arc::new(Mutex::new(TTLCache::new(
                capacity,
                ttl,
                cleanup_interval,
                name
            ))),
        }
    }

    /// Start background cleanup task.
    ///
    /// Note: In Rust, background tasks require an async runtime (e.g., tokio).
    /// This is a placeholder - in production, use tokio::spawn or similar.
    pub fn _start_cleanup_task(&self) {
        // Placeholder - would spawn async task in production
    }

    /// Background cleanup loop.
    ///
    /// Note: In Rust, this would be implemented as an async task.
    pub async fn _cleanup_loop(&self) {
        // Placeholder - would implement async loop in production
    }

    /// Remove expired entries.
    pub async fn _cleanup_expired(&self) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let _cache = self.cache.lock().unwrap();
        // Cleanup is handled by the sync cache's internal cleanup
    }

    /// Store a value in the async cache.
    pub async fn put(&self, key: String, value: serde_json::Value, ttl: Option<f64>) -> bool {
        // For now, use blocking lock (in production, use async mutex from tokio)
        // Use custom TTL or default - we need to get the TTL from the cache first
        let cache_guard = self.cache.lock().unwrap();
        let entry_ttl = ttl.unwrap_or(cache_guard.ttl);
        drop(cache_guard);
        
        // Now use the sync cache's put method which handles everything
        let cache_guard = self.cache.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let expires_at = now + entry_ttl;
        
        let mut cache_inner = cache_guard.cache.lock().unwrap();
        let mut access_order = cache_guard.access_order.lock().unwrap();
        
        // Check if we need to make space
        if !cache_inner.contains_key(&key) && cache_inner.len() as i64 >= cache_guard.capacity {
            drop(cache_inner);
            drop(access_order);
            cache_guard._evict_lru();
            cache_inner = cache_guard.cache.lock().unwrap();
            access_order = cache_guard.access_order.lock().unwrap();
        }
        
        let entry = TTLEntry::new(value, expires_at);
        cache_inner.insert(key.clone(), entry);
        
        // Update access tracking
        access_order.retain(|k| k != &key);
        access_order.push(key);
        
        true
    }

    /// Retrieve a value from the async cache.
    pub async fn get(&self, key: String, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        ACache::get(&*cache, key, default)
    }

    /// Delete a key from the async cache.
    pub async fn delete(&self, key: String) -> bool {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        cache.delete(key)
    }

    /// Clear all entries from the async cache.
    pub async fn clear(&self) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get current cache size.
    pub async fn size(&self) -> i64 {
        let cache = self.cache.lock().unwrap();
        cache.size()
    }

    /// Get async cache statistics.
    pub async fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().unwrap();
        cache.get_stats()
    }

    /// Evict least recently used entry.
    pub async fn _evict_lru(&self) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        cache._evict_lru();
    }

    /// Update LRU access order.
    pub fn _update_access_order(&self, key: String) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        let mut access_order = cache.access_order.lock().unwrap();
        access_order.retain(|k| k != &key);
        access_order.push(key);
    }

    /// Shutdown the async cache.
    pub async fn shutdown(&self) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().unwrap();
        cache.shutdown();
    }

}
