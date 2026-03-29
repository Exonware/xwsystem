#exonware/xwsystem/src/exonware/xwsystem/http_client/advanced_client.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.26
Generation Date: September 04, 2025
Advanced HTTP client with HTTP/2, streaming, pluggable transports, and modern features.
"""

from __future__ import annotations
import asyncio
import os
import ssl
from contextlib import asynccontextmanager
from dataclasses import dataclass, field
from typing import Any

from collections.abc import AsyncIterator
from urllib.parse import urljoin
from .contracts import ITransport as Transport
# Prevent httpx from importing rich (Python 3.8+ only, no legacy deps)
os.environ.setdefault("HTTPX_NO_RICH", "1")
# Import httpx - lazy installation system will handle it if missing
import httpx
from ..config.logging_setup import get_logger
from ..monitoring.error_recovery import retry_with_backoff
from .client import HttpError, RetryConfig
logger = get_logger("xwsystem.http_client.advanced_client")
@dataclass


class StreamingConfig:
    """Configuration for streaming operations."""
    chunk_size: int = 8192
    buffer_size: int = 65536
    timeout_per_chunk: float = 30.0
    max_content_length: int | None = None  # None = unlimited
@dataclass


class Http2Config:
    """Configuration for HTTP/2 features."""
    enabled: bool = True
    max_concurrent_streams: int = 100
    initial_window_size: int = 65536
    max_frame_size: int = 16384
    enable_push: bool = False
@dataclass


class AdvancedHttpConfig:
    """Advanced HTTP client configuration."""
    http2: Http2Config = field(default_factory=Http2Config)
    streaming: StreamingConfig = field(default_factory=StreamingConfig)
    retry: RetryConfig = field(default_factory=RetryConfig)
    verify_ssl: bool = True
    ssl_context: ssl.SSLContext | None = None
    trust_env: bool = True
    follow_redirects: bool = True
    max_redirects: int = 20


class MockTransport:
    """Mock transport for testing purposes."""

    def __init__(self, responses: dict[str, Any]):
        """
        Initialize mock transport.
        Args:
            responses: Dictionary mapping URLs to mock responses
        """
        self.responses = responses
        self.requests = []

    async def handle_async_request(self, request: Any) -> Any:
        """Handle async request with mock response."""
        self.requests.append(request)
        url = str(request.url)
        if url in self.responses:
            response_data = self.responses[url]
            # Create mock response
            return MockResponse(
                status_code=response_data.get('status_code', 200),
                content=response_data.get('content', b''),
                headers=response_data.get('headers', {}),
                url=url
            )
        # Default 404 response
        return MockResponse(status_code=404, content=b'Not Found', url=url)

    def handle_request(self, request: Any) -> Any:
        """Handle sync request with mock response."""
        # For simplicity, delegate to async version
        return asyncio.run(self.handle_async_request(request))


class MockResponse:
    """Mock HTTP response for testing."""

    def __init__(self, status_code: int, content: bytes, headers: dict[str, str] = None, url: str = ""):
        self.status_code = status_code
        self.content = content
        self.headers = headers or {}
        self.url = url
        self.text = content.decode('utf-8', errors='ignore')

    def json(self):
        """Parse JSON response (xwsystem JsonSerializer, no stdlib json)."""
        from exonware.xwsystem import get_serializer, JsonSerializer
        return get_serializer(JsonSerializer).loads(self.text)

    def raise_for_status(self):
        """Raise exception for HTTP error status."""
        if 400 <= self.status_code < 600:
            raise HttpError(f"HTTP {self.status_code}", self.status_code, self.text)

    async def aiter_bytes(self, chunk_size: int = 8192):
        """Async byte iterator compatible with httpx.Response."""
        for i in range(0, len(self.content), chunk_size):
            yield self.content[i:i + chunk_size]

    async def aiter_lines(self):
        """Async line iterator compatible with httpx.Response."""
        for line in self.text.splitlines():
            yield line


class AdvancedHttpClient:
    """
    Advanced HTTP client with HTTP/2, streaming, pluggable transports, and modern features.
    Features:
    - HTTP/2 support with multiplexing
    - Streaming request/response bodies
    - Pluggable transport layer for testing
    - Advanced connection management
    - Comprehensive retry and error handling
    - SSL/TLS configuration
    - Request/response hooks
    """

    def __init__(
        self,
        base_url: str | None = None,
        timeout: float = 30.0,
        config: AdvancedHttpConfig | None = None,
        transport: Transport | None = None,
        default_headers: dict[str, str] | None = None,
    ) -> None:
        """
        Initialize advanced HTTP client.
        Args:
            base_url: Base URL for all requests
            timeout: Request timeout in seconds
            config: Advanced configuration options
            transport: Custom transport (for testing/mocking)
            default_headers: Default headers for all requests
        """
        # Lazy installation system will handle httpx if missing
        self.base_url = base_url
        self.config = config or AdvancedHttpConfig()
        self.transport = transport
        self.default_headers = default_headers or {}
        # Configure HTTP/2 if enabled
        http2 = self.config.http2.enabled and not transport  # Disable HTTP/2 with custom transports
        # Configure limits
        limits = httpx.Limits(
            max_connections=100,
            max_keepalive_connections=20
        )
        # Configure SSL context
        ssl_context = self.config.ssl_context
        if ssl_context is None and self.config.verify_ssl:
            ssl_context = ssl.create_default_context()
        # Initialize clients
        client_kwargs = {
            'base_url': base_url,
            'timeout': timeout,
            'limits': limits,
            'headers': self.default_headers,
            'http2': http2,
            'verify': ssl_context if ssl_context else self.config.verify_ssl,
            'trust_env': self.config.trust_env,
            'follow_redirects': self.config.follow_redirects,
            'max_redirects': self.config.max_redirects,
        }
        if transport:
            # Use custom transport
            self._client = None
            self._async_client = None
        else:
            self._client = httpx.Client(**client_kwargs)
            self._async_client = httpx.AsyncClient(**client_kwargs)

    async def __aenter__(self) -> AdvancedHttpClient:
        """Async context manager entry."""
        return self

    async def __aexit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Async context manager exit."""
        await self.aclose()

    def __enter__(self) -> AdvancedHttpClient:
        """Sync context manager entry."""
        return self

    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None:
        """Sync context manager exit."""
        self.close()

    def close(self) -> None:
        """Close sync client."""
        if self._client:
            self._client.close()

    async def aclose(self) -> None:
        """Close async client."""
        if self._async_client:
            await self._async_client.aclose()

    async def get(
        self,
        url: str,
        params: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async GET request."""
        return await self.request("GET", url, params=params, headers=headers, **kwargs)

    async def post(
        self,
        url: str,
        json: Any | None = None,
        data: Any | None = None,
        files: dict[str, Any] | None = None,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async POST request."""
        return await self.request("POST", url, json=json, data=data, files=files, headers=headers, **kwargs)

    async def put(
        self,
        url: str,
        json: Any | None = None,
        data: Any | None = None,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async PUT request."""
        return await self.request("PUT", url, json=json, data=data, headers=headers, **kwargs)

    async def patch(
        self,
        url: str,
        json: Any | None = None,
        data: Any | None = None,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async PATCH request."""
        return await self.request("PATCH", url, json=json, data=data, headers=headers, **kwargs)

    async def delete(
        self,
        url: str,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async DELETE request."""
        return await self.request("DELETE", url, headers=headers, **kwargs)

    async def head(
        self,
        url: str,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async HEAD request."""
        return await self.request("HEAD", url, headers=headers, **kwargs)

    async def options(
        self,
        url: str,
        headers: dict[str, str] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Async OPTIONS request."""
        return await self.request("OPTIONS", url, headers=headers, **kwargs)

    async def request(
        self,
        method: str,
        url: str,
        headers: dict[str, str] | None = None,
        params: dict[str, Any] | None = None,
        json: Any | None = None,
        data: Any | None = None,
        files: dict[str, Any] | None = None,
        **kwargs
    ) -> httpx.Response:
        """Make async HTTP request with retry logic."""
        # Combine headers
        request_headers = {**self.default_headers}
        if headers:
            request_headers.update(headers)
        # Prepare request arguments
        request_kwargs = {
            'method': method,
            'url': url,
            'headers': request_headers,
            'params': params,
            'files': files,
            **kwargs
        }
        if json is not None:
            request_kwargs['json'] = json
        elif data is not None:
            request_kwargs['data'] = data
        # Handle custom transport
        if self.transport:
            @retry_with_backoff(
                max_retries=self.config.retry.max_retries,
                base_delay=self.config.retry.base_delay,
                max_delay=self.config.retry.max_delay,
                backoff_factor=self.config.retry.exponential_base,
                exceptions=tuple([*self.config.retry.retry_on_exceptions, HttpError]),
            )
            async def _transport_request():
                request_url = url
                if self.base_url and isinstance(url, str) and url.startswith("/"):
                    request_url = urljoin(self.base_url, url)
                request_kwargs_for_transport = dict(request_kwargs)
                request_kwargs_for_transport["url"] = request_url
                request = httpx.Request(**request_kwargs_for_transport)
                response = await self.transport.handle_async_request(request)
                if self._should_retry(response, None):
                    raise HttpError(
                        f"HTTP {response.status_code}",
                        status_code=response.status_code,
                        response_data=getattr(response, "text", "")
                    )
                if hasattr(response, "raise_for_status"):
                    response.raise_for_status()
                return response
            return await _transport_request()
        @retry_with_backoff(
            max_retries=self.config.retry.max_retries,
            base_delay=self.config.retry.base_delay,
            max_delay=self.config.retry.max_delay,
            backoff_factor=self.config.retry.exponential_base,
            exceptions=tuple([*self.config.retry.retry_on_exceptions, HttpError]),
        )
        async def _request() -> httpx.Response:
            try:
                response = await self._async_client.request(**request_kwargs)
                if self._should_retry(response, None):
                    raise HttpError(
                        f"HTTP {response.status_code}: {response.text}",
                        status_code=response.status_code,
                        response_data=response.text
                    )
                return response
            except Exception as e:
                if self._should_retry(None, e):
                    logger.warning(f"Retrying request due to exception: {e}")
                    raise
                else:
                    raise HttpError(f"Request failed: {e}") from e
        return await _request()
    @asynccontextmanager

    async def stream(
        self,
        method: str,
        url: str,
        headers: dict[str, str] | None = None,
        params: dict[str, Any] | None = None,
        json: Any | None = None,
        data: Any | None = None,
        **kwargs
    ):
        """
        Stream HTTP request/response.
        Args:
            method: HTTP method
            url: Request URL
            headers: Request headers
            params: Query parameters
            json: JSON data
            data: Request body data
            **kwargs: Additional request arguments
        Yields:
            Streaming response object
        """
        # Combine headers
        request_headers = {**self.default_headers}
        if headers:
            request_headers.update(headers)
        # Prepare request arguments
        request_kwargs = {
            'method': method,
            'url': url,
            'headers': request_headers,
            'params': params,
            **kwargs
        }
        if json is not None:
            request_kwargs['json'] = json
        elif data is not None:
            request_kwargs['data'] = data
        if self.transport:
            # Custom transport doesn't support streaming
            response = await self.request(method, url, headers, params, json, data, **kwargs)
            yield response
            return
        async with self._async_client.stream(**request_kwargs) as response:
            yield response

    async def stream_content(
        self,
        method: str,
        url: str,
        chunk_size: int | None = None,
        **kwargs
    ) -> AsyncIterator[bytes]:
        """
        Stream response content in chunks.
        Args:
            method: HTTP method
            url: Request URL
            chunk_size: Size of each chunk
            **kwargs: Additional request arguments
        Yields:
            Content chunks as bytes
        """
        chunk_size = chunk_size or self.config.streaming.chunk_size
        async with self.stream(method, url, **kwargs) as response:
            response.raise_for_status()
            total_size = 0
            max_size = self.config.streaming.max_content_length
            async for chunk in response.aiter_bytes(chunk_size):
                if max_size and total_size + len(chunk) > max_size:
                    raise HttpError(f"Response too large: exceeds {max_size} bytes")
                total_size += len(chunk)
                yield chunk

    async def stream_lines(
        self,
        method: str,
        url: str,
        encoding: str = 'utf-8',
        **kwargs
    ) -> AsyncIterator[str]:
        """
        Stream response content line by line.
        Args:
            method: HTTP method
            url: Request URL
            encoding: Text encoding
            **kwargs: Additional request arguments
        Yields:
            Content lines as strings
        """
        async with self.stream(method, url, **kwargs) as response:
            response.raise_for_status()
            async for line in response.aiter_lines():
                yield line

    async def download_file(
        self,
        url: str,
        file_path: str,
        chunk_size: int | None = None,
        progress_callback: callable | None = None,
        **kwargs
    ) -> None:
        """
        Download file with streaming and progress tracking.
        Args:
            url: URL to download from
            file_path: Local file path to save to
            chunk_size: Size of each chunk
            progress_callback: Optional callback for progress updates
            **kwargs: Additional request arguments
        """
        from ..io.stream.async_operations import async_safe_write_bytes
        import platform
        import aiofiles
        chunk_size = chunk_size or self.config.streaming.chunk_size
        async with self.stream("GET", url, **kwargs) as response:
            response.raise_for_status()
            total_size = int(response.headers.get('content-length', 0))
            downloaded = 0
            content = b''
            async for chunk in response.aiter_bytes(chunk_size):
                content += chunk
                downloaded += len(chunk)
                if progress_callback:
                    progress_callback(downloaded, total_size)
            # Windows keeps NamedTemporaryFile handles locked; direct write avoids
            # atomic rename/remove failures when caller holds an open handle.
            if platform.system() == "Windows":
                async with aiofiles.open(file_path, "wb") as f:
                    await f.write(content)
            else:
                await async_safe_write_bytes(file_path, content)

    def _should_retry(self, response: httpx.Response | None, exception: Exception | None) -> bool:
        """Determine if a request should be retried."""
        if exception:
            return any(isinstance(exception, exc_type) for exc_type in self.config.retry.retry_on_exceptions)
        if response:
            return response.status_code in self.config.retry.retry_on_status
        return False
    # Sync wrappers (delegate to async)

    def _run_sync(self, coro):
        """Run coroutine safely from sync context (even inside an active loop)."""
        try:
            asyncio.get_running_loop()
        except RuntimeError:
            return asyncio.run(coro)

        # Already inside an event loop: run in a dedicated thread.
        import threading
        result: dict[str, Any] = {}
        error: dict[str, BaseException] = {}

        def _runner():
            try:
                result["value"] = asyncio.run(coro)
            except BaseException as exc:  # pragma: no cover
                error["exc"] = exc

        thread = threading.Thread(target=_runner, daemon=True)
        thread.start()
        thread.join()
        if "exc" in error:
            raise error["exc"]
        return result.get("value")

    def sync_get(self, url: str, **kwargs) -> httpx.Response:
        """Sync GET request."""
        return self._run_sync(self.get(url, **kwargs))

    def sync_post(self, url: str, **kwargs) -> httpx.Response:
        """Sync POST request."""
        return self._run_sync(self.post(url, **kwargs))

    def sync_put(self, url: str, **kwargs) -> httpx.Response:
        """Sync PUT request."""
        return self._run_sync(self.put(url, **kwargs))

    def sync_patch(self, url: str, **kwargs) -> httpx.Response:
        """Sync PATCH request."""
        return self._run_sync(self.patch(url, **kwargs))

    def sync_delete(self, url: str, **kwargs) -> httpx.Response:
        """Sync DELETE request."""
        return self._run_sync(self.delete(url, **kwargs))

    def sync_request(self, method: str, url: str, **kwargs) -> httpx.Response:
        """Sync HTTP request."""
        return self._run_sync(self.request(method, url, **kwargs))
