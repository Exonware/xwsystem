// #exonware/xwsystem/rust/src/caching/warming.rs
//exonware/xwsystem/caching/warming.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Cache warming utilities for preloading data.
//! Performance Priority #4 - Reduce cold start penalties.


use std::time::{SystemTime, UNIX_EPOCH};
use crate::caching::base::{ACache, Hashable};

/// Abstract base class for cache warming strategies.
pub trait AWarmingStrategy {
    /// Warm cache with data.
    /// Args:
    /// cache: Cache instance to warm
    /// keys: Keys to preload
    /// loader: Function to load data for a key
    /// Returns:
    /// Number of keys successfully loaded
    fn warm<F>(&self, cache: &mut dyn ACache, keys: Vec<Hashable>, loader: F) -> i64
    where
        F: Fn(&Hashable) -> Option<serde_json::Value>;
}

/// Preload warming strategy - load all keys upfront.
///
/// Suitable for small datasets that fit entirely in cache.
pub struct PreloadWarmingStrategy;

impl AWarmingStrategy for PreloadWarmingStrategy {
    fn warm<F>(&self, cache: &mut dyn ACache, keys: Vec<Hashable>, loader: F) -> i64
    where
        F: Fn(&Hashable) -> Option<serde_json::Value>,
    {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let mut success_count = 0;
        
        for key in &keys {
            if let Some(value) = loader(key) {
                cache.put(key.clone(), value);
                success_count += 1;
            }
        }
        
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64() - start_time;
        
        if elapsed > 0.0 {
            let rate = keys.len() as f64 / elapsed;
            println!("Cache warming complete: {}/{} loaded in {:.2}s ({:.0} keys/sec)", 
                success_count, keys.len(), elapsed, rate);
        }
        
        success_count
    }
}

impl PreloadWarmingStrategy {
    /// Create a new preload warming strategy.
    pub fn new() -> Self {
        Self
    }
}

/// Lazy warming strategy - load keys on-demand as accessed.
///
/// Suitable for large datasets where only subset will be accessed.
pub struct LazyWarmingStrategy {
    preload_limit: i64,
}

impl AWarmingStrategy for LazyWarmingStrategy {
    fn warm<F>(&self, cache: &mut dyn ACache, keys: Vec<Hashable>, loader: F) -> i64
    where
        F: Fn(&Hashable) -> Option<serde_json::Value>,
    {
        // Preload up to limit
        let preload_keys: Vec<Hashable> = keys.iter()
            .take(self.preload_limit as usize)
            .cloned()
            .collect();
        
        println!("Lazy warming: preloading {}/{} keys", preload_keys.len(), keys.len());
        
        let mut success_count = 0;
        for key in &preload_keys {
            if let Some(value) = loader(key) {
                cache.put(key.clone(), value);
                success_count += 1;
            }
        }
        
        success_count
    }
}

impl LazyWarmingStrategy {
    /// Initialize lazy warming strategy.
    ///
    /// Args:
    /// preload_limit: Maximum keys to preload upfront
    pub fn new(
        preload_limit: Option<i64>
    ) -> Self {
        Self {
            preload_limit: preload_limit.unwrap_or(100),
        }
    }
}

/// Priority-based warming - load high-priority keys first.
///
/// Suitable when some keys are more critical than others.
pub struct PriorityWarmingStrategy<F> {
    priority_func: F,
}

impl<F> AWarmingStrategy for PriorityWarmingStrategy<F>
where
    F: Fn(&Hashable) -> f64,
{
    fn warm<L>(&self, cache: &mut dyn ACache, keys: Vec<Hashable>, loader: L) -> i64
    where
        L: Fn(&Hashable) -> Option<serde_json::Value>,
    {
        // Sort keys by priority (descending)
        let mut sorted_keys = keys;
        sorted_keys.sort_by(|a, b| {
            let priority_a = (self.priority_func)(a);
            let priority_b = (self.priority_func)(b);
            priority_b.partial_cmp(&priority_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        println!("Priority warming: loading {} keys by priority", sorted_keys.len());
        
        let mut success_count = 0;
        for key in &sorted_keys {
            if let Some(value) = loader(key) {
                cache.put(key.clone(), value);
                success_count += 1;
            }
        }
        
        success_count
    }
}

impl<F> PriorityWarmingStrategy<F>
where
    F: Fn(&Hashable) -> f64,
{
    /// Initialize priority warming strategy.
    ///
    /// Args:
    /// priority_func: Function that returns priority score for a key
    /// (higher score = higher priority)
    pub fn new(priority_func: F) -> Self {
        Self { priority_func }
    }
}

/// Warm cache with data using specified strategy.
///
/// Args:
/// cache: Cache instance to warm
/// loader: Function to load data for a key
/// keys: List of keys to preload
/// strategy: Warming strategy (default: PreloadWarmingStrategy)
///
/// Returns:
/// Number of keys successfully loaded
pub fn warm_cache<F>(
    cache: &mut dyn ACache,
    keys: Vec<Hashable>,
    loader: F,
    _strategy: Option<()>  // Changed from Option<Box<dyn AWarmingStrategy>> to avoid trait object issue
) -> i64
where
    F: Fn(&Hashable) -> Option<serde_json::Value>,
{
    // Note: AWarmingStrategy trait is not dyn-compatible due to generic method
    // For now, use PreloadWarmingStrategy directly
    let strategy = PreloadWarmingStrategy::new();
    strategy.warm(cache, keys, loader)
}

/// Warm cache with default preload strategy.
pub fn warm_cache_simple<F>(
    cache: &mut dyn ACache,
    keys: Vec<Hashable>,
    loader: F
) -> i64
where
    F: Fn(&Hashable) -> Option<serde_json::Value>,
{
    let strategy = PreloadWarmingStrategy::new();
    strategy.warm(cache, keys, loader)
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs

