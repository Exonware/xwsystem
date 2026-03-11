// #exonware/xwsystem/rust/src/utils/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Utils module contracts - interfaces and enums for utility functionality.


use std::collections::HashMap;

use crate::defs::{LazyLoadMode, LazyLoadStrategy, PathType, ResourceType, UtilityType};
use std::path::{Path};

/// Interface for lazy loading operations.
pub trait ILazyLoader {
    /// Load the object.
    fn load(&self) -> T;

    /// Check if object is loaded.
    fn is_loaded(&self) -> bool;

    /// Unload the object.
    fn unload(&self) -> ();

    /// Reload the object.
    fn reload(&self) -> T;

    /// Get the loader function.
    fn get_loader_function(&self) -> fn();

}

/// Interface for path utility operations.
pub trait IPathUtils {
    /// Normalize path.
    fn normalize_path(&self, path: String) -> Path;

    /// Resolve path to absolute.
    fn resolve_path(&self, path: String) -> Path;

    /// Get path type.
    fn get_path_type(&self, path: String) -> PathType;

    /// Check if path is safe.
    fn is_safe_path(&self, path: String) -> bool;

    /// Sanitize path.
    fn sanitize_path(&self, path: String) -> Path;

    /// Get relative path.
    fn get_relative_path(&self, path: String, base: String) -> Path;

    /// Ensure path exists.
    fn ensure_path_exists(&self, path: String) -> ();

}

/// Interface for utility registry operations.
pub trait IUtilityRegistry {
    /// Register utility.
    fn register_utility(&self, name: String, utility: serde_json::Value) -> ();

    /// Get utility by name.
    fn get_utility(&self, name: String) -> Option<serde_json::Value>;

    /// Unregister utility.
    fn unregister_utility(&self, name: String) -> ();

    /// List all registered utilities.
    fn list_utilities(&self) -> Vec<String>;

    /// Check if utility is registered.
    fn has_utility(&self, name: String) -> bool;

}

/// Interface for configuration management.
pub trait IConfigManager {
    /// Get configuration value.
    fn get_config(&self, key: String, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set configuration value.
    fn set_config(&self, key: String, value: serde_json::Value) -> ();

    /// Load configuration from source.
    fn load_config(&self, source: String) -> ();

    /// Save configuration to destination.
    fn save_config(&self, destination: String) -> ();

    /// Get all configuration.
    fn get_all_config(&self) -> HashMap<String, serde_json::Value>;

    /// Clear all configuration.
    fn clear_config(&self) -> ();

}

/// Interface for resource management.
pub trait IResourceManager {
    /// Acquire resource.
    fn acquire_resource(&self, resource_id: String) -> serde_json::Value;

    /// Release resource.
    fn release_resource(&self, resource_id: String) -> ();

    /// Check if resource is available.
    fn is_resource_available(&self, resource_id: String) -> bool;

    /// Get total resource count.
    fn get_resource_count(&self) -> i64;

    /// Get list of available resources.
    fn get_available_resources(&self) -> Vec<String>;

}
