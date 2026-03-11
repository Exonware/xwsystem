// #exonware/xwsystem/rust/src_old/contracts.rs
//! Core contracts (interfaces) for XWSystem.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module provides trait definitions (interfaces) for XWSystem components.
//! In Rust, interfaces are represented as traits.

use crate::defs::{CloneMode, CoreMode, CoreState, DataType};
use std::collections::HashMap;

// ============================================================================
// CORE IDENTITY TRAITS
// ============================================================================

/// Trait for objects that have unique identification.
///
/// Enforces consistent ID management across XWSystem components.
pub trait IId {
    /// Get the primary identifier.
    fn id(&self) -> &str;

    /// Get the unique identifier (UUID).
    fn uid(&self) -> &str;

    /// Generate a new ID.
    fn generate_id(&mut self) -> String;

    /// Validate an ID format.
    fn validate_id(&self, id_value: &str) -> bool;

    /// Check if this object has the same ID as another.
    fn is_same_id(&self, other: &dyn IId) -> bool {
        self.id() == other.id()
    }
}

// ============================================================================
// NATIVE DATA CONVERSION TRAITS
// ============================================================================

/// Trait for objects that can convert to/from string representation.
///
/// Enforces consistent string conversion behavior across XWSystem.
pub trait IStringable {
    /// Convert object to string representation.
    fn to_string(&self) -> String;

    /// Initialize object from string representation.
    fn from_string(&mut self, string: &str) -> Result<(), String>;
}

/// Trait for objects that can convert to/from native Rust types.
///
/// Enforces consistent native data conversion across XWSystem.
pub trait INative {
    /// Convert to native Rust object (serde_json::Value).
    fn to_native(&self) -> serde_json::Value;

    /// Create from native Rust object.
    fn from_native(&mut self, data: &serde_json::Value) -> Result<(), String>;

    /// Check if data is compatible with native conversion.
    fn is_native_compatible(&self, data: &serde_json::Value) -> bool;

    /// Get the native data type.
    fn get_native_type(&self) -> DataType;
}

// ============================================================================
// CORE OBJECT TRAIT
// ============================================================================

use chrono::{DateTime, Utc};

/// Core trait for all objects in the eXonware ecosystem.
///
/// This is the foundational object trait that combines identity (IId),
/// native conversion (INative), timestamps, metadata (title/description),
/// serialization, and storage operations.
pub trait IObject: IId + INative {
    /// Get the creation timestamp.
    fn created_at(&self) -> DateTime<Utc>;

    /// Get the last update timestamp.
    fn updated_at(&self) -> DateTime<Utc>;

    /// Get the object title.
    fn title(&self) -> Option<&str>;

    /// Get the object description.
    fn description(&self) -> Option<&str>;

    /// Export object as dictionary (serde_json::Value).
    ///
    /// Should include id, uid, created_at, updated_at, title, description,
    /// and any object-specific data.
    fn to_dict(&self) -> serde_json::Value;

    /// Save object to storage.
    ///
    /// Subclasses implement object-specific storage logic.
    fn save(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Load object from storage.
    ///
    /// Subclasses implement object-specific loading logic.
    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

// ============================================================================
// CLONING TRAITS
// ============================================================================

/// Trait for objects that can be cloned.
///
/// Enforces consistent cloning behavior across XWSystem.
pub trait ICloneable: Clone {
    /// Create a clone of this object.
    fn clone_with_mode(&self, mode: CloneMode) -> Self;

    /// Create a deep clone.
    fn deep_clone(&self) -> Self {
        self.clone_with_mode(CloneMode::Deep)
    }

    /// Create a shallow clone.
    fn shallow_clone(&self) -> Self {
        self.clone_with_mode(CloneMode::Shallow)
    }

    /// Create a reference clone (same object, different reference).
    fn reference_clone(&self) -> Self {
        self.clone_with_mode(CloneMode::Reference)
    }

    /// Check if object can be cloned in given mode.
    fn is_cloneable(&self, _mode: CloneMode) -> bool {
        true // Default implementation
    }
}

// ============================================================================
// COMPARISON TRAITS
// ============================================================================

/// Trait for objects that can be compared.
///
/// Enforces consistent comparison behavior across XWSystem.
pub trait IComparable: PartialEq + Eq {
    /// Check if this object equals another (using PartialEq).
    fn equals(&self, other: &Self) -> bool {
        self == other
    }

    /// Compare this object to another.
    ///
    /// Returns: -1 if less than, 0 if equal, 1 if greater than
    fn compare_to(&self, other: &Self) -> i32;

    /// Get hash code for this object.
    fn hash_code(&self) -> u64;

    /// Check if this object can be compared to another.
    fn is_comparable(&self, _other: &Self) -> bool {
        true // Default implementation
    }
}

// ============================================================================
// ITERATION TRAITS
// ============================================================================

/// Trait for objects that can be iterated.
///
/// Enforces consistent iteration behavior across XWSystem.
pub trait IIterable {
    /// Get length of this object.
    fn len(&self) -> usize;

    /// Check if object is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if object contains item.
    fn contains(&self, item: &serde_json::Value) -> bool;

    /// Check if object is iterable.
    fn is_iterable(&self) -> bool {
        true // Default implementation
    }

    /// Get the type of iterator this object provides.
    fn get_iterator_type(&self) -> String;
}

// ============================================================================
// CONTAINER TRAITS
// ============================================================================

/// Trait for objects that act as containers.
///
/// Enforces consistent container behavior across XWSystem.
pub trait IContainer {
    /// Add item to container.
    fn add(&mut self, item: serde_json::Value) -> Result<(), String>;

    /// Remove item from container.
    fn remove(&mut self, item: &serde_json::Value) -> Result<(), String>;

    /// Clear all items from container.
    fn clear(&mut self);

    /// Check if container is empty.
    fn is_empty(&self) -> bool;

    /// Get size of container.
    fn size(&self) -> usize;

    /// Check if container contains item.
    fn contains(&self, item: &serde_json::Value) -> bool;
}

// ============================================================================
// METADATA TRAITS
// ============================================================================

/// Trait for objects that have metadata.
///
/// Enforces consistent metadata handling across XWSystem.
pub trait IMetadata {
    /// Get metadata value by key.
    fn get_metadata(&self, key: &str) -> Option<&serde_json::Value>;

    /// Set metadata value by key.
    fn set_metadata(&mut self, key: &str, value: serde_json::Value);

    /// Check if metadata key exists.
    fn has_metadata(&self, key: &str) -> bool;

    /// Remove metadata by key.
    fn remove_metadata(&mut self, key: &str) -> bool;

    /// Get all metadata.
    fn get_all_metadata(&self) -> HashMap<String, serde_json::Value>;

    /// Clear all metadata.
    fn clear_metadata(&mut self);
}

// ============================================================================
// LIFECYCLE TRAITS
// ============================================================================

/// Trait for objects with lifecycle management.
///
/// Enforces consistent lifecycle behavior across XWSystem.
pub trait ILifecycle {
    /// Initialize the object.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Start the object.
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Stop the object.
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown the object.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if object is initialized.
    fn is_initialized(&self) -> bool;

    /// Check if object is running.
    fn is_running(&self) -> bool;

    /// Get current state.
    fn get_state(&self) -> String;
}

// ============================================================================
// FACTORY TRAITS
// ============================================================================

/// Trait for factory objects.
///
/// Enforces consistent factory behavior across XWSystem.
pub trait IFactory {
    /// Create a new instance.
    fn create(&self, args: &serde_json::Value) -> Result<Box<dyn std::any::Any>, String>;

    /// Create instance from configuration.
    fn create_from_config(&self, config: &serde_json::Value) -> Result<Box<dyn std::any::Any>, String>;

    /// Get list of supported types.
    fn get_supported_types(&self) -> Vec<String>;

    /// Check if factory can create type.
    fn can_create(&self, type_name: &str) -> bool;
}

// ============================================================================
// CORE TRAIT
// ============================================================================

/// Trait for core functionality.
pub trait ICore {
    /// Get core mode.
    fn mode(&self) -> CoreMode;

    /// Get core state.
    fn state(&self) -> CoreState;

    /// Initialize core functionality.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown core functionality.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if core is initialized.
    fn is_initialized(&self) -> bool;

    /// Check if core is shutdown.
    fn is_shutdown(&self) -> bool;

    /// Get all dependencies.
    fn get_dependencies(&self) -> Vec<String>;

    /// Check if all dependencies are satisfied.
    fn check_dependencies(&self) -> bool;
}

// ============================================================================
// ROOT-LEVEL SYSTEM INTERFACES (matching Python contracts.py)
// ============================================================================

/// Trait for root XWSystem framework.
///
/// Matches Python's IXWSystem interface.
/// Provides unified access to all XWSystem capabilities.
pub trait IXWSystem {
    /// Get XWSystem version.
    fn get_version(&self) -> &str;

    /// Get current system mode.
    fn get_mode(&self) -> CoreMode;

    /// Get current system state.
    fn get_state(&self) -> CoreState;

    /// Initialize XWSystem.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown XWSystem.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if XWSystem is initialized.
    fn is_initialized(&self) -> bool;

    /// Get configuration manager.
    fn get_config(&self) -> Option<&dyn std::any::Any>;

    /// Get security validator.
    fn get_security(&self) -> Option<&dyn std::any::Any>;

    /// Get monitoring instance.
    fn get_monitoring(&self) -> Option<&dyn std::any::Any>;

    /// Get cache manager.
    fn get_cache(&self) -> Option<&dyn std::any::Any>;

    /// Get plugin manager.
    fn get_plugins(&self) -> Option<&dyn std::any::Any>;
}

/// Trait for system components.
///
/// Matches Python's ISystemComponent interface.
/// All XWSystem components should implement this trait.
pub trait ISystemComponent {
    /// Get component name.
    fn get_name(&self) -> &str;

    /// Get component version.
    fn get_version(&self) -> &str;

    /// Initialize component.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown component.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if component is initialized.
    fn is_initialized(&self) -> bool;

    /// Get component configuration.
    fn get_config(&self) -> HashMap<String, serde_json::Value>;

    /// Set component configuration.
    fn set_config(&mut self, config: HashMap<String, serde_json::Value>);
}

/// Trait for configurable components.
///
/// Matches Python's IConfigurable interface from config.contracts.
pub trait IConfigurable {
    /// Configure component with options.
    fn configure(&mut self, options: HashMap<String, serde_json::Value>);

    /// Get current configuration.
    fn get_config(&self) -> HashMap<String, serde_json::Value>;

    /// Reset configuration to defaults.
    fn reset_config(&mut self);

    /// Update single configuration value.
    fn update_config(&mut self, key: &str, value: serde_json::Value);

    /// Check if configuration key exists.
    fn has_config(&self, key: &str) -> bool;

    /// Remove configuration key.
    fn remove_config(&mut self, key: &str) -> bool;

    /// Merge configuration with existing.
    fn merge_config(&mut self, config: HashMap<String, serde_json::Value>, priority: Option<serde_json::Value>);
}

/// Trait for extensible components.
///
/// Matches Python's IExtensible interface.
/// Components that support plugins and extensions.
pub trait IExtensible {
    /// Register a plugin.
    fn register_plugin(&mut self, plugin: Box<dyn std::any::Any>) -> Result<(), String>;

    /// Unregister a plugin.
    fn unregister_plugin(&mut self, plugin_id: &str) -> Result<(), String>;

    /// Get all registered plugins.
    fn get_plugins(&self) -> Vec<&dyn std::any::Any>;

    /// Check if plugin is registered.
    fn has_plugin(&self, plugin_id: &str) -> bool;
}

/// Combined trait for configurable system components.
///
/// Matches Python's IConfigurableComponent interface.
/// Combines ISystemComponent and IConfigurable for convenience.
pub trait IConfigurableComponent: ISystemComponent + IConfigurable {}

/// Combined trait for monitored system components.
///
/// Matches Python's IMonitoredComponent interface.
/// Combines ISystemComponent with monitoring capabilities.
pub trait IMonitoredComponent: ISystemComponent {
    /// Start monitoring.
    fn start_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Stop monitoring.
    fn stop_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Get component metrics.
    fn get_metrics(&self) -> HashMap<String, serde_json::Value>;
}

/// Combined trait for secure system components.
///
/// Matches Python's ISecureComponent interface.
/// Combines ISystemComponent with security capabilities.
pub trait ISecureComponent: ISystemComponent {
    /// Validate security of data.
    fn validate_security(&self, data: &serde_json::Value) -> bool;

    /// Check if action is permitted on resource.
    fn check_permissions(&self, action: &str, resource: &str) -> bool;
}

#[cfg(test)]
mod tests {
    // Test implementations would go here
    // For now, we just verify the traits compile
}

