#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/validation/schema_discovery.py
"""
Schema validator discovery (plugin entry points).
This module provides a lightweight "plugin" mechanism (via Python packaging
entry points) to discover an implementation of
`xwsystem.validation.contracts.ISchemaProvider`.
Why this exists:
- Avoids hard dependency cycles (e.g., xwdata <-> xwschema)
- Keeps xwsystem as the stable contract owner
- Enables optional providers to be installed and discovered at runtime
Entry point group:
  - xwsystem.schema_validators
Providers should register an entry point that resolves to either:
- an ISchemaProvider instance
- a class (constructible with no args) returning an ISchemaProvider instance
- a callable (factory) returning an ISchemaProvider instance
"""

from __future__ import annotations
from dataclasses import dataclass
import threading
from typing import Any, Optional
from ..config.logging_setup import get_logger
from .contracts import ISchemaProvider
logger = get_logger("xwsystem.validation.schema_discovery")
DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP = "xwsystem.schema_validators"
@dataclass(frozen=True)


class SchemaValidatorDiscoveryResult:
    """Structured discovery information for observability/debugging."""
    group: str
    loaded: dict[str, str]  # name -> provider repr
    errors: dict[str, str]  # name -> error string
_lock = threading.RLock()
_attempted_groups: set[str] = set()
_providers_by_group: dict[str, dict[str, ISchemaProvider]] = {}
_default_provider_by_group: dict[str, str] = {}


def _as_schema_provider(obj: Any) -> Optional[ISchemaProvider]:
    """Coerce an entry-point loaded object into an ISchemaProvider instance."""
    if obj is None:
        return None
    # 1) Direct instance
    try:
        if isinstance(obj, ISchemaProvider):
            return obj
    except Exception:
        # Protocol runtime checks can raise if obj is weird; treat as not valid.
        return None
    # 2) Class -> instantiate
    if isinstance(obj, type):
        try:
            inst = obj()
        except Exception:
            return None
        try:
            return inst if isinstance(inst, ISchemaProvider) else None
        except Exception:
            return None
    # 3) Factory callable -> call
    if callable(obj):
        try:
            inst = obj()
        except Exception:
            return None
        try:
            return inst if isinstance(inst, ISchemaProvider) else None
        except Exception:
            return None
    return None


def discover_schema_validators(
    *,
    group: str = DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
    force: bool = False,
) -> tuple[dict[str, ISchemaProvider], SchemaValidatorDiscoveryResult]:
    """
    Discover schema validator providers from entry points.
    Args:
        group: Entry-point group to search.
        force: If True, re-run discovery even if already attempted.
    Returns:
        (providers, discovery_result)
    """
    with _lock:
        already_attempted = group in _attempted_groups
        if already_attempted and not force:
            providers = dict(_providers_by_group.get(group, {}))
            loaded_repr = {k: repr(v) for k, v in providers.items()}
            return providers, SchemaValidatorDiscoveryResult(group=group, loaded=loaded_repr, errors={})
    loaded: dict[str, ISchemaProvider] = {}
    loaded_repr: dict[str, str] = {}
    errors: dict[str, str] = {}
    try:
        from importlib.metadata import entry_points
        eps = entry_points(group=group)
    except Exception as e:
        # Cache attempted to avoid repeated expensive failures.
        with _lock:
            _attempted_groups.add(group)
            _providers_by_group.setdefault(group, {})
        logger.debug(f"Schema validator discovery failed for group '{group}': {e}")
        return {}, SchemaValidatorDiscoveryResult(group=group, loaded={}, errors={"__discovery__": str(e)})
    for ep in eps:
        name = getattr(ep, "name", "<unknown>")
        try:
            obj = ep.load()
            inst = _as_schema_provider(obj)
            if inst is None:
                errors[name] = "Entry point did not resolve to an ISchemaProvider (instance/class/factory)."
                continue
            loaded[name] = inst
            loaded_repr[name] = repr(inst)
        except Exception as e:
            errors[name] = str(e)
    with _lock:
        _attempted_groups.add(group)
        _providers_by_group[group] = dict(loaded)
        # Preserve existing default selection if still valid; else choose first.
        if group not in _default_provider_by_group or _default_provider_by_group[group] not in loaded:
            if loaded:
                _default_provider_by_group[group] = sorted(loaded.keys())[0]
    if errors:
        logger.debug(f"Schema validator discovery had errors for group '{group}': {errors}")
    return dict(loaded), SchemaValidatorDiscoveryResult(group=group, loaded=loaded_repr, errors=errors)


def set_schema_validator(
    validator: ISchemaProvider,
    *,
    name: str = "manual",
    group: str = DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
    make_default: bool = True,
) -> None:
    """Manually register a schema provider (useful for tests / custom wiring)."""
    with _lock:
        _attempted_groups.add(group)
        _providers_by_group.setdefault(group, {})
        _providers_by_group[group][name] = validator
        if make_default:
            _default_provider_by_group[group] = name


def get_schema_validator(
    *,
    name: Optional[str] = None,
    group: str = DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
) -> Optional[ISchemaProvider]:
    """
    Get a schema provider (validator).
    Args:
        name: Optional provider name; if omitted, returns the group's default provider.
        group: Entry-point group.
    Returns:
        ISchemaProvider instance or None if no providers available.
    """
    providers, _ = discover_schema_validators(group=group, force=False)
    if not providers:
        return None
    if name:
        return providers.get(name)
    with _lock:
        default_name = _default_provider_by_group.get(group)
    if default_name and default_name in providers:
        return providers[default_name]
    # Fall back to deterministic choice
    return providers[sorted(providers.keys())[0]]


def available_schema_validators(
    *,
    group: str = DEFAULT_SCHEMA_VALIDATOR_ENTRYPOINT_GROUP,
) -> list[str]:
    """List discovered provider names."""
    providers, _ = discover_schema_validators(group=group, force=False)
    return sorted(providers.keys())
