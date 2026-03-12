#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/query/errors.py
Query provider errors (foundation layer).
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.5
Generation Date: 28-Dec-2025
"""


class QueryProviderError(Exception):
    """Base error for query provider integration."""


class QueryProviderNotRegisteredError(QueryProviderError):
    """
    Raised when a consumer requests a query provider but none is registered.
    """
