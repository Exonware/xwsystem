#exonware/xwsystem/src/exonware/xwsystem/http_client/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.43
Generation Date: September 04, 2025
xwsystem HTTP Package
Provides high-performance HTTP client with retry mechanisms, 
connection pooling, and comprehensive error handling.
"""

try:
    from .client import HttpClient, AsyncHttpClient, HttpError, RetryConfig
except (ImportError, ModuleNotFoundError):
    HttpClient = None
    AsyncHttpClient = None
    HttpError = None
    RetryConfig = None

try:
    from .advanced_client import (
        AdvancedHttpClient,
        AdvancedHttpConfig,
        Http2Config,
        StreamingConfig,
        MockTransport,
        MockResponse,
    )
except (ImportError, ModuleNotFoundError):
    AdvancedHttpClient = None
    AdvancedHttpConfig = None
    Http2Config = None
    StreamingConfig = None
    MockTransport = None
    MockResponse = None

# Unified Facade
try:
    from .facade import XWHTTP
except (ImportError, ModuleNotFoundError):
    XWHTTP = None
__all__ = [
    # Unified Facade
    "XWHTTP",
    # Core Classes
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
]
