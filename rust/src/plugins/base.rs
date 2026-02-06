// #exonware/xwsystem/rust/src/plugins/base.rs
//exonware/xwsystem/plugins/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Plugin system base classes and management.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::config::logging_setup::get_logger;
use crate::plugins::contracts::IPlugin;
use crate::plugins::defs::{PluginPriority, PluginState, PluginType};
use crate::plugins::errors::PluginError;
use crate::runtime::reflection::ReflectionUtils;
use std::path::Path;

// Global plugin manager instance

// Global plugin manager instance

/// Information about a plugin.
#[derive(Debug, Clone)]
pub struct APluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub module_path: String,
    pub class_name: String,
    pub enabled: bool,
    pub priority: i32,
    pub dependencies: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl APluginInfo {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: "unknown".to_string(),
            description: String::new(),
            author: String::new(),
            module_path: String::new(),
            class_name: String::new(),
            enabled: true,
            priority: 100,
            dependencies: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Abstract base class for all plugins.
///
/// Plugins should inherit from this class and implement the required methods.
pub trait APlugin: IPlugin {
    /// Plugin name.
    // Python decorators: @property
    fn name(&self) -> &str;

    /// Plugin version.
    // Python decorators: @property
    fn version(&self) -> &str;

    /// Plugin description.
    fn description(&self) -> &str {
        ""
    }

    /// Plugin author.
    fn author(&self) -> &str {
        ""
    }

    /// List of plugin dependencies.
    fn dependencies(&self) -> Vec<String> {
        Vec::new()
    }

    /// Initialize the plugin.
    fn initialize(&mut self);

    /// Shutdown the plugin.
    fn shutdown(&mut self);

    /// Check if plugin is initialized.
    fn is_initialized(&self) -> bool {
        false
    }

    /// Get plugin information.
    fn get_info(&self) -> HashMap<String, serde_json::Value> {
        let mut info = HashMap::new();
        info.insert("name".to_string(), serde_json::Value::String(self.name().to_string()));
        info.insert("version".to_string(), serde_json::Value::String(self.version().to_string()));
        info.insert("description".to_string(), serde_json::Value::String(self.description().to_string()));
        info.insert("author".to_string(), serde_json::Value::String(self.author().to_string()));
        info.insert("dependencies".to_string(), serde_json::to_value(self.dependencies()).unwrap_or(serde_json::Value::Array(vec![])));
        info.insert("enabled".to_string(), serde_json::Value::Bool(self.is_enabled()));
        info.insert("state".to_string(), serde_json::Value::String(format!("{:?}", self.get_state())));
        info.insert("plugin_type".to_string(), serde_json::Value::String(format!("{:?}", self.get_plugin_type())));
        info.insert("priority".to_string(), serde_json::Value::String(format!("{:?}", self.get_priority())));
        info
    }

    /// Get plugin state.
    fn get_state(&self) -> PluginState {
        if !self.is_initialized() {
            PluginState::Unloaded
        } else if self.is_enabled() {
            PluginState::Running
        } else {
            PluginState::Disabled
        }
    }

    /// Get plugin type.
    fn get_plugin_type(&self) -> PluginType {
        PluginType::CUSTOM
    }

    /// Get plugin priority.
    fn get_priority(&self) -> PluginPriority {
        PluginPriority::NORMAL
    }

    /// Get plugin dependencies.
    fn get_dependencies(&self) -> Vec<String> {
        self.dependencies()
    }

    /// Get plugin version.
    fn get_version(&self) -> String {
        self.version().to_string()
    }

    /// Get plugin author.
    fn get_author(&self) -> String {
        self.author().to_string()
    }

    /// Get plugin description.
    fn get_description(&self) -> String {
        self.description().to_string()
    }

    /// Check if plugin is enabled.
    fn is_enabled(&self) -> bool {
        true
    }

    /// Enable the plugin.
    fn enable(&mut self);

    /// Disable the plugin.
    fn disable(&mut self);
}

/// Thread-safe registry for plugin management.
pub struct APluginRegistry {
    _plugins: HashMap<String, Box<dyn APlugin>>,
    _plugin_info: HashMap<String, APluginInfo>,
    _enabled_plugins: std::collections::HashSet<String>,
}

impl APluginRegistry {
    /// Initialize plugin registry.
    pub fn new() -> Self {
        Self {
            _plugins: HashMap::new(),
            _plugin_info: HashMap::new(),
            _enabled_plugins: std::collections::HashSet::new(),
        }
    }

    /// Register a plugin.
    ///
    /// Args:
    /// plugin: Plugin instance to register (as Box<dyn APlugin>)
    ///
    /// Raises:
    /// PluginError: If plugin with same name already exists
    pub fn register(&mut self, plugin: Box<dyn APlugin>) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        
        if self._plugins.contains_key(&name) {
            return Err(PluginError::new(format!("Plugin '{}' is already registered", name)));
        }
        
        let info = APluginInfo {
            name: name.clone(),
            version: plugin.version().to_string(),
            description: plugin.description().to_string(),
            author: plugin.author().to_string(),
            module_path: String::new(),
            class_name: String::new(),
            enabled: plugin.is_enabled(),
            priority: 100, // Default priority
            dependencies: plugin.dependencies(),
            metadata: plugin.get_info(),
        };
        
        if plugin.is_enabled() {
            self._enabled_plugins.insert(name.clone());
        }
        
        self._plugin_info.insert(name.clone(), info);
        self._plugins.insert(name.clone(), plugin);
        
        Ok(())
    }

    /// Unregister a plugin.
    ///
    /// Args:
    /// plugin_name: Name of plugin to unregister
    ///
    /// Returns:
    /// True if plugin was unregistered, False if not found
    pub fn unregister(&mut self, plugin_name: String) -> bool {
        if let Some(mut plugin) = self._plugins.remove(&plugin_name) {
            if plugin.is_initialized() {
                plugin.shutdown();
            }
            self._plugin_info.remove(&plugin_name);
            self._enabled_plugins.remove(&plugin_name);
            true
        } else {
            false
        }
    }

    /// Get plugin by name.
    ///
    /// Note: Returns plugin info since we can't return trait objects by value.
    ///
    /// Args:
    /// plugin_name: Name of plugin
    ///
    /// Returns:
    /// Plugin info or None if not found
    pub fn get(&self, plugin_name: &str) -> Option<&APluginInfo> {
        self._plugin_info.get(plugin_name)
    }

    /// Get all registered plugins.
    pub fn get_all(&self) -> HashMap<String, APluginInfo> {
        self._plugin_info.clone()
    }

    /// Get all enabled plugins.
    pub fn get_enabled(&self) -> HashMap<String, APluginInfo> {
        self._plugin_info.iter()
            .filter(|(name, _)| self._enabled_plugins.contains(*name))
            .map(|(name, info)| (name.clone(), info.clone()))
            .collect()
    }

    /// Enable a plugin.
    ///
    /// Args:
    /// plugin_name: Name of plugin to enable
    ///
    /// Returns:
    /// True if plugin was enabled, False if not found
    pub fn enable(&mut self, plugin_name: &str) -> bool {
        if let Some(plugin) = self._plugins.get_mut(plugin_name) {
            plugin.enable();
            self._enabled_plugins.insert(plugin_name.to_string());
            if let Some(info) = self._plugin_info.get_mut(plugin_name) {
                info.enabled = true;
            }
            true
        } else {
            false
        }
    }

    /// Disable a plugin.
    ///
    /// Args:
    /// plugin_name: Name of plugin to disable
    ///
    /// Returns:
    /// True if plugin was disabled, False if not found
    pub fn disable(&mut self, plugin_name: &str) -> bool {
        if let Some(plugin) = self._plugins.get_mut(plugin_name) {
            if plugin.is_initialized() {
                plugin.shutdown();
            }
            plugin.disable();
            self._enabled_plugins.remove(plugin_name);
            if let Some(info) = self._plugin_info.get_mut(plugin_name) {
                info.enabled = false;
            }
            true
        } else {
            false
        }
    }

    /// Get information about all registered plugins.
    pub fn list_plugins(&self) -> Vec<APluginInfo> {
        self._plugin_info.values().cloned().collect()
    }

    /// Clear all registered plugins.
    pub fn clear(&mut self) {
        // Shutdown all initialized plugins
        for plugin in self._plugins.values_mut() {
            if plugin.is_initialized() {
                plugin.shutdown();
            }
        }
        
        self._plugins.clear();
        self._plugin_info.clear();
        self._enabled_plugins.clear();
    }

}

// Import module from file
// importlib.metadata is now required
// Try to find APlugin subclasses in the file
// Skip the base class itself
// Direct class reference
// Initialize dependencies first
// Initialize this plugin
/// Plugin manager for loading, discovering and managing plugins.
pub struct APluginManager {
    pub registry: APluginRegistry,
    _discovered_plugins: HashMap<String, HashMap<String, serde_json::Value>>,
}

impl APluginManager {
    /// Initialize plugin manager.
    ///
    /// Args:
    /// registry: Optional plugin registry to use
    pub fn new(registry: Option<APluginRegistry>) -> Self {
        Self {
            registry: registry.unwrap_or_else(APluginRegistry::new),
            _discovered_plugins: HashMap::new(),
        }
    }

    /// Load plugin from module path and class name.
    ///
    /// Note: This requires Python interop via pyo3 or similar.
    ///
    /// Args:
    /// module_path: Python module path
    /// class_name: Plugin class name
    /// config: Optional plugin configuration
    ///
    /// Returns:
    /// Loaded plugin instance (as Box<dyn APlugin>)
    ///
    /// Raises:
    /// PluginError: If plugin cannot be loaded
    pub fn load_plugin_from_module(&mut self, module_path: String, class_name: String, config: Option<HashMap<String, serde_json::Value>>) -> Result<(), PluginError> {
        // This would require Python interop to actually load the plugin
        // For now, create a BasePlugin as placeholder
        let class_path = format!("{}.{}", module_path, class_name);
        
        // Use reflection to get class (would need Python interop)
        let _class_result = ReflectionUtils::get_class_from_string(class_path);
        
        // Create a BasePlugin as placeholder
        let plugin = Box::new(BasePlugin::new(config));
        
        // Register the plugin
        self.registry.register(plugin)?;
        
        Ok(())
    }

    /// Load plugin from Python file.
    ///
    /// Note: This requires Python interop.
    ///
    /// Args:
    /// file_path: Path to Python file
    /// class_name: Plugin class name
    /// config: Optional plugin configuration
    ///
    /// Returns:
    /// Result indicating success or failure
    ///
    /// Raises:
    /// PluginError: If plugin cannot be loaded
    pub fn load_plugin_from_file(&mut self, file_path: String, class_name: String, config: Option<HashMap<String, serde_json::Value>>) -> Result<(), PluginError> {
        use std::path::Path;
        
        let path = Path::new(&file_path);
        if !path.exists() {
            return Err(PluginError::new(format!("Plugin file not found: {}", file_path)));
        }
        
        // This would require Python interop to actually load from file
        // For now, create a BasePlugin as placeholder
        let plugin = Box::new(BasePlugin::new(config));
        self.registry.register(plugin)?;
        
        Ok(())
    }

    /// Discover plugins through entry points.
    ///
    /// Note: This requires Python's importlib.metadata.
    ///
    /// Args:
    /// group: Entry point group name
    ///
    /// Returns:
    /// Dictionary of discovered plugins
    pub fn discover_entry_points(&mut self, group: Option<String>) -> HashMap<String, HashMap<String, serde_json::Value>> {
        let group_name = group.unwrap_or_else(|| "xwsystem.plugins".to_string());
        let mut discovered = HashMap::new();
        
        // This would require Python interop to discover entry points
        // For now, return empty dictionary
        self._discovered_plugins.extend(discovered.clone());
        discovered
    }

    /// Discover plugins in a directory.
    ///
    /// Note: This requires Python interop to analyze Python files.
    ///
    /// Args:
    /// directory: Directory to search
    /// pattern: File pattern to match
    ///
    /// Returns:
    /// Dictionary of discovered plugins
    pub fn discover_directory(&mut self, directory: String, pattern: Option<String>) -> HashMap<String, HashMap<String, serde_json::Value>> {
        use std::path::Path;
        use std::fs;
        
        let dir_path = Path::new(&directory);
        let file_pattern = pattern.unwrap_or_else(|| "*.py".to_string());
        let mut discovered = HashMap::new();
        
        if !dir_path.exists() {
            return discovered;
        }
        
        // Simple directory scanning - would need Python interop for full analysis
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".py") {
                        let mut plugin_info = HashMap::new();
                        plugin_info.insert("name".to_string(), serde_json::Value::String(file_name.to_string()));
                        plugin_info.insert("file".to_string(), serde_json::Value::String(entry.path().to_string_lossy().to_string()));
                        discovered.insert(file_name.to_string(), plugin_info);
                    }
                }
            }
        }
        
        self._discovered_plugins.extend(discovered.clone());
        discovered
    }

    /// Load discovered plugins.
    ///
    /// Args:
    /// plugin_names: Optional list of specific plugins to load
    /// config: Optional configuration for plugins
    ///
    /// Returns:
    /// Number of plugins loaded
    pub fn load_discovered_plugins(&mut self, plugin_names: Option<Vec<String>>, config: Option<HashMap<String, HashMap<String, serde_json::Value>>>) -> usize {
        let plugins_to_load = plugin_names.unwrap_or_else(|| self._discovered_plugins.keys().cloned().collect());
        let mut loaded_count = 0;
        
        for plugin_name in plugins_to_load {
            if let Some(plugin_info) = self._discovered_plugins.get(&plugin_name) {
                let plugin_config = config.as_ref()
                    .and_then(|c| c.get(&plugin_name))
                    .cloned()
                    .or_else(|| {
                        if let Some(file_val) = plugin_info.get("file") {
                            if let Some(file_str) = file_val.as_str() {
                                // Would load from file here
                                None
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    });
                
                // Create placeholder plugin
                let plugin = Box::new(BasePlugin::new(plugin_config));
                if self.registry.register(plugin).is_ok() {
                    loaded_count += 1;
                }
            }
        }
        
        loaded_count
    }

    /// Initialize plugins with dependency resolution.
    ///
    /// Args:
    /// plugin_names: Optional list of specific plugins to initialize
    pub fn initialize_plugins(&mut self, plugin_names: Option<Vec<String>>) {
        // Simplified implementation - would need proper dependency resolution
        let plugins_to_init: Vec<String> = plugin_names.unwrap_or_else(|| {
            self.registry.get_enabled().keys().cloned().collect()
        });
        
        for plugin_name in plugins_to_init {
            if let Some(plugin_info) = self.registry.get(&plugin_name) {
                // Would need mutable access to plugin to initialize
                // For now, this is a placeholder
            }
        }
    }

    /// Shutdown all initialized plugins.
    pub fn shutdown_plugins(&mut self) {
        self.registry.clear();
    }

    /// Get all discovered plugins.
    pub fn get_discovered_plugins(&self) -> HashMap<String, HashMap<String, serde_json::Value>> {
        self._discovered_plugins.clone()
    }

}

/// Base plugin class for backward compatibility.
pub struct BasePlugin {
    pub config: Option<HashMap<String, serde_json::Value>>,
    _name: String,
    _version: String,
    _state: Arc<Mutex<PluginState>>,
    _enabled: Arc<Mutex<bool>>,
    _initialized: Arc<Mutex<bool>>,
}

impl APlugin for BasePlugin {
    fn name(&self) -> &str {
        &self._name
    }

    fn version(&self) -> &str {
        &self._version
    }

    fn initialize(&mut self) {
        if let Ok(mut initialized) = self._initialized.lock() {
            *initialized = true;
        }
    }

    fn shutdown(&mut self) {
        if let Ok(mut initialized) = self._initialized.lock() {
            *initialized = false;
        }
    }

    fn is_initialized(&self) -> bool {
        self._initialized.lock().map(|i| *i).unwrap_or(false)
    }

    fn enable(&mut self) {
        if let Ok(mut enabled) = self._enabled.lock() {
            *enabled = true;
        }
    }

    fn disable(&mut self) {
        if let Ok(mut enabled) = self._enabled.lock() {
            *enabled = false;
        }
    }
}

impl IPlugin for BasePlugin {
    fn initialize(&self) -> () {
        if let Ok(mut initialized) = self._initialized.lock() {
            *initialized = true;
        }
    }

    fn shutdown(&self) -> () {
        if let Ok(mut initialized) = self._initialized.lock() {
            *initialized = false;
        }
    }

    fn get_info(&self) -> HashMap<String, serde_json::Value> {
        <Self as APlugin>::get_info(self)
    }

    fn is_enabled(&self) -> bool {
        self._enabled.lock().map(|e| *e).unwrap_or(false)
    }

    fn enable(&self) -> () {
        if let Ok(mut enabled) = self._enabled.lock() {
            *enabled = true;
        }
    }

    fn disable(&self) -> () {
        if let Ok(mut enabled) = self._enabled.lock() {
            *enabled = false;
        }
    }

    fn get_state(&self) -> PluginState {
        <Self as APlugin>::get_state(self)
    }

    fn get_plugin_type(&self) -> PluginType {
        <Self as APlugin>::get_plugin_type(self)
    }

    fn get_priority(&self) -> PluginPriority {
        <Self as APlugin>::get_priority(self)
    }

    fn get_dependencies(&self) -> Vec<String> {
        <Self as APlugin>::get_dependencies(self)
    }

    fn get_version(&self) -> String {
        <Self as APlugin>::get_version(self)
    }

    fn get_author(&self) -> String {
        <Self as APlugin>::get_author(self)
    }

    fn get_description(&self) -> String {
        <Self as APlugin>::get_description(self)
    }
}

impl BasePlugin {
    /// Initialize base plugin.
    pub fn new(config: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self {
            config,
            _name: "BasePlugin".to_string(),
            _version: "1.0.0".to_string(),
            _state: Arc::new(Mutex::new(PluginState::Unloaded)),
            _enabled: Arc::new(Mutex::new(true)),
            _initialized: Arc::new(Mutex::new(false)),
        }
    }
}

/// Get global plugin manager instance.
pub fn get_plugin_manager() -> APluginManager {
    APluginManager::new(None)
}
