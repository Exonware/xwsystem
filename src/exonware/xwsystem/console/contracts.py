#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/contracts.py
#exonware/xwsystem/console/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.1.0.11
Generation Date: 2025-01-27
Console module contracts - interfaces for console functionality.
"""

from typing import Protocol, runtime_checkable, Optional, Any
from .defs import LogLevel, ConsoleEventType, ConsoleEvent
@runtime_checkable

class IEventLogger(Protocol):
    """Interface for event logging operations."""

    def log(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None,
        color: Optional[str] = None,
        label: Optional[str] = None
    ) -> ConsoleEvent:
        """Log a general message."""
        ...

    def info(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None
    ) -> ConsoleEvent:
        """Log an info message."""
        ...

    def warn(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None
    ) -> ConsoleEvent:
        """Log a warning message."""
        ...

    def error(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None,
        stack: Optional[str] = None
    ) -> ConsoleEvent:
        """Log an error message."""
        ...

    def debug(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None
    ) -> ConsoleEvent:
        """Log a debug message."""
        ...

    def get_events(
        self,
        event_type: Optional[ConsoleEventType] = None,
        level: Optional[LogLevel] = None,
        source: Optional[str] = None,
        limit: Optional[int] = None
    ) -> list[dict]:
        """Get logged events as dictionaries."""
        ...

    def clear(self) -> None:
        """Clear all logged events."""
        ...

    def count(self) -> int:
        """Get the number of logged events."""
        ...
@runtime_checkable

class IConsoleWriter(Protocol):
    """Interface for console writing operations (user interaction, not logging)."""

    def write(self, text: str, **kwargs) -> None:
        """Write text to console."""
        ...

    def write_line(self, text: str = "", **kwargs) -> None:
        """Write a line to console."""
        ...

    def read(self, prompt: str = "") -> str:
        """Read input from console."""
        ...

    def clear(self) -> None:
        """Clear console screen."""
        ...
__all__ = [
    'IEventLogger',
    'IConsoleWriter',
]
