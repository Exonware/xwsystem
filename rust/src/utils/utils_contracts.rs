// #exonware/xwsystem/rust/src/utils/utils_contracts.rs
//exonware/xwsystem/utils/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Utils types and enums for XWSystem.


use serde::{Serialize, Deserialize};

use crate::shared::defs::{PathType};

/// Lazy loading strategies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LazyLoadStrategy {
    #[serde(rename = "on_demand")]
    OnDemand,
    Cached,
    Preload,
    Background,
}

/// Lazy loading modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LazyLoadMode {
    Eager,
    Lazy,
    #[serde(rename = "on_demand")]
    OnDemand,
    Cached,
    Preload,
    Background,
}

/// Utility types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UtilityType {
    Path,
    Config,
    Resource,
    Cache,
    Logging,
    Validation,
    Serialization,
    Encryption,
    Compression,
    Custom,
}

/// Resource types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    File,
    Memory,
    Network,
    Database,
    Cache,
    Thread,
    Process,
    Connection,
    Custom,
}
