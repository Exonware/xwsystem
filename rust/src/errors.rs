// #exonware/xwsystem/rust/src/errors.rs
//exonware/xwsystem/src/exonware/xwsystem/errors.py
//! Root-level error classes for XWSystem.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Jan-2025
//! 
//! This module provides root-level error classes that serve as the base
//! error hierarchy for all XWSystem components.


// ============================================================================
// ROOT ERROR CLASSES
// ============================================================================


use crate::caching::errors::{CacheConfigurationError, CacheDeserializationError, CacheError, CacheEvictionError, CacheIntegrityError, CacheKeyError, CacheOperationError, CacheSerializationError, CacheSizeError, CacheStatsError, CacheTimeoutError, CacheValueError, CacheWarmingError};
use crate::cli::errors::{ArgsError, CLIError, ColorError, ConsoleError, ProgressError, PromptError, TableError};
use crate::http_client::errors::{HttpAuthenticationError, HttpAuthorizationError, HttpCircuitBreakerError, HttpClientError, HttpConnectionError, HttpRequestError, HttpResponseError, HttpRetryError, HttpTimeoutError};
use crate::ipc::errors::{AsyncFabricError, IPCError, MessageQueueError, PipeError, ProcessManagerError, ProcessPoolError, SharedMemoryError};
use crate::monitoring::errors::{ErrorRecoveryError, MemoryMonitorError, MetricsError, MonitoringError, PerformanceError, SystemMonitorError, TracingError};
use crate::operations::errors::{DiffError, MergeError, OperationError, PatchError};
use crate::patterns::errors::{AdapterError, AggregateError, ArchitecturalError, BuilderError, ChainHandlerError, CommandError, ConcurrencyError as PatternConcurrencyError, DecoratorError, FacadeError, FactoryError, HandlerError, IteratorError, MediatorError, MementoError, ObserverError, PatternError, PrototypeError, ProxyError, RegistryError, SpecificationError, StateError, StrategyError, ValueObjectError, VisitorError};
use crate::plugins::errors::{PluginBackupError, PluginCapabilityError, PluginCleanupError, PluginCommunicationError, PluginCompatibilityError, PluginConfigurationError, PluginConflictError, PluginDependencyCycleError, PluginDependencyError, PluginDiscoveryError, PluginDuplicateError, PluginEntryPointError, PluginError, PluginEventError, PluginExecutionError, PluginHookError, PluginHotReloadError, PluginImportError, PluginInitializationError, PluginInterfaceError, PluginLifecycleError, PluginLoadError, PluginManagerError, PluginManagerNotInitializedError, PluginManagerShutdownError, PluginMetadataError, PluginMethodError, PluginMigrationError, PluginMonitoringError, PluginNotFoundError, PluginPermissionError, PluginPriorityError, PluginRegistrationError, PluginRegistryError, PluginRegistryFullError, PluginRegistryLockError, PluginResourceError, PluginSandboxError, PluginSecurityError, PluginStateError, PluginTimeoutError, PluginUnloadError, PluginValidationError, PluginVersionError};
use crate::query::errors::{QueryProviderError, QueryProviderNotRegisteredError};
use crate::runtime::errors::{AttributeNotFoundError, ClassNotFoundError, EnvironmentError as XWEnvironmentError, EnvironmentTypeError, EnvironmentVariableError, FunctionNotFoundError, ModuleNotFoundError, PlatformError, PlatformInfoError, PlatformNotSupportedError, PythonError, PythonPackageError, PythonVersionError, ReflectionError, RuntimeConfigError, RuntimeError as XWRuntimeError, RuntimeInitializationError, RuntimeModeError, RuntimeShutdownError};
use crate::security::errors::{AuthenticationError, AuthorizationError, CryptographicError, DecryptionError, EncryptionError, HashError, JWTError, KeyError as SecurityKeyError, KeyGenerationError, KeyValidationError, OAuth2Error, PathSecurityError, PathTraversalError, ResourceLimitError, ResourceQuotaError, ResourceTimeoutError, SAMLError, SecurityConfigurationError, SecurityError, SecurityPermissionError, SecurityPolicyError, SecurityValidationError, SignatureError, TokenExpiredError};
use crate::shared::errors::{CoreConfigurationError, CoreDependencyError, CoreError, CoreInitializationError, CoreOperationError, CorePermissionError, CoreResourceError, CoreShutdownError, CoreStateError, CoreTimeoutError, CoreValidationError};
use crate::structures::errors::{CircularBreakError, CircularDetectionError, CircularReferenceError, GraphEdgeError, GraphError, GraphNodeError, GraphTraversalError, GraphValidationError, StructureError, StructureIndexError, StructureKeyError, StructureOperationError, StructureSizeError, StructureTypeError, StructureValidationError, TreeError, TreeNodeError, TreeTraversalError, TreeValidationError};
use crate::utils::dt::errors::{DateRangeError, DateTimeError, DateTimeFormatError, DateTimeOverflowError, DateTimeParseError, DateTimeUnderflowError, DateTimeValidationError, HumanizeError, TimeCalculationError, TimezoneConversionError, TimezoneError, TimezoneNotFoundError};
use crate::utils::errors::{ConfigLoadError, ConfigManagerError, ConfigSaveError, ConfigValidationError, LazyLoadError, LazyLoaderError, LazyReloadError, LazyUnloadError, PathNormalizationError, PathResolutionError, PathSanitizationError, PathUtilsError, PathValidationError as UtilsPathValidationError, ResourceAcquisitionError, ResourceExhaustionError, ResourceManagerError, ResourceNotFoundError, ResourceReleaseError, UtilityNotFoundError, UtilityRegistrationError, UtilityRegistryError, UtilityUnregistrationError, UtilsError};
use crate::validation::errors::{ConstraintValidationError, DataValidationError, DeclarativeValidationError, DepthValidationError, FieldValidationError, FormatValidationError, MemoryValidationError, OptionalFieldError, PathValidationError, PatternValidationError, RangeValidationError, RequiredFieldError, SchemaValidationError, TypeCoercionError, TypeSafetyError, TypeValidationError, ValidationConfigurationError, ValidationContextError, ValidationError, ValidationRuleError, ValueValidationError};
use std::sync::errors::{AsyncError, AsyncPrimitiveError, AsyncTimeoutError, ConcurrencyError, DeadlockError, LockAcquisitionError, LockError, LockReleaseError, LockTimeoutError, RaceConditionError, SafeFactoryError, ThreadCreationError, ThreadError, ThreadJoinError, ThreadSafetyError, ThreadStartError, ThreadStopError, ThreadTimeoutError, ThreadingError};

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// Caching errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// Caching errors

// HTTP client errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// Caching errors

// HTTP client errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// Caching errors

// HTTP client errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// Caching errors

// HTTP client errors

// Patterns errors

// ============================================================================

// RE-EXPORTED ERROR CLASSES FROM SUBMODULES

// ============================================================================

// Core errors

// Security errors

// Validation errors

// IO errors - import from serialization module

// type: ignore

// Define common IO errors if not available

// Threading errors

// Runtime errors

// Plugin errors

// Utils errors

// Structures errors

// Operations errors

// Query errors

// DateTime errors

// Monitoring errors

// Caching errors

// HTTP client errors

// Patterns errors

// ============================================================================

// ============================================================================

/// Base exception for all XWSystem errors.
///
/// All XWSystem-specific exceptions should inherit from this class.
#[derive(Debug)]
pub struct XWSystemError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemError {
            message: message.into(),
            source: None,
        }
    }

}

/// Raised when XWSystem initialization fails.
#[derive(Debug)]
pub struct XWSystemInitializationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemInitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemInitializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemInitializationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemInitializationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemInitializationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem configuration is invalid.
#[derive(Debug)]
pub struct XWSystemConfigurationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemConfigurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemConfigurationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemConfigurationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemConfigurationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem state is invalid.
#[derive(Debug)]
pub struct XWSystemStateError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemStateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemStateError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemStateError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemStateError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem dependency is missing or invalid.
#[derive(Debug)]
pub struct XWSystemDependencyError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemDependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemDependencyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemDependencyError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemDependencyError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemDependencyError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem resource operation fails.
#[derive(Debug)]
pub struct XWSystemResourceError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemResourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemResourceError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemResourceError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemResourceError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem operation times out.
#[derive(Debug)]
pub struct XWSystemTimeoutError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemTimeoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemTimeoutError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemTimeoutError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemTimeoutError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem permission is denied.
#[derive(Debug)]
pub struct XWSystemPermissionError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemPermissionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemPermissionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemPermissionError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemPermissionError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemPermissionError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem validation fails.
#[derive(Debug)]
pub struct XWSystemValidationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemValidationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemValidationError {
            message: message.into(),
            source: Some(source),
        }
    }
}

/// Raised when XWSystem operation fails.
#[derive(Debug)]
pub struct XWSystemOperationError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl std::fmt::Display for XWSystemOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for XWSystemOperationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

impl XWSystemOperationError {
    pub fn new(message: impl Into<String>) -> Self {
        XWSystemOperationError {
            message: message.into(),
            source: None,
        }
    }

    pub fn from_source(message: impl Into<String>, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        XWSystemOperationError {
            message: message.into(),
            source: Some(source),
        }
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
pub use XWSystemError;
pub use XWSystemInitializationError;
pub use XWSystemConfigurationError;
pub use XWSystemStateError;
pub use XWSystemDependencyError;
pub use XWSystemResourceError;
pub use XWSystemTimeoutError;
pub use XWSystemPermissionError;
pub use XWSystemValidationError;
pub use XWSystemOperationError;
pub use CoreError;
pub use CoreInitializationError;
pub use CoreShutdownError;
pub use CoreStateError;
pub use CoreDependencyError;
pub use CoreConfigurationError;
pub use CoreResourceError;
pub use CoreTimeoutError;
pub use CorePermissionError;
pub use CoreValidationError;
pub use CoreOperationError;
pub use SecurityError;
pub use CryptographicError;
pub use EncryptionError;
pub use DecryptionError;
pub use HashError;
pub use SignatureError;
pub use SecurityKeyError;
pub use KeyGenerationError;
pub use KeyValidationError;
pub use PathSecurityError;
pub use PathTraversalError;
pub use ResourceLimitError;
pub use ResourceQuotaError;
pub use ResourceTimeoutError;
pub use SecurityValidationError;
pub use SecurityPermissionError;
pub use SecurityConfigurationError;
pub use SecurityPolicyError;
pub use AuthenticationError;
pub use AuthorizationError;
pub use TokenExpiredError;
pub use OAuth2Error;
pub use JWTError;
pub use SAMLError;
pub use ValidationError;
pub use PathValidationError;
pub use DepthValidationError;
pub use MemoryValidationError;
pub use DataValidationError;
pub use TypeValidationError;
pub use ValueValidationError;
pub use RangeValidationError;
pub use FormatValidationError;
pub use PatternValidationError;
pub use ConstraintValidationError;
pub use SchemaValidationError;
pub use FieldValidationError;
pub use RequiredFieldError;
pub use OptionalFieldError;
pub use DeclarativeValidationError;
pub use TypeSafetyError;
pub use TypeCoercionError;
pub use ValidationRuleError;
pub use ValidationContextError;
pub use ValidationConfigurationError;
pub use XWIOError;
pub use SerializationError;
pub use DeserializationError;
pub use FormatError;
pub use CodecError;
pub use ArchiveError;
pub use FileError;
pub use StreamError;
pub use ThreadingError;
pub use ThreadError;
pub use ThreadCreationError;
pub use ThreadStartError;
pub use ThreadStopError;
pub use ThreadJoinError;
pub use ThreadTimeoutError;
pub use LockError;
pub use LockAcquisitionError;
pub use LockReleaseError;
pub use LockTimeoutError;
pub use DeadlockError;
pub use RaceConditionError;
pub use AsyncError;
pub use AsyncPrimitiveError;
pub use AsyncTimeoutError;
pub use SafeFactoryError;
pub use ThreadSafetyError;
pub use ConcurrencyError;
pub use XWRuntimeError;
pub use XWEnvironmentError;
pub use EnvironmentVariableError;
pub use EnvironmentTypeError;
pub use PlatformError;
pub use PlatformNotSupportedError;
pub use PlatformInfoError;
pub use PythonError;
pub use PythonVersionError;
pub use PythonPackageError;
pub use ReflectionError;
pub use ClassNotFoundError;
pub use FunctionNotFoundError;
pub use ModuleNotFoundError;
pub use AttributeNotFoundError;
pub use RuntimeConfigError;
pub use RuntimeModeError;
pub use RuntimeInitializationError;
pub use RuntimeShutdownError;
pub use PluginError;
pub use PluginNotFoundError;
pub use PluginLoadError;
pub use PluginImportError;
pub use PluginDependencyError;
pub use PluginVersionError;
pub use PluginRegistrationError;
pub use PluginDuplicateError;
pub use PluginValidationError;
pub use PluginInitializationError;
pub use PluginConfigurationError;
pub use PluginResourceError;
pub use PluginExecutionError;
pub use PluginMethodError;
pub use PluginTimeoutError;
pub use PluginPermissionError;
pub use PluginCleanupError;
pub use PluginUnloadError;
pub use PluginRegistryError;
pub use PluginRegistryFullError;
pub use PluginRegistryLockError;
pub use PluginManagerError;
pub use PluginManagerNotInitializedError;
pub use PluginManagerShutdownError;
pub use PluginDiscoveryError;
pub use PluginEntryPointError;
pub use PluginMetadataError;
pub use PluginInterfaceError;
pub use PluginLifecycleError;
pub use PluginStateError;
pub use PluginHookError;
pub use PluginEventError;
pub use PluginCommunicationError;
pub use PluginSecurityError;
pub use PluginSandboxError;
pub use PluginCapabilityError;
pub use PluginCompatibilityError;
pub use PluginConflictError;
pub use PluginPriorityError;
pub use PluginDependencyCycleError;
pub use PluginHotReloadError;
pub use PluginBackupError;
pub use PluginMigrationError;
pub use PluginMonitoringError;
pub use UtilsError;
pub use LazyLoaderError;
pub use LazyLoadError;
pub use LazyUnloadError;
pub use LazyReloadError;
pub use PathUtilsError;
pub use PathNormalizationError;
pub use PathResolutionError;
pub use UtilsPathValidationError;
pub use PathSanitizationError;
pub use UtilityRegistryError;
pub use UtilityNotFoundError;
pub use UtilityRegistrationError;
pub use UtilityUnregistrationError;
pub use ConfigManagerError;
pub use ConfigLoadError;
pub use ConfigSaveError;
pub use ConfigValidationError;
pub use ResourceManagerError;
pub use ResourceAcquisitionError;
pub use ResourceReleaseError;
pub use ResourceNotFoundError;
pub use ResourceExhaustionError;
pub use StructureError;
pub use TreeError;
pub use TreeNodeError;
pub use TreeTraversalError;
pub use TreeValidationError;
pub use GraphError;
pub use GraphNodeError;
pub use GraphEdgeError;
pub use GraphTraversalError;
pub use GraphValidationError;
pub use CircularReferenceError;
pub use CircularDetectionError;
pub use CircularBreakError;
pub use StructureValidationError;
pub use StructureTypeError;
pub use StructureSizeError;
pub use StructureOperationError;
pub use StructureIndexError;
pub use StructureKeyError;
pub use OperationError;
pub use MergeError;
pub use DiffError;
pub use PatchError;
pub use QueryProviderError;
pub use QueryProviderNotRegisteredError;
pub use DateTimeError;
pub use DateTimeParseError;
pub use DateTimeFormatError;
pub use DateTimeValidationError;
pub use TimezoneError;
pub use TimezoneNotFoundError;
pub use TimezoneConversionError;
pub use HumanizeError;
pub use DateRangeError;
pub use TimeCalculationError;
pub use DateTimeOverflowError;
pub use DateTimeUnderflowError;
pub use MonitoringError;
pub use MetricsError;
pub use PerformanceError;
pub use TracingError;
pub use SystemMonitorError;
pub use MemoryMonitorError;
pub use ErrorRecoveryError;
pub use CacheError;
pub use CacheKeyError;
pub use CacheValueError;
pub use CacheTimeoutError;
pub use CacheSizeError;
pub use CacheConfigurationError;
pub use CacheOperationError;
pub use CacheSerializationError;
pub use CacheDeserializationError;
pub use CacheIntegrityError;
pub use CacheEvictionError;
pub use CacheWarmingError;
pub use CacheStatsError;
pub use HttpClientError;
pub use HttpRequestError;
pub use HttpResponseError;
pub use HttpTimeoutError;
pub use HttpConnectionError;
pub use HttpAuthenticationError;
pub use HttpAuthorizationError;
pub use HttpRetryError;
pub use HttpCircuitBreakerError;
pub use IPCError;
pub use MessageQueueError;
pub use SharedMemoryError;
pub use ProcessManagerError;
pub use ProcessPoolError;
pub use PipeError;
pub use AsyncFabricError;
pub use CLIError;
pub use ConsoleError;
pub use ProgressError;
pub use PromptError;
pub use TableError;
pub use ArgsError;
pub use ColorError;
pub use PatternError;
pub use RegistryError;
pub use HandlerError;
pub use FactoryError;
pub use StrategyError;
pub use ObserverError;
pub use CommandError;
pub use StateError;
pub use BuilderError;
pub use PrototypeError;
pub use AdapterError;
pub use DecoratorError;
pub use ProxyError;
pub use FacadeError;
pub use ChainHandlerError;
pub use MediatorError;
pub use MementoError;
pub use VisitorError;
pub use IteratorError;
pub use PatternConcurrencyError;
pub use ArchitecturalError;
pub use SpecificationError;
pub use ValueObjectError;
pub use AggregateError;
