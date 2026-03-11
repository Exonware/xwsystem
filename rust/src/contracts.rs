// #exonware/xwsystem/rust/src/contracts.rs
//exonware/xwsystem/src/exonware/xwsystem/contracts.py
//! Root-level Protocol interfaces for XWSystem.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Jan-2025
//! 
//! This module provides root-level Protocol interfaces that aggregate
//! the main interfaces from submodules for consistent API design.


use std::collections::HashMap;

use crate::config::contracts::{IConfigManager, IConfigSecrets, IConfigSource, IConfigTemplate, IConfigValidator, IConfigWatcher, IConfigurable, IConfigurableSimple, IEnvironment, ISettings};
use crate::defs::{CoreMode, CorePriority, CoreState};
use crate::patterns::contracts::{IAdapter, IAggregate, IArchitectural, IBuilder, IChainHandler, ICommand, IConcurrency, IContextManager, IDecorator, IFacade, IFactory as IPatternFactory, IHandler, IHandlerFactory, IIterator, IMediator, IMemento, IObserver, IPrototype, IProxy, IRegistry, ISpecification, IState, IStrategy, IValueObject, IVisitor};
use crate::shared::contracts::{ICloneable, IComparable, IContainer, ICore, IFactory, IID, IIterable, ILifecycle, IMetadata, INative, IStringable};

// ============================================================================

// ============================================================================

/// Root interface for XWSystem framework.
///
/// Provides unified access to all XWSystem capabilities.
pub trait IXWSystem {
    /// Get XWSystem version.
    fn get_version(&self) -> String;

    /// Get current system mode.
    fn get_mode(&self) -> CoreMode;

    /// Get current system state.
    fn get_state(&self) -> CoreState;

    /// Initialize XWSystem.
    fn initialize(&mut self);

    /// Shutdown XWSystem.
    fn shutdown(&mut self);

    /// Check if XWSystem is initialized.
    fn is_initialized(&self) -> bool;

    /// Get configuration manager.
    fn get_config(&self) -> serde_json::Value;

    /// Get security validator.
    fn get_security(&self) -> serde_json::Value;

    /// Get monitoring instance.
    fn get_monitoring(&self) -> serde_json::Value;

    /// Get cache manager.
    fn get_cache(&self) -> serde_json::Value;

    /// Get plugin manager.
    fn get_plugins(&self) -> serde_json::Value;

}

/// Interface for system components.
///
/// All XWSystem components should implement this interface.
pub trait ISystemComponent {
    /// Get component name.
    fn get_name(&self) -> String;

    /// Get component version.
    fn get_version(&self) -> String;

    /// Initialize component.
    fn initialize(&mut self);

    /// Shutdown component.
    fn shutdown(&mut self);

    /// Check if component is initialized.
    fn is_initialized(&self) -> bool;

    /// Get component configuration.
    fn get_config(&self) -> HashMap<String, serde_json::Value>;

    /// Set component configuration.
    fn set_config(&mut self, config: HashMap<String, serde_json::Value>);

}

/// Interface for extensible components.
///
/// Components that support plugins and extensions.
pub trait IExtensible {
    /// Register a plugin.
    fn register_plugin(&mut self, plugin: serde_json::Value);

    /// Unregister a plugin.
    fn unregister_plugin(&mut self, plugin_id: &str);

    /// Get all registered plugins.
    fn get_plugins(&self) -> Vec<serde_json::Value>;

    /// Check if plugin is registered.
    fn has_plugin(&self, plugin_id: &str) -> bool;

}

/// Combined interface for configurable system components.
///
/// Combines ISystemComponent and IConfigurable for convenience.
pub trait IConfigurableComponent: ISystemComponent + IConfigurable {
}

/// Combined interface for monitored system components.
///
/// Combines ISystemComponent with monitoring capabilities.
pub trait IMonitoredComponent: ISystemComponent {
}

/// Combined interface for secure system components.
///
/// Combines ISystemComponent with security capabilities.
pub trait ISecureComponent: ISystemComponent {
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use IXWSystem;
pub use ISystemComponent;
pub use IExtensible;
pub use IConfigurableComponent;
pub use IMonitoredComponent;
pub use ISecureComponent;
pub use IID;
pub use IStringable;
pub use INative;
pub use ICloneable;
pub use IComparable;
pub use IIterable;
pub use IContainer;
pub use IMetadata;
pub use ILifecycle;
pub use IFactory;
pub use ICore;
pub use IConfigurable;
pub use ISettings;
pub use IEnvironment;
pub use IConfigValidator;
pub use IConfigSource;
pub use IConfigManager;
pub use IConfigWatcher;
pub use IConfigTemplate;
pub use IConfigSecrets;
pub use IConfigurableSimple;
pub use IHandler;
pub use IHandlerFactory;
pub use IContextManager;
pub use IRegistry;
pub use IStrategy;
pub use IPatternFactory;
pub use IObserver;
pub use ICommand;
pub use IState;
pub use IBuilder;
pub use IPrototype;
pub use IAdapter;
pub use IDecorator;
pub use IProxy;
pub use IFacade;
pub use IChainHandler;
pub use IMediator;
pub use IMemento;
pub use IVisitor;
pub use IIterator;
pub use IConcurrency;
pub use IArchitectural;
pub use ISpecification;
pub use IValueObject;
pub use IAggregate;
