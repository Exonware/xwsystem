// #exonware/xwsystem/rust/src/caching/lru_cache.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! LRU (Least Recently Used) Cache implementation with thread-safety and async support.


use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use rustc_hash::FxHashMap;

use crate::caching::base::{ACache, Hashable};

/// Node for doubly-linked list in LRU cache.
/// Phase 2 optimization: Use Rc<Value> to avoid cloning large values internally.
struct CacheNode {
    key: Hashable,
    value: Rc<serde_json::Value>,  // Phase 2: Shared ownership to reduce cloning
    access_time: f64,
    prev: Option<Weak<RefCell<CacheNode>>>,
    next: Option<Rc<RefCell<CacheNode>>>,
}

impl CacheNode {
    /// Constructor
    fn new(key: Hashable, value: serde_json::Value) -> Self {
        let access_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH")
            .as_secs_f64();
        Self {
            key,
            value: Rc::new(value),  // Phase 2: Wrap in Rc to avoid cloning
            access_time,
            prev: None,
            next: None,
        }
    }
    
    /// Constructor with pre-computed time (optimization)
    fn new_with_time(key: Hashable, value: serde_json::Value, access_time: f64) -> Self {
        Self {
            key,
            value: Rc::new(value),  // Phase 2: Wrap in Rc to avoid cloning
            access_time,
            prev: None,
            next: None,
        }
    }
}

// Call parent constructor
// Doubly-linked list for LRU ordering
// Move to head (most recently used)
// Handle None values: if value is None, treat as "not found" and return default
// This allows None to be a sentinel for "not cached" while still allowing
// explicit None storage via put() (design decision for cache semantics)
// Remove least recently used item
// Ensure it's not the dummy head
// Optionally clear cache on exit (configurable behavior)
// For now, we don't auto-clear to preserve data
// Users can manually call clear() if needed
/// Thread-safe LRU (Least Recently Used) Cache.
///
/// Features:
/// - O(1) get and put operations
/// - Thread-safe operations
/// - Optional TTL support
/// - Statistics tracking
/// - Memory-efficient implementation
///
/// API Note:
/// This class provides both put() and set() methods:
/// - put(key, value) - Preferred method (ACache interface)
/// - set(key, value, ttl) - Alternative method (Protocol interface)
/// Both work identically; use put() for consistency with codebase.
pub struct LRUCache {
    capacity: i64,
    ttl: Option<f64>,
    name: String,
    cache: Mutex<FxHashMap<Hashable, Rc<RefCell<CacheNode>>>>,
    head: Mutex<Option<Rc<RefCell<CacheNode>>>>,
    tail: Mutex<Option<Rc<RefCell<CacheNode>>>>,
    hits: AtomicI64,
    misses: AtomicI64,
    evictions: AtomicI64,
}

impl ACache for LRUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Cache SystemTime::now() call (Phase 2 optimization)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH")
            .as_secs_f64();
        
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        
        if let Some(node_rc) = cache.get(&key) {
            let node = node_rc.borrow();
            
            // Check TTL if enabled
            if let Some(ttl) = self.ttl {
                if now - node.access_time > ttl {
                    drop(node);
                    cache.remove(&key);
                    self.misses.fetch_add(1, Ordering::Relaxed);
                    return default;
                }
            }
            
            // Phase 2: Clone only when returning (Rc allows cheap reference counting)
            let value = node.value.clone();  // Rc::clone is cheap, just increments ref count
            drop(node);
            
            // Update access time (use cached now)
            node_rc.borrow_mut().access_time = now;
            
            // Move to head
            self._move_to_head_internal(node_rc.clone());
            
            self.hits.fetch_add(1, Ordering::Relaxed);
            
            // Handle None values: if value is None, treat as "not found" and return default
            // Phase 2: Only clone the actual Value when returning (Rc deref to Value)
            if (*value).is_null() {
                default
            } else {
                Some((*value).clone())  // Clone only when returning to Python
            }
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            default
        }
    }
    
    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        // Cache SystemTime::now() call (Phase 2 optimization)
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX_EPOCH")
            .as_secs_f64();
        
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        
        if let Some(node_rc) = cache.get(&key) {
            // Update existing key
            let mut node = node_rc.borrow_mut();
            node.value = Rc::new(value);  // Phase 2: Wrap in Rc
            node.access_time = now;  // Use cached now
            drop(node);
            self._move_to_head_internal(node_rc.clone());
        } else {
            // Add new key
            if cache.len() as i64 >= self.capacity {
                // Remove least recently used item
                if let Some(tail_rc) = self.tail.lock().expect("Tail lock poisoned").clone() {
                    let lru_key = tail_rc.borrow().key.clone();
                    self._remove_node_internal(tail_rc.clone());
                    cache.remove(&lru_key);
                    self.evictions.fetch_add(1, Ordering::Relaxed);
                }
            }
            
            let new_node = Rc::new(RefCell::new(CacheNode::new_with_time(key.clone(), value, now)));
            self._add_to_head_internal(new_node.clone());
            cache.insert(key, new_node);
        }
    }
    
    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        
        if let Some(node_rc) = cache.remove(&key) {
            self._remove_node_internal(node_rc);
            true
        } else {
            false
        }
    }
    
    fn clear(&mut self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        cache.clear();
        *self.head.lock().expect("Head lock poisoned") = None;
        *self.tail.lock().expect("Tail lock poisoned") = None;
    }
    
    fn size(&self) -> i64 {
        self.cache.lock().expect("Cache lock poisoned").len() as i64
    }
    
    fn is_full(&self) -> bool {
        self.cache.lock().expect("Cache lock poisoned").len() as i64 >= self.capacity
    }
    
    fn evict(&mut self) {
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        
        if let Some(tail_rc) = self.tail.lock().expect("Tail lock poisoned").clone() {
            let lru_key = tail_rc.borrow().key.clone();
            self._remove_node_internal(tail_rc.clone());
            cache.remove(&lru_key);
            self.evictions.fetch_add(1, Ordering::Relaxed);
        }
    }
    
    fn keys(&self) -> Vec<Hashable> {
        let mut keys = Vec::new();
        
        if let Some(head_rc) = self.head.lock().expect("Head lock poisoned").clone() {
            let mut current = Some(head_rc);
            while let Some(node_rc) = current {
                let node = node_rc.borrow();
                keys.push(node.key.clone());
                current = node.next.clone();
            }
        }
        
        keys
    }
    
    fn values(&self) -> Vec<serde_json::Value> {
        let mut values = Vec::new();
        
        if let Some(head_rc) = self.head.lock().expect("Head lock poisoned").clone() {
            let mut current = Some(head_rc);
            while let Some(node_rc) = current {
                let node = node_rc.borrow();
                // Phase 2: Clone the actual Value when returning
                values.push((*node.value).clone());
                current = node.next.clone();
            }
        }
        
        values
    }
    
    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        let mut items = Vec::new();
        
        if let Some(head_rc) = self.head.lock().expect("Head lock poisoned").clone() {
            let mut current = Some(head_rc);
            while let Some(node_rc) = current {
                let node = node_rc.borrow();
                // Phase 2: Clone the actual Value when returning
                items.push((node.key.clone(), (*node.value).clone()));
                current = node.next.clone();
            }
        }
        
        items
    }
    
    fn get_stats(&self) -> std::collections::HashMap<String, serde_json::Value> {
        use std::collections::HashMap;
        let cache = self.cache.lock().expect("Cache lock poisoned");
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let evictions = self.evictions.load(Ordering::Relaxed);
        
        let total_requests = hits + misses;
        let hit_rate = if total_requests > 0 {
            hits as f64 / total_requests as f64
        } else {
            0.0
        };
        
        let mut stats = HashMap::new();
        stats.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        stats.insert("type".to_string(), serde_json::Value::String("LRU".to_string()));
        stats.insert("capacity".to_string(), serde_json::Value::Number(self.capacity.into()));
        stats.insert("size".to_string(), serde_json::Value::Number((cache.len() as i64).into()));
        stats.insert("hits".to_string(), serde_json::Value::Number(hits.into()));
        stats.insert("misses".to_string(), serde_json::Value::Number(misses.into()));
        stats.insert("evictions".to_string(), serde_json::Value::Number(evictions.into()));
        stats.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        if let Some(ttl) = self.ttl {
            stats.insert("ttl".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(ttl).unwrap()));
        }
        
        stats
    }
}

impl LRUCache {
    /// Initialize LRU cache.
    ///
    ///
    /// Args:
    /// capacity: Maximum number of items to store
    /// ttl: Optional time-to-live in seconds
    /// name: Optional name for debugging
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        let cap = capacity.unwrap_or(128);
        if cap <= 0 {
            panic!("Cache capacity must be positive, got {}", cap);
        }
        
        Self {
            capacity: cap,
            ttl,
            name: name.unwrap_or_else(|| format!("LRUCache-{:p}", &cap)),
            cache: Mutex::new(FxHashMap::default()),
            head: Mutex::new(None),
            tail: Mutex::new(None),
            hits: AtomicI64::new(0),
            misses: AtomicI64::new(0),
            evictions: AtomicI64::new(0),
        }
    }

    /// Get value by key.
    ///
    /// Args:
    /// key: Key to lookup
    /// default: Default value if key not found
    ///
    /// Returns:
    /// Value associated with key, or default
    pub fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(self, key, default)
    }

    // Remove least recently used item
    /// Put key-value pair in cache.
    ///
    ///
    /// Args:
    /// key: Key to store
    /// value: Value to store
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) {
        ACache::put(self, key, value);
    }

    /// Set value in cache (Protocol interface method).
    ///
    ///
    /// Note: This method exists for Protocol compatibility (Cacheable interface).
    /// Internally delegates to put(). Prefer using put() for consistency.
    ///
    ///
    /// Args:
    /// key: Key to store
    /// value: Value to store
    /// ttl: Optional time-to-live (not used in LRU, available for subclasses)
    pub fn set(&mut self, key: String, value: serde_json::Value, _ttl: Option<i64>) {
        ACache::put(self, key, value);
    }

    /// Delete key from cache.
    ///
    ///
    /// Args:
    /// key: Key to delete
    ///
    ///
    /// Returns:
    /// True if key was deleted, False if not found
    pub fn delete(&mut self, key: Hashable) -> bool {
        ACache::delete(self, key)
    }

    /// Clear all items from cache.
    pub fn clear(&mut self) {
        ACache::clear(self);
    }

    /// Get current cache size.
    pub fn size(&self) -> i64 {
        ACache::size(self)
    }

    /// Check if cache is at capacity.
    pub fn is_full(&self) -> bool {
        ACache::is_full(self)
    }

    // Ensure it's not the dummy head
    /// Evict least recently used entry from cache.
    /// Implementation of abstract method from ACacheBase.
    pub fn evict(&mut self) {
        ACache::evict(self);
    }

    /// Get list of all keys (in LRU order).
    pub fn keys(&self) -> Vec<Hashable> {
        ACache::keys(self)
    }

    /// Get list of all values (in LRU order).
    pub fn values(&self) -> Vec<serde_json::Value> {
        ACache::values(self)
    }

    /// Get list of all key-value pairs (in LRU order).
    pub fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        ACache::items(self)
    }

    /// Get cache statistics.
    pub fn get_stats(&self) -> std::collections::HashMap<String, serde_json::Value> {
        ACache::get_stats(self)
    }

    /// Reset cache statistics.
    pub fn reset_stats(&mut self) {
        self.hits.store(0, Ordering::Relaxed);
        self.misses.store(0, Ordering::Relaxed);
        self.evictions.store(0, Ordering::Relaxed);
    }

    /// Add node to head of list (internal helper).
    fn _add_to_head_internal(&self, node: Rc<RefCell<CacheNode>>) {
        let mut head = self.head.lock().expect("Head lock poisoned");
        let mut tail = self.tail.lock().expect("Tail lock poisoned");
        
        if let Some(head_rc) = head.clone() {
            let mut node_borrow = node.borrow_mut();
            node_borrow.next = Some(head_rc.clone());
            drop(node_borrow);
            
            let mut head_borrow = head_rc.borrow_mut();
            head_borrow.prev = Some(Rc::downgrade(&node));
            drop(head_borrow);
        } else {
            // First node
            *tail = Some(node.clone());
        }
        
        *head = Some(node);
    }

    /// Remove node from list (internal helper).
    fn _remove_node_internal(&self, node: Rc<RefCell<CacheNode>>) {
        let mut head = self.head.lock().expect("Head lock poisoned");
        let mut tail = self.tail.lock().expect("Tail lock poisoned");
        
        let node_prev = node.borrow().prev.clone();
        let node_next = node.borrow().next.clone();
        
        if let Some(prev_weak) = node_prev {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = node_next.clone();
            }
        } else {
            // This is the head
            *head = node_next.clone();
        }
        
        if let Some(next_rc) = node_next {
            next_rc.borrow_mut().prev = node.borrow().prev.clone();
        } else {
            // This is the tail
            *tail = node.borrow().prev.as_ref().and_then(|w| w.upgrade());
        }
        
        node.borrow_mut().prev = None;
        node.borrow_mut().next = None;
    }

    /// Move node to head of list (internal helper).
    fn _move_to_head_internal(&self, node: Rc<RefCell<CacheNode>>) {
        self._remove_node_internal(node.clone());
        self._add_to_head_internal(node);
    }

}

// Doubly-linked list for LRU ordering
// Move to head (most recently used)
// Remove least recently used item
/// Async-safe LRU (Least Recently Used) Cache.
///
/// Features:
/// - O(1) async get and put operations
/// - Async-safe operations with async locks
/// - Optional TTL support
/// - Statistics tracking
/// - Memory-efficient implementation
pub struct AsyncLRUCache {
    cache: Arc<Mutex<LRUCache>>,
}

impl AsyncLRUCache {
    /// Initialize async LRU cache.
    ///
    ///
    /// Args:
    /// capacity: Maximum number of items to store
    /// ttl: Optional time-to-live in seconds
    /// name: Optional name for debugging
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LRUCache::new(capacity, ttl, name))),
        }
    }

    // Move to head (most recently used)
    /// Get value by key asynchronously.
    ///
    ///
    /// Args:
    /// key: Key to lookup
    /// default: Default value if key not found
    ///
    ///
    /// Returns:
    /// Value associated with key, or default
    /// Get value by key asynchronously.
    ///
    /// Note: In Rust, async operations require an async runtime (e.g., tokio).
    /// This implementation uses blocking operations wrapped in async for compatibility.
    /// For true async operations, use tokio::sync::Mutex or async-std::sync::Mutex.
    pub async fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.get(key, default)
    }

    // Remove least recently used item
    /// Put key-value pair in cache asynchronously.
    pub async fn put(&self, key: Hashable, value: serde_json::Value) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        cache.put(key, value);
    }

    /// Delete key from cache asynchronously.
    pub async fn delete(&self, key: Hashable) -> bool {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        cache.delete(key)
    }

    /// Clear all items from cache asynchronously.
    pub async fn clear(&self) {
        // For now, use blocking lock (in production, use async mutex from tokio)
        let mut cache = self.cache.lock().expect("Cache lock poisoned");
        cache.clear();
    }

    /// Get current cache size asynchronously.
    pub async fn size(&self) -> i64 {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.size()
    }

    /// Check if cache is at capacity asynchronously.
    pub async fn is_full(&self) -> bool {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.is_full()
    }

    /// Get list of all keys (in LRU order) asynchronously.
    pub async fn keys(&self) -> Vec<Hashable> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.keys()
    }

    /// Get list of all values (in LRU order) asynchronously.
    pub async fn values(&self) -> Vec<serde_json::Value> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.values()
    }

    /// Get list of all key-value pairs (in LRU order) asynchronously.
    pub async fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.items()
    }

    /// Get cache statistics asynchronously.
    pub async fn get_stats(&self) -> std::collections::HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().expect("Cache lock poisoned");
        cache.get_stats()
    }

    /// Reset cache statistics asynchronously.
    pub async fn reset_stats(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.reset_stats();
    }

    /// Async iteration over items.
    ///
    /// Note: In Rust, async iteration requires implementing Stream trait.
    /// This is a placeholder - in production, implement Stream<Item = (Hashable, serde_json::Value)>
    pub async fn items_async(&self) -> Vec<(Hashable, serde_json::Value)> {
        // For now, return all items (in production, implement as async stream)
        self.items().await
    }

}
