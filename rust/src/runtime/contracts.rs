// #exonware/xwsystem/rust/src/runtime/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Runtime module contracts - interfaces and enums for runtime environment functionality.


use std::collections::HashMap;

use crate::defs::{EnvironmentType, PlatformType, PythonVersion, RuntimeMode};

/// Interface for environment management.
pub trait IEnvironmentManager {
    /// Get current environment type.
    fn get_environment_type(&self) -> EnvironmentType;

    /// Set environment type.
    fn set_environment_type(&self, env_type: EnvironmentType) -> ();

    /// Get environment variable.
    fn get_environment_variable(&self, name: String, default: Option<String>) -> Option<String>;

    /// Set environment variable.
    fn set_environment_variable(&self, name: String, value: String) -> ();

    /// Get all environment variables.
    fn get_all_environment_variables(&self) -> HashMap<String, String>;

}

/// Interface for platform information.
pub trait IPlatformInfo {
    /// Get platform type.
    fn get_platform_type(&self) -> PlatformType;

    /// Get platform version.
    fn get_platform_version(&self) -> String;

    /// Get system architecture.
    fn get_architecture(&self) -> String;

    /// Get system hostname.
    fn get_hostname(&self) -> String;

    /// Get current username.
    fn get_username(&self) -> String;

}

/// Interface for Python information.
pub trait IPythonInfo {
    /// Get Python version.
    fn get_python_version(&self) -> PythonVersion;

    /// Get Python executable path.
    fn get_python_executable(&self) -> String;

    /// Get Python path.
    fn get_python_path(&self) -> Vec<String>;

    /// Get installed packages.
    fn get_installed_packages(&self) -> HashMap<String, String>;

    /// Check if package is installed.
    fn is_package_installed(&self, package_name: String) -> bool;

}

/// Interface for reflection utilities.
pub trait IReflectionUtils {
    /// Get class from string path.
    fn get_class_from_string(&self, class_path: String) -> String;

    /// Get function from string path.
    fn get_function_from_string(&self, function_path: String) -> fn();

    /// Find classes in module.
    fn find_classes_in_module(&self, module: serde_json::Value, base_class: String) -> Vec<String>;

    /// Get class hierarchy.
    fn get_class_hierarchy(&self) -> Vec<String>;

    /// Get class attributes.
    fn get_class_attributes(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for runtime configuration.
pub trait IRuntimeConfig {
    /// Get runtime mode.
    fn get_runtime_mode(&self) -> RuntimeMode;

    /// Set runtime mode.
    fn set_runtime_mode(&self, mode: RuntimeMode) -> ();

    /// Get configuration value.
    fn get_config_value(&self, key: String, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set configuration value.
    fn set_config_value(&self, key: String, value: serde_json::Value) -> ();

    /// Load configuration from file.
    fn load_config_from_file(&self, file_path: String) -> ();

    /// Save configuration to file.
    fn save_config_to_file(&self, file_path: String) -> ();

}
