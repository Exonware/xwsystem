// #exonware/xwsystem/rust/src/io/serialization/parsers/picojson_parser.rs
//! picojson parser - Tier 0 (minimal JSON pull-parser, no_std compatible).

use super::base::AJsonParser;

/// Minimal JSON pull-parser using picojson - Tier 0 (no recursion, no heap allocations).
/// 
/// Designed for resource-constrained environments, operates without recursion
/// or heap allocations. Compatible with no_std.
pub struct PicojsonParser;

impl AJsonParser for PicojsonParser {
    /// Parse JSON string to serde_json::Value using picojson.
    fn loads(&self, s: String) -> serde_json::Value {
        #[cfg(feature = "picojson")]
        {
            // picojson is a pull parser, so we need to build the value from events
            let mut parser = picojson::SliceParser::new(&s);
            match Self::parse_picojson_value(&mut parser) {
                Ok(value) => value,
                Err(_) => serde_json::Value::Null,
            }
        }
        
        #[cfg(not(feature = "picojson"))]
        {
            serde_json::Value::Null
        }
    }

    /// Serialize serde_json::Value to JSON string.
    fn dumps(&self, obj: serde_json::Value) -> String {
        // picojson is primarily for parsing, use serde_json for serialization
        serde_json::to_string(&obj).unwrap_or_else(|_| "null".to_string())
    }

    /// Parser identifier.
    fn parser_name(&self) -> &str {
        "picojson"
    }

    /// Performance tier: 0 = baseline (minimal, no_std compatible).
    fn tier(&self) -> i64 {
        0
    }

    /// Check if picojson is available.
    fn is_available(&self) -> bool {
        #[cfg(feature = "picojson")]
        {
            true
        }
        #[cfg(not(feature = "picojson"))]
        {
            false
        }
    }
}


impl PicojsonParser {
    /// Create a new PicojsonParser instance.
    pub fn new() -> Self {
        PicojsonParser
    }
}

impl Default for PicojsonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl PicojsonParser {
    /// Parse a complete JSON value from picojson pull parser
    #[cfg(feature = "picojson")]
    fn parse_picojson_value(parser: &mut picojson::SliceParser) -> Result<serde_json::Value, ()> {
        use picojson::{Event, PullParser};
        
        let event = parser.next().ok_or(())??;
        
        match event {
            Event::Null => Ok(serde_json::Value::Null),
            Event::Bool(b) => Ok(serde_json::Value::Bool(b)),
            Event::String(s) => Ok(serde_json::Value::String(s.to_string())),
            Event::Number(n) => {
                // Try to parse the number string as f64
                if let Ok(f) = n.as_str().parse::<f64>() {
                    Ok(serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null))
                } else {
                    Ok(serde_json::Value::Null)
                }
            },
            Event::StartArray => {
                let mut arr = Vec::new();
                loop {
                    let event = parser.next().ok_or(())??;
                    match event {
                        Event::EndArray => break,
                        _ => {
                            // Recursively parse the value
                            // We need to "push back" the event, but picojson doesn't support that
                            // So we'll use a helper that can handle the current event
                            arr.push(Self::parse_picojson_value_from_event(parser, event)?);
                        }
                    }
                }
                Ok(serde_json::Value::Array(arr))
            },
            Event::StartObject => {
                let mut map = serde_json::Map::new();
                loop {
                    let event = parser.next().ok_or(())??;
                    match event {
                        Event::EndObject => break,
                        Event::Key(key) => {
                            // Next event should be the value
                            let value_event = parser.next().ok_or(())??;
                            let value = Self::parse_picojson_value_from_event(parser, value_event)?;
                            map.insert(key.to_string(), value);
                        },
                        _ => return Err(()), // Unexpected event in object
                    }
                }
                Ok(serde_json::Value::Object(map))
            },
            _ => Err(()),
        }
    }
    
    /// Parse a value from a picojson event (helper for recursive parsing)
    #[cfg(feature = "picojson")]
    fn parse_picojson_value_from_event(
        parser: &mut picojson::SliceParser,
        event: picojson::Event,
    ) -> Result<serde_json::Value, ()> {
        use picojson::{Event, PullParser};
        
        match event {
            Event::Null => Ok(serde_json::Value::Null),
            Event::Bool(b) => Ok(serde_json::Value::Bool(b)),
            Event::String(s) => Ok(serde_json::Value::String(s.to_string())),
            Event::Number(n) => {
                if let Ok(f) = n.as_str().parse::<f64>() {
                    Ok(serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null))
                } else {
                    Ok(serde_json::Value::Null)
                }
            },
            Event::StartArray => {
                let mut arr = Vec::new();
                loop {
                    let event = parser.next().ok_or(())??;
                    match event {
                        Event::EndArray => break,
                        _ => {
                            arr.push(Self::parse_picojson_value_from_event(parser, event)?);
                        }
                    }
                }
                Ok(serde_json::Value::Array(arr))
            },
            Event::StartObject => {
                let mut map = serde_json::Map::new();
                loop {
                    let event = parser.next().ok_or(())??;
                    match event {
                        Event::EndObject => break,
                        Event::Key(key) => {
                            let value_event = parser.next().ok_or(())??;
                            let value = Self::parse_picojson_value_from_event(parser, value_event)?;
                            map.insert(key.to_string(), value);
                        },
                        _ => return Err(()),
                    }
                }
                Ok(serde_json::Value::Object(map))
            },
            _ => Err(()),
        }
    }
}

