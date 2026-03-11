// #exonware/xwsystem/rust/src/runtime/defs.rs
//exonware/xwsystem/runtime/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Runtime types and enums for XWSystem.

/// Environment types.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentType {
    Development,
    Testing,
    Staging,
    Production,
    Unknown,
}

/// Platform types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformType {
    Windows,
    Linux,
    Macos,
    Unknown,
}

/// Python version types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PythonVersion {
    #[serde(rename = "3.8")]
    Python38,
    #[serde(rename = "3.9")]
    Python39,
    #[serde(rename = "3.10")]
    Python310,
    #[serde(rename = "3.11")]
    Python311,
    #[serde(rename = "3.12")]
    Python312,
    Unknown,
}

/// Runtime modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeMode {
    Normal,
    Debug,
    Release,
    Profile,
    Optimized,
}
