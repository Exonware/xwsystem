// #exonware/xwsystem/rust/src/structures/circular_detector.rs
//! Circular reference detection and management utilities.

use std::collections::{HashMap, HashSet};

/// Raised when circular references are detected.
#[derive(Debug)]
pub struct CircularReferenceError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for CircularReferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CircularReferenceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl CircularReferenceError {
    pub fn new(message: impl Into<String>) -> Self {
        CircularReferenceError {
            message: message.into(),
            source: None,
        }
    }

}

// Currently being visited objects
// Object ID to depth mapping
// Skip basic types that can't contain references
// Check if we're currently visiting this object (immediate circular reference)
// Check if we've seen this object before at a different depth
// Mark as currently being visited
// Traverse based on object type
// Mark as completely visited
// Remove from currently visiting set
// Convert key to string for path tracking
// Track reference relationship
// Track reference relationship
// Skip certain types that are known to be safe
// Skip private attributes and known safe attributes
// Track reference relationship
// Object doesn't have __dict__ or it's not accessible
// Found circular reference
// Add current object to path
/// Utility for detecting and managing circular references in data structures.
///
/// This helps prevent infinite loops and memory leaks when traversing
/// complex data structures that may contain circular references.
pub struct CircularReferenceDetector {
    pub max_depth: usize,
    _visiting: HashSet<usize>,
    _visited: HashSet<usize>,
    _depth_map: HashMap<usize, usize>,
    _reference_graph: HashMap<usize, HashSet<usize>>,
    _current_depth: usize,
}

impl CircularReferenceDetector {
    /// Initialize circular reference detector.
    pub fn new(max_depth: Option<usize>) -> Self {
        Self {
            max_depth: max_depth.unwrap_or(100),
            _visiting: HashSet::new(),
            _visited: HashSet::new(),
            _depth_map: HashMap::new(),
            _reference_graph: HashMap::new(),
            _current_depth: 0,
        }
    }

    /// Check if an object contains circular references.
    pub fn is_circular(&mut self, obj: &serde_json::Value, path: Option<Vec<String>>) -> bool {
        match self.traverse(obj, path.unwrap_or_default()) {
            Ok(_) => {
                self.reset();
                false
            }
            Err(_) => {
                self.reset();
                true
            }
        }
    }

    /// Traverse an object checking for circular references.
    pub fn traverse(&mut self, obj: &serde_json::Value, path: Vec<String>) -> Result<(), CircularReferenceError> {
        // Skip basic types that can't contain references
        if matches!(obj, serde_json::Value::Null | serde_json::Value::String(_) | serde_json::Value::Number(_) | serde_json::Value::Bool(_)) {
            return Ok(());
        }

        let obj_hash = self._hash_value(obj);
        let current_path = if path.is_empty() {
            "root".to_string()
        } else {
            path.join(".")
        };

        // Check depth limit
        if self._current_depth > self.max_depth {
            return Err(CircularReferenceError::new(format!(
                "Maximum traversal depth ({}) exceeded at path: {}",
                self.max_depth, current_path
            )));
        }

        // Check if we're currently visiting this object (immediate circular reference)
        if self._visiting.contains(&obj_hash) {
            return Err(CircularReferenceError::new(format!(
                "Circular reference detected at path: {} (object {} already being visited)",
                current_path, obj_hash
            )));
        }

        // Check if we've seen this object before at a different depth
        if let Some(previous_depth) = self._depth_map.get(&obj_hash) {
            if *previous_depth <= self._current_depth {
                return Err(CircularReferenceError::new(format!(
                    "Circular reference detected: object {} encountered again at path {} (previously at depth {}, now at depth {})",
                    obj_hash, current_path, previous_depth, self._current_depth
                )));
            }
        }

        // Mark as currently being visited
        self._visiting.insert(obj_hash);
        self._depth_map.insert(obj_hash, self._current_depth);

        let result = {
            self._current_depth += 1;

            // Traverse based on object type
            let traverse_result = match obj {
                serde_json::Value::Object(map) => self._traverse_dict(map, path.clone()),
                serde_json::Value::Array(arr) => self._traverse_sequence(arr, path.clone()),
                _ => Ok(()),
            };

            // Mark as completely visited
            self._visited.insert(obj_hash);
            traverse_result
        };

        // Remove from currently visiting set
        self._visiting.remove(&obj_hash);
        self._current_depth -= 1;

        result
    }

    fn _hash_value(&self, value: &serde_json::Value) -> usize {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.to_string().hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Traverse dictionary objects.
    fn _traverse_dict(&mut self, obj: &serde_json::Map<String, serde_json::Value>, path: Vec<String>) -> Result<(), CircularReferenceError> {
        for (key, value) in obj {
            let new_path = {
                let mut p = path.clone();
                p.push(key.clone());
                p
            };

            // Track reference relationship
            let value_hash = self._hash_value(value);
            let obj_hash = self._hash_value(&serde_json::Value::Object(obj.clone()));
            self._reference_graph.entry(obj_hash).or_insert_with(HashSet::new).insert(value_hash);

            self.traverse(value, new_path)?;
        }
        Ok(())
    }

    /// Traverse sequence objects (list, tuple, set).
    fn _traverse_sequence(&mut self, obj: &Vec<serde_json::Value>, path: Vec<String>) -> Result<(), CircularReferenceError> {
        for (i, item) in obj.iter().enumerate() {
            let new_path = {
                let mut p = path.clone();
                p.push(format!("[{}]", i));
                p
            };

            // Track reference relationship
            let item_hash = self._hash_value(item);
            let obj_hash = self._hash_value(&serde_json::Value::Array(obj.clone()));
            self._reference_graph.entry(obj_hash).or_insert_with(HashSet::new).insert(item_hash);

            self.traverse(item, new_path)?;
        }
        Ok(())
    }

    /// Reset the detector state for reuse.
    pub fn reset(&mut self) {
        self._visiting.clear();
        self._visited.clear();
        self._depth_map.clear();
        self._reference_graph.clear();
        self._current_depth = 0;
    }

    /// Get the reference graph discovered during traversal.
    pub fn get_reference_graph(&self) -> HashMap<usize, HashSet<usize>> {
        self._reference_graph.clone()
    }

    /// Find all circular reference paths in an object.
    pub fn find_circular_paths(&self, obj: &serde_json::Value) -> Vec<Vec<String>> {
        let mut circular_paths = Vec::new();
        let mut path_objects: HashMap<usize, Vec<String>> = HashMap::new();

        self._traverse_with_path_tracking(obj, Vec::new(), &mut path_objects, &mut circular_paths);
        circular_paths
    }

    fn _traverse_with_path_tracking(
        &self,
        current_obj: &serde_json::Value,
        current_path: Vec<String>,
        path_objects: &mut HashMap<usize, Vec<String>>,
        circular_paths: &mut Vec<Vec<String>>,
    ) {
        if matches!(current_obj, serde_json::Value::Null | serde_json::Value::String(_) | serde_json::Value::Number(_) | serde_json::Value::Bool(_)) {
            return;
        }

        let obj_hash = self._hash_value(current_obj);

        if let Some(existing_path) = path_objects.get(&obj_hash) {
            // Found circular reference
            let mut circular_path = existing_path.clone();
            circular_path.extend(current_path.clone());
            circular_paths.push(circular_path);
            return;
        }

        // Add current object to path
        path_objects.insert(obj_hash, current_path.clone());

        match current_obj {
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    let mut new_path = current_path.clone();
                    new_path.push(key.clone());
                    self._traverse_with_path_tracking(value, new_path, path_objects, circular_paths);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, item) in arr.iter().enumerate() {
                    let mut new_path = current_path.clone();
                    new_path.push(format!("[{}]", i));
                    self._traverse_with_path_tracking(item, new_path, path_objects, circular_paths);
                }
            }
            _ => {}
        }
    }

    fn _hash_value(&self, value: &serde_json::Value) -> usize {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.to_string().hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Get statistics about the last traversal.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("visited_objects".to_string(), serde_json::Value::Number(self._visited.len().into()));
        stats.insert(
            "max_depth_reached".to_string(),
            serde_json::Value::Number(
                self._depth_map.values().max().copied().unwrap_or(0).into()
            ),
        );
        stats.insert(
            "reference_count".to_string(),
            serde_json::Value::Number(
                self._reference_graph.values().map(|refs| refs.len()).sum::<usize>().into()
            ),
        );
        stats.insert(
            "objects_with_references".to_string(),
            serde_json::Value::Number(self._reference_graph.len().into()),
        );
        stats
    }
}

/// Alias for CircularReferenceDetector for backward compatibility.
pub type CircularDetector = CircularReferenceDetector;

impl CircularDetector {
    /// Detect circular references (alias for is_circular).
    pub fn detect(&mut self, obj: &serde_json::Value) -> bool {
        self.is_circular(obj, None)
    }

    /// Check for circular references (alias for is_circular).
    pub fn check(&mut self, obj: &serde_json::Value) -> bool {
        self.is_circular(obj, None)
    }
}

/// Quick function to check if an object has circular references.
pub fn has_circular_references(obj: &serde_json::Value, max_depth: Option<usize>) -> bool {
    let mut detector = CircularReferenceDetector::new(max_depth);
    detector.is_circular(obj, None)
}

/// Safely traverse an object while avoiding circular references.
pub fn safe_traverse<F>(obj: &serde_json::Value, visitor_func: F, max_depth: Option<usize>) -> Option<serde_json::Value>
where
    F: Fn(&serde_json::Value) -> serde_json::Value,
{
    let mut detector = CircularReferenceDetector::new(max_depth);
    
    match detector.traverse(obj, Vec::new()) {
        Ok(_) => Some(visitor_func(obj)),
        Err(_) => None,
    }
}
