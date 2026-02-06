// #exonware/xwsystem/rust/tests/version_tests.rs
//! Integration tests for the version module

use xwsystem_rust::version;

#[test]
fn test_version_module_public_api() {
    // Test that all public functions work correctly
    let version_str = version::get_version();
    assert!(!version_str.is_empty());
    
    let (major, minor, patch, build) = version::get_version_info();
    // Version components are always non-negative (u32 type)
    let _ = (major, minor, patch, build);
    
    let info = version::get_version_dict();
    assert_eq!(info.version, version_str);
    assert_eq!(info.major, major);
    assert_eq!(info.minor, minor);
    assert_eq!(info.patch, patch);
    assert_eq!(info.build, build);
    
    // Test dev/release version checks
    let is_dev = version::is_dev_version();
    let is_release = version::is_release_version();
    assert_ne!(is_dev, is_release); // They should be opposite
}

#[test]
fn test_version_info_serialization() {
    use serde_json;
    
    let info = version::get_version_dict();
    let json = serde_json::to_string(&info).expect("Should serialize to JSON");
    let deserialized: version::VersionInfo = serde_json::from_str(&json)
        .expect("Should deserialize from JSON");
    
    assert_eq!(info, deserialized);
}

