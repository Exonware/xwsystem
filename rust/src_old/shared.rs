// #exonware/xwsystem/rust/src_old/shared.rs
//! Shared types and utilities for XWSystem modules.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module re-exports shared types, enums, errors, and contracts.
//! Matches Python's shared/__init__.py exports 100%.

// Re-export defs (matching Python shared/__init__.py exports)
pub use crate::defs::{
    ValidationLevel, PerformanceLevel, AuthType, LockType, PathType, LogLevel,
    OperationResult, DataType, CloneMode, CoreState, CoreMode, CorePriority,
};

// Re-export errors (matching Python shared/__init__.py exports)
pub use crate::errors::{
    CoreError, CoreInitializationError, CoreShutdownError, CoreStateError,
    CoreDependencyError, CoreConfigurationError, CoreResourceError, CoreTimeoutError,
    CorePermissionError, CoreValidationError, CoreOperationError,
};

// Re-export contracts (matching Python shared/__init__.py exports)
pub use crate::contracts::{
    IId, IStringable, INative, ICloneable, IComparable, IIterable,
    IContainer, IMetadata, ILifecycle, IFactory, ICore, IObject,
};

// Re-export base classes (matching Python shared/base.py exports)
pub use crate::base::{
    ACoreBase, AResourceManagerBase, AConfigurationBase, AValidationBase,
    AOperationBase, BaseCore, AObject, AObjectBase, XWObject,
};

