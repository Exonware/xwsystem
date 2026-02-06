// #exonware/xwsystem/rust/src/io/serialization/parsers/justjson_parser.rs
//! justjson parser - Tier 1 (efficient parsing with minimal allocations).

use super::base::AJsonParser;

/// Efficient JSON parser using justjson - Tier 1 (optimized for &str and &[u8]).
/// 
/// Returns a Value type that keeps strings and numbers in their original form,
/// fully validated during parsing, reducing allocations.
pub struct JustjsonParser;

impl AJsonParser for JustjsonParser {
    /// Parse JSON string to serde_json::Value using justjson.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "justjson")]
        {
            match justjson::Value::from_json(&s) {
                Ok(value) => Self::justjson_value_to_serde(&value),
                Err(_) => serde_json::Value::Null,
            }
        }
        
        #[cfg(not(feature = "justjson"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    fn dumps(&self, obj: serde_json::Value) -> String {
        // justjson is primarily for parsing, use serde_json for serialization
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "justjson"
    }

    /// Performance tier: 1 = high-performance (minimal allocations).
    fn tier(&self) -> i64 {
        1
    }

    /// Check if justjson is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "justjson")]
        {
            true
        }
        #[cfg(not(feature = "justjson"))]
        {
            false
        }
    }
}


impl JustjsonParser {
    /// Create a new JustjsonParser instance.
    pub fn new() -> Self {
        JustjsonParser
    }
}

impl Default for JustjsonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl JustjsonParser {
    /// Convert justjson::Value to serde_json::Value
    #[cfg(feature = "justjson")]
    fn justjson_value_to_serde(value: &justjson::Value<&str>) -> serde_json::Value {
        use justjson::Value as JustjsonValue;
        
        match value {
            JustjsonValue::Null => serde_json::Value::Null,
            JustjsonValue::Boolean(b) => serde_json::Value::Bool(*b),
            JustjsonValue::String(s) => serde_json::Value::String(s.as_ref().to_string()),
            JustjsonValue::Number(n) => {
                // Try to get as f64 first, then try integer
                if let Some(f) = n.as_f64() {
                    serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null)
                } else if let Some(i) = n.as_i64() {
                    serde_json::Value::Number(serde_json::Number::from(i))
                } else {
                    serde_json::Value::Null
                }
            },
            JustjsonValue::Array(arr) => {
                let mut serde_arr = Vec::new();
                for item in arr {
                    serde_arr.push(Self::justjson_value_to_serde(item));
                }
                serde_json::Value::Array(serde_arr)
            },
            JustjsonValue::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, val) in obj.iter() {
                    map.insert(key.as_ref().to_string(), Self::justjson_value_to_serde(val));
                }
                serde_json::Value::Object(map)
            },
        }
    }
}

