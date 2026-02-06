// #exonware/xwsystem/rust/tests/1.unit/defs_tests/test_defs.rs
//! Unit tests for defs module.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! These tests verify individual functions and types in defs module.

use xwsystem_rust::defs::*;

#[test]
fn test_enum_serialization() {
    // Test that enums can be serialized/deserialized
    let status = SystemStatus::Ready;
    let json = serde_json::to_string(&status).unwrap();
    let deserialized: SystemStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(status, deserialized);

    let level = ValidationLevel::Strict;
    let json = serde_json::to_string(&level).unwrap();
    let deserialized: ValidationLevel = serde_json::from_str(&json).unwrap();
    assert_eq!(level, deserialized);
}

#[test]
fn test_enum_defaults() {
    assert_eq!(ValidationLevel::default(), ValidationLevel::Basic);
    assert_eq!(PerformanceLevel::default(), PerformanceLevel::Medium);
    assert_eq!(AuthType::default(), AuthType::None);
}

#[test]
fn test_constants_match_python() {
    // Verify constants match Python defs.py exactly
    // These values must match Python 100%
    assert_eq!(DEFS_VERSION_MAJOR, 0);
    assert_eq!(DEFS_VERSION_MINOR, 1);
    assert_eq!(DEFS_VERSION_PATCH, 0);
    assert_eq!(DEFS_VERSION_BUILD, 1);
    assert_eq!(DEFS_VERSION_STRING, "0.1.0.1");
}

