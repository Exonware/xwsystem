// #exonware/xwsystem/rust/src/caching/bloom_cache.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Bloom filter-enhanced cache for faster negative lookups.
//! Performance Priority #4 - Probabilistic data structure for efficiency.


use std::collections::HashMap;
use std::sync::Mutex;
use sha2::{Sha256, Digest};
use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;

// Use different hash functions
/// Simple Bloom filter implementation.
///
/// Provides probabilistic membership testing with no false negatives.
pub struct SimpleBloomFilter {
    size: i64,
    hash_count: i64,
    bit_array: Mutex<Vec<bool>>,
    items_added: Mutex<i64>,
}

impl SimpleBloomFilter {
    /// Initialize Bloom filter.
    ///
    /// Args:
    /// size: Bit array size
    /// hash_count: Number of hash functions to use
    pub fn new(
        size: Option<i64>,
        hash_count: Option<i64>
    ) -> Self {
        let size_val = size.unwrap_or(10000);
        let hash_count_val = hash_count.unwrap_or(3);
        Self {
            size: size_val,
            hash_count: hash_count_val,
            bit_array: Mutex::new(vec![false; size_val as usize]),
            items_added: Mutex::new(0),
        }
    }

    /// Generate multiple hash values for item.
    fn _hashes(&self, item: &Hashable) -> Vec<i64> {
        let mut hashes = Vec::new();
        let item_str = item.clone();
        let item_bytes = item_str.as_bytes();
        
        for i in 0..self.hash_count {
            // Use different hash functions by appending index
            let mut hasher = Sha256::new();
            hasher.update(item_bytes);
            hasher.update(i.to_string().as_bytes());
            let hash_bytes = hasher.finalize();
            
            // Convert to i64 and mod by size
            let hash_val = u64::from_be_bytes([
                hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
                hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
            ]) as i64 % self.size;
            hashes.push(hash_val);
        }
        
        hashes
    }

    /// Add item to Bloom filter.
    pub fn add(&self, item: Hashable) {
        let hashes = self._hashes(&item);
        let mut bit_array = self.bit_array.lock().unwrap();
        for hash_val in hashes {
            bit_array[hash_val as usize] = true;
        }
        *self.items_added.lock().unwrap() += 1;
    }

    /// Check if item might be in set.
    ///
    /// Returns:
    /// True if item might be present (or false positive)
    /// False if item is definitely NOT present
    pub fn might_contain(&self, item: &Hashable) -> bool {
        let hashes = self._hashes(item);
        let bit_array = self.bit_array.lock().unwrap();
        hashes.iter().all(|&h| bit_array[h as usize])
    }

    /// Clear all items.
    pub fn clear(&self) {
        let mut bit_array = self.bit_array.lock().unwrap();
        bit_array.fill(false);
        *self.items_added.lock().unwrap() = 0;
    }

    /// Get Bloom filter size.
    pub fn size(&self) -> i64 {
        self.size
    }

    /// Get number of items added.
    pub fn items_added(&self) -> i64 {
        *self.items_added.lock().unwrap()
    }
}

// Initialize Bloom filter
// Bloom said "yes", was in cache
// Bloom said "yes", wasn't in cache (false positive)
// Bloom said "no" (definitely not in cache)
// Fast negative lookup (no lock needed)
// Bloom filter says might be present - check cache
// Calculate Bloom filter efficiency
/// LRU Cache enhanced with Bloom filter for fast negative lookups.
///
/// Uses Bloom filter to quickly determine if a key is definitely not in cache,
/// avoiding expensive cache lookups for non-existent keys.
pub struct BloomFilterCache {
    cache: Mutex<LRUCache>,
    bloom: SimpleBloomFilter,
    bloom_hits: Mutex<i64>,
    bloom_misses: Mutex<i64>,
    bloom_negatives: Mutex<i64>,
}

impl ACache for BloomFilterCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Fast negative lookup (no lock needed)
        if !self.bloom.might_contain(&key) {
            *self.bloom_negatives.lock().unwrap() += 1;
            return default;
        }
        
        // Bloom filter says might be present - check cache
        let value = self.cache.lock().unwrap().get(key.clone(), default.clone());
        
        if value.is_some() && value != default {
            *self.bloom_hits.lock().unwrap() += 1;
        } else {
            *self.bloom_misses.lock().unwrap() += 1; // False positive
        }
        
        value
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        let mut cache = self.cache.lock().unwrap();
        cache.put(key.clone(), value);
        drop(cache);
        self.bloom.add(key);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        // Note: Bloom filter is not updated (can't remove from Bloom filter)
        let mut cache = self.cache.lock().unwrap();
        cache.delete(key)
    }

    fn clear(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        drop(cache);
        self.bloom.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        cache.evict();
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
        let bloom_hits = *self.bloom_hits.lock().unwrap();
        let bloom_misses = *self.bloom_misses.lock().unwrap();
        let bloom_negatives = *self.bloom_negatives.lock().unwrap();
        
        stats.insert("bloom_size".to_string(), serde_json::Value::Number(serde_json::Number::from(self.bloom.size())));
        stats.insert("bloom_items".to_string(), serde_json::Value::Number(serde_json::Number::from(self.bloom.items_added())));
        stats.insert("bloom_hits".to_string(), serde_json::Value::Number(serde_json::Number::from(bloom_hits)));
        stats.insert("bloom_misses".to_string(), serde_json::Value::Number(serde_json::Number::from(bloom_misses)));
        stats.insert("bloom_negatives".to_string(), serde_json::Value::Number(serde_json::Number::from(bloom_negatives)));
        
        // Calculate Bloom filter efficiency
        let total_bloom = bloom_hits + bloom_misses;
        if total_bloom > 0 {
            let false_positive_rate = bloom_misses as f64 / total_bloom as f64;
            stats.insert("bloom_false_positive_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(false_positive_rate).unwrap()));
        }
        
        stats
    }
}

impl BloomFilterCache {
    /// Initialize Bloom filter cache.
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// bloom_size: Bloom filter size (default: capacity * 10)
    /// ttl: Optional TTL in seconds
    /// name: Cache name
    pub fn new(
        capacity: Option<i64>,
        bloom_size: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>
    ) -> Self {
        let cap = capacity.unwrap_or(128);
        let bloom_sz = bloom_size.unwrap_or(cap * 10);
        
        Self {
            cache: Mutex::new(LRUCache::new(Some(cap), ttl, name)),
            bloom: SimpleBloomFilter::new(Some(bloom_sz), Some(3)),
            bloom_hits: Mutex::new(0),
            bloom_misses: Mutex::new(0),
            bloom_negatives: Mutex::new(0),
        }
    }

    /// Check if key might be in cache (fast, no lock).
    ///
    /// Args:
    /// key: Key to check
    ///
    /// Returns:
    /// True if key might be in cache
    /// False if key is definitely NOT in cache
    pub fn might_contain(&self, key: &Hashable) -> bool {
        self.bloom.might_contain(key)
    }

    /// Rebuild Bloom filter from current cache entries.
    pub fn rebuild_bloom_filter(&self) {
        self.bloom.clear();
        let keys = self.cache.lock().unwrap().keys();
        for key in keys {
            self.bloom.add(key);
        }
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
