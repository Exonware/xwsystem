#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/query/registry.py
Global query provider registry (foundation layer).
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.8
Generation Date: 28-Dec-2025
"""

from __future__ import annotations
from threading import RLock
from .contracts import IQueryProvider
from .errors import QueryProviderNotRegisteredError


class QueryProviderRegistry:
    """
    Thread-safe registry for query providers.
    Design notes:
    - Multiple providers may be registered (future-proofing).
    - A default provider may be selected; consumers typically use the default.
    """

    def __init__(self) -> None:
        self._lock = RLock()
        self._providers: dict[str, IQueryProvider] = {}
        self._default_provider_id: str | None = None

    def register(self, provider: IQueryProvider, *, overwrite: bool = False, make_default: bool = True) -> None:
        provider_id = provider.provider_id.strip().lower()
        if not provider_id:
            raise ValueError("provider.provider_id must be a non-empty string")
        with self._lock:
            if provider_id in self._providers and not overwrite:
                return
            self._providers[provider_id] = provider
            if make_default or self._default_provider_id is None:
                self._default_provider_id = provider_id

    def unregister(self, provider_id: str) -> bool:
        pid = provider_id.strip().lower()
        with self._lock:
            if pid not in self._providers:
                return False
            del self._providers[pid]
            if self._default_provider_id == pid:
                self._default_provider_id = next(iter(self._providers.keys()), None)
            return True

    def get(self, provider_id: str) -> IQueryProvider | None:
        pid = provider_id.strip().lower()
        with self._lock:
            return self._providers.get(pid)

    def get_default(self) -> IQueryProvider | None:
        with self._lock:
            if not self._default_provider_id:
                return None
            return self._providers.get(self._default_provider_id)

    def require_default(self) -> IQueryProvider:
        provider = self.get_default()
        if provider is None:
            raise QueryProviderNotRegisteredError(
                "No query provider registered. "
                "Install and import exonware-xwquery to register a provider."
            )
        return provider

    def list_provider_ids(self) -> list[str]:
        with self._lock:
            return list(self._providers.keys())

    def clear(self) -> None:
        with self._lock:
            self._providers.clear()
            self._default_provider_id = None
_global_registry: QueryProviderRegistry | None = None
_global_lock = RLock()


def get_query_provider_registry() -> QueryProviderRegistry:
    """
    Get the global query provider registry (thread-safe singleton).
    """
    global _global_registry
    if _global_registry is None:
        with _global_lock:
            if _global_registry is None:
                _global_registry = QueryProviderRegistry()
    return _global_registry


def reset_query_provider_registry() -> None:
    """
    Reset the global registry (primarily for testing).
    """
    global _global_registry
    with _global_lock:
        _global_registry = None


def register_query_provider(provider: IQueryProvider, *, overwrite: bool = False, make_default: bool = True) -> None:
    """
    Register a query provider in the global registry.
    """
    get_query_provider_registry().register(provider, overwrite=overwrite, make_default=make_default)


def get_query_provider(provider_id: str | None = None) -> IQueryProvider | None:
    """
    Get a provider by id, or the default provider if provider_id is None.
    """
    registry = get_query_provider_registry()
    if provider_id is None:
        return registry.get_default()
    return registry.get(provider_id)
