// #exonware/xwsystem/rust/src/config/version_manager.rs
//! Centralized version management for eXonware projects.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Generation Date: January 27, 2025
//! 
//! This module provides a centralized VersionManager class that can be used
//! across all eXonware projects to maintain consistent version management.

// Convenience function for creating version managers

// If build is not a number, append .1
/// Centralized version management for eXonware projects.
///
/// This class provides a unified way to manage versions across all
/// eXonware libraries, ensuring consistency and reducing code duplication.
use std::collections::HashMap;

pub struct VersionManager {
    version: String,
    project_name: String,
    version_components: (i64, i64, i64, Option<String>),
    suffix: String,
}

impl VersionManager {
    /// Initialize the version manager.
    ///
    /// Args:
    /// version: Version string in format "major.minor.patch" or "major.minor.patch.build"
    /// project_name: Optional project name for identification
    pub fn new(version: String, project_name: Option<String>) -> Self {
        let components = Self::_parse_version(&version);
        Self {
            version: version.clone(),
            project_name: project_name.unwrap_or_default(),
            version_components: components,
            suffix: String::new(),
        }
    }

    /// Parse version string into components.
    fn _parse_version(version: &str) -> (i64, i64, i64, Option<String>) {
        let parts: Vec<&str> = version.split('.').collect();
        let major = parts.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
        let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
        let build = parts.get(3).map(|s| s.to_string());
        (major, minor, patch, build)
    }

    /// Get the full version string.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the version string with suffix.
    pub fn version_string(&self) -> String {
        format!("{}{}", self.version, self.suffix)
    }

    /// Get the major version number.
    pub fn major(&self) -> i64 {
        self.version_components.0
    }

    /// Get the minor version number.
    pub fn minor(&self) -> i64 {
        self.version_components.1
    }

    /// Get the patch version number.
    pub fn patch(&self) -> i64 {
        self.version_components.2
    }

    /// Get the build version.
    pub fn build(&self) -> Option<String> {
        self.version_components.3.clone()
    }

    /// Get the version suffix.
    pub fn suffix(&self) -> &str {
        &self.suffix
    }

    /// Set the version suffix.
    pub fn set_suffix(&mut self, value: String) {
        self.suffix = value;
    }

    /// Get version as a tuple (major, minor, patch, build).
    pub fn get_version_info(&self) -> (i64, i64, i64, Option<String>) {
        self.version_components.clone()
    }

    /// Get version information as a dictionary.
    pub fn get_version_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut dict = HashMap::new();
        dict.insert("version".to_string(), serde_json::Value::String(self.version_string()));
        dict.insert("major".to_string(), serde_json::Value::Number(serde_json::Number::from(self.major())));
        dict.insert("minor".to_string(), serde_json::Value::Number(serde_json::Number::from(self.minor())));
        dict.insert("patch".to_string(), serde_json::Value::Number(serde_json::Number::from(self.patch())));
        if let Some(build) = &self.build() {
            dict.insert("build".to_string(), serde_json::Value::String(build.clone()));
        }
        dict.insert("project_name".to_string(), serde_json::Value::String(self.project_name.clone()));
        dict
    }

    /// Check if this is a development version.
    pub fn is_dev_version(&self) -> bool {
        self.suffix.contains("dev") || self.suffix.contains("alpha") || self.suffix.contains("beta")
    }

    /// Check if this is a release version.
    pub fn is_release_version(&self) -> bool {
        !self.is_dev_version()
    }

    /// Bump major version and return new version string.
    pub fn bump_major(&self) -> String {
        format!("{}.0.0", self.major() + 1)
    }

    /// Bump minor version and return new version string.
    pub fn bump_minor(&self) -> String {
        format!("{}.{}.0", self.major(), self.minor() + 1)
    }

    /// Bump patch version and return new version string.
    pub fn bump_patch(&self) -> String {
        format!("{}.{}.{}", self.major(), self.minor(), self.patch() + 1)
    }

    /// Bump build version and return new version string.
    pub fn bump_build(&self) -> String {
        if let Some(build) = &self.build() {
            if let Ok(build_num) = build.parse::<i64>() {
                format!("{}.{}.{}.{}", self.major(), self.minor(), self.patch(), build_num + 1)
            } else {
                format!("{}.{}.{}.{}.1", self.major(), self.minor(), self.patch(), build)
            }
        } else {
            format!("{}.{}.{}.1", self.major(), self.minor(), self.patch())
        }
    }
}

/// Create a VersionManager instance.
///
/// Args:
/// version: Version string
/// project_name: Optional project name
///
/// Returns:
/// VersionManager instance
pub fn create_version_manager(version: &str, project_name: Option<&str>) -> VersionManager {
    VersionManager::new(
        version.to_string(),
        project_name.map(|s| s.to_string()),
    )
}
