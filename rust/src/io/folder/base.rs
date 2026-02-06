// #exonware/xwsystem/rust/src/io/folder/base.rs
//exonware/xwsystem/src/exonware/xwsystem/io/folder/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Base classes for folder operations.


use crate::contracts::{IFolderSource};
use std::option::{Option};
use std::path::{Path};

/// Abstract base for folder sources.
pub trait AFolderSource: IFolderSource {
    /// Create directory.
    fn create(&self, parents: bool, exist_ok: bool) -> bool;

    /// Delete directory.
    fn delete(&self, recursive: bool) -> bool;

}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use AFolderSource;
