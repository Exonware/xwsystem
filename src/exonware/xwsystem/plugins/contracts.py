#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/plugins/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.5
Generation Date: September 04, 2025
Plugin protocol interfaces for XWSystem.
"""

from typing import Any, Optional, Iterator, Callable, Protocol, runtime_checkable
import importlib
# Import enums from types module
from .defs import (
    PluginState,
    PluginType,
    PluginPriority,
    HookType,
    PluginEvent
)
# ============================================================================
# PLUGIN INTERFACES
# ============================================================================
@runtime_checkable

class IPlugin(Protocol):
    """
    Interface for plugins.
    Enforces consistent plugin behavior across XWSystem.
    """

    def initialize(self) -> None:
        """
        Initialize the plugin.
        """
        ...

    def shutdown(self) -> None:
        """
        Shutdown the plugin.
        """
        ...

    def get_info(self) -> dict[str, Any]:
        """
        Get plugin information.
        Returns:
            Plugin information dictionary
        """
        ...

    def is_enabled(self) -> bool:
        """
        Check if plugin is enabled.
        Returns:
            True if enabled
        """
        ...

    def enable(self) -> None:
        """
        Enable the plugin.
        """
        ...

    def disable(self) -> None:
        """
        Disable the plugin.
        """
        ...

    def get_state(self) -> PluginState:
        """
        Get plugin state.
        Returns:
            Current plugin state
        """
        ...

    def get_plugin_type(self) -> PluginType:
        """
        Get plugin type.
        Returns:
            Plugin type
        """
        ...

    def get_priority(self) -> PluginPriority:
        """
        Get plugin priority.
        Returns:
            Plugin priority
        """
        ...

    def get_dependencies(self) -> list[str]:
        """
        Get plugin dependencies.
        Returns:
            List of dependency names
        """
        ...

    def get_version(self) -> str:
        """
        Get plugin version.
        Returns:
            Plugin version string
        """
        ...

    def get_author(self) -> str:
        """
        Get plugin author.
        Returns:
            Plugin author
        """
        ...

    def get_description(self) -> str:
        """
        Get plugin description.
        Returns:
            Plugin description
        """
        ...
# ============================================================================
# EXTENSIBLE INTERFACES
# ============================================================================
@runtime_checkable

class IExtensible(Protocol):
    """
    Interface for extensible objects.
    Enforces consistent extension behavior across XWSystem.
    """

    def add_extension(self, extension: Any) -> bool:
        """
        Add extension.
        Args:
            extension: Extension to add
        Returns:
            True if added successfully
        """
        ...

    def remove_extension(self, name: str) -> bool:
        """
        Remove extension by name.
        Args:
            name: Extension name
        Returns:
            True if removed
        """
        ...

    def get_extensions(self) -> list[Any]:
        """
        Get all extensions.
        Returns:
            List of extensions
        """
        ...

    def get_extension(self, name: str) -> Optional[Any]:
        """
        Get extension by name.
        Args:
            name: Extension name
        Returns:
            Extension or None
        """
        ...

    def has_extension(self, name: str) -> bool:
        """
        Check if extension exists.
        Args:
            name: Extension name
        Returns:
            True if exists
        """
        ...

    def list_extension_names(self) -> list[str]:
        """
        List extension names.
        Returns:
            List of extension names
        """
        ...

    def clear_extensions(self) -> None:
        """
        Clear all extensions.
        """
        ...

    def get_extension_count(self) -> int:
        """
        Get number of extensions.
        Returns:
            Number of extensions
        """
        ...
# ============================================================================
# HOOKABLE INTERFACES
# ============================================================================
@runtime_checkable

class IHookable(Protocol):
    """
    Interface for hookable objects.
    Enforces consistent hook behavior across XWSystem.
    """

    def add_hook(self, event: str, callback: Callable, hook_type: HookType = HookType.ACTION) -> str:
        """
        Add hook callback.
        Args:
            event: Event name
            callback: Callback function
            hook_type: Type of hook
        Returns:
            Hook ID
        """
        ...

    def remove_hook(self, event: str, callback: Callable) -> bool:
        """
        Remove hook callback.
        Args:
            event: Event name
            callback: Callback function
        Returns:
            True if removed
        """
        ...

    def trigger_hook(self, event: str, data: Any = None) -> Any:
        """
        Trigger hook event.
        Args:
            event: Event name
            data: Event data
        Returns:
            Hook result
        """
        ...

    def list_hooks(self, event: Optional[str] = None) -> list[dict[str, Any]]:
        """
        List hooks.
        Args:
            event: Filter by event name
        Returns:
            List of hook information
        """
        ...

    def has_hooks(self, event: str) -> bool:
        """
        Check if event has hooks.
        Args:
            event: Event name
        Returns:
            True if has hooks
        """
        ...

    def clear_hooks(self, event: Optional[str] = None) -> None:
        """
        Clear hooks.
        Args:
            event: Clear hooks for specific event, or all if None
        """
        ...

    def get_hook_count(self, event: Optional[str] = None) -> int:
        """
        Get hook count.
        Args:
            event: Count hooks for specific event, or all if None
        Returns:
            Number of hooks
        """
        ...
# ============================================================================
# PLUGIN MANAGER INTERFACES
# ============================================================================
@runtime_checkable

class IPluginManager(Protocol):
    """
    Interface for plugin management.
    Enforces consistent plugin management across XWSystem.
    """

    def load_plugin(self, plugin_path: str) -> bool:
        """
        Load plugin from path.
        Args:
            plugin_path: Plugin path or module name
        Returns:
            True if loaded successfully
        """
        ...

    def unload_plugin(self, plugin_name: str) -> bool:
        """
        Unload plugin.
        Args:
            plugin_name: Plugin name
        Returns:
            True if unloaded successfully
        """
        ...

    def reload_plugin(self, plugin_name: str) -> bool:
        """
        Reload plugin.
        Args:
            plugin_name: Plugin name
        Returns:
            True if reloaded successfully
        """
        ...

    def get_plugin(self, plugin_name: str) -> Optional[IPlugin]:
        """
        Get plugin by name.
        Args:
            plugin_name: Plugin name
        Returns:
            Plugin instance or None
        """
        ...

    def list_plugins(self) -> list[str]:
        """
        List all plugin names.
        Returns:
            List of plugin names
        """
        ...

    def list_loaded_plugins(self) -> list[str]:
        """
        List loaded plugin names.
        Returns:
            List of loaded plugin names
        """
        ...

    def is_plugin_loaded(self, plugin_name: str) -> bool:
        """
        Check if plugin is loaded.
        Args:
            plugin_name: Plugin name
        Returns:
            True if loaded
        """
        ...

    def get_plugin_info(self, plugin_name: str) -> Optional[dict[str, Any]]:
        """
        Get plugin information.
        Args:
            plugin_name: Plugin name
        Returns:
            Plugin information or None
        """
        ...
# ============================================================================
# PLUGIN REGISTRY INTERFACES
# ============================================================================
@runtime_checkable

class IPluginRegistry(Protocol):
    """
    Interface for plugin registry.
    Enforces consistent plugin registration across XWSystem.
    """

    def register_plugin(self, plugin_class: type[IPlugin], name: str, priority: PluginPriority = PluginPriority.NORMAL) -> bool:
        """
        Register plugin class.
        Args:
            plugin_class: Plugin class
            name: Plugin name
            priority: Plugin priority
        Returns:
            True if registered successfully
        """
        ...

    def unregister_plugin(self, name: str) -> bool:
        """
        Unregister plugin.
        Args:
            name: Plugin name
        Returns:
            True if unregistered
        """
        ...

    def get_registered_plugins(self) -> dict[str, type[IPlugin]]:
        """
        Get all registered plugins.
        Returns:
            Dictionary of registered plugins
        """
        ...

    def is_plugin_registered(self, name: str) -> bool:
        """
        Check if plugin is registered.
        Args:
            name: Plugin name
        Returns:
            True if registered
        """
        ...

    def get_plugin_class(self, name: str) -> Optional[type[IPlugin]]:
        """
        Get plugin class by name.
        Args:
            name: Plugin name
        Returns:
            Plugin class or None
        """
        ...

    def clear_registry(self) -> None:
        """
        Clear plugin registry.
        """
        ...

    def get_registry_stats(self) -> dict[str, Any]:
        """
        Get registry statistics.
        Returns:
            Registry statistics dictionary
        """
        ...
# ============================================================================
# PLUGIN DISCOVERY INTERFACES
# ============================================================================
@runtime_checkable

class IPluginDiscovery(Protocol):
    """
    Interface for plugin discovery.
    Enforces consistent plugin discovery across XWSystem.
    """

    def discover_plugins(self, search_paths: list[str]) -> list[str]:
        """
        Discover plugins in search paths.
        Args:
            search_paths: Paths to search for plugins
        Returns:
            List of discovered plugin paths
        """
        ...

    def scan_directory(self, directory: str) -> list[str]:
        """
        Scan directory for plugins.
        Args:
            directory: Directory to scan
        Returns:
            List of plugin files found
        """
        ...

    def validate_plugin(self, plugin_path: str) -> tuple[bool, list[str]]:
        """
        Validate plugin.
        Args:
            plugin_path: Plugin path to validate
        Returns:
            Tuple of (is_valid, error_messages)
        """
        ...

    def get_plugin_metadata(self, plugin_path: str) -> Optional[dict[str, Any]]:
        """
        Get plugin metadata.
        Args:
            plugin_path: Plugin path
        Returns:
            Plugin metadata or None
        """
        ...

    def is_plugin_file(self, file_path: str) -> bool:
        """
        Check if file is a plugin.
        Args:
            file_path: File path to check
        Returns:
            True if file is a plugin
        """
        ...

    def get_supported_extensions(self) -> list[str]:
        """
        Get supported plugin file extensions.
        Returns:
            List of supported extensions
        """
        ...
# ============================================================================
# PLUGIN CONFIGURATION INTERFACES
# ============================================================================
@runtime_checkable

class IPluginConfig(Protocol):
    """
    Interface for plugin configuration.
    Enforces consistent plugin configuration across XWSystem.
    """

    def get_plugin_config(self, plugin_name: str) -> dict[str, Any]:
        """
        Get plugin configuration.
        Args:
            plugin_name: Plugin name
        Returns:
            Plugin configuration dictionary
        """
        ...

    def set_plugin_config(self, plugin_name: str, config: dict[str, Any]) -> None:
        """
        Set plugin configuration.
        Args:
            plugin_name: Plugin name
            config: Configuration dictionary
        """
        ...

    def update_plugin_config(self, plugin_name: str, key: str, value: Any) -> None:
        """
        Update plugin configuration value.
        Args:
            plugin_name: Plugin name
            key: Configuration key
            value: Configuration value
        """
        ...

    def get_plugin_config_value(self, plugin_name: str, key: str, default: Any = None) -> Any:
        """
        Get plugin configuration value.
        Args:
            plugin_name: Plugin name
            key: Configuration key
            default: Default value
        Returns:
            Configuration value or default
        """
        ...

    def has_plugin_config(self, plugin_name: str, key: str) -> bool:
        """
        Check if plugin has configuration key.
        Args:
            plugin_name: Plugin name
            key: Configuration key
        Returns:
            True if key exists
        """
        ...

    def remove_plugin_config(self, plugin_name: str, key: str) -> bool:
        """
        Remove plugin configuration key.
        Args:
            plugin_name: Plugin name
            key: Configuration key
        Returns:
            True if removed
        """
        ...

    def clear_plugin_config(self, plugin_name: str) -> None:
        """
        Clear plugin configuration.
        Args:
            plugin_name: Plugin name
        """
        ...

    def save_plugin_config(self, plugin_name: str, file_path: str) -> bool:
        """
        Save plugin configuration to file.
        Args:
            plugin_name: Plugin name
            file_path: File path to save to
        Returns:
            True if saved successfully
        """
        ...

    def load_plugin_config(self, plugin_name: str, file_path: str) -> bool:
        """
        Load plugin configuration from file.
        Args:
            plugin_name: Plugin name
            file_path: File path to load from
        Returns:
            True if loaded successfully
        """
        ...
# ============================================================================
# PLUGIN EVENTS INTERFACES
# ============================================================================
@runtime_checkable

class IPluginEvents(Protocol):
    """
    Interface for plugin events.
    Enforces consistent plugin event handling across XWSystem.
    """

    def emit_event(self, event: PluginEvent, plugin_name: str, data: Any = None) -> None:
        """
        Emit plugin event.
        Args:
            event: Plugin event
            plugin_name: Plugin name
            data: Event data
        """
        ...

    def subscribe_to_event(self, event: PluginEvent, callback: Callable) -> str:
        """
        Subscribe to plugin event.
        Args:
            event: Plugin event
            callback: Event callback
        Returns:
            Subscription ID
        """
        ...

    def unsubscribe_from_event(self, subscription_id: str) -> bool:
        """
        Unsubscribe from plugin event.
        Args:
            subscription_id: Subscription ID
        Returns:
            True if unsubscribed
        """
        ...

    def get_event_subscribers(self, event: PluginEvent) -> list[Callable]:
        """
        Get event subscribers.
        Args:
            event: Plugin event
        Returns:
            List of subscriber callbacks
        """
        ...

    def clear_event_subscribers(self, event: Optional[PluginEvent] = None) -> None:
        """
        Clear event subscribers.
        Args:
            event: Clear subscribers for specific event, or all if None
        """
        ...

    def get_event_history(self, event: Optional[PluginEvent] = None, limit: int = 100) -> list[dict[str, Any]]:
        """
        Get event history.
        Args:
            event: Filter by event type
            limit: Maximum number of events
        Returns:
            List of event history entries
        """
        ...

    def clear_event_history(self) -> None:
        """
        Clear event history.
        """
        ...
# ============================================================================
# PLUGIN DEPENDENCY INTERFACES
# ============================================================================
@runtime_checkable

class IPluginDependency(Protocol):
    """
    Interface for plugin dependency management.
    Enforces consistent plugin dependency handling across XWSystem.
    """

    def add_dependency(self, plugin_name: str, dependency: str, version: Optional[str] = None) -> None:
        """
        Add plugin dependency.
        Args:
            plugin_name: Plugin name
            dependency: Dependency name
            version: Required version
        """
        ...

    def remove_dependency(self, plugin_name: str, dependency: str) -> bool:
        """
        Remove plugin dependency.
        Args:
            plugin_name: Plugin name
            dependency: Dependency name
        Returns:
            True if removed
        """
        ...

    def get_dependencies(self, plugin_name: str) -> list[dict[str, Any]]:
        """
        Get plugin dependencies.
        Args:
            plugin_name: Plugin name
        Returns:
            List of dependency information
        """
        ...

    def check_dependencies(self, plugin_name: str) -> tuple[bool, list[str]]:
        """
        Check if plugin dependencies are satisfied.
        Args:
            plugin_name: Plugin name
        Returns:
            Tuple of (all_satisfied, missing_dependencies)
        """
        ...

    def resolve_dependencies(self, plugin_name: str) -> list[str]:
        """
        Resolve plugin dependency order.
        Args:
            plugin_name: Plugin name
        Returns:
            List of plugins in dependency order
        """
        ...

    def get_dependents(self, plugin_name: str) -> list[str]:
        """
        Get plugins that depend on this plugin.
        Args:
            plugin_name: Plugin name
        Returns:
            List of dependent plugin names
        """
        ...

    def has_circular_dependency(self, plugin_name: str) -> bool:
        """
        Check for circular dependencies.
        Args:
            plugin_name: Plugin name
        Returns:
            True if circular dependency exists
        """
        ...

    def get_dependency_graph(self) -> dict[str, list[str]]:
        """
        Get plugin dependency graph.
        Returns:
            Dependency graph dictionary
        """
        ...
