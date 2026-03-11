// #exonware/xwsystem/rust/tests/shared_tests.rs
//! Comprehensive tests for shared module components.
//!
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! Tests for defs, errors, contracts, base, and shared modules.

use xwsystem_rust::defs::*;
use xwsystem_rust::errors::*;
use xwsystem_rust::base::*;
use xwsystem_rust::shared::*;
use serde_json::json;
use std::collections::HashMap;

// ============================================================================
// DEFS TESTS
// ============================================================================

#[test]
fn test_validation_level_enum() {
    assert_eq!(ValidationLevel::None.to_string(), "none");
    assert_eq!(ValidationLevel::Basic.to_string(), "basic");
    assert_eq!(ValidationLevel::Strict.to_string(), "strict");
    assert_eq!(ValidationLevel::Paranoid.to_string(), "paranoid");
    assert_eq!(ValidationLevel::Comprehensive.to_string(), "comprehensive");
    
    // Test default
    assert_eq!(ValidationLevel::default(), ValidationLevel::Basic);
}

#[test]
fn test_performance_level_enum() {
    assert_eq!(PerformanceLevel::Low.to_string(), "low");
    assert_eq!(PerformanceLevel::Medium.to_string(), "medium");
    assert_eq!(PerformanceLevel::High.to_string(), "high");
    assert_eq!(PerformanceLevel::Critical.to_string(), "critical");
    assert_eq!(PerformanceLevel::Excellent.to_string(), "excellent");
    assert_eq!(PerformanceLevel::Good.to_string(), "good");
    assert_eq!(PerformanceLevel::Average.to_string(), "average");
    assert_eq!(PerformanceLevel::Poor.to_string(), "poor");
    
    // Test default
    assert_eq!(PerformanceLevel::default(), PerformanceLevel::Medium);
}

#[test]
fn test_auth_type_enum() {
    assert_eq!(AuthType::None.to_string(), "none");
    assert_eq!(AuthType::Basic.to_string(), "basic");
    assert_eq!(AuthType::Bearer.to_string(), "bearer");
    assert_eq!(AuthType::ApiKey.to_string(), "api_key");
    assert_eq!(AuthType::Oauth2.to_string(), "oauth2");
    assert_eq!(AuthType::Jwt.to_string(), "jwt");
    assert_eq!(AuthType::Saml.to_string(), "saml");
    
    // Test default
    assert_eq!(AuthType::default(), AuthType::None);
}

#[test]
fn test_lock_type_enum() {
    assert_eq!(LockType::Shared.to_string(), "shared");
    assert_eq!(LockType::Exclusive.to_string(), "exclusive");
    assert_eq!(LockType::NonBlocking.to_string(), "non_blocking");
    assert_eq!(LockType::Mutex.to_string(), "mutex");
    assert_eq!(LockType::RwLock.to_string(), "rwlock");
    assert_eq!(LockType::Semaphore.to_string(), "semaphore");
    assert_eq!(LockType::Condition.to_string(), "condition");
    assert_eq!(LockType::Event.to_string(), "event");
    assert_eq!(LockType::Barrier.to_string(), "barrier");
    
    // Test default
    assert_eq!(LockType::default(), LockType::Mutex);
}

#[test]
fn test_path_type_enum() {
    assert_eq!(PathType::File.to_string(), "file");
    assert_eq!(PathType::Directory.to_string(), "directory");
    assert_eq!(PathType::Symlink.to_string(), "symlink");
    assert_eq!(PathType::Absolute.to_string(), "absolute");
    assert_eq!(PathType::Relative.to_string(), "relative");
    
    // Test default
    assert_eq!(PathType::default(), PathType::File);
}

#[test]
fn test_log_level_enum() {
    assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
    assert_eq!(LogLevel::Info.to_string(), "INFO");
    assert_eq!(LogLevel::Warning.to_string(), "WARNING");
    assert_eq!(LogLevel::Error.to_string(), "ERROR");
    assert_eq!(LogLevel::Critical.to_string(), "CRITICAL");
    
    // Test default
    assert_eq!(LogLevel::default(), LogLevel::Info);
}

#[test]
fn test_operation_result_enum() {
    assert_eq!(OperationResult::Success.to_string(), "success");
    assert_eq!(OperationResult::Failed.to_string(), "failed");
    assert_eq!(OperationResult::Partial.to_string(), "partial");
    assert_eq!(OperationResult::Skipped.to_string(), "skipped");
    assert_eq!(OperationResult::Timeout.to_string(), "timeout");
    assert_eq!(OperationResult::Error.to_string(), "error");
    assert_eq!(OperationResult::Warning.to_string(), "warning");
    
    // Test default
    assert_eq!(OperationResult::default(), OperationResult::Success);
}

#[test]
fn test_data_type_enum() {
    assert_eq!(DataType::String.to_string(), "string");
    assert_eq!(DataType::Integer.to_string(), "integer");
    assert_eq!(DataType::Float.to_string(), "float");
    assert_eq!(DataType::Boolean.to_string(), "boolean");
    assert_eq!(DataType::List.to_string(), "list");
    assert_eq!(DataType::Dict.to_string(), "dict");
    assert_eq!(DataType::Bytes.to_string(), "bytes");
    assert_eq!(DataType::None.to_string(), "none");
    assert_eq!(DataType::Custom.to_string(), "custom");
    
    // Test default
    assert_eq!(DataType::default(), DataType::String);
}

#[test]
fn test_clone_mode_enum() {
    assert_eq!(CloneMode::Shallow.to_string(), "shallow");
    assert_eq!(CloneMode::Deep.to_string(), "deep");
    assert_eq!(CloneMode::Reference.to_string(), "reference");
    
    // Test default
    assert_eq!(CloneMode::default(), CloneMode::Deep);
}

#[test]
fn test_core_state_enum() {
    assert_eq!(CoreState::Uninitialized.to_string(), "uninitialized");
    assert_eq!(CoreState::Initializing.to_string(), "initializing");
    assert_eq!(CoreState::Initialized.to_string(), "initialized");
    assert_eq!(CoreState::Starting.to_string(), "starting");
    assert_eq!(CoreState::Running.to_string(), "running");
    assert_eq!(CoreState::Stopping.to_string(), "stopping");
    assert_eq!(CoreState::Stopped.to_string(), "stopped");
    assert_eq!(CoreState::ShuttingDown.to_string(), "shutting_down");
    assert_eq!(CoreState::Shutdown.to_string(), "shutdown");
    assert_eq!(CoreState::Error.to_string(), "error");
    
    // Test default
    assert_eq!(CoreState::default(), CoreState::Uninitialized);
}

#[test]
fn test_core_mode_enum() {
    assert_eq!(CoreMode::Development.to_string(), "development");
    assert_eq!(CoreMode::Testing.to_string(), "testing");
    assert_eq!(CoreMode::Staging.to_string(), "staging");
    assert_eq!(CoreMode::Production.to_string(), "production");
    assert_eq!(CoreMode::Debug.to_string(), "debug");
    assert_eq!(CoreMode::Performance.to_string(), "performance");
    
    // Test default
    assert_eq!(CoreMode::default(), CoreMode::Development);
}

#[test]
fn test_core_priority_enum() {
    assert_eq!(CorePriority::Low.to_string(), "low");
    assert_eq!(CorePriority::Normal.to_string(), "normal");
    assert_eq!(CorePriority::High.to_string(), "high");
    assert_eq!(CorePriority::Critical.to_string(), "critical");
    
    // Test default
    assert_eq!(CorePriority::default(), CorePriority::Normal);
}

#[test]
fn test_enum_serialization() {
    // Test ValidationLevel serialization
    let level = ValidationLevel::Strict;
    let json = serde_json::to_string(&level).unwrap();
    assert_eq!(json, "\"strict\"");
    let deserialized: ValidationLevel = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, ValidationLevel::Strict);
    
    // Test CoreState serialization
    let state = CoreState::Running;
    let json = serde_json::to_string(&state).unwrap();
    assert_eq!(json, "\"running\"");
    let deserialized: CoreState = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, CoreState::Running);
}

// ============================================================================
// ERRORS TESTS
// ============================================================================

#[test]
fn test_core_error_creation() {
    let error = CoreError::new("Test error message");
    assert_eq!(error.message(), "Test error message");
    assert!(error.source_error().is_none());
}

#[test]
fn test_core_error_with_source() {
    let source = Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
    let error = CoreError::with_source("Test error", source);
    assert_eq!(error.message(), "Test error");
    assert!(error.source_error().is_some());
}

#[test]
fn test_core_initialization_error() {
    let error = CoreInitializationError::new("Failed to initialize");
    assert_eq!(error.message(), "Failed to initialize");
}

#[test]
fn test_core_shutdown_error() {
    let error = CoreShutdownError::new("Failed to shutdown");
    assert_eq!(error.message(), "Failed to shutdown");
}

#[test]
fn test_core_state_error() {
    let error = CoreStateError::new("Invalid state");
    assert_eq!(error.message(), "Invalid state");
}

#[test]
fn test_core_dependency_error() {
    let error = CoreDependencyError::new("Missing dependency");
    assert_eq!(error.message(), "Missing dependency");
}

#[test]
fn test_core_configuration_error() {
    let error = CoreConfigurationError::new("Invalid configuration");
    assert_eq!(error.message(), "Invalid configuration");
}

#[test]
fn test_core_resource_error() {
    let error = CoreResourceError::new("Resource unavailable");
    assert_eq!(error.message(), "Resource unavailable");
}

#[test]
fn test_core_timeout_error() {
    let error = CoreTimeoutError::new("Operation timed out");
    assert_eq!(error.message(), "Operation timed out");
}

#[test]
fn test_core_permission_error() {
    let error = CorePermissionError::new("Permission denied");
    assert_eq!(error.message(), "Permission denied");
}

#[test]
fn test_core_validation_error() {
    let error = CoreValidationError::new("Validation failed");
    assert_eq!(error.message(), "Validation failed");
}

#[test]
fn test_core_operation_error() {
    let error = CoreOperationError::new("Operation failed");
    assert_eq!(error.message(), "Operation failed");
}

#[test]
fn test_error_display() {
    let error = CoreError::new("Test error");
    let display = format!("{}", error);
    assert!(display.contains("Core error"));
    assert!(display.contains("Test error"));
}

// ============================================================================
// BASE TESTS
// ============================================================================

#[test]
fn test_base_core_initialization() {
    use xwsystem_rust::base::ACoreBase;
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
    use xwsystem_rust::base::ACoreBase;
    let mut core = BaseCore::new(CoreMode::Development);
    ACoreBase::add_dependency(&mut core, "test_dep".to_string());
    assert_eq!(ACoreBase::get_dependencies(&core).len(), 1);
    assert!(ACoreBase::get_dependencies(&core).contains(&"test_dep".to_string()));
    
    ACoreBase::remove_dependency(&mut core, "test_dep");
    assert_eq!(ACoreBase::get_dependencies(&core).len(), 0);
}

#[test]
fn test_base_core_state_transitions() {
    use xwsystem_rust::base::ACoreBase;
    let mut core = BaseCore::new(CoreMode::Development);
    ACoreBase::set_state(&mut core, CoreState::Starting);
    assert_eq!(ACoreBase::get_state(&core), CoreState::Starting);
    
    ACoreBase::set_state(&mut core, CoreState::Running);
    assert_eq!(ACoreBase::get_state(&core), CoreState::Running);
}

#[test]
fn test_base_resource_manager() {
    let mut manager = BaseResourceManager::new(10);
    assert_eq!(manager.get_resource_count(), 0);
    assert!(manager.is_resource_available("test"));
    
    // Test resource acquisition
    let result = manager.acquire_resource("test", CorePriority::Normal);
    assert!(result.is_ok());
    
    // Resource should now be locked
    assert!(!manager.is_resource_available("test"));
    
    // Release resource
    manager.release_resource("test").unwrap();
    assert!(manager.is_resource_available("test"));
}

#[test]
fn test_base_configuration() {
    let mut config = BaseConfiguration::new();
    let mut config_data = std::collections::HashMap::new();
    config_data.insert("key1".to_string(), json!("value1"));
    config_data.insert("key2".to_string(), json!("value2"));
    
    config.load_config(&config_data).unwrap();
    assert_eq!(config.get_config_value("key1", None).unwrap(), &json!("value1"));
    
    config.set_config_value("key3", json!("value3"));
    assert_eq!(config.get_config_value("key3", None).unwrap(), &json!("value3"));
    
    assert!(config.validate_config());
    
    let exported = config.export_config();
    assert_eq!(exported.len(), 3);
}

#[test]
fn test_base_validation() {
    let mut validator = BaseValidation::new();
    assert!(validator.validate_input(&json!({"test": "data"})));
    assert!(validator.validate_output(&json!({"test": "data"})));
    
    let args = std::collections::HashMap::new();
    assert!(validator.validate_operation("test", &args));
    
    assert_eq!(validator.get_validation_errors().len(), 0);
    validator.clear_validation_errors();
}

#[test]
fn test_base_operation() {
    let mut operation = BaseOperation::new(Some(30));
    assert!(!operation.is_running());
    assert!(!operation.is_completed());
    assert!(!operation.is_failed());
    
    let args = std::collections::HashMap::new();
    let result = operation.execute(&args);
    assert!(result.is_ok());
    assert!(operation.is_completed());
    
    assert!(operation.get_duration().is_some());
}

#[test]
fn test_xw_object_creation() {
    let obj = XWObject::new(None);
    assert!(!obj.id().is_empty());
    assert!(!obj.uid().is_empty());
    assert!(obj.title().is_none());
    assert!(obj.description().is_none());
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
    assert!(dict.get("uid").is_some());
    assert!(dict.get("created_at").is_some());
    assert!(dict.get("updated_at").is_some());
}

#[test]
fn test_xw_object_from_native() {
    let mut obj = XWObject::new(None);
    let data = json!({
        "id": "new_id",
        "uid": "new_uid",
        "title": "New Title",
        "description": "New Description"
    });
    obj.from_native(&data).unwrap();
    assert_eq!(obj.id(), "new_id");
    assert_eq!(obj.title(), Some("New Title"));
    assert_eq!(obj.description(), Some("New Description"));
}

#[test]
fn test_xw_object_id_validation() {
    let obj = XWObject::new(None);
    assert!(obj.validate_id("valid_id"));
    assert!(!obj.validate_id(""));
}

#[test]
fn test_xw_object_is_same_id() {
    let obj1 = XWObject::new(Some("same_id".to_string()));
    let obj2 = XWObject::new(Some("same_id".to_string()));
    // Note: In Rust, we'd need to implement a way to compare IDs
    // For now, we test the ID getter
    assert_eq!(obj1.id(), obj2.id());
}

#[test]
fn test_xw_object_timestamp_update() {
    let mut obj = XWObject::new(None);
    let initial_updated = obj.updated_at();
    std::thread::sleep(std::time::Duration::from_millis(10));
    obj.update_timestamp();
    assert!(obj.updated_at() > initial_updated);
}

#[test]
fn test_xw_object_native_compatibility() {
    let obj = XWObject::new(None);
    assert!(obj.is_native_compatible(&json!({"test": "data"})));
    assert!(!obj.is_native_compatible(&json!("not an object")));
    assert_eq!(obj.get_native_type(), DataType::Dict);
}

// ============================================================================
// NEW BASE CLASSES TESTS (AXWSystemBase, BaseSystemComponent, etc.)
// ============================================================================

#[test]
fn test_base_system_component_initialization() {
    use xwsystem_rust::base::{AXWSystemBase, BaseSystemComponent};
    let mut component = BaseSystemComponent::new("test_component".to_string(), CoreMode::Development);
    
    assert!(!AXWSystemBase::is_initialized(&component));
    assert!(!AXWSystemBase::is_shutdown(&component));
    assert_eq!(AXWSystemBase::state(&component), CoreState::Uninitialized);
    
    AXWSystemBase::initialize(&mut component).unwrap();
    assert!(AXWSystemBase::is_initialized(&component));
    assert_eq!(AXWSystemBase::state(&component), CoreState::Initialized);
    
    AXWSystemBase::shutdown(&mut component).unwrap();
    assert!(AXWSystemBase::is_shutdown(&component));
    assert_eq!(AXWSystemBase::state(&component), CoreState::Shutdown);
}

#[test]
fn test_base_system_component_name_version() {
    use xwsystem_rust::base::{ASystemComponent, BaseSystemComponent};
    let component = BaseSystemComponent::new("test_component".to_string(), CoreMode::Development);
    
    assert_eq!(ASystemComponent::get_name(&component), "test_component");
    assert_eq!(ASystemComponent::get_version(&component), "0.1.0.1");
}

#[test]
fn test_base_system_component_config() {
    use xwsystem_rust::base::{ASystemComponent, BaseSystemComponent};
    let mut component = BaseSystemComponent::new("test_component".to_string(), CoreMode::Development);
    
    let mut config = HashMap::new();
    config.insert("key1".to_string(), json!("value1"));
    config.insert("key2".to_string(), json!("value2"));
    
    ASystemComponent::set_config(&mut component, config.clone());
    let retrieved_config = ASystemComponent::get_config(&component);
    assert_eq!(retrieved_config.len(), 2);
    assert_eq!(retrieved_config.get("key1"), Some(&json!("value1")));
}

#[test]
fn test_base_configurable_component() {
    use xwsystem_rust::base::{AConfigurableComponent, BaseConfigurableComponent, IConfigurable};
    let mut component = BaseConfigurableComponent::new("test_component".to_string(), CoreMode::Development);
    
    let mut config = HashMap::new();
    config.insert("key1".to_string(), json!("value1"));
    
    IConfigurable::configure(&mut component, config.clone());
    let retrieved_config = IConfigurable::get_config(&component);
    assert_eq!(retrieved_config.len(), 1);
    assert_eq!(retrieved_config.get("key1"), Some(&json!("value1")));
    
    IConfigurable::update_config(&mut component, "key2", json!("value2"));
    assert!(IConfigurable::has_config(&component, "key2"));
    
    IConfigurable::remove_config(&mut component, "key1");
    assert!(!IConfigurable::has_config(&component, "key1"));
}

// ============================================================================
// NEW ERROR TYPES TESTS
// ============================================================================

#[test]
fn test_xwsystem_error_creation() {
    let error = XWSystemError::new("Test error message");
    assert_eq!(error.message(), "Test error message");
    assert!(error.source().is_none());
}

#[test]
fn test_xwsystem_error_with_source() {
    use std::error::Error;
    let source = Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"));
    let error = XWSystemError::with_source("Test error", source);
    assert_eq!(error.message(), "Test error");
    assert!(error.source().is_some());
}

#[test]
fn test_xwsystem_initialization_error() {
    let error = XWSystemInitializationError::new("Failed to initialize");
    assert_eq!(error.message(), "Failed to initialize");
}

#[test]
fn test_xwsystem_configuration_error() {
    let error = XWSystemConfigurationError::new("Invalid configuration");
    assert_eq!(error.message(), "Invalid configuration");
}

#[test]
fn test_xwsystem_state_error() {
    let error = XWSystemStateError::new("Invalid state");
    assert_eq!(error.message(), "Invalid state");
}

#[test]
fn test_xwsystem_dependency_error() {
    let error = XWSystemDependencyError::new("Missing dependency");
    assert_eq!(error.message(), "Missing dependency");
}

#[test]
fn test_xwsystem_resource_error() {
    let error = XWSystemResourceError::new("Resource unavailable");
    assert_eq!(error.message(), "Resource unavailable");
}

#[test]
fn test_xwsystem_timeout_error() {
    let error = XWSystemTimeoutError::new("Operation timed out");
    assert_eq!(error.message(), "Operation timed out");
}

#[test]
fn test_xwsystem_permission_error() {
    let error = XWSystemPermissionError::new("Permission denied");
    assert_eq!(error.message(), "Permission denied");
}

#[test]
fn test_xwsystem_validation_error() {
    let error = XWSystemValidationError::new("Validation failed");
    assert_eq!(error.message(), "Validation failed");
}

#[test]
fn test_xwsystem_operation_error() {
    let error = XWSystemOperationError::new("Operation failed");
    assert_eq!(error.message(), "Operation failed");
}

// ============================================================================
// NEW ENUMS TESTS
// ============================================================================

#[test]
fn test_system_status_enum() {
    assert_eq!(SystemStatus::Unknown.to_string(), "unknown");
    assert_eq!(SystemStatus::Initializing.to_string(), "initializing");
    assert_eq!(SystemStatus::Ready.to_string(), "ready");
    assert_eq!(SystemStatus::Running.to_string(), "running");
    assert_eq!(SystemStatus::Paused.to_string(), "paused");
    assert_eq!(SystemStatus::Stopping.to_string(), "stopping");
    assert_eq!(SystemStatus::Stopped.to_string(), "stopped");
    assert_eq!(SystemStatus::Error.to_string(), "error");
    assert_eq!(SystemStatus::Maintenance.to_string(), "maintenance");
    
    assert_eq!(SystemStatus::default(), SystemStatus::Unknown);
}

#[test]
fn test_component_type_enum() {
    assert_eq!(ComponentType::Core.to_string(), "core");
    assert_eq!(ComponentType::Io.to_string(), "io");
    assert_eq!(ComponentType::Security.to_string(), "security");
    assert_eq!(ComponentType::Monitoring.to_string(), "monitoring");
    assert_eq!(ComponentType::Validation.to_string(), "validation");
    assert_eq!(ComponentType::Caching.to_string(), "caching");
    assert_eq!(ComponentType::Plugin.to_string(), "plugin");
    assert_eq!(ComponentType::HttpClient.to_string(), "http_client");
    assert_eq!(ComponentType::Operations.to_string(), "operations");
    assert_eq!(ComponentType::Threading.to_string(), "threading");
    assert_eq!(ComponentType::Runtime.to_string(), "runtime");
    assert_eq!(ComponentType::Structures.to_string(), "structures");
    assert_eq!(ComponentType::Utils.to_string(), "utils");
    assert_eq!(ComponentType::Ipc.to_string(), "ipc");
    assert_eq!(ComponentType::Cli.to_string(), "cli");
    assert_eq!(ComponentType::Patterns.to_string(), "patterns");
    assert_eq!(ComponentType::Query.to_string(), "query");
    assert_eq!(ComponentType::Config.to_string(), "config");
    assert_eq!(ComponentType::Shared.to_string(), "shared");
    
    assert_eq!(ComponentType::default(), ComponentType::Core);
}

#[test]
fn test_error_severity_enum() {
    assert_eq!(ErrorSeverity::Low.to_string(), "low");
    assert_eq!(ErrorSeverity::Medium.to_string(), "medium");
    assert_eq!(ErrorSeverity::High.to_string(), "high");
    assert_eq!(ErrorSeverity::Critical.to_string(), "critical");
    
    assert_eq!(ErrorSeverity::default(), ErrorSeverity::Medium);
}

#[test]
fn test_log_category_enum() {
    assert_eq!(LogCategory::System.to_string(), "system");
    assert_eq!(LogCategory::Security.to_string(), "security");
    assert_eq!(LogCategory::Performance.to_string(), "performance");
    assert_eq!(LogCategory::Error.to_string(), "error");
    assert_eq!(LogCategory::Warning.to_string(), "warning");
    assert_eq!(LogCategory::Info.to_string(), "info");
    assert_eq!(LogCategory::Debug.to_string(), "debug");
    assert_eq!(LogCategory::Trace.to_string(), "trace");
    
    assert_eq!(LogCategory::default(), LogCategory::Info);
}

#[test]
fn test_config_source_enum() {
    assert_eq!(ConfigSource::File.to_string(), "file");
    assert_eq!(ConfigSource::Environment.to_string(), "environment");
    assert_eq!(ConfigSource::Defaults.to_string(), "defaults");
    assert_eq!(ConfigSource::CommandLine.to_string(), "command_line");
    assert_eq!(ConfigSource::Database.to_string(), "database");
    assert_eq!(ConfigSource::Api.to_string(), "api");
    assert_eq!(ConfigSource::Memory.to_string(), "memory");
    
    assert_eq!(ConfigSource::default(), ConfigSource::Defaults);
}

#[test]
fn test_config_format_enum() {
    assert_eq!(ConfigFormat::Json.to_string(), "json");
    assert_eq!(ConfigFormat::Yaml.to_string(), "yaml");
    assert_eq!(ConfigFormat::Toml.to_string(), "toml");
    assert_eq!(ConfigFormat::Ini.to_string(), "ini");
    assert_eq!(ConfigFormat::Env.to_string(), "env");
    assert_eq!(ConfigFormat::Xml.to_string(), "xml");
    assert_eq!(ConfigFormat::Python.to_string(), "python");
    
    assert_eq!(ConfigFormat::default(), ConfigFormat::Json);
}

#[test]
fn test_config_priority_enum() {
    assert_eq!(ConfigPriority::Low.to_string(), "low");
    assert_eq!(ConfigPriority::Normal.to_string(), "normal");
    assert_eq!(ConfigPriority::High.to_string(), "high");
    assert_eq!(ConfigPriority::Critical.to_string(), "critical");
    
    assert_eq!(ConfigPriority::default(), ConfigPriority::Normal);
}

#[test]
fn test_config_type_enum() {
    assert_eq!(ConfigType::Application.to_string(), "application");
    assert_eq!(ConfigType::User.to_string(), "user");
    assert_eq!(ConfigType::System.to_string(), "system");
    assert_eq!(ConfigType::Runtime.to_string(), "runtime");
    
    assert_eq!(ConfigType::default(), ConfigType::Application);
}

#[test]
fn test_performance_mode_enum() {
    assert_eq!(PerformanceMode::Normal.to_string(), "normal");
    assert_eq!(PerformanceMode::Optimized.to_string(), "optimized");
    assert_eq!(PerformanceMode::Debug.to_string(), "debug");
    
    assert_eq!(PerformanceMode::default(), PerformanceMode::Normal);
}

#[test]
fn test_advanced_performance_mode_enum() {
    assert_eq!(AdvancedPerformanceMode::Standard.to_string(), "standard");
    assert_eq!(AdvancedPerformanceMode::HighPerformance.to_string(), "high_performance");
    assert_eq!(AdvancedPerformanceMode::UltraPerformance.to_string(), "ultra_performance");
    assert_eq!(AdvancedPerformanceMode::Balanced.to_string(), "balanced");
    
    assert_eq!(AdvancedPerformanceMode::default(), AdvancedPerformanceMode::Standard);
}

#[test]
fn test_new_enums_serialization() {
    // Test SystemStatus serialization
    let status = SystemStatus::Running;
    let json = serde_json::to_string(&status).unwrap();
    assert_eq!(json, "\"running\"");
    let deserialized: SystemStatus = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, SystemStatus::Running);
    
    // Test ComponentType serialization
    let component_type = ComponentType::Security;
    let json = serde_json::to_string(&component_type).unwrap();
    assert_eq!(json, "\"security\"");
    let deserialized: ComponentType = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, ComponentType::Security);
    
    // Test ConfigSource serialization
    let source = ConfigSource::File;
    let json = serde_json::to_string(&source).unwrap();
    assert_eq!(json, "\"file\"");
    let deserialized: ConfigSource = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, ConfigSource::File);
}

