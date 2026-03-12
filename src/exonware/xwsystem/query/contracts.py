#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/query/contracts.py
Query contracts (foundation layer).
These interfaces live in xwsystem to avoid circular dependencies between:
- xwnode (data structures / node facade)
- xwquery (query engine)
- xwdata (data facade built on xwnode + xwquery)
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.4
Generation Date: 28-Dec-2025
"""

from __future__ import annotations
from typing import Any, Protocol, runtime_checkable, TypeAlias
QueryResult: TypeAlias = Any
@runtime_checkable

class IQueryProvider(Protocol):
    """
    Interface for query execution providers.
    Implementations live in higher-level packages (e.g., xwquery) and register
    themselves via `exonware.xwsystem.query.registry`.
    """
    provider_id: str

    def execute(
        self,
        query: str,
        data: Any,
        *,
        format: str | None = None,
        auto_detect: bool = True,
        **opts: Any,
    ) -> QueryResult:
        """
        Execute a query against data.
        Args:
            query: Query string (SQL, GraphQL, xwquery script, etc.)
            data: Target data (XWNode, dict, list, etc.)
            format: Optional explicit format identifier
            auto_detect: Whether provider should auto-detect when format is None
            **opts: Provider-specific options
        """
        ...
