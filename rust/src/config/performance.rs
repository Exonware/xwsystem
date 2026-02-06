// #exonware/xwsystem/rust/src/config/performance.rs
//! Performance optimization configuration for xwsystem I/O operations.
//! 
//! This module provides configuration options to enable/disable performance
//! optimizations (parallel index building, append-only logs) with automatic
//! fallback to original implementations.


use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Configuration for performance optimizations.
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub enable_parallel_index: bool,
    pub parallel_index_workers: Option<i64>,
    pub parallel_index_chunk_size_mb: i64,
    pub parallel_index_threshold_mb: i64,
    pub enable_append_log: bool,
    pub append_log_threshold_mb: i64,
    pub append_log_compaction_threshold_mb: i64,
    pub fallback_on_error: bool,
}

impl PerformanceConfig {
    /// Create config from environment variables.
    pub fn from_env() -> Self {
        Self {
            enable_parallel_index: std::env::var("XWSYSTEM_PARALLEL_INDEX")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
            parallel_index_workers: std::env::var("XWSYSTEM_PARALLEL_WORKERS")
                .ok()
                .and_then(|s| s.parse().ok())
                .filter(|&v: &i64| v > 0),
            parallel_index_chunk_size_mb: std::env::var("XWSYSTEM_CHUNK_SIZE_MB")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            parallel_index_threshold_mb: std::env::var("XWSYSTEM_PARALLEL_THRESHOLD_MB")
                .unwrap_or_else(|_| "200".to_string())
                .parse()
                .unwrap_or(200),
            enable_append_log: std::env::var("XWSYSTEM_APPEND_LOG")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
            append_log_threshold_mb: std::env::var("XWSYSTEM_APPEND_LOG_THRESHOLD_MB")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            append_log_compaction_threshold_mb: std::env::var("XWSYSTEM_LOG_THRESHOLD_MB")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            fallback_on_error: std::env::var("XWSYSTEM_FALLBACK")
                .unwrap_or_else(|_| "true".to_string())
                .to_lowercase() == "true",
        }
    }

    /// Conservative config (disable optimizations, use originals).
    pub fn conservative() -> Self {
        Self {
            enable_parallel_index: false,
            parallel_index_workers: None,
            parallel_index_chunk_size_mb: 100,
            parallel_index_threshold_mb: 200,
            enable_append_log: false,
            append_log_threshold_mb: 100,
            append_log_compaction_threshold_mb: 100,
            fallback_on_error: true,
        }
    }

    /// Aggressive config (enable all optimizations, no fallback).
    pub fn aggressive() -> Self {
        Self {
            enable_parallel_index: true,
            parallel_index_workers: None,
            parallel_index_chunk_size_mb: 100,
            parallel_index_threshold_mb: 200,
            enable_append_log: true,
            append_log_threshold_mb: 100,
            append_log_compaction_threshold_mb: 100,
            fallback_on_error: false,
        }
    }
}

/// Placeholder for backward compatibility.
#[derive(Debug, Clone, Default)]
pub struct PerformanceLimits {}

/// Placeholder for backward compatibility.
#[derive(Debug, Clone, Default)]
pub struct SerializationLimits {}

/// Placeholder for backward compatibility.
#[derive(Debug, Clone, Default)]
pub struct NetworkLimits {}

/// Placeholder for backward compatibility.
#[derive(Debug, Clone, Default)]
pub struct SecurityLimits {}

// Global config instance
use std::sync::OnceLock;

static GLOBAL_CONFIG: OnceLock<Arc<RwLock<Option<PerformanceConfig>>>> = OnceLock::new();

fn get_global_config() -> Arc<RwLock<Option<PerformanceConfig>>> {
    GLOBAL_CONFIG.get_or_init(|| Arc::new(RwLock::new(None))).clone()
}

/// Get global performance config.
pub fn get_performance_config() -> PerformanceConfig {
    let config = get_global_config().read().unwrap();
    config.clone().unwrap_or_else(PerformanceConfig::from_env)
}

/// Set global performance config.
pub fn set_performance_config(config: PerformanceConfig) {
    let mut global = get_global_config().write().unwrap();
    *global = Some(config);
}

/// Placeholder for backward compatibility.
pub fn configure_performance(_kwargs: HashMap<String, String>) {
    // Configuration would be applied here
}

/// Placeholder for backward compatibility.
pub fn get_serialization_limits() -> SerializationLimits {
    SerializationLimits::default()
}

/// Placeholder for backward compatibility.
pub fn get_network_limits() -> NetworkLimits {
    NetworkLimits::default()
}

/// Placeholder for backward compatibility.
pub fn get_security_limits() -> SecurityLimits {
    SecurityLimits::default()
}


