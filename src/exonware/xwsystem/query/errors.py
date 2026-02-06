#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/query/errors.py

Query provider errors (foundation layer).

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.3
Generation Date: 28-Dec-2025
"""


class QueryProviderError(Exception):
    """Base error for query provider integration."""


class QueryProviderNotRegisteredError(QueryProviderError):
    """
    Raised when a consumer requests a query provider but none is registered.
    """
