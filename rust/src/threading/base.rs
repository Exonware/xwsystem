// #exonware/xwsystem/rust/src/threading/base.rs
//exonware/xwsystem/threading/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Threading module base classes - abstract classes for threading functionality.


use std::collections::HashMap;

use crate::contracts::{AsyncPrimitiveType, ConcurrencyMode, LockType, ThreadState};

/// Abstract base class for thread operations.
pub trait AThreadBase {
    /// Start thread.
    fn start(&self) -> ();

    /// Stop thread.
    fn stop(&self) -> ();

    /// Join thread.
    fn join(&self, timeout: Option<f64>) -> ();

    /// Check if thread is alive.
    fn is_alive(&self) -> bool;

    /// Check if thread is running.
    fn is_running(&self) -> bool;

    /// Get thread state.
    fn get_state(&self) -> ThreadState;

    /// Get thread identifier.
    fn get_ident(&self) -> i64;

    /// Set thread target function.
    fn set_target(&self, target: fn()) -> ();

    /// Get thread result.
    fn get_result(&self) -> serde_json::Value;

    /// Get thread exception.
    fn get_exception(&self) -> Option<Exception>;

}

/// Abstract base class for lock operations.
pub trait ALockBase {
    /// Acquire lock.
    fn acquire(&self, blocking: bool, timeout: Option<f64>) -> bool;

    /// Release lock.
    fn release(&self) -> ();

    /// Check if lock is locked.
    fn is_locked(&self) -> bool;

    /// Check if lock is owned by current thread.
    fn is_owned(&self) -> bool;

    /// Get lock owner thread ID.
    fn get_owner(&self) -> Option<i64>;

    /// Get number of waiting threads.
    fn get_waiting_count(&self) -> i64;

    /// Try to acquire lock without blocking.
    fn try_acquire(&self) -> bool;

}

/// Abstract base class for async primitives.
pub trait AAsyncPrimitiveBase {
    /// Wait for primitive.
    async fn wait(&self, timeout: Option<f64>) -> bool;

    /// Set primitive.
    async fn set(&self) -> ();

    /// Clear primitive.
    async fn clear(&self) -> ();

    /// Check if primitive is set.
    async fn is_set(&self) -> bool;

    /// Wait for condition.
    async fn wait_for(&self, condition: fn(), timeout: Option<f64>) -> bool;

    /// Notify waiting coroutines.
    async fn notify(&self) -> ();

    /// Notify all waiting coroutines.
    async fn notify_all(&self) -> ();

    /// Get number of waiting coroutines.
    async fn get_waiting_count(&self) -> i64;

}

/// Abstract base class for safe factory operations.
pub trait ASafeFactoryBase {
    /// Create thread-safe instance.
    fn create_instance(&self, instance_type: String) -> serde_json::Value;

    /// Get instance by ID.
    fn get_instance(&self, instance_id: String) -> Option<serde_json::Value>;

    /// Remove instance.
    fn remove_instance(&self, instance_id: String) -> bool;

    /// List all instance IDs.
    fn list_instances(&self) -> Vec<String>;

    /// Clear all instances.
    fn clear_instances(&self) -> ();

    /// Get instance count.
    fn get_instance_count(&self) -> i64;

    /// Check if instance exists.
    fn is_instance_exists(&self, instance_id: String) -> bool;

    /// Get instance information.
    fn get_instance_info(&self, instance_id: String) -> HashMap<String, serde_json::Value>;

}

/// Abstract base class for thread pool operations.
pub trait AThreadPoolBase {
    /// Submit task to thread pool.
    fn submit(&self, func: fn()) -> serde_json::Value;

    /// Map function over iterable using thread pool.
    fn map(&self, func: fn(), iterable: Vec<serde_json::Value>) -> Vec<serde_json::Value>;

    /// Shutdown thread pool.
    fn shutdown(&self, wait: bool) -> ();

    /// Check if thread pool is shutdown.
    fn is_shutdown(&self) -> bool;

    /// Get number of active threads.
    fn get_active_count(&self) -> i64;

    /// Get number of pending tasks.
    fn get_pending_count(&self) -> i64;

    /// Get number of completed tasks.
    fn get_completed_count(&self) -> i64;

    /// Get number of worker threads.
    fn get_worker_count(&self) -> i64;

}

/// Abstract base class for concurrency management.
pub trait AConcurrencyManagerBase {
    /// Create new thread.
    fn create_thread(&self, name: String, target: fn()) -> AThreadBase;

    /// Create new lock.
    fn create_lock(&self, name: String, lock_type: LockType) -> ALockBase;

    /// Create new async primitive.
    fn create_async_primitive(&self, name: String, primitive_type: AsyncPrimitiveType) -> AAsyncPrimitiveBase;

    /// Get thread by name.
    fn get_thread(&self, name: String) -> Option<AThreadBase>;

    /// Get lock by name.
    fn get_lock(&self, name: String) -> Option<ALockBase>;

    /// Get async primitive by name.
    fn get_async_primitive(&self, name: String) -> Option<AAsyncPrimitiveBase>;

    /// Remove thread.
    fn remove_thread(&self, name: String) -> bool;

    /// Remove lock.
    fn remove_lock(&self, name: String) -> bool;

    /// Remove async primitive.
    fn remove_async_primitive(&self, name: String) -> bool;

    /// Detect potential deadlocks.
    fn detect_deadlocks(&self) -> Vec<Vec<String>>;

    /// Enable deadlock detection.
    fn enable_deadlock_detection(&self) -> ();

    /// Disable deadlock detection.
    fn disable_deadlock_detection(&self) -> ();

    /// Get concurrency statistics.
    fn get_concurrency_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Shutdown all managed resources.
    fn shutdown_all(&self) -> ();

}
