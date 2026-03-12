#exonware/xwsystem/src/exonware/xwsystem/plugins/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.3
Generation Date: September 04, 2025
XSystem Plugins Package
Provides plugin discovery, registration and management system
with support for entry points and dynamic loading.
"""

from .base import APluginManager, APlugin, APluginRegistry
# Convenience aliases following DEV_GUIDELINES.md naming conventions
PluginManager = APluginManager
PluginBase = APlugin
PluginRegistry = APluginRegistry
__all__ = [
    "APluginManager",
    "APlugin", 
    "APluginRegistry",
    "PluginManager",
    "PluginBase", 
    "PluginRegistry",
]
