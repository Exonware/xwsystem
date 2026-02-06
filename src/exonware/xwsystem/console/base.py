#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/base.py
#exonware/xwsystem/console/base.py
"""
Company: eXonware.com
Author: Eng. Muhammad AlShehri
Email: connect@exonware.com
Version: 0.1.0.4
Generation Date: 2025-01-27

Console module base classes - abstract base classes for console functionality.
"""

from abc import ABC, abstractmethod
from typing import Optional, Any
from .contracts import IEventLogger, IConsoleWriter
from .defs import LogLevel, ConsoleEventType, ConsoleEvent


class AEventLogger(ABC, IEventLogger):
    """
    Abstract base class for event logging.
    
    All event loggers should extend this class.
    """
    
    @abstractmethod
    def log(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None,
        color: Optional[str] = None,
        label: Optional[str] = None
    ) -> ConsoleEvent:
        """Log a general message."""
        pass
    
    @abstractmethod
    def info(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None
    ) -> ConsoleEvent:
        """Log an info message."""
        pass
    
    @abstractmethod
    def warn(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None
    ) -> ConsoleEvent:
        """Log a warning message."""
        pass
    
    @abstractmethod
    def error(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None,
        stack: Optional[str] = None
    ) -> ConsoleEvent:
        """Log an error message."""
        pass
    
    @abstractmethod
    def debug(
        self,
        msg: str,
        source: Optional[str] = None,
        data: Optional[Any] = None
    ) -> ConsoleEvent:
        """Log a debug message."""
        pass
    
    @abstractmethod
    def get_events(
        self,
        event_type: Optional[ConsoleEventType] = None,
        level: Optional[LogLevel] = None,
        source: Optional[str] = None,
        limit: Optional[int] = None
    ) -> list[dict]:
        """Get logged events as dictionaries."""
        pass
    
    @abstractmethod
    def clear(self) -> None:
        """Clear all logged events."""
        pass
    
    @abstractmethod
    def count(self) -> int:
        """Get the number of logged events."""
        pass


class AConsoleWriter(ABC, IConsoleWriter):
    """
    Abstract base class for console writing (user interaction, not logging).
    
    Can be used as a base class for bots and other user interaction systems.
    """
    
    @abstractmethod
    def write(self, text: str, **kwargs) -> None:
        """Write text to console."""
        pass
    
    @abstractmethod
    def write_line(self, text: str = "", **kwargs) -> None:
        """Write a line to console."""
        pass
    
    @abstractmethod
    def read(self, prompt: str = "") -> str:
        """Read input from console."""
        pass
    
    @abstractmethod
    def clear(self) -> None:
        """Clear console screen."""
        pass


__all__ = [
    'AEventLogger',
    'AConsoleWriter',
]
