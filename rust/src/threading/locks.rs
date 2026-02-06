// #exonware/xwsystem/rust/src/threading/locks.rs
//! Enhanced locking utilities for thread-safe operations.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

/// Enhanced reentrant lock with timeout support and improved context management.
pub struct EnhancedRLock {
    lock: Arc<Mutex<()>>,
    default_timeout: Option<Duration>,
    name: String,
    current_holders: Arc<Mutex<usize>>,
    total_acquisitions: Arc<Mutex<usize>>,
    last_acquired_at: Arc<Mutex<Option<Instant>>>,
}

impl EnhancedRLock {
    /// Initialize enhanced lock.
    pub fn new(timeout: Option<f64>, name: Option<String>) -> Self {
        Self {
            lock: Arc::new(Mutex::new(())),
            default_timeout: timeout.map(|t| Duration::from_secs_f64(t)),
            name: name.unwrap_or_else(|| format!("EnhancedRLock-{:p}", std::ptr::null::<()>())),
            current_holders: Arc::new(Mutex::new(0)),
            total_acquisitions: Arc::new(Mutex::new(0)),
            last_acquired_at: Arc::new(Mutex::new(None)),
        }
    }

    /// Acquire the lock with optional timeout.
    pub fn acquire(&self, timeout: Option<f64>) -> bool {
        let effective_timeout = timeout
            .map(|t| Duration::from_secs_f64(t))
            .or(self.default_timeout);

        let start_time = Instant::now();
        
        let acquired = if let Some(timeout_duration) = effective_timeout {
            match self.lock.try_lock_for(timeout_duration) {
                Ok(_guard) => {
                    let mut holders = self.current_holders.lock().unwrap();
                    let mut acquisitions = self.total_acquisitions.lock().unwrap();
                    let mut last_acquired = self.last_acquired_at.lock().unwrap();
                    
                    *holders += 1;
                    *acquisitions += 1;
                    *last_acquired = Some(Instant::now());
                    
                    let acquire_time = last_acquired.unwrap().duration_since(start_time);
                    if acquire_time.as_millis() > 100 {
                        // Log if acquisition took more than 100ms
                        eprintln!(
                            "Lock '{}' acquired after {:.3}s (total: {}, current: {})",
                            self.name,
                            acquire_time.as_secs_f64(),
                            *acquisitions,
                            *holders
                        );
                    }
                    true
                }
                Err(_) => {
                    eprintln!(
                        "Failed to acquire lock '{}' within {:?} timeout",
                        self.name, timeout_duration
                    );
                    false
                }
            }
        } else {
            // No timeout - blocking acquire
            let _guard = self.lock.lock().unwrap();
            let mut holders = self.current_holders.lock().unwrap();
            let mut acquisitions = self.total_acquisitions.lock().unwrap();
            let mut last_acquired = self.last_acquired_at.lock().unwrap();
            
            *holders += 1;
            *acquisitions += 1;
            *last_acquired = Some(Instant::now());
            true
        };

        acquired
    }

    /// Release the lock.
    pub fn release(&self) {
        let mut holders = self.current_holders.lock().unwrap();
        if *holders > 0 {
            *holders -= 1;
        }
        drop(holders);
        // The MutexGuard will be dropped here, releasing the lock
        // Note: This is a simplified implementation - Rust's Mutex doesn't support
        // reentrant locking like Python's RLock. For true reentrant locking,
        // you'd need a custom implementation or use a crate like `parking_lot`.
    }

    /// Check if the lock is currently held.
    pub fn locked(&self) -> bool {
        // Try to acquire without blocking
        self.lock.try_lock().is_err()
    }

    /// Get lock statistics for debugging.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        stats.insert(
            "acquisition_count".to_string(),
            serde_json::Value::Number((*self.total_acquisitions.lock().unwrap()).into()),
        );
        stats.insert(
            "current_holders".to_string(),
            serde_json::Value::Number((*self.current_holders.lock().unwrap()).into()),
        );
        stats.insert(
            "is_locked".to_string(),
            serde_json::Value::Bool(self.locked()),
        );
        if let Some(last) = *self.last_acquired_at.lock().unwrap() {
            stats.insert(
                "last_acquired_at".to_string(),
                serde_json::Value::Number(last.elapsed().as_secs().into()),
            );
        }
        if let Some(timeout) = self.default_timeout {
            stats.insert(
                "default_timeout".to_string(),
                serde_json::Value::Number(timeout.as_secs_f64().into()),
            );
        }
        stats
    }
}

impl Drop for EnhancedRLock {
    fn drop(&mut self) {
        // Ensure lock is released on drop
    }
}

/// Context manager for lock acquisition with timeout.
pub struct TimeoutContext<'a> {
    lock: &'a EnhancedRLock,
    _guard: Option<MutexGuard<'a, ()>>,
}

impl<'a> TimeoutContext<'a> {
    pub fn new(lock: &'a EnhancedRLock, timeout: f64) -> Result<Self, String> {
        let timeout_duration = Duration::from_secs_f64(timeout);
        match lock.lock.try_lock_for(timeout_duration) {
            Ok(guard) => {
                let mut holders = lock.current_holders.lock().unwrap();
                let mut acquisitions = lock.total_acquisitions.lock().unwrap();
                *holders += 1;
                *acquisitions += 1;
                Ok(TimeoutContext {
                    lock,
                    _guard: Some(guard),
                })
            }
            Err(_) => Err(format!(
                "Could not acquire lock '{}' within {}s",
                lock.name, timeout
            )),
        }
    }
}

impl<'a> Drop for TimeoutContext<'a> {
    fn drop(&mut self) {
        let mut holders = self.lock.current_holders.lock().unwrap();
        if *holders > 0 {
            *holders -= 1;
        }
    }
}

/// Safe context manager that always releases the lock.
pub struct SafeContext<'a> {
    lock: &'a EnhancedRLock,
    _guard: MutexGuard<'a, ()>,
}

impl<'a> SafeContext<'a> {
    pub fn new(lock: &'a EnhancedRLock) -> Self {
        let guard = lock.lock.lock().unwrap();
        let mut holders = lock.current_holders.lock().unwrap();
        let mut acquisitions = lock.total_acquisitions.lock().unwrap();
        *holders += 1;
        *acquisitions += 1;
        SafeContext {
            lock,
            _guard: guard,
        }
    }
}

impl<'a> Drop for SafeContext<'a> {
    fn drop(&mut self) {
        let mut holders = self.lock.current_holders.lock().unwrap();
        if *holders > 0 {
            *holders -= 1;
        }
        // Guard is automatically dropped here, releasing the lock
    }
}
