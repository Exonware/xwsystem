// #exonware/xwsystem/rust/src/ipc/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! IPC module contracts - interfaces and enums for inter-process communication.


use std::collections::HashMap;

use crate::defs::{IPCType, MessageQueueType, MessageType, ProcessState, QueueType, SharedMemoryType};

// Aliases for backward compatibility

/// Interface for message queue operations.
pub trait IMessageQueue {
    /// Put message in queue.
    async fn put(&self, message: serde_json::Value, message_type: MessageType) -> ();

    /// Get message from queue.
    async fn get(&self, timeout: Option<f64>) -> serde_json::Value;

    /// Get message without waiting.
    async fn get_nowait(&self) -> serde_json::Value;

    /// Check if queue is empty.
    fn empty(&self) -> bool;

    /// Check if queue is full.
    fn full(&self) -> bool;

    /// Get queue size.
    fn size(&self) -> i64;

}

/// Interface for pipe operations.
pub trait IPipe {
    /// Send data through pipe.
    async fn send(&self, data: serde_json::Value) -> ();

    /// Receive data from pipe.
    async fn recv(&self, timeout: Option<f64>) -> serde_json::Value;

    /// Receive data without waiting.
    async fn recv_nowait(&self) -> serde_json::Value;

    /// Close pipe.
    fn close(&self) -> ();

    /// Check if pipe is closed.
    fn closed(&self) -> bool;

}

/// Interface for shared memory operations.
pub trait ISharedMemory {
    /// Create shared memory segment.
    fn create(&self, name: String, size: i64) -> ();

    /// Attach to shared memory segment.
    fn attach(&self, name: String) -> ();

    /// Detach from shared memory segment.
    fn detach(&self) -> ();

    /// Unlink shared memory segment.
    fn unlink(&self) -> ();

    /// Read from shared memory.
    fn read(&self, offset: i64, size: Option<i64>) -> Vec<u8>;

    /// Write to shared memory.
    fn write(&self, data: Vec<u8>, offset: i64) -> ();

}

/// Interface for process management.
pub trait IProcessManager {
    /// Create new process.
    fn create_process(&self, target: fn(), args: (), kwargs: Option<HashMap<String, serde_json::Value>>) -> Process;

    /// Start process.
    fn start_process(&self, process: Process) -> ();

    /// Stop process.
    fn stop_process(&self, process: Process, timeout: Option<f64>) -> ();

    /// Get process state.
    fn get_process_state(&self, process: Process) -> ProcessState;

    /// Check if process is alive.
    fn is_process_alive(&self, process: Process) -> bool;

}

/// Interface for process pool operations.
pub trait IProcessPool {
    /// Submit task to pool.
    fn submit(&self, func: fn()) -> serde_json::Value;

    /// Submit async task to pool.
    async fn submit_async(&self, func: fn()) -> serde_json::Value;

    /// Map function over iterable.
    fn map(&self, func: fn(), iterable: Vec<serde_json::Value>) -> Vec<serde_json::Value>;

    /// Map function over iterable asynchronously.
    async fn map_async(&self, func: fn(), iterable: Vec<serde_json::Value>) -> Vec<serde_json::Value>;

    /// Close process pool.
    fn close(&self) -> ();

    /// Join all processes.
    fn join(&self) -> ();

}
