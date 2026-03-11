// #exonware/xwsystem/rust/src/shared/base.rs
//exonware/xwsystem/shared/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Shared base classes (merged from the former core module).


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::contracts::{CoreMode, CorePriority, CoreState, IObject, IID, INative};
use super::defs::DataType;

// Simple UUID v4-like generator
// In production, use the uuid crate: uuid::Uuid::new_v4().to_string()
fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:016x}-{:04x}-{:04x}-{:04x}-{:012x}",
        timestamp & 0xffffffffffff,
        (timestamp >> 48) as u16 & 0xffff,
        ((timestamp >> 32) as u16 & 0x0fff) | 0x4000, // version 4
        ((timestamp >> 16) as u16 & 0x3fff) | 0x8000, // variant
        timestamp as u64 & 0xffffffffffff
    )
}

// ============================================================================

// OBJECT BASE CLASSES

// ============================================================================

/// Abstract base class for core functionality.
pub trait ACoreBase {
    /// Initialize core functionality.
    fn initialize(&mut self);

    /// Shutdown core functionality.
    fn shutdown(&mut self);

    /// Get current core state.
    fn get_state(&self) -> CoreState;

    /// Set core state.
    fn set_state(&mut self, state: CoreState);

    /// Check if core is initialized.
    fn is_initialized(&self) -> bool;

    /// Check if core is shutdown.
    fn is_shutdown(&self) -> bool;

    /// Add core dependency.
    fn add_dependency(&mut self, dependency: String);

    /// Remove core dependency.
    fn remove_dependency(&mut self, dependency: String);

    /// Get all dependencies.
    fn get_dependencies(&self) -> Vec<String>;

    /// Check if all dependencies are satisfied.
    fn check_dependencies(&self) -> bool;
}

/// Abstract base class for resource management.
pub trait AResourceManagerBase {
    /// Acquire resource.
    fn acquire_resource(&mut self, resource_id: String, priority: CorePriority) -> serde_json::Value;

    /// Release resource.
    fn release_resource(&mut self, resource_id: String);

    /// Check if resource is available.
    fn is_resource_available(&self, resource_id: &str) -> bool;

    /// Get number of managed resources.
    fn get_resource_count(&self) -> usize;

    /// List all resource IDs.
    fn list_resources(&self) -> Vec<String>;

    /// Cleanup all resources.
    fn cleanup_resources(&mut self);
}

/// Abstract base class for core configuration.
pub trait AConfigurationBase {
    /// Load configuration data.
    fn load_config(&mut self, config_data: HashMap<String, serde_json::Value>);

    /// Get configuration value.
    fn get_config_value(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set configuration value.
    fn set_config_value(&mut self, key: String, value: serde_json::Value);

    /// Validate configuration.
    fn validate_config(&self) -> bool;

    /// Reset configuration to defaults.
    fn reset_to_defaults(&mut self);

    /// Export configuration as dictionary.
    fn export_config(&self) -> HashMap<String, serde_json::Value>;
}

/// Abstract base class for core validation.
pub trait AValidationBase {
    /// Validate input data.
    fn validate_input(&self, data: &serde_json::Value) -> bool;

    /// Validate output data.
    fn validate_output(&self, data: &serde_json::Value) -> bool;

    /// Validate operation.
    fn validate_operation(&self, operation: &str) -> bool;

    /// Get validation errors.
    fn get_validation_errors(&self) -> Vec<String>;

    /// Clear validation errors.
    fn clear_validation_errors(&mut self);
}

/// Abstract base class for core operations.
pub trait AOperationBase {
    /// Execute operation.
    fn execute(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>>;

    /// Check if operation is running.
    fn is_running(&self) -> bool;

    /// Check if operation is completed.
    fn is_completed(&self) -> bool;

    /// Check if operation failed.
    fn is_failed(&self) -> bool;

    /// Get operation duration.
    fn get_duration(&self) -> Option<f64>;

    /// Cancel operation.
    fn cancel(&mut self);

    /// Get operation result.
    fn get_result(&self) -> Option<serde_json::Value>;

    /// Get operation error.
    fn get_error(&self) -> Option<String>;
}

/// Base implementation of core functionality.
pub struct BaseCore {
    pub mode: CoreMode,
    pub state: CoreState,
    pub _dependencies: Vec<String>,
    pub _resources: HashMap<String, serde_json::Value>,
    pub _initialized: bool,
    pub _shutdown: bool,
}

impl ACoreBase for BaseCore {
    fn initialize(&mut self) {
        self.state = CoreState::Initializing;
        self._initialized = true;
        self.state = CoreState::Initialized;
    }

    fn shutdown(&mut self) {
        self.state = CoreState::ShuttingDown;
        self._shutdown = true;
        self.state = CoreState::Shutdown;
    }

    fn get_state(&self) -> CoreState {
        self.state
    }

    fn set_state(&mut self, state: CoreState) {
        self.state = state;
    }

    fn is_initialized(&self) -> bool {
        self._initialized
    }

    fn is_shutdown(&self) -> bool {
        self._shutdown
    }

    fn add_dependency(&mut self, dependency: String) {
        if !self._dependencies.contains(&dependency) {
            self._dependencies.push(dependency);
        }
    }

    fn remove_dependency(&mut self, dependency: String) {
        self._dependencies.retain(|d| d != &dependency);
    }

    fn get_dependencies(&self) -> Vec<String> {
        self._dependencies.clone()
    }

    fn check_dependencies(&self) -> bool {
        // Basic implementation - can be overridden
        true
    }
}

impl BaseCore {
    /// Initialize base core.
    pub fn new(mode: Option<CoreMode>) -> Self {
        Self {
            mode: mode.unwrap_or(CoreMode::Development),
            state: CoreState::Initializing,
            _dependencies: Vec::new(),
            _resources: HashMap::new(),
            _initialized: false,
            _shutdown: false,
        }
    }
}

// Timestamps are initialized by subclasses
// They must set self._created_at and self._updated_at in their __init__
// Title and description are optional and initialized by subclasses
// Default implementation - subclasses can override
/// Abstract base class for all objects in the eXonware ecosystem.
///
/// Provides common functionality for objects across xwauth, xwstorage, xwentity,
/// and other libraries. Extends IObject interface. All object types share:
/// - Identity (id, uid properties from IID)
/// - Timestamps (created_at, updated_at)
/// - Metadata (title, description)
/// - Native conversion (to_native, from_native from INative)
/// - Serialization (to_dict)
/// - Storage operations (save, load)
///
/// Subclasses must implement:
/// - id property (returns object identifier)
/// - created_at property
/// - updated_at property
/// - to_dict() method (should include title and description)
/// - save() method (object-specific storage logic)
/// - load() method (object-specific loading logic)
pub trait AObject: IObject {
    /// Get the unique object identifier.
    // Python decorators: @property
    fn id(&self) -> &str;

    /// Get the unique object GUID (universal identifier).
    /// This should be generated on object creation. XWObject provides
    /// a default implementation that generates a UUID.
    fn uid(&self) -> &str {
        // Default implementation - subclasses can override
        // This will be implemented in XWObject
        ""
    }

    /// Get the creation timestamp.
    fn created_at(&self) -> String;

    /// Get the last update timestamp.
    fn updated_at(&self) -> String;

    /// Get the object title.
    /// Default implementation returns None. Subclasses should override
    /// to provide title support, storing it in self._title.
    fn title(&self) -> Option<String> {
        None
    }

    /// Get the object description.
    /// Default implementation returns None. Subclasses should override
    /// to provide description support, storing it in self._description.
    fn description(&self) -> Option<String> {
        None
    }

    /// Export object as dictionary.
    /// Should include id, uid, created_at, updated_at, title, description,
    /// and any object-specific data.
    fn to_dict(&self) -> HashMap<String, serde_json::Value>;

    /// Get object as native representation.
    /// Default implementation returns to_dict(). Subclasses can override
    /// if they need a different native representation.
    fn to_native(&self) -> serde_json::Value {
        serde_json::to_value(self.to_dict()).unwrap_or(serde_json::Value::Null)
    }

    /// Create from native Python object.
    /// Default implementation - subclasses should override.
    fn from_native(&mut self, _data: serde_json::Value) {
        // Default does nothing - subclasses must implement
    }

    /// Check if data is compatible with native conversion.
    /// Default implementation checks if data is a dict.
    fn is_native_compatible(&self, data: &serde_json::Value) -> bool {
        data.is_object()
    }

    /// Save object to storage.
    /// Subclasses must implement this with object-specific storage logic.
    /// This method can be decorated with @XWAction to enable action-based
    /// execution, validation, and authorization.
    fn save(&self) -> ();

    /// Load object from storage.
    /// Subclasses must implement this with object-specific loading logic.
    /// This method can be decorated with @XWAction to enable action-based
    /// execution, validation, and authorization.
    fn load(&self) -> ();

    /// Generate a new ID.
    /// Default implementation generates a UUID. Subclasses can override
    /// for custom ID generation logic.
    fn generate_id(&self) -> String {
        generate_uuid()
    }

    /// Validate an ID format.
    /// Default implementation checks if ID is a non-empty string.
    fn validate_id(&self, id_value: &str) -> bool {
        !id_value.is_empty()
    }

    /// Check if this object has the same ID as another.
    fn is_same_id(&self, other: &dyn IObject) -> bool {
        self.id() == other.id()
    }

}

// Generate uid (GUID) on creation
// Timestamps are initialized by subclasses
// They must set self._created_at and self._updated_at in their __init__
// Title and description are optional
// Subclasses must implement this by updating self._updated_at
// This is a helper method that can be overridden
/// Concrete base class for all objects in the eXonware ecosystem.
///
/// Provides common functionality shared by objects across xwauth, xwstorage,
/// xwentity, and other libraries:
/// - Identity management (id property, uid with UUID generation)
/// - Timestamp tracking (created_at, updated_at - abstract, must be set by subclasses)
/// - Metadata (title, description - optional)
/// - Serialization (to_dict, to_native)
/// - Storage operations (save, load - abstract, to be implemented by subclasses)
///
/// Subclasses must implement:
/// - id property (returns object identifier)
/// - created_at property
/// - updated_at property
/// - to_dict() method (should include id, uid, created_at, updated_at, title, description)
/// - save() method (object-specific storage logic)
/// - load() method (object-specific loading logic)
///
/// Example:
/// >>> class MyEntity(XWObject):
/// ...     def __init__(self):
/// ...         super().__init__()
/// ...         self._id = "my_id"
/// ...         self._created_at = datetime.now()
/// ...         self._updated_at = self._created_at
/// ...         self._title = "My Entity"
/// ...         self._description = "Description of my entity"
/// ...
/// ...     @property
/// ...     def id(self) -> str:
/// ...         return self._id
/// ...
/// ...     @property
/// ...     def created_at(self) -> datetime:
/// ...         return self._created_at
/// ...
/// ...     @property
/// ...     def updated_at(self) -> datetime:
/// ...         return self._updated_at
pub struct XWObject {
    pub _id: String,
    pub _uid: String,
    pub _created_at: String,
    pub _updated_at: String,
    pub _title: Option<String>,
    pub _description: Option<String>,
}

impl IID for XWObject {
    fn id(&self) -> &str {
        &self._id
    }

    fn uid(&self) -> &str {
        &self._uid
    }

    fn generate_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    fn validate_id(&self, id_value: &str) -> bool {
        !id_value.is_empty()
    }

    fn is_same_id(&self, other: &str) -> bool {
        self._id == other
    }
}

impl INative for XWObject {
    fn to_native(&self) -> serde_json::Value {
        serde_json::to_value(self.to_dict()).unwrap_or(serde_json::Value::Null)
    }

    fn from_native(&mut self, _data: serde_json::Value) -> String {
        // Default implementation - subclasses should override
        String::new()
    }

    fn is_native_compatible(&self, data: &serde_json::Value) -> bool {
        data.is_object()
    }

    fn get_native_type(&self) -> DataType {
        DataType::Dict
    }
}

impl IObject for XWObject {
    fn created_at(&self) -> String {
        self._created_at.clone()
    }

    fn updated_at(&self) -> String {
        self._updated_at.clone()
    }

    fn title(&self) -> Option<String> {
        self._title.clone()
    }

    fn description(&self) -> Option<String> {
        self._description.clone()
    }

    fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut dict = HashMap::new();
        dict.insert("id".to_string(), serde_json::Value::String(self._id.clone()));
        dict.insert("uid".to_string(), serde_json::Value::String(self._uid.clone()));
        dict.insert("created_at".to_string(), serde_json::Value::String(self._created_at.clone()));
        dict.insert("updated_at".to_string(), serde_json::Value::String(self._updated_at.clone()));
        if let Some(ref title) = self._title {
            dict.insert("title".to_string(), serde_json::Value::String(title.clone()));
        }
        if let Some(ref desc) = self._description {
            dict.insert("description".to_string(), serde_json::Value::String(desc.clone()));
        }
        dict
    }

    fn save(&self) {
        // Abstract - subclasses must implement
    }

    fn load(&mut self) {
        // Abstract - subclasses must implement
    }
}

impl AObject for XWObject {
    fn id(&self) -> &str {
        &self._id
    }

    fn uid(&self) -> &str {
        &self._uid
    }

    fn created_at(&self) -> String {
        self._created_at.clone()
    }

    fn updated_at(&self) -> String {
        self._updated_at.clone()
    }

    fn title(&self) -> Option<String> {
        self._title.clone()
    }

    fn description(&self) -> Option<String> {
        self._description.clone()
    }

    fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut dict = HashMap::new();
        dict.insert("id".to_string(), serde_json::Value::String(self._id.clone()));
        dict.insert("uid".to_string(), serde_json::Value::String(self._uid.clone()));
        dict.insert("created_at".to_string(), serde_json::Value::String(self._created_at.clone()));
        dict.insert("updated_at".to_string(), serde_json::Value::String(self._updated_at.clone()));
        if let Some(ref title) = self._title {
            dict.insert("title".to_string(), serde_json::Value::String(title.clone()));
        }
        if let Some(ref desc) = self._description {
            dict.insert("description".to_string(), serde_json::Value::String(desc.clone()));
        }
        dict
    }

    fn to_native(&self) -> serde_json::Value {
        serde_json::to_value(self.to_dict()).unwrap_or(serde_json::Value::Null)
    }

    fn from_native(&mut self, _data: serde_json::Value) {
        // Default does nothing - subclasses must implement
    }

    fn is_native_compatible(&self, data: &serde_json::Value) -> bool {
        data.is_object()
    }

    fn save(&self) {
        // Abstract - subclasses must implement
    }

    fn load(&mut self) {
        // Abstract - subclasses must implement
    }

    fn generate_id(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }

    fn validate_id(&self, id_value: &str) -> bool {
        !id_value.is_empty()
    }

    fn is_same_id(&self, other: &dyn IObject) -> bool {
        self.id() == other.id()
    }
}

impl XWObject {
    /// Initialize XWObject base class.
    pub fn new(object_id: Option<String>) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        
        Self {
            _id: object_id.unwrap_or_else(|| generate_uuid()),
            _uid: generate_uuid(),
            _created_at: now.clone(),
            _updated_at: now,
            _title: None,
            _description: None,
        }
    }

    /// Update the updated_at timestamp.
    pub fn _update_timestamp(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        self._updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
    }
}
