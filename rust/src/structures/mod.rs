// #exonware/xwsystem/rust/src/structures/mod.rs
//! Data structure utilities for xwsystem.

pub mod circular_detector;
pub mod tree_walker;

pub use circular_detector::{CircularReferenceDetector, CircularReferenceError};

pub use tree_walker::{TreeWalker, apply_user_defined_links, count_nodes_by_type, find_deep_paths, resolve_proxies_in_dict, walk_and_replace};
