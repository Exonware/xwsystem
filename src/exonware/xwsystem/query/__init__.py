#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/query/__init__.py

Query foundation layer for eXonware ecosystem.

This package defines query contracts and a small registry that higher-level
libraries (e.g., xwquery) can register into, while lower-level libraries
(e.g., xwnode) can depend on without creating circular dependencies.

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.5
Generation Date: 28-Dec-2025
"""

from .contracts import IQueryProvider, QueryResult
from .errors import QueryProviderError, QueryProviderNotRegisteredError
from .registry import (
    get_query_provider,
    get_query_provider_registry,
    register_query_provider,
    reset_query_provider_registry,
)

__all__ = [
    "IQueryProvider",
    "QueryResult",
    "QueryProviderError",
    "QueryProviderNotRegisteredError",
    "get_query_provider",
    "get_query_provider_registry",
    "register_query_provider",
    "reset_query_provider_registry",
]
