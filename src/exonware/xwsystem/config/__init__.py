#exonware/xwsystem/src/exonware/xwsystem/config/__init__.py
"""
Configuration module for xwsystem
This module provides configuration management for various xwsystem features.
Author: eXonware Backend Team
Company: eXonware.com
Email: connect@exonware.com
Version: 0.9.0.17
Generated: 2025-01-27
"""
# Import all default constants

from .defaults import (
    DEFAULT_ENCODING,
    DEFAULT_PATH_DELIMITER,
    DEFAULT_LOCK_TIMEOUT,
    DEFAULT_MAX_FILE_SIZE_MB,
    DEFAULT_MAX_MEMORY_USAGE_MB,
    DEFAULT_MAX_DICT_DEPTH,
    DEFAULT_MAX_PATH_DEPTH,
    DEFAULT_MAX_PATH_LENGTH,
    DEFAULT_MAX_RESOLUTION_DEPTH,
    DEFAULT_MAX_TO_DICT_SIZE_MB,
    DEFAULT_MAX_CIRCULAR_DEPTH,
    DEFAULT_MAX_EXTENSION_LENGTH,
    DEFAULT_CONTENT_SNIPPET_LENGTH,
    DEFAULT_MAX_TRAVERSAL_DEPTH,
    URI_SCHEME_SEPARATOR,
    JSON_POINTER_PREFIX,
    PATH_SEPARATOR_FORWARD,
    PATH_SEPARATOR_BACKWARD,
    CIRCULAR_REFERENCE_PLACEHOLDER,
    MAX_DEPTH_EXCEEDED_PLACEHOLDER,
    LOGGING_ENABLED,
    LOGGING_LEVEL,
    DefaultConfig,
)
# Import performance configuration
from .performance import (
    PerformanceConfig,
    get_performance_config,
    set_performance_config,
)
# Import logging setup
from .logging_setup import get_logger, setup_logging
# Import console event logger (moved to console module)
from ..console import ConsoleEventLogger, ConsoleEvent
__all__ = [
    # Default constants
    'DEFAULT_ENCODING',
    'DEFAULT_PATH_DELIMITER',
    'DEFAULT_LOCK_TIMEOUT',
    'DEFAULT_MAX_FILE_SIZE_MB',
    'DEFAULT_MAX_MEMORY_USAGE_MB',
    'DEFAULT_MAX_DICT_DEPTH',
    'DEFAULT_MAX_PATH_DEPTH',
    'DEFAULT_MAX_PATH_LENGTH',
    'DEFAULT_MAX_RESOLUTION_DEPTH',
    'DEFAULT_MAX_TO_DICT_SIZE_MB',
    'DEFAULT_MAX_CIRCULAR_DEPTH',
    'DEFAULT_MAX_EXTENSION_LENGTH',
    'DEFAULT_CONTENT_SNIPPET_LENGTH',
    'DEFAULT_MAX_TRAVERSAL_DEPTH',
    'URI_SCHEME_SEPARATOR',
    'JSON_POINTER_PREFIX',
    'PATH_SEPARATOR_FORWARD',
    'PATH_SEPARATOR_BACKWARD',
    'CIRCULAR_REFERENCE_PLACEHOLDER',
    'MAX_DEPTH_EXCEEDED_PLACEHOLDER',
    'LOGGING_ENABLED',
    'LOGGING_LEVEL',
    'DefaultConfig',
    # Performance configuration
    'PerformanceConfig',
    'get_performance_config',
    'set_performance_config',
    # Logging
    'setup_logging',
    'get_logger',
    # Console Event Logger
    'ConsoleEventLogger',
    'ConsoleEvent',
]
