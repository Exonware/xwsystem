// #exonware/xwsystem/rust/src/plugins/defs.rs
//exonware/xwsystem/plugins/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Plugins types and enums for XWSystem.

/// Plugin states.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginState {
    Unloaded,
    Loading,
    Loaded,
    Initializing,
    Initialized,
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
    Disabled,
}

/// Plugin types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginType {
    Core,
    Extension,
    Middleware,
    Service,
    Utility,
    Custom,
}

/// Plugin priorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginPriority {
    Lowest,
    Low,
    Normal,
    High,
    Highest,
    Critical,
}

/// Hook types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HookType {
    Pre,
    Post,
    Around,
    Filter,
    Action,
}

/// Plugin events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PluginEvent {
    Loaded,
    Unloaded,
    Initialized,
    Started,
    Stopped,
    Error,
    Configured,
    Disabled,
    Enabled,
}
