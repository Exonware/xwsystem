// #exonware/xwsystem/rust/src/caching/tagging.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Tag-based cache invalidation.
//! Extensibility Priority #5 - Flexible invalidation patterns.


use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;

// Remove from tag mappings
// Clean up empty tag sets
// Get all keys with this tag
/// Cache with tag-based invalidation support.
///
/// Allows associating tags with cache entries for bulk invalidation.
///
/// Example:
/// ```rust
/// let mut cache = TaggedCache::new(Some(1000), None, None);
///
/// // Tag entries
/// cache.put("user:1".to_string(), serde_json::json!("user1_data"), Some(vec!["user".to_string(), "active".to_string()]));
/// cache.put("user:2".to_string(), serde_json::json!("user2_data"), Some(vec!["user".to_string(), "inactive".to_string()]));
///
/// // Invalidate all user entries
/// cache.invalidate_by_tag("user".to_string());
/// ```
pub struct TaggedCache {
    cache: LRUCache,
    key_to_tags: Mutex<HashMap<Hashable, HashSet<String>>>,
    tag_to_keys: Mutex<HashMap<String, HashSet<Hashable>>>,
    invalidations: Mutex<i64>,
}

impl ACache for TaggedCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        self.cache.get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        self.cache.put(key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        self._remove_from_tags(&key);
        self.cache.delete(key)
    }

    fn clear(&mut self) {
        let mut key_to_tags = self.key_to_tags.lock().unwrap();
        let mut tag_to_keys = self.tag_to_keys.lock().unwrap();
        key_to_tags.clear();
        tag_to_keys.clear();
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
        let invalidations = *self.invalidations.lock().unwrap();
        let tag_count = self.tag_to_keys.lock().unwrap().len();
        stats.insert("invalidations".to_string(), serde_json::Value::Number(serde_json::Number::from(invalidations)));
        stats.insert("tag_count".to_string(), serde_json::Value::Number(serde_json::Number::from(tag_count as i64)));
        stats
    }
}

impl TaggedCache {
    /// Initialize tagged cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// ttl: Optional TTL in seconds
    /// name: Cache name
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        Self {
            cache: LRUCache::new(capacity, ttl, name),
            key_to_tags: Mutex::new(HashMap::new()),
            tag_to_keys: Mutex::new(HashMap::new()),
            invalidations: Mutex::new(0),
        }
    }

    /// Put value with optional tags.
    ///
    /// Args:
    /// key: Cache key
    /// value: Value to cache
    /// tags: List of tags to associate with entry
    pub fn put(&mut self, key: Hashable, value: serde_json::Value, tags: Option<Vec<String>>) {
        // Store in cache
        ACache::put(self, key.clone(), value);
        
        // Update tags
        if let Some(tag_list) = tags {
            let mut key_to_tags = self.key_to_tags.lock().unwrap();
            let mut tag_to_keys = self.tag_to_keys.lock().unwrap();
            
            // Remove old tags for this key
            if let Some(old_tags) = key_to_tags.get(&key) {
                for tag in old_tags {
                    tag_to_keys.get_mut(tag).map(|keys| keys.remove(&key));
                }
            }
            
            // Add new tags
            let tag_set: HashSet<String> = tag_list.into_iter().collect();
            key_to_tags.insert(key.clone(), tag_set.clone());
            
            for tag in &tag_set {
                tag_to_keys.entry(tag.clone())
                    .or_insert_with(HashSet::new)
                    .insert(key.clone());
            }
        }
    }

    /// Delete key and remove from tag mappings.
    ///
    /// Args:
    /// key: Key to delete
    ///
    /// Returns:
    /// True if deleted
    pub fn delete(&mut self, key: Hashable) -> bool {
        self._remove_from_tags(&key);
        ACache::delete(self, key)
    }

    fn _remove_from_tags(&self, key: &Hashable) {
        let mut key_to_tags = self.key_to_tags.lock().unwrap();
        let mut tag_to_keys = self.tag_to_keys.lock().unwrap();
        
        if let Some(tags) = key_to_tags.remove(key) {
            for tag in tags {
                if let Some(keys) = tag_to_keys.get_mut(&tag) {
                    keys.remove(key);
                    // Clean up empty tag sets
                    if keys.is_empty() {
                        tag_to_keys.remove(&tag);
                    }
                }
            }
        }
    }

    /// Invalidate all entries with a specific tag.
    ///
    /// Args:
    /// tag: Tag to invalidate
    ///
    /// Returns:
    /// Number of entries invalidated
    pub fn invalidate_by_tag(&mut self, tag: String) -> i64 {
        let tag_to_keys = self.tag_to_keys.lock().unwrap();
        
        if let Some(keys) = tag_to_keys.get(&tag) {
            let keys_to_delete: Vec<Hashable> = keys.iter().cloned().collect();
            drop(tag_to_keys);
            
            let mut count = 0;
            for key in keys_to_delete {
                if self.delete(key) {
                    count += 1;
                }
            }
            
            *self.invalidations.lock().unwrap() += count;
            count
        } else {
            0
        }
    }

    /// Invalidate all entries with any of the specified tags.
    ///
    /// Args:
    /// tags: List of tags to invalidate
    ///
    /// Returns:
    /// Number of entries invalidated
    pub fn invalidate_by_tags(&mut self, tags: Vec<String>) -> i64 {
        let tag_to_keys = self.tag_to_keys.lock().unwrap();
        let mut keys_to_delete = HashSet::new();
        
        for tag in &tags {
            if let Some(keys) = tag_to_keys.get(tag) {
                keys_to_delete.extend(keys.iter().cloned());
            }
        }
        drop(tag_to_keys);
        
        let keys_vec: Vec<Hashable> = keys_to_delete.into_iter().collect();
        let mut count = 0;
        for key in keys_vec {
            if self.delete(key) {
                count += 1;
            }
        }
        
        *self.invalidations.lock().unwrap() += count;
        count
    }

    /// Get all keys associated with a tag.
    ///
    /// Args:
    /// tag: Tag to query
    ///
    /// Returns:
    /// Set of keys with this tag
    pub fn get_keys_by_tag(&self, tag: String) -> HashSet<Hashable> {
        let tag_to_keys = self.tag_to_keys.lock().unwrap();
        tag_to_keys.get(&tag)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    /// Get all tags associated with a key.
    ///
    /// Args:
    /// key: Key to query
    ///
    /// Returns:
    /// Set of tags for this key
    pub fn get_tags(&self, key: Hashable) -> HashSet<String> {
        let key_to_tags = self.key_to_tags.lock().unwrap();
        key_to_tags.get(&key)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    /// Get all tags in cache.
    ///
    /// Returns:
    /// Set of all tags
    pub fn get_all_tags(&self) -> HashSet<String> {
        let tag_to_keys = self.tag_to_keys.lock().unwrap();
        tag_to_keys.keys().cloned().collect()
    }

    /// Clear cache and all tag mappings.
    pub fn clear(&mut self) {
        ACache::clear(self);
    }

}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Type exported via mod.rs
