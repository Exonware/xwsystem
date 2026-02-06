// #exonware/xwsystem/rust/src/caching/defs.rs
//exonware/xwsystem/caching/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Caching types and enums for XWSystem.

// Least Frequently Used
/// Cache eviction policies.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CachePolicy {
    Lru,
    Lfu,
    Fifo,
    Lifo,
    Ttl,
    Size,
    Random,
}

/// Cache status states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheStatus {
    Active,
    Inactive,
    Paused,
    Error,
    Maintenance,
}

/// Cache events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheEvent {
    Hit,
    Miss,
    Set,
    Delete,
    Expire,
    Evict,
    Clear,
    Error,
}

/// Cache levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheLevel {
    L1,
    L2,
    L3,
    Distributed,
}
