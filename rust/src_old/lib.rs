// #exonware/xwsystem/rust/src_old/lib.rs
//! XWSystem Rust Implementation
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This crate provides Rust implementations of XWSystem modules.
//! This module mirrors Python's xwsystem module 100%.

pub mod version;
pub mod dummy;

// Core modules
pub mod defs;
pub mod errors;
pub mod contracts;
pub mod shared;
pub mod base;

// I/O modules
pub mod io;

// Python bindings (only compiled when building Python extension)
#[cfg(feature = "python")]
pub mod python_bindings;

// ============================================================================
// ROOT-LEVEL EXPORTS (matching Python xwsystem/__init__.py)
// ============================================================================

// Re-export version
pub use version::{VERSION, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_BUILD, VERSION_STRING};

// Re-export defs (root-level enums and constants)
pub use defs::{
    // Root-level enums
    SystemStatus, ComponentType, ErrorSeverity, LogCategory,
    // Shared enums
    ValidationLevel, PerformanceLevel, AuthType, LockType, PathType, LogLevel,
    OperationResult, DataType, CloneMode, CoreState, CoreMode, CorePriority,
    // Config enums
    ConfigSource, ConfigFormat, ConfigPriority, ConfigType, PerformanceMode, AdvancedPerformanceMode,
    // Constants
    VERSION_MAJOR as DEFS_VERSION_MAJOR, VERSION_MINOR as DEFS_VERSION_MINOR,
    VERSION_PATCH as DEFS_VERSION_PATCH, VERSION_BUILD as DEFS_VERSION_BUILD,
    VERSION_STRING as DEFS_VERSION_STRING, DEFAULT_TIMEOUT, DEFAULT_RETRY_COUNT,
    DEFAULT_RETRY_DELAY, MAX_RETRY_DELAY, DEFAULT_CONFIG_FILE, DEFAULT_CONFIG_DIR,
    DEFAULT_LOG_DIR, DEFAULT_CACHE_DIR, MIN_PASSWORD_LENGTH, MAX_PASSWORD_LENGTH,
    DEFAULT_HASH_ALGORITHM, DEFAULT_ENCRYPTION_ALGORITHM, DEFAULT_CACHE_SIZE,
    DEFAULT_CACHE_TTL, DEFAULT_THREAD_POOL_SIZE, DEFAULT_MAX_WORKERS,
    MAX_DEPTH_DEFAULT, MAX_SIZE_DEFAULT, MAX_ITEMS_DEFAULT, DEFAULT_METRICS_INTERVAL,
    DEFAULT_TRACE_SAMPLE_RATE, DEFAULT_LOG_LEVEL, DEFAULT_PLUGIN_DIR, DEFAULT_PLUGIN_TIMEOUT,
    MAX_PLUGIN_SIZE, DEFAULT_HTTP_TIMEOUT, DEFAULT_HTTP_RETRIES,
    DEFAULT_HTTP_RETRY_DELAY, DEFAULT_MAX_REDIRECTS, DEFAULT_IPC_TIMEOUT,
    DEFAULT_MESSAGE_QUEUE_SIZE, DEFAULT_SHARED_MEMORY_SIZE, DEFAULT_PROMPT,
    DEFAULT_PROGRESS_WIDTH, DEFAULT_TABLE_WIDTH,
    // Type aliases
    ConfigDict, ConfigList, ErrorDict, MetricsDict, MetadataDict,
    ConfigCallback, ErrorCallback, MetricsCallback, LifecycleCallback,
};

// Re-export errors (root-level errors)
pub use errors::{
    // Root error classes
    XWSystemError, XWSystemInitializationError, XWSystemConfigurationError,
    XWSystemStateError, XWSystemDependencyError, XWSystemResourceError,
    XWSystemTimeoutError, XWSystemPermissionError, XWSystemValidationError,
    XWSystemOperationError,
    // Core errors
    CoreError, CoreInitializationError, CoreShutdownError, CoreStateError,
    CoreDependencyError, CoreConfigurationError, CoreResourceError, CoreTimeoutError,
    CorePermissionError, CoreValidationError, CoreOperationError,
};

// Re-export contracts (root-level interfaces)
pub use contracts::{
    // Root-level interfaces
    IXWSystem, ISystemComponent, IExtensible, IConfigurableComponent,
    IMonitoredComponent, ISecureComponent, IConfigurable,
    // Core contracts
    IId, IStringable, INative, ICloneable, IComparable, IIterable,
    IContainer, IMetadata, ILifecycle, IFactory, ICore, IObject,
};

// Re-export base classes (root-level base classes)
pub use base::{
    // Root abstract base classes
    AXWSystemBase, BaseXWSystem, ASystemComponent, BaseSystemComponent,
    AConfigurableComponent, BaseConfigurableComponent, AMonitoredComponent,
    ASecureComponent,
    // Shared base classes
    ACoreBase, AResourceManagerBase, BaseCore, BaseResourceManager,
    AConfigurationBase, BaseConfiguration, AValidationBase, BaseValidation,
    AOperationBase, BaseOperation, AObject, AObjectBase, XWObject,
};

// Re-export shared (matching Python shared/__init__.py)
// Note: shared module re-exports are already covered above, so we don't duplicate them here
// The shared module is for internal organization, root-level exports are done above

