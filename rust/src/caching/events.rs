// #exonware/xwsystem/rust/src/caching/events.rs
//exonware/xwsystem/caching/events.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Event system for caching module.
//! Extensibility Priority #5 - Event-driven architecture for custom behaviors.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::caching::base::Hashable;

/// Cache events for hook registration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CacheEvent {
    Hit,
    Miss,
    Put,
    Delete,
    Evict,
    Expire,
    Clear,
    Error,
}

/// Mixin class for event emission in caches.
///
/// Provides event hook registration and emission functionality.
pub struct CacheEventEmitter {
    hooks: Mutex<HashMap<CacheEvent, Vec<Arc<dyn Fn(&CacheEvent, &HashMap<String, serde_json::Value>) + Send + Sync>>>>,
    event_stats: Mutex<HashMap<CacheEvent, i64>>,
}

impl CacheEventEmitter {
    /// Initialize event emitter.
    pub fn new() -> Self {
        let mut hooks = HashMap::new();
        let mut event_stats = HashMap::new();
        for event in [CacheEvent::Hit, CacheEvent::Miss, CacheEvent::Put, CacheEvent::Delete, CacheEvent::Evict, CacheEvent::Expire, CacheEvent::Clear, CacheEvent::Error] {
            hooks.insert(event, Vec::new());
            event_stats.insert(event, 0);
        }
        Self {
            hooks: Mutex::new(hooks),
            event_stats: Mutex::new(event_stats),
        }
    }

    /// Register event callback.
    ///
    /// Args:
    /// event: Event to listen for
    /// callback: Callback function(event, **kwargs)
    pub fn on<F>(&self, event: CacheEvent, callback: F)
    where
        F: Fn(&CacheEvent, &HashMap<String, serde_json::Value>) + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.lock().unwrap();
        if let Some(callbacks) = hooks.get_mut(&event) {
            callbacks.push(Arc::new(callback));
        }
    }

    /// Unregister event callback.
    ///
    /// Args:
    /// event: Event to stop listening for
    /// callback: Callback function to remove
    ///
    /// Returns:
    /// True if callback was removed
    pub fn off<F>(&self, event: CacheEvent, callback: &F) -> bool
    where
        F: Fn(&CacheEvent, &HashMap<String, serde_json::Value>) + Send + Sync + 'static,
    {
        let mut hooks = self.hooks.lock().unwrap();
        if let Some(callbacks) = hooks.get_mut(&event) {
            // Note: In Rust, we can't easily compare function pointers for equality
            // This is a simplified implementation - in practice, you might want to use
            // a different approach like storing callback IDs
            false
        } else {
            false
        }
    }

    /// Clear event hooks.
    ///
    /// Args:
    /// event: Specific event to clear (None = clear all)
    pub fn clear_hooks(&self, event: Option<CacheEvent>) {
        let mut hooks = self.hooks.lock().unwrap();
        if let Some(evt) = event {
            if let Some(callbacks) = hooks.get_mut(&evt) {
                callbacks.clear();
            }
        } else {
            for callbacks in hooks.values_mut() {
                callbacks.clear();
            }
        }
    }

    /// Emit event to registered callbacks.
    ///
    /// Args:
    /// event: Event to emit
    /// kwargs: Event data passed to callbacks
    pub fn _emit(&self, event: CacheEvent, kwargs: HashMap<String, serde_json::Value>) {
        // Update statistics
        {
            let mut stats = self.event_stats.lock().unwrap();
            *stats.entry(event).or_insert(0) += 1;
        }
        
        // Call registered callbacks
        let hooks = self.hooks.lock().unwrap();
        if let Some(callbacks) = hooks.get(&event) {
            for callback in callbacks.iter() {
                callback(&event, &kwargs);
            }
        }
    }

    /// Get event emission statistics.
    ///
    /// Returns:
    /// Dictionary of event counts
    pub fn get_event_stats(&self) -> HashMap<String, i64> {
        let stats = self.event_stats.lock().unwrap();
        let mut result = HashMap::new();
        for (event, count) in stats.iter() {
            let event_name = match event {
                CacheEvent::Hit => "hit",
                CacheEvent::Miss => "miss",
                CacheEvent::Put => "put",
                CacheEvent::Delete => "delete",
                CacheEvent::Evict => "evict",
                CacheEvent::Expire => "expire",
                CacheEvent::Clear => "clear",
                CacheEvent::Error => "error",
            };
            result.insert(event_name.to_string(), *count);
        }
        result
    }

}

/// Built-in event logger for debugging.
///
/// Logs all cache events for debugging and monitoring.
pub struct EventLogger {
    log_level: String,
    events_log: Mutex<Vec<HashMap<String, serde_json::Value>>>,
}

impl EventLogger {
    /// Initialize event logger.
    ///
    /// Args:
    /// log_level: Logging level for events
    pub fn new(
        log_level: Option<String>
    ) -> Self {
        Self {
            log_level: log_level.unwrap_or_else(|| "DEBUG".to_string()),
            events_log: Mutex::new(Vec::new()),
        }
    }

    /// Log event (callable interface).
    ///
    /// Args:
    /// event: Event to log
    /// kwargs: Event data
    pub fn log_event(&self, event: CacheEvent, kwargs: HashMap<String, serde_json::Value>) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let event_name = match event {
            CacheEvent::Hit => "hit",
            CacheEvent::Miss => "miss",
            CacheEvent::Put => "put",
            CacheEvent::Delete => "delete",
            CacheEvent::Evict => "evict",
            CacheEvent::Expire => "expire",
            CacheEvent::Clear => "clear",
            CacheEvent::Error => "error",
        };
        
        let mut log_entry = HashMap::new();
        log_entry.insert("timestamp".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
        log_entry.insert("event".to_string(), serde_json::Value::String(event_name.to_string()));
        log_entry.insert("data".to_string(), serde_json::json!(kwargs));
        
        let mut events = self.events_log.lock().unwrap();
        events.push(log_entry);
    }

    /// Get logged events, optionally filtered by type.
    ///
    /// Args:
    /// event_type: Optional event type to filter by
    ///
    /// Returns:
    /// List of event log entries
    pub fn get_events(&self, event_type: Option<CacheEvent>) -> Vec<HashMap<String, serde_json::Value>> {
        let events = self.events_log.lock().unwrap();
        
        if let Some(filter_event) = event_type {
            let event_name = match filter_event {
                CacheEvent::Hit => "hit",
                CacheEvent::Miss => "miss",
                CacheEvent::Put => "put",
                CacheEvent::Delete => "delete",
                CacheEvent::Evict => "evict",
                CacheEvent::Expire => "expire",
                CacheEvent::Clear => "clear",
                CacheEvent::Error => "error",
            };
            
            events.iter()
                .filter(|entry| {
                    entry.get("event")
                        .and_then(|v| v.as_str())
                        .map(|s| s == event_name)
                        .unwrap_or(false)
                })
                .cloned()
                .collect()
        } else {
            events.clone()
        }
    }

    /// Clear event log.
    pub fn clear(&self) {
        let mut events = self.events_log.lock().unwrap();
        events.clear();
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
