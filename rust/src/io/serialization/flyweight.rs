// #exonware/xwsystem/rust/src/io/serialization/flyweight.rs
//exonware\xwsystem\serialization\flyweight.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 05, 2025
//! 
//! Flyweight Pattern Implementation for Serializers
//! 
//! Optimizes memory usage by sharing serializer instances with identical configurations.
//! This prevents creating multiple instances of the same serializer type with the same
//! configuration, which is especially important for high-throughput applications.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::config::logging_setup::get_logger;
use crate::io::serialization::contracts::ISerialization;

// Global flyweight instance

// Global flyweight instance

// Factory function for easy serializer creation with flyweight optimization

// Create a hashable key from the class and configuration
// Check if we already have this instance
// Removed expensive debug logging from hot path for performance
// Removed expensive debug logging from hot path for performance
// Keep error logging as it's not in hot path
// Start with class name and module
// Add sorted configuration parameters
// Only include hashable values to avoid issues
// For non-hashable values, use their string representation
/// Flyweight factory for serializer instances.
///
/// Manages shared serializer instances to reduce memory footprint and
/// improve performance by avoiding redundant object creation.
pub struct SerializerFlyweight {
    instances: Arc<Mutex<HashMap<String, String>>>, // Placeholder for Box<dyn ISerialization>
    stats: Arc<Mutex<HashMap<String, i64>>>,
}

impl SerializerFlyweight {
    /// Initialize the flyweight factory.
    pub fn new() -> Self {
        let mut stats = HashMap::new();
        stats.insert("created".to_string(), 0);
        stats.insert("reused".to_string(), 0);
        stats.insert("cache_hits".to_string(), 0);
        stats.insert("cache_misses".to_string(), 0);
        
        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(stats)),
        }
    }

    // Create a hashable key from the class and configuration
    // Check if we already have this instance
    // Removed expensive debug logging from hot path for performance
    // Removed expensive debug logging from hot path for performance
    // Keep error logging as it's not in hot path
    /// Get a serializer instance, creating or reusing based on configuration.
    ///
    ///
    /// Args:
    /// serializer_class: The serializer class to instantiate
    /// **config: Configuration parameters for the serializer
    ///
    ///
    /// Returns:
    /// Shared serializer instance
    pub fn get_serializer(&self, serializer_class: String, config: HashMap<String, String>) -> String {
        // Create cache key
        let cache_key = self._create_cache_key(serializer_class.clone(), 
            config.iter().map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone()))).collect());
        
        let mut instances = self.instances.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();
        
        // Check if we already have this instance
        if instances.contains_key(&cache_key) {
            *stats.get_mut("cache_hits").unwrap() += 1;
            *stats.get_mut("reused").unwrap() += 1;
            return instances.get(&cache_key).unwrap().clone();
        }
        
        // Create new instance (placeholder - actual instantiation requires type system)
        *stats.get_mut("cache_misses").unwrap() += 1;
        *stats.get_mut("created").unwrap() += 1;
        
        // TODO: Actually instantiate serializer when type system is available
        let instance_key = format!("{}:{}", serializer_class, cache_key);
        instances.insert(cache_key, instance_key.clone());
        instance_key
    }

    // Start with class name and module
    // Add sorted configuration parameters
    // Only include hashable values to avoid issues
    // For non-hashable values, use their string representation
    /// Create a hashable cache key from class and configuration.
    ///
    ///
    /// Args:
    /// serializer_class: The serializer class
    /// config: Configuration dictionary
    ///
    ///
    /// Returns:
    /// String cache key
    pub fn _create_cache_key(&self, serializer_class: String, config: HashMap<String, serde_json::Value>) -> String {
        // Start with class name
        let mut key_parts = vec![serializer_class];
        
        // Add sorted configuration parameters
        let mut sorted_keys: Vec<&String> = config.keys().collect();
        sorted_keys.sort();
        
        for key in sorted_keys {
            if let Some(value) = config.get(key) {
                if self._is_hashable(value.clone()) {
                    key_parts.push(format!("{}={}", key, value));
                } else {
                    // For non-hashable values, use their type name
                    key_parts.push(format!("{}={}:{}", key, 
                        match value {
                            serde_json::Value::Object(_) => "Object",
                            serde_json::Value::Array(_) => "Array",
                            _ => "Other",
                        },
                        value.to_string()));
                }
            }
        }
        
        key_parts.join("|")
    }

    /// Check if an object is hashable.
    pub fn _is_hashable(&self, obj: serde_json::Value) -> bool {
        // Check if value is hashable (primitive types)
        matches!(obj, 
            serde_json::Value::String(_) | 
            serde_json::Value::Number(_) | 
            serde_json::Value::Bool(_) | 
            serde_json::Value::Null
        )
    }

    /// Get flyweight usage statistics.
    ///
    ///
    /// Returns:
    /// Dictionary with usage statistics
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let instances = self.instances.lock().unwrap();
        let stats = self.stats.lock().unwrap();
        
        let created = *stats.get("created").unwrap_or(&0);
        let reused = *stats.get("reused").unwrap_or(&0);
        let cache_hits = *stats.get("cache_hits").unwrap_or(&0);
        let cache_misses = *stats.get("cache_misses").unwrap_or(&0);
        let total_requests = cache_hits + cache_misses;
        let total_instances = created + reused;
        
        let mut result = HashMap::new();
        result.insert("created".to_string(), serde_json::Value::Number(created.into()));
        result.insert("reused".to_string(), serde_json::Value::Number(reused.into()));
        result.insert("cache_hits".to_string(), serde_json::Value::Number(cache_hits.into()));
        result.insert("cache_misses".to_string(), serde_json::Value::Number(cache_misses.into()));
        result.insert("active_instances".to_string(), serde_json::Value::Number(instances.len().into()));
        
        let hit_rate = if total_requests > 0 {
            cache_hits as f64 / total_requests as f64
        } else {
            0.0
        };
        result.insert("hit_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(hit_rate).unwrap()));
        
        let reuse_rate = if total_instances > 0 {
            reused as f64 / total_instances as f64
        } else {
            0.0
        };
        result.insert("reuse_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(reuse_rate).unwrap()));
        
        result
    }

    /// Clear the serializer cache.
    pub fn clear_cache(&self) {
        let mut instances = self.instances.lock().unwrap();
        instances.clear();
    }

    /// Get detailed cache information.
    ///
    ///
    /// Returns:
    /// Dictionary with cache details
    pub fn get_cache_info(&self) -> HashMap<String, serde_json::Value> {
        let instances = self.instances.lock().unwrap();
        let mut result = HashMap::new();
        result.insert("cache_size".to_string(), serde_json::Value::Number(instances.len().into()));
        result.insert("cache_keys".to_string(), serde_json::Value::Array(
            instances.keys().map(|k| serde_json::Value::String(k.clone())).collect()
        ));
        result
    }

}

// Update access tracking
// Check if we need to evict
// Evict least recently used
// Evict least frequently used
// Remove from all tracking dictionaries
/// Advanced serializer pool with size limits and eviction policies.
///
/// Provides more advanced caching with configurable size limits and
/// eviction strategies for high-throughput applications.
pub struct SerializerPool {
    pub max_size: i64,
    pub eviction_policy: String,
}

impl SerializerPool {
    /// Initialize serializer pool.
    ///
    ///
    /// Args:
    /// max_size: Maximum number of cached instances
    /// eviction_policy: Eviction policy ("LRU", "LFU", "FIFO")
    pub fn new(
        max_size: Option<i64>,
        eviction_policy: Option<String>
    ) -> Self {
        Self {
            max_size,
            eviction_policy,
        }
    }

    // Update access tracking
    // Check if we need to evict
    /// Get serializer from pool with size-limited caching.
    pub fn get_serializer(&mut self, serializer_class: String, config: HashMap<String, String>) -> T
    {
        // TODO: This function uses external dependencies.
        // The original Python implementation should be maintained/adapted.
        //
        // Python → Rust crate mappings:
        //   config → (no known Rust equivalent)
        //   contracts → (no known Rust equivalent)
        //
        // Add these crates to Cargo.toml if implementing:
        todo!()
    }

    // Evict least recently used
    // Evict least frequently used
    // Remove from all tracking dictionaries
    /// Evict an instance based on the configured policy.
    pub fn _evict_instance(&self) -> ()
    {
        // TODO: Implement
        todo!()
    }

    /// Create cache key (same as flyweight implementation).
    pub fn _create_cache_key(&self, serializer_class: String, config: HashMap<String, serde_json::Value>) -> String
    {
        // TODO: This function uses external dependencies.
        // The original Python implementation should be maintained/adapted.
        //
        // Python → Rust crate mappings:
        //   contracts → (no known Rust equivalent)
        //
        // Add these crates to Cargo.toml if implementing:
        todo!()
    }

    /// Get pool statistics.
    pub fn get_pool_stats(&self) -> HashMap<String, serde_json::Value>
    {
        // TODO: Implement
        todo!()
    }

}

    /// Get a serializer instance using the flyweight pattern.
    ///
    /// This is the main entry point for getting serializer instances with
    /// automatic memory optimization through instance sharing.
    ///
    ///
    /// Args:
    /// serializer_class: The serializer class to instantiate
    /// **config: Configuration parameters for the serializer
    ///
    ///
    /// Returns:
    /// Shared serializer instance
    ///
    /// Example:
    /// >>> from xwsystem.serialization import JsonSerializer
    /// >>> # These will return the same instance
    /// >>> json1 = get_serializer(JsonSerializer, validate_input=True)
    /// >>> json2 = get_serializer(JsonSerializer, validate_input=True)
    /// >>> assert json1 is json2  # Same instance
    pub fn get_serializer(serializer_class: &str, config: HashMap<String, String>) -> String {
        // Use global flyweight instance
        static GLOBAL_FLYWEIGHT: std::sync::OnceLock<SerializerFlyweight> = std::sync::OnceLock::new();
        let flyweight = GLOBAL_FLYWEIGHT.get_or_init(|| SerializerFlyweight::new());
        flyweight.get_serializer(serializer_class.to_string(), config)
    }

    /// Get flyweight usage statistics.
    ///
    ///
    /// Returns:
    /// Dictionary with usage statistics including:
    /// - created: Number of new instances created
    /// - reused: Number of times existing instances were reused
    /// - cache_hits: Number of successful cache lookups
    /// - cache_misses: Number of failed cache lookups
    /// - active_instances: Current number of cached instances
    /// - hit_rate: Cache hit rate (0.0 to 1.0)
    /// - reuse_rate: Instance reuse rate (0.0 to 1.0)
    pub fn get_flyweight_stats() -> HashMap<String, serde_json::Value> {
        static GLOBAL_FLYWEIGHT: std::sync::OnceLock<SerializerFlyweight> = std::sync::OnceLock::new();
        let flyweight = GLOBAL_FLYWEIGHT.get_or_init(|| SerializerFlyweight::new());
        flyweight.get_stats()
    }

    /// Clear the global serializer cache.
    pub fn clear_serializer_cache() {
        static GLOBAL_FLYWEIGHT: std::sync::OnceLock<SerializerFlyweight> = std::sync::OnceLock::new();
        let flyweight = GLOBAL_FLYWEIGHT.get_or_init(|| SerializerFlyweight::new());
        flyweight.clear_cache();
    }

    /// Get detailed information about cached serializer instances.
    pub fn get_cache_info() -> HashMap<String, serde_json::Value> {
        static GLOBAL_FLYWEIGHT: std::sync::OnceLock<SerializerFlyweight> = std::sync::OnceLock::new();
        let flyweight = GLOBAL_FLYWEIGHT.get_or_init(|| SerializerFlyweight::new());
        flyweight.get_cache_info()
    }

    // Factory function for easy serializer creation with flyweight optimization
    // Import here to avoid circular imports
    // Core formats (always available)
    // NOTE: Enterprise formats moved to exonware-xwformats per DEV_GUIDELINES.md
    // Users should explicitly import from xwformats:
    // from exonware.xwformats import AvroSerializer, ProtobufSerializer, ...
    // Then use them directly or via registry
    // Core formats (17 formats)
    // Check for enterprise formats (available in xwformats)
    // Try to get from xwformats registry
    /// Create a serializer instance by format name using flyweight pattern.
    ///
    ///
    /// Args:
    /// format_name: Name of the serialization format
    /// **config: Configuration parameters
    ///
    ///
    /// Returns:
    /// Serializer instance
    ///
    ///
    /// Raises:
    /// ValueError: If format is not supported
    pub fn create_serializer(format_name: &str, config: HashMap<String, String>) -> String {
        // Use flyweight pattern to get/create serializer
        get_serializer(format_name, config)
    }
