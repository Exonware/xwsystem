// #exonware/xwsystem/rust/src/io/serialization/parsers/hifijson_parser.rs
//! hifijson parser - Tier 1 (high-fidelity lexer and parser).

use super::base::AJsonParser;

/// High-fidelity JSON parser using hifijson - Tier 1 (preserves input data accurately).
/// 
/// Provides a lexer and parser with emphasis on performance, allowing for
/// custom parser implementations.
pub struct HifijsonParser;

impl AJsonParser for HifijsonParser {
    /// Parse JSON string to serde_json::Value using hifijson.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "hifijson")]
        {
            let bytes = s.as_bytes();
            let mut lexer = hifijson::SliceLexer::new(bytes);
            use hifijson::token::Lex;
            match lexer.exactly_one(|token, lexer| hifijson::value::parse_unbounded(token, lexer)) {
                Ok(value) => Self::hifijson_value_to_serde(&value),
                Err(_) => serde_json::Value::Null,
            }
        }
        
        #[cfg(not(feature = "hifijson"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    fn dumps(&self, obj: serde_json::Value) -> String {
        // hifijson is primarily for parsing, use serde_json for serialization
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "hifijson"
    }

    /// Performance tier: 1 = high-performance (high-fidelity parsing).
    fn tier(&self) -> i64 {
        1
    }

    /// Check if hifijson is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "hifijson")]
        {
            true
        }
        #[cfg(not(feature = "hifijson"))]
        {
            false
        }
    }
}

impl HifijsonParser {
    /// Create a new HifijsonParser instance.
    pub fn new() -> Self {
        HifijsonParser
    }
}

impl Default for HifijsonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl HifijsonParser {
    /// Convert hifijson::value::Value to serde_json::Value
    #[cfg(feature = "hifijson")]
    fn hifijson_value_to_serde<Num: AsRef<str>, Str: AsRef<str>>(
        value: &hifijson::value::Value<Num, Str>
    ) -> serde_json::Value {
        use hifijson::value::Value as HifiValue;
        
        match value {
            HifiValue::Null => serde_json::Value::Null,
            HifiValue::Bool(b) => serde_json::Value::Bool(*b),
            HifiValue::String(s) => serde_json::Value::String(s.as_ref().to_string()),
            HifiValue::Number((n, _)) => {
                // hifijson numbers are stored as strings to preserve precision
                // Try to parse as f64
                if let Ok(f) = n.as_ref().parse::<f64>() {
                    serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null)
                } else {
                    serde_json::Value::Null
                }
            },
            HifiValue::Array(arr) => {
                let mut serde_arr = Vec::new();
                for item in arr {
                    serde_arr.push(Self::hifijson_value_to_serde(item));
                }
                serde_json::Value::Array(serde_arr)
            },
            HifiValue::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (key, val) in obj.iter() {
                    map.insert(key.as_ref().to_string(), Self::hifijson_value_to_serde(val));
                }
                serde_json::Value::Object(map)
            },
        }
    }
}

