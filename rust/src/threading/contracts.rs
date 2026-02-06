// #exonware/xwsystem/rust/src/threading/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Threading protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::defs::{AsyncState, ConcurrencyMode, LockType, ThreadPriority, ThreadState};

// ============================================================================

// ASYNC INTERFACES

// ============================================================================

// ============================================================================

// THREAD SAFETY INTERFACES

// ============================================================================

// ============================================================================

// THREAD POOL INTERFACES

// ============================================================================

// ============================================================================

// CONCURRENCY CONTROL INTERFACES

// ============================================================================

// ============================================================================

// SYNCHRONIZATION INTERFACES

// ============================================================================

// ============================================================================

// DEADLOCK DETECTION INTERFACES

// ============================================================================

// ============================================================================

// THREAD MONITORING INTERFACES

// ============================================================================

// ============================================================================

// ASYNC CONTEXT MANAGER INTERFACES

// ============================================================================

/// Interface for lockable objects.
///
/// Enforces consistent locking behavior across XWSystem.
pub trait ILockable {
    /// Acquire lock.
    /// Args:
    /// blocking: Whether to block until lock is acquired
    /// timeout: Timeout in seconds
    /// Returns:
    /// True if lock acquired
    fn acquire_lock(&self, blocking: bool, timeout: Option<f64>) -> bool;

    /// Release lock.
    fn release_lock(&self) -> ();

    /// Check if object is locked.
    /// Returns:
    /// True if locked
    fn is_locked(&self) -> bool;

    /// Try to acquire lock without blocking.
    /// Args:
    /// timeout: Timeout in seconds
    /// Returns:
    /// True if lock acquired
    fn try_lock(&self, timeout: Option<f64>) -> bool;

    /// Get lock type.
    /// Returns:
    /// Lock type
    fn get_lock_type(&self) -> LockType;

    /// Get lock owner thread ID.
    /// Returns:
    /// Thread ID of lock owner or None
    fn get_lock_owner(&self) -> Option<String>;

    /// Get lock acquisition count.
    /// Returns:
    /// Number of times lock has been acquired
    fn get_lock_count(&self) -> i64;

}

/// Interface for async operations.
///
/// Enforces consistent async behavior across XWSystem.
pub trait IAsync {
    /// Execute method asynchronously.
    /// Args:
    /// *args: Positional arguments
    /// **kwargs: Keyword arguments
    /// Returns:
    /// Method result
    async fn async_method(&self) -> serde_json::Value;

    /// Await coroutine result.
    /// Args:
    /// coroutine: Coroutine to await
    /// Returns:
    /// Coroutine result
    fn await_result(&self, coroutine: Coroutine) -> serde_json::Value;

    /// Check if object supports async operations.
    /// Returns:
    /// True if async supported
    fn is_async(&self) -> bool;

    /// Get future for async operation.
    /// Returns:
    /// Future object
    fn get_future(&self) -> String;

    /// Get async operation state.
    /// Returns:
    /// Current async state
    fn get_async_state(&self) -> AsyncState;

    /// Cancel async operation.
    /// Returns:
    /// True if cancelled
    fn cancel_async(&self) -> bool;

    /// Check if async operation is completed.
    /// Returns:
    /// True if completed
    fn is_async_completed(&self) -> bool;

    /// Get async operation result.
    /// Returns:
    /// Async operation result
    fn get_async_result(&self) -> serde_json::Value;

}

/// Interface for thread-safe objects.
///
/// Enforces consistent thread safety across XWSystem.
pub trait IThreadSafe {
    /// Execute method in thread-safe manner.
    /// Args:
    /// *args: Positional arguments
    /// **kwargs: Keyword arguments
    /// Returns:
    /// Method result
    fn thread_safe_method(&self) -> serde_json::Value;

    /// Get current thread ID.
    /// Returns:
    /// Thread ID
    fn get_thread_id(&self) -> i64;

    /// Check if object is thread-safe.
    /// Returns:
    /// True if thread-safe
    fn is_thread_safe(&self) -> bool;

    /// Get number of active threads.
    /// Returns:
    /// Number of active threads
    fn get_thread_count(&self) -> i64;

    /// Get thread information.
    /// Returns:
    /// Thread information dictionary
    fn get_thread_info(&self) -> HashMap<String, serde_json::Value>;

    /// Wait for all threads to complete.
    /// Args:
    /// timeout: Timeout in seconds
    /// Returns:
    /// True if all threads completed
    fn wait_for_threads(&self, timeout: Option<f64>) -> bool;

    /// Interrupt all threads.
    fn interrupt_threads(&self) -> ();

    /// Join all threads.
    fn join_threads(&self) -> ();

}

/// Interface for thread pool management.
///
/// Enforces consistent thread pool behavior across XWSystem.
pub trait IThreadPool {
    /// Submit task to thread pool.
    /// Args:
    /// func: Function to execute
    /// *args: Function arguments
    /// **kwargs: Function keyword arguments
    /// Returns:
    /// Task result or future
    fn submit_task(&self, func: fn()) -> serde_json::Value;

    /// Submit async task to thread pool.
    /// Args:
    /// func: Async function to execute
    /// *args: Function arguments
    /// **kwargs: Function keyword arguments
    /// Returns:
    /// Future object
    fn submit_async_task(&self, func: fn()) -> String;

    /// Get thread pool size.
    /// Returns:
    /// Number of threads in pool
    fn get_pool_size(&self) -> i64;

    /// Set thread pool size.
    /// Args:
    /// size: New pool size
    fn set_pool_size(&self, size: i64) -> ();

    /// Get number of active threads.
    /// Returns:
    /// Number of active threads
    fn get_active_count(&self) -> i64;

    /// Get task queue size.
    /// Returns:
    /// Number of queued tasks
    fn get_queue_size(&self) -> i64;

    /// Shutdown thread pool.
    /// Args:
    /// wait: Whether to wait for tasks to complete
    fn shutdown(&self, wait: bool) -> ();

    /// Check if thread pool is shutdown.
    /// Returns:
    /// True if shutdown
    fn is_shutdown(&self) -> bool;

}

/// Interface for concurrency control.
///
/// Enforces consistent concurrency control across XWSystem.
pub trait IConcurrencyControl {
    /// Acquire resource for exclusive access.
    /// Args:
    /// resource_id: Resource identifier
    /// timeout: Timeout in seconds
    /// Returns:
    /// True if resource acquired
    fn acquire_resource(&self, resource_id: String, timeout: Option<f64>) -> bool;

    /// Release resource.
    /// Args:
    /// resource_id: Resource identifier
    fn release_resource(&self, resource_id: String) -> ();

    /// Check if resource is available.
    /// Args:
    /// resource_id: Resource identifier
    /// Returns:
    /// True if available
    fn is_resource_available(&self, resource_id: String) -> bool;

    /// Get resource owner.
    /// Args:
    /// resource_id: Resource identifier
    /// Returns:
    /// Owner thread ID or None
    fn get_resource_owner(&self, resource_id: String) -> Option<String>;

    /// Wait for resource to become available.
    /// Args:
    /// resource_id: Resource identifier
    /// timeout: Timeout in seconds
    /// Returns:
    /// True if resource became available
    fn wait_for_resource(&self, resource_id: String, timeout: Option<f64>) -> bool;

    /// Get concurrency mode.
    /// Returns:
    /// Current concurrency mode
    fn get_concurrency_mode(&self) -> ConcurrencyMode;

    /// Set concurrency mode.
    /// Args:
    /// mode: Concurrency mode to set
    fn set_concurrency_mode(&self, mode: ConcurrencyMode) -> ();

}

/// Interface for synchronization primitives.
///
/// Enforces consistent synchronization across XWSystem.
pub trait ISynchronization {
    /// Wait for condition.
    /// Args:
    /// timeout: Timeout in seconds
    /// Returns:
    /// True if condition met
    fn wait(&self, timeout: Option<f64>) -> bool;

    /// Notify waiting threads.
    fn notify(&self) -> ();

    /// Notify all waiting threads.
    fn notify_all(&self) -> ();

    /// Signal condition.
    fn signal(&self) -> ();

    /// Reset synchronization primitive.
    fn reset(&self) -> ();

    /// Check if condition is set.
    /// Returns:
    /// True if set
    fn is_set(&self) -> bool;

    /// Get number of waiting threads.
    /// Returns:
    /// Number of waiting threads
    fn get_waiting_count(&self) -> i64;

}

/// Interface for deadlock detection.
///
/// Enforces consistent deadlock detection across XWSystem.
pub trait IDeadlockDetection {
    /// Detect deadlocks.
    /// Returns:
    /// List of deadlock information
    fn detect_deadlock(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Check if system is deadlocked.
    /// Returns:
    /// True if deadlocked
    fn is_deadlocked(&self) -> bool;

    /// Resolve deadlock.
    /// Args:
    /// deadlock_info: Deadlock information
    /// Returns:
    /// True if resolved
    fn resolve_deadlock(&self, deadlock_info: HashMap<String, serde_json::Value>) -> bool;

    /// Get lock dependency graph.
    /// Returns:
    /// Lock dependency graph
    fn get_lock_graph(&self) -> HashMap<String, Vec<String>>;

    /// Add lock dependency.
    /// Args:
    /// resource1: First resource
    /// resource2: Second resource
    fn add_lock_dependency(&self, resource1: String, resource2: String) -> ();

    /// Remove lock dependency.
    /// Args:
    /// resource1: First resource
    /// resource2: Second resource
    fn remove_lock_dependency(&self, resource1: String, resource2: String) -> ();

    /// Get deadlock prevention mode.
    /// Returns:
    /// True if prevention enabled
    fn get_deadlock_prevention_mode(&self) -> bool;

    /// Set deadlock prevention mode.
    /// Args:
    /// enabled: Whether to enable prevention
    fn set_deadlock_prevention_mode(&self, enabled: bool) -> ();

}

/// Interface for thread monitoring.
///
/// Enforces consistent thread monitoring across XWSystem.
pub trait IThreadMonitor {
    /// Get thread statistics.
    /// Returns:
    /// Thread statistics dictionary
    fn get_thread_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Get list of all threads.
    /// Returns:
    /// List of thread information
    fn get_thread_list(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Get thread by ID.
    /// Args:
    /// thread_id: Thread ID
    /// Returns:
    /// Thread information or None
    fn get_thread_by_id(&self, thread_id: i64) -> Option<HashMap<String, serde_json::Value>>;

    /// Monitor thread performance.
    /// Args:
    /// thread_id: Thread ID
    /// Returns:
    /// Performance metrics
    fn monitor_thread_performance(&self, thread_id: i64) -> HashMap<String, serde_json::Value>;

    /// Detect thread leaks.
    /// Returns:
    /// List of leaked thread IDs
    fn detect_thread_leaks(&self) -> Vec<i64>;

    /// Cleanup thread leaks.
    /// Args:
    /// thread_ids: Thread IDs to cleanup
    /// Returns:
    /// Number of threads cleaned up
    fn cleanup_thread_leaks(&self, thread_ids: Vec<i64>) -> i64;

    /// Get thread priority.
    /// Args:
    /// thread_id: Thread ID
    /// Returns:
    /// Thread priority
    fn get_thread_priority(&self, thread_id: i64) -> ThreadPriority;

    /// Set thread priority.
    /// Args:
    /// thread_id: Thread ID
    /// priority: New priority
    /// Returns:
    /// True if set successfully
    fn set_thread_priority(&self, thread_id: i64, priority: ThreadPriority) -> bool;

}

/// Interface for async context managers.
///
/// Enforces consistent async context management across XWSystem.
pub trait IAsyncContextManager {
    /// Check if async context is active.
    /// Returns:
    /// True if active
    fn is_async_context_active(&self) -> bool;

    /// Get async context information.
    /// Returns:
    /// Context information dictionary
    fn get_async_context_info(&self) -> HashMap<String, serde_json::Value>;

}
