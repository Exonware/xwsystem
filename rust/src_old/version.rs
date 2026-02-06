// #exonware/xwsystem/rust/src_old/version.rs
//! Version management module for XWSystem.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module provides centralized version management for the entire project.
//! All version references should use this module to ensure consistency.

use serde::{Deserialize, Serialize};

// =============================================================================
// VERSION CONFIGURATION
// =============================================================================

/// Main version - update this to change version across entire project
pub const VERSION: &str = "0.1.0.1";

/// Version components for programmatic access
pub const VERSION_MAJOR: u32 = 0;
pub const VERSION_MINOR: u32 = 1;
pub const VERSION_PATCH: u32 = 0;
pub const VERSION_BUILD: Option<u32> = Some(1); // Set to None for releases, or build number for dev builds

/// Version metadata
pub const VERSION_SUFFIX: &str = ""; // e.g., "dev", "alpha", "beta", "rc1"
pub const VERSION_STRING: &str = "0.1.0.1";

// =============================================================================
// VERSION STRUCTURES
// =============================================================================

/// Version information structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub build: Option<u32>,
    pub suffix: String,
}

// =============================================================================
// VERSION UTILITIES
// =============================================================================

/// Get the current version string.
pub fn get_version() -> &'static str {
    VERSION_STRING
}

/// Get version as a tuple (major, minor, patch, build).
pub fn get_version_info() -> (u32, u32, u32, Option<u32>) {
    (VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_BUILD)
}

/// Get version information as a VersionInfo struct.
pub fn get_version_dict() -> VersionInfo {
    VersionInfo {
        version: VERSION_STRING.to_string(),
        major: VERSION_MAJOR,
        minor: VERSION_MINOR,
        patch: VERSION_PATCH,
        build: VERSION_BUILD,
        suffix: VERSION_SUFFIX.to_string(),
    }
}

/// Check if this is a development version.
pub fn is_dev_version() -> bool {
    VERSION_SUFFIX == "dev" 
        || VERSION_SUFFIX == "alpha" 
        || VERSION_SUFFIX == "beta" 
        || VERSION_BUILD.is_some()
}

/// Check if this is a release version.
pub fn is_release_version() -> bool {
    !is_dev_version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        assert_eq!(get_version(), "0.1.0.1");
    }

    #[test]
    fn test_get_version_info() {
        let (major, minor, patch, build) = get_version_info();
        assert_eq!(major, 0);
        assert_eq!(minor, 1);
        assert_eq!(patch, 0);
        assert_eq!(build, Some(1));
    }

    #[test]
    fn test_get_version_dict() {
        let info = get_version_dict();
        assert_eq!(info.version, "0.1.0.1");
        assert_eq!(info.major, 0);
        assert_eq!(info.minor, 1);
        assert_eq!(info.patch, 0);
        assert_eq!(info.build, Some(1));
        assert_eq!(info.suffix, "");
    }

    #[test]
    fn test_is_dev_version() {
        // With build number, should be dev version
        assert!(is_dev_version());
    }

    #[test]
    fn test_is_release_version() {
        // Currently not a release version due to build number
        assert!(!is_release_version());
    }

    #[test]
    fn test_version_constants() {
        assert_eq!(VERSION, "0.1.0.1");
        assert_eq!(VERSION_MAJOR, 0);
        assert_eq!(VERSION_MINOR, 1);
        assert_eq!(VERSION_PATCH, 0);
        assert_eq!(VERSION_BUILD, Some(1));
        assert_eq!(VERSION_SUFFIX, "");
        assert_eq!(VERSION_STRING, "0.1.0.1");
    }
}

