// #exonware/xwsystem/rust/src/config/mod.rs
//! Configuration module for xwsystem
//! 
//! This module provides configuration management for various xwsystem features.
//! 
//! Author: eXonware Backend Team
//! Company: eXonware.com
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generated: 2025-01-27

pub mod base;
pub mod contracts;
pub mod defaults;
pub mod defs;
pub mod errors;
pub mod logging;
pub mod logging_setup;
pub mod performance;
pub mod performance_modes;
pub mod version_manager;

pub use base::{AConfigBase, AConfigManagerBase, AConfigValidatorBase, ALoggingConfigBase, APerformanceConfigBase, BaseConfig};

pub use contracts::{IConfigManager, IConfigSecrets, IConfigSource, IConfigTemplate, IConfigValidator, IConfigWatcher, IConfigurable, IConfigurableSimple, IEnvironment, ISettings};

pub use defaults::{CIRCULAR_REFERENCE_PLACEHOLDER, DEFAULT_CONTENT_SNIPPET_LENGTH, DEFAULT_ENCODING, DEFAULT_LOCK_TIMEOUT, DEFAULT_MAX_CIRCULAR_DEPTH, DEFAULT_MAX_DICT_DEPTH, DEFAULT_MAX_EXTENSION_LENGTH, DEFAULT_MAX_FILE_SIZE_MB, DEFAULT_MAX_MEMORY_USAGE_MB, DEFAULT_MAX_PATH_DEPTH, DEFAULT_MAX_PATH_LENGTH, DEFAULT_MAX_RESOLUTION_DEPTH, DEFAULT_MAX_TO_DICT_SIZE_MB, DEFAULT_MAX_TRAVERSAL_DEPTH, DEFAULT_PATH_DELIMITER, DefaultConfig, JSON_POINTER_PREFIX, LOGGING_ENABLED, LOGGING_LEVEL, MAX_DEPTH_EXCEEDED_PLACEHOLDER, PATH_SEPARATOR_BACKWARD, PATH_SEPARATOR_FORWARD, URI_SCHEME_SEPARATOR};

pub use defs::{AdvancedPerformanceMode, ConfigFormat, ConfigPriority, ConfigSource, ConfigType, PerformanceMode};

pub use errors::{ConfigBackupError, ConfigError, ConfigKeyError, ConfigLockError, ConfigNotFoundError, ConfigParseError, ConfigPermissionError, ConfigRestoreError, ConfigTypeError, ConfigValidationError, ConfigValueError, LoggingConfigError, PerformanceConfigError};

pub use logging::{LoggingConfig, logging_disable, logging_enable, logging_set_level};

pub use logging_setup::{get_logger, setup_logging, LoggingSetup};

pub use performance::{NetworkLimits, PerformanceConfig, PerformanceLimits, SecurityLimits, SerializationLimits, configure_performance, get_network_limits, get_performance_config, get_security_limits, get_serialization_limits};

pub use performance_modes::{AdaptiveLearningEngine, AdaptiveProfile, DualAdaptiveProfile, DualPhaseAdaptiveEngine, PerformanceMetrics, PerformanceModeManager, PerformanceModes, PerformanceProfile, PerformanceProfiles};

pub use version_manager::{VersionManager, create_version_manager};
