// #exonware/xwsystem/rust/src/structures/defs.rs
//exonware/xwsystem/structures/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Structures types and enums for XWSystem.


use serde::{Serialize, Deserialize};

use crate::shared::defs::{ValidationLevel};

/// Data structure types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructureType {
    Tree,
    Graph,
    List,
    Dict,
    Set,
    Queue,
    Stack,
    Heap,
    Generic,
    Custom,
}

/// Tree traversal orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraversalOrder {
    Preorder,
    Inorder,
    Postorder,
    #[serde(rename = "level_order")]
    LevelOrder,
}

/// Traversal types for data structures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraversalType {
    #[serde(rename = "depth_first")]
    DepthFirst,
    #[serde(rename = "breadth_first")]
    BreadthFirst,
    Iterative,
    Recursive,
    Inorder,
    Preorder,
    Postorder,
    #[serde(rename = "level_order")]
    LevelOrder,
}

/// Graph types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GraphType {
    Directed,
    Undirected,
    Weighted,
    Unweighted,
}

/// Circular detection methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircularDetectionMethod {
    Dfs,
    Bfs,
    Tarjan,
    Kahn,
}
