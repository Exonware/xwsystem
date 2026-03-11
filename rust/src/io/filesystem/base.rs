// #exonware/xwsystem/rust/src/io/filesystem/base.rs
//exonware/xwsystem/src/exonware/xwsystem/io/filesystem/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Base classes for virtual filesystem.


use crate::contracts::{IVirtualFS};

/// Abstract base for filesystem implementations.
pub trait AFileSystem: IVirtualFS {
    /// URI scheme.
    // Python decorators: @property
    fn scheme(&self) -> &str;

}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use AFileSystem;
