// #exonware/xwsystem/rust/src/version.rs
//! Centralized version management for eXonware projects.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! 
//! This module provides centralized version management for the entire project.
//! All version references should import from this module to ensure consistency.

// =============================================================================
// VERSION CONFIGURATION
// =============================================================================

// Main version - update this to change version across entire project

// Version components for programmatic access

// Version metadata

// =============================================================================
// VERSION UTILITIES
// =============================================================================

use std::collections::HashMap;

/// Constant: VERSION_MAJOR

pub const VERSION_MAJOR: i64 = 0;

/// Constant: VERSION_MINOR
pub const VERSION_MINOR: i64 = 1;

/// Constant: VERSION_PATCH
pub const VERSION_PATCH: i64 = 0;

/// Constant: VERSION_BUILD
pub const VERSION_BUILD: i64 = 1;

/// Constant: VERSION_SUFFIX
pub const VERSION_SUFFIX: &str = "";

/// Constant: VERSION_STRING
pub const VERSION_STRING: &str = "0.1.0.1";

/// Constant: __version__
pub const __version__: &str = VERSION_STRING;

/// Get the current version string.
pub fn get_version() -> String {
    VERSION_STRING.to_string()
}

/// Get version as a tuple (major, minor, patch, build).
pub fn get_version_info() -> (i64, i64, i64, i64) {
    (VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_BUILD)
}

/// Get version information as a dictionary.
pub fn get_version_dict() -> HashMap<String, serde_json::Value> {
    let mut dict = HashMap::new();
    dict.insert("version".to_string(), serde_json::Value::String(VERSION_STRING.to_string()));
    dict.insert("major".to_string(), serde_json::Value::Number(VERSION_MAJOR.into()));
    dict.insert("minor".to_string(), serde_json::Value::Number(VERSION_MINOR.into()));
    dict.insert("patch".to_string(), serde_json::Value::Number(VERSION_PATCH.into()));
    dict.insert("build".to_string(), serde_json::Value::Number(VERSION_BUILD.into()));
    dict.insert("suffix".to_string(), serde_json::Value::String(VERSION_SUFFIX.to_string()));
    dict
}

/// Check if this is a development version.
pub fn is_dev_version() -> bool {
    VERSION_SUFFIX == "dev" || VERSION_SUFFIX == "alpha" || VERSION_SUFFIX == "beta" || VERSION_BUILD != 0
}

/// Check if this is a release version.
pub fn is_release_version() -> bool {
    !is_dev_version()
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use __version__;
pub use VERSION_MAJOR;
pub use VERSION_MINOR;
pub use VERSION_PATCH;
pub use VERSION_BUILD;
pub use VERSION_SUFFIX;
pub use VERSION_STRING;
pub use get_version;
pub use get_version_info;
pub use get_version_dict;
pub use is_dev_version;
pub use is_release_version;
