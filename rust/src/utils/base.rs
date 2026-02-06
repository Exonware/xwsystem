// #exonware/xwsystem/rust/src/utils/base.rs
//exonware/xwsystem/utils/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Utils module base classes - abstract classes for utility functionality.


use std::collections::HashMap;

use crate::contracts::{LazyLoadMode, PathType, ResourceType, UtilityType};
use std::path::{Path};

/// Abstract base class for lazy loading operations.
pub trait ALazyLoaderBase {
    /// Set function to load object.
    fn set_load_function(&self, load_func: fn()) -> ();

    /// Load object.
    fn load(&self) -> T;

    /// Unload object.
    fn unload(&self) -> ();

    /// Reload object.
    fn reload(&self) -> T;

    /// Check if object is loaded.
    fn is_loaded(&self) -> bool;

    /// Check if object is currently loading.
    fn is_loading(&self) -> bool;

    /// Get loaded object.
    fn get_object(&self) -> Option<T>;

    /// Preload object.
    fn preload(&self) -> ();

    /// Get object load time.
    fn get_load_time(&self) -> Option<f64>;

    /// Get object memory usage.
    fn get_memory_usage(&self) -> i64;

}

/// Abstract base class for path utility operations.
pub trait APathUtilsBase {
    /// Normalize file path.
    fn normalize_path(&self, path: String) -> Path;

    /// Resolve file path.
    fn resolve_path(&self, path: String) -> Path;

    /// Get absolute path.
    fn absolute_path(&self, path: String) -> Path;

    /// Get relative path.
    fn relative_path(&self, path: String, start: Option<String>) -> Path;

    /// Join multiple paths.
    fn join_paths(&self) -> Path;

    /// Split path into directory and filename.
    fn split_path(&self, path: String) -> (Path, String);

    /// Get file extension.
    fn get_extension(&self, path: String) -> String;

    /// Get file stem.
    fn get_stem(&self, path: String) -> String;

    /// Get file/directory name.
    fn get_name(&self, path: String) -> String;

    /// Get parent directory.
    fn get_parent(&self, path: String) -> Path;

    /// Check if path is absolute.
    fn is_absolute(&self, path: String) -> bool;

    /// Check if path is relative.
    fn is_relative(&self, path: String) -> bool;

    /// Check if path exists.
    fn exists(&self, path: String) -> bool;

    /// Check if path is file.
    fn is_file(&self, path: String) -> bool;

    /// Check if path is directory.
    fn is_directory(&self, path: String) -> bool;

    /// Get path size.
    fn get_size(&self, path: String) -> i64;

    /// Get path modification time.
    fn get_modified_time(&self, path: String) -> f64;

    /// Sanitize path for security.
    fn sanitize_path(&self, path: String) -> String;

}

/// Abstract base class for utility registry operations.
pub trait AUtilityRegistryBase {
    /// Register utility.
    fn register_utility(&self, name: String, utility: serde_json::Value, utility_type: UtilityType, metadata: Option<HashMap<String, serde_json::Value>>) -> ();

    /// Unregister utility.
    fn unregister_utility(&self, name: String) -> bool;

    /// Get utility by name.
    fn get_utility(&self, name: String) -> Option<serde_json::Value>;

    /// List utilities.
    fn list_utilities(&self, utility_type: Option<UtilityType>) -> Vec<String>;

    /// Check if utility exists.
    fn has_utility(&self, name: String) -> bool;

    /// Get utility type.
    fn get_utility_type(&self, name: String) -> Option<UtilityType>;

    /// Get utility metadata.
    fn get_utility_metadata(&self, name: String) -> Option<HashMap<String, serde_json::Value>>;

    /// Update utility metadata.
    fn update_utility_metadata(&self, name: String, metadata: HashMap<String, serde_json::Value>) -> ();

    /// Clear all utilities.
    fn clear_utilities(&self) -> ();

    /// Get utility count.
    fn get_utility_count(&self) -> i64;

    /// Export utilities registry.
    fn export_utilities(&self) -> HashMap<String, serde_json::Value>;

    /// Import utilities registry.
    fn import_utilities(&self, utilities_data: HashMap<String, serde_json::Value>) -> ();

}

/// Abstract base class for configuration management.
pub trait AConfigManagerBase {
    /// Load configuration.
    fn load_config(&self, config_name: String, config_data: HashMap<String, serde_json::Value>) -> ();

    /// Save configuration to file.
    fn save_config(&self, config_name: String, file_path: String) -> ();

    /// Get configuration.
    fn get_config(&self, config_name: String) -> Option<HashMap<String, serde_json::Value>>;

    /// Set configuration value.
    fn set_config_value(&self, config_name: String, key: String, value: serde_json::Value) -> ();

    /// Get configuration value.
    fn get_config_value(&self, config_name: String, key: String, default: serde_json::Value) -> serde_json::Value;

    /// Check if configuration exists.
    fn has_config(&self, config_name: String) -> bool;

    /// Remove configuration.
    fn remove_config(&self, config_name: String) -> bool;

    /// List all configurations.
    fn list_configs(&self) -> Vec<String>;

    /// Validate configuration.
    fn validate_config(&self, config_name: String) -> bool;

    /// Set configuration schema.
    fn set_config_schema(&self, config_name: String, schema: HashMap<String, serde_json::Value>) -> ();

    /// Get configuration schema.
    fn get_config_schema(&self, config_name: String) -> Option<HashMap<String, serde_json::Value>>;

    /// Set configuration validator.
    fn set_config_validator(&self, config_name: String, validator: fn()) -> ();

    /// Get configuration validator.
    fn get_config_validator(&self, config_name: String) -> Option<fn()>;

}

/// Abstract base class for resource management.
pub trait AResourceManagerBase {
    /// Acquire resource.
    fn acquire_resource(&self, resource_id: String, resource_type: ResourceType) -> Option<serde_json::Value>;

    /// Release resource.
    fn release_resource(&self, resource_id: String) -> ();

    /// Get resource by ID.
    fn get_resource(&self, resource_id: String) -> Option<serde_json::Value>;

    /// Check if resource exists.
    fn has_resource(&self, resource_id: String) -> bool;

    /// List resources.
    fn list_resources(&self, resource_type: Option<ResourceType>) -> Vec<String>;

    /// Get resource type.
    fn get_resource_type(&self, resource_id: String) -> Option<ResourceType>;

    /// Check if resource is locked.
    fn is_resource_locked(&self, resource_id: String) -> bool;

    /// Lock resource.
    fn lock_resource(&self, resource_id: String) -> bool;

    /// Unlock resource.
    fn unlock_resource(&self, resource_id: String) -> ();

    /// Get resource usage statistics.
    fn get_resource_usage(&self, resource_id: String) -> Option<HashMap<String, serde_json::Value>>;

    /// Cleanup unused resources.
    fn cleanup_resources(&self) -> i64;

    /// Get resource count.
    fn get_resource_count(&self) -> i64;

    /// Get resource statistics.
    fn get_resource_stats(&self) -> HashMap<String, serde_json::Value>;

}

/// Base implementation of utility functions.
pub struct BaseUtils {
    _utilities: HashMap<String, serde_json::Value>,
    _initialized: bool,
}

impl BaseUtils {
    /// Initialize base utils.
    pub fn new() -> Self {
        Self {
            _utilities: HashMap::new(),
            _initialized: false,
        }
    }

    /// Initialize utilities.
    pub fn initialize(&mut self) {
        self._initialized = true;
    }

    /// Check if utilities are initialized.
    pub fn is_initialized(&self) -> bool {
        self._initialized
    }

    /// Register a utility.
    pub fn register_utility(&mut self, name: String, utility: serde_json::Value) {
        self._utilities.insert(name, utility);
    }

    /// Get utility by name.
    pub fn get_utility(&self, name: &str) -> Option<serde_json::Value> {
        self._utilities.get(name).cloned()
    }

    /// Check if utility exists.
    pub fn has_utility(&self, name: &str) -> bool {
        self._utilities.contains_key(name)
    }

    /// List all utilities.
    pub fn list_utilities(&self) -> Vec<String> {
        self._utilities.keys().cloned().collect()
    }

    /// Remove utility.
    pub fn remove_utility(&mut self, name: &str) -> bool {
        self._utilities.remove(name).is_some()
    }

    /// Clear all utilities.
    pub fn clear_utilities(&mut self) {
        self._utilities.clear();
    }

    /// Get utility count.
    pub fn get_utility_count(&self) -> i64 {
        self._utilities.len() as i64
    }
}
