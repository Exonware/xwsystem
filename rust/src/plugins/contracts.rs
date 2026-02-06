// #exonware/xwsystem/rust/src/plugins/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Plugin protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::plugins::defs::{HookType, PluginEvent, PluginPriority, PluginState, PluginType};

// ============================================================================

// EXTENSIBLE INTERFACES

// ============================================================================

// ============================================================================

// HOOKABLE INTERFACES

// ============================================================================

// ============================================================================

// PLUGIN MANAGER INTERFACES

// ============================================================================

// ============================================================================

// PLUGIN REGISTRY INTERFACES

// ============================================================================

// ============================================================================

// PLUGIN DISCOVERY INTERFACES

// ============================================================================

// ============================================================================

// PLUGIN CONFIGURATION INTERFACES

// ============================================================================

// ============================================================================

// PLUGIN EVENTS INTERFACES

// ============================================================================

// ============================================================================

// PLUGIN DEPENDENCY INTERFACES

// ============================================================================

/// Interface for plugins.
///
/// Enforces consistent plugin behavior across XWSystem.
pub trait IPlugin {
    /// Initialize the plugin.
    fn initialize(&self) -> ();

    /// Shutdown the plugin.
    fn shutdown(&self) -> ();

    /// Get plugin information.
    /// Returns:
    /// Plugin information dictionary
    fn get_info(&self) -> HashMap<String, serde_json::Value>;

    /// Check if plugin is enabled.
    /// Returns:
    /// True if enabled
    fn is_enabled(&self) -> bool;

    /// Enable the plugin.
    fn enable(&self) -> ();

    /// Disable the plugin.
    fn disable(&self) -> ();

    /// Get plugin state.
    /// Returns:
    /// Current plugin state
    fn get_state(&self) -> PluginState;

    /// Get plugin type.
    /// Returns:
    /// Plugin type
    fn get_plugin_type(&self) -> PluginType;

    /// Get plugin priority.
    /// Returns:
    /// Plugin priority
    fn get_priority(&self) -> PluginPriority;

    /// Get plugin dependencies.
    /// Returns:
    /// List of dependency names
    fn get_dependencies(&self) -> Vec<String>;

    /// Get plugin version.
    /// Returns:
    /// Plugin version string
    fn get_version(&self) -> String;

    /// Get plugin author.
    /// Returns:
    /// Plugin author
    fn get_author(&self) -> String;

    /// Get plugin description.
    /// Returns:
    /// Plugin description
    fn get_description(&self) -> String;

}

/// Interface for extensible objects.
///
/// Enforces consistent extension behavior across XWSystem.
pub trait IExtensible {
    /// Add extension.
    /// Args:
    /// extension: Extension to add
    /// Returns:
    /// True if added successfully
    fn add_extension(&self, extension: serde_json::Value) -> bool;

    /// Remove extension by name.
    /// Args:
    /// name: Extension name
    /// Returns:
    /// True if removed
    fn remove_extension(&self, name: String) -> bool;

    /// Get all extensions.
    /// Returns:
    /// List of extensions
    fn get_extensions(&self) -> Vec<serde_json::Value>;

    /// Get extension by name.
    /// Args:
    /// name: Extension name
    /// Returns:
    /// Extension or None
    fn get_extension(&self, name: String) -> Option<serde_json::Value>;

    /// Check if extension exists.
    /// Args:
    /// name: Extension name
    /// Returns:
    /// True if exists
    fn has_extension(&self, name: String) -> bool;

    /// List extension names.
    /// Returns:
    /// List of extension names
    fn list_extension_names(&self) -> Vec<String>;

    /// Clear all extensions.
    fn clear_extensions(&self) -> ();

    /// Get number of extensions.
    /// Returns:
    /// Number of extensions
    fn get_extension_count(&self) -> i64;

}

/// Interface for hookable objects.
///
/// Enforces consistent hook behavior across XWSystem.
pub trait IHookable {
    /// Add hook callback.
    /// Args:
    /// event: Event name
    /// callback: Callback function
    /// hook_type: Type of hook
    /// Returns:
    /// Hook ID
    fn add_hook(&self, event: String, callback: fn(), hook_type: HookType) -> String;

    /// Remove hook callback.
    /// Args:
    /// event: Event name
    /// callback: Callback function
    /// Returns:
    /// True if removed
    fn remove_hook(&self, event: String, callback: fn()) -> bool;

    /// Trigger hook event.
    /// Args:
    /// event: Event name
    /// data: Event data
    /// Returns:
    /// Hook result
    fn trigger_hook(&self, event: String, data: serde_json::Value) -> serde_json::Value;

    /// List hooks.
    /// Args:
    /// event: Filter by event name
    /// Returns:
    /// List of hook information
    fn list_hooks(&self, event: Option<String>) -> Vec<HashMap<String, serde_json::Value>>;

    /// Check if event has hooks.
    /// Args:
    /// event: Event name
    /// Returns:
    /// True if has hooks
    fn has_hooks(&self, event: String) -> bool;

    /// Clear hooks.
    /// Args:
    /// event: Clear hooks for specific event, or all if None
    fn clear_hooks(&self, event: Option<String>) -> ();

    /// Get hook count.
    /// Args:
    /// event: Count hooks for specific event, or all if None
    /// Returns:
    /// Number of hooks
    fn get_hook_count(&self, event: Option<String>) -> i64;

}

/// Interface for plugin management.
///
/// Enforces consistent plugin management across XWSystem.
pub trait IPluginManager {
    /// Load plugin from path.
    /// Args:
    /// plugin_path: Plugin path or module name
    /// Returns:
    /// True if loaded successfully
    fn load_plugin(&self, plugin_path: String) -> bool;

    /// Unload plugin.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// True if unloaded successfully
    fn unload_plugin(&self, plugin_name: String) -> bool;

    /// Reload plugin.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// True if reloaded successfully
    fn reload_plugin(&self, plugin_name: String) -> bool;

    /// Get plugin by name.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// Plugin instance or None
    fn get_plugin(&self, plugin_name: String) -> Option<IPlugin>;

    /// List all plugin names.
    /// Returns:
    /// List of plugin names
    fn list_plugins(&self) -> Vec<String>;

    /// List loaded plugin names.
    /// Returns:
    /// List of loaded plugin names
    fn list_loaded_plugins(&self) -> Vec<String>;

    /// Check if plugin is loaded.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// True if loaded
    fn is_plugin_loaded(&self, plugin_name: String) -> bool;

    /// Get plugin information.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// Plugin information or None
    fn get_plugin_info(&self, plugin_name: String) -> Option<HashMap<String, serde_json::Value>>;

}

/// Interface for plugin registry.
///
/// Enforces consistent plugin registration across XWSystem.
pub trait IPluginRegistry {
    /// Register plugin class.
    /// Args:
    /// plugin_class: Plugin class
    /// name: Plugin name
    /// priority: Plugin priority
    /// Returns:
    /// True if registered successfully
    fn register_plugin(&self, plugin_class: String, name: String, priority: PluginPriority) -> bool;

    /// Unregister plugin.
    /// Args:
    /// name: Plugin name
    /// Returns:
    /// True if unregistered
    fn unregister_plugin(&self, name: String) -> bool;

    /// Get all registered plugins.
    /// Returns:
    /// Dictionary of registered plugins
    fn get_registered_plugins(&self) -> HashMap<String, String>;

    /// Check if plugin is registered.
    /// Args:
    /// name: Plugin name
    /// Returns:
    /// True if registered
    fn is_plugin_registered(&self, name: String) -> bool;

    /// Get plugin class by name.
    /// Args:
    /// name: Plugin name
    /// Returns:
    /// Plugin class or None
    fn get_plugin_class(&self, name: String) -> Option<String>;

    /// Clear plugin registry.
    fn clear_registry(&self) -> ();

    /// Get registry statistics.
    /// Returns:
    /// Registry statistics dictionary
    fn get_registry_stats(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for plugin discovery.
///
/// Enforces consistent plugin discovery across XWSystem.
pub trait IPluginDiscovery {
    /// Discover plugins in search paths.
    /// Args:
    /// search_paths: Paths to search for plugins
    /// Returns:
    /// List of discovered plugin paths
    fn discover_plugins(&self, search_paths: Vec<String>) -> Vec<String>;

    /// Scan directory for plugins.
    /// Args:
    /// directory: Directory to scan
    /// Returns:
    /// List of plugin files found
    fn scan_directory(&self, directory: String) -> Vec<String>;

    /// Validate plugin.
    /// Args:
    /// plugin_path: Plugin path to validate
    /// Returns:
    /// Tuple of (is_valid, error_messages)
    fn validate_plugin(&self, plugin_path: String) -> (bool, Vec<String>);

    /// Get plugin metadata.
    /// Args:
    /// plugin_path: Plugin path
    /// Returns:
    /// Plugin metadata or None
    fn get_plugin_metadata(&self, plugin_path: String) -> Option<HashMap<String, serde_json::Value>>;

    /// Check if file is a plugin.
    /// Args:
    /// file_path: File path to check
    /// Returns:
    /// True if file is a plugin
    fn is_plugin_file(&self, file_path: String) -> bool;

    /// Get supported plugin file extensions.
    /// Returns:
    /// List of supported extensions
    fn get_supported_extensions(&self) -> Vec<String>;

}

/// Interface for plugin configuration.
///
/// Enforces consistent plugin configuration across XWSystem.
pub trait IPluginConfig {
    /// Get plugin configuration.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// Plugin configuration dictionary
    fn get_plugin_config(&self, plugin_name: String) -> HashMap<String, serde_json::Value>;

    /// Set plugin configuration.
    /// Args:
    /// plugin_name: Plugin name
    /// config: Configuration dictionary
    fn set_plugin_config(&self, plugin_name: String, config: HashMap<String, serde_json::Value>) -> ();

    /// Update plugin configuration value.
    /// Args:
    /// plugin_name: Plugin name
    /// key: Configuration key
    /// value: Configuration value
    fn update_plugin_config(&self, plugin_name: String, key: String, value: serde_json::Value) -> ();

    /// Get plugin configuration value.
    /// Args:
    /// plugin_name: Plugin name
    /// key: Configuration key
    /// default: Default value
    /// Returns:
    /// Configuration value or default
    fn get_plugin_config_value(&self, plugin_name: String, key: String, default: serde_json::Value) -> serde_json::Value;

    /// Check if plugin has configuration key.
    /// Args:
    /// plugin_name: Plugin name
    /// key: Configuration key
    /// Returns:
    /// True if key exists
    fn has_plugin_config(&self, plugin_name: String, key: String) -> bool;

    /// Remove plugin configuration key.
    /// Args:
    /// plugin_name: Plugin name
    /// key: Configuration key
    /// Returns:
    /// True if removed
    fn remove_plugin_config(&self, plugin_name: String, key: String) -> bool;

    /// Clear plugin configuration.
    /// Args:
    /// plugin_name: Plugin name
    fn clear_plugin_config(&self, plugin_name: String) -> ();

    /// Save plugin configuration to file.
    /// Args:
    /// plugin_name: Plugin name
    /// file_path: File path to save to
    /// Returns:
    /// True if saved successfully
    fn save_plugin_config(&self, plugin_name: String, file_path: String) -> bool;

    /// Load plugin configuration from file.
    /// Args:
    /// plugin_name: Plugin name
    /// file_path: File path to load from
    /// Returns:
    /// True if loaded successfully
    fn load_plugin_config(&self, plugin_name: String, file_path: String) -> bool;

}

/// Interface for plugin events.
///
/// Enforces consistent plugin event handling across XWSystem.
pub trait IPluginEvents {
    /// Emit plugin event.
    /// Args:
    /// event: Plugin event
    /// plugin_name: Plugin name
    /// data: Event data
    fn emit_event(&self, event: PluginEvent, plugin_name: String, data: serde_json::Value) -> ();

    /// Subscribe to plugin event.
    /// Args:
    /// event: Plugin event
    /// callback: Event callback
    /// Returns:
    /// Subscription ID
    fn subscribe_to_event(&self, event: PluginEvent, callback: fn()) -> String;

    /// Unsubscribe from plugin event.
    /// Args:
    /// subscription_id: Subscription ID
    /// Returns:
    /// True if unsubscribed
    fn unsubscribe_from_event(&self, subscription_id: String) -> bool;

    /// Get event subscribers.
    /// Args:
    /// event: Plugin event
    /// Returns:
    /// List of subscriber callbacks
    fn get_event_subscribers(&self, event: PluginEvent) -> Vec<fn()>;

    /// Clear event subscribers.
    /// Args:
    /// event: Clear subscribers for specific event, or all if None
    fn clear_event_subscribers(&self, event: Option<PluginEvent>) -> ();

    /// Get event history.
    /// Args:
    /// event: Filter by event type
    /// limit: Maximum number of events
    /// Returns:
    /// List of event history entries
    fn get_event_history(&self, event: Option<PluginEvent>, limit: i64) -> Vec<HashMap<String, serde_json::Value>>;

    /// Clear event history.
    fn clear_event_history(&self) -> ();

}

/// Interface for plugin dependency management.
///
/// Enforces consistent plugin dependency handling across XWSystem.
pub trait IPluginDependency {
    /// Add plugin dependency.
    /// Args:
    /// plugin_name: Plugin name
    /// dependency: Dependency name
    /// version: Required version
    fn add_dependency(&self, plugin_name: String, dependency: String, version: Option<String>) -> ();

    /// Remove plugin dependency.
    /// Args:
    /// plugin_name: Plugin name
    /// dependency: Dependency name
    /// Returns:
    /// True if removed
    fn remove_dependency(&self, plugin_name: String, dependency: String) -> bool;

    /// Get plugin dependencies.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// List of dependency information
    fn get_dependencies(&self, plugin_name: String) -> Vec<HashMap<String, serde_json::Value>>;

    /// Check if plugin dependencies are satisfied.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// Tuple of (all_satisfied, missing_dependencies)
    fn check_dependencies(&self, plugin_name: String) -> (bool, Vec<String>);

    /// Resolve plugin dependency order.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// List of plugins in dependency order
    fn resolve_dependencies(&self, plugin_name: String) -> Vec<String>;

    /// Get plugins that depend on this plugin.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// List of dependent plugin names
    fn get_dependents(&self, plugin_name: String) -> Vec<String>;

    /// Check for circular dependencies.
    /// Args:
    /// plugin_name: Plugin name
    /// Returns:
    /// True if circular dependency exists
    fn has_circular_dependency(&self, plugin_name: String) -> bool;

    /// Get plugin dependency graph.
    /// Returns:
    /// Dependency graph dictionary
    fn get_dependency_graph(&self) -> HashMap<String, Vec<String>>;

}
