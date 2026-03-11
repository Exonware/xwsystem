// #exonware/xwsystem/rust/src/caching/validation.rs
//exonware/xwsystem/caching/validation.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Input validation for caching module - Security Priority #1.
//! Implements comprehensive validation to prevent security vulnerabilities.


use crate::caching::errors::{CacheKeySizeError, CacheValidationError, CacheValueSizeError};
use crate::caching::utils::estimate_object_size;

/// Constant: DEFAULT_MAX_KEY_SIZE
pub const DEFAULT_MAX_KEY_SIZE: i64 = 1024;

/// Constant: DEFAULT_MAX_VALUE_SIZE_MB
pub const DEFAULT_MAX_VALUE_SIZE_MB: i64 = 10;

/// Constant: DEFAULT_MAX_VALUE_SIZE
pub const DEFAULT_MAX_VALUE_SIZE: i64 = DEFAULT_MAX_VALUE_SIZE_MB * 1024 * 1024; // 10 MB in bytes

// Check if key is hashable
// Check key size (prevent memory exhaustion)
/// Validate cache key for security and correctness.
///
///
/// Args:
/// key: Cache key to validate
/// max_size: Maximum key size in bytes
/// allow_none: Whether to allow None as a key
///
///
/// Raises:
/// CacheValidationError: If key is invalid
/// CacheKeySizeError: If key exceeds maximum size
///
/// Security considerations:
/// - Prevents memory exhaustion attacks via large keys
/// - Ensures keys are hashable
/// - Validates key types
pub fn validate_cache_key(key: &serde_json::Value, max_size: Option<i64>, allow_none: Option<bool>) -> Result<(), CacheValidationError> {
    let max = max_size.unwrap_or(DEFAULT_MAX_KEY_SIZE);
    let allow_none = allow_none.unwrap_or(false);
    
    // Check for None/null
    if key.is_null() {
        if !allow_none {
            return Err(CacheValidationError::new(
                "Cache key cannot be None/null. Use a valid hashable object (str, int, etc.)"
            ));
        }
        return Ok(());
    }
    
    // Check key size (prevent memory exhaustion)
    let key_size = match key {
        serde_json::Value::String(s) => s.len() as i64,
        serde_json::Value::Number(n) => {
            // Estimate number size
            if let Some(_) = n.as_i64() {
                8
            } else if let Some(_) = n.as_u64() {
                8
            } else if let Some(_) = n.as_f64() {
                8
            } else {
                16
            }
        }
        serde_json::Value::Array(_arr) => {
            // Estimate array size
            serde_json::to_string(key).map(|s| s.len() as i64).unwrap_or(1024)
        }
        serde_json::Value::Object(_map) => {
            // Estimate object size
            serde_json::to_string(key).map(|s| s.len() as i64).unwrap_or(1024)
        }
        _ => serde_json::to_string(key).map(|s| s.len() as i64).unwrap_or(1024),
    };
    
    if key_size > max {
        return Err(CacheValidationError::from_source(
            format!(
                "Cache key too large: {} bytes (max: {}). This prevents memory exhaustion attacks. Consider using a hash of the key instead.",
                key_size, max
            ),
            Box::new(CacheKeySizeError::new("Key size exceeded"))
        ));
    }
    
    Ok(())
}

/// Validate cache value for security and correctness.
///
///
/// Args:
/// value: Cache value to validate
/// max_size_mb: Maximum value size in megabytes
/// max_size: Maximum value size in bytes (overrides max_size_mb)
///
///
/// Raises:
/// CacheValueSizeError: If value exceeds maximum size
///
/// Security considerations:
/// - Prevents memory exhaustion attacks via large values
/// - Protects against DoS via excessive memory usage
pub fn validate_cache_value(value: &serde_json::Value, max_size_mb: Option<f64>, max_size: Option<i64>) -> Result<(), CacheValueSizeError> {
    // Determine max size
    let max = max_size.unwrap_or_else(|| {
        (max_size_mb.unwrap_or(DEFAULT_MAX_VALUE_SIZE_MB as f64) * 1024.0 * 1024.0) as i64
    });
    
    // Estimate value size
    let value_size = estimate_object_size(value);
    
    if value_size > max {
        return Err(CacheValueSizeError::new(format!(
            "Cache value too large: {} bytes ({:.2} MB) (max: {} bytes / {:.1} MB). This prevents memory exhaustion. Consider storing a reference instead of the full value.",
            value_size,
            value_size as f64 / (1024.0 * 1024.0),
            max,
            max as f64 / (1024.0 * 1024.0)
        )));
    }
    
    Ok(())
}

// Already a string, return as-is
// Simple types: convert to string
// Bytes: decode if possible, otherwise use hex
// Collections: convert to string representation
// Fallback: use string representation
/// Sanitize cache key to ensure it's safe for use.
///
///
/// Args:
/// key: Cache key to sanitize
///
///
/// Returns:
/// Sanitized key as string
///
///
/// Note:
/// This function converts any hashable object to a safe string representation.
/// For complex objects, use a custom key builder function.
pub fn sanitize_key(key: &serde_json::Value) -> String {
    match key {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
            // Collections: convert to string representation
            serde_json::to_string(key).unwrap_or_else(|_| format!("{:?}", key))
        }
    }
}

/// Validate TTL (time-to-live) parameter.
///
///
/// Args:
/// ttl: TTL in seconds
/// min_ttl: Minimum allowed TTL
/// max_ttl: Maximum allowed TTL (default: 1 year)
///
///
/// Raises:
/// CacheValidationError: If TTL is invalid
pub fn validate_ttl(ttl: f64, min_ttl: Option<f64>, max_ttl: Option<f64>) -> Result<(), CacheValidationError> {
    let min = min_ttl.unwrap_or(0.0);
    let max = max_ttl.unwrap_or(86400.0 * 365.0); // 1 year default
    
    if ttl < min {
        return Err(CacheValidationError::new(format!(
            "TTL must be at least {} seconds, got {}. Example: ttl=60.0 (1 minute)",
            min, ttl
        )));
    }
    
    if ttl > max {
        return Err(CacheValidationError::new(format!(
            "TTL too large: {:.0} seconds ({:.1} days). Maximum: {:.0} seconds ({:.1} days)",
            ttl, ttl / 86400.0, max, max / 86400.0
        )));
    }
    
    Ok(())
}

/// Validate cache capacity parameter.
///
///
/// Args:
/// capacity: Capacity to validate
/// min_capacity: Minimum allowed capacity
/// max_capacity: Maximum allowed capacity
///
///
/// Raises:
/// CacheValidationError: If capacity is invalid
pub fn validate_capacity(capacity: i64, min_capacity: Option<i64>, max_capacity: Option<i64>) -> Result<(), CacheValidationError> {
    let min = min_capacity.unwrap_or(1);
    let max = max_capacity.unwrap_or(10_000_000);
    
    if capacity < min {
        return Err(CacheValidationError::new(format!(
            "Cache capacity must be at least {}, got {}. Example: capacity={}",
            min, capacity, min
        )));
    }
    
    if capacity > max {
        return Err(CacheValidationError::new(format!(
            "Cache capacity too large: {} (max: {}). For very large caches, consider using a distributed cache system.",
            capacity, max
        )));
    }
    
    Ok(())
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Functions exported via mod.rs
