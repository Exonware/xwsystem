// #exonware/xwsystem/rust/src/io/serialization/parsers/standard.rs
//! Standard library JSON parser (baseline implementation).


use std::collections::HashMap;

use crate::base::{AJsonParser};

// Always available (stdlib)
/// Standard library JSON parser - Tier 0 (baseline).
pub struct StandardJsonParser;

impl AJsonParser for StandardJsonParser {
    /// Parse JSON string/bytes to Python object.
    fn loads(&self, s: String) -> serde_json::Value {
        // Handle both string and bytes (bytes will be UTF-8 encoded string)
        serde_json::from_str(&s).unwrap_or_else(|_| serde_json::Value::Null)
    }

    /// Serialize Python object to JSON.
    fn dumps(&self, obj: serde_json::Value) -> String {
        // Basic serialization - kwargs support can be added later if needed
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "standard"
    }

    /// Performance tier: 0 = stdlib (baseline).
    fn tier(&self) -> i64 {
        0
    }

    /// Always available (stdlib).
    fn is_available(&self) -> bool {
        true
    }
}

impl StandardJsonParser {
    /// Constructor.
    pub fn new() -> Self {
        StandardJsonParser
    }
}

impl Default for StandardJsonParser {
    fn default() -> Self {
        Self::new()
    }
}
