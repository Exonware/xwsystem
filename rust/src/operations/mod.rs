// #exonware/xwsystem/rust/src/operations/mod.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 26, 2025
//! 
//! Universal operations library for data manipulation.

pub mod base;
pub mod contracts;
pub mod defs;
pub mod diff;
pub mod merge;
pub mod patch;

pub use base::{DiffError, MergeError, OperationError, PatchError};

pub use contracts::{IDiffOperation, IMergeOperation, IOperation, IPatchOperation};

pub use defs::{DiffMode, DiffResult, MergeStrategy, PatchOperation, PatchResult};

pub use diff::{DiffOperation, generate_diff};

pub use merge::{MergeOperation, deep_merge};

pub use patch::{PatchOperationImpl, apply_patch};
