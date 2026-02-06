// #exonware/xwsystem/rust/src/io/serialization/parsers/json_fast_parser.rs
//! json_fast parser - Tier 1 (smart caching and zero-copy techniques).

use super::base::AJsonParser;

/// Fast JSON parser using json_fast - Tier 1 (up to 35% faster than serde_json).
/// 
/// Utilizes smart caching and zero-copy techniques to enhance parsing speed.
pub struct JsonFastParser;

impl AJsonParser for JsonFastParser {
    /// Parse JSON string to serde_json::Value using json_fast.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "json_fast")]
        {
            let parser = json_fast::JsonFast::new();
            match parser.parse(&s) {
                Ok(value) => Self::json_fast_value_to_serde(&value),
                Err(_) => serde_json::Value::Null,
            }
        }
        
        #[cfg(not(feature = "json_fast"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    fn dumps(&self, obj: serde_json::Value) -> String {
        // json_fast is primarily for parsing, use serde_json for serialization
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "json_fast"
    }

    /// Performance tier: 1 = high-performance (35% faster than serde_json).
    fn tier(&self) -> i64 {
        1
    }

    /// Check if json_fast is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "json_fast")]
        {
            true
        }
        #[cfg(not(feature = "json_fast"))]
        {
            false
        }
    }
}

impl JsonFastParser {
    /// Create a new JsonFastParser instance.
    pub fn new() -> Self {
        JsonFastParser
    }
}

impl Default for JsonFastParser {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonFastParser {
    /// Convert json_fast::JsonValue to serde_json::Value
    #[cfg(feature = "json_fast")]
    fn json_fast_value_to_serde(value: &json_fast::JsonValue) -> serde_json::Value {
        use json_fast::JsonValue;
        
        match value {
            JsonValue::Null => serde_json::Value::Null,
            JsonValue::Bool(b) => serde_json::Value::Bool(*b),
            JsonValue::String(s) => serde_json::Value::String(s.clone()),
            JsonValue::Number(n) => {
                serde_json::Number::from_f64(*n)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            },
            JsonValue::Object(map) => {
                let mut serde_map = serde_json::Map::new();
                for (k, v) in map {
                    serde_map.insert(k.clone(), Self::json_fast_value_to_serde(v));
                }
                serde_json::Value::Object(serde_map)
            },
        }
    }
}

