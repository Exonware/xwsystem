// #exonware/xwsystem/rust/src/io/serialization/parsers/simd_parser.rs
//! simd-json parser - Tier 1 (high-performance SIMD-accelerated JSON parsing).

use super::base::AJsonParser;

/// High-performance JSON parser using simd-json - Tier 1 (3-4x faster than serde_json).
/// 
/// Uses SIMD instructions for accelerated JSON parsing, similar to orjson in Python.
pub struct SimdJsonParser;

impl AJsonParser for SimdJsonParser {
    /// Parse JSON string to serde_json::Value using simd-json.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "simd-json")]
        {
            // Convert string to owned bytes for simd-json (requires mutable slice)
            let mut bytes = s.into_bytes();
            use simd_json::serde::from_slice;
            from_slice::<serde_json::Value>(&mut bytes).unwrap_or_else(|_| serde_json::Value::Null)
        }
        
        #[cfg(not(feature = "simd-json"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    /// 
    /// Note: simd-json is primarily for parsing. For serialization, we use serde_json
    /// which is already highly optimized.
    fn dumps(&self, obj: serde_json::Value) -> String {
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "simd-json"
    }

    /// Performance tier: 1 = high-performance (3-4x faster than baseline).
    fn tier(&self) -> i64 {
        1
    }

    /// Check if simd-json is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "simd-json")]
        {
            true
        }
        #[cfg(not(feature = "simd-json"))]
        {
            false
        }
    }
}

impl SimdJsonParser {
    /// Create a new SimdJsonParser instance.
    pub fn new() -> Self {
        SimdJsonParser
    }
}

impl Default for SimdJsonParser {
    fn default() -> Self {
        Self::new()
    }
}

