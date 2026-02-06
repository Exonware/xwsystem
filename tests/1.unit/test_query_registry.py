#!/usr/bin/env python3
"""
#exonware/xwsystem/tests/1.unit/test_query_registry.py

Unit tests for xwsystem.query registry (foundation layer).

Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.0.1
Generation Date: 28-Dec-2025
"""

import pytest

from exonware.xwsystem.query import (
    QueryProviderNotRegisteredError,
    get_query_provider_registry,
    reset_query_provider_registry,
)
from exonware.xwsystem.query.contracts import IQueryProvider


class DummyQueryProvider(IQueryProvider):
    provider_id = "dummy"

    def execute(self, query: str, data, *, format: str | None = None, auto_detect: bool = True, **opts):
        return {"query": query, "format": format, "auto_detect": auto_detect, "data": data, "opts": opts}


@pytest.mark.xwsystem_unit
class TestQueryProviderRegistry:
    def teardown_method(self):
        reset_query_provider_registry()

    def test_require_default_raises_when_empty(self):
        registry = get_query_provider_registry()
        with pytest.raises(QueryProviderNotRegisteredError):
            registry.require_default()

    def test_register_sets_default_and_execute(self):
        registry = get_query_provider_registry()
        registry.register(DummyQueryProvider())

        provider = registry.require_default()
        result = provider.execute("SELECT 1", {"k": "v"}, format="sql", foo="bar")

        assert isinstance(result, dict)
        assert result["query"] == "SELECT 1"
        assert result["format"] == "sql"
        assert result["opts"]["foo"] == "bar"
