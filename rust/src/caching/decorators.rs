// #exonware/xwsystem/rust/src/caching/decorators.rs
//exonware/xwsystem/caching/decorators.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Advanced cache decorators with hooks and customization.
//! Extensibility Priority #5 - Flexible caching decorators for functions.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::caching::base::{ACache, Hashable};
use crate::caching::lru_cache::LRUCache;
use crate::caching::utils::default_key_builder;

// Simplified aliases with XW prefix

/// Configuration for cached function decorator.
pub struct CachedConfig {
    pub cache: Option<Arc<Mutex<dyn ACache + Send + Sync>>>,
    pub ttl: Option<i64>,
    pub namespace: Option<String>,
}

impl Default for CachedConfig {
    fn default() -> Self {
        Self {
            cache: None,
            ttl: None,
            namespace: None,
        }
    }
}

/// Helper function to create a cached wrapper for a function.
/// 
/// Note: In Rust, we use a different approach than Python decorators.
/// This provides a builder pattern for creating cached functions.
pub fn xwcached() -> CachedConfig {
    CachedConfig::default()
}

impl CachedConfig {
    /// Set TTL for cached results.
    pub fn ttl(mut self, ttl: i64) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Set namespace for key prefixing.
    pub fn namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }

    /// Set custom cache instance.
    pub fn cache<C>(mut self, cache: C) -> Self
    where
        C: ACache + Send + Sync + 'static,
    {
        self.cache = Some(Arc::new(Mutex::new(cache)));
        self
    }
}

/// Configuration for async cached function decorator.
pub struct AsyncCachedConfig {
    pub ttl: Option<i64>,
    pub namespace: Option<String>,
}

impl Default for AsyncCachedConfig {
    fn default() -> Self {
        Self {
            ttl: None,
            namespace: None,
        }
    }
}

/// Helper function to create an async cached wrapper.
pub fn xw_async_cached() -> AsyncCachedConfig {
    AsyncCachedConfig::default()
}

impl AsyncCachedConfig {
    /// Set TTL for cached results.
    pub fn ttl(mut self, ttl: i64) -> Self {
        self.ttl = Some(ttl);
        self
    }

    /// Set namespace for key prefixing.
    pub fn namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }
}

/// Simple cache decorator (eXonware naming convention).
/// 
/// This is a convenience function that creates a cached function wrapper.
pub fn xwcache(ttl: Option<i64>) -> CachedConfig {
    let mut config = CachedConfig::default();
    if let Some(t) = ttl {
        config.ttl = Some(t);
    }
    config
}

/// Simple async cache decorator (eXonware naming convention).
pub fn xw_async_cache(ttl: Option<i64>) -> AsyncCachedConfig {
    let mut config = AsyncCachedConfig::default();
    if let Some(t) = ttl {
        config.ttl = Some(t);
    }
    config
}


