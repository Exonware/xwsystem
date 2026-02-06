// #exonware/xwsystem/rust/src/io/file/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/file/__init__.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! File-specific implementations and utilities.
//! 
//! Following codec/ pattern with MODULAR PAGING SYSTEM!
//! 
//! Priority 1 (Security): Safe file operations with validation
//! Priority 2 (Usability): Simple file API + auto-detection
//! Priority 3 (Maintainability): Focused file-specific code
//! Priority 4 (Performance): Efficient file I/O + pluggable strategies
//! Priority 5 (Extensibility): Easy to add new file features & paging strategies

// Contracts
pub mod base;
pub mod conversion;
pub mod file;
pub mod paged_source;
pub mod paging;
pub mod source;

pub use base::{AFileSource, APagedSource};

pub use conversion::{FormatConverter, convert_file};

pub use file::{XWFile};

pub use paged_source::{PagedFileSource};

pub use paging::{BytePagingStrategy, LinePagingStrategy, PagingStrategyRegistry, RecordPagingStrategy, auto_detect_paging_strategy, get_global_paging_registry, get_paging_strategy, register_paging_strategy};

pub use source::{FileDataSource};
