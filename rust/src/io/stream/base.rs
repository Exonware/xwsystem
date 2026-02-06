// #exonware/xwsystem/rust/src/io/stream/base.rs
//exonware/xwsystem/src/exonware/xwsystem/io/stream/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Base classes for stream operations.


use crate::contracts::{ICodecIO, IPagedCodecIO};

/// Abstract base for codec I/O operations.
pub trait ACodecIO {
    /// The codec.
    // Python decorators: @property
    fn codec(&self) -> ()
 {
        // TODO: Implement default behavior
        todo!()
    }

    /// The source.
    // Python decorators: @property
    fn source(&self) -> ()
 {
        // TODO: Implement default behavior
        todo!()
    }

}

/// Abstract base for paged codec I/O.
pub trait APagedCodecIO {
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use ACodecIO;
pub use APagedCodecIO;
