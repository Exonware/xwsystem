// #exonware/xwsystem/rust/src_old/defs.rs
//! Type definitions, enums, and constants for XWSystem.
//!
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//!
//! This module provides all type definitions, enums, and constants used across XWSystem.

use serde::{Deserialize, Serialize};

// ============================================================================
// SHARED ENUMS
// ============================================================================

/// Standard validation levels used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValidationLevel {
    None,
    Basic,
    Strict,
    Paranoid,
    Comprehensive, // For structures module
}

impl Default for ValidationLevel {
    fn default() -> Self {
        ValidationLevel::Basic
    }
}

impl std::fmt::Display for ValidationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationLevel::None => write!(f, "none"),
            ValidationLevel::Basic => write!(f, "basic"),
            ValidationLevel::Strict => write!(f, "strict"),
            ValidationLevel::Paranoid => write!(f, "paranoid"),
            ValidationLevel::Comprehensive => write!(f, "comprehensive"),
        }
    }
}

/// Standard performance levels used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceLevel {
    // Performance module levels
    Low,
    Medium,
    High,
    Critical,
    
    // Monitoring module levels
    Excellent,
    Good,
    Average,
    Poor,
}

impl Default for PerformanceLevel {
    fn default() -> Self {
        PerformanceLevel::Medium
    }
}

impl std::fmt::Display for PerformanceLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceLevel::Low => write!(f, "low"),
            PerformanceLevel::Medium => write!(f, "medium"),
            PerformanceLevel::High => write!(f, "high"),
            PerformanceLevel::Critical => write!(f, "critical"),
            PerformanceLevel::Excellent => write!(f, "excellent"),
            PerformanceLevel::Good => write!(f, "good"),
            PerformanceLevel::Average => write!(f, "average"),
            PerformanceLevel::Poor => write!(f, "poor"),
        }
    }
}

/// Standard authentication types used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    None,
    Basic,
    Bearer,
    ApiKey,
    Oauth2,
    Jwt,
    Saml,
}

impl Default for AuthType {
    fn default() -> Self {
        AuthType::None
    }
}

impl std::fmt::Display for AuthType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthType::None => write!(f, "none"),
            AuthType::Basic => write!(f, "basic"),
            AuthType::Bearer => write!(f, "bearer"),
            AuthType::ApiKey => write!(f, "api_key"),
            AuthType::Oauth2 => write!(f, "oauth2"),
            AuthType::Jwt => write!(f, "jwt"),
            AuthType::Saml => write!(f, "saml"),
        }
    }
}

/// Standard lock types used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LockType {
    // IO module lock types
    Shared,
    Exclusive,
    NonBlocking,
    
    // Threading module lock types
    Mutex,
    RwLock,
    Semaphore,
    Condition,
    Event,
    Barrier,
}

impl Default for LockType {
    fn default() -> Self {
        LockType::Mutex
    }
}

impl std::fmt::Display for LockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockType::Shared => write!(f, "shared"),
            LockType::Exclusive => write!(f, "exclusive"),
            LockType::NonBlocking => write!(f, "non_blocking"),
            LockType::Mutex => write!(f, "mutex"),
            LockType::RwLock => write!(f, "rwlock"),
            LockType::Semaphore => write!(f, "semaphore"),
            LockType::Condition => write!(f, "condition"),
            LockType::Event => write!(f, "event"),
            LockType::Barrier => write!(f, "barrier"),
        }
    }
}

/// Standard path types used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PathType {
    // IO module path types
    File,
    Directory,
    Symlink,
    Mount,
    Socket,
    Pipe,
    Device,
    
    // Utils module path types
    Absolute,
    Relative,
    Resolved,
    Normalized,
}

impl Default for PathType {
    fn default() -> Self {
        PathType::File
    }
}

impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathType::File => write!(f, "file"),
            PathType::Directory => write!(f, "directory"),
            PathType::Symlink => write!(f, "symlink"),
            PathType::Mount => write!(f, "mount"),
            PathType::Socket => write!(f, "socket"),
            PathType::Pipe => write!(f, "pipe"),
            PathType::Device => write!(f, "device"),
            PathType::Absolute => write!(f, "absolute"),
            PathType::Relative => write!(f, "relative"),
            PathType::Resolved => write!(f, "resolved"),
            PathType::Normalized => write!(f, "normalized"),
        }
    }
}

/// Standard logging levels used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Standard operation result status used across XWSystem modules.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationResult {
    Success,
    Failed,
    Partial,
    Skipped,
    Timeout,
    Error,
    Warning,
}

impl Default for OperationResult {
    fn default() -> Self {
        OperationResult::Success
    }
}

impl std::fmt::Display for OperationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationResult::Success => write!(f, "success"),
            OperationResult::Failed => write!(f, "failed"),
            OperationResult::Partial => write!(f, "partial"),
            OperationResult::Skipped => write!(f, "skipped"),
            OperationResult::Timeout => write!(f, "timeout"),
            OperationResult::Error => write!(f, "error"),
            OperationResult::Warning => write!(f, "warning"),
        }
    }
}

// ============================================================================
// ROOT-LEVEL ENUMS (matching Python defs.py)
// ============================================================================

/// System status enumeration.
///
/// Matches Python's SystemStatus enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SystemStatus {
    Unknown,
    Initializing,
    Ready,
    Running,
    Paused,
    Stopping,
    Stopped,
    Error,
    Maintenance,
}

impl Default for SystemStatus {
    fn default() -> Self {
        SystemStatus::Unknown
    }
}

impl std::fmt::Display for SystemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemStatus::Unknown => write!(f, "unknown"),
            SystemStatus::Initializing => write!(f, "initializing"),
            SystemStatus::Ready => write!(f, "ready"),
            SystemStatus::Running => write!(f, "running"),
            SystemStatus::Paused => write!(f, "paused"),
            SystemStatus::Stopping => write!(f, "stopping"),
            SystemStatus::Stopped => write!(f, "stopped"),
            SystemStatus::Error => write!(f, "error"),
            SystemStatus::Maintenance => write!(f, "maintenance"),
        }
    }
}

/// Component type enumeration.
///
/// Matches Python's ComponentType enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ComponentType {
    Core,
    Io,
    Security,
    Monitoring,
    Validation,
    Caching,
    Plugin,
    HttpClient,
    Operations,
    Threading,
    Runtime,
    Structures,
    Utils,
    Ipc,
    Cli,
    Patterns,
    Query,
    Config,
    Shared,
}

impl Default for ComponentType {
    fn default() -> Self {
        ComponentType::Core
    }
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentType::Core => write!(f, "core"),
            ComponentType::Io => write!(f, "io"),
            ComponentType::Security => write!(f, "security"),
            ComponentType::Monitoring => write!(f, "monitoring"),
            ComponentType::Validation => write!(f, "validation"),
            ComponentType::Caching => write!(f, "caching"),
            ComponentType::Plugin => write!(f, "plugin"),
            ComponentType::HttpClient => write!(f, "http_client"),
            ComponentType::Operations => write!(f, "operations"),
            ComponentType::Threading => write!(f, "threading"),
            ComponentType::Runtime => write!(f, "runtime"),
            ComponentType::Structures => write!(f, "structures"),
            ComponentType::Utils => write!(f, "utils"),
            ComponentType::Ipc => write!(f, "ipc"),
            ComponentType::Cli => write!(f, "cli"),
            ComponentType::Patterns => write!(f, "patterns"),
            ComponentType::Query => write!(f, "query"),
            ComponentType::Config => write!(f, "config"),
            ComponentType::Shared => write!(f, "shared"),
        }
    }
}

/// Error severity levels.
///
/// Matches Python's ErrorSeverity enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ErrorSeverity {
    fn default() -> Self {
        ErrorSeverity::Medium
    }
}

impl std::fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSeverity::Low => write!(f, "low"),
            ErrorSeverity::Medium => write!(f, "medium"),
            ErrorSeverity::High => write!(f, "high"),
            ErrorSeverity::Critical => write!(f, "critical"),
        }
    }
}

/// Logging categories.
///
/// Matches Python's LogCategory enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogCategory {
    System,
    Security,
    Performance,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Default for LogCategory {
    fn default() -> Self {
        LogCategory::Info
    }
}

impl std::fmt::Display for LogCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogCategory::System => write!(f, "system"),
            LogCategory::Security => write!(f, "security"),
            LogCategory::Performance => write!(f, "performance"),
            LogCategory::Error => write!(f, "error"),
            LogCategory::Warning => write!(f, "warning"),
            LogCategory::Info => write!(f, "info"),
            LogCategory::Debug => write!(f, "debug"),
            LogCategory::Trace => write!(f, "trace"),
        }
    }
}

// ============================================================================
// CORE ENUMS
// ============================================================================

/// Core data types supported by XWSystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
    List,
    Dict,
    Bytes,
    None,
    Custom,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::String
    }
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String => write!(f, "string"),
            DataType::Integer => write!(f, "integer"),
            DataType::Float => write!(f, "float"),
            DataType::Boolean => write!(f, "boolean"),
            DataType::List => write!(f, "list"),
            DataType::Dict => write!(f, "dict"),
            DataType::Bytes => write!(f, "bytes"),
            DataType::None => write!(f, "none"),
            DataType::Custom => write!(f, "custom"),
        }
    }
}

/// Cloning modes for object duplication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CloneMode {
    Shallow,
    Deep,
    Reference,
}

impl Default for CloneMode {
    fn default() -> Self {
        CloneMode::Deep
    }
}

impl std::fmt::Display for CloneMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CloneMode::Shallow => write!(f, "shallow"),
            CloneMode::Deep => write!(f, "deep"),
            CloneMode::Reference => write!(f, "reference"),
        }
    }
}

/// Core system states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoreState {
    Uninitialized,
    Initializing,
    Initialized,
    Starting,
    Running,
    Stopping,
    Stopped,
    ShuttingDown,
    Shutdown,
    Error,
}

impl Default for CoreState {
    fn default() -> Self {
        CoreState::Uninitialized
    }
}

impl std::fmt::Display for CoreState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreState::Uninitialized => write!(f, "uninitialized"),
            CoreState::Initializing => write!(f, "initializing"),
            CoreState::Initialized => write!(f, "initialized"),
            CoreState::Starting => write!(f, "starting"),
            CoreState::Running => write!(f, "running"),
            CoreState::Stopping => write!(f, "stopping"),
            CoreState::Stopped => write!(f, "stopped"),
            CoreState::ShuttingDown => write!(f, "shutting_down"),
            CoreState::Shutdown => write!(f, "shutdown"),
            CoreState::Error => write!(f, "error"),
        }
    }
}

/// Core system modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CoreMode {
    Development,
    Testing,
    Staging,
    Production,
    Debug,
    Performance,
}

impl Default for CoreMode {
    fn default() -> Self {
        CoreMode::Development
    }
}

impl std::fmt::Display for CoreMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreMode::Development => write!(f, "development"),
            CoreMode::Testing => write!(f, "testing"),
            CoreMode::Staging => write!(f, "staging"),
            CoreMode::Production => write!(f, "production"),
            CoreMode::Debug => write!(f, "debug"),
            CoreMode::Performance => write!(f, "performance"),
        }
    }
}

/// Core system priorities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CorePriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for CorePriority {
    fn default() -> Self {
        CorePriority::Normal
    }
}

impl std::fmt::Display for CorePriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CorePriority::Low => write!(f, "low"),
            CorePriority::Normal => write!(f, "normal"),
            CorePriority::High => write!(f, "high"),
            CorePriority::Critical => write!(f, "critical"),
        }
    }
}

// ============================================================================
// ROOT-LEVEL CONSTANTS (matching Python defs.py)
// ============================================================================

/// Version information constants.
pub const VERSION_MAJOR: u32 = 0;
pub const VERSION_MINOR: u32 = 1;
pub const VERSION_PATCH: u32 = 0;
pub const VERSION_BUILD: u32 = 1;
pub const VERSION_STRING: &str = "0.1.0.1";

/// System constants.
pub const DEFAULT_TIMEOUT: u64 = 30; // seconds
pub const DEFAULT_RETRY_COUNT: u32 = 3;
pub const DEFAULT_RETRY_DELAY: f64 = 1.0; // seconds
pub const MAX_RETRY_DELAY: f64 = 60.0; // seconds

/// Configuration constants.
pub const DEFAULT_CONFIG_FILE: &str = "xwsystem.json";
pub const DEFAULT_CONFIG_DIR: &str = ".xwsystem";
pub const DEFAULT_LOG_DIR: &str = "logs";
pub const DEFAULT_CACHE_DIR: &str = ".cache";

/// Security constants.
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;
pub const DEFAULT_HASH_ALGORITHM: &str = "sha256";
pub const DEFAULT_ENCRYPTION_ALGORITHM: &str = "AES-256-GCM";

/// Performance constants.
pub const DEFAULT_CACHE_SIZE: usize = 1000;
pub const DEFAULT_CACHE_TTL: u64 = 3600; // seconds
pub const DEFAULT_THREAD_POOL_SIZE: usize = 10;
pub const DEFAULT_MAX_WORKERS: usize = 4;

/// Validation constants.
pub const MAX_DEPTH_DEFAULT: usize = 100;
pub const MAX_SIZE_DEFAULT: usize = 10 * 1024 * 1024; // 10 MB
pub const MAX_ITEMS_DEFAULT: usize = 10000;

/// Monitoring constants.
pub const DEFAULT_METRICS_INTERVAL: u64 = 60; // seconds
pub const DEFAULT_TRACE_SAMPLE_RATE: f64 = 0.1; // 10%
/// Default log level (matches Python's DEFAULT_LOG_LEVEL = LogLevel.INFO).
pub const DEFAULT_LOG_LEVEL: LogLevel = LogLevel::Info;

/// Plugin constants.
pub const DEFAULT_PLUGIN_DIR: &str = "plugins";
pub const DEFAULT_PLUGIN_TIMEOUT: u64 = 30; // seconds
pub const MAX_PLUGIN_SIZE: usize = 10 * 1024 * 1024; // 10 MB

/// HTTP client constants.
pub const DEFAULT_HTTP_TIMEOUT: u64 = 30; // seconds
pub const DEFAULT_HTTP_RETRIES: u32 = 3;
pub const DEFAULT_HTTP_RETRY_DELAY: f64 = 1.0; // seconds
pub const DEFAULT_MAX_REDIRECTS: u32 = 5;

/// IPC constants.
pub const DEFAULT_IPC_TIMEOUT: u64 = 30; // seconds
pub const DEFAULT_MESSAGE_QUEUE_SIZE: usize = 1000;
pub const DEFAULT_SHARED_MEMORY_SIZE: usize = 10 * 1024 * 1024; // 10 MB

/// CLI constants.
pub const DEFAULT_PROMPT: &str = "xwsystem> ";
pub const DEFAULT_PROGRESS_WIDTH: usize = 50;
pub const DEFAULT_TABLE_WIDTH: usize = 80;

// ============================================================================
// CONFIG ENUMS (matching Python config/defs.py)
// ============================================================================

/// Configuration sources.
///
/// Matches Python's ConfigSource enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigSource {
    File,
    Environment,
    Defaults,
    CommandLine,
    Database,
    Api,
    Memory,
}

impl Default for ConfigSource {
    fn default() -> Self {
        ConfigSource::Defaults
    }
}

impl std::fmt::Display for ConfigSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigSource::File => write!(f, "file"),
            ConfigSource::Environment => write!(f, "environment"),
            ConfigSource::Defaults => write!(f, "defaults"),
            ConfigSource::CommandLine => write!(f, "command_line"),
            ConfigSource::Database => write!(f, "database"),
            ConfigSource::Api => write!(f, "api"),
            ConfigSource::Memory => write!(f, "memory"),
        }
    }
}

/// Configuration formats.
///
/// Matches Python's ConfigFormat enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Ini,
    Env,
    Xml,
    Python,
}

impl Default for ConfigFormat {
    fn default() -> Self {
        ConfigFormat::Json
    }
}

impl std::fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFormat::Json => write!(f, "json"),
            ConfigFormat::Yaml => write!(f, "yaml"),
            ConfigFormat::Toml => write!(f, "toml"),
            ConfigFormat::Ini => write!(f, "ini"),
            ConfigFormat::Env => write!(f, "env"),
            ConfigFormat::Xml => write!(f, "xml"),
            ConfigFormat::Python => write!(f, "python"),
        }
    }
}

/// Configuration priority levels.
///
/// Matches Python's ConfigPriority enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfigPriority {
    Lowest = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Highest = 4,
    Critical = 5,
}

impl Default for ConfigPriority {
    fn default() -> Self {
        ConfigPriority::Normal
    }
}

impl std::fmt::Display for ConfigPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigPriority::Lowest => write!(f, "lowest"),
            ConfigPriority::Low => write!(f, "low"),
            ConfigPriority::Normal => write!(f, "normal"),
            ConfigPriority::High => write!(f, "high"),
            ConfigPriority::Highest => write!(f, "highest"),
            ConfigPriority::Critical => write!(f, "critical"),
        }
    }
}

/// Configuration types.
///
/// Matches Python's ConfigType enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigType {
    Dict,
    File,
    Environment,
    Database,
    Api,
}

impl Default for ConfigType {
    fn default() -> Self {
        ConfigType::Dict
    }
}

impl std::fmt::Display for ConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigType::Dict => write!(f, "dict"),
            ConfigType::File => write!(f, "file"),
            ConfigType::Environment => write!(f, "environment"),
            ConfigType::Database => write!(f, "database"),
            ConfigType::Api => write!(f, "api"),
        }
    }
}

/// Performance modes.
///
/// Matches Python's PerformanceMode enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceMode {
    Fast,
    Balanced,
    MemoryOptimized,
}

impl Default for PerformanceMode {
    fn default() -> Self {
        PerformanceMode::Balanced
    }
}

impl std::fmt::Display for PerformanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerformanceMode::Fast => write!(f, "fast"),
            PerformanceMode::Balanced => write!(f, "balanced"),
            PerformanceMode::MemoryOptimized => write!(f, "memory_optimized"),
        }
    }
}

/// Advanced performance optimization modes for system operations.
///
/// Matches Python's AdvancedPerformanceMode enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvancedPerformanceMode {
    Global,
    Auto,
    Parent,
    Default,
    Fast,
    Optimized,
    Manual,
    Adaptive,
    DualAdaptive,
}

impl Default for AdvancedPerformanceMode {
    fn default() -> Self {
        AdvancedPerformanceMode::Default
    }
}

impl std::fmt::Display for AdvancedPerformanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancedPerformanceMode::Global => write!(f, "global"),
            AdvancedPerformanceMode::Auto => write!(f, "auto"),
            AdvancedPerformanceMode::Parent => write!(f, "parent"),
            AdvancedPerformanceMode::Default => write!(f, "default"),
            AdvancedPerformanceMode::Fast => write!(f, "fast"),
            AdvancedPerformanceMode::Optimized => write!(f, "optimized"),
            AdvancedPerformanceMode::Manual => write!(f, "manual"),
            AdvancedPerformanceMode::Adaptive => write!(f, "adaptive"),
            AdvancedPerformanceMode::DualAdaptive => write!(f, "dual_adaptive"),
        }
    }
}

// ============================================================================
// IO ENUMS (matching Python io/defs.py)
// ============================================================================

use bitflags::bitflags;

/// Codec capabilities for introspection.
///
/// Matches Python's CodecCapability Flag enum.
/// Uses bitflags for combining capabilities with | operator.
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CodecCapability: u32 {
        /// Streaming support
        const STREAMING = 1 << 0;
        /// Bidirectional encode/decode
        const BIDIRECTIONAL = 1 << 1;
        /// Schema-based serialization
        const SCHEMA_BASED = 1 << 2;
        /// Binary output format
        const BINARY_OUTPUT = 1 << 3;
        /// Text output format
        const TEXT_OUTPUT = 1 << 4;
        /// Compression support
        const COMPRESSION = 1 << 5;
    }
}

// Manual serde implementation for bitflags
impl Serialize for CodecCapability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.bits().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CodecCapability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u32::deserialize(deserializer)?;
        CodecCapability::from_bits(bits)
            .ok_or_else(|| serde::de::Error::custom(format!("Invalid CodecCapability bits: {}", bits)))
    }
}

impl Default for CodecCapability {
    fn default() -> Self {
        CodecCapability::BIDIRECTIONAL
    }
}

// ============================================================================
// TYPE ALIASES (matching Python defs.py)
// ============================================================================

use std::collections::HashMap;

/// Common type aliases matching Python defs.py.
/// 
/// In Rust, we use type aliases to match Python's type system.
/// Note: Rust's type system is more strict, so some aliases are for
/// documentation and API compatibility purposes.

/// Configuration dictionary type (matches Python's ConfigDict = dict[str, Any]).
pub type ConfigDict = HashMap<String, serde_json::Value>;

/// Configuration list type (matches Python's ConfigList = list[Any]).
pub type ConfigList = Vec<serde_json::Value>;

/// Error dictionary type (matches Python's ErrorDict = dict[str, Any]).
pub type ErrorDict = HashMap<String, serde_json::Value>;

/// Metrics dictionary type (matches Python's MetricsDict = dict[str, Any]).
pub type MetricsDict = HashMap<String, serde_json::Value>;

/// Metadata dictionary type (matches Python's MetadataDict = dict[str, Any]).
pub type MetadataDict = HashMap<String, serde_json::Value>;

// ============================================================================
// CALLBACK TYPES (matching Python defs.py)
// ============================================================================

/// Configuration callback type (matches Python's ConfigCallback).
/// 
/// Python: ConfigCallback = Callable[[str, Any, Any], None]
/// Rust equivalent: Fn(&str, &serde_json::Value, &serde_json::Value) -> ()
/// 
/// Note: In Rust, we use trait objects or function pointers for callbacks.
/// For API compatibility, we define the signature here.
pub type ConfigCallback = Box<dyn Fn(&str, &serde_json::Value, &serde_json::Value)>;

/// Error callback type (matches Python's ErrorCallback).
/// 
/// Python: ErrorCallback = Callable[[Exception], None]
/// Rust equivalent: Fn(&dyn std::error::Error) -> ()
pub type ErrorCallback = Box<dyn Fn(&dyn std::error::Error)>;

/// Metrics callback type (matches Python's MetricsCallback).
/// 
/// Python: MetricsCallback = Callable[[MetricsDict], None]
/// Rust equivalent: Fn(&MetricsDict) -> ()
pub type MetricsCallback = Box<dyn Fn(&MetricsDict)>;

/// Lifecycle callback type (matches Python's LifecycleCallback).
/// 
/// Python: LifecycleCallback = Callable[[], None]
/// Rust equivalent: Fn() -> ()
pub type LifecycleCallback = Box<dyn Fn()>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_level_display() {
        assert_eq!(ValidationLevel::Basic.to_string(), "basic");
        assert_eq!(ValidationLevel::Strict.to_string(), "strict");
    }

    #[test]
    fn test_performance_level_display() {
        assert_eq!(PerformanceLevel::Medium.to_string(), "medium");
        assert_eq!(PerformanceLevel::High.to_string(), "high");
    }

    #[test]
    fn test_core_state_display() {
        assert_eq!(CoreState::Running.to_string(), "running");
        assert_eq!(CoreState::ShuttingDown.to_string(), "shutting_down");
    }

    #[test]
    fn test_system_status_display() {
        assert_eq!(SystemStatus::Running.to_string(), "running");
        assert_eq!(SystemStatus::Ready.to_string(), "ready");
    }

    #[test]
    fn test_component_type_display() {
        assert_eq!(ComponentType::Core.to_string(), "core");
        assert_eq!(ComponentType::Security.to_string(), "security");
    }

    #[test]
    fn test_error_severity_display() {
        assert_eq!(ErrorSeverity::High.to_string(), "high");
        assert_eq!(ErrorSeverity::Critical.to_string(), "critical");
    }

    #[test]
    fn test_log_category_display() {
        assert_eq!(LogCategory::System.to_string(), "system");
        assert_eq!(LogCategory::Security.to_string(), "security");
    }

    #[test]
    fn test_config_source_display() {
        assert_eq!(ConfigSource::File.to_string(), "file");
        assert_eq!(ConfigSource::Environment.to_string(), "environment");
    }

    #[test]
    fn test_config_format_display() {
        assert_eq!(ConfigFormat::Json.to_string(), "json");
        assert_eq!(ConfigFormat::Yaml.to_string(), "yaml");
    }

    #[test]
    fn test_enum_serialization() {
        let level = ValidationLevel::Strict;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"strict\"");
        
        let deserialized: ValidationLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ValidationLevel::Strict);
    }

    #[test]
    fn test_constants() {
        assert_eq!(VERSION_STRING, "0.1.0.1");
        assert_eq!(DEFAULT_TIMEOUT, 30);
        assert_eq!(DEFAULT_RETRY_COUNT, 3);
        assert_eq!(MIN_PASSWORD_LENGTH, 8);
        assert_eq!(MAX_PASSWORD_LENGTH, 128);
    }
}

