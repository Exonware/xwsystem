// #exonware/xwsystem/rust/src/io/common/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/common/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Common utilities shared across IO operations.
//! 
//! Following codec/ pattern:
//! - contracts.py: Interfaces
//! - defs.py: Enums
//! - base.py: Abstract classes
//! - errors.py: Exceptions
//! - [concrete files]: Implementations
//! 
//! Priority 1 (Security): Safe atomic operations, path validation
//! Priority 2 (Usability): Simple, reusable utilities
//! Priority 3 (Maintainability): Centralized common code
//! Priority 4 (Performance): Efficient helper functions
//! Priority 5 (Extensibility): Easy to add new utilities

// Core abstractions
pub mod atomic;
pub mod base;
pub mod lock;
pub mod path_manager;
pub mod watcher;

pub use atomic::{AtomicFileWriter, FileOperationError, safe_read_bytes, safe_read_text, safe_read_with_fallback, safe_write_bytes, safe_write_text};

pub use base::{AAtomicWriter, AFileLock, AFileWatcher, APathValidator};

pub use lock::{FileLock};

pub use path_manager::{PathManager};

pub use watcher::{FileWatcher};
