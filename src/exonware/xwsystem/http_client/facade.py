#exonware/xwsystem/src/exonware/xwsystem/http_client/facade.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.1
Generation Date: January 2026
XWHTTP - Unified HTTP Client Facade
Simplified API for HTTP operations:
- Simple requests (GET, POST, PUT, DELETE, etc.)
- Client instances with configuration
- Async support
- Retry logic
"""

from typing import Any, Optional, Dict
from pathlib import Path
from .client import HttpClient, AsyncHttpClient, HttpError, RetryConfig
from .advanced_client import AdvancedHttpClient, AdvancedHttpConfig
from ..config.logging_setup import get_logger
logger = get_logger(__name__)


class XWHTTP:
    """
    Unified HTTP client facade - simple API for HTTP operations.
    Examples:
        >>> # Simple requests
        >>> response = XWHTTP.get("https://api.example.com/data")
        >>> response = XWHTTP.post("https://api.example.com/users", json={"name": "John"})
        >>> # Client instance
        >>> client = XWHTTP(base_url="https://api.example.com", timeout=30)
        >>> response = client.get("/users")
        >>> response = client.post("/users", json=data)
        >>> # Async
        >>> async def fetch():
        ...     async with XWHTTP() as client:
        ...         response = await client.get("https://api.example.com")
    """

    def __init__(
        self,
        base_url: Optional[str] = None,
        timeout: int = 30,
        retries: int = 3,
        async_mode: bool = False,
        **kwargs
    ):
        """
        Initialize HTTP client.
        Args:
            base_url: Base URL for all requests
            timeout: Request timeout in seconds
            retries: Number of retry attempts
            async_mode: Use async client
            **kwargs: Additional client configuration
        """
        self.base_url = base_url
        self.timeout = timeout
        self.retries = retries
        self.async_mode = async_mode
        self.kwargs = kwargs
        # Create client instance
        if async_mode:
            self._client = AsyncHttpClient(
                base_url=base_url,
                timeout=timeout,
                retry_config=RetryConfig(max_retries=retries),
                **kwargs
            )
        else:
            self._client = HttpClient(
                base_url=base_url,
                timeout=timeout,
                retry_config=RetryConfig(max_retries=retries),
                **kwargs
            )

    def get(self, url: str, **kwargs) -> Any:
        """GET request."""
        full_url = self._build_url(url)
        return self._client.get(full_url, **kwargs)

    def post(self, url: str, **kwargs) -> Any:
        """POST request."""
        full_url = self._build_url(url)
        return self._client.post(full_url, **kwargs)

    def put(self, url: str, **kwargs) -> Any:
        """PUT request."""
        full_url = self._build_url(url)
        return self._client.put(full_url, **kwargs)

    def delete(self, url: str, **kwargs) -> Any:
        """DELETE request."""
        full_url = self._build_url(url)
        return self._client.delete(full_url, **kwargs)

    def patch(self, url: str, **kwargs) -> Any:
        """PATCH request."""
        full_url = self._build_url(url)
        return self._client.patch(full_url, **kwargs)

    def _build_url(self, url: str) -> str:
        """Build full URL from base_url and relative URL."""
        if self.base_url and not url.startswith(("http://", "https://")):
            return f"{self.base_url.rstrip('/')}/{url.lstrip('/')}"
        return url

    def __enter__(self):
        """Context manager entry."""
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        # Clients handle their own cleanup
        pass
    @staticmethod

    def get(url: str, **kwargs) -> Any:
        """Static GET request."""
        client = HttpClient()
        return client.get(url, **kwargs)
    @staticmethod

    def post(url: str, **kwargs) -> Any:
        """Static POST request."""
        client = HttpClient()
        return client.post(url, **kwargs)
    @staticmethod

    def put(url: str, **kwargs) -> Any:
        """Static PUT request."""
        client = HttpClient()
        return client.put(url, **kwargs)
    @staticmethod

    def delete(url: str, **kwargs) -> Any:
        """Static DELETE request."""
        client = HttpClient()
        return client.delete(url, **kwargs)
    @staticmethod

    def patch(url: str, **kwargs) -> Any:
        """Static PATCH request."""
        client = HttpClient()
        return client.patch(url, **kwargs)
