// #exonware/xwsystem/rust/src/base.rs
//exonware/xwsystem/src/exonware/xwsystem/base.py
//! Root-level abstract base classes for XWSystem.
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Jan-2025
//! 
//! This module provides root-level abstract base classes that serve as
//! foundation classes for all XWSystem components.


use std::collections::HashMap;

use crate::config::base::{AConfigBase, AConfigManagerBase, AConfigValidatorBase, ALoggingConfigBase, APerformanceConfigBase, BaseConfig};
use crate::contracts::{IConfigurable, ISystemComponent};
use crate::defs::{CoreMode, CorePriority, CoreState};
use crate::shared::base::{AConfigurationBase, ACoreBase, AOperationBase, AResourceManagerBase, AValidationBase, BaseCore};

// ============================================================================

// RE-EXPORTED BASE CLASSES FROM SUBMODULES

// ============================================================================

// ============================================================================

// RE-EXPORTED BASE CLASSES FROM SUBMODULES

// ============================================================================

// ============================================================================

// RE-EXPORTED BASE CLASSES FROM SUBMODULES

// ============================================================================

// Re-export base classes from submodules (import as needed)

// These are available but not all may be needed at root level

// ============================================================================

// ============================================================================

/// Abstract base class for XWSystem framework.
///
/// Provides common functionality for all XWSystem components.
pub trait AXWSystemBase {
    /// Get system mode.
    // Python decorators: @property
    fn mode(&self) -> CoreMode;

    /// Get system state.
    // Python decorators: @property
    fn state(&self) -> CoreState;

    /// Initialize the component.
    fn initialize(&mut self);

    /// Shutdown the component.
    fn shutdown(&mut self);

    /// Check if component is initialized.
    fn is_initialized(&self) -> bool;

    /// Check if component is shutdown.
    fn is_shutdown(&self) -> bool;
}

/// Abstract base class for system components.
///
/// All XWSystem components should inherit from this class.
pub trait ASystemComponent: AXWSystemBase + ISystemComponent {
    /// Get component name.
    fn get_name(&self) -> String;

    /// Get component version.
    fn get_version(&self) -> String;

    /// Get component configuration.
    fn get_config(&self) -> HashMap<String, serde_json::Value>;

    /// Set component configuration.
    fn set_config(&mut self, config: HashMap<String, serde_json::Value>);

    /// Initialize component.
    fn initialize(&mut self);

    /// Shutdown component.
    fn shutdown(&mut self);
}

/// Abstract base class for configurable system components.
///
/// Combines ASystemComponent with IConfigurable interface.
pub trait AConfigurableComponent: ASystemComponent + IConfigurable {
    /// Configure component with options.
    fn configure(&mut self);

    /// Get current configuration.
    fn get_config(&self) -> HashMap<String, serde_json::Value>;

    /// Reset configuration to defaults.
    fn reset_config(&mut self);

    /// Update single configuration value.
    fn update_config(&mut self, key: String, value: serde_json::Value);

    /// Check if configuration key exists.
    fn has_config(&self, key: &str) -> bool;

    /// Remove configuration key.
    fn remove_config(&mut self, key: &str) -> bool;

    /// Merge configuration with existing.
    fn merge_config(&mut self, config: HashMap<String, serde_json::Value>, priority: Option<serde_json::Value>);
}

/// Abstract base class for monitored system components.
///
/// Provides monitoring capabilities for system components.
pub trait AMonitoredComponent: ASystemComponent {
    /// Start monitoring.
    fn start_monitoring(&mut self);

    /// Stop monitoring.
    fn stop_monitoring(&mut self);

    /// Get component metrics.
    fn get_metrics(&self) -> HashMap<String, serde_json::Value>;
}

/// Abstract base class for secure system components.
///
/// Provides security capabilities for system components.
pub trait ASecureComponent: ASystemComponent {
    /// Validate security of data.
    fn validate_security(&self, data: &serde_json::Value) -> bool;

    /// Check if action is permitted on resource.
    fn check_permissions(&self, action: &str, resource: &str) -> bool;
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use AXWSystemBase;
pub use ASystemComponent;
pub use AConfigurableComponent;
pub use AMonitoredComponent;
pub use ASecureComponent;
pub use ACoreBase;
pub use AResourceManagerBase;
pub use AConfigurationBase;
pub use AValidationBase;
pub use AOperationBase;
pub use BaseCore;
pub use AConfigBase;
pub use ALoggingConfigBase;
pub use APerformanceConfigBase;
pub use AConfigValidatorBase;
pub use AConfigManagerBase;
pub use BaseConfig;
