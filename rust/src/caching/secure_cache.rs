// #exonware/xwsystem/rust/src/caching/secure_cache.rs
//exonware/xwsystem/caching/secure_cache.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Secure cache implementations with validation, integrity checks, and rate limiting.
//! Security Priority #1 - Production-grade security features.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::caching::base::{ACache, Hashable};
use crate::caching::errors::{CacheIntegrityError, CacheRateLimitError, CacheValidationError};
use crate::caching::integrity::{CacheEntry, create_secure_entry, verify_entry_integrity};
use crate::caching::lfu_cache::LFUCache;
use crate::caching::lru_cache::LRUCache;
use crate::caching::rate_limiter::RateLimiter;
use crate::caching::ttl_cache::TTLCache;
use crate::caching::validation::{DEFAULT_MAX_KEY_SIZE, DEFAULT_MAX_VALUE_SIZE_MB, validate_cache_key, validate_cache_value};

// Track integrity violations
// Re-raise with additional context
// Store with integrity if enabled
// Wrap value in secure entry
// Verify integrity if enabled
// Log violation and remove corrupted entry
/// LRU Cache with security features.
///
/// Additional security features:
/// - Input validation (keys and values)
/// - Integrity verification with checksums
/// - Rate limiting to prevent DoS
/// - Optional mode for performance vs security tradeoff
pub struct SecureLRUCache {
    cache: Mutex<LRUCache>,
    rate_limiter: Option<RateLimiter>,
    enable_integrity: bool,
    enable_rate_limit: bool,
    max_key_size: i64,
    max_value_size_mb: f64,
    integrity_violations: Mutex<i64>,
}

impl SecureLRUCache {
    /// Initialize secure LRU cache.
    ///
    ///
    /// Args:
    /// capacity: Maximum cache size
    /// ttl: Optional TTL in seconds
    /// name: Cache name for debugging
    /// enable_integrity: Enable integrity verification
    /// enable_rate_limit: Enable rate limiting
    /// max_ops_per_second: Maximum operations per second
    /// max_key_size: Maximum key size in bytes
    /// max_value_size_mb: Maximum value size in MB
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        name: Option<String>,
        enable_integrity: Option<bool>,
        enable_rate_limit: Option<bool>,
        max_ops_per_second: Option<i64>,
        max_key_size: Option<i64>,
        max_value_size_mb: Option<f64>
    ) -> Self {
        let enable_integrity_val = enable_integrity.unwrap_or(true);
        let enable_rate_limit_val = enable_rate_limit.unwrap_or(true);
        let max_ops = max_ops_per_second.unwrap_or(10000);
        
        Self {
            cache: Mutex::new(LRUCache::new(capacity, ttl, name)),
            rate_limiter: if enable_rate_limit_val {
                Some(RateLimiter::new(Some(max_ops), None, None))
            } else {
                None
            },
            enable_integrity: enable_integrity_val,
            enable_rate_limit: enable_rate_limit_val,
            max_key_size: max_key_size.unwrap_or(DEFAULT_MAX_KEY_SIZE),
            max_value_size_mb: max_value_size_mb.unwrap_or(DEFAULT_MAX_VALUE_SIZE_MB as f64),
            integrity_violations: Mutex::new(0),
        }
    }

    /// Check rate limit if enabled.
    fn _check_rate_limit(&self) -> Result<(), CacheRateLimitError> {
        if self.enable_rate_limit {
            if let Some(ref limiter) = self.rate_limiter {
                limiter.acquire(None)?;
            }
        }
        Ok(())
    }
}

impl ACache for SecureLRUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        // Rate limiting
        if let Err(_) = self._check_rate_limit() {
            return default;
        }
        
        // Get from cache
        let cache = self.cache.lock().unwrap();
        let result = cache.get(key.clone(), default.clone());
        
        if result == default {
            return default;
        }
        
        // Verify integrity if enabled
        if self.enable_integrity {
            // Try to deserialize as CacheEntry
            if let Some(entry_json) = &result {
                if let Ok(entry) = serde_json::from_value::<CacheEntry>(entry_json.clone()) {
                    match verify_entry_integrity(&entry) {
                        Ok(_) => {
                            // Return the value from the entry
                            return Some(entry.value);
                        }
                        Err(_) => {
                            // Log violation and remove corrupted entry
                            *self.integrity_violations.lock().unwrap() += 1;
                            drop(cache);
                            self.cache.lock().unwrap().delete(key);
                            return default;
                        }
                    }
                }
            }
        }
        
        result
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        // Rate limiting
        if let Err(_) = self._check_rate_limit() {
            return;
        }
        
        // Validate key
        let key_json = serde_json::Value::String(key.clone());
        if let Err(_) = validate_cache_key(&key_json, Some(self.max_key_size), Some(false)) {
            return;
        }
        
        // Validate value
        if let Err(_) = validate_cache_value(&value, Some(self.max_value_size_mb), None) {
            return;
        }
        
        // Store with integrity if enabled
        let mut cache = self.cache.lock().unwrap();
        if self.enable_integrity {
            // Wrap value in secure entry
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            let entry = create_secure_entry(
                serde_json::Value::String(key.clone()),
                value,
                now,
                None
            );
            ACache::put(&mut *cache, key.clone(), serde_json::json!(entry));
        } else {
            ACache::put(&mut *cache, key, value);
        }
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache = self.cache.lock().unwrap();
        cache.delete(key)
    }

    fn clear(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    fn size(&self) -> i64 {
        let cache = self.cache.lock().unwrap();
        cache.size()
    }

    fn is_full(&self) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.is_full()
    }

    fn evict(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        cache.evict();
    }

    fn keys(&self) -> Vec<Hashable> {
        let cache = self.cache.lock().unwrap();
        cache.keys()
    }

    fn values(&self) -> Vec<serde_json::Value> {
        let cache = self.cache.lock().unwrap();
        cache.values()
    }

    fn items(&self) -> Vec<(Hashable, serde_json::Value)> {
        let cache = self.cache.lock().unwrap();
        cache.items()
    }

    fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let cache = self.cache.lock().unwrap();
        let mut stats = cache.get_stats();
        stats.insert("enable_integrity".to_string(), serde_json::Value::Bool(self.enable_integrity));
        stats.insert("enable_rate_limit".to_string(), serde_json::Value::Bool(self.enable_rate_limit));
        stats.insert("integrity_violations".to_string(), serde_json::Value::Number(serde_json::Number::from(*self.integrity_violations.lock().unwrap())));
        stats.insert("max_key_size".to_string(), serde_json::Value::Number(serde_json::Number::from(self.max_key_size)));
        stats.insert("max_value_size_mb".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.max_value_size_mb).unwrap()));
        
        if let Some(ref limiter) = self.rate_limiter {
            let limiter_stats = limiter.get_stats();
            stats.insert("rate_limiter".to_string(), serde_json::json!(limiter_stats));
        }
        
        stats
    }
}

impl SecureLRUCache {
    /// Get security-related statistics.
    pub fn get_security_stats(&self) -> HashMap<String, serde_json::Value> {
        self.get_stats()
    }
}

/// LFU Cache with security features.
///
/// Additional security features:
/// - Input validation (keys and values)
/// - Rate limiting to prevent DoS
pub struct SecureLFUCache {
    cache: Mutex<LFUCache>,
    rate_limiter: Option<RateLimiter>,
    enable_rate_limit: bool,
    max_key_size: i64,
    max_value_size_mb: f64,
}

impl ACache for SecureLFUCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        if let Err(_) = self._check_rate_limit() {
            return default;
        }
        let cache = self.cache.lock().unwrap();
        cache.get(key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        if let Err(_) = self._check_rate_limit() {
            return;
        }
        
        let key_json = serde_json::Value::String(key.clone());
        if let Err(_) = validate_cache_key(&key_json, Some(self.max_key_size), Some(false)) {
            return;
        }
        if let Err(_) = validate_cache_value(&value, Some(self.max_value_size_mb), None) {
            return;
        }
        
        let mut cache = self.cache.lock().unwrap();
        ACache::put(&mut *cache, key, value);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let mut cache = self.cache.lock().unwrap();
        cache.delete(key)
    }

    fn clear(&mut self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
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
        self.cache.lock().unwrap().get_stats()
    }
}

impl SecureLFUCache {
    /// Initialize secure LFU cache.
    pub fn new(
        capacity: Option<i64>,
        name: Option<String>,
        enable_rate_limit: Option<bool>,
        max_ops_per_second: Option<i64>,
        max_key_size: Option<i64>,
        max_value_size_mb: Option<f64>
    ) -> Self {
        let enable_rate_limit_val = enable_rate_limit.unwrap_or(true);
        let max_ops = max_ops_per_second.unwrap_or(10000);
        
        Self {
            cache: Mutex::new(LFUCache::new(capacity, name)),
            rate_limiter: if enable_rate_limit_val {
                Some(RateLimiter::new(Some(max_ops), None, None))
            } else {
                None
            },
            enable_rate_limit: enable_rate_limit_val,
            max_key_size: max_key_size.unwrap_or(DEFAULT_MAX_KEY_SIZE),
            max_value_size_mb: max_value_size_mb.unwrap_or(DEFAULT_MAX_VALUE_SIZE_MB as f64),
        }
    }

    /// Check rate limit if enabled.
    fn _check_rate_limit(&self) -> Result<(), CacheRateLimitError> {
        if self.enable_rate_limit {
            if let Some(ref limiter) = self.rate_limiter {
                limiter.acquire(None)?;
            }
        }
        Ok(())
    }

    /// Put value with security checks.
    pub fn put(&mut self, key: Hashable, value: serde_json::Value) {
        ACache::put(self, key, value);
    }

    /// Get value with rate limiting.
    pub fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(self, key, default)
    }
}

/// TTL Cache with security features.
///
/// Additional security features:
/// - Input validation (keys and values)
/// - Rate limiting to prevent DoS
pub struct SecureTTLCache {
    cache: Mutex<TTLCache>,
    rate_limiter: Option<RateLimiter>,
    enable_rate_limit: bool,
    max_key_size: i64,
    max_value_size_mb: f64,
}

impl ACache for SecureTTLCache {
    fn get(&self, key: Hashable, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        if let Err(_) = self._check_rate_limit() {
            return default;
        }
        ACache::get(self, key, default)
    }

    fn put(&mut self, key: Hashable, value: serde_json::Value) {
        if let Err(_) = self._check_rate_limit() {
            return;
        }
        
        let key_json = serde_json::Value::String(key.clone());
        if let Err(_) = validate_cache_key(&key_json, Some(self.max_key_size), Some(false)) {
            return;
        }
        if let Err(_) = validate_cache_value(&value, Some(self.max_value_size_mb), None) {
            return;
        }
        
        let cache = self.cache.lock().unwrap();
        // Note: TTL cache put doesn't take ttl parameter in ACache trait
        // We'd need to use a different method for TTL-specific put
        cache.put(key, value, None);
    }

    fn delete(&mut self, key: Hashable) -> bool {
        let cache = self.cache.lock().unwrap();
        cache.delete(key)
    }

    fn clear(&mut self) {
        let cache = self.cache.lock().unwrap();
        cache.clear();
    }

    fn size(&self) -> i64 {
        self.cache.lock().unwrap().size()
    }

    fn is_full(&self) -> bool {
        self.cache.lock().unwrap().is_full()
    }

    fn evict(&mut self) {
        let cache = self.cache.lock().unwrap();
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
        self.cache.lock().unwrap().get_stats()
    }
}

impl SecureTTLCache {
    /// Initialize secure TTL cache.
    pub fn new(
        capacity: Option<i64>,
        ttl: Option<f64>,
        cleanup_interval: Option<f64>,
        name: Option<String>,
        enable_rate_limit: Option<bool>,
        max_ops_per_second: Option<i64>,
        max_key_size: Option<i64>,
        max_value_size_mb: Option<f64>
    ) -> Self {
        let enable_rate_limit_val = enable_rate_limit.unwrap_or(true);
        let max_ops = max_ops_per_second.unwrap_or(10000);
        
        Self {
            cache: Mutex::new(TTLCache::new(capacity, ttl, cleanup_interval, name)),
            rate_limiter: if enable_rate_limit_val {
                Some(RateLimiter::new(Some(max_ops), None, None))
            } else {
                None
            },
            enable_rate_limit: enable_rate_limit_val,
            max_key_size: max_key_size.unwrap_or(DEFAULT_MAX_KEY_SIZE),
            max_value_size_mb: max_value_size_mb.unwrap_or(DEFAULT_MAX_VALUE_SIZE_MB as f64),
        }
    }

    /// Check rate limit if enabled.
    fn _check_rate_limit(&self) -> Result<(), CacheRateLimitError> {
        if self.enable_rate_limit {
            if let Some(ref limiter) = self.rate_limiter {
                limiter.acquire(None)?;
            }
        }
        Ok(())
    }

    /// Put value with security checks.
    pub fn put(&self, key: String, value: serde_json::Value, _ttl: Option<f64>) -> bool {
        if let Err(_) = self._check_rate_limit() {
            return false;
        }
        
        let key_json = serde_json::Value::String(key.clone());
        if let Err(_) = validate_cache_key(&key_json, Some(self.max_key_size), Some(false)) {
            return false;
        }
        if let Err(_) = validate_cache_value(&value, Some(self.max_value_size_mb), None) {
            return false;
        }
        
        // Note: TTL cache put with custom TTL would need a different method
        // For now, we'll use the standard put
        let cache = self.cache.lock().unwrap();
        cache.put(key, value, None);
        true
    }

    /// Get value with rate limiting.
    pub fn get(&self, key: String, default: Option<serde_json::Value>) -> Option<serde_json::Value> {
        ACache::get(self, key, default)
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
