// #exonware/xwsystem/rust/src/patterns/mod.rs
//! XSystem Patterns Package
//! 
//! Provides design patterns and utilities for common programming patterns.

pub mod context_manager;
pub mod handler_factory;
pub mod import_registry;
pub mod object_pool;

pub use context_manager::{ContextualLogger, ThreadSafeSingleton, combine_contexts, create_operation_logger, enhanced_error_context};

pub use handler_factory::{GenericHandlerFactory};

pub use import_registry::{register_imports_batch, register_imports_flat, register_imports_tree};

pub use object_pool::{ObjectPool, PooledObject, clear_object_pool, get_all_pool_stats, get_object_pool};
