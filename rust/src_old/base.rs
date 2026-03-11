// #exonware/xwsystem/rust/src_old/base.rs
//! Base classes and abstract implementations for XWSystem.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module provides base implementations of core functionality.
//! In Rust, abstract base classes are represented as traits with default implementations
//! or as structs that implement traits.
//!
//! This module mirrors Python's base.py 100% including:
//! - AXWSystemBase (root abstract base class)
//! - ASystemComponent (system component base)
//! - AConfigurableComponent (configurable component base)
//! - AMonitoredComponent (monitored component base)
//! - ASecureComponent (secure component base)

use crate::contracts::{ICore, IObject, IId, INative, ISystemComponent, IConfigurable};
use crate::defs::{CoreMode, CorePriority, CoreState, DataType};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// ROOT ABSTRACT BASE CLASSES (matching Python base.py)
// ============================================================================

/// Abstract base trait for XWSystem framework.
///
/// Matches Python's AXWSystemBase abstract base class.
/// Provides common functionality for all XWSystem components.
pub trait AXWSystemBase {
    /// Get system mode.
    fn mode(&self) -> CoreMode;

    /// Get system state.
    fn state(&self) -> CoreState;

    /// Initialize the component.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown the component.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if component is initialized.
    fn is_initialized(&self) -> bool;

    /// Check if component is shutdown.
    fn is_shutdown(&self) -> bool;
}

/// Base implementation of AXWSystemBase.
///
/// Matches Python's AXWSystemBase with default implementation.
pub struct BaseXWSystem {
    mode: CoreMode,
    state: CoreState,
    initialized: bool,
    shutdown: bool,
}

impl BaseXWSystem {
    /// Create a new BaseXWSystem with the given mode.
    pub fn new(mode: CoreMode) -> Self {
        BaseXWSystem {
            mode,
            state: CoreState::Uninitialized,
            initialized: false,
            shutdown: false,
        }
    }
}

impl AXWSystemBase for BaseXWSystem {
    fn mode(&self) -> CoreMode {
        self.mode
    }

    fn state(&self) -> CoreState {
        self.state
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CoreState::Initializing;
        self.initialized = true;
        self.state = CoreState::Initialized;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CoreState::ShuttingDown;
        self.shutdown = true;
        self.state = CoreState::Shutdown;
        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn is_shutdown(&self) -> bool {
        self.shutdown
    }
}

/// Abstract base trait for system components.
///
/// Matches Python's ASystemComponent abstract base class.
/// All XWSystem components should implement this trait.
pub trait ASystemComponent: AXWSystemBase + ISystemComponent {
    /// Get component name.
    fn get_name(&self) -> &str;

    /// Get component version.
    fn get_version(&self) -> &str;

    /// Get component configuration.
    fn get_config(&self) -> HashMap<String, Value>;

    /// Set component configuration.
    fn set_config(&mut self, config: HashMap<String, Value>);
}

/// Base implementation of ASystemComponent.
///
/// Matches Python's ASystemComponent with default implementation.
pub struct BaseSystemComponent {
    base: BaseXWSystem,
    name: String,
    version: String,
    config: HashMap<String, Value>,
}

impl BaseSystemComponent {
    /// Create a new BaseSystemComponent.
    pub fn new(name: String, mode: CoreMode) -> Self {
        BaseSystemComponent {
            base: BaseXWSystem::new(mode),
            name,
            version: "0.1.0.1".to_string(),
            config: HashMap::new(),
        }
    }
}

impl AXWSystemBase for BaseSystemComponent {
    fn mode(&self) -> CoreMode {
        self.base.mode()
    }

    fn state(&self) -> CoreState {
        self.base.state()
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::initialize(&mut self.base)
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::shutdown(&mut self.base)
    }

    fn is_initialized(&self) -> bool {
        AXWSystemBase::is_initialized(&self.base)
    }

    fn is_shutdown(&self) -> bool {
        AXWSystemBase::is_shutdown(&self.base)
    }
}

impl ASystemComponent for BaseSystemComponent {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_version(&self) -> &str {
        &self.version
    }

    fn get_config(&self) -> HashMap<String, Value> {
        self.config.clone()
    }

    fn set_config(&mut self, config: HashMap<String, Value>) {
        self.config = config;
    }
}

impl ISystemComponent for BaseSystemComponent {
    fn get_name(&self) -> &str {
        ASystemComponent::get_name(self)
    }

    fn get_version(&self) -> &str {
        ASystemComponent::get_version(self)
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::initialize(self)
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::shutdown(self)
    }

    fn is_initialized(&self) -> bool {
        AXWSystemBase::is_initialized(self)
    }

    fn get_config(&self) -> HashMap<String, Value> {
        ASystemComponent::get_config(self)
    }

    fn set_config(&mut self, config: HashMap<String, Value>) {
        ASystemComponent::set_config(self, config)
    }
}

/// Abstract base trait for configurable system components.
///
/// Matches Python's AConfigurableComponent abstract base class.
/// Combines ASystemComponent with IConfigurable interface.
pub trait AConfigurableComponent: ASystemComponent + IConfigurable {
    /// Configure component with options.
    fn configure(&mut self, options: HashMap<String, Value>);

    /// Reset configuration to defaults.
    fn reset_config(&mut self);

    /// Update single configuration value.
    fn update_config(&mut self, key: &str, value: Value);

    /// Check if configuration key exists.
    fn has_config(&self, key: &str) -> bool;

    /// Remove configuration key.
    fn remove_config(&mut self, key: &str) -> bool;

    /// Merge configuration with existing.
    fn merge_config(&mut self, config: HashMap<String, Value>, _priority: Option<Value>);
}

/// Base implementation of AConfigurableComponent.
///
/// Matches Python's AConfigurableComponent with default implementation.
pub struct BaseConfigurableComponent {
    component: BaseSystemComponent,
    config_dict: HashMap<String, Value>,
}

impl BaseConfigurableComponent {
    /// Create a new BaseConfigurableComponent.
    pub fn new(name: String, mode: CoreMode) -> Self {
        BaseConfigurableComponent {
            component: BaseSystemComponent::new(name, mode),
            config_dict: HashMap::new(),
        }
    }
}

impl AXWSystemBase for BaseConfigurableComponent {
    fn mode(&self) -> CoreMode {
        AXWSystemBase::mode(&self.component)
    }

    fn state(&self) -> CoreState {
        AXWSystemBase::state(&self.component)
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::initialize(&mut self.component)
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::shutdown(&mut self.component)
    }

    fn is_initialized(&self) -> bool {
        AXWSystemBase::is_initialized(&self.component)
    }

    fn is_shutdown(&self) -> bool {
        AXWSystemBase::is_shutdown(&self.component)
    }
}

impl ASystemComponent for BaseConfigurableComponent {
    fn get_name(&self) -> &str {
        ASystemComponent::get_name(&self.component)
    }

    fn get_version(&self) -> &str {
        ASystemComponent::get_version(&self.component)
    }

    fn get_config(&self) -> HashMap<String, Value> {
        self.config_dict.clone()
    }

    fn set_config(&mut self, config: HashMap<String, Value>) {
        ASystemComponent::set_config(&mut self.component, config.clone());
        self.config_dict = config;
    }
}

impl ISystemComponent for BaseConfigurableComponent {
    fn get_name(&self) -> &str {
        ASystemComponent::get_name(self)
    }

    fn get_version(&self) -> &str {
        ASystemComponent::get_version(self)
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::initialize(self)
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        AXWSystemBase::shutdown(self)
    }

    fn is_initialized(&self) -> bool {
        AXWSystemBase::is_initialized(self)
    }

    fn get_config(&self) -> HashMap<String, Value> {
        ASystemComponent::get_config(self)
    }

    fn set_config(&mut self, config: HashMap<String, Value>) {
        ASystemComponent::set_config(self, config)
    }
}

impl IConfigurable for BaseConfigurableComponent {
    fn configure(&mut self, options: HashMap<String, Value>) {
        self.config_dict.extend(options.clone());
        ASystemComponent::set_config(&mut self.component, options);
    }

    fn get_config(&self) -> HashMap<String, Value> {
        self.config_dict.clone()
    }

    fn reset_config(&mut self) {
        self.config_dict.clear();
        ASystemComponent::set_config(&mut self.component, HashMap::new());
    }

    fn update_config(&mut self, key: &str, value: Value) {
        self.config_dict.insert(key.to_string(), value.clone());
        let mut config = ASystemComponent::get_config(&self.component);
        config.insert(key.to_string(), value);
        ASystemComponent::set_config(&mut self.component, config);
    }

    fn has_config(&self, key: &str) -> bool {
        self.config_dict.contains_key(key)
    }

    fn remove_config(&mut self, key: &str) -> bool {
        let removed = self.config_dict.remove(key).is_some();
        if removed {
            let mut config = ASystemComponent::get_config(&self.component);
            config.remove(key);
            ASystemComponent::set_config(&mut self.component, config);
        }
        removed
    }

    fn merge_config(&mut self, config: HashMap<String, Value>, _priority: Option<Value>) {
        self.config_dict.extend(config.clone());
        ASystemComponent::set_config(&mut self.component, config);
    }
}

impl AConfigurableComponent for BaseConfigurableComponent {
    fn configure(&mut self, options: HashMap<String, Value>) {
        IConfigurable::configure(self, options)
    }

    fn reset_config(&mut self) {
        IConfigurable::reset_config(self)
    }

    fn update_config(&mut self, key: &str, value: Value) {
        IConfigurable::update_config(self, key, value)
    }

    fn has_config(&self, key: &str) -> bool {
        IConfigurable::has_config(self, key)
    }

    fn remove_config(&mut self, key: &str) -> bool {
        IConfigurable::remove_config(self, key)
    }

    fn merge_config(&mut self, config: HashMap<String, Value>, priority: Option<Value>) {
        IConfigurable::merge_config(self, config, priority)
    }
}

/// Abstract base trait for monitored system components.
///
/// Matches Python's AMonitoredComponent abstract base class.
/// Provides monitoring capabilities for system components.
pub trait AMonitoredComponent: ASystemComponent {
    /// Start monitoring.
    fn start_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Stop monitoring.
    fn stop_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Get component metrics.
    fn get_metrics(&self) -> HashMap<String, Value>;
}

/// Abstract base trait for secure system components.
///
/// Matches Python's ASecureComponent abstract base class.
/// Provides security capabilities for system components.
pub trait ASecureComponent: ASystemComponent {
    /// Validate security of data.
    fn validate_security(&self, data: &Value) -> bool;

    /// Check if action is permitted on resource.
    fn check_permissions(&self, action: &str, resource: &str) -> bool;
}

// ============================================================================
// ABSTRACT BASE TRAITS (matching Python ACoreBase, AResourceManagerBase)
// ============================================================================

/// Abstract base trait for core functionality.
/// 
/// Matches Python's ACoreBase abstract base class.
pub trait ACoreBase {
    /// Initialize core functionality.
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Shutdown core functionality.
    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>>;

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
    fn remove_dependency(&mut self, dependency: &str);

    /// Get all dependencies.
    fn get_dependencies(&self) -> Vec<String>;

    /// Check if all dependencies are satisfied.
    fn check_dependencies(&self) -> bool;
}

/// Abstract base trait for resource management.
///
/// Matches Python's AResourceManagerBase abstract base class.
pub trait AResourceManagerBase {
    /// Acquire resource.
    fn acquire_resource(
        &mut self,
        resource_id: &str,
        priority: CorePriority,
    ) -> Result<Option<&serde_json::Value>, String>;

    /// Release resource.
    fn release_resource(&mut self, resource_id: &str) -> Result<(), String>;

    /// Check if resource is available.
    fn is_resource_available(&self, resource_id: &str) -> bool;

    /// Get number of managed resources.
    fn get_resource_count(&self) -> usize;

    /// List all resource IDs.
    fn list_resources(&self) -> Vec<String>;

    /// Cleanup all resources.
    fn cleanup_resources(&mut self);
}

/// Base implementation of core functionality.
///
/// Provides a concrete implementation of the ICore trait.
pub struct BaseCore {
    mode: CoreMode,
    state: CoreState,
    initialized: bool,
    shutdown: bool,
    dependencies: Vec<String>,
    resources: HashMap<String, serde_json::Value>,
}

impl BaseCore {
    /// Create a new BaseCore with the given mode.
    pub fn new(mode: CoreMode) -> Self {
        BaseCore {
            mode,
            state: CoreState::Uninitialized,
            initialized: false,
            shutdown: false,
            dependencies: Vec::new(),
            resources: HashMap::new(),
        }
    }

    /// Set the core state.
    pub fn set_state(&mut self, state: CoreState) {
        self.state = state;
    }

    /// Add a dependency.
    pub fn add_dependency(&mut self, dependency: String) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    /// Remove a dependency.
    pub fn remove_dependency(&mut self, dependency: &str) {
        self.dependencies.retain(|d| d != dependency);
    }
}

impl ACoreBase for BaseCore {
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CoreState::Initializing;
        self.initialized = true;
        self.state = CoreState::Initialized;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CoreState::ShuttingDown;
        self.shutdown = true;
        self.state = CoreState::Shutdown;
        Ok(())
    }

    fn get_state(&self) -> CoreState {
        self.state
    }

    fn set_state(&mut self, state: CoreState) {
        self.state = state;
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    fn add_dependency(&mut self, dependency: String) {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
    }

    fn remove_dependency(&mut self, dependency: &str) {
        self.dependencies.retain(|d| d != dependency);
    }

    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn check_dependencies(&self) -> bool {
        true // Basic implementation
    }
}

impl ICore for BaseCore {
    fn mode(&self) -> CoreMode {
        self.mode
    }

    fn state(&self) -> CoreState {
        self.state
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CoreState::Initializing;
        self.initialized = true;
        self.state = CoreState::Initialized;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.state = CoreState::ShuttingDown;
        self.shutdown = true;
        self.state = CoreState::Shutdown;
        Ok(())
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }

    fn check_dependencies(&self) -> bool {
        // Basic implementation - can be overridden
        true
    }
}

/// Resource manager base implementation.
///
/// Provides resource management functionality.
pub struct BaseResourceManager {
    max_resources: usize,
    resources: HashMap<String, serde_json::Value>,
    resource_locks: HashMap<String, bool>,
}

impl AResourceManagerBase for BaseResourceManager {
    fn acquire_resource(
        &mut self,
        resource_id: &str,
        _priority: CorePriority,
    ) -> Result<Option<&serde_json::Value>, String> {
        if self.resources.len() >= self.max_resources {
            return Err("Maximum resources reached".to_string());
        }

        if self.resource_locks.get(resource_id).copied().unwrap_or(false) {
            return Err("Resource is locked".to_string());
        }

        self.resource_locks.insert(resource_id.to_string(), true);
        Ok(self.resources.get(resource_id))
    }

    fn release_resource(&mut self, resource_id: &str) -> Result<(), String> {
        self.resource_locks.insert(resource_id.to_string(), false);
        Ok(())
    }

    fn is_resource_available(&self, resource_id: &str) -> bool {
        !self.resource_locks.get(resource_id).copied().unwrap_or(false)
    }

    fn get_resource_count(&self) -> usize {
        self.resources.len()
    }

    fn list_resources(&self) -> Vec<String> {
        self.resources.keys().cloned().collect()
    }

    fn cleanup_resources(&mut self) {
        self.resources.clear();
        self.resource_locks.clear();
    }
}

impl BaseResourceManager {
    /// Create a new BaseResourceManager.
    pub fn new(max_resources: usize) -> Self {
        BaseResourceManager {
            max_resources,
            resources: HashMap::new(),
            resource_locks: HashMap::new(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_core_initialization() {
        use super::ACoreBase;
        let mut core = BaseCore::new(CoreMode::Development);
        assert!(!ACoreBase::is_initialized(&core));
        assert!(!ACoreBase::is_shutdown(&core));
        assert_eq!(ACoreBase::get_state(&core), CoreState::Uninitialized);

        ACoreBase::initialize(&mut core).unwrap();
        assert!(ACoreBase::is_initialized(&core));
        assert_eq!(ACoreBase::get_state(&core), CoreState::Initialized);

        ACoreBase::shutdown(&mut core).unwrap();
        assert!(ACoreBase::is_shutdown(&core));
        assert_eq!(ACoreBase::get_state(&core), CoreState::Shutdown);
    }

    #[test]
    fn test_base_core_dependencies() {
        use super::ACoreBase;
        let mut core = BaseCore::new(CoreMode::Development);
        ACoreBase::add_dependency(&mut core, "test_dep".to_string());
        assert_eq!(ACoreBase::get_dependencies(&core).len(), 1);
        assert!(ACoreBase::get_dependencies(&core).contains(&"test_dep".to_string()));

        ACoreBase::remove_dependency(&mut core, "test_dep");
        assert_eq!(ACoreBase::get_dependencies(&core).len(), 0);
    }

    #[test]
    fn test_resource_manager() {
        let manager = BaseResourceManager::new(10);
        assert_eq!(manager.get_resource_count(), 0);
        assert!(manager.is_resource_available("test"));
    }
}

// ============================================================================
// CONFIGURATION BASE
// ============================================================================

/// Abstract base trait for core configuration.
pub trait AConfigurationBase {
    /// Load configuration data.
    fn load_config(&mut self, config_data: &HashMap<String, Value>) -> Result<(), String>;

    /// Get configuration value.
    fn get_config_value<'a>(&'a self, key: &str, default: Option<&'a Value>) -> Option<&'a Value>;

    /// Set configuration value.
    fn set_config_value(&mut self, key: &str, value: Value);

    /// Validate configuration.
    fn validate_config(&self) -> bool;

    /// Reset configuration to defaults.
    fn reset_to_defaults(&mut self);

    /// Export configuration as dictionary.
    fn export_config(&self) -> HashMap<String, Value>;
}

/// Base implementation of configuration.
pub struct BaseConfiguration {
    config: HashMap<String, Value>,
    defaults: HashMap<String, Value>,
}

impl BaseConfiguration {
    pub fn new() -> Self {
        BaseConfiguration {
            config: HashMap::new(),
            defaults: HashMap::new(),
        }
    }
}

impl AConfigurationBase for BaseConfiguration {
    fn load_config(&mut self, config_data: &HashMap<String, Value>) -> Result<(), String> {
        self.config = config_data.clone();
        Ok(())
    }

    fn get_config_value<'a>(&'a self, key: &str, default: Option<&'a Value>) -> Option<&'a Value> {
        self.config.get(key).or(default).or_else(|| self.defaults.get(key))
    }

    fn set_config_value(&mut self, key: &str, value: Value) {
        self.config.insert(key.to_string(), value);
    }

    fn validate_config(&self) -> bool {
        true // Basic implementation
    }

    fn reset_to_defaults(&mut self) {
        self.config = self.defaults.clone();
    }

    fn export_config(&self) -> HashMap<String, Value> {
        self.config.clone()
    }
}

// ============================================================================
// VALIDATION BASE
// ============================================================================

/// Abstract base trait for core validation.
pub trait AValidationBase {
    /// Validate input data.
    fn validate_input(&self, data: &Value) -> bool;

    /// Validate output data.
    fn validate_output(&self, data: &Value) -> bool;

    /// Validate operation.
    fn validate_operation(&self, operation: &str, args: &HashMap<String, Value>) -> bool;

    /// Get validation errors.
    fn get_validation_errors(&self) -> Vec<String>;

    /// Clear validation errors.
    fn clear_validation_errors(&mut self);
}

/// Base implementation of validation.
pub struct BaseValidation {
    errors: Vec<String>,
}

impl BaseValidation {
    pub fn new() -> Self {
        BaseValidation {
            errors: Vec::new(),
        }
    }
}

impl AValidationBase for BaseValidation {
    fn validate_input(&self, _data: &Value) -> bool {
        true // Basic implementation
    }

    fn validate_output(&self, _data: &Value) -> bool {
        true // Basic implementation
    }

    fn validate_operation(&self, _operation: &str, _args: &HashMap<String, Value>) -> bool {
        true // Basic implementation
    }

    fn get_validation_errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn clear_validation_errors(&mut self) {
        self.errors.clear();
    }
}

// ============================================================================
// OPERATION BASE
// ============================================================================

/// Abstract base trait for core operations.
pub trait AOperationBase {
    /// Execute operation.
    fn execute(&mut self, args: &HashMap<String, Value>) -> Result<Value, String>;

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
    fn get_result(&self) -> Option<&Value>;

    /// Get operation error.
    fn get_error(&self) -> Option<&String>;
}

/// Base implementation of operation.
pub struct BaseOperation {
    timeout: Option<u64>,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    running: bool,
    completed: bool,
    failed: bool,
    result: Option<Value>,
    error: Option<String>,
}

impl BaseOperation {
    pub fn new(timeout: Option<u64>) -> Self {
        BaseOperation {
            timeout,
            start_time: None,
            end_time: None,
            running: false,
            completed: false,
            failed: false,
            result: None,
            error: None,
        }
    }
}

impl AOperationBase for BaseOperation {
    fn execute(&mut self, _args: &HashMap<String, Value>) -> Result<Value, String> {
        self.running = true;
        self.start_time = Some(Utc::now());
        // Basic implementation - subclasses should override
        self.running = false;
        self.completed = true;
        self.end_time = Some(Utc::now());
        Ok(Value::Null)
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn is_completed(&self) -> bool {
        self.completed
    }

    fn is_failed(&self) -> bool {
        self.failed
    }

    fn get_duration(&self) -> Option<f64> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => {
                let duration = end.signed_duration_since(start);
                Some(duration.num_milliseconds() as f64 / 1000.0)
            }
            _ => None,
        }
    }

    fn cancel(&mut self) {
        self.running = false;
        self.failed = true;
        self.end_time = Some(Utc::now());
    }

    fn get_result(&self) -> Option<&Value> {
        self.result.as_ref()
    }

    fn get_error(&self) -> Option<&String> {
        self.error.as_ref()
    }
}

// ============================================================================
// OBJECT BASE CLASSES
// ============================================================================

/// Abstract base trait for all objects in the eXonware ecosystem.
///
/// Matches Python's AObject abstract base class.
/// Extends IObject interface with default implementations.
pub trait AObject: IObject {
    /// Generate a new ID.
    ///
    /// Default implementation generates a UUID.
    fn generate_id(&mut self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Validate an ID format.
    ///
    /// Default implementation checks if ID is a non-empty string.
    fn validate_id(&self, id_value: &str) -> bool {
        !id_value.is_empty()
    }

    /// Check if this object has the same ID as another.
    fn is_same_id(&self, other: &dyn IObject) -> bool {
        self.id() == other.id()
    }
}

/// Abstract base struct for all objects in the eXonware ecosystem.
///
/// In Rust, we use a struct with trait implementations rather than abstract classes.
/// This struct provides the base fields that concrete objects will use.
pub struct AObjectBase {
    pub(crate) id: String,
    pub(crate) uid: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
}

impl AObjectBase {
    /// Create a new AObjectBase with generated ID and timestamps.
    pub fn new(object_id: Option<String>) -> Self {
        let id = object_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let now = Utc::now();
        AObjectBase {
            id: id.clone(),
            uid: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            title: None,
            description: None,
        }
    }

    /// Update the updated_at timestamp.
    pub fn update_timestamp(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl IId for AObjectBase {
    fn id(&self) -> &str {
        &self.id
    }

    fn uid(&self) -> &str {
        &self.uid
    }

    fn generate_id(&mut self) -> String {
        let new_id = Uuid::new_v4().to_string();
        self.id = new_id.clone();
        new_id
    }

    fn validate_id(&self, id_value: &str) -> bool {
        !id_value.is_empty()
    }

    fn is_same_id(&self, other: &dyn IId) -> bool {
        self.id() == other.id()
    }
}

impl INative for AObjectBase {
    fn to_native(&self) -> Value {
        serde_json::json!({
            "id": self.id,
            "uid": self.uid,
            "created_at": self.created_at.to_rfc3339(),
            "updated_at": self.updated_at.to_rfc3339(),
            "title": self.title,
            "description": self.description,
        })
    }

    fn from_native(&mut self, data: &Value) -> Result<(), String> {
        if let Some(obj) = data.as_object() {
            if let Some(id) = obj.get("id").and_then(|v| v.as_str()) {
                self.id = id.to_string();
            }
            if let Some(uid) = obj.get("uid").and_then(|v| v.as_str()) {
                self.uid = uid.to_string();
            }
            if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                self.title = Some(title.to_string());
            }
            if let Some(desc) = obj.get("description").and_then(|v| v.as_str()) {
                self.description = Some(desc.to_string());
            }
            Ok(())
        } else {
            Err("Data is not an object".to_string())
        }
    }

    fn is_native_compatible(&self, data: &Value) -> bool {
        data.is_object()
    }

    fn get_native_type(&self) -> DataType {
        DataType::Dict
    }
}

/// Concrete base class for all objects in the eXonware ecosystem.
///
/// This is the Rust equivalent of XWObject from Python.
pub struct XWObject {
    base: AObjectBase,
}

impl XWObject {
    /// Create a new XWObject.
    pub fn new(object_id: Option<String>) -> Self {
        XWObject {
            base: AObjectBase::new(object_id),
        }
    }

    /// Get the object ID.
    pub fn id(&self) -> &str {
        self.base.id()
    }

    /// Get the object UID.
    pub fn uid(&self) -> &str {
        self.base.uid()
    }

    /// Get the creation timestamp.
    pub fn created_at(&self) -> DateTime<Utc> {
        self.base.created_at
    }

    /// Get the last update timestamp.
    pub fn updated_at(&self) -> DateTime<Utc> {
        self.base.updated_at
    }

    /// Get the object title.
    pub fn title(&self) -> Option<&str> {
        self.base.title.as_deref()
    }

    /// Set the object title.
    pub fn set_title(&mut self, title: Option<String>) {
        self.base.title = title;
        self.base.update_timestamp();
    }

    /// Get the object description.
    pub fn description(&self) -> Option<&str> {
        self.base.description.as_deref()
    }

    /// Set the object description.
    pub fn set_description(&mut self, description: Option<String>) {
        self.base.description = description;
        self.base.update_timestamp();
    }

    /// Export object as dictionary.
    pub fn to_dict(&self) -> Value {
        self.base.to_native()
    }

    /// Update the updated_at timestamp.
    pub fn update_timestamp(&mut self) {
        self.base.update_timestamp();
    }
}

impl IId for XWObject {
    fn id(&self) -> &str {
        self.base.id()
    }

    fn uid(&self) -> &str {
        self.base.uid()
    }

    fn generate_id(&mut self) -> String {
        self.base.generate_id()
    }

    fn validate_id(&self, id_value: &str) -> bool {
        self.base.validate_id(id_value)
    }

    fn is_same_id(&self, other: &dyn IId) -> bool {
        self.base.is_same_id(other)
    }
}

impl INative for XWObject {
    fn to_native(&self) -> Value {
        self.base.to_native()
    }

    fn from_native(&mut self, data: &Value) -> Result<(), String> {
        self.base.from_native(data)
    }

    fn is_native_compatible(&self, data: &Value) -> bool {
        self.base.is_native_compatible(data)
    }

    fn get_native_type(&self) -> DataType {
        self.base.get_native_type()
    }
}

impl AObject for XWObject {}

impl IObject for XWObject {
    fn created_at(&self) -> DateTime<Utc> {
        self.base.created_at
    }

    fn updated_at(&self) -> DateTime<Utc> {
        self.base.updated_at
    }

    fn title(&self) -> Option<&str> {
        self.base.title.as_deref()
    }

    fn description(&self) -> Option<&str> {
        self.base.description.as_deref()
    }

    fn to_dict(&self) -> Value {
        self.base.to_native()
    }

    fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Abstract - subclasses must implement
        Err("save() must be implemented by subclasses".into())
    }

    fn load(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Abstract - subclasses must implement
        Err("load() must be implemented by subclasses".into())
    }
}

#[cfg(test)]
mod object_tests {
    use super::*;

    #[test]
    fn test_xw_object_creation() {
        let obj = XWObject::new(None);
        assert!(!obj.id().is_empty());
        assert!(!obj.uid().is_empty());
    }

    #[test]
    fn test_xw_object_with_id() {
        let obj = XWObject::new(Some("test_id".to_string()));
        assert_eq!(obj.id(), "test_id");
    }

    #[test]
    fn test_xw_object_title_description() {
        let mut obj = XWObject::new(None);
        obj.set_title(Some("Test Title".to_string()));
        obj.set_description(Some("Test Description".to_string()));
        assert_eq!(obj.title(), Some("Test Title"));
        assert_eq!(obj.description(), Some("Test Description"));
    }

    #[test]
    fn test_xw_object_to_dict() {
        let mut obj = XWObject::new(Some("test_id".to_string()));
        obj.set_title(Some("Test".to_string()));
        let dict = obj.to_dict();
        assert_eq!(dict["id"], "test_id");
        assert_eq!(dict["title"], "Test");
    }

    #[test]
    fn test_xw_object_from_native() {
        let mut obj = XWObject::new(None);
        let data = serde_json::json!({
            "id": "new_id",
            "uid": "new_uid",
            "title": "New Title"
        });
        obj.from_native(&data).unwrap();
        assert_eq!(obj.id(), "new_id");
        assert_eq!(obj.title(), Some("New Title"));
    }
}

