// #exonware/xwsystem/rust/src/io/file/paging/mod.rs
//exonware/xwsystem/src/exonware/xwsystem/io/file/paging/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Modular paging system with strategy pattern.
//! 
//! Like codec system but for paging algorithms!
//! 
//! Priority 1 (Security): Safe paging operations
//! Priority 2 (Usability): Auto-detection of best strategy
//! Priority 3 (Maintainability): Pluggable strategies
//! Priority 4 (Performance): Efficient strategy selection
//! Priority 5 (Extensibility): Easy to add new strategies

pub mod byte_paging;
pub mod line_paging;
pub mod record_paging;
pub mod registry;

pub use byte_paging::{BytePagingStrategy};

pub use line_paging::{LinePagingStrategy};

pub use record_paging::{RecordPagingStrategy};

pub use registry::{PagingStrategyRegistry, auto_detect_paging_strategy, get_global_paging_registry, get_paging_strategy, register_paging_strategy};
