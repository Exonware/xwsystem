#exonware/xwsystem/src/exonware/xwsystem/http_client/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.26
Generation Date: September 04, 2025
xwsystem HTTP Package
Provides high-performance HTTP client with retry mechanisms, 
connection pooling, and comprehensive error handling.
"""

from .client import HttpClient, AsyncHttpClient, HttpError, RetryConfig
from .advanced_client import (
    AdvancedHttpClient,
    AdvancedHttpConfig,
    Http2Config,
    StreamingConfig,
    MockTransport,
    MockResponse,
)
# Unified Facade
from .facade import XWHTTP
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
