// #exonware/xwsystem/rust/src/io/serialization/parsers/mod.rs
//! JSON Parser abstraction layer for pluggable performance optimizations.
//! 
//! This module provides a pluggable parser system that allows switching between
//! different JSON parsing implementations (serde_json, simd-json, sonic-rs, etc.) for performance.

pub mod base;
pub mod hifijson_parser;
pub mod json_fast_parser;
pub mod jsonic_parser;
pub mod justjson_parser;
pub mod picojson_parser;
pub mod registry;
pub mod simd_parser;
pub mod sonic_parser;
pub mod standard;

pub use base::AJsonParser;
pub use hifijson_parser::HifijsonParser;
pub use json_fast_parser::JsonFastParser;
pub use jsonic_parser::JsonicParser;
pub use justjson_parser::JustjsonParser;
pub use picojson_parser::PicojsonParser;
pub use simd_parser::SimdJsonParser;
pub use sonic_parser::SonicParser;
pub use standard::StandardJsonParser;

pub use registry::{get_best_available_parser, get_parser, register_parser};
