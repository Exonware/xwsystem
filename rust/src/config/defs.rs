// #exonware/xwsystem/rust/src/config/defs.rs
//exonware/xwsystem/config/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Config types and enums for XWSystem.


use serde::{Serialize, Deserialize};

/// Configuration sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigSource {
    File,
    Environment,
    Defaults,
    #[serde(rename = "command_line")]
    CommandLine,
    Database,
    Api,
    Memory,
}

/// Configuration formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Ini,
    Env,
    Xml,
    Python,
}

/// Configuration priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigPriority {
    Lowest,
    Low,
    Normal,
    High,
    Highest,
    Critical,
}

/// Configuration types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigType {
    Dict,
    File,
    Environment,
    Database,
    Api,
}

/// Performance modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceMode {
    Fast,
    Balanced,
    #[serde(rename = "memory_optimized")]
    MemoryOptimized,
}

// Follow global system settings
// Automatic selection based on data characteristics
// Inherit from parent object
// Use default balanced settings
// Prioritize speed over memory
// Prioritize memory efficiency
// Use specific custom settings
// Runtime adaptation based on performance monitoring
// Smart dual-phase: fast cruise + intelligent deep-dive
/// Advanced performance optimization modes for system operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdvancedPerformanceMode {
    Global,
    Auto,
    Parent,
    Default,
    Fast,
    Optimized,
    Manual,
    Adaptive,
    #[serde(rename = "dual_adaptive")]
    DualAdaptive,
}
