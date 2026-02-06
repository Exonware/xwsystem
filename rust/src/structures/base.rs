// #exonware/xwsystem/rust/src/structures/base.rs
//exonware/xwsystem/structures/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Structures module base classes - abstract classes for data structure functionality.


use std::collections::HashMap;

use crate::contracts::{StructureType, TraversalType, ValidationLevel};

/// Abstract base class for data structures.
pub trait AStructureBase {
    /// Add element to structure.
    fn add(&self, element: serde_json::Value) -> bool;

    /// Remove element from structure.
    fn remove(&self, element: serde_json::Value) -> bool;

    /// Check if structure contains element.
    fn contains(&self, element: serde_json::Value) -> bool;

    /// Get structure size.
    fn size(&self) -> i64;

    /// Check if structure is empty.
    fn is_empty(&self) -> bool;

    /// Clear all elements from structure.
    fn clear(&self) -> ();

    /// Convert structure to list.
    fn to_list(&self) -> Vec<serde_json::Value>;

    /// Initialize structure from list.
    fn from_list(&self, elements: Vec<serde_json::Value>) -> ();

    /// Validate structure.
    fn validate(&self, validation_level: ValidationLevel) -> bool;

}

/// Abstract base class for tree structures.
pub trait ATreeBase {
    /// Insert value into tree.
    fn insert(&self, value: serde_json::Value) -> bool;

    /// Delete value from tree.
    fn delete(&self, value: serde_json::Value) -> bool;

    /// Search for value in tree.
    fn search(&self, value: serde_json::Value) -> Option<serde_json::Value>;

    /// Get tree root.
    fn get_root(&self) -> Option<serde_json::Value>;

    /// Get tree height.
    fn get_height(&self) -> i64;

    /// Get tree size.
    fn get_size(&self) -> i64;

    /// Check if tree is empty.
    fn is_empty(&self) -> bool;

    /// Clear tree.
    fn clear(&self) -> ();

    /// Traverse tree.
    fn traverse(&self, traversal_type: TraversalType) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Get tree leaves.
    fn get_leaves(&self) -> Vec<serde_json::Value>;

    /// Get internal nodes.
    fn get_internal_nodes(&self) -> Vec<serde_json::Value>;

}

/// Abstract base class for graph structures.
pub trait AGraphBase {
    /// Add vertex to graph.
    fn add_vertex(&self, vertex: serde_json::Value) -> bool;

    /// Remove vertex from graph.
    fn remove_vertex(&self, vertex: serde_json::Value) -> bool;

    /// Add edge to graph.
    fn add_edge(&self, vertex1: serde_json::Value, vertex2: serde_json::Value, weight: Option<f64>) -> bool;

    /// Remove edge from graph.
    fn remove_edge(&self, vertex1: serde_json::Value, vertex2: serde_json::Value) -> bool;

    /// Check if graph has vertex.
    fn has_vertex(&self, vertex: serde_json::Value) -> bool;

    /// Check if graph has edge.
    fn has_edge(&self, vertex1: serde_json::Value, vertex2: serde_json::Value) -> bool;

    /// Get all vertices.
    fn get_vertices(&self) -> Vec<serde_json::Value>;

    /// Get all edges.
    fn get_edges(&self) -> Vec<(serde_json::Value, serde_json::Value)>;

    /// Get vertex neighbors.
    fn get_neighbors(&self, vertex: serde_json::Value) -> Vec<serde_json::Value>;

    /// Get vertex degree.
    fn get_degree(&self, vertex: serde_json::Value) -> i64;

    /// Get vertex count.
    fn get_vertex_count(&self) -> i64;

    /// Get edge count.
    fn get_edge_count(&self) -> i64;

    /// Check if graph is connected.
    fn is_connected(&self) -> bool;

    /// Check if graph is cyclic.
    fn is_cyclic(&self) -> bool;

}

/// Abstract base class for circular reference detection.
pub trait ACircularDetectorBase {
    /// Detect circular references in object.
    fn detect_circular_references(&self, obj: serde_json::Value, max_depth: i64) -> Vec<Vec<serde_json::Value>>;

    /// Check if object has circular references.
    fn has_circular_reference(&self, obj: serde_json::Value, max_depth: i64) -> bool;

    /// Get detected circular paths.
    fn get_circular_paths(&self) -> Vec<Vec<serde_json::Value>>;

    /// Clear detection cache.
    fn clear_detection_cache(&self) -> ();

    /// Break circular references in object.
    fn break_circular_references(&self, obj: serde_json::Value) -> serde_json::Value;

    /// Get reference count for object.
    fn get_reference_count(&self, obj: serde_json::Value) -> i64;

    /// Get object size in bytes.
    fn get_object_size(&self, obj: serde_json::Value) -> i64;

    /// Get memory usage statistics.
    fn get_memory_usage(&self, obj: serde_json::Value) -> HashMap<String, i64>;

}

/// Abstract base class for tree traversal.
pub trait ATreeWalkerBase {
    /// Walk tree with specified traversal type.
    fn walk(&self, root: serde_json::Value, traversal_type: TraversalType) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Walk tree in preorder.
    fn walk_preorder(&self, root: serde_json::Value) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Walk tree in inorder.
    fn walk_inorder(&self, root: serde_json::Value) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Walk tree in postorder.
    fn walk_postorder(&self, root: serde_json::Value) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Walk tree in level order.
    fn walk_level_order(&self, root: serde_json::Value) -> Box<dyn Iterator<Item = serde_json::Value>>;

    /// Find path to target node.
    fn find_path(&self, root: serde_json::Value, target: serde_json::Value) -> Option<Vec<serde_json::Value>>;

    /// Get depth of target node.
    fn get_depth(&self, root: serde_json::Value, target: serde_json::Value) -> i64;

    /// Get ancestors of target node.
    fn get_ancestors(&self, root: serde_json::Value, target: serde_json::Value) -> Vec<serde_json::Value>;

    /// Get descendants of target node.
    fn get_descendants(&self, root: serde_json::Value, target: serde_json::Value) -> Vec<serde_json::Value>;

    /// Get siblings of target node.
    fn get_siblings(&self, root: serde_json::Value, target: serde_json::Value) -> Vec<serde_json::Value>;

    /// Get all leaf nodes.
    fn get_leaf_nodes(&self, root: serde_json::Value) -> Vec<serde_json::Value>;

    /// Get all internal nodes.
    fn get_internal_nodes(&self, root: serde_json::Value) -> Vec<serde_json::Value>;

}

/// Abstract base class for structure validation.
pub trait AStructureValidatorBase {
    /// Validate data structure.
    fn validate_structure(&self, structure: serde_json::Value) -> bool;

    /// Validate tree structure.
    fn validate_tree(&self, tree: serde_json::Value) -> bool;

    /// Validate graph structure.
    fn validate_graph(&self, graph: serde_json::Value) -> bool;

    /// Validate for circular references.
    fn validate_circular_references(&self, obj: serde_json::Value) -> bool;

    /// Get validation errors.
    fn get_validation_errors(&self) -> Vec<String>;

    /// Clear validation errors.
    fn clear_validation_errors(&self) -> ();

    /// Add validation rule.
    fn add_validation_rule(&self, rule_name: String, rule_func: String) -> ();

    /// Remove validation rule.
    fn remove_validation_rule(&self, rule_name: String) -> ();

    /// Get validation score.
    fn get_validation_score(&self) -> f64;

}

/// Base implementation of data structure.
pub struct BaseStructure {
    pub structure_type: StructureType,
    _size: usize,
    _elements: Vec<serde_json::Value>,
}

impl AStructureBase for BaseStructure {
    fn add(&mut self, element: serde_json::Value) -> bool {
        self._elements.push(element);
        self._size += 1;
        true
    }

    fn remove(&mut self, element: serde_json::Value) -> bool {
        if let Some(pos) = self._elements.iter().position(|e| e == &element) {
            self._elements.remove(pos);
            self._size -= 1;
            true
        } else {
            false
        }
    }

    fn contains(&self, element: &serde_json::Value) -> bool {
        self._elements.contains(element)
    }

    fn size(&self) -> i64 {
        self._size as i64
    }

    fn is_empty(&self) -> bool {
        self._size == 0
    }

    fn clear(&mut self) {
        self._elements.clear();
        self._size = 0;
    }

    fn to_list(&self) -> Vec<serde_json::Value> {
        self._elements.clone()
    }

    fn from_list(&mut self, elements: Vec<serde_json::Value>) {
        self._elements = elements;
        self._size = self._elements.len();
    }

    fn validate(&self, validation_level: ValidationLevel) -> bool {
        if validation_level == ValidationLevel::NONE {
            return true;
        }
        
        // Basic validation
        if self._size != self._elements.len() {
            return false;
        }
        
        if matches!(validation_level, ValidationLevel::STRICT | ValidationLevel::COMPREHENSIVE) {
            // Check for None/null elements
            if self._elements.iter().any(|e| e.is_null()) {
                return false;
            }
        }
        
        true
    }
}

impl BaseStructure {
    /// Initialize base structure.
    pub fn new(structure_type: Option<StructureType>) -> Self {
        Self {
            structure_type: structure_type.unwrap_or(StructureType::GENERIC),
            _size: 0,
            _elements: Vec::new(),
        }
    }
}
