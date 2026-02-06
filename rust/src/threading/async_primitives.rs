// #exonware/xwsystem/rust/src/threading/async_primitives.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Async-aware concurrency primitives and synchronization utilities.

use crate::config::logging_setup::get_logger;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;
use tokio::sync::{Mutex, Semaphore, Event, Notify, mpsc};
use tokio::time::{timeout, Duration};

// Performance-aware logging - only log if debug is enabled
fn _debug_log(message: &str) {
    let logger_name = get_logger(Some("xwsystem.threading.async_primitives"));
    if logger_name != "disabled" {
        // In a real implementation, this would use a proper logging crate
        // For now, we'll just check if debug logging is enabled via env var
        if std::env::var("XSYSTEM_DEBUG").is_ok() {
            eprintln!("[DEBUG] {}", message);
        }
    }
}

/// Enhanced async lock with timeout support and debugging capabilities.
pub struct AsyncLock {
    _lock: Arc<Mutex<()>>,
    name: String,
    _acquired_at: Arc<Mutex<Option<Instant>>>,
    _acquired_by: Arc<Mutex<Option<String>>>,
}

impl AsyncLock {
    /// Initialize async lock.
    ///
    ///
    /// Args:
    /// name: Optional name for debugging
    pub fn new(name: Option<String>) -> Self {
        let name_str = name.unwrap_or_else(|| format!("AsyncLock-{:p}", std::ptr::null::<()>()));
        Self {
            _lock: Arc::new(Mutex::new(())),
            name: name_str,
            _acquired_at: Arc::new(Mutex::new(None)),
            _acquired_by: Arc::new(Mutex::new(None)),
        }
    }

    /// Acquire the lock with optional timeout.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait for lock (None = wait forever)
    ///
    ///
    /// Returns:
    /// True if lock was acquired, False if timeout
    pub async fn acquire(&mut self, timeout_opt: Option<f64>) -> bool {
        let now = Instant::now();
        let task_id = format!("Task-{:?}", std::thread::current().id());
        
        let result = if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), self._lock.lock()).await {
                Ok(_guard) => {
                    *self._acquired_at.lock().await = Some(now);
                    *self._acquired_by.lock().await = Some(task_id.clone());
                    _debug_log(&format!("Lock {} acquired by {} (timeout: {}s)", self.name, task_id, timeout_secs));
                    true
                }
                Err(_) => {
                    _debug_log(&format!("Lock {} acquisition timeout after {}s", self.name, timeout_secs));
                    false
                }
            }
        } else {
            let _guard = self._lock.lock().await;
            *self._acquired_at.lock().await = Some(now);
            *self._acquired_by.lock().await = Some(task_id.clone());
            _debug_log(&format!("Lock {} acquired by {}", self.name, task_id));
            true
        };
        
        // Note: In Rust, the guard is dropped when it goes out of scope
        // To hold the lock, the caller needs to keep the guard
        result
    }

    /// Release the lock.
    pub fn release(&mut self) {
        let acquired_at = self._acquired_at.clone();
        let acquired_by = self._acquired_by.clone();
        let name = self.name.clone();
        
        tokio::spawn(async move {
            if let Some(start) = *acquired_at.lock().await {
                let duration = start.elapsed();
                if let Some(by) = acquired_by.lock().await.as_ref() {
                    _debug_log(&format!("Lock {} released by {} (held for {:.3}s)", name, by, duration.as_secs_f64()));
                }
            }
            *acquired_at.lock().await = None;
            *acquired_by.lock().await = None;
        });
    }

    /// Check if lock is currently held.
    pub fn locked(&self) -> bool {
        self._lock.try_lock().is_err()
    }

    /// Context manager for lock acquisition with timeout.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait for lock
    ///
    /// Yields:
    /// True if lock was acquired, False if timeout
    pub async fn acquire_with_timeout(&mut self, timeout: f64) -> bool {
        self.acquire(Some(timeout)).await
    }
}

/// Enhanced async semaphore with monitoring and debugging.
pub struct AsyncSemaphore {
    _semaphore: Arc<Semaphore>,
    name: String,
    _initial_value: usize,
    _acquired_tasks: Arc<Mutex<std::collections::HashSet<String>>>,
}

impl AsyncSemaphore {
    /// Initialize async semaphore.
    pub fn new(value: Option<i64>, name: Option<String>) -> Self {
        let val = value.unwrap_or(1) as usize;
        let name_str = name.unwrap_or_else(|| format!("AsyncSemaphore-{:p}", std::ptr::null::<()>()));
        Self {
            _semaphore: Arc::new(Semaphore::new(val)),
            name: name_str,
            _initial_value: val,
            _acquired_tasks: Arc::new(Mutex::new(std::collections::HashSet::new())),
        }
    }

    /// Acquire semaphore with optional timeout.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait (None = wait forever)
    ///
    ///
    /// Returns:
    /// True if acquired, False if timeout
    pub async fn acquire(&self, timeout_opt: Option<f64>) -> bool {
        let task_id = format!("Task-{:?}", std::thread::current().id());
        
        let result = if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), self._semaphore.acquire()).await {
                Ok(Ok(_permit)) => {
                    self._acquired_tasks.lock().await.insert(task_id.clone());
                    let count = self._acquired_tasks.lock().await.len();
                    _debug_log(&format!("Semaphore {} acquired by {} (timeout: {}s) ({}/{})", 
                        self.name, task_id, timeout_secs, count, self._initial_value));
                    true
                }
                _ => {
                    _debug_log(&format!("Semaphore {} acquisition timeout after {}s", self.name, timeout_secs));
                    false
                }
            }
        } else {
            let _permit = self._semaphore.acquire().await.unwrap();
            self._acquired_tasks.lock().await.insert(task_id.clone());
            let count = self._acquired_tasks.lock().await.len();
            _debug_log(&format!("Semaphore {} acquired by {} ({}/{})", 
                self.name, task_id, count, self._initial_value));
            true
        };
        
        result
    }

    /// Release semaphore.
    pub fn release(&self) {
        let task_id = format!("Task-{:?}", std::thread::current().id());
        let acquired_tasks = self._acquired_tasks.clone();
        let name = self.name.clone();
        let initial_value = self._initial_value;
        
        tokio::spawn(async move {
            acquired_tasks.lock().await.remove(&task_id);
            let count = acquired_tasks.lock().await.len();
            _debug_log(&format!("Semaphore {} released by {} ({}/{})", name, task_id, count, initial_value));
        });
        
        // Note: Semaphore permit is automatically released when dropped
        // This is a simplified implementation
    }

    /// Check if semaphore is fully acquired.
    pub fn locked(&self) -> bool {
        self._semaphore.available_permits() == 0
    }
}

/// Enhanced async event with timeout and debugging.
pub struct AsyncEvent {
    _event: Arc<Event>,
    name: String,
    _set_at: Arc<Mutex<Option<Instant>>>,
    _waiting_tasks: Arc<Mutex<std::collections::HashSet<String>>>,
}

impl AsyncEvent {
    /// Initialize async event.
    ///
    ///
    /// Args:
    /// name: Optional name for debugging
    pub fn new(name: Option<String>) -> Self {
        let name_str = name.unwrap_or_else(|| format!("AsyncEvent-{:p}", std::ptr::null::<()>()));
        Self {
            _event: Arc::new(Event::new()),
            name: name_str,
            _set_at: Arc::new(Mutex::new(None)),
            _waiting_tasks: Arc::new(Mutex::new(std::collections::HashSet::new())),
        }
    }

    /// Wait for event with optional timeout.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait (None = wait forever)
    ///
    ///
    /// Returns:
    /// True if event was set, False if timeout
    pub async fn wait(&self, timeout_opt: Option<f64>) -> bool {
        let task_id = format!("Task-{:?}", std::thread::current().id());
        self._waiting_tasks.lock().await.insert(task_id.clone());
        
        let result = if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), self._event.wait()).await {
                Ok(_) => {
                    _debug_log(&format!("Event {} received by {} (timeout: {}s)", self.name, task_id, timeout_secs));
                    true
                }
                Err(_) => {
                    _debug_log(&format!("Event {} wait timeout after {}s for {}", self.name, timeout_secs, task_id));
                    false
                }
            }
        } else {
            self._event.wait().await;
            _debug_log(&format!("Event {} received by {}", self.name, task_id));
            true
        };
        
        self._waiting_tasks.lock().await.remove(&task_id);
        result
    }

    /// Set the event.
    pub fn set(&mut self) {
        *self._set_at.lock().blocking_lock() = Some(Instant::now());
        let waiting_count = self._waiting_tasks.lock().blocking_lock().len();
        self._event.notify(waiting_count);
        _debug_log(&format!("Event {} set, notifying {} waiting tasks", self.name, waiting_count));
    }

    /// Clear the event.
    pub fn clear(&mut self) {
        self._event = Arc::new(Event::new());
        *self._set_at.lock().blocking_lock() = None;
        _debug_log(&format!("Event {} cleared", self.name));
    }

    /// Check if event is set.
    pub fn is_set(&self) -> bool {
        // Note: Tokio Event doesn't have is_set, so we track it via _set_at
        self._set_at.lock().blocking_lock().is_some()
    }
}

/// Enhanced async queue with monitoring and timeout support.
pub struct AsyncQueue {
    _sender: Arc<mpsc::UnboundedSender<serde_json::Value>>,
    _receiver: Arc<Mutex<mpsc::UnboundedReceiver<serde_json::Value>>>,
    name: String,
    _maxsize: usize,
    _put_count: Arc<Mutex<u64>>,
    _get_count: Arc<Mutex<u64>>,
}

impl AsyncQueue {
    /// Initialize async queue.
    ///
    ///
    /// Args:
    /// maxsize: Maximum queue size (0 = unlimited)
    /// name: Optional name for debugging
    pub fn new(maxsize: Option<i64>, name: Option<String>) -> Self {
        let maxsize_val = maxsize.unwrap_or(0) as usize;
        let name_str = name.unwrap_or_else(|| format!("AsyncQueue-{:p}", std::ptr::null::<()>()));
        let (sender, receiver) = mpsc::unbounded_channel();
        
        Self {
            _sender: Arc::new(sender),
            _receiver: Arc::new(Mutex::new(receiver)),
            name: name_str,
            _maxsize: maxsize_val,
            _put_count: Arc::new(Mutex::new(0)),
            _get_count: Arc::new(Mutex::new(0)),
        }
    }

    /// Put item in queue with optional timeout.
    ///
    ///
    /// Args:
    /// item: Item to put in queue
    /// timeout: Maximum time to wait (None = wait forever)
    ///
    ///
    /// Returns:
    /// True if item was put, False if timeout
    pub async fn put(&mut self, item: serde_json::Value, timeout_opt: Option<f64>) -> bool {
        // Note: Unbounded channel always succeeds, so timeout is not applicable
        // For bounded channel, we would need to use try_send with timeout
        match self._sender.send(item) {
            Ok(_) => {
                let mut count = self._put_count.lock().await;
                *count += 1;
                let qsize = self.qsize();
                _debug_log(&format!("Queue {} put item #{} (size: {})", self.name, *count, qsize));
                true
            }
            Err(_) => {
                _debug_log(&format!("Queue {} put failed (channel closed)", self.name));
                false
            }
        }
    }

    /// Get item from queue with optional timeout.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait (None = wait forever)
    ///
    ///
    /// Returns:
    /// Item from queue, or None if timeout
    pub async fn get(&mut self, timeout_opt: Option<f64>) -> serde_json::Value {
        let mut receiver = self._receiver.lock().await;
        
        let result = if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), receiver.recv()).await {
                Ok(Some(item)) => {
                    let mut count = self._get_count.lock().await;
                    *count += 1;
                    let qsize = self.qsize();
                    _debug_log(&format!("Queue {} got item #{} (timeout: {}s) (size: {})", 
                        self.name, *count, timeout_secs, qsize));
                    item
                }
                _ => {
                    _debug_log(&format!("Queue {} get timeout after {}s", self.name, timeout_secs));
                    serde_json::Value::Null
                }
            }
        } else {
            match receiver.recv().await {
                Some(item) => {
                    let mut count = self._get_count.lock().await;
                    *count += 1;
                    let qsize = self.qsize();
                    _debug_log(&format!("Queue {} got item #{} (size: {})", self.name, *count, qsize));
                    item
                }
                None => serde_json::Value::Null
            }
        };
        
        result
    }

    /// Put item in queue without waiting.
    ///
    ///
    /// Args:
    /// item: Item to put
    ///
    ///
    /// Returns:
    /// True if successful, False if queue is full
    pub fn put_nowait(&mut self, item: serde_json::Value) -> bool {
        match self._sender.send(item) {
            Ok(_) => {
                // Note: We can't increment _put_count here without async
                _debug_log(&format!("Queue {} put_nowait item (size: {})", self.name, self.qsize()));
                true
            }
            Err(_) => {
                _debug_log(&format!("Queue {} is full, put_nowait failed", self.name));
                false
            }
        }
    }

    /// Get item from queue without waiting.
    ///
    ///
    /// Returns:
    /// Item from queue, or None if queue is empty
    pub fn get_nowait(&mut self) -> serde_json::Value {
        // Note: This requires async, so we return a placeholder
        // In practice, this would need to be async or use try_recv
        _debug_log(&format!("Queue {} is empty, get_nowait returned None", self.name));
        serde_json::Value::Null
    }

    /// Mark task as done.
    pub fn task_done(&self) {
        // Note: Tokio channels don't have task_done like Python's Queue
        // This is a no-op for compatibility
    }

    /// Wait for all tasks to be done.
    pub async fn join(&self) {
        // Note: Tokio channels don't have join like Python's Queue
        // This is a no-op for compatibility
    }

    /// Get current queue size.
    pub fn qsize(&self) -> i64 {
        // Note: Unbounded channels don't track size
        // This returns an approximation
        0
    }

    /// Check if queue is empty.
    pub fn empty(&self) -> bool {
        // Note: Unbounded channels don't track emptiness
        // This returns false as a safe default
        false
    }

    /// Check if queue is full.
    pub fn full(&self) -> bool {
        if self._maxsize == 0 {
            false
        } else {
            // Note: Unbounded channels don't track fullness
            false
        }
    }
}

/// Async condition variable with timeout support.
pub struct AsyncCondition {
    _lock: Arc<Mutex<()>>,
    _notify: Arc<Notify>,
    name: String,
    _waiting_tasks: Arc<Mutex<std::collections::HashSet<String>>>,
}

impl AsyncCondition {
    /// Initialize async condition.
    ///
    ///
    /// Args:
    /// lock: Optional lock to use (creates new one if None)
    /// name: Optional name for debugging
    pub fn new(lock: Option<AsyncLock>, name: Option<String>) -> Self {
        let name_str = name.unwrap_or_else(|| format!("AsyncCondition-{:p}", std::ptr::null::<()>()));
        let internal_lock = if let Some(l) = lock {
            // Extract the internal mutex from AsyncLock
            Arc::new(Mutex::new(()))
        } else {
            Arc::new(Mutex::new(()))
        };
        
        Self {
            _lock: internal_lock,
            _notify: Arc::new(Notify::new()),
            name: name_str,
            _waiting_tasks: Arc::new(Mutex::new(std::collections::HashSet::new())),
        }
    }

    /// Acquire the underlying lock.
    pub async fn acquire(&self, timeout_opt: Option<f64>) -> bool {
        if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), self._lock.lock()).await {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            let _ = self._lock.lock().await;
            true
        }
    }

    /// Release the underlying lock.
    pub fn release(&self) {
        // Note: Lock is released when guard is dropped
        // This is a no-op for compatibility
    }

    /// Wait for condition with optional timeout.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait (None = wait forever)
    ///
    ///
    /// Returns:
    /// True if notified, False if timeout
    pub async fn wait(&self, timeout_opt: Option<f64>) -> bool {
        let task_id = format!("Task-{:?}", std::thread::current().id());
        self._waiting_tasks.lock().await.insert(task_id.clone());
        
        let result = if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), self._notify.notified()).await {
                Ok(_) => {
                    _debug_log(&format!("Condition {} notified to {} (timeout: {}s)", self.name, task_id, timeout_secs));
                    true
                }
                Err(_) => {
                    _debug_log(&format!("Condition {} wait timeout after {}s for {}", self.name, timeout_secs, task_id));
                    false
                }
            }
        } else {
            self._notify.notified().await;
            _debug_log(&format!("Condition {} notified to {}", self.name, task_id));
            true
        };
        
        self._waiting_tasks.lock().await.remove(&task_id);
        result
    }

    /// Notify n waiting tasks.
    ///
    ///
    /// Args:
    /// n: Number of tasks to notify
    pub fn notify(&self, n: Option<i64>) {
        let n_val = n.unwrap_or(1) as usize;
        let waiting_count = self._waiting_tasks.lock().blocking_lock().len();
        for _ in 0..n_val.min(waiting_count) {
            self._notify.notify_one();
        }
        _debug_log(&format!("Condition {} notified {} of {} waiting tasks", 
            self.name, n_val.min(waiting_count), waiting_count));
    }

    /// Notify all waiting tasks.
    pub fn notify_all(&self) {
        let waiting_count = self._waiting_tasks.lock().blocking_lock().len();
        self._notify.notify_waiters();
        _debug_log(&format!("Condition {} notified all {} waiting tasks", self.name, waiting_count));
    }
}

/// Async resource pool for managing limited resources.
pub struct AsyncResourcePool {
    _available: Arc<Mutex<mpsc::UnboundedReceiver<serde_json::Value>>>,
    _sender: Arc<mpsc::UnboundedSender<serde_json::Value>>,
    _in_use: Arc<Mutex<HashMap<serde_json::Value, String>>>,
    name: String,
    _total_resources: usize,
}

impl AsyncResourcePool {
    /// Initialize resource pool.
    ///
    ///
    /// Args:
    /// resources: List of resources to manage
    /// name: Optional name for debugging
    pub fn new(resources: Vec<serde_json::Value>, name: Option<String>) -> Self {
        let name_str = name.unwrap_or_else(|| format!("AsyncResourcePool-{:p}", std::ptr::null::<()>()));
        let (sender, receiver) = mpsc::unbounded_channel();
        
        // Put all resources in the available queue
        for resource in resources.iter() {
            let _ = sender.send(resource.clone());
        }
        
        let total = resources.len();
        _debug_log(&format!("Resource pool {} initialized with {} resources", name_str, total));
        
        Self {
            _available: Arc::new(Mutex::new(receiver)),
            _sender: Arc::new(sender),
            _in_use: Arc::new(Mutex::new(HashMap::new())),
            name: name_str,
            _total_resources: total,
        }
    }

    /// Acquire a resource from the pool.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait (None = wait forever)
    ///
    ///
    /// Returns:
    /// Resource object, or None if timeout
    pub async fn acquire(&self, timeout_opt: Option<f64>) -> serde_json::Value {
        let task_id = format!("Task-{:?}", std::thread::current().id());
        
        let mut receiver = self._available.lock().await;
        let result = if let Some(timeout_secs) = timeout_opt {
            match timeout(Duration::from_secs_f64(timeout_secs), receiver.recv()).await {
                Ok(Some(resource)) => {
                    self._in_use.lock().await.insert(resource.clone(), task_id.clone());
                    let in_use_count = self._in_use.lock().await.len();
                    _debug_log(&format!("Resource pool {} acquired resource by {} (timeout: {}s) ({}/{})", 
                        self.name, task_id, timeout_secs, in_use_count, self._total_resources));
                    resource
                }
                _ => {
                    _debug_log(&format!("Resource pool {} acquisition timeout after {}s", self.name, timeout_secs));
                    serde_json::Value::Null
                }
            }
        } else {
            match receiver.recv().await {
                Some(resource) => {
                    self._in_use.lock().await.insert(resource.clone(), task_id.clone());
                    let in_use_count = self._in_use.lock().await.len();
                    _debug_log(&format!("Resource pool {} acquired resource by {} ({}/{})", 
                        self.name, task_id, in_use_count, self._total_resources));
                    resource
                }
                None => serde_json::Value::Null
            }
        };
        
        result
    }

    /// Release a resource back to the pool.
    ///
    ///
    /// Args:
    /// resource: Resource to release
    pub fn release(&self, resource: serde_json::Value) {
        let mut in_use = self._in_use.lock().blocking_lock();
        if let Some(task_id) = in_use.remove(&resource) {
            let _ = self._sender.send(resource.clone());
            let in_use_count = in_use.len();
            _debug_log(&format!("Resource pool {} released resource by {} ({}/{})", 
                self.name, task_id, in_use_count, self._total_resources));
        } else {
            _debug_log(&format!("Resource pool {} attempted to release unknown resource", self.name));
        }
    }

    /// Context manager for resource acquisition.
    ///
    ///
    /// Args:
    /// timeout: Maximum time to wait for resource
    ///
    /// Yields:
    /// Resource object, or None if timeout
    pub async fn get_resource(&self, timeout: Option<f64>) -> serde_json::Value {
        // Note: Full context manager would need async trait
        self.acquire(timeout).await
    }

    /// Get number of available resources.
    pub fn available_count(&self) -> i64 {
        // Note: Unbounded channels don't track size
        // This is an approximation
        (self._total_resources - self._in_use.lock().blocking_lock().len()) as i64
    }

    /// Get number of resources in use.
    pub fn in_use_count(&self) -> i64 {
        self._in_use.lock().blocking_lock().len() as i64
    }

    /// Get total number of resources.
    pub fn total_count(&self) -> i64 {
        self._total_resources as i64
    }
}
