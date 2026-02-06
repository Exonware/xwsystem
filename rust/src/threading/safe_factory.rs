// #exonware/xwsystem/rust/src/threading/safe_factory.rs
//exonware/xwsystem/threading/safe_factory.py
//! Thread-safe factory pattern for handler registration and management.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Generic thread-safe factory for handler registration and retrieval.
///
/// This can be used as a base class for any handler factory that needs
/// thread-safe registration and lookup of handlers by name/extension.
pub struct ThreadSafeFactory<T> {
    handlers: Arc<Mutex<HashMap<String, Arc<T>>>>,
    extensions: Arc<Mutex<HashMap<String, String>>>,
    methods_generated: Arc<Mutex<bool>>,
    methods_lock: Arc<Mutex<()>>,
}

impl<T> ThreadSafeFactory<T> {
    /// Constructor
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            extensions: Arc::new(Mutex::new(HashMap::new())),
            methods_generated: Arc::new(Mutex::new(false)),
            methods_lock: Arc::new(Mutex::new(())),
        }
    }

    /// Register a handler with optional file extensions.
    pub fn register(&self, name: &str, handler: Arc<T>, extensions: Option<Vec<String>>) {
        let name_lower = name.to_lowercase();
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(name_lower.clone(), handler);

        let mut processed_exts = Vec::new();
        if let Some(exts) = extensions {
            processed_exts = exts.iter()
                .map(|ext| ext.to_lowercase().trim_start_matches('.').to_string())
                .collect();
        } else if !processed_exts.contains(&name_lower) {
            processed_exts.push(name_lower.clone());
        }

        let mut extensions_map = self.extensions.lock().unwrap();
        for ext in processed_exts {
            if let Some(existing) = extensions_map.get(&ext) {
                if existing != &name_lower {
                    // Extension conflict - will use content detection
                }
            }
            extensions_map.insert(ext, name_lower.clone());
        }
    }

    /// Get a handler by name.
    pub fn get_handler(&self, name: &str) -> Result<Arc<T>, String> {
        let handlers = self.handlers.lock().unwrap();
        handlers.get(&name.to_lowercase())
            .cloned()
            .ok_or_else(|| format!("No handler registered for: {}", name))
    }

    /// Get a handler by name, returning None if not found.
    pub fn get_handler_if_exists(&self, name: &str) -> Option<Arc<T>> {
        let handlers = self.handlers.lock().unwrap();
        handlers.get(&name.to_lowercase()).cloned()
    }

    /// Get format name by file extension.
    pub fn get_format_by_extension(&self, extension: &str) -> Option<String> {
        let extensions = self.extensions.lock().unwrap();
        extensions.get(&extension.to_lowercase().trim_start_matches('.').to_string())
            .cloned()
    }

    /// Get list of all registered format names.
    pub fn get_available_formats(&self) -> Vec<String> {
        let handlers = self.handlers.lock().unwrap();
        handlers.keys().cloned().collect()
    }

    /// Check if a handler is registered.
    pub fn has_handler(&self, name: &str) -> bool {
        let handlers = self.handlers.lock().unwrap();
        handlers.contains_key(&name.to_lowercase())
    }

    /// Unregister a handler.
    pub fn unregister(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        let mut handlers = self.handlers.lock().unwrap();
        
        if handlers.remove(&name_lower).is_some() {
            // Remove associated extensions
            let mut extensions = self.extensions.lock().unwrap();
            extensions.retain(|_, format_name| format_name != &name_lower);
            true
        } else {
            false
        }
    }

    /// Clear all registered handlers.
    pub fn clear(&self) {
        let mut handlers = self.handlers.lock().unwrap();
        let mut extensions = self.extensions.lock().unwrap();
        handlers.clear();
        extensions.clear();
    }
}

impl<T> Default for ThreadSafeFactory<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for thread-safe dynamic method generation.
pub struct MethodGenerator;

impl MethodGenerator {
    /// Generate dynamic methods on a class based on registered formats.
    ///
    /// Note: Rust doesn't support dynamic method generation at runtime like Python.
    /// This is a placeholder that documents the intended functionality.
    /// In Rust, you would typically use macros or trait-based approaches instead.
    pub fn generate_export_methods<T>(
        _target_class: &str,
        _factory: &ThreadSafeFactory<T>,
        _method_template: fn(),
        _method_name_pattern: Option<&str>,
        _method_doc_pattern: Option<&str>,
    ) {
        // Rust doesn't support runtime method generation like Python
        // Use macros or trait-based approaches instead
        // This is a placeholder for documentation purposes
    }
}
