// #exonware/xwsystem/rust/src/config/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Configuration protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::config::defs::{AdvancedPerformanceMode, ConfigFormat, ConfigPriority, ConfigSource, ConfigType, PerformanceMode};
use crate::shared::defs::{LogLevel, ValidationLevel};

// ============================================================================

// SETTINGS INTERFACES

// ============================================================================

// ============================================================================

// ENVIRONMENT INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION VALIDATION INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION SOURCE INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION MANAGER INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION WATCHER INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION TEMPLATE INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION SECRET INTERFACES

// ============================================================================

// ============================================================================

// CONFIGURATION PROTOCOLS

// ============================================================================

// Aliases for backward compatibility

// Aliases for backward compatibility

// Aliases for backward compatibility

/// Interface for configurable objects.
///
/// Enforces consistent configuration behavior across XWSystem.
pub trait IConfigurable {
    /// Configure object with options.
    fn configure(&mut self);

    /// Get current configuration.
    /// Returns:
    /// Current configuration dictionary
    fn get_config(&self) -> HashMap<String, serde_json::Value>;

    /// Reset configuration to defaults.
    fn reset_config(&mut self);

    /// Update single configuration value.
    /// Args:
    /// key: Configuration key
    /// value: Configuration value
    fn update_config(&mut self, key: String, value: serde_json::Value);

    /// Check if configuration key exists.
    /// Args:
    /// key: Configuration key
    /// Returns:
    /// True if key exists
    fn has_config(&self, key: &str) -> bool;

    /// Remove configuration key.
    /// Args:
    /// key: Configuration key to remove
    /// Returns:
    /// True if removed
    fn remove_config(&mut self, key: &str) -> bool;

    /// Merge configuration with existing.
    /// Args:
    /// config: Configuration to merge
    /// priority: Merge priority
    fn merge_config(&mut self, config: HashMap<String, serde_json::Value>, priority: ConfigPriority);
}

/// Interface for settings management.
///
/// Enforces consistent settings behavior across XWSystem.
pub trait ISettings {
    /// Get setting value.
    /// Args:
    /// key: Setting key
    /// default: Default value if not found
    /// Returns:
    /// Setting value
    fn get_setting(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set setting value.
    /// Args:
    /// key: Setting key
    /// value: Setting value
    fn set_setting(&mut self, key: String, value: serde_json::Value);

    /// Check if setting exists.
    /// Args:
    /// key: Setting key
    /// Returns:
    /// True if setting exists
    fn has_setting(&self, key: &str) -> bool;

    /// Remove setting.
    /// Args:
    /// key: Setting key to remove
    /// Returns:
    /// True if removed
    fn remove_setting(&mut self, key: &str) -> bool;

    /// Get all settings.
    /// Returns:
    /// Dictionary of all settings
    fn get_all_settings(&self) -> HashMap<String, serde_json::Value>;

    /// Clear all settings.
    fn clear_settings(&mut self);

    /// Load settings from source.
    /// Args:
    /// source: Settings source (file path, dict, etc.)
    fn load_settings(&mut self, source: String);

    /// Save settings to destination.
    /// Args:
    /// destination: Save destination
    /// Returns:
    /// True if saved successfully
    fn save_settings(&self, destination: &str) -> bool;
}

/// Interface for environment variable management.
///
/// Enforces consistent environment behavior across XWSystem.
pub trait IEnvironment {
    /// Get environment variable.
    /// Args:
    /// key: Environment variable key
    /// default: Default value if not found
    /// Returns:
    /// Environment variable value
    fn get_env(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set environment variable.
    /// Args:
    /// key: Environment variable key
    /// value: Environment variable value
    fn set_env(&mut self, key: String, value: serde_json::Value);

    /// Check if environment variable exists.
    /// Args:
    /// key: Environment variable key
    /// Returns:
    /// True if exists
    fn has_env(&self, key: &str) -> bool;

    /// Remove environment variable.
    /// Args:
    /// key: Environment variable key to remove
    /// Returns:
    /// True if removed
    fn remove_env(&mut self, key: &str) -> bool;

    /// Load environment variables from file.
    /// Args:
    /// file_path: Environment file path
    fn load_env(&mut self, file_path: &str);

    /// Save environment variables to file.
    /// Args:
    /// file_path: Environment file path
    /// Returns:
    /// True if saved successfully
    fn save_env(&self, file_path: &str) -> bool;

    /// Get all environment variables.
    /// Returns:
    /// Dictionary of environment variables
    fn get_all_env(&self) -> HashMap<String, String>;

    /// Clear all environment variables.
    fn clear_env(&mut self);
}

/// Interface for configuration validation.
///
/// Enforces consistent configuration validation across XWSystem.
pub trait IConfigValidator {
    /// Validate configuration.
    /// Args:
    /// config: Configuration to validate
    /// Returns:
    /// True if valid
    fn validate_config(&self, config: &HashMap<String, serde_json::Value>) -> bool;

    /// Get configuration validation errors.
    /// Args:
    /// config: Configuration to validate
    /// Returns:
    /// List of validation error messages
    fn get_validation_errors(&self, config: &HashMap<String, serde_json::Value>) -> Vec<String>;

    /// Add validation rule for configuration key.
    /// Args:
    /// key: Configuration key
    /// rule: Validation function
    /// message: Error message if validation fails
    fn add_validation_rule(&mut self, key: String, rule: fn(&serde_json::Value) -> bool, message: String);

    /// Remove validation rule for configuration key.
    /// Args:
    /// key: Configuration key
    /// Returns:
    /// True if removed
    fn remove_validation_rule(&mut self, key: &str) -> bool;

    /// Set validation level.
    /// Args:
    /// level: Validation level
    fn set_validation_level(&mut self, level: ValidationLevel);

    /// Get current validation level.
    /// Returns:
    /// Current validation level
    fn get_validation_level(&self) -> ValidationLevel;
}

/// Interface for configuration sources.
///
/// Enforces consistent configuration source behavior across XWSystem.
pub trait IConfigSource {
    /// Load configuration from source.
    /// Returns:
    /// Configuration dictionary
    fn load_config(&self) -> HashMap<String, serde_json::Value>;

    /// Save configuration to source.
    /// Args:
    /// config: Configuration to save
    /// Returns:
    /// True if saved successfully
    fn save_config(&mut self, config: HashMap<String, serde_json::Value>) -> bool;

    /// Get configuration source type.
    /// Returns:
    /// Source type
    fn get_source_type(&self) -> ConfigSource;

    /// Get source information.
    /// Returns:
    /// Source information dictionary
    fn get_source_info(&self) -> HashMap<String, serde_json::Value>;

    /// Check if source is available.
    /// Returns:
    /// True if available
    fn is_available(&self) -> bool;

    /// Get source priority.
    /// Returns:
    /// Source priority
    fn get_priority(&self) -> ConfigPriority;
}

/// Interface for configuration management.
///
/// Enforces consistent configuration management across XWSystem.
pub trait IConfigManager {
    /// Add configuration source.
    /// Args:
    /// source: Configuration source to add
    fn add_source(&mut self, source: Box<dyn IConfigSource>);

    /// Remove configuration source.
    /// Args:
    /// source_type: Source type to remove
    /// Returns:
    /// True if removed
    fn remove_source(&mut self, source_type: ConfigSource) -> bool;

    /// Load configuration from all sources.
    /// Returns:
    /// Merged configuration dictionary
    fn load_all_configs(&self) -> HashMap<String, serde_json::Value>;

    /// Save configuration to all sources.
    /// Args:
    /// config: Configuration to save
    /// Returns:
    /// True if saved to all sources
    fn save_all_configs(&mut self, config: HashMap<String, serde_json::Value>) -> bool;

    /// Get configuration value from all sources.
    /// Args:
    /// key: Configuration key
    /// default: Default value
    /// Returns:
    /// Configuration value
    fn get_config_value(&self, key: &str, default: Option<serde_json::Value>) -> serde_json::Value;

    /// Set configuration value in all sources.
    /// Args:
    /// key: Configuration key
    /// value: Configuration value
    fn set_config_value(&mut self, key: String, value: serde_json::Value);

    /// Reload configuration from all sources.
    fn reload_config(&mut self);

    /// Get all configuration sources.
    /// Returns:
    /// List of configuration sources
    fn get_sources(&self) -> Vec<Box<dyn IConfigSource>>;
}

/// Interface for configuration change watching.
///
/// Enforces consistent configuration watching across XWSystem.
pub trait IConfigWatcher {
    /// Start watching for configuration changes.
    fn start_watching(&mut self);

    /// Stop watching for configuration changes.
    fn stop_watching(&mut self);

    /// Check if currently watching.
    /// Returns:
    /// True if watching
    fn is_watching(&self) -> bool;

    /// Add callback for configuration changes.
    /// Args:
    /// callback: Function to call on changes (key, old_value, new_value)
    fn add_change_callback(&mut self, callback: Box<dyn Fn(&str, &serde_json::Value, &serde_json::Value)>);

    /// Remove change callback.
    /// Args:
    /// callback_id: Callback identifier to remove
    /// Returns:
    /// True if removed
    fn remove_change_callback(&mut self, callback_id: usize) -> bool;

    /// Get list of watched configuration keys.
    /// Returns:
    /// List of watched keys
    fn get_watched_keys(&self) -> Vec<String>;

    /// Start watching specific configuration key.
    /// Args:
    /// key: Configuration key to watch
    fn watch_key(&mut self, key: String);

    /// Stop watching specific configuration key.
    /// Args:
    /// key: Configuration key to stop watching
    fn unwatch_key(&mut self, key: &str);
}

/// Interface for configuration templates.
///
/// Enforces consistent configuration templating across XWSystem.
pub trait IConfigTemplate {
    /// Create configuration template.
    /// Args:
    /// config: Configuration to template
    /// Returns:
    /// Template string
    fn create_template(&self, config: &HashMap<String, serde_json::Value>) -> String;

    /// Apply template with values.
    /// Args:
    /// template: Template string
    /// values: Values to apply
    /// Returns:
    /// Configuration dictionary
    fn apply_template(&self, template: &str, values: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value>;

    /// Validate template syntax.
    /// Args:
    /// template: Template to validate
    /// Returns:
    /// True if valid
    fn validate_template(&self, template: &str) -> bool;

    /// Get template variables.
    /// Args:
    /// template: Template string
    /// Returns:
    /// List of variable names
    fn get_template_variables(&self, template: &str) -> Vec<String>;

    /// Save template to file.
    /// Args:
    /// template: Template string
    /// path: File path
    /// Returns:
    /// True if saved successfully
    fn save_template(&self, template: &str, path: &str) -> bool;

    /// Load template from file.
    /// Args:
    /// path: File path
    /// Returns:
    /// Template string
    fn load_template(&self, path: &str) -> String;
}

/// Interface for configuration secrets management.
///
/// Enforces consistent secrets handling across XWSystem.
pub trait IConfigSecrets {
    /// Encrypt secret value.
    /// Args:
    /// value: Secret value to encrypt
    /// Returns:
    /// Encrypted secret
    fn encrypt_secret(&self, value: &str) -> String;

    /// Decrypt secret value.
    /// Args:
    /// encrypted_value: Encrypted secret
    /// Returns:
    /// Decrypted secret
    fn decrypt_secret(&self, encrypted_value: &str) -> String;

    /// Check if configuration key is secret.
    /// Args:
    /// key: Configuration key
    /// Returns:
    /// True if secret
    fn is_secret(&self, key: &str) -> bool;

    /// Mark configuration key as secret.
    /// Args:
    /// key: Configuration key to mark
    fn mark_as_secret(&mut self, key: String);

    /// Unmark configuration key as secret.
    /// Args:
    /// key: Configuration key to unmark
    fn unmark_as_secret(&mut self, key: &str);

    /// Get list of secret configuration keys.
    /// Returns:
    /// List of secret keys
    fn get_secret_keys(&self) -> Vec<String>;

    /// Sanitize configuration by hiding secrets.
    /// Args:
    /// config: Configuration to sanitize
    /// Returns:
    /// Sanitized configuration
    fn sanitize_config(&self, config: &HashMap<String, serde_json::Value>) -> HashMap<String, serde_json::Value>;
}

/// Protocol for objects that support configuration (simpler interface than IConfigurable).
pub trait IConfigurableSimple {
    /// Configure object with parameters.
    fn configure(&mut self);

    /// Get current configuration.
    fn get_config(&self) -> HashMap<String, serde_json::Value>;
}
