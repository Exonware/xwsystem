// #exonware/xwsystem/rust/src/io/serialization/parsers/registry.rs
//! Parser registry for JSON parser selection and auto-detection.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use super::base::AJsonParser;
use super::hifijson_parser::HifijsonParser;
use super::json_fast_parser::JsonFastParser;
use super::jsonic_parser::JsonicParser;
use super::justjson_parser::JustjsonParser;
use super::picojson_parser::PicojsonParser;
use super::simd_parser::SimdJsonParser;
use super::sonic_parser::SonicParser;
use super::standard::StandardJsonParser;

// Type alias for parser factory functions
type ParserFactory = fn() -> Box<dyn AJsonParser>;

// Registry of available parsers (factory functions)
fn get_parsers() -> &'static Mutex<HashMap<String, ParserFactory>> {
    static PARSERS: OnceLock<Mutex<HashMap<String, ParserFactory>>> = OnceLock::new();
    PARSERS.get_or_init(|| {
        let mut map = HashMap::new();
        // Register parsers in order of performance (highest first)
        // Tier 1: Ultra-high performance
        map.insert("sonic-rs".to_string(), || {
            Box::new(SonicParser::new()) as Box<dyn AJsonParser>
        });
        map.insert("simd-json".to_string(), || {
            Box::new(SimdJsonParser::new()) as Box<dyn AJsonParser>
        });
        map.insert("jsonic".to_string(), || {
            Box::new(JsonicParser::new()) as Box<dyn AJsonParser>
        });
        map.insert("justjson".to_string(), || {
            Box::new(JustjsonParser::new()) as Box<dyn AJsonParser>
        });
        map.insert("json_fast".to_string(), || {
            Box::new(JsonFastParser::new()) as Box<dyn AJsonParser>
        });
        map.insert("hifijson".to_string(), || {
            Box::new(HifijsonParser::new()) as Box<dyn AJsonParser>
        });
        // Tier 0: Baseline
        map.insert("standard".to_string(), || {
            Box::new(StandardJsonParser::new()) as Box<dyn AJsonParser>
        });
        map.insert("picojson".to_string(), || {
            Box::new(PicojsonParser::new()) as Box<dyn AJsonParser>
        });
        Mutex::new(map)
    })
}

/// Auto-detect and return the best available parser.
///
/// Priority (tries in order):
/// 1. sonic-rs (ultra-high-performance, often fastest)
/// 2. simd-json (high-performance, 3-4x faster than serde_json)
/// 3. jsonic (zero-copy, minimal allocations)
/// 4. justjson (efficient parsing)
/// 5. json_fast (smart caching, 35% faster than serde_json)
/// 6. hifijson (high-fidelity parsing)
/// 7. standard (serde_json, always available)
/// 8. picojson (minimal, no_std compatible)
///
/// Returns:
/// Best available parser instance
pub fn get_best_available_parser() -> Box<dyn AJsonParser> {
    // Try parsers in order of performance (highest first)
    let parsers_to_try = vec![
        "sonic-rs",
        "simd-json",
        "jsonic",
        "justjson",
        "json_fast",
        "hifijson",
        "picojson",
    ];
    
    for parser_name in parsers_to_try {
        let parser = get_parser(Some(parser_name.to_string()));
        if parser.is_available() {
            return parser;
        }
    }
    
    // Fallback to standard (always available)
    get_parser(Some("standard".to_string()))
}

/// Get parser by name or auto-detect best available.
///
/// Args:
/// name: Parser name ("sonic-rs", "simd-json", "jsonic", "justjson", "json_fast", 
///       "hifijson", "standard", "picojson", or None for auto-detect)
///
/// Returns:
/// Parser instance (falls back to best available if requested parser unavailable)
pub fn get_parser(name: Option<String>) -> Box<dyn AJsonParser> {
    let parser_name = name.as_deref().unwrap_or("standard");
    
    // Try to get parser from registry
    let parsers = get_parsers().lock().unwrap();
    if let Some(factory) = parsers.get(parser_name) {
        let factory_fn = *factory; // Copy the function pointer
        drop(parsers); // Release lock before calling factory
        let parser = factory_fn();
        if parser.is_available() {
            return parser;
        }
    } else {
        drop(parsers);
    }
    
    // Fallback to standard if requested parser is not available
    if parser_name != "standard" {
        return get_parser(Some("standard".to_string()));
    }
    
    // Last resort: create standard parser directly
    Box::new(StandardJsonParser::new())
}

/// Register a new parser implementation.
///
/// Args:
/// name: Parser identifier
/// factory: Factory function that creates a parser instance
pub fn register_parser(name: &str, factory: ParserFactory) -> () {
    let mut parsers = get_parsers().lock().unwrap();
    parsers.insert(name.to_string(), factory);
}
