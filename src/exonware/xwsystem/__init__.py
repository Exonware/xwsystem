#exonware/xwsystem/src/exonware/xwsystem/__init__.py
#exonware/xwsystem/__init__.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.5
Generation Date: October 10, 2025

XWSystem - Enterprise-grade Python framework with AI-powered performance optimization.

🚀 QUICK START:
    from xwsystem import JsonSerializer, HttpClient, SecureHash
    
    # Serialize data
    data = {"project": "awesome", "version": "1.0"}
    json_str = JsonSerializer().dumps(data)
    
    # Make HTTP requests
    client = HttpClient()
    response = client.get("https://api.example.com/data")
    
    # Hash passwords
    password_hash = SecureHash.sha256("user_password")

📚 FEATURE OVERVIEW:
    - 30+ serialization formats (JSON, YAML, MessagePack, Avro, Protobuf, Cap'n Proto, LevelDB, LMDB, Zarr, HDF5, Feather, GraphDB, etc.)
    - Military-grade security with hazmat layer
    - Advanced HTTP client with retry logic  
    - Performance monitoring and circuit breakers
    - Thread-safe operations and async support
    - Enterprise features (schema registry, tracing, auth)

🎯 COMMON PATTERNS:
    # Serialization (30+ formats: Text, Binary, Enterprise, Key-Value, Scientific)
    from xwsystem import JsonSerializer, YamlSerializer, MsgPackSerializer, AvroSerializer, ProtobufSerializer, LmdbSerializer, ZarrSerializer, Hdf5Serializer, FeatherSerializer, GraphDbSerializer
    
    # Security & Crypto
    from xwsystem import SecureHash, SymmetricEncryption, PathValidator
    
    # HTTP & Networking
    from xwsystem import HttpClient, AsyncHttpClient, RetryConfig
    
    # Performance & Monitoring
    from xwsystem import PerformanceMonitor, CircuitBreaker, MemoryMonitor
    
    # Threading & Async
    from xwsystem import ThreadSafeFactory, AsyncLock, AsyncQueue

This module provides common utilities that can be used across different
components including threading, security, I/O, data structures, and design patterns.
"""

import logging
from typing import TYPE_CHECKING

# Performance optimization: lazy import expensive modules
if TYPE_CHECKING:
    from typing import Any

# =============================================================================
# XWLAZY INTEGRATION - Auto-install missing dependencies silently (EARLY)
# =============================================================================
# Activate xwlazy BEFORE other imports to enable auto-installation of missing dependencies
# This enables silent auto-installation of missing libraries when they are imported
try:
    from exonware.xwlazy import auto_enable_lazy
    auto_enable_lazy(__package__ or "exonware.xwsystem", mode="smart")
except ImportError:
    # xwlazy not installed - lazy mode simply stays disabled (normal behavior)
    pass

# Logging utilities
from .config.logging_setup import get_logger, setup_logging

# Serialization utilities (19 core formats + intelligent auto-detection)
# Enterprise formats (Protobuf, Avro, Parquet, HDF5, etc.) available in exonware-xwformats
from .io.serialization import (
    ISerialization,
    ASerialization,
    SerializationError,
    # Text formats (10)
    JsonSerializer,
    Json5Serializer,
    JsonLinesSerializer,
    YamlSerializer,
    TomlSerializer,
    XmlSerializer,
    CsvSerializer,
    ConfigParserSerializer,
    FormDataSerializer,
    MultipartSerializer,
    # Binary formats (6)
    MsgPackSerializer,
    PickleSerializer,
    BsonSerializer,
    MarshalSerializer,
    CborSerializer,
    PlistSerializer,
    # Database formats (3)
    Sqlite3Serializer,
    DbmSerializer,
    ShelveSerializer,
    # Registry
    SerializationRegistry, get_serialization_registry,
    # Flyweight optimization
    get_serializer, get_flyweight_stats, clear_serializer_cache,
    # Universal options
    map_universal_options, get_supported_universal_options, validate_universal_options,
    get_all_supported_formats, get_format_option_info, UniversalOption, 
    get_cache_info, create_serializer, SerializerPool,
    # Auto-detection utilities
    XWSerializer, AutoSerializer,
    detect_format,
)

# HTTP utilities
from .http_client import HttpClient, AsyncHttpClient, HttpError, RetryConfig
from .http_client.advanced_client import (
    AdvancedHttpClient, AdvancedHttpConfig, Http2Config, 
    StreamingConfig, MockTransport, MockResponse
)

# Runtime utilities
from .runtime import EnvironmentManager, ReflectionUtils

# Plugin system
from .plugins import PluginManager, PluginBase, PluginRegistry

# I/O utilities
from .io.facade import XWIO
from .io.common.atomic import (
    AtomicFileWriter,
    FileOperationError,
    safe_read_bytes,
    safe_read_text,
    safe_read_with_fallback,
    safe_write_bytes,
    safe_write_text,
)
from .io.stream.async_operations import (
    AsyncAtomicFileWriter, async_atomic_write,
    async_safe_write_text, async_safe_write_bytes,
    async_safe_read_text, async_safe_read_bytes, async_safe_read_with_fallback
)
from .patterns.context_manager import (
    ContextualLogger,
    ThreadSafeSingleton,
    combine_contexts,
    create_operation_logger,
    enhanced_error_context,
)

# Pattern utilities
from .patterns.handler_factory import GenericHandlerFactory
from .patterns.import_registry import (
    register_imports_batch,
    register_imports_flat,
    register_imports_tree,
)

# Security utilities
from .security.path_validator import PathSecurityError, PathValidator
from .security.crypto import (
    AsymmetricEncryption, AsyncAsymmetricEncryption,
    CryptographicError,
    SecureHash,
    SecureRandom,
    SecureStorage, AsyncSecureStorage,
    SymmetricEncryption, AsyncSymmetricEncryption,
    generate_api_key,
    generate_session_token,
    hash_password, hash_password_async,
    verify_password, verify_password_async,
)

# Data structure utilities
from .structures.circular_detector import (
    CircularReferenceDetector,
    CircularReferenceError,
)
from .structures.tree_walker import (
    TreeWalker,
    apply_user_defined_links,
    resolve_proxies_in_dict,
    walk_and_replace,
)
from .threading.locks import EnhancedRLock

# Threading utilities
from .threading.safe_factory import MethodGenerator, ThreadSafeFactory
from .threading.async_primitives import (
    AsyncLock, AsyncSemaphore, AsyncEvent, AsyncQueue, 
    AsyncCondition, AsyncResourcePool
)

# Performance management (now in monitoring module)
from .monitoring import GenericPerformanceManager, PerformanceRecommendation, HealthStatus

# Caching utilities
from .caching import (
    LRUCache, AsyncLRUCache, LFUCache, AsyncLFUCache, CacheManager, CacheConfig, CacheStats
)
from .caching.ttl_cache import TTLCache, AsyncTTLCache

# CLI utilities - import general console enums from console level (priority)
from .console.defs import Colors, Style, Alignment, BorderStyle
from .console.cli.colors import colorize, CliColoredOutput
from .console.cli.args import CliArgumentParser, CliArgument, CliCommand, ArgumentType
from .console.cli.progress import CliProgressBar, CliSpinnerProgress, CliMultiProgress, CliProgressConfig
from .console.cli.tables import CliTable, CliTableFormatter, CliColumn
from .console.cli.console import CliConsole

# Validation utilities
from .validation.declarative import XModel, Field, ValidationError
from .validation.data_validator import DataValidator

# Security hazmat layer
from .security.hazmat import (
    AES_GCM, ChaCha20Poly1305_Cipher, X25519_KeyExchange, Ed25519_Signature,
    HKDF_Expand, PBKDF2_Derive, X509Certificate, secure_hash,
    constant_time_compare, secure_random, is_cryptography_available
)

# System monitoring
from .monitoring.system_monitor import (
    SystemMonitor, ProcessInfo, SystemInfo, DiskInfo, NetworkInfo,
    list_processes, get_process, get_system_info, get_cpu_usage,
    get_memory_usage, get_hardware_info, is_monitoring_available
)

# DateTime utilities
from .utils.dt import (
    humanize_timedelta, humanize_timestamp, time_ago, time_until,
    duration_to_human, parse_human_duration, TimezoneManager,
    convert_timezone, get_timezone_info, list_timezones,
    format_datetime, get_datetime, get_date, get_date_from_to_month,
    calculate_duration_days, parse_timestamp_milliseconds
)
from .utils.dt.parsing import parse_datetime, parse_date, parse_time, parse_iso8601, parse_timestamp

# String utilities
from .utils.string import find_nth_occurrence

# Web utilities
from .utils.web import validate_url_accessible, extract_webpage_text

# IPC utilities
from .ipc import (
    ProcessManager, ProcessInfo, SharedMemoryManager, SharedData,
    MessageQueue, AsyncMessageQueue, ProcessPool, AsyncProcessPool,
    Pipe, AsyncPipe
)


# Simple logging control
logging_enabled = True


def disable_logging() -> None:
    """Disable all logging."""
    global logging_enabled
    logging_enabled = False
    logging.disable(logging.CRITICAL)


def enable_logging() -> None:
    """Enable logging."""
    global logging_enabled
    logging_enabled = True
    logging.disable(logging.NOTSET)


# Configuration utilities
from .config import (
    DEFAULT_ENCODING, DEFAULT_PATH_DELIMITER, DEFAULT_LOCK_TIMEOUT,
    DEFAULT_MAX_FILE_SIZE_MB, DEFAULT_MAX_MEMORY_USAGE_MB, DEFAULT_MAX_DICT_DEPTH,
    DEFAULT_MAX_PATH_DEPTH, DEFAULT_MAX_PATH_LENGTH, DEFAULT_MAX_RESOLUTION_DEPTH,
    DEFAULT_MAX_TO_DICT_SIZE_MB, DEFAULT_MAX_CIRCULAR_DEPTH,
    DEFAULT_MAX_EXTENSION_LENGTH, DEFAULT_CONTENT_SNIPPET_LENGTH,
    DEFAULT_MAX_TRAVERSAL_DEPTH, URI_SCHEME_SEPARATOR, JSON_POINTER_PREFIX,
    PATH_SEPARATOR_FORWARD, PATH_SEPARATOR_BACKWARD,
    CIRCULAR_REFERENCE_PLACEHOLDER, MAX_DEPTH_EXCEEDED_PLACEHOLDER,
    LOGGING_ENABLED, LOGGING_LEVEL, setup_logging, get_logger,
    ConsoleEventLogger, ConsoleEvent,
    PerformanceConfig
)
from .config.performance import (
    get_performance_config,
    set_performance_config,
)

# Monitoring utilities
from .monitoring import (  # Performance Monitor; Memory Monitoring; Error Recovery; Performance Validation
    CircuitBreaker,
    CircuitBreakerConfig,
    CircuitState,
    ErrorContext,
    ErrorRecoveryManager,
    MemoryLeakReport,
    MemoryMonitor,
    MemorySnapshot,
    PerformanceMetric,
    PerformanceMonitor,
    PerformanceReport,
    PerformanceStats,
    PerformanceThreshold,
    PerformanceValidator,
    calculate_performance_summary,
    circuit_breaker,
    create_performance_monitor,
    enhanced_error_context,
    force_memory_cleanup,
    format_performance_report,
    get_error_recovery_manager,
    get_memory_monitor,
    get_memory_stats,
    get_performance_statistics,
    get_performance_validator,
    graceful_degradation,
    handle_error,
    performance_context,
    performance_monitor,
    record_performance_metric,
    register_object_for_monitoring,
    retry_with_backoff,
    start_memory_monitoring,
    start_performance_validation,
    stop_memory_monitoring,
    stop_performance_validation,
    unregister_object_from_monitoring,
    validate_performance,
)

# Validation utilities
from .validation import XModel, Field, ValidationError
from .validation.data_validator import (
    DataValidator, validate_path_input, check_data_depth, estimate_memory_usage
)
from .validation.type_safety import (
    SafeTypeValidator, GenericSecurityError, validate_untrusted_data
)

# Enterprise utilities - distributed across security, monitoring, and io/serialization
# Note: Authentication provider implementations (OAuth2Provider, JWTProvider, SAMLProvider, EnterpriseAuth)
# have been moved to xwauth, which extends xwsystem. Use xwauth for actual authentication providers.
from .security import (
    AuthenticationError, AuthorizationError, TokenExpiredError,
    AAuthProvider, ATokenInfo, AUserInfo,
    IAuthenticatable, IAuthorization, ISecurityToken
)
from .monitoring import (
    TracingManager, OpenTelemetryTracer, JaegerTracer,
    TracingError, SpanContext, TraceContext
)
# NOTE: Schema Registry classes moved to exonware-xwschema
# Available in: pip install exonware-xwschema
# from .io.serialization import (
#     ASchemaRegistry, ConfluentSchemaRegistry, AwsGlueSchemaRegistry,
#     SchemaRegistryError, SchemaNotFoundError, SchemaValidationError
# )

# Core interfaces
from .shared.contracts import IStringable

# Note: Protocol definitions are now in their respective module contracts files:
# - core/contracts.py for core interfaces (IStringable, IID, INative, etc.)
# - serialization/contracts.py for serialization protocols
# - caching/contracts.py for caching protocols  
# - security/contracts.py for security protocols
# - validation/contracts.py for validation protocols

# Import version from centralized location
from .version import __version__

try:
    import sys as _sys
    _parent_pkg = _sys.modules.get("exonware")
    if _parent_pkg is not None:
        _parent_pkg.__version__ = __version__
except Exception:  # pragma: no cover
    pass

# =============================================================================
# CONVENIENCE FUNCTIONS - Quick access to common operations
# =============================================================================

def quick_serialize(data, format="json", **kwargs):
    """
    Quick serialization with support for all 24+ formats.
    
    Args:
        data: Data to serialize
        format: Format name - supports all XWSystem formats:
               Text: json, yaml, toml, xml, csv, ini/configparser, formdata, multipart
               Binary: bson, msgpack, cbor, pickle, marshal, dbm, shelve, plist
               Enterprise: avro, protobuf, thrift, parquet, orc, capnproto, flatbuffers
        **kwargs: Additional serialization options
        
    Returns:
        Serialized data as string or bytes
        
    Examples:
        >>> quick_serialize({"hello": "world"}, "json")
        '{"hello": "world"}'
        >>> quick_serialize({"hello": "world"}, "yaml") 
        'hello: world\\n'
        >>> quick_serialize({"hello": "world"}, "msgpack")  # Binary format
        b'\\x81\\xa5hello\\xa5world'
        >>> quick_serialize({"hello": "world"}, "avro")     # Enterprise format
        # Returns Avro binary data
    """
    from .io.serialization import create_serializer
    serializer = create_serializer(format)
    return serializer.dumps(data, **kwargs)

def quick_deserialize(data, format="auto", **kwargs):
    """
    Quick deserialization with support for all 24+ formats and auto-detection.
    
    Args:
        data: Data to deserialize (string, bytes, or file path)
        format: Format name or "auto" for intelligent auto-detection
               Supports: json, yaml, toml, xml, csv, ini, formdata, multipart,
                        bson, msgpack, cbor, pickle, marshal, dbm, shelve, plist,
                        avro, protobuf, thrift, parquet, orc, capnproto, flatbuffers
        **kwargs: Additional deserialization options
        
    Returns:
        Deserialized Python object
        
    Examples:
        >>> quick_deserialize('{"hello": "world"}', "json")
        {'hello': 'world'}
        >>> quick_deserialize('{"hello": "world"}')  # auto-detect JSON
        {'hello': 'world'}
        >>> quick_deserialize(binary_msgpack_data, "msgpack")
        {'hello': 'world'}
        >>> quick_deserialize(avro_bytes, "avro")  # Enterprise format
        {'hello': 'world'}
    """
    if format == "auto":
        from .io.serialization import XWSerializer
        return XWSerializer().loads(data, **kwargs)
    else:
        from .io.serialization import create_serializer
        serializer = create_serializer(format)
        return serializer.loads(data, **kwargs)

def quick_hash(data, algorithm="sha256"):
    """
    Quick hashing with common algorithms.
    
    Args:
        data: Data to hash (str or bytes)
        algorithm: Hash algorithm (sha256, sha512, blake2b)
        
    Returns:
        Hex digest string
        
    Example:
        >>> quick_hash("hello world")
        'b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9'
    """
    if algorithm == "sha256":
        return SecureHash.sha256(data)
    elif algorithm == "sha512":
        return SecureHash.sha512(data)
    elif algorithm == "blake2b":
        return SecureHash.blake2b(data)
    else:
        raise ValueError(f"Unsupported algorithm: {algorithm}")

def quick_encrypt(data, password=None):
    """
    Quick symmetric encryption with password.
    
    Args:
        data: Data to encrypt (str or bytes)
        password: Password for encryption (auto-generated if None)
        
    Returns:
        Tuple of (encrypted_data, key) if password is None, else encrypted_data
        
    Example:
        >>> encrypted, key = quick_encrypt("secret data")
        >>> decrypted = quick_decrypt(encrypted, key)
    """
    if password is None:
        encryption = SymmetricEncryption()
        encrypted = encryption.encrypt(data)
        return encrypted, encryption.key
    else:
        encryption = SymmetricEncryption.from_password(password)
        return encryption.encrypt(data)

def quick_decrypt(encrypted_data, key_or_password):
    """
    Quick symmetric decryption.
    
    Args:
        encrypted_data: Encrypted data
        key_or_password: Encryption key or password
        
    Returns:
        Decrypted data
    """
    if isinstance(key_or_password, str):
        encryption = SymmetricEncryption.from_password(key_or_password)
    else:
        encryption = SymmetricEncryption(key_or_password)
    return encryption.decrypt(encrypted_data)

def list_available_formats():
    """
    List all available serialization formats.
    
    Returns:
        dict with format categories and their available formats
        
    Example:
        >>> formats = list_available_formats()
        >>> print(f"Total formats: {len(formats['all'])}")
        >>> print(f"Enterprise formats: {formats['enterprise']}")
    """
    from .io.serialization.flyweight import create_serializer
    
    # Test which formats are available by trying to create serializers
    all_formats = [
        'json', 'jsonl', 'ndjson', 'yaml', 'toml', 'xml', 'csv', 'ini', 'formdata', 'multipart',  # Text (NDJSON/JSONL explicitly listed)
        'bson', 'msgpack', 'cbor', 'pickle', 'marshal', 'dbm', 'shelve', 'plist',  # Binary
        'avro', 'protobuf', 'thrift', 'parquet', 'orc', 'capnproto', 'flatbuffers',  # Enterprise
        'leveldb', 'lmdb', 'zarr',  # Key-value stores
        'hdf5', 'feather', 'graphdb'  # Scientific & analytics
    ]
    
    available = []
    missing = []
    
    for fmt in all_formats:
        try:
            # Try to create serializer to test if dependencies are available
            create_serializer(fmt)
            available.append(fmt)
        except (ImportError, ValueError):
            missing.append(fmt)
    
    return {
        'all': available,
        'missing': missing,
        'text': [f for f in available if f in ['json', 'jsonl', 'ndjson', 'yaml', 'toml', 'xml', 'csv', 'ini', 'formdata', 'multipart']],
        'binary': [f for f in available if f in ['bson', 'msgpack', 'cbor', 'pickle', 'marshal', 'dbm', 'shelve', 'plist']],
        'enterprise': [f for f in available if f in ['avro', 'protobuf', 'thrift', 'parquet', 'orc', 'capnproto', 'flatbuffers']],
        'keyvalue': [f for f in available if f in ['leveldb', 'lmdb', 'zarr']],
        'scientific': [f for f in available if f in ['hdf5', 'feather', 'graphdb']],
        'total_count': len(available)
    }

# =============================================================================
# MODULE METADATA AND EXPORTS
# =============================================================================

__all__ = [
        # Core interfaces
    "IStringable",
        # Serialization (17 core formats - enterprise formats in exonware-xwformats)
    "ISerialization",
    "ASerialization", 
    "SerializationError",
    
    # Text formats (7) - I→A pattern (CSV moved to tabular)
    "JsonSerializer",
    "Json5Serializer",
    "JsonLinesSerializer",
    "YamlSerializer",
    "TomlSerializer",
    "XmlSerializer",
    "ConfigParserSerializer",
    "FormDataSerializer",
    "MultipartSerializer",
    
    # Tabular formats (4) - I→A→ATabular pattern
    "ATabularSerialization",
    "ExcelSerializer",
    "CsvSerializer",  # Moved from text/ to tabular/
    "GoogleSheetsSerializer",
    "DataFrameSerializer",
    
    # Binary formats (6) - I→A pattern
    "MsgPackSerializer",
    "PickleSerializer",
    "BsonSerializer",
    "MarshalSerializer",
    "CborSerializer",
    "PlistSerializer",
    
    # Database formats (3) - I→A pattern
    "Sqlite3Serializer",
    "DbmSerializer",
    "ShelveSerializer",
    
    # Registry and utilities
    "SerializationRegistry", "get_serialization_registry",
    "XWSerializer", "AutoSerializer",
    "detect_format",
    "get_serializer", "get_flyweight_stats", "clear_serializer_cache", 
    "get_cache_info", "create_serializer", "SerializerPool",
    # Universal options
    "map_universal_options", "get_supported_universal_options", "validate_universal_options",
    "get_all_supported_formats", "get_format_option_info", "UniversalOption",
    # HTTP
    "HttpClient",
    "AsyncHttpClient",
    "HttpError",
    "RetryConfig",
    "AdvancedHttpClient",
    "AdvancedHttpConfig", 
    "Http2Config",
    "StreamingConfig",
    "MockTransport",
    "MockResponse",
    # Runtime
    "EnvironmentManager",
    "ReflectionUtils",
    # Plugins
    "PluginManager",
    "PluginBase",
    "PluginRegistry",
    # Threading
    "ThreadSafeFactory",
    "MethodGenerator",
    "EnhancedRLock",
    # Async Threading
    "AsyncLock",
    "AsyncSemaphore", 
    "AsyncEvent",
    "AsyncQueue",
    "AsyncCondition",
    "AsyncResourcePool",
    # Security
    "PathValidator",
    "PathSecurityError",
    "AsymmetricEncryption", "AsyncAsymmetricEncryption",
    "CryptographicError",
    "SecureHash",
    "SecureRandom",
    "SecureStorage", "AsyncSecureStorage",
    "SymmetricEncryption", "AsyncSymmetricEncryption",
    "generate_api_key",
    "generate_session_token",
    "hash_password", "hash_password_async",
    "verify_password", "verify_password_async",
    # I/O
    "XWIO",
    "AtomicFileWriter",
    "FileOperationError",
    "safe_write_text",
    "safe_write_bytes",
    "safe_read_text",
    "safe_read_bytes",
    "safe_read_with_fallback",
    # Async I/O
    "AsyncAtomicFileWriter",
    "async_atomic_write",
    "async_safe_write_text",
    "async_safe_write_bytes", 
    "async_safe_read_text",
    "async_safe_read_bytes",
    "async_safe_read_with_fallback",
    # Structures
    "CircularReferenceDetector",
    "CircularReferenceError",
    "TreeWalker",
    "resolve_proxies_in_dict",
    "apply_user_defined_links",
    "walk_and_replace",
    # Patterns
    "GenericHandlerFactory",
    "combine_contexts",
    "enhanced_error_context",
    "ContextualLogger",
    "create_operation_logger",
    "ThreadSafeSingleton",
    "register_imports_flat",
    "register_imports_tree",
    "register_imports_batch",
    # Performance Management (available via direct import)
    # 'GenericPerformanceManager',
    # 'PerformanceRecommendation',
    # 'HealthStatus',
    # Logging
    "setup_logging",
    "get_logger",
    "disable_logging",
    "enable_logging",
    "ConsoleEventLogger",
    "ConsoleEvent",
    # Performance Configuration
    "get_performance_config",
    # Configuration
    "DEFAULT_ENCODING",
    "DEFAULT_PATH_DELIMITER",
    "DEFAULT_LOCK_TIMEOUT",
    "DEFAULT_MAX_FILE_SIZE_MB",
    "DEFAULT_MAX_MEMORY_USAGE_MB",
    "DEFAULT_MAX_DICT_DEPTH",
    "DEFAULT_MAX_PATH_DEPTH",
    "DEFAULT_MAX_PATH_LENGTH",
    "DEFAULT_MAX_RESOLUTION_DEPTH",
    "DEFAULT_MAX_TO_DICT_SIZE_MB",
    "DEFAULT_MAX_CIRCULAR_DEPTH",
    "DEFAULT_MAX_EXTENSION_LENGTH",
    "DEFAULT_CONTENT_SNIPPET_LENGTH",
    "DEFAULT_MAX_TRAVERSAL_DEPTH",
    "URI_SCHEME_SEPARATOR",
    "JSON_POINTER_PREFIX",
    "PATH_SEPARATOR_FORWARD",
    "PATH_SEPARATOR_BACKWARD",
    "CIRCULAR_REFERENCE_PLACEHOLDER",
    "MAX_DEPTH_EXCEEDED_PLACEHOLDER",
    # Performance Configuration
    "PerformanceConfig",
    # Validation
    "DataValidator",
    "check_data_depth",
    "validate_path_input",
    "validate_resolution_depth",
    "estimate_memory_usage",
    "ValidationError",
    "PathValidationError",
    "DepthValidationError",
    "MemoryValidationError",
    # Monitoring
    "PerformanceMonitor",
    "PerformanceStats",
    "create_performance_monitor",
    "performance_context",
    "calculate_performance_summary",
    "format_performance_report",
    # Memory Monitoring
    "MemoryMonitor",
    "MemorySnapshot",
    "MemoryLeakReport",
    "get_memory_monitor",
    "start_memory_monitoring",
    "stop_memory_monitoring",
    "force_memory_cleanup",
    "get_memory_stats",
    "register_object_for_monitoring",
    "unregister_object_from_monitoring",
    # Error Recovery
    "CircuitBreaker",
    "CircuitBreakerConfig",
    "CircuitState",
    "ErrorRecoveryManager",
    "ErrorContext",
    "get_error_recovery_manager",
    "circuit_breaker",
    "retry_with_backoff",
    "graceful_degradation",
    "handle_error",
    # Performance Validation
    "PerformanceValidator",
    "PerformanceMetric",
    "PerformanceThreshold",
    "PerformanceReport",
    "get_performance_validator",
    "start_performance_validation",
    "stop_performance_validation",
    "record_performance_metric",
    "validate_performance",
    "get_performance_statistics",
    "performance_monitor",
    
    # Caching
    "LRUCache",
    "AsyncLRUCache",
    "LFUCache", 
    "AsyncLFUCache",
    "TTLCache",
    "AsyncTTLCache",
    "CacheManager",
    "CacheConfig",
    "CacheStats",
    
    # CLI - general console enums (from console level)
    "Colors",
    "Style",
    "Alignment",
    "BorderStyle",
    # CLI-specific classes
    "CliArgumentParser",
    "CliArgument",
    "CliCommand", 
    "ArgumentType",
    "CliColoredOutput",
    "colorize",
    "CliProgressBar",
    "CliSpinnerProgress",
    "CliMultiProgress",
    "CliProgressConfig",
    "CliTable",
    "CliTableFormatter",
    "CliColumn",
    "CliConsole",  # From console.cli.console
    
        # Validation
    "XModel",
    "Field", 
    "ValidationError",
    
    # Enterprise Features
    # NOTE: Schema Registry classes moved to exonware-xwschema
    # Available in: pip install exonware-xwschema
    # NOTE: Authentication provider implementations (OAuth2Provider, JWTProvider, SAMLProvider, EnterpriseAuth)
    # have been moved to xwauth, which extends xwsystem. Use xwauth for actual authentication providers.
    "TracingManager", "OpenTelemetryTracer", "JaegerTracer",
    "TracingError", "SpanContext", "TraceContext",
    "AuthenticationError", "AuthorizationError", "TokenExpiredError",
    "AAuthProvider", "ATokenInfo", "AUserInfo",
    "IAuthenticatable", "IAuthorization", "ISecurityToken",
    
    # Security Hazmat
    "AES_GCM",
    "ChaCha20Poly1305_Cipher",
    "X25519_KeyExchange",
    "Ed25519_Signature",
    "HKDF_Expand",
    "PBKDF2_Derive",
    "X509Certificate",
    "secure_hash",
    "constant_time_compare",
    "secure_random",
    "is_cryptography_available",
    
    # System Monitoring
    "SystemMonitor",
    "ProcessInfo",
    "SystemInfo",
    "DiskInfo",
    "NetworkInfo",
    "list_processes",
    "get_process",
    "get_system_info",
    "get_cpu_usage",
    "get_memory_usage",
    "get_hardware_info",
    "is_monitoring_available",
    
    # DateTime
    "humanize_timedelta",
    "humanize_timestamp",
    "time_ago",
    "time_until",
    "duration_to_human",
    "parse_human_duration",
    "TimezoneManager",
    "convert_timezone",
    "get_timezone_info",
    "list_timezones",
    "parse_datetime",
    "parse_date",
    "parse_time",
    "parse_iso8601",
    "parse_timestamp",
    "parse_timestamp_milliseconds",
    "format_datetime",
    "get_datetime",
    "get_date",
    "get_date_from_to_month",
    "calculate_duration_days",
    # String utilities
    "find_nth_occurrence",
    # Web utilities
    "validate_url_accessible",
    "extract_webpage_text",
    
    # IPC
    "ProcessManager",
    "ProcessInfo",
    "SharedMemoryManager", 
    "SharedData",
    "MessageQueue",
    "AsyncMessageQueue",
    "ProcessPool",
    "AsyncProcessPool",
    "Pipe",
    "AsyncPipe",
    
    # Convenience Functions - Quick access patterns
    "quick_serialize",
    "quick_deserialize", 
    "quick_hash",
    "quick_encrypt",
    "quick_decrypt",
    "list_available_formats",
    
    # Protocol Definitions
    "Serializable",
    "AsyncSerializable", 
    "Hashable",
    "Encryptable",
    "Validatable",
    "Cacheable",
    "Monitorable",
    "Configurable",
    "SerializationData",
    "HashAlgorithm",
    "EncryptionKey",
    "ValidationRule",
    "CacheKey",
    "ConfigValue",
    
    # Unified Facades (DX Improvements)
    "XWCache",
    "XWArchive",
    "XWIndex",
    "XWSecurity",
    "XWCrypto",
    "XWHTTP",
    "XWValidator",
    "XWMonitor",
    "XWConcurrency",
]

# =============================================================================
# UNIFIED FACADES - Simplified Developer Experience (DX)
# =============================================================================

# Import unified facades from their respective modules
from .caching.facade import XWCache
from .io.archive.facade import XWArchive
from .io.indexing.facade import XWIndex
from .security.facade import XWSecurity, XWCrypto
from .http_client.facade import XWHTTP
from .validation.facade import XWValidator
from .monitoring.facade import XWMonitor
from .threading.facade import XWConcurrency

# XWSerializer already exists in io.serialization.serializer
# XWIO already exists in io.facade
