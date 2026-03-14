#!/usr/bin/env python3
"""
Unit tests for TwoTierCache (ICache contract: put, get(key, default)).
Regression: TwoTierCache must implement put() and get(key, default) so that
benchmark harness and cache facades can use the same interface.
"""

from __future__ import annotations
import tempfile
from pathlib import Path
import pytest
from exonware.xwsystem.caching.two_tier_cache import TwoTierCache
@pytest.mark.xwsystem_unit
@pytest.mark.xwsystem_caching


class TestTwoTierCachePutGetContract:
    """Regression: TwoTierCache put() and get(key, default) for cache contract."""

    def test_put_exists_and_persists_value(self):
        """put(key, value) stores value; get returns it (benchmark harness uses put)."""
        with tempfile.TemporaryDirectory(prefix="two_tier_") as tmp:
            cache = TwoTierCache(memory_size=100, disk_size=200, disk_cache_dir=tmp)
            cache.put("k1", 42)
            assert cache.get("k1") == 42
            cache.put("k2", "v2")
            assert cache.get("k2") == "v2"

    def test_get_with_default_on_miss(self):
        """get(key, default) returns default when key is missing."""
        with tempfile.TemporaryDirectory(prefix="two_tier_") as tmp:
            cache = TwoTierCache(memory_size=100, disk_size=200, disk_cache_dir=tmp)
            assert cache.get("missing", None) is None
            assert cache.get("missing", 99) == 99
            assert cache.get("missing", "default") == "default"

    def test_put_then_get_consistent_with_set(self):
        """put() and set() are equivalent; get after put matches set behavior."""
        with tempfile.TemporaryDirectory(prefix="two_tier_") as tmp:
            cache = TwoTierCache(memory_size=100, disk_size=200, disk_cache_dir=tmp)
            cache.put("a", 1)
            cache.set("b", 2)
            assert cache.get("a") == 1
            assert cache.get("b") == 2
