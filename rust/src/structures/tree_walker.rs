// #exonware/xwsystem/rust/src/structures/tree_walker.rs
//! Generic tree walking utilities for processing nested data structures.
//! 
//! This module provides reusable tree traversal functionality that was
//! previously embedded in xData but is generally useful across xLib components.

use std::collections::{HashMap, HashSet};

// Check for circular references
// Return unmodified to break cycle
// Only track containers that could cause cycles
// Recursively process children
// Leaf node, return processed value
// Check if node is a leaf (not a container)
/// Generic tree walking utility with customizable node processors.
///
/// This class provides safe traversal of nested data structures (dicts, lists)
/// with protection against circular references and deep recursion.
pub struct TreeWalker {
    pub max_depth: usize,
    pub track_visited: bool,
    pub visited: HashSet<usize>,
    _depth: usize,
}

impl TreeWalker {
    /// Initialize tree walker.
    pub fn new(
        max_depth: Option<usize>,
        track_visited: Option<bool>,
        visit_tracker: Option<HashSet<usize>>
    ) -> Self {
        Self {
            max_depth: max_depth.unwrap_or(1000),
            track_visited: track_visited.unwrap_or(true),
            visited: visit_tracker.unwrap_or_else(HashSet::new),
            _depth: 0,
        }
    }

    /// Walk data structure and apply processor to each node.
    pub fn walk_and_process<F>(&mut self, data: serde_json::Value, node_processor: F, path: String) -> Result<serde_json::Value, String>
    where
        F: Fn(serde_json::Value, String, usize) -> serde_json::Value,
    {
        self._depth += 1;

        // Check depth limit
        if self._depth > self.max_depth {
            return Err(format!("Tree traversal depth limit exceeded: {}", self.max_depth));
        }

        // Check for circular references
        if self.track_visited {
            // Use a simple hash of the value as object ID (not perfect but works for JSON)
            let obj_hash = self._hash_value(&data);
            if self.visited.contains(&obj_hash) {
                return Ok(data); // Return unmodified to break cycle
            }

            // Only track containers that could cause cycles
            if self._is_container(&data) {
                self.visited.insert(obj_hash);
            }
        }

        // Process current node
        let processed_data = node_processor(data.clone(), path.clone(), self._depth);

        // Recursively process children
        let result = match &processed_data {
            serde_json::Value::Object(map) => {
                let mut result_map = serde_json::Map::new();
                for (key, value) in map {
                    let key_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}/{}", path, key)
                    };
                    match self.walk_and_process(value.clone(), &node_processor, key_path) {
                        Ok(processed) => {
                            result_map.insert(key.clone(), processed);
                        }
                        Err(e) => return Err(e),
                    }
                }
                serde_json::Value::Object(result_map)
            }
            serde_json::Value::Array(arr) => {
                let mut result_vec = Vec::new();
                for (i, item) in arr.iter().enumerate() {
                    let item_path = if path.is_empty() {
                        format!("[{}]", i)
                    } else {
                        format!("{}[{}]", path, i)
                    };
                    match self.walk_and_process(item.clone(), &node_processor, item_path) {
                        Ok(processed) => result_vec.push(processed),
                        Err(e) => return Err(e),
                    }
                }
                serde_json::Value::Array(result_vec)
            }
            _ => {
                // Leaf node, return processed value
                processed_data
            }
        };

        self._depth -= 1;
        Ok(result)
    }

    fn _hash_value(&self, value: &serde_json::Value) -> usize {
        // Simple hash for cycle detection
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.to_string().hash(&mut hasher);
        hasher.finish() as usize
    }

    fn _is_container(&self, value: &serde_json::Value) -> bool {
        matches!(value, serde_json::Value::Object(_) | serde_json::Value::Array(_))
    }

    /// Walk data structure, applying processor only to nodes that pass filter.
    pub fn walk_with_filter<F, P>(&mut self, data: serde_json::Value, filter_func: F, processor: P, path: String) -> Result<serde_json::Value, String>
    where
        F: Fn(&serde_json::Value, &str, usize) -> bool,
        P: Fn(serde_json::Value, String, usize) -> serde_json::Value,
    {
        let conditional_processor = |node: serde_json::Value, node_path: String, depth: usize| -> serde_json::Value {
            if filter_func(&node, &node_path, depth) {
                processor(node, node_path, depth)
            } else {
                node
            }
        };
        self.walk_and_process(data, conditional_processor, path)
    }

    /// Find all nodes that match a predicate.
    pub fn find_nodes<F>(&mut self, data: serde_json::Value, predicate: F, path: String) -> Result<Vec<HashMap<String, serde_json::Value>>, String>
    where
        F: Fn(&serde_json::Value, &str, usize) -> bool,
    {
        let mut matches = Vec::new();

        let collector = |node: serde_json::Value, node_path: String, depth: usize| -> serde_json::Value {
            if predicate(&node, &node_path, depth) {
                let mut match_map = HashMap::new();
                match_map.insert("value".to_string(), node);
                match_map.insert("path".to_string(), serde_json::Value::String(node_path.clone()));
                match_map.insert("depth".to_string(), serde_json::Value::Number(depth.into()));
                matches.push(match_map);
            }
            node
        };

        self.walk_and_process(data, collector, path)?;
        Ok(matches)
    }

    /// Transform only leaf nodes (non-container values).
    pub fn transform_leaves<F>(&mut self, data: serde_json::Value, leaf_transformer: F, path: String) -> Result<serde_json::Value, String>
    where
        F: Fn(serde_json::Value) -> serde_json::Value,
    {
        let leaf_processor = |node: serde_json::Value, _node_path: String, _depth: usize| -> serde_json::Value {
            // Check if node is a leaf (not a container)
            if !matches!(node, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
                leaf_transformer(node)
            } else {
                node
            }
        };
        self.walk_and_process(data, leaf_processor, path)
    }
}

/// Walk data structure and replace values according to replacement map.
pub fn walk_and_replace(data: serde_json::Value, replacements: HashMap<serde_json::Value, serde_json::Value>, max_depth: Option<usize>) -> Result<serde_json::Value, String> {
    let mut walker = TreeWalker::new(max_depth, Some(true), None);
    
    let replacer = |node: serde_json::Value, _path: String, _depth: usize| -> serde_json::Value {
        replacements.get(&node).cloned().unwrap_or(node)
    };
    
    walker.walk_and_process(data, replacer, String::new())
}

/// Generic utility to resolve proxy objects in nested data structures.
pub fn resolve_proxies_in_dict(
    data: serde_json::Value,
    resolving_paths: &mut HashSet<String>,
    visited_objects: Option<HashSet<usize>>,
    max_depth: Option<usize>,
) -> Result<serde_json::Value, String> {
    let mut walker = TreeWalker::new(max_depth, Some(true), visited_objects);
    
    let proxy_resolver = |node: serde_json::Value, path: String, _depth: usize| -> serde_json::Value {
        // In Rust, we can't easily check for methods on serde_json::Value
        // This would need to be handled differently - for now, just return the node
        // The Python version checks for hasattr(node, "resolve") which isn't directly translatable
        node
    };
    
    walker.walk_and_process(data, proxy_resolver, String::new())
}

/// Apply user-defined link processing to data structure.
pub fn apply_user_defined_links<F>(
    data: serde_json::Value,
    link_processor: F,
    link_key: Option<&str>,
    max_depth: Option<usize>,
) -> Result<serde_json::Value, String>
where
    F: Fn(String, String) -> serde_json::Value,
{
    let mut walker = TreeWalker::new(max_depth, Some(true), None);
    let key = link_key.unwrap_or("_link");
    
    let link_replacer = |node: serde_json::Value, path: String, _depth: usize| -> serde_json::Value {
        if let serde_json::Value::Object(ref map) = node {
            if let Some(link_value) = map.get(key) {
                if let Some(link_str) = link_value.as_str() {
                    let mut processed_node = map.clone();
                    processed_node.insert(key.to_string(), link_processor(link_str.to_string(), path));
                    return serde_json::Value::Object(processed_node);
                }
            }
        }
        node
    };
    
    walker.walk_and_process(data, link_replacer, String::new())
}

/// Count nodes in data structure by type.
pub fn count_nodes_by_type(data: serde_json::Value, max_depth: Option<usize>) -> Result<HashMap<String, i64>, String> {
    let mut walker = TreeWalker::new(max_depth, Some(true), None);
    let mut type_counts = HashMap::new();
    
    let type_counter = |node: serde_json::Value, _path: String, _depth: usize| -> serde_json::Value {
        let node_type = match &node {
            serde_json::Value::Null => "Null",
            serde_json::Value::Bool(_) => "Bool",
            serde_json::Value::Number(_) => "Number",
            serde_json::Value::String(_) => "String",
            serde_json::Value::Array(_) => "Array",
            serde_json::Value::Object(_) => "Object",
        };
        *type_counts.entry(node_type.to_string()).or_insert(0) += 1;
        node
    };
    
    walker.walk_and_process(data, type_counter, String::new())?;
    Ok(type_counts)
}

/// Find paths that exceed a minimum depth threshold.
pub fn find_deep_paths(data: serde_json::Value, min_depth: Option<usize>, max_depth: Option<usize>) -> Result<Vec<String>, String> {
    let mut walker = TreeWalker::new(max_depth, Some(true), None);
    let min = min_depth.unwrap_or(5);
    let mut deep_paths = Vec::new();
    
    let depth_tracker = |node: serde_json::Value, path: String, depth: usize| -> serde_json::Value {
        // Only report leaf nodes at deep paths
        if depth >= min && !matches!(node, serde_json::Value::Object(_) | serde_json::Value::Array(_)) {
            deep_paths.push(format!("{} (depth: {})", path, depth));
        }
        node
    };
    
    walker.walk_and_process(data, depth_tracker, String::new())?;
    Ok(deep_paths)
}
