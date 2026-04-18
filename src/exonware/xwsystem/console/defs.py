#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/defs.py
#exonware/xwsystem/console/defs.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.42
Generation Date: 2025-01-27
Type definitions and constants for console module.
"""

from typing import Literal, Any
from dataclasses import dataclass
from enum import Enum
# ============================================================================
# TYPE DEFINITIONS FOR STRUCTURED LOGGING
# ============================================================================
LogLevel = Literal['log', 'error', 'warn', 'info', 'debug']
ConsoleEventType = Literal[
    'log', 'info', 'warn', 'error', 'debug', 'trace',
    'group', 'groupCollapsed', 'groupEnd', 'table', 'success', 'system'
]
@dataclass

class ConsoleEvent:
    """
    Console event structure for structured logging.
    Attributes:
        id: Unique identifier for the event
        type: Type of console event
        timestamp: Unix timestamp (seconds or milliseconds) or ISO string
        color: CSS color for the event display
        label: Short label for the badge
        msg: Human-readable message
        source: Optional source/operation/module name
        level: Optional explicit log level (overrides type mapping)
        data: Optional additional structured data
    """
    id: int
    type: ConsoleEventType
    timestamp: float | int | str  # Unix timestamp or ISO string
    color: str
    label: str
    msg: str
    source: str | None = None
    level: LogLevel | None = None
    data: Any | None = None

    def to_dict(self) -> dict:
        """Convert to dictionary for JSON serialization."""
        result = {
            'id': self.id,
            'type': self.type,
            'timestamp': self.timestamp,
            'color': self.color,
            'label': self.label,
            'msg': self.msg,
        }
        if self.source is not None:
            result['source'] = self.source
        if self.level is not None:
            result['level'] = self.level
        if self.data is not None:
            result['data'] = self.data
        return result
# ============================================================================
# GENERAL CONSOLE ENUMS (moved from CLI - general console concepts)
# ============================================================================


class Alignment(Enum):
    """Text alignment options."""
    LEFT = "left"
    CENTER = "center"
    RIGHT = "right"


class BorderStyle(Enum):
    """Table border styles."""
    NONE = "none"
    ASCII = "ascii"
    SIMPLE = "simple"
    ROUNDED = "rounded"
    DOUBLE = "double"
    THICK = "thick"


class Colors(Enum):
    """ANSI color codes."""
    # Standard colors
    BLACK = "\033[30m"
    RED = "\033[31m"
    GREEN = "\033[32m"
    YELLOW = "\033[33m"
    BLUE = "\033[34m"
    MAGENTA = "\033[35m"
    CYAN = "\033[36m"
    WHITE = "\033[37m"
    # Bright colors
    BRIGHT_BLACK = "\033[90m"
    BRIGHT_RED = "\033[91m"
    BRIGHT_GREEN = "\033[92m"
    BRIGHT_YELLOW = "\033[93m"
    BRIGHT_BLUE = "\033[94m"
    BRIGHT_MAGENTA = "\033[95m"
    BRIGHT_CYAN = "\033[96m"
    BRIGHT_WHITE = "\033[97m"
    # Background colors
    BG_BLACK = "\033[40m"
    BG_RED = "\033[41m"
    BG_GREEN = "\033[42m"
    BG_YELLOW = "\033[43m"
    BG_BLUE = "\033[44m"
    BG_MAGENTA = "\033[45m"
    BG_CYAN = "\033[46m"
    BG_WHITE = "\033[47m"
    # Reset
    RESET = "\033[0m"


class Style(Enum):
    """ANSI style codes."""
    RESET = "\033[0m"
    BOLD = "\033[1m"
    DIM = "\033[2m"
    ITALIC = "\033[3m"
    UNDERLINE = "\033[4m"
    BLINK = "\033[5m"
    REVERSE = "\033[7m"
    STRIKETHROUGH = "\033[9m"
__all__ = [
    # Structured logging types
    'LogLevel',
    'ConsoleEventType',
    'ConsoleEvent',
    # General console enums
    'Alignment',
    'BorderStyle',
    'Colors',
    'Style',
]
