#exonware/xwsystem/src/exonware/xwsystem/utils/base.py
#exonware/xwsystem/utils/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.32
Generation Date: September 04, 2025
Utils module base classes - abstract classes for utility functionality.
"""

from abc import ABC, abstractmethod
from collections.abc import Callable
from typing import Any
# Root cause: Migrating to Python 3.12 built-in generic syntax for consistency
# Priority #3: Maintainability - Modern type annotations improve code clarity
from pathlib import Path
from .contracts import LazyLoadMode, PathType, UtilityType, ResourceType


class ALazyLoaderBase[T](ABC):
    """Abstract base class for lazy loading operations."""

    def __init__(self, load_mode: LazyLoadMode = LazyLoadMode.ON_DEMAND):
        """
        Initialize lazy loader.
        Args:
            load_mode: Lazy loading mode
        """
        self.load_mode = load_mode
        self._loaded = False
        self._loading = False
        self._object: T | None = None
        self._load_function: Callable[[], T] | None = None
    @abstractmethod

    def set_load_function(self, load_func: Callable[[], T]) -> None:
        """Set function to load object."""
        pass
    @abstractmethod

    def load(self) -> T:
        """Load object."""
        pass
    @abstractmethod

    def unload(self) -> None:
        """Unload object."""
        pass
    @abstractmethod

    def reload(self) -> T:
        """Reload object."""
        pass
    @abstractmethod

    def is_loaded(self) -> bool:
        """Check if object is loaded."""
        pass
    @abstractmethod

    def is_loading(self) -> bool:
        """Check if object is currently loading."""
        pass
    @abstractmethod

    def get_object(self) -> T | None:
        """Get loaded object."""
        pass
    @abstractmethod

    def preload(self) -> None:
        """Preload object."""
        pass
    @abstractmethod

    def get_load_time(self) -> float | None:
        """Get object load time."""
        pass
    @abstractmethod

    def get_memory_usage(self) -> int:
        """Get object memory usage."""
        pass


class APathUtilsBase(ABC):
    """Abstract base class for path utility operations."""

    def __init__(self):
        """Initialize path utils."""
        self._path_cache: dict[str, Path] = {}
        self._normalized_paths: dict[str, str] = {}
    @abstractmethod

    def normalize_path(self, path: str | Path) -> Path:
        """Normalize file path."""
        pass
    @abstractmethod

    def resolve_path(self, path: str | Path) -> Path:
        """Resolve file path."""
        pass
    @abstractmethod

    def absolute_path(self, path: str | Path) -> Path:
        """Get absolute path."""
        pass
    @abstractmethod

    def relative_path(self, path: str | Path, start: str | Path | None = None) -> Path:
        """Get relative path."""
        pass
    @abstractmethod

    def join_paths(self, *paths: str | Path) -> Path:
        """Join multiple paths."""
        pass
    @abstractmethod

    def split_path(self, path: str | Path) -> tuple[Path, str]:
        """Split path into directory and filename."""
        pass
    @abstractmethod

    def get_extension(self, path: str | Path) -> str:
        """Get file extension."""
        pass
    @abstractmethod

    def get_stem(self, path: str | Path) -> str:
        """Get file stem."""
        pass
    @abstractmethod

    def get_name(self, path: str | Path) -> str:
        """Get file/directory name."""
        pass
    @abstractmethod

    def get_parent(self, path: str | Path) -> Path:
        """Get parent directory."""
        pass
    @abstractmethod

    def is_absolute(self, path: str | Path) -> bool:
        """Check if path is absolute."""
        pass
    @abstractmethod

    def is_relative(self, path: str | Path) -> bool:
        """Check if path is relative."""
        pass
    @abstractmethod

    def exists(self, path: str | Path) -> bool:
        """Check if path exists."""
        pass
    @abstractmethod

    def is_file(self, path: str | Path) -> bool:
        """Check if path is file."""
        pass
    @abstractmethod

    def is_directory(self, path: str | Path) -> bool:
        """Check if path is directory."""
        pass
    @abstractmethod

    def get_size(self, path: str | Path) -> int:
        """Get path size."""
        pass
    @abstractmethod

    def get_modified_time(self, path: str | Path) -> float:
        """Get path modification time."""
        pass
    @abstractmethod

    def sanitize_path(self, path: str | Path) -> str:
        """Sanitize path for security."""
        pass


class AUtilityRegistryBase(ABC):
    """Abstract base class for utility registry operations."""

    def __init__(self):
        """Initialize utility registry."""
        self._utilities: dict[str, Any] = {}
        self._utility_types: dict[str, UtilityType] = {}
        self._utility_metadata: dict[str, dict[str, Any] | None] = {}
    @abstractmethod

    def register_utility(
        self,
        name: str,
        utility: Any,
        utility_type: UtilityType,
        metadata: dict[str, Any] | None = None,
    ) -> None:
        """Register utility."""
        pass
    @abstractmethod

    def unregister_utility(self, name: str) -> bool:
        """Unregister utility."""
        pass
    @abstractmethod

    def get_utility(self, name: str) -> Any | None:
        """Get utility by name."""
        pass
    @abstractmethod

    def list_utilities(self, utility_type: UtilityType | None = None) -> list[str]:
        """List utilities."""
        pass
    @abstractmethod

    def has_utility(self, name: str) -> bool:
        """Check if utility exists."""
        pass
    @abstractmethod

    def get_utility_type(self, name: str) -> UtilityType | None:
        """Get utility type."""
        pass
    @abstractmethod

    def get_utility_metadata(self, name: str) -> dict[str, Any] | None:
        """Get utility metadata."""
        pass
    @abstractmethod

    def update_utility_metadata(self, name: str, metadata: dict[str, Any]) -> None:
        """Update utility metadata."""
        pass
    @abstractmethod

    def clear_utilities(self) -> None:
        """Clear all utilities."""
        pass
    @abstractmethod

    def get_utility_count(self) -> int:
        """Get utility count."""
        pass
    @abstractmethod

    def export_utilities(self) -> dict[str, Any]:
        """Export utilities registry."""
        pass
    @abstractmethod

    def import_utilities(self, utilities_data: dict[str, Any]) -> None:
        """Import utilities registry."""
        pass


class AConfigManagerBase(ABC):
    """Abstract base class for configuration management."""

    def __init__(self):
        """Initialize config manager."""
        self._configs: dict[str, dict[str, Any]] = {}
        self._config_schemas: dict[str, dict[str, Any]] = {}
        self._config_validators: dict[str, Callable] = {}
    @abstractmethod

    def load_config(self, config_name: str, config_data: dict[str, Any]) -> None:
        """Load configuration."""
        pass
    @abstractmethod

    def save_config(self, config_name: str, file_path: str | Path) -> None:
        """Save configuration to file."""
        pass
    @abstractmethod

    def get_config(self, config_name: str) -> dict[str, Any] | None:
        """Get configuration."""
        pass
    @abstractmethod

    def set_config_value(self, config_name: str, key: str, value: Any) -> None:
        """Set configuration value."""
        pass
    @abstractmethod

    def get_config_value(self, config_name: str, key: str, default: Any = None) -> Any:
        """Get configuration value."""
        pass
    @abstractmethod

    def has_config(self, config_name: str) -> bool:
        """Check if configuration exists."""
        pass
    @abstractmethod

    def remove_config(self, config_name: str) -> bool:
        """Remove configuration."""
        pass
    @abstractmethod

    def list_configs(self) -> list[str]:
        """List all configurations."""
        pass
    @abstractmethod

    def validate_config(self, config_name: str) -> bool:
        """Validate configuration."""
        pass
    @abstractmethod

    def set_config_schema(self, config_name: str, schema: dict[str, Any]) -> None:
        """Set configuration schema."""
        pass
    @abstractmethod

    def get_config_schema(self, config_name: str) -> dict[str, Any] | None:
        """Get configuration schema."""
        pass
    @abstractmethod

    def set_config_validator(self, config_name: str, validator: Callable) -> None:
        """Set configuration validator."""
        pass
    @abstractmethod

    def get_config_validator(self, config_name: str) -> Callable | None:
        """Get configuration validator."""
        pass


class AResourceManagerBase(ABC):
    """Abstract base class for resource management."""

    def __init__(self):
        """Initialize resource manager."""
        self._resources: dict[str, Any] = {}
        self._resource_types: dict[str, ResourceType] = {}
        self._resource_locks: dict[str, bool] = {}
        self._resource_usage: dict[str, dict[str, Any]] = {}
    @abstractmethod

    def acquire_resource(
        self,
        resource_id: str,
        resource_type: ResourceType,
        **kwargs,
    ) -> Any | None:
        """Acquire resource."""
        pass
    @abstractmethod

    def release_resource(self, resource_id: str) -> None:
        """Release resource."""
        pass
    @abstractmethod

    def get_resource(self, resource_id: str) -> Any | None:
        """Get resource by ID."""
        pass
    @abstractmethod

    def has_resource(self, resource_id: str) -> bool:
        """Check if resource exists."""
        pass
    @abstractmethod

    def list_resources(self, resource_type: ResourceType | None = None) -> list[str]:
        """List resources."""
        pass
    @abstractmethod

    def get_resource_type(self, resource_id: str) -> ResourceType | None:
        """Get resource type."""
        pass
    @abstractmethod

    def is_resource_locked(self, resource_id: str) -> bool:
        """Check if resource is locked."""
        pass
    @abstractmethod

    def lock_resource(self, resource_id: str) -> bool:
        """Lock resource."""
        pass
    @abstractmethod

    def unlock_resource(self, resource_id: str) -> None:
        """Unlock resource."""
        pass
    @abstractmethod

    def get_resource_usage(self, resource_id: str) -> dict[str, Any] | None:
        """Get resource usage statistics."""
        pass
    @abstractmethod

    def cleanup_resources(self) -> int:
        """Cleanup unused resources."""
        pass
    @abstractmethod

    def get_resource_count(self) -> int:
        """Get resource count."""
        pass
    @abstractmethod

    def get_resource_stats(self) -> dict[str, Any]:
        """Get resource statistics."""
        pass


class BaseUtils:
    """Base implementation of utility functions."""

    def __init__(self):
        """Initialize base utils."""
        self._utilities: dict[str, Any] = {}
        self._initialized = False

    def initialize(self) -> None:
        """Initialize utilities."""
        self._initialized = True

    def is_initialized(self) -> bool:
        """Check if utilities are initialized."""
        return self._initialized

    def register_utility(self, name: str, utility: Any) -> None:
        """Register a utility."""
        self._utilities[name] = utility

    def get_utility(self, name: str) -> Any | None:
        """Get utility by name."""
        return self._utilities.get(name)

    def has_utility(self, name: str) -> bool:
        """Check if utility exists."""
        return name in self._utilities

    def list_utilities(self) -> list[str]:
        """List all utilities."""
        return list(self._utilities.keys())

    def remove_utility(self, name: str) -> bool:
        """Remove utility."""
        if name in self._utilities:
            del self._utilities[name]
            return True
        return False

    def clear_utilities(self) -> None:
        """Clear all utilities."""
        self._utilities.clear()

    def get_utility_count(self) -> int:
        """Get utility count."""
        return len(self._utilities)
