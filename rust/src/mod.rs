// #exonware/xwsystem/rust/src/mod.rs
//exonware/xwsystem/__init__.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: October 10, 2025
//! 
//! XWSystem - Enterprise-grade Python framework with AI-powered performance optimization.
//! 
//! 🚀 QUICK START:
//! from xwsystem import JsonSerializer, HttpClient, SecureHash
//! 
//! # Serialize data
//! data = {"project": "awesome", "version": "1.0"}
//! json_str = JsonSerializer().dumps(data)
//! 
//! # Make HTTP requests
//! client = HttpClient()
//! response = client.get("https://api.example.com/data")
//! 
//! # Hash passwords
//! password_hash = SecureHash.sha256("user_password")
//! 
//! 📚 FEATURE OVERVIEW:
//! - 30+ serialization formats (JSON, YAML, MessagePack, Avro, Protobuf, Cap'n Proto, LevelDB, LMDB, Zarr, HDF5, Feather, GraphDB, etc.)
//! - Military-grade security with hazmat layer
//! - Advanced HTTP client with retry logic
//! - Performance monitoring and circuit breakers
//! - Thread-safe operations and async support
//! - Enterprise features (schema registry, tracing, auth)
//! 
//! 🎯 COMMON PATTERNS:
//! # Serialization (30+ formats: Text, Binary, Enterprise, Key-Value, Scientific)
//! from xwsystem import JsonSerializer, YamlSerializer, MsgPackSerializer, AvroSerializer, ProtobufSerializer, LmdbSerializer, ZarrSerializer, Hdf5Serializer, FeatherSerializer, GraphDbSerializer
//! 
//! # Security & Crypto
//! from xwsystem import SecureHash, SymmetricEncryption, PathValidator
//! 
//! # HTTP & Networking
//! from xwsystem import HttpClient, AsyncHttpClient, RetryConfig
//! 
//! # Performance & Monitoring
//! from xwsystem import PerformanceMonitor, CircuitBreaker, MemoryMonitor
//! 
//! # Threading & Async
//! from xwsystem import ThreadSafeFactory, AsyncLock, AsyncQueue
//! 
//! This module provides common utilities that can be used across different
//! components including threading, security, I/O, data structures, and design patterns.

pub mod base;
pub mod caching;
pub mod caching_ttl_cache;
pub mod cli_args;
pub mod cli_colors;
pub mod cli_progress;
pub mod cli_tables;
pub mod conf;
pub mod config;
pub mod config_logging_setup;
pub mod config_performance;
pub mod contracts;
pub mod defs;
pub mod errors;
pub mod http_client;
pub mod http_client_advanced_client;
pub mod io_common_atomic;
pub mod io_facade;
pub mod io_serialization;
pub mod io_stream_async_operations;
pub mod ipc;
pub mod monitoring;
pub mod monitoring_system_monitor;
pub mod patterns_context_manager;
pub mod patterns_handler_factory;
pub mod patterns_import_registry;
pub mod plugins;
pub mod runtime;
pub mod security;
pub mod security_crypto;
pub mod security_hazmat;
pub mod security_path_validator;
pub mod shared_contracts;
pub mod structures_circular_detector;
pub mod structures_tree_walker;
pub mod threading_async_primitives;
pub mod threading_locks;
pub mod threading_safe_factory;
pub mod utils_dt;
pub mod utils_dt_parsing;
pub mod validation;
pub mod validation_data_validator;
pub mod validation_declarative;
pub mod validation_type_safety;
pub mod version;

pub use caching::{AsyncLFUCache, AsyncLRUCache, CacheConfig, CacheManager, CacheStats, LFUCache, LRUCache};

pub use caching_ttl_cache::{AsyncTTLCache, TTLCache};

pub use cli_args::{Argument, ArgumentParser, ArgumentType, Command};

pub use cli_colors::{Colors, Style, colorize};

pub use cli_progress::{MultiProgress, ProgressBar, ProgressConfig, SpinnerProgress};

pub use cli_tables::{Alignment, BorderStyle, Column, Table, TableFormatter};

pub use config::{CIRCULAR_REFERENCE_PLACEHOLDER, DEFAULT_CONTENT_SNIPPET_LENGTH, DEFAULT_ENCODING, DEFAULT_LOCK_TIMEOUT, DEFAULT_MAX_CIRCULAR_DEPTH, DEFAULT_MAX_DICT_DEPTH, DEFAULT_MAX_EXTENSION_LENGTH, DEFAULT_MAX_FILE_SIZE_MB, DEFAULT_MAX_MEMORY_USAGE_MB, DEFAULT_MAX_PATH_DEPTH, DEFAULT_MAX_PATH_LENGTH, DEFAULT_MAX_RESOLUTION_DEPTH, DEFAULT_MAX_TO_DICT_SIZE_MB, DEFAULT_MAX_TRAVERSAL_DEPTH, DEFAULT_PATH_DELIMITER, JSON_POINTER_PREFIX, LOGGING_ENABLED, LOGGING_LEVEL, MAX_DEPTH_EXCEEDED_PLACEHOLDER, NetworkLimits, PATH_SEPARATOR_BACKWARD, PATH_SEPARATOR_FORWARD, PerformanceConfig, PerformanceLimits, SecurityLimits, SerializationLimits, URI_SCHEME_SEPARATOR, get_logger, setup_logging};

pub use config_logging_setup::{get_logger, setup_logging};

pub use config_performance::{configure_performance, get_network_limits, get_performance_config, get_security_limits, get_serialization_limits};

pub use http_client::{AsyncHttpClient, HttpClient, HttpError, RetryConfig};

pub use http_client_advanced_client::{AdvancedHttpClient, AdvancedHttpConfig, Http2Config, MockResponse, MockTransport, StreamingConfig};

pub use io_common_atomic::{AtomicFileWriter, FileOperationError, safe_read_bytes, safe_read_text, safe_read_with_fallback, safe_write_bytes, safe_write_text};

pub use io_facade::{XWIO};

pub use io_serialization::{ASerialization, AutoSerializer, BsonSerializer, CborSerializer, ConfigParserSerializer, CsvSerializer, DbmSerializer, FormDataSerializer, ISerialization, Json5Serializer, JsonLinesSerializer, JsonSerializer, MarshalSerializer, MsgPackSerializer, MultipartSerializer, PickleSerializer, PlistSerializer, SerializationError, SerializationRegistry, SerializerPool, ShelveSerializer, Sqlite3Serializer, TomlSerializer, XWSerializer, XmlSerializer, YamlSerializer, clear_serializer_cache, create_serializer, detect_format, get_cache_info, get_flyweight_stats, get_serialization_registry, get_serializer};

pub use io_stream_async_operations::{AsyncAtomicFileWriter, async_atomic_write, async_safe_read_bytes, async_safe_read_text, async_safe_read_with_fallback, async_safe_write_bytes, async_safe_write_text};

pub use ipc::{AsyncMessageQueue, AsyncPipe, AsyncProcessPool, MessageQueue, Pipe, ProcessInfo, ProcessManager, ProcessPool, SharedData, SharedMemoryManager};

pub use monitoring::{CircuitBreaker, CircuitBreakerConfig, CircuitState, ErrorContext, ErrorRecoveryManager, GenericPerformanceManager, HealthStatus, JaegerTracer, MemoryLeakReport, MemoryMonitor, MemorySnapshot, OpenTelemetryTracer, PerformanceMetric, PerformanceMonitor, PerformanceRecommendation, PerformanceReport, PerformanceStats, PerformanceThreshold, PerformanceValidator, SpanContext, TraceContext, TracingError, TracingManager, calculate_performance_summary, circuit_breaker, create_performance_monitor, enhanced_error_context, force_memory_cleanup, format_performance_report, get_error_recovery_manager, get_memory_monitor, get_memory_stats, get_performance_statistics, get_performance_validator, graceful_degradation, handle_error, performance_context, performance_monitor, record_performance_metric, register_object_for_monitoring, retry_with_backoff, start_memory_monitoring, start_performance_validation, stop_memory_monitoring, stop_performance_validation, unregister_object_from_monitoring, validate_performance};

pub use monitoring_system_monitor::{DiskInfo, NetworkInfo, ProcessInfo, SystemInfo, SystemMonitor, get_cpu_usage, get_hardware_info, get_memory_usage, get_process, get_system_info, is_monitoring_available, list_processes};

pub use patterns_context_manager::{ContextualLogger, ThreadSafeSingleton, combine_contexts, create_operation_logger, enhanced_error_context};

pub use patterns_handler_factory::{GenericHandlerFactory};

pub use patterns_import_registry::{register_imports_batch, register_imports_flat, register_imports_tree};

pub use plugins::{PluginBase, PluginManager, PluginRegistry};

pub use runtime::{EnvironmentManager, ReflectionUtils};

pub use security::{AAuthProvider, ATokenInfo, AUserInfo, AuthenticationError, AuthorizationError, IAuthenticatable, IAuthorization, ISecurityToken, TokenExpiredError};

pub use security_crypto::{AsymmetricEncryption, AsyncAsymmetricEncryption, AsyncSecureStorage, AsyncSymmetricEncryption, CryptographicError, SecureHash, SecureRandom, SecureStorage, SymmetricEncryption, generate_api_key, generate_session_token, hash_password, hash_password_async, verify_password, verify_password_async};

pub use security_hazmat::{AES_GCM, ChaCha20Poly1305_Cipher, Ed25519_Signature, HKDF_Expand, PBKDF2_Derive, X25519_KeyExchange, X509Certificate, constant_time_compare, is_cryptography_available, secure_hash, secure_random};

pub use security_path_validator::{PathSecurityError, PathValidator};

pub use shared_contracts::{IStringable};

pub use structures_circular_detector::{CircularReferenceDetector, CircularReferenceError};

pub use structures_tree_walker::{TreeWalker, apply_user_defined_links, resolve_proxies_in_dict, walk_and_replace};

pub use threading_async_primitives::{AsyncCondition, AsyncEvent, AsyncLock, AsyncQueue, AsyncResourcePool, AsyncSemaphore};

pub use threading_locks::{EnhancedRLock};

pub use threading_safe_factory::{MethodGenerator, ThreadSafeFactory};

pub use utils_dt::{TimezoneManager, convert_timezone, duration_to_human, format_datetime, get_timezone_info, humanize_timedelta, humanize_timestamp, list_timezones, parse_human_duration, time_ago, time_until};

pub use utils_dt_parsing::{parse_date, parse_datetime, parse_iso8601, parse_time, parse_timestamp};

pub use validation::{Field, ValidationError, XModel};

pub use validation_data_validator::{DataValidator, DataValidator, check_data_depth, estimate_memory_usage, validate_path_input};

pub use validation_declarative::{Field, ValidationError, XModel};

pub use validation_type_safety::{GenericSecurityError, SafeTypeValidator, validate_untrusted_data};

pub use version::{__version__, get_version, get_version_info, get_version_dict, is_dev_version, is_release_version, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_BUILD, VERSION_SUFFIX, VERSION_STRING};

// Re-export root-level modules
pub use base::{AXWSystemBase, ASystemComponent, AConfigurableComponent, AMonitoredComponent, ASecureComponent};
pub use config::{XWSystemConfig, create_config, load_config_from_file, save_config_to_file};
pub use contracts::{IXWSystem, ISystemComponent, IExtensible, IConfigurableComponent, IMonitoredComponent, ISecureComponent};
pub use defs::{SystemStatus, ComponentType, ErrorSeverity, LogCategory, ConfigDict, ConfigList, ErrorDict, MetricsDict, MetadataDict};
pub use errors::{XWSystemError, XWSystemInitializationError, XWSystemConfigurationError, XWSystemStateError, XWSystemDependencyError, XWSystemResourceError, XWSystemTimeoutError, XWSystemPermissionError, XWSystemValidationError, XWSystemOperationError};
