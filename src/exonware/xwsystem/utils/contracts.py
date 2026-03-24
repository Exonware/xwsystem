#exonware/xwsystem/src/exonware/xwsystem/utils/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.18
Generation Date: September 04, 2025
Utils module contracts - interfaces and enums for utility functionality.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import Callable
# Root cause: Migrating to Python 3.12 built-in generic syntax for consistency
# Priority #3: Maintainability - Modern type annotations improve code clarity
from pathlib import Path
# Import enums from types module
from .defs import (
    LazyLoadStrategy,
    LazyLoadMode,
    UtilityType,
    ResourceType,
    PathType
)
@runtime_checkable

class ILazyLoader[T](Protocol):
    """Interface for lazy loading operations."""

    def load(self) -> T:
        """Load the object."""
        ...

    def is_loaded(self) -> bool:
        """Check if object is loaded."""
        ...

    def unload(self) -> None:
        """Unload the object."""
        ...

    def reload(self) -> T:
        """Reload the object."""
        ...

    def get_loader_function(self) -> Callable[[], T]:
        """Get the loader function."""
        ...
@runtime_checkable

class IPathUtils(Protocol):
    """Interface for path utility operations."""

    def normalize_path(self, path: str | Path) -> Path:
        """Normalize path."""
        ...

    def resolve_path(self, path: str | Path) -> Path:
        """Resolve path to absolute."""
        ...

    def get_path_type(self, path: str | Path) -> PathType:
        """Get path type."""
        ...

    def is_safe_path(self, path: str | Path) -> bool:
        """Check if path is safe."""
        ...

    def sanitize_path(self, path: str | Path) -> Path:
        """Sanitize path."""
        ...

    def get_relative_path(self, path: str | Path, base: str | Path) -> Path:
        """Get relative path."""
        ...

    def ensure_path_exists(self, path: str | Path) -> None:
        """Ensure path exists."""
        ...
@runtime_checkable

class IUtilityRegistry(Protocol):
    """Interface for utility registry operations."""

    def register_utility(self, name: str, utility: Any) -> None:
        """Register utility."""
        ...

    def get_utility(self, name: str) -> Any | None:
        """Get utility by name."""
        ...

    def unregister_utility(self, name: str) -> None:
        """Unregister utility."""
        ...

    def list_utilities(self) -> list[str]:
        """List all registered utilities."""
        ...

    def has_utility(self, name: str) -> bool:
        """Check if utility is registered."""
        ...
@runtime_checkable

class IConfigManager(Protocol):
    """Interface for configuration management."""

    def get_config(self, key: str, default: Any | None = None) -> Any:
        """Get configuration value."""
        ...

    def set_config(self, key: str, value: Any) -> None:
        """Set configuration value."""
        ...

    def load_config(self, source: str | Path | dict[str, Any]) -> None:
        """Load configuration from source."""
        ...

    def save_config(self, destination: str | Path) -> None:
        """Save configuration to destination."""
        ...

    def get_all_config(self) -> dict[str, Any]:
        """Get all configuration."""
        ...

    def clear_config(self) -> None:
        """Clear all configuration."""
        ...
@runtime_checkable

class IResourceManager(Protocol):
    """Interface for resource management."""

    def acquire_resource(self, resource_id: str) -> Any:
        """Acquire resource."""
        ...

    def release_resource(self, resource_id: str) -> None:
        """Release resource."""
        ...

    def is_resource_available(self, resource_id: str) -> bool:
        """Check if resource is available."""
        ...

    def get_resource_count(self) -> int:
        """Get total resource count."""
        ...

    def get_available_resources(self) -> list[str]:
        """Get list of available resources."""
        ...
