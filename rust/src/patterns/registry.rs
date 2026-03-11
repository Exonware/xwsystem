// #exonware/xwsystem/rust/src/patterns/registry.rs
//exonware/xwsystem/patterns/registry.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 26, 2025
//! 
//! Generic registry pattern for dynamic registration and discovery.


use std::collections::HashMap;

use crate::config::logging_setup::{get_logger};
use crate::contracts::{IGenericRegistry};

// Global registry manager

// Global registry manager

// Global registry manager

/// Registry-specific error.
#[derive(Debug)]
pub struct RegistryError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RegistryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl RegistryError {
    pub fn new(message: impl Into<String>) -> Self {
        RegistryError {
            message: message.into(),
            source: None,
        }
    }

}

// Check if already exists
// Store factory if provided
// Call registration callbacks
// Call unregistration callbacks
// Check if we have a factory for lazy creation
// Call unregistration callbacks for all items
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generic registry for dynamic registration and discovery.
///
/// Features:
/// - Thread-safe registration/lookup
/// - Support for factories
/// - Automatic discovery hooks
/// - Metadata storage per registered item
/// - Type validation
/// - Callback support for registration events
pub struct GenericRegistry {
    pub name: String,
    pub item_type: Option<String>,
    pub allow_overwrite: bool,
    pub auto_discovery: bool,
    _items: Mutex<HashMap<String, serde_json::Value>>,
    _metadata: Mutex<HashMap<String, HashMap<String, serde_json::Value>>>,
    _factories: Mutex<HashMap<String, Box<dyn Fn() -> serde_json::Value + Send + Sync>>>,
    _registration_callbacks: Mutex<Vec<Box<dyn Fn(&str, &serde_json::Value, &HashMap<String, serde_json::Value>) + Send + Sync>>>,
    _unregistration_callbacks: Mutex<Vec<Box<dyn Fn(&str, &serde_json::Value) + Send + Sync>>>,
    _stats: Mutex<HashMap<String, i64>>,
}

impl GenericRegistry {
    /// Initialize generic registry.
    pub fn new(
        name: Option<String>,
        item_type: Option<String>,
        allow_overwrite: Option<bool>,
        auto_discovery: Option<bool>
    ) -> Self {
        let mut stats = HashMap::new();
        stats.insert("registrations".to_string(), 0);
        stats.insert("unregistrations".to_string(), 0);
        stats.insert("lookups".to_string(), 0);
        stats.insert("hits".to_string(), 0);
        stats.insert("misses".to_string(), 0);
        
        Self {
            name: name.unwrap_or_else(|| "generic".to_string()),
            item_type,
            allow_overwrite: allow_overwrite.unwrap_or(false),
            auto_discovery: auto_discovery.unwrap_or(false),
            _items: Mutex::new(HashMap::new()),
            _metadata: Mutex::new(HashMap::new()),
            _factories: Mutex::new(HashMap::new()),
            _registration_callbacks: Mutex::new(Vec::new()),
            _unregistration_callbacks: Mutex::new(Vec::new()),
            _stats: Mutex::new(stats),
        }
    }

    /// Register an item with optional metadata and factory.
    pub fn register(
        &self,
        name: String,
        item: serde_json::Value,
        metadata: Option<HashMap<String, serde_json::Value>>,
        factory: Option<Box<dyn Fn() -> serde_json::Value + Send + Sync>>
    ) -> Result<bool, RegistryError> {
        if name.is_empty() {
            return Err(RegistryError::new("Name must be a non-empty string"));
        }
        
        let mut items = self._items.lock().unwrap();
        let mut metadata_map = self._metadata.lock().unwrap();
        let mut factories = self._factories.lock().unwrap();
        let mut stats = self._stats.lock().unwrap();
        
        if items.contains_key(&name) && !self.allow_overwrite {
            return Err(RegistryError::new(format!("Item '{}' already registered", name)));
        }
        
        items.insert(name.clone(), item.clone());
        
        let mut meta = metadata.unwrap_or_default();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64;
        meta.insert("registered_at".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(now).unwrap()));
        meta.insert("has_factory".to_string(), serde_json::Value::Bool(factory.is_some()));
        metadata_map.insert(name.clone(), meta.clone());
        
        if let Some(factory_fn) = factory {
            factories.insert(name.clone(), factory_fn);
        }
        
        *stats.get_mut("registrations").unwrap() += 1;
        
        let callbacks = self._registration_callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback(&name, &item, &meta);
        }
        
        Ok(true)
    }

    /// Unregister an item.
    pub fn unregister(&self, name: &str) -> bool {
        let mut items = self._items.lock().unwrap();
        let mut metadata_map = self._metadata.lock().unwrap();
        let mut factories = self._factories.lock().unwrap();
        let mut stats = self._stats.lock().unwrap();
        
        if let Some(item) = items.remove(name) {
            metadata_map.remove(name);
            factories.remove(name);
            
            *stats.get_mut("unregistrations").unwrap() += 1;
            
            let callbacks = self._unregistration_callbacks.lock().unwrap();
            for callback in callbacks.iter() {
                callback(name, &item);
            }
            
            true
        } else {
            false
        }
    }

    /// Get an item by name.
    pub fn get(&self, name: &str) -> Option<serde_json::Value> {
        let mut items = self._items.lock().unwrap();
        let mut stats = self._stats.lock().unwrap();
        
        *stats.get_mut("lookups").unwrap() += 1;
        
        if let Some(item) = items.get(name) {
            *stats.get_mut("hits").unwrap() += 1;
            Some(item.clone())
        } else {
            let factories = self._factories.lock().unwrap();
            if let Some(factory) = factories.get(name) {
                let new_item = factory();
                items.insert(name.to_string(), new_item.clone());
                *stats.get_mut("hits").unwrap() += 1;
                Some(new_item)
            } else {
                *stats.get_mut("misses").unwrap() += 1;
                None
            }
        }
    }

    /// List all registered names.
    pub fn list_names(&self) -> Vec<String> {
        let items = self._items.lock().unwrap();
        items.keys().cloned().collect()
    }

    /// Check if an item exists.
    pub fn exists(&self, name: &str) -> bool {
        let items = self._items.lock().unwrap();
        items.contains_key(name)
    }

    /// Clear all registrations.
    pub fn clear(&self) -> bool {
        let mut items = self._items.lock().unwrap();
        let mut metadata_map = self._metadata.lock().unwrap();
        let mut factories = self._factories.lock().unwrap();
        
        let callbacks = self._unregistration_callbacks.lock().unwrap();
        for (name, item) in items.iter() {
            for callback in callbacks.iter() {
                callback(name, item);
            }
        }
        
        items.clear();
        metadata_map.clear();
        factories.clear();
        
        true
    }

    /// Get metadata for an item.
    pub fn get_metadata(&self, name: &str) -> Option<HashMap<String, serde_json::Value>> {
        let metadata_map = self._metadata.lock().unwrap();
        metadata_map.get(name).cloned()
    }

    /// Update metadata for an item.
    pub fn update_metadata(&self, name: &str, metadata: HashMap<String, serde_json::Value>) -> bool {
        let mut metadata_map = self._metadata.lock().unwrap();
        let items = self._items.lock().unwrap();
        
        if !items.contains_key(name) {
            return false;
        }
        
        let existing = metadata_map.entry(name.to_string()).or_insert_with(HashMap::new);
        existing.extend(metadata);
        true
    }

    /// Add callback for registration events.
    pub fn add_registration_callback(&self, callback: Box<dyn Fn(&str, &serde_json::Value, &HashMap<String, serde_json::Value>) + Send + Sync>) {
        let mut callbacks = self._registration_callbacks.lock().unwrap();
        callbacks.push(callback);
    }

    /// Add callback for unregistration events.
    pub fn add_unregistration_callback(&self, callback: Box<dyn Fn(&str, &serde_json::Value) + Send + Sync>) {
        let mut callbacks = self._unregistration_callbacks.lock().unwrap();
        callbacks.push(callback);
    }

    /// Get registry statistics.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let stats = self._stats.lock().unwrap();
        let items = self._items.lock().unwrap();
        
        let hits = *stats.get("hits").unwrap_or(&0);
        let misses = *stats.get("misses").unwrap_or(&0);
        let total_lookups = hits + misses;
        let hit_rate = if total_lookups > 0 {
            hits as f64 / total_lookups as f64
        } else {
            0.0
        };
        
        let mut result = HashMap::new();
        result.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        result.insert("item_type".to_string(), serde_json::Value::String(self.item_type.clone().unwrap_or_default()));
        result.insert("size".to_string(), serde_json::Value::Number(serde_json::Number::from(items.len())));
        result.insert("registrations".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("registrations").unwrap_or(&0))));
        result.insert("unregistrations".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("unregistrations").unwrap_or(&0))));
        result.insert("lookups".to_string(), serde_json::Value::Number(serde_json::Number::from(*stats.get("lookups").unwrap_or(&0))));
        result.insert("hits".to_string(), serde_json::Value::Number(serde_json::Number::from(hits)));
        result.insert("misses".to_string(), serde_json::Value::Number(serde_json::Number::from(misses)));
        result.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        result.insert("allow_overwrite".to_string(), serde_json::Value::Bool(self.allow_overwrite));
        result.insert("auto_discovery".to_string(), serde_json::Value::Bool(self.auto_discovery));
        
        result
    }

    /// Discover and register items using a discovery function.
    pub fn discover_items<F>(&self, discovery_func: F) -> i64
    where
        F: Fn() -> Vec<(String, serde_json::Value, HashMap<String, serde_json::Value>)>,
    {
        let discovered_items = discovery_func();
        let mut registered_count = 0;
        
        for (name, item, metadata) in discovered_items {
            if self.register(name, item, Some(metadata), None).is_ok() {
                registered_count += 1;
            }
        }
        
        registered_count
    }

}

static REGISTRIES: OnceLock<Mutex<HashMap<String, Arc<GenericRegistry>>>> = OnceLock::new();

fn get_registries() -> &'static Mutex<HashMap<String, Arc<GenericRegistry>>> {
    REGISTRIES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Get or create a registry by name.
pub fn get_registry(
    name: &str,
    item_type: Option<String>,
    allow_overwrite: Option<bool>,
    auto_discovery: Option<bool>
) -> Arc<GenericRegistry> {
    let registries = get_registries();
    let mut regs = registries.lock().unwrap();
    
    regs.entry(name.to_string()).or_insert_with(|| {
        Arc::new(GenericRegistry::new(
            Some(name.to_string()),
            item_type,
            allow_overwrite,
            auto_discovery
        ))
    }).clone()
}

/// List all registry names.
pub fn list_registries() -> Vec<String> {
    let registries = get_registries();
    let regs = registries.lock().unwrap();
    regs.keys().cloned().collect()
}

/// Clear a specific registry.
pub fn clear_registry(name: &str) -> bool {
    let registries = get_registries();
    let regs = registries.lock().unwrap();
    
    if let Some(registry) = regs.get(name) {
        registry.clear()
    } else {
        false
    }
}

/// Clear all registries.
pub fn clear_all_registries() -> bool {
    let registries = get_registries();
    let mut regs = registries.lock().unwrap();
    
    for registry in regs.values() {
        registry.clear();
    }
    regs.clear();
    true
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use RegistryError;
pub use GenericRegistry;
pub use get_registry;
pub use list_registries;
pub use clear_registry;
pub use clear_all_registries;
