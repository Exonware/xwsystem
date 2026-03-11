// #exonware/xwsystem/rust/src/plugins/errors.rs
//exonware/xwsystem/plugins/errors.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Plugin-specific error classes for XSystem plugin system.

/// Base exception for all plugin-related errors.
#[derive(Debug)]
pub struct PluginError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PluginError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl PluginError {
    pub fn new(message: impl Into<String>) -> Self {
        PluginError {
            message: message.into(),
            source: None,
        }
    }

}

/// Error when a requested plugin is not found.
#[derive(Debug)]
pub struct PluginNotFoundError {
    pub plugin_name: String,
    pub available_plugins: Option<Vec<String>>,
}

impl std::fmt::Display for PluginNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("Plugin '{}' not found", self.plugin_name);
        if let Some(ref available) = self.available_plugins {
            if !available.is_empty() {
                msg.push_str(&format!(". Available plugins: {}", available.join(", ")));
            }
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginNotFoundError {}

impl PluginNotFoundError {
    /// Constructor
    pub fn new(
        plugin_name: String,
        available_plugins: Option<Vec<String>>
    ) -> Self {
        Self {
            plugin_name,
            available_plugins,
        }
    }

}

/// Error when plugin loading fails.
#[derive(Debug)]
pub struct PluginLoadError {
    pub message: String,
    pub plugin_name: String,
    pub plugin_path: Option<String>,
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref path) = self.plugin_path {
            msg.push_str(&format!(" (Path: {})", path));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginLoadError {}

impl PluginLoadError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        plugin_path: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            plugin_path,
        }
    }

}

/// Error when plugin import fails.
#[derive(Debug)]
pub struct PluginImportError {
    pub message: String,
    pub plugin_name: String,
    pub module_name: Option<String>,
}

impl std::fmt::Display for PluginImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref module) = self.module_name {
            msg.push_str(&format!(" (Module: {})", module));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginImportError {}

impl PluginImportError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        module_name: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            module_name,
        }
    }

}

/// Error when plugin dependencies are not met.
#[derive(Debug)]
pub struct PluginDependencyError {
    pub message: String,
    pub plugin_name: String,
    pub missing_dependencies: Option<Vec<String>>,
}


impl std::fmt::Display for PluginDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref deps) = self.missing_dependencies {
            if !deps.is_empty() {
                msg.push_str(&format!(" (Missing dependencies: {})", deps.join(", ")));
            }
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginDependencyError {}

impl PluginDependencyError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        missing_dependencies: Option<Vec<String>>
    ) -> Self {
        Self {
            message,
            plugin_name,
            missing_dependencies,
        }
    }

}

/// Error when plugin version is incompatible.
#[derive(Debug)]
pub struct PluginVersionError {
    pub message: String,
    pub plugin_name: String,
    pub required_version: Option<String>,
    pub actual_version: Option<String>,
}

impl std::fmt::Display for PluginVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref req_ver) = self.required_version {
            msg.push_str(&format!(" (Required: {})", req_ver));
        }
        if let Some(ref act_ver) = self.actual_version {
            msg.push_str(&format!(" (Actual: {})", act_ver));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginVersionError {}

impl PluginVersionError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        required_version: Option<String>,
        actual_version: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            required_version,
            actual_version,
        }
    }

}

/// Error when plugin registration fails.
#[derive(Debug)]
pub struct PluginRegistrationError {
    pub message: String,
    pub plugin_name: String,
    pub plugin_class: Option<String>,
}

impl std::fmt::Display for PluginRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref class) = self.plugin_class {
            msg.push_str(&format!(" (Class: {})", class));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginRegistrationError {}

impl PluginRegistrationError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        plugin_class: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            plugin_class,
        }
    }

}

/// Error when trying to register a duplicate plugin.
#[derive(Debug)]
pub struct PluginDuplicateError {
    pub message: String,
    pub plugin_name: String,
    pub existing_version: Option<String>,
}

impl std::fmt::Display for PluginDuplicateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref ver) = self.existing_version {
            msg.push_str(&format!(" (Existing version: {})", ver));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginDuplicateError {}

impl PluginDuplicateError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        existing_version: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            existing_version,
        }
    }

}

/// Error when plugin validation fails.
#[derive(Debug)]
pub struct PluginValidationError {
    pub message: String,
    pub plugin_name: String,
    pub validation_errors: Option<Vec<String>>,
}

impl std::fmt::Display for PluginValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref errors) = self.validation_errors {
            if !errors.is_empty() {
                msg.push_str(&format!(" (Validation errors: {})", errors.join(", ")));
            }
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginValidationError {}

impl PluginValidationError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        validation_errors: Option<Vec<String>>
    ) -> Self {
        Self {
            message,
            plugin_name,
            validation_errors,
        }
    }

}

/// Error when plugin initialization fails.
#[derive(Debug)]
pub struct PluginInitializationError {
    pub message: String,
    pub plugin_name: String,
    pub plugin_version: Option<String>,
}

impl std::fmt::Display for PluginInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref ver) = self.plugin_version {
            msg.push_str(&format!(" (Version: {})", ver));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginInitializationError {}

impl PluginInitializationError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        plugin_version: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            plugin_version,
        }
    }

}

/// Error when plugin configuration is invalid.
#[derive(Debug)]
pub struct PluginConfigurationError {
    pub message: String,
    pub plugin_name: String,
    pub config_key: Option<String>,
    pub config_value: Option<serde_json::Value>,
}

impl std::fmt::Display for PluginConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref key) = self.config_key {
            msg.push_str(&format!(" (Config key: {})", key));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginConfigurationError {}

impl PluginConfigurationError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        config_key: Option<String>,
        config_value: Option<serde_json::Value>
    ) -> Self {
        Self {
            message,
            plugin_name,
            config_key,
            config_value,
        }
    }

}

/// Error when plugin resource allocation fails.
#[derive(Debug)]
pub struct PluginResourceError {
    pub message: String,
    pub plugin_name: String,
    pub resource_type: Option<String>,
}

impl std::fmt::Display for PluginResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref res_type) = self.resource_type {
            msg.push_str(&format!(" (Resource type: {})", res_type));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginResourceError {}

impl PluginResourceError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        resource_type: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            resource_type,
        }
    }

}

/// Error when plugin execution fails.
#[derive(Debug)]
pub struct PluginExecutionError {
    pub message: String,
    pub plugin_name: String,
    pub plugin_version: Option<String>,
    pub operation: Option<String>,
}

impl std::fmt::Display for PluginExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref ver) = self.plugin_version {
            msg.push_str(&format!(" (Version: {})", ver));
        }
        if let Some(ref op) = self.operation {
            msg.push_str(&format!(" (Operation: {})", op));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginExecutionError {}

impl PluginExecutionError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        plugin_version: Option<String>,
        operation: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            plugin_version,
            operation,
        }
    }

}

/// Error when plugin method execution fails.
#[derive(Debug)]
pub struct PluginMethodError {
    pub message: String,
    pub plugin_name: String,
    pub method_name: String,
}

impl std::fmt::Display for PluginMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Plugin: {}] {} (Method: {})", self.plugin_name, self.message, self.method_name)
    }
}

impl std::error::Error for PluginMethodError {}

impl PluginMethodError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        method_name: String
    ) -> Self {
        Self {
            message,
            plugin_name,
            method_name,
        }
    }

}

/// Error when plugin operation times out.
#[derive(Debug)]
pub struct PluginTimeoutError {
    pub message: String,
    pub plugin_name: String,
    pub timeout_duration: f64,
}

impl std::fmt::Display for PluginTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Plugin: {}] {} (Timeout: {}s)", self.plugin_name, self.message, self.timeout_duration)
    }
}

impl std::error::Error for PluginTimeoutError {}

impl PluginTimeoutError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        timeout_duration: f64
    ) -> Self {
        Self {
            message,
            plugin_name,
            timeout_duration,
        }
    }

}

/// Error when plugin lacks required permissions.
#[derive(Debug)]
pub struct PluginPermissionError {
    pub message: String,
    pub plugin_name: String,
    pub required_permission: Option<String>,
}

impl std::fmt::Display for PluginPermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref perm) = self.required_permission {
            msg.push_str(&format!(" (Required permission: {})", perm));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginPermissionError {}

impl PluginPermissionError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        required_permission: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            required_permission,
        }
    }

}

/// Error when plugin cleanup fails.
#[derive(Debug)]
pub struct PluginCleanupError {
    pub message: String,
    pub plugin_name: String,
    pub plugin_version: Option<String>,
}

impl std::fmt::Display for PluginCleanupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref ver) = self.plugin_version {
            msg.push_str(&format!(" (Version: {})", ver));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginCleanupError {}

impl PluginCleanupError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        plugin_version: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            plugin_version,
        }
    }

}

/// Error when plugin unloading fails.
#[derive(Debug)]
pub struct PluginUnloadError {
    pub message: String,
    pub plugin_name: String,
}

impl std::fmt::Display for PluginUnloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Plugin: {}] {}", self.plugin_name, self.message)
    }
}

impl std::error::Error for PluginUnloadError {}

impl PluginUnloadError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String
    ) -> Self {
        Self {
            message,
            plugin_name,
        }
    }

}

/// Error related to plugin registry operations.
#[derive(Debug)]
pub struct PluginRegistryError {
    pub message: String,
    pub registry_operation: Option<String>,
}

impl std::fmt::Display for PluginRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = self.message.clone();
        if let Some(ref op) = self.registry_operation {
            msg.push_str(&format!(" (Operation: {})", op));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginRegistryError {}

impl PluginRegistryError {
    /// Constructor
    pub fn new(
        message: String,
        registry_operation: Option<String>
    ) -> Self {
        Self {
            message,
            registry_operation,
        }
    }

}

/// Error when plugin registry is full.
#[derive(Debug)]
pub struct PluginRegistryFullError {
    pub message: String,
    pub max_plugins: i64,
    pub current_count: i64,
}

impl std::fmt::Display for PluginRegistryFullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Current: {}/{})", self.message, self.current_count, self.max_plugins)
    }
}

impl std::error::Error for PluginRegistryFullError {}

impl PluginRegistryFullError {
    /// Constructor
    pub fn new(
        message: String,
        max_plugins: i64,
        current_count: i64
    ) -> Self {
        Self {
            message,
            max_plugins,
            current_count,
        }
    }

}

/// Error when plugin registry is locked.
#[derive(Debug)]
pub struct PluginRegistryLockError {
    pub message: String,
    pub registry_operation: String,
}

impl std::fmt::Display for PluginRegistryLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (Operation: {})", self.message, self.registry_operation)
    }
}

impl std::error::Error for PluginRegistryLockError {}

impl PluginRegistryLockError {
    /// Constructor
    pub fn new(
        message: String,
        registry_operation: String
    ) -> Self {
        Self {
            message,
            registry_operation,
        }
    }

}

/// Error related to plugin manager operations.
#[derive(Debug)]
pub struct PluginManagerError {
    pub message: String,
    pub manager_operation: Option<String>,
}

impl std::fmt::Display for PluginManagerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = self.message.clone();
        if let Some(ref op) = self.manager_operation {
            msg.push_str(&format!(" (Operation: {})", op));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginManagerError {}

impl PluginManagerError {
    /// Constructor
    pub fn new(
        message: String,
        manager_operation: Option<String>
    ) -> Self {
        Self {
            message,
            manager_operation,
        }
    }

}

/// Error when plugin manager is not initialized.
#[derive(Debug)]
pub struct PluginManagerNotInitializedError {
    pub message: String,
}

impl std::fmt::Display for PluginManagerNotInitializedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PluginManagerNotInitializedError {}

impl PluginManagerNotInitializedError {
    /// Constructor
    pub fn new(
        message: String
    ) -> Self {
        Self {
            message,
        }
    }

}

/// Error when plugin manager shutdown fails.
#[derive(Debug)]
pub struct PluginManagerShutdownError {
    pub message: String,
}

impl std::fmt::Display for PluginManagerShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PluginManagerShutdownError {}

impl PluginManagerShutdownError {
    /// Constructor
    pub fn new(
        message: String
    ) -> Self {
        Self {
            message,
        }
    }

}

/// Error when plugin discovery fails.
#[derive(Debug)]
pub struct PluginDiscoveryError {
    pub message: String,
    pub discovery_path: Option<String>,
}

impl std::fmt::Display for PluginDiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = self.message.clone();
        if let Some(ref path) = self.discovery_path {
            msg.push_str(&format!(" (Path: {})", path));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginDiscoveryError {}

impl PluginDiscoveryError {
    /// Constructor
    pub fn new(
        message: String,
        discovery_path: Option<String>
    ) -> Self {
        Self {
            message,
            discovery_path,
        }
    }

}

/// Error when plugin entry point is invalid.
#[derive(Debug)]
pub struct PluginEntryPointError {
    pub message: String,
    pub entry_point: Option<String>,
}

impl std::fmt::Display for PluginEntryPointError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = self.message.clone();
        if let Some(ref ep) = self.entry_point {
            msg.push_str(&format!(" (Entry point: {})", ep));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginEntryPointError {}

impl PluginEntryPointError {
    /// Constructor
    pub fn new(
        message: String,
        entry_point: Option<String>
    ) -> Self {
        Self {
            message,
            entry_point,
        }
    }

}

/// Error when plugin metadata is invalid.
#[derive(Debug)]
pub struct PluginMetadataError {
    pub message: String,
    pub plugin_name: String,
    pub metadata_key: Option<String>,
}

impl std::fmt::Display for PluginMetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref key) = self.metadata_key {
            msg.push_str(&format!(" (Metadata key: {})", key));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginMetadataError {}

impl PluginMetadataError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        metadata_key: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            metadata_key,
        }
    }

}

/// Error when plugin interface is invalid.
#[derive(Debug)]
pub struct PluginInterfaceError {
    pub message: String,
    pub plugin_name: String,
    pub interface_name: Option<String>,
}

impl std::fmt::Display for PluginInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref iface) = self.interface_name {
            msg.push_str(&format!(" (Interface: {})", iface));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginInterfaceError {}

impl PluginInterfaceError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        interface_name: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            interface_name,
        }
    }

}

/// Error when plugin lifecycle operation fails.
#[derive(Debug)]
pub struct PluginLifecycleError {
    pub message: String,
    pub plugin_name: String,
    pub lifecycle_stage: Option<String>,
}

impl std::fmt::Display for PluginLifecycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref stage) = self.lifecycle_stage {
            msg.push_str(&format!(" (Lifecycle stage: {})", stage));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginLifecycleError {}

impl PluginLifecycleError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        lifecycle_stage: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            lifecycle_stage,
        }
    }

}

/// Error when plugin is in invalid state for operation.
#[derive(Debug)]
pub struct PluginStateError {
    pub message: String,
    pub plugin_name: String,
    pub current_state: String,
    pub required_state: String,
}

impl std::fmt::Display for PluginStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Plugin: {}] {} (Current: {}, Required: {})", 
               self.plugin_name, self.message, self.current_state, self.required_state)
    }
}

impl std::error::Error for PluginStateError {}

impl PluginStateError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        current_state: String,
        required_state: String
    ) -> Self {
        Self {
            message,
            plugin_name,
            current_state,
            required_state,
        }
    }

}

/// Error when plugin hook execution fails.
#[derive(Debug)]
pub struct PluginHookError {
    pub message: String,
    pub plugin_name: String,
    pub hook_name: String,
}

impl std::fmt::Display for PluginHookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Plugin: {}] {} (Hook: {})", self.plugin_name, self.message, self.hook_name)
    }
}

impl std::error::Error for PluginHookError {}

impl PluginHookError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        hook_name: String
    ) -> Self {
        Self {
            message,
            plugin_name,
            hook_name,
        }
    }

}

/// Error when plugin event handling fails.
#[derive(Debug)]
pub struct PluginEventError {
    pub message: String,
    pub plugin_name: String,
    pub event_name: String,
}

impl std::fmt::Display for PluginEventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Plugin: {}] {} (Event: {})", self.plugin_name, self.message, self.event_name)
    }
}

impl std::error::Error for PluginEventError {}

impl PluginEventError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        event_name: String
    ) -> Self {
        Self {
            message,
            plugin_name,
            event_name,
        }
    }

}

/// Error when plugin communication fails.
#[derive(Debug)]
pub struct PluginCommunicationError {
    pub message: String,
    pub plugin_name: String,
    pub target_plugin: Option<String>,
}

impl std::fmt::Display for PluginCommunicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref target) = self.target_plugin {
            msg.push_str(&format!(" (Target plugin: {})", target));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginCommunicationError {}

impl PluginCommunicationError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        target_plugin: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            target_plugin,
        }
    }

}

/// Error when plugin security check fails.
#[derive(Debug)]
pub struct PluginSecurityError {
    pub message: String,
    pub plugin_name: String,
    pub security_violation: Option<String>,
}

impl std::fmt::Display for PluginSecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref violation) = self.security_violation {
            msg.push_str(&format!(" (Security violation: {})", violation));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginSecurityError {}

impl PluginSecurityError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        security_violation: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            security_violation,
        }
    }

}

/// Error when plugin sandbox operation fails.
#[derive(Debug)]
pub struct PluginSandboxError {
    pub message: String,
    pub plugin_name: String,
    pub sandbox_operation: Option<String>,
}

impl std::fmt::Display for PluginSandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref op) = self.sandbox_operation {
            msg.push_str(&format!(" (Sandbox operation: {})", op));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginSandboxError {}

impl PluginSandboxError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        sandbox_operation: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            sandbox_operation,
        }
    }

}

/// Error when plugin capability is insufficient.
#[derive(Debug)]
pub struct PluginCapabilityError {
    pub message: String,
    pub plugin_name: String,
    pub required_capability: Option<String>,
}

impl std::fmt::Display for PluginCapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref cap) = self.required_capability {
            msg.push_str(&format!(" (Required capability: {})", cap));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginCapabilityError {}

impl PluginCapabilityError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        required_capability: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            required_capability,
        }
    }

}

/// Error when plugin compatibility check fails.
#[derive(Debug)]
pub struct PluginCompatibilityError {
    pub message: String,
    pub plugin_name: String,
    pub system_version: Option<String>,
    pub plugin_version: Option<String>,
}

impl std::fmt::Display for PluginCompatibilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref sys_ver) = self.system_version {
            msg.push_str(&format!(" (System version: {})", sys_ver));
        }
        if let Some(ref plugin_ver) = self.plugin_version {
            msg.push_str(&format!(" (Plugin version: {})", plugin_ver));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginCompatibilityError {}

impl PluginCompatibilityError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        system_version: Option<String>,
        plugin_version: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            system_version,
            plugin_version,
        }
    }

}

/// Error when plugin conflicts with another plugin.
#[derive(Debug)]
pub struct PluginConflictError {
    pub message: String,
    pub plugin_name: String,
    pub conflicting_plugin: String,
    pub conflict_type: Option<String>,
}

impl std::fmt::Display for PluginConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {} (Conflicting plugin: {})", 
                             self.plugin_name, self.message, self.conflicting_plugin);
        if let Some(ref ctype) = self.conflict_type {
            msg.push_str(&format!(" (Conflict type: {})", ctype));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginConflictError {}

impl PluginConflictError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        conflicting_plugin: String,
        conflict_type: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            conflicting_plugin,
            conflict_type,
        }
    }

}

/// Error when plugin priority is invalid.
#[derive(Debug)]
pub struct PluginPriorityError {
    pub message: String,
    pub plugin_name: String,
    pub priority: Option<i64>,
}

impl std::fmt::Display for PluginPriorityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(prio) = self.priority {
            msg.push_str(&format!(" (Priority: {})", prio));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginPriorityError {}

impl PluginPriorityError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        priority: Option<i64>
    ) -> Self {
        Self {
            message,
            plugin_name,
            priority,
        }
    }

}

/// Error when plugin dependency cycle is detected.
#[derive(Debug)]
pub struct PluginDependencyCycleError {
    pub message: String,
    pub plugin_name: String,
    pub dependency_cycle: Option<Vec<String>>,
}

impl std::fmt::Display for PluginDependencyCycleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref cycle) = self.dependency_cycle {
            if !cycle.is_empty() {
                msg.push_str(&format!(" (Cycle: {})", cycle.join(" -> ")));
            }
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginDependencyCycleError {}

impl PluginDependencyCycleError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        dependency_cycle: Option<Vec<String>>
    ) -> Self {
        Self {
            message,
            plugin_name,
            dependency_cycle,
        }
    }

}

/// Error when plugin hot reload fails.
#[derive(Debug)]
pub struct PluginHotReloadError {
    pub message: String,
    pub plugin_name: String,
    pub reload_reason: Option<String>,
}

impl std::fmt::Display for PluginHotReloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref reason) = self.reload_reason {
            msg.push_str(&format!(" (Reason: {})", reason));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginHotReloadError {}

impl PluginHotReloadError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        reload_reason: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            reload_reason,
        }
    }

}

/// Error when plugin backup/restore fails.
#[derive(Debug)]
pub struct PluginBackupError {
    pub message: String,
    pub plugin_name: String,
    pub backup_operation: Option<String>,
}

impl std::fmt::Display for PluginBackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref op) = self.backup_operation {
            msg.push_str(&format!(" (Operation: {})", op));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginBackupError {}

impl PluginBackupError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        backup_operation: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            backup_operation,
        }
    }

}

/// Error when plugin migration fails.
#[derive(Debug)]
pub struct PluginMigrationError {
    pub message: String,
    pub plugin_name: String,
    pub from_version: Option<String>,
    pub to_version: Option<String>,
}

impl std::fmt::Display for PluginMigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref from) = self.from_version {
            msg.push_str(&format!(" (From: {})", from));
        }
        if let Some(ref to) = self.to_version {
            msg.push_str(&format!(" (To: {})", to));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginMigrationError {}

impl PluginMigrationError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        from_version: Option<String>,
        to_version: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            from_version,
            to_version,
        }
    }

}

/// Error when plugin monitoring fails.
#[derive(Debug)]
pub struct PluginMonitoringError {
    pub message: String,
    pub plugin_name: String,
    pub monitoring_metric: Option<String>,
}

impl std::fmt::Display for PluginMonitoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref metric) = self.monitoring_metric {
            msg.push_str(&format!(" (Metric: {})", metric));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginMonitoringError {}

impl PluginMonitoringError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        monitoring_metric: Option<String>
    ) -> Self {
        Self {
            message,
            plugin_name,
            monitoring_metric,
        }
    }

}

/// Error when plugin performance is below threshold.
#[derive(Debug)]
pub struct PluginPerformanceError {
    pub message: String,
    pub plugin_name: String,
    pub performance_metric: Option<String>,
    pub threshold: Option<f64>,
    pub actual_value: Option<f64>,
}

impl std::fmt::Display for PluginPerformanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("[Plugin: {}] {}", self.plugin_name, self.message);
        if let Some(ref metric) = self.performance_metric {
            msg.push_str(&format!(" (Metric: {})", metric));
        }
        if let Some(threshold) = self.threshold {
            msg.push_str(&format!(" (Threshold: {})", threshold));
        }
        if let Some(actual) = self.actual_value {
            msg.push_str(&format!(" (Actual: {})", actual));
        }
        write!(f, "{}", msg)
    }
}

impl std::error::Error for PluginPerformanceError {}

impl PluginPerformanceError {
    /// Constructor
    pub fn new(
        message: String,
        plugin_name: String,
        performance_metric: Option<String>,
        threshold: Option<f64>,
        actual_value: Option<f64>
    ) -> Self {
        Self {
            message,
            plugin_name,
            performance_metric,
            threshold,
            actual_value,
        }
    }

}
