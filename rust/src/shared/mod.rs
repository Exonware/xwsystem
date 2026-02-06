// #exonware/xwsystem/rust/src/shared/mod.rs
//exonware/xwsystem/shared/__init__.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 10-Sep-2025
//! 
//! Shared types and utilities for XWSystem modules.

pub mod base;
pub mod contracts;
pub mod defs;
pub mod errors;

pub use base::{AConfigurationBase, ACoreBase, AObject, AOperationBase, AResourceManagerBase, AValidationBase, BaseCore, XWObject};

pub use contracts::{ICloneable, IComparable, IContainer, ICore, IFactory, IID, IIterable, ILifecycle, IMetadata, INative, IObject, IStringable};

pub use defs::{AuthType, CloneMode, CoreMode, CorePriority, CoreState, DataType, LockType, LogLevel, OperationResult, PathType, PerformanceLevel, ValidationLevel};

pub use errors::{CoreConfigurationError, CoreDependencyError, CoreError, CoreInitializationError, CoreOperationError, CorePermissionError, CoreResourceError, CoreShutdownError, CoreStateError, CoreTimeoutError, CoreValidationError};
