// #exonware/xwsystem/rust/src/plugins/mod.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! XSystem Plugins Package
//! 
//! Provides plugin discovery, registration and management system
//! with support for entry points and dynamic loading.

pub mod base;
pub mod contracts;
pub mod defs;
pub mod errors;

pub use base::{APlugin, APluginInfo, APluginManager, APluginRegistry, BasePlugin, get_plugin_manager};
pub use contracts::IPlugin;
pub use defs::{HookType, PluginEvent, PluginPriority, PluginState, PluginType};
pub use errors::PluginError;
