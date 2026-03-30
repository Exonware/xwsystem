#exonware/xwsystem/src/exonware/xwsystem/runtime/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.30
Generation Date: September 04, 2025
Runtime module contracts - interfaces and enums for runtime environment functionality.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import Callable
import sys
# Import enums from types module
from .defs import (
    EnvironmentType,
    PlatformType,
    PythonVersion,
    RuntimeMode
)
@runtime_checkable

class IEnvironmentManager(Protocol):
    """Interface for environment management."""

    def get_environment_type(self) -> EnvironmentType:
        """Get current environment type."""
        ...

    def set_environment_type(self, env_type: EnvironmentType) -> None:
        """Set environment type."""
        ...

    def get_environment_variable(self, name: str, default: str | None = None) -> str | None:
        """Get environment variable."""
        ...

    def set_environment_variable(self, name: str, value: str) -> None:
        """Set environment variable."""
        ...

    def get_all_environment_variables(self) -> dict[str, str]:
        """Get all environment variables."""
        ...
@runtime_checkable

class IPlatformInfo(Protocol):
    """Interface for platform information."""

    def get_platform_type(self) -> PlatformType:
        """Get platform type."""
        ...

    def get_platform_version(self) -> str:
        """Get platform version."""
        ...

    def get_architecture(self) -> str:
        """Get system architecture."""
        ...

    def get_hostname(self) -> str:
        """Get system hostname."""
        ...

    def get_username(self) -> str:
        """Get current username."""
        ...
@runtime_checkable

class IPythonInfo(Protocol):
    """Interface for Python information."""

    def get_python_version(self) -> PythonVersion:
        """Get Python version."""
        ...

    def get_python_executable(self) -> str:
        """Get Python executable path."""
        ...

    def get_python_path(self) -> list[str]:
        """Get Python path."""
        ...

    def get_installed_packages(self) -> dict[str, str]:
        """Get installed packages."""
        ...

    def is_package_installed(self, package_name: str) -> bool:
        """Check if package is installed."""
        ...
@runtime_checkable

class IReflectionUtils(Protocol):
    """Interface for reflection utilities."""

    def get_class_from_string(self, class_path: str) -> type:
        """Get class from string path."""
        ...

    def get_function_from_string(self, function_path: str) -> Callable:
        """Get function from string path."""
        ...

    def find_classes_in_module(self, module: Any, base_class: type) -> list[type]:
        """Find classes in module."""
        ...

    def get_class_hierarchy(self, cls: type) -> list[type]:
        """Get class hierarchy."""
        ...

    def get_class_attributes(self, cls: type) -> dict[str, Any]:
        """Get class attributes."""
        ...
@runtime_checkable

class IRuntimeConfig(Protocol):
    """Interface for runtime configuration."""

    def get_runtime_mode(self) -> RuntimeMode:
        """Get runtime mode."""
        ...

    def set_runtime_mode(self, mode: RuntimeMode) -> None:
        """Set runtime mode."""
        ...

    def get_config_value(self, key: str, default: Any | None = None) -> Any:
        """Get configuration value."""
        ...

    def set_config_value(self, key: str, value: Any) -> None:
        """Set configuration value."""
        ...

    def load_config_from_file(self, file_path: str) -> None:
        """Load configuration from file."""
        ...

    def save_config_to_file(self, file_path: str) -> None:
        """Save configuration to file."""
        ...
