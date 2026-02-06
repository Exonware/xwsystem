// #exonware/xwsystem/rust/src/security/resource_limits.rs
//! Generic resource limits and DoS protection.
//! 
//! This module provides resource limit enforcement to prevent DoS attacks
//! and resource exhaustion in any application.


use std::collections::HashMap;

use crate::config::logging_setup::{get_logger};

// Global resource limits registry

// Global resource limits registry

/// Exception raised when resource limits are exceeded.
#[derive(Debug)]
pub struct GenericLimitError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for GenericLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GenericLimitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl GenericLimitError {
    pub fn new(message: impl Into<String>) -> Self {
        GenericLimitError {
            message: message.into(),
            source: None,
        }
    }

}

/// Enforces resource limits to prevent DoS attacks.
pub struct ResourceLimits {
    pub component_name: String,
    pub max_depth: i64,
    pub max_resources: i64,
    pub max_path_length: i64,
    _resource_count: i64,
    _depth_count: i64,
}

impl ResourceLimits {
    /// Constructor
    pub fn new(
        component_name: Option<String>,
        max_depth: Option<i64>,
        max_resources: Option<i64>,
        max_path_length: Option<i64>
    ) -> Self {
        Self {
            component_name: component_name.unwrap_or_else(|| "generic".to_string()),
            max_depth: max_depth.unwrap_or(1000),
            max_resources: max_resources.unwrap_or(1000000),
            max_path_length: max_path_length.unwrap_or(1000),
            _resource_count: 0,
            _depth_count: 0,
        }
    }

    /// Check if depth exceeds maximum allowed.
    pub fn check_depth(&mut self, depth: i64) -> Result<(), GenericLimitError> {
        if depth > self.max_depth {
            return Err(GenericLimitError::new(
                format!("Maximum depth {} exceeded: {}", self.max_depth, depth)
            ));
        }
        self._depth_count = self._depth_count.max(depth);
        Ok(())
    }

    /// Check if resource count exceeds maximum allowed.
    pub fn check_resource_count(&mut self, count: i64) -> Result<(), GenericLimitError> {
        if count > self.max_resources {
            return Err(GenericLimitError::new(
                format!("Maximum resource count {} exceeded: {}", self.max_resources, count)
            ));
        }
        self._resource_count = self._resource_count.max(count);
        Ok(())
    }

    /// Check if path length exceeds maximum allowed.
    pub fn check_path_length(&self, path: &str) -> Result<(), GenericLimitError> {
        if path.len() as i64 > self.max_path_length {
            return Err(GenericLimitError::new(
                format!("Path length {} exceeds maximum {}", path.len(), self.max_path_length)
            ));
        }
        Ok(())
    }

    /// Increment resource count and check limit.
    pub fn increment_resource_count(&mut self) -> Result<(), GenericLimitError> {
        self._resource_count += 1;
        self.check_resource_count(self._resource_count)
    }

    /// Get current resource usage statistics.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("component".to_string(), serde_json::Value::String(self.component_name.clone()));
        stats.insert("resource_count".to_string(), serde_json::Value::Number(self._resource_count.into()));
        stats.insert("max_depth".to_string(), serde_json::Value::Number(self._depth_count.into()));
        
        let mut limits = HashMap::new();
        limits.insert("max_resources".to_string(), serde_json::Value::Number(self.max_resources.into()));
        limits.insert("max_depth".to_string(), serde_json::Value::Number(self.max_depth.into()));
        limits.insert("max_path_length".to_string(), serde_json::Value::Number(self.max_path_length.into()));
        stats.insert("limits".to_string(), serde_json::to_value(limits).unwrap());
        
        stats
    }
}

// Global resource limits registry
use std::sync::{Arc, Mutex, OnceLock};

static LIMITS_REGISTRY: OnceLock<Mutex<HashMap<String, Arc<Mutex<ResourceLimits>>>>> = OnceLock::new();

fn get_registry() -> &'static Mutex<HashMap<String, Arc<Mutex<ResourceLimits>>>> {
    LIMITS_REGISTRY.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Get resource limits instance for a component.
pub fn get_resource_limits(
    component_name: Option<&str>,
    max_depth: Option<i64>,
    max_resources: Option<i64>,
    max_path_length: Option<i64>
) -> Arc<Mutex<ResourceLimits>> {
    let name = component_name.unwrap_or("generic");
    let mut registry = get_registry().lock().unwrap();
    
    registry.entry(name.to_string())
        .or_insert_with(|| {
            Arc::new(Mutex::new(ResourceLimits::new(
                Some(name.to_string()),
                max_depth,
                max_resources,
                max_path_length
            )))
        })
        .clone()
}

/// Reset resource limits for a component or all components.
pub fn reset_resource_limits(component_name: Option<&str>) {
    let mut registry = get_registry().lock().unwrap();
    if let Some(name) = component_name {
        registry.remove(name);
    } else {
        registry.clear();
    }
}
