// #exonware/xwsystem/rust/src/io/serialization/parsers/base.rs
//! Base JSON parser interface.

/// Abstract JSON parser base class for pluggable implementations.
pub trait AJsonParser {
    /// Parse JSON string/bytes to Python object.
    /// Args:
    /// s: JSON string or bytes
    /// Returns:
    /// Parsed Python object
    fn loads(&self, s: String) -> serde_json::Value;

    /// Serialize Python object to JSON.
    /// Args:
    /// obj: Python object to serialize
    /// **kwargs: Serialization options (ensure_ascii, indent, etc.)
    /// Returns:
    /// JSON string or bytes
    fn dumps(&self, obj: serde_json::Value) -> String;

    /// Parser identifier (e.g., 'standard', 'simd-json').
    fn parser_name(&self) -> &str;

    /// Performance tier:
    /// 0 = serde_json (baseline)
    /// 1 = simd-json (3-4x faster)
    /// 2 = Future optimizations (5-7x faster)
    /// 3 = Future pure Rust core (6-8x faster)
    fn tier(&self) -> i64;

    /// Check if parser is available (dependencies/features enabled).
    fn is_available(&self) -> bool;

}
