// #exonware/xwsystem/rust/src/io/serialization/parsers/jsonic_parser.rs
//! jsonic parser - Tier 1 (lightweight, zero-copy JSON parsing).

use super::base::AJsonParser;

/// Lightweight JSON parser using jsonic - Tier 1 (zero-copy, minimal allocations).
/// 
/// Fast JSON parser with no dependencies, focusing on high-speed extraction
/// of JSON data with minimal memory footprint.
pub struct JsonicParser;

impl AJsonParser for JsonicParser {
    /// Parse JSON string to serde_json::Value using jsonic.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "jsonic")]
        {
            match jsonic::parse(&s) {
                Ok(item) => Self::jsonic_item_to_value(&item),
                Err(_) => serde_json::Value::Null,
            }
        }
        
        #[cfg(not(feature = "jsonic"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    fn dumps(&self, obj: serde_json::Value) -> String {
        // jsonic is primarily for parsing, use serde_json for serialization
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "jsonic"
    }

    /// Performance tier: 1 = high-performance (zero-copy, minimal allocations).
    fn tier(&self) -> i64 {
        1
    }

    /// Check if jsonic is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "jsonic")]
        {
            true
        }
        #[cfg(not(feature = "jsonic"))]
        {
            false
        }
    }
}

impl JsonicParser {
    /// Create a new JsonicParser instance.
    pub fn new() -> Self {
        JsonicParser
    }
}

impl Default for JsonicParser {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonicParser {
    /// Convert jsonic::JsonItem to serde_json::Value
    #[cfg(feature = "jsonic")]
    fn jsonic_item_to_value(item: &jsonic::json_item::JsonItem) -> serde_json::Value {
        use jsonic::json_type::JsonType;
        
        match item.get_type() {
            JsonType::JsonNull => serde_json::Value::Null,
            JsonType::JsonTrue => serde_json::Value::Bool(true),
            JsonType::JsonFalse => serde_json::Value::Bool(false),
            JsonType::JsonString => {
                serde_json::Value::String(item.as_str().unwrap_or("").to_string())
            },
            JsonType::JsonNumber => {
                // Try integer first, then float
                if let Some(i) = item.as_i128() {
                    serde_json::Value::Number(
                        serde_json::Number::from(i)
                    )
                } else if let Some(f) = item.as_f64() {
                    serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null)
                } else {
                    serde_json::Value::Null
                }
            },
            JsonType::JsonArray => {
                let mut arr = Vec::new();
                if let Some(mut elements) = item.elements() {
                    for element in elements {
                        arr.push(Self::jsonic_item_to_value(&element));
                    }
                }
                serde_json::Value::Array(arr)
            },
            JsonType::JsonMap => {
                let mut map = serde_json::Map::new();
                if let Some(mut entries) = item.entries() {
                    for (key, value) in entries {
                        let key_str = key.as_str().to_string();
                        map.insert(key_str, Self::jsonic_item_to_value(&value));
                    }
                }
                serde_json::Value::Object(map)
            },
            _ => serde_json::Value::Null,
        }
    }
}

