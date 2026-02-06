// #exonware/xwsystem/rust/src/runtime/mod.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! XSystem Runtime Package
//! 
//! Provides runtime utilities for environment detection, path management,
//! and reflection capabilities.

pub mod base;
pub mod contracts;
pub mod defs;
pub mod env;
pub mod errors;
pub mod reflection;

pub use base::{AEnvironmentBase, APlatformBase, APythonBase, AReflectionBase, ARuntimeBase, ARuntimeManagerBase, BaseRuntime};

pub use contracts::{IEnvironmentManager, IPlatformInfo, IPythonInfo, IReflectionUtils, IRuntimeConfig};
pub use defs::{EnvironmentType, PlatformType, PythonVersion, RuntimeMode};

pub use env::{EnvironmentManager};

pub use errors::{AttributeNotFoundError, ClassNotFoundError, EnvironmentError, EnvironmentTypeError, EnvironmentVariableError, FunctionNotFoundError, ModuleNotFoundError, PlatformError, PlatformInfoError, PlatformNotSupportedError, PythonError, PythonPackageError, PythonVersionError, ReflectionError, RuntimeConfigError, RuntimeError, RuntimeInitializationError, RuntimeModeError, RuntimeShutdownError};

pub use reflection::{ReflectionUtils};
