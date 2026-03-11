// #exonware/xwsystem/rust/src/io/file/base.rs
//exonware/xwsystem/src/exonware/xwsystem/io/file/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Base classes for file operations.
//! 
//! Like codec/base.py: contains abstract bases + utilities.
//! 
//! Priority 1 (Security): Safe base implementations
//! Priority 2 (Usability): Easy to extend
//! Priority 3 (Maintainability): Clean abstractions
//! Priority 4 (Performance): Efficient operations
//! Priority 5 (Extensibility): Ready for new file types


use crate::contracts::{IFileSource, IPagedSource, IPagingStrategy};
use crate::defs::{PagingMode};
use std::path::{Path};

/// Abstract base for file data sources.
///
/// Provides common file source functionality.
pub trait AFileSource: IFileSource {
    /// Return file:// URI.
    // Python decorators: @property
    fn uri(&self) -> &str
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Return scheme identifier.
    // Python decorators: @property
    fn scheme(&self) -> &str
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Check if file exists.
    fn exists(&self) -> bool
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Delete file.
    fn delete(&self) -> ()
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Read entire file content.
    fn read(&self) -> Vec<u8>;

    /// Write entire content to file.
    fn write(&self, data: Vec<u8>) -> ();

}

// Auto-detect strategy if not provided
/// Abstract base for paged file sources.
///
/// Provides common paged reading functionality.
/// Uses pluggable paging strategies!
pub trait APagedSource: IPagedSource {
    /// Total file size in bytes.
    // Python decorators: @property
    fn total_size(&self) -> i64
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Read specific page using strategy.
    fn read_page(&self, page: i64, page_size: i64) -> Vec<u8>
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Iterate over pages using strategy.
    fn iter_pages(&self, page_size: i64) -> ()
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// Read chunk by byte offset.
    fn read_chunk(&self, offset: i64, size: i64) -> Vec<u8>;

    /// Iterate over chunks.
    fn iter_chunks(&self, chunk_size: i64) -> ();

}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use AFileSource;
pub use APagedSource;
