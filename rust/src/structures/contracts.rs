// #exonware/xwsystem/rust/src/structures/contracts.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Structures module contracts - interfaces and enums for data structure functionality.


use crate::defs::{CircularDetectionMethod, GraphType, StructureType, TraversalOrder, TraversalType, ValidationLevel};

/// Interface for tree node operations.
pub trait ITreeNode {
    /// Node value.
    // Python decorators: @property
    fn value(&self) -> serde_json::Value;

    /// Node children.
    // Python decorators: @property
    fn children(&self) -> Vec<String>;

    /// Node parent.
    // Python decorators: @property
    fn parent(&self) -> Option<String>;

    /// Add child node.
    fn add_child(&self, child: String) -> ();

    /// Remove child node.
    fn remove_child(&self, child: String) -> ();

    /// Check if node is leaf.
    fn is_leaf(&self) -> bool;

    /// Get node depth.
    fn get_depth(&self) -> i64;

}

/// Interface for tree walking operations.
pub trait ITreeWalker {
    /// Walk tree in preorder.
    fn walk_preorder(&self, root: ITreeNode) -> Box<dyn Iterator<Item = ITreeNode>>;

    /// Walk tree in inorder.
    fn walk_inorder(&self, root: ITreeNode) -> Box<dyn Iterator<Item = ITreeNode>>;

    /// Walk tree in postorder.
    fn walk_postorder(&self, root: ITreeNode) -> Box<dyn Iterator<Item = ITreeNode>>;

    /// Walk tree in level order.
    fn walk_level_order(&self, root: ITreeNode) -> Box<dyn Iterator<Item = ITreeNode>>;

    /// Find nodes matching predicate.
    fn find_nodes(&self, root: ITreeNode, predicate: fn()) -> Vec<ITreeNode>;

}

/// Interface for circular reference detection.
pub trait ICircularDetector {
    /// Detect circular reference in object.
    fn detect_circular_reference(&self, obj: serde_json::Value) -> bool;

    /// Find all circular paths.
    fn find_circular_paths(&self, obj: serde_json::Value) -> Vec<Vec<serde_json::Value>>;

    /// Get objects involved in circular references.
    fn get_circular_objects(&self, obj: serde_json::Value) -> Vec<serde_json::Value>;

    /// Break circular references.
    fn break_circular_references(&self, obj: serde_json::Value) -> serde_json::Value;

}

/// Interface for graph node operations.
pub trait IGraphNode {
    /// Node ID.
    // Python decorators: @property
    fn id(&self) -> &str;

    /// Node data.
    // Python decorators: @property
    fn data(&self) -> serde_json::Value;

    /// Node neighbors.
    // Python decorators: @property
    fn neighbors(&self) -> Vec<String>;

    /// Add neighbor node.
    fn add_neighbor(&self, neighbor: String, weight: Option<f64>) -> ();

    /// Remove neighbor node.
    fn remove_neighbor(&self, neighbor: String) -> ();

    /// Get edge weight to neighbor.
    fn get_edge_weight(&self, neighbor: String) -> Option<f64>;

}

/// Interface for graph operations.
pub trait IGraph {
    /// Add node to graph.
    fn add_node(&self, node: IGraphNode) -> ();

    /// Remove node from graph.
    fn remove_node(&self, node_id: String) -> ();

    /// Get node by ID.
    fn get_node(&self, node_id: String) -> Option<IGraphNode>;

    /// Add edge between nodes.
    fn add_edge(&self, from_node: String, to_node: String, weight: Option<f64>) -> ();

    /// Remove edge between nodes.
    fn remove_edge(&self, from_node: String, to_node: String) -> ();

    /// Get all nodes.
    fn get_all_nodes(&self) -> Vec<IGraphNode>;

    /// Get all edges.
    fn get_all_edges(&self) -> Vec<()>;

    /// Check if nodes are connected.
    fn is_connected(&self, from_node: String, to_node: String) -> bool;

}
