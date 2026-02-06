// #exonware/xwsystem/rust/src/threading/defs.rs
//exonware/xwsystem/threading/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Threading types and enums for XWSystem.


use serde::{Serialize, Deserialize};

use crate::shared::defs::{LockType};

/// Thread states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadState {
    Initialized,
    Running,
    Waiting,
    Blocked,
    Terminated,
}

/// Thread priorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreadPriority {
    Lowest,
    Low,
    Normal,
    High,
    Highest,
}

/// Async operation states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsyncState {
    Pending,
    Running,
    Completed,
    Cancelled,
    Failed,
}

/// Concurrency modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcurrencyMode {
    Sequential,
    Parallel,
    Concurrent,
    Async,
}
