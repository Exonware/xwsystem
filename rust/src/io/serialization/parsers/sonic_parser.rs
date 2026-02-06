// #exonware/xwsystem/rust/src/io/serialization/parsers/sonic_parser.rs
//! sonic-rs parser - Tier 1 (ultra-fast SIMD-optimized JSON parsing).

use super::base::AJsonParser;

/// Ultra-fast JSON parser using sonic-rs - Tier 1 (often faster than simd-json).
/// 
/// Uses SIMD optimizations for accelerated JSON parsing, particularly fast for
/// deserializing into Rust structs.
pub struct SonicParser;

impl AJsonParser for SonicParser {
    /// Parse JSON string to serde_json::Value using sonic-rs.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "sonic-rs")]
        {
            use sonic_rs::from_str;
            from_str::<serde_json::Value>(&s).unwrap_or_else(|_| serde_json::Value::Null)
        }
        
        #[cfg(not(feature = "sonic-rs"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    fn dumps(&self, obj: serde_json::Value) -> String {
        #[cfg(feature = "sonic-rs")]
        {
            use sonic_rs::to_string;
            to_string(&obj).unwrap_or_else(|_| "null".to_string())
        }
        
        #[cfg(not(feature = "sonic-rs"))]
        {
            String::new()
        }
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "sonic-rs"
    }

    /// Performance tier: 1 = ultra-high-performance (often faster than simd-json).
    fn tier(&self) -> i64 {
        1
    }

    /// Check if sonic-rs is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "sonic-rs")]
        {
            true
        }
        #[cfg(not(feature = "sonic-rs"))]
        {
            false
        }
    }
}

impl SonicParser {
    /// Create a new SonicParser instance.
    pub fn new() -> Self {
        SonicParser
    }
}

impl Default for SonicParser {
    fn default() -> Self {
        Self::new()
    }
}

