#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/__init__.py
#exonware/xwsystem/console/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.34
Generation Date: 2025-01-27
Console module for structured logging and user interaction.
"""

from .defs import (
    LogLevel,
    ConsoleEventType,
    ConsoleEvent,
)
from .contracts import (
    IEventLogger,
    IConsoleWriter,
)
from .base import (
    AEventLogger,
    AConsoleWriter,
)
from .event_logger import ConsoleEventLogger
from .writer import ConsoleWriter
from .errors import (
    ConsoleError,
    EventLoggerError,
    ConsoleWriterError,
)
__all__ = [
    # Type definitions
    'LogLevel',
    'ConsoleEventType',
    'ConsoleEvent',
    # Interfaces
    'IEventLogger',
    'IConsoleWriter',
    # Abstract base classes
    'AEventLogger',
    'AConsoleWriter',
    # Implementations
    'ConsoleEventLogger',
    'ConsoleWriter',
    # Errors
    'ConsoleError',
    'EventLoggerError',
    'ConsoleWriterError',
]
