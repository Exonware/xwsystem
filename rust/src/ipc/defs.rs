// #exonware/xwsystem/rust/src/ipc/defs.rs
//exonware/xwsystem/ipc/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! IPC types and enums for XWSystem.

/// IPC communication types.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IPCType {
    Pipe,
    Queue,
    #[serde(rename = "shared_memory")]
    SharedMemory,
    Socket,
    File,
}

/// Message types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageType {
    Data,
    Control,
    Heartbeat,
    Error,
    Shutdown,
}

/// Process states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessState {
    Created,
    Starting,
    Running,
    Stopping,
    Stopped,
    Failed,
}

/// Queue types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueType {
    Fifo,
    Lifo,
    Priority,
    Bounded,
}

/// Shared memory types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SharedMemoryType {
    Mmap,
    Shm,
    Posix,
    Windows,
    #[serde(rename = "system_v")]
    SystemV,
}

/// Types of message queues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageQueueType {
    #[serde(rename = "thread_safe")]
    ThreadSafe,
    #[serde(rename = "process_safe")]
    ProcessSafe,
    Async,
}
