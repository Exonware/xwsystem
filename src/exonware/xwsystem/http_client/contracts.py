#exonware/xwsystem/src/exonware/xwsystem/http_client/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.31
Generation Date: September 04, 2025
HTTP module contracts - interfaces and enums for HTTP client functionality.
"""

from __future__ import annotations
from typing import Any, Protocol, runtime_checkable

from collections.abc import AsyncGenerator
from urllib.parse import ParseResult
# Import enums from types module
from .defs import (
    HttpMethod,
    HttpStatus,
    ContentType,
    AuthType,
    RetryStrategy
)
@runtime_checkable

class IHttpClient(Protocol):
    """Interface for HTTP client operations."""

    async def get(self, url: str, **kwargs) -> IHttpResponse:
        """Make GET request."""
        ...

    async def post(self, url: str, data: Any | None = None, **kwargs) -> IHttpResponse:
        """Make POST request."""
        ...

    async def put(self, url: str, data: Any | None = None, **kwargs) -> IHttpResponse:
        """Make PUT request."""
        ...

    async def delete(self, url: str, **kwargs) -> IHttpResponse:
        """Make DELETE request."""
        ...

    async def request(self, method: HttpMethod, url: str, **kwargs) -> IHttpResponse:
        """Make HTTP request."""
        ...
@runtime_checkable

class IHttpResponse(Protocol):
    """Interface for HTTP response."""
    @property

    def status_code(self) -> int:
        """Response status code."""
        ...
    @property

    def headers(self) -> dict[str, str]:
        """Response headers."""
        ...
    @property

    def content(self) -> bytes:
        """Response content as bytes."""
        ...
    @property

    def text(self) -> str:
        """Response content as text."""
        ...

    def json(self) -> Any:
        """Response content as JSON."""
        ...

    async def stream(self) -> AsyncGenerator[bytes]:
        """Stream response content."""
        ...
@runtime_checkable

class IHttpSession(Protocol):
    """Interface for HTTP session management."""

    async def __aenter__(self) -> IHttpSession:
        """Async context manager entry."""
        ...

    async def __aexit__(self, exc_type, exc_val, exc_tb) -> None:
        """Async context manager exit."""
        ...

    def set_auth(self, auth_type: AuthType, **kwargs) -> None:
        """Set authentication."""
        ...

    def set_headers(self, headers: dict[str, str]) -> None:
        """Set default headers."""
        ...

    def set_timeout(self, timeout: float) -> None:
        """Set request timeout."""
        ...
@runtime_checkable

class IRetryConfig(Protocol):
    """Interface for retry configuration."""
    @property

    def max_retries(self) -> int:
        """Maximum number of retries."""
        ...
    @property

    def strategy(self) -> RetryStrategy:
        """Retry strategy."""
        ...
    @property

    def backoff_factor(self) -> float:
        """Backoff factor for retries."""
        ...
    @property

    def retry_on_status(self) -> list[int]:
        """Status codes to retry on."""
        ...
@runtime_checkable

class ITransport(Protocol):
    """Abstract base class for HTTP transport implementations."""

    async def request(self, method: str, url: str, **kwargs) -> Any:
        """Make an HTTP request."""
        ...
