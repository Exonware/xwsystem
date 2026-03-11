// #exonware/xwsystem/rust/src/caching/utils.rs
//exonware/xwsystem/caching/utils.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Common utility functions for caching module.

// For objects that don't support getsizeof
// Try pickling as fallback
// Last resort: return a conservative estimate
/// Estimate memory size of object in bytes.
///
///
/// Args:
/// obj: Object to estimate size of
///
///
/// Returns:
/// Estimated size in bytes
///
///
/// Note:
/// This is a rough estimate using sys.getsizeof.
/// For more accurate memory profiling, use memory_profiler.
use std::collections::HashMap;

pub fn estimate_object_size(obj: &serde_json::Value) -> i64 {
    // Estimate size by serializing to JSON and measuring bytes
    // This is a rough estimate similar to Python's sys.getsizeof + pickle fallback
    match serde_json::to_string(obj) {
        Ok(json_str) => json_str.len() as i64,
        Err(_) => {
            // Fallback: estimate based on type
            match obj {
                serde_json::Value::Null => 4,
                serde_json::Value::Bool(_) => 4,
                serde_json::Value::Number(_) => 8,
                serde_json::Value::String(s) => s.len() as i64 + 8, // String overhead
                serde_json::Value::Array(arr) => {
                    arr.iter().map(estimate_object_size).sum::<i64>() + 8
                }
                serde_json::Value::Object(map) => {
                    map.iter()
                        .map(|(k, v)| k.len() as i64 + estimate_object_size(v) + 8)
                        .sum::<i64>() + 8
                }
            }
        }
    }
}

// Serialize value to bytes
/// Compute cryptographic checksum of value.
///
///
/// Args:
/// value: Value to compute checksum for
/// algorithm: Hash algorithm ('sha256', 'md5', 'sha1')
///
///
/// Returns:
/// Hexadecimal checksum string
///
///
/// Raises:
/// ValueError: If algorithm is not supported
pub fn compute_checksum(value: &serde_json::Value, algorithm: Option<&str>) -> String {
    use sha2::{Sha256, Sha512, Digest};
    use sha1::Sha1;
    
    let algo = algorithm.unwrap_or("sha256");
    
    // Serialize value to JSON bytes
    let value_bytes = match serde_json::to_vec(value) {
        Ok(bytes) => bytes,
        Err(_) => {
            // Fallback: use string representation
            serde_json::to_string(value).unwrap_or_default().into_bytes()
        }
    };
    
    // Compute hash based on algorithm
    let hash_hex = match algo {
        "md5" => {
            format!("{:x}", md5::compute(&value_bytes))
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(&value_bytes);
            format!("{:x}", hasher.finalize())
        }
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(&value_bytes);
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(&value_bytes);
            format!("{:x}", hasher.finalize())
        }
        _ => {
            // Default to sha256 for unsupported algorithms
            let mut hasher = Sha256::new();
            hasher.update(&value_bytes);
            format!("{:x}", hasher.finalize())
        }
    };
    
    hash_hex
}

/// Format bytes as human-readable string.
///
///
/// Args:
/// size: Size in bytes
///
///
/// Returns:
/// Formatted string (e.g., "1.23 MB")
///
/// Examples:
/// >>> format_bytes(1024)
/// '1.00 KB'
/// >>> format_bytes(1536)
/// '1.50 KB'
/// >>> format_bytes(1048576)
/// '1.00 MB'
pub fn format_bytes(size: i64) -> String {
    if size < 0 {
        return "Invalid size".to_string();
    }
    
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut unit_index = 0;
    let mut size_float = size as f64;
    
    while size_float >= 1024.0 && unit_index < units.len() - 1 {
        size_float /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size_float, units[unit_index])
}

// Build key from function name and arguments
// Convert args and kwargs to hashable representation
/// Build cache key from function and arguments.
///
///
/// Args:
/// func: Function being cached
/// args: Positional arguments
/// kwargs: Keyword arguments
///
///
/// Returns:
/// Cache key string
///
///
/// Note:
/// This is a basic implementation. For production use,
/// consider using functools.lru_cache or cachetools.
pub fn default_key_builder(func: &str, args: &[serde_json::Value], kwargs: &HashMap<String, serde_json::Value>) -> String {
    // Build key from function name and arguments
    let args_str = serde_json::to_string(args).unwrap_or_else(|_| format!("{:?}", args));
    
    // Sort kwargs for consistent key generation
    let mut sorted_kwargs: Vec<_> = kwargs.iter().collect();
    sorted_kwargs.sort_by_key(|(k, _)| k.clone());
    let kwargs_str = serde_json::to_string(&sorted_kwargs).unwrap_or_else(|_| format!("{:?}", sorted_kwargs));
    
    format!("{}:{}:{}", func, args_str, kwargs_str)
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
/// ValueError: If capacity is invalid
pub fn validate_capacity(capacity: i64, min_capacity: Option<i64>, max_capacity: Option<i64>) -> Result<(), String> {
    let min = min_capacity.unwrap_or(1);
    let max = max_capacity.unwrap_or(10_000_000);
    
    if capacity < min {
        return Err(format!(
            "Cache capacity must be at least {}, got {}. Example: capacity={}",
            min, capacity, min
        ));
    }
    
    if capacity > max {
        return Err(format!(
            "Cache capacity too large: {} (max: {}). Consider using a distributed cache for very large capacities.",
            capacity, max
        ));
    }
    
    Ok(())
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
/// ValueError: If TTL is invalid
pub fn validate_ttl(ttl: f64, min_ttl: Option<f64>, max_ttl: Option<f64>) -> Result<(), String> {
    let min = min_ttl.unwrap_or(0.0);
    let max = max_ttl.unwrap_or(86400.0 * 365.0); // 1 year default
    
    if ttl < min {
        return Err(format!(
            "TTL must be at least {} seconds, got {}. Example: ttl=60.0 (1 minute)",
            min, ttl
        ));
    }
    
    if ttl > max {
        return Err(format!(
            "TTL too large: {:.0} seconds ({:.1} days). Maximum: {:.0} seconds ({:.1} days)",
            ttl, ttl / 86400.0, max, max / 86400.0
        ));
    }
    
    Ok(())
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Functions exported via mod.rs
