#!/usr/bin/env python3
#exonware/xwsystem/tests/0.core/validation/test_schema_validator_discovery.py
"""
Tests for schema validator discovery via entry points.
"""

import importlib.metadata
import threading
from typing import Any


def test_schema_validator_discovery_via_entry_points(monkeypatch):
    from exonware.xwsystem.validation.schema_discovery import (
        DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
        discover_schema_validators,
        get_schema_validator,
    )

    class DummySchemaValidator:
        def validate_schema(self, data: Any, schema: dict[str, Any]) -> tuple[bool, list[str]]:
            return True, []

        def create_schema(self, data: Any) -> dict[str, Any]:
            return {"type": "string"}

        def validate_type(self, data: Any, expected_type: str) -> bool:
            return True

        def validate_range(self, data: Any, min_value: Any, max_value: Any) -> bool:
            return True

        def validate_pattern(self, data: str, pattern: str) -> bool:
            return True

    class EP:
        def __init__(self, name: str, obj: Any):
            self.name = name
            self._obj = obj

        def load(self) -> Any:
            return self._obj

    def fake_entry_points(*, group: str):
        assert group == DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP
        return [
            EP("dummy_factory", lambda: DummySchemaValidator()),
        ]

    monkeypatch.setattr(importlib.metadata, "entry_points", fake_entry_points)

    providers, result = discover_schema_validators(force=True)
    assert "dummy_factory" in providers
    assert result.errors == {}

    v = get_schema_validator(name="dummy_factory")
    assert v is not None
    ok, errors = v.validate_schema({"a": 1}, {"type": "object"})
    assert ok is True
    assert errors == []


def test_schema_validator_discovery_ignores_invalid_provider(monkeypatch):
    from exonware.xwsystem.validation.schema_discovery import (
        DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
        discover_schema_validators,
    )

    class EP:
        def __init__(self, name: str, obj: Any):
            self.name = name
            self._obj = obj

        def load(self) -> Any:
            return self._obj

    def fake_entry_points(*, group: str):
        assert group == DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP
        return [
            EP("bad", object()),  # not instance/class/factory for ISchemaProvider
        ]

    monkeypatch.setattr(importlib.metadata, "entry_points", fake_entry_points)

    providers, result = discover_schema_validators(force=True)
    assert providers == {}
    assert "bad" in result.errors


def test_schema_validator_discovery_caches_without_force(monkeypatch):
    """
    Ensure discovery is cached per-group when force=False.
    """
    from exonware.xwsystem.validation.schema_discovery import discover_schema_validators

    calls = {"count": 0}

    class DummySchemaValidator:
        def validate_schema(self, data: Any, schema: dict[str, Any]) -> tuple[bool, list[str]]:
            return True, []

        def create_schema(self, data: Any) -> dict[str, Any]:
            return {"type": "string"}

        def validate_type(self, data: Any, expected_type: str) -> bool:
            return True

        def validate_range(self, data: Any, min_value: Any, max_value: Any) -> bool:
            return True

        def validate_pattern(self, data: str, pattern: str) -> bool:
            return True

    class EP:
        def __init__(self, name: str, obj: Any):
            self.name = name
            self._obj = obj

        def load(self) -> Any:
            return self._obj

    def fake_entry_points(*, group: str):
        calls["count"] += 1
        return [EP("dummy_factory", lambda: DummySchemaValidator())]

    monkeypatch.setattr(importlib.metadata, "entry_points", fake_entry_points)

    group = "xwsystem.schema_validators.test_cache"
    p1, _ = discover_schema_validators(group=group, force=False)
    p2, _ = discover_schema_validators(group=group, force=False)
    assert "dummy_factory" in p1
    assert "dummy_factory" in p2
    assert calls["count"] == 1


def test_schema_validator_discovery_force_reloads(monkeypatch):
    """
    Ensure force=True re-runs discovery even if already attempted.
    """
    from exonware.xwsystem.validation.schema_discovery import discover_schema_validators

    calls = {"count": 0}

    class DummySchemaValidator:
        def validate_schema(self, data: Any, schema: dict[str, Any]) -> tuple[bool, list[str]]:
            return True, []

        def create_schema(self, data: Any) -> dict[str, Any]:
            return {"type": "string"}

        def validate_type(self, data: Any, expected_type: str) -> bool:
            return True

        def validate_range(self, data: Any, min_value: Any, max_value: Any) -> bool:
            return True

        def validate_pattern(self, data: str, pattern: str) -> bool:
            return True

    class EP:
        def __init__(self, name: str, obj: Any):
            self.name = name
            self._obj = obj

        def load(self) -> Any:
            return self._obj

    def fake_entry_points(*, group: str):
        calls["count"] += 1
        return [EP("dummy_factory", lambda: DummySchemaValidator())]

    monkeypatch.setattr(importlib.metadata, "entry_points", fake_entry_points)

    group = "xwsystem.schema_validators.test_force"
    discover_schema_validators(group=group, force=True)
    discover_schema_validators(group=group, force=True)
    assert calls["count"] == 2


def test_schema_validator_discovery_thread_stress(monkeypatch):
    """
    Stress-ish test: concurrent calls should not crash or raise.
    """
    from exonware.xwsystem.validation.schema_discovery import get_schema_validator

    class DummySchemaValidator:
        def validate_schema(self, data: Any, schema: dict[str, Any]) -> tuple[bool, list[str]]:
            return True, []

        def create_schema(self, data: Any) -> dict[str, Any]:
            return {"type": "string"}

        def validate_type(self, data: Any, expected_type: str) -> bool:
            return True

        def validate_range(self, data: Any, min_value: Any, max_value: Any) -> bool:
            return True

        def validate_pattern(self, data: str, pattern: str) -> bool:
            return True

    class EP:
        def __init__(self, name: str, obj: Any):
            self.name = name
            self._obj = obj

        def load(self) -> Any:
            return self._obj

    def fake_entry_points(*, group: str):
        return [EP("dummy_factory", lambda: DummySchemaValidator())]

    monkeypatch.setattr(importlib.metadata, "entry_points", fake_entry_points)

    group = "xwsystem.schema_validators.test_threads"

    errors: list[str] = []

    def worker():
        try:
            for _ in range(200):
                v = get_schema_validator(group=group)
                assert v is not None
                ok, errs = v.validate_schema({"a": 1}, {"type": "object"})
                assert ok is True
                assert errs == []
        except Exception as e:
            errors.append(str(e))

    threads = [threading.Thread(target=worker) for _ in range(20)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()

    assert errors == []
