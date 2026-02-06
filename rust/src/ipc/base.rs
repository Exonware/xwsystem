// #exonware/xwsystem/rust/src/ipc/base.rs
//exonware/xwsystem/ipc/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! IPC module base classes - abstract classes for inter-process communication functionality.


use crate::contracts::{MessageType, ProcessState, QueueType, SharedMemoryType};

/// Abstract base class for message queue operations.
pub trait AMessageQueueBase {
    /// Connect to message queue.
    fn connect(&self) -> ();

    /// Disconnect from message queue.
    fn disconnect(&self) -> ();

    /// Check if connected to message queue.
    fn is_connected(&self) -> bool;

    /// Send message to queue.
    fn send(&self, message: serde_json::Value, message_type: MessageType) -> bool;

    /// Receive message from queue.
    fn receive(&self, timeout: Option<i64>) -> Option<serde_json::Value>;

    /// Peek at next message without removing it.
    fn peek(&self) -> Option<serde_json::Value>;

    /// Get queue size.
    fn size(&self) -> i64;

    /// Check if queue is empty.
    fn is_empty(&self) -> bool;

    /// Check if queue is full.
    fn is_full(&self) -> bool;

    /// Clear all messages from queue.
    fn clear(&self) -> ();

}

/// Abstract base class for pipe operations.
pub trait APipeBase {
    /// Create pipe.
    fn create(&self) -> ();

    /// Connect to pipe.
    fn connect(&self) -> ();

    /// Disconnect from pipe.
    fn disconnect(&self) -> ();

    /// Check if connected to pipe.
    fn is_connected(&self) -> bool;

    /// Write data to pipe.
    fn write(&self, data: String) -> i64;

    /// Read data from pipe.
    fn read(&self, size: Option<i64>) -> String;

    /// Close read end of pipe.
    fn close_read(&self) -> ();

    /// Close write end of pipe.
    fn close_write(&self) -> ();

    /// Close pipe.
    fn close(&self) -> ();

}

/// Abstract base class for shared memory operations.
pub trait ASharedMemoryBase {
    /// Create shared memory segment.
    fn create(&self) -> bool;

    /// Attach to shared memory segment.
    fn attach(&self) -> bool;

    /// Detach from shared memory segment.
    fn detach(&self) -> ();

    /// Destroy shared memory segment.
    fn destroy(&self) -> bool;

    /// Check if attached to shared memory.
    fn is_attached(&self) -> bool;

    /// Write data to shared memory.
    fn write(&self, data: Vec<u8>, offset: i64) -> i64;

    /// Read data from shared memory.
    fn read(&self, size: i64, offset: i64) -> Vec<u8>;

    /// Get shared memory size.
    fn get_size(&self) -> i64;

    /// Lock shared memory for exclusive access.
    fn lock(&self) -> ();

    /// Unlock shared memory.
    fn unlock(&self) -> ();

}

/// Abstract base class for process management.
pub trait AProcessManagerBase {
    /// Start new process.
    fn start_process(&self, name: String, command: Vec<String>) -> bool;

    /// Stop process.
    fn stop_process(&self, name: String, timeout: Option<i64>) -> bool;

    /// Kill process.
    fn kill_process(&self, name: String) -> bool;

    /// Get process by name.
    fn get_process(&self, name: String) -> Option<serde_json::Value>;

    /// List all managed processes.
    fn list_processes(&self) -> Vec<String>;

    /// Get process state.
    fn get_process_state(&self, name: String) -> ProcessState;

    /// Check if process is running.
    fn is_process_running(&self, name: String) -> bool;

    /// Get process PID.
    fn get_process_pid(&self, name: String) -> Option<i64>;

    /// Get process output.
    fn get_process_output(&self, name: String) -> Option<String>;

    /// Get process error output.
    fn get_process_error(&self, name: String) -> Option<String>;

}

/// Abstract base class for process pool management.
pub trait AProcessPoolBase {
    /// Initialize process pool.
    fn initialize(&self) -> ();

    /// Shutdown process pool.
    fn shutdown(&self) -> ();

    /// Get available process from pool.
    fn get_process(&self) -> Option<serde_json::Value>;

    /// Return process to pool.
    fn return_process(&self, process: serde_json::Value) -> ();

    /// Execute task using process from pool.
    fn execute_task(&self, task: String) -> serde_json::Value;

    /// Get current pool size.
    fn get_pool_size(&self) -> i64;

    /// Get number of available processes.
    fn get_available_count(&self) -> i64;

    /// Get number of busy processes.
    fn get_busy_count(&self) -> i64;

    /// Check if pool is full.
    fn is_pool_full(&self) -> bool;

}

/// Base IPC class for backward compatibility.
///
/// Provides a simple interface for IPC operations.
pub struct BaseIPC {
    _initialized: bool,
}

impl BaseIPC {
    /// Initialize base IPC.
    pub fn new() -> Self {
        Self {
            _initialized: false,
        }
    }

    /// Initialize IPC components.
    pub fn initialize(&mut self) -> bool {
        self._initialized = true;
        true
    }

    /// Cleanup IPC components.
    pub fn cleanup(&mut self) -> bool {
        self._initialized = false;
        true
    }

    /// Check if IPC is initialized.
    pub fn is_initialized(&self) -> bool {
        self._initialized
    }

    /// Shutdown IPC components.
    pub fn shutdown(&mut self) -> bool {
        self.cleanup()
    }
}
