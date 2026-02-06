#!/usr/bin/env python3
"""
Source reader: load text content from a URI or local path.

All logic for "read from path or URI" lives in xwsystem. Supports multiple
schemes (file, http, https, ftp, etc.) with configurable security:
- allowed_schemes: which protocols are allowed
- allow_external: whether non-file (remote) sources are allowed
- timeout_sec, max_size_mb: limits

Callers (e.g. xwdata) pass SourceLoadConfig from their own config; xwsystem
enforces it. No scheme or fetch logic in xwdata—single entry point here and
via XWFile.
"""

from __future__ import annotations

import asyncio
import urllib.parse
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from .common.atomic import FileOperationError
from .stream.async_operations import async_safe_read_text


# -----------------------------------------------------------------------------
# Configuration (consumed by xwdata / xwschema; defined here for single source)
# -----------------------------------------------------------------------------

@dataclass
class SourceLoadConfig:
    """
    Configuration for loading from a URI or path. Enforced by xwsystem.

    Callers (xwdata) set this from their SecurityConfig/ReferenceConfig;
    xwsystem validates scheme and size against these values.
    """
    allowed_schemes: tuple[str, ...] = ('file', 'http', 'https', 'ftp')
    allow_external: bool = True
    timeout_sec: float = 30.0
    max_size_mb: float = 100.0
    encoding: str = 'utf-8'

    @classmethod
    def strict(cls) -> SourceLoadConfig:
        """Local files only, no external URIs."""
        return cls(
            allowed_schemes=('file',),
            allow_external=False,
            timeout_sec=10.0,
            max_size_mb=10.0,
        )

    @classmethod
    def relaxed(cls) -> SourceLoadConfig:
        """Allow common external schemes."""
        return cls(
            allowed_schemes=('file', 'http', 'https', 'ftp'),
            allow_external=True,
            timeout_sec=60.0,
            max_size_mb=500.0,
        )


# -----------------------------------------------------------------------------
# Scheme detection and validation
# -----------------------------------------------------------------------------

def get_scheme(uri_or_path: str) -> str:
    """
    Return the URI scheme, or 'file' for a bare path.

    Args:
        uri_or_path: URI (http://..., file://...) or local path

    Returns:
        Lowercase scheme (e.g. 'file', 'http', 'https', 'ftp') or 'file' for paths.
    """
    if not uri_or_path or not isinstance(uri_or_path, str):
        return 'file'
    parsed = urllib.parse.urlparse(uri_or_path)
    scheme = (parsed.scheme or '').lower()
    if scheme and parsed.netloc:
        return scheme
    return 'file'


def is_external_scheme(scheme: str) -> bool:
    """Return True if scheme is not local file."""
    return scheme not in ('file', '')


def is_http_url(path_or_url: str) -> bool:
    """Return True if the string is an http(s) URL (convenience)."""
    return get_scheme(path_or_url) in ('http', 'https')


def _validate_config(uri_or_path: str, config: SourceLoadConfig) -> None:
    """Raise FileOperationError if URI/path is not allowed by config."""
    scheme = get_scheme(uri_or_path)
    if scheme not in config.allowed_schemes:
        raise FileOperationError(
            f"Scheme '{scheme}' not allowed. Allowed: {config.allowed_schemes}"
        )
    if is_external_scheme(scheme) and not config.allow_external:
        raise FileOperationError(
            "External sources are disabled (allow_external=False)"
        )


# -----------------------------------------------------------------------------
# Async read: single entry point for xwsystem (and XWFile / xwdata)
# -----------------------------------------------------------------------------

async def read_source_text(
    uri_or_path: str,
    config: Optional[SourceLoadConfig] = None,
    timeout_sec: Optional[float] = None,
    max_size_mb: Optional[float] = None,
    encoding: Optional[str] = None,
) -> tuple[str, dict]:
    """
    Read text from a URI or local path (async). All scheme and fetch logic here.

    Security and behavior are driven by config (from xwdata). Supported schemes:
    - file: local path (existing async_safe_read_text)
    - http, https: aiohttp or urllib
    - ftp: urllib (ftp://)

    Args:
        uri_or_path: File path or URI (file://, http://, https://, ftp://)
        config: SourceLoadConfig (allowed_schemes, allow_external, limits). If None, relaxed defaults.
        timeout_sec: Override config timeout (optional)
        max_size_mb: Override config max size (optional)
        encoding: Override config encoding for file reads (optional)

    Returns:
        (content_str, metadata) with at least 'source' and 'content_type'.
    """
    cfg = config or SourceLoadConfig.relaxed()
    timeout = timeout_sec if timeout_sec is not None else cfg.timeout_sec
    max_mb = max_size_mb if max_size_mb is not None else cfg.max_size_mb
    enc = encoding or cfg.encoding

    _validate_config(uri_or_path, cfg)
    scheme = get_scheme(uri_or_path)

    if scheme == 'file':
        path = uri_or_path
        if path.startswith('file://'):
            path = urllib.parse.unquote(urllib.parse.urlparse(path).path)
        content = await async_safe_read_text(
            Path(path), encoding=enc, max_size_mb=max_mb
        )
        return content, {'source': uri_or_path, 'content_type': None}

    if scheme in ('http', 'https'):
        return await _read_http_text(uri_or_path, timeout, max_mb)

    if scheme == 'ftp':
        return await _read_ftp_text(uri_or_path, timeout, max_mb)

    raise FileOperationError(f"Unsupported scheme for reading: {scheme}")


async def _read_http_text(
    url: str, timeout_sec: float, max_size_mb: float
) -> tuple[str, dict]:
    """Fetch http(s) URL; return (text, metadata)."""
    content_type: Optional[str] = None
    try:
        try:
            import aiohttp
            timeout = aiohttp.ClientTimeout(total=timeout_sec)
            async with aiohttp.ClientSession(timeout=timeout) as session:
                async with session.get(url) as response:
                    response.raise_for_status()
                    raw = await response.read()
                    content_type = response.headers.get('Content-Type') or None
        except ImportError:
            def _fetch() -> tuple[bytes, Optional[str]]:
                req = urllib.request.Request(url)
                with urllib.request.urlopen(req, timeout=timeout_sec) as resp:
                    ct = resp.headers.get('Content-Type')
                    return resp.read(), ct if ct else None

            loop = asyncio.get_event_loop()
            raw, content_type = await loop.run_in_executor(None, _fetch)
    except Exception as e:
        raise FileOperationError(f"Failed to load from URL: {e}") from e

    size_mb = len(raw) / (1024 * 1024)
    if size_mb > max_size_mb:
        raise FileOperationError(
            f"URL response size ({size_mb:.1f}MB) exceeds max ({max_size_mb}MB)"
        )
    text = raw.decode('utf-8', errors='replace')
    return text, {'source': url, 'content_type': content_type}


async def _read_ftp_text(
    url: str, timeout_sec: float, max_size_mb: float
) -> tuple[str, dict]:
    """Fetch ftp URL via urllib in executor; return (text, metadata)."""
    def _fetch() -> bytes:
        with urllib.request.urlopen(url, timeout=timeout_sec) as resp:
            return resp.read()

    loop = asyncio.get_event_loop()
    try:
        raw = await loop.run_in_executor(None, _fetch)
    except Exception as e:
        raise FileOperationError(f"Failed to load from FTP: {e}") from e

    size_mb = len(raw) / (1024 * 1024)
    if size_mb > max_size_mb:
        raise FileOperationError(
            f"FTP response size ({size_mb:.1f}MB) exceeds max ({max_size_mb}MB)"
        )
    text = raw.decode('utf-8', errors='replace')
    return text, {'source': url, 'content_type': None}
