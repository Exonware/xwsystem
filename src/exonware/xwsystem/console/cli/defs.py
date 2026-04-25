#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/cli/defs.py
#exonware/xwsystem/console/cli/cli_defs.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.43
Generation Date: 07-Sep-2025
CLI-specific types and enums for XWSystem.
"""

from enum import Enum
# Import general console enums from console level (priority)
from ..defs import Alignment, BorderStyle, Colors, Style
# ============================================================================
# CLI-SPECIFIC ENUMS
# ============================================================================


class ColorType(Enum):
    """Color types for CLI output."""
    RESET = "reset"
    BOLD = "bold"
    DIM = "dim"
    UNDERLINE = "underline"
    RED = "red"
    GREEN = "green"
    YELLOW = "yellow"
    BLUE = "blue"
    MAGENTA = "magenta"
    CYAN = "cyan"
    WHITE = "white"
    GRAY = "gray"


class ProgressStyle(Enum):
    """Progress bar styles."""
    BAR = "bar"
    SPINNER = "spinner"
    PERCENTAGE = "percentage"
    COUNTER = "counter"


class TableStyle(Enum):
    """Table display styles."""
    SIMPLE = "simple"
    GRID = "grid"
    FANCY = "fancy"
    MINIMAL = "minimal"


class PromptType(Enum):
    """Prompt input types."""
    TEXT = "text"
    PASSWORD = "password"
    CONFIRM = "confirm"
    SELECT = "select"
    MULTISELECT = "multiselect"


class ArgumentType(Enum):
    """Types of command-line arguments."""
    STRING = "string"
    INTEGER = "int"
    FLOAT = "float"
    BOOLEAN = "bool"
    FILE = "file"
    DIRECTORY = "dir"
    CHOICE = "choice"
__all__ = [
    # Re-export general console enums
    'Alignment',
    'BorderStyle',
    'Colors',
    'Style',
    # CLI-specific enums
    'ColorType',
    'ProgressStyle',
    'TableStyle',
    'PromptType',
    'ArgumentType',
]
