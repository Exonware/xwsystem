"""
Shared registration helpers for codec adapter auto-registration.
"""

from typing import Any

from collections.abc import Callable

from .registry import get_registry


def register_adapter_instance(
    adapter: Any,
    base_adapter_cls: type,
    *,
    registry=None,
    class_name: str | None = None,
):
    """
    Register an adapter instance via a generated adapter subclass.

    This preserves current registry semantics while removing duplicated
    dynamic class-generation logic from feature libraries.
    """
    if registry is None:
        registry = get_registry()

    generated_name = class_name or f"{adapter.codec_id.upper()}CodecAdapter"

    # Capture the concrete adapter in default args to avoid late binding.
    adapter_class = type(
        generated_name,
        (base_adapter_cls,),
        {"__init__": lambda self, a=adapter: base_adapter_cls.__init__(self, *(
            getattr(a, "_parser", None) if hasattr(a, "_parser") else getattr(a, "_handler", None),
            getattr(a, "_codec_id", getattr(a, "codec_id", None)),
            getattr(a, "_file_extensions", getattr(a, "file_extensions", [])),
            getattr(a, "_media_types", getattr(a, "media_types", [])),
            getattr(a, "_aliases", getattr(a, "aliases", [])),
            getattr(a, "_generator", None),
        )) if hasattr(a, "_parser") else base_adapter_cls.__init__(self, getattr(a, "_handler"))},
    )
    registry.register(adapter_class, adapter)
    return adapter_class


def auto_register_from_factories(
    factories: list[Callable[[], Any]],
    base_adapter_cls: type,
    *,
    registry=None,
    class_name_builder: Callable[[Any], str] | None = None,
) -> int:
    """Register all adapters produced by factories."""
    if registry is None:
        registry = get_registry()

    registered_count = 0
    for factory in factories:
        adapter = factory()
        class_name = class_name_builder(adapter) if class_name_builder else None
        register_adapter_instance(
            adapter,
            base_adapter_cls,
            registry=registry,
            class_name=class_name,
        )
        registered_count += 1
    return registered_count


__all__ = ["register_adapter_instance", "auto_register_from_factories"]

