#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/utils/utils_contracts.py
#exonware/xwsystem/utils/types.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.5
Generation Date: 07-Sep-2025
Utils types and enums for XWSystem.
"""

from enum import Enum
from ..shared.defs import PathType
# ============================================================================
# UTILS ENUMS
# ============================================================================


class LazyLoadStrategy(Enum):
    """Lazy loading strategies."""
    ON_DEMAND = "on_demand"
    CACHED = "cached"
    PRELOAD = "preload"
    BACKGROUND = "background"


class LazyLoadMode(Enum):
    """Lazy loading modes."""
    EAGER = "eager"
    LAZY = "lazy"
    ON_DEMAND = "on_demand"
    CACHED = "cached"
    PRELOAD = "preload"
    BACKGROUND = "background"


class UtilityType(Enum):
    """Utility types."""
    PATH = "path"
    CONFIG = "config"
    RESOURCE = "resource"
    CACHE = "cache"
    LOGGING = "logging"
    VALIDATION = "validation"
    SERIALIZATION = "serialization"
    ENCRYPTION = "encryption"
    COMPRESSION = "compression"
    CUSTOM = "custom"


class ResourceType(Enum):
    """Resource types."""
    FILE = "file"
    MEMORY = "memory"
    NETWORK = "network"
    DATABASE = "database"
    CACHE = "cache"
    THREAD = "thread"
    PROCESS = "process"
    CONNECTION = "connection"
    CUSTOM = "custom"
