#exonware/xwsystem/src/exonware/xwsystem/console/cli/event_logger.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.30
Generation Date: January 2025
CLI Event Logger - Extends Python's logging.Logger with event logging capabilities.
"""

import logging
from typing import Any
from ..event_logger import ConsoleEventLogger
from ..contracts import IEventLogger
from ..defs import LogLevel, ConsoleEventType, ConsoleEvent


class CliEventLogger(logging.Logger):
    """
    CLI Event Logger that extends Python's logging.Logger.
    Combines Python's standard logging.Logger with structured event logging
    from ConsoleEventLogger. All standard logging.Logger methods work as expected,
    and additional event logging methods are available.
    """

    def __init__(self, name: str, level: int = logging.NOTSET):
        """
        Initialize CLI Event Logger.
        Args:
            name: Logger name
            level: Logging level
        """
        # Initialize Python's Logger
        super().__init__(name, level)
        # Initialize internal event logger for structured logging
        self._event_logger = ConsoleEventLogger(
            max_entries=1000,
            use_milliseconds=True,
            default_source=name
        )
    # Event logging methods (from IEventLogger)

    def log_event(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None,
        color: str | None = None,
        label: str | None = None
    ) -> ConsoleEvent:
        """Log a general event message."""
        event = self._event_logger.log(msg, source=source, data=data, color=color, label=label)
        # Also log using standard logger
        super().info(msg)
        return event

    def info_event(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log an info event."""
        event = self._event_logger.info(msg, source=source, data=data)
        super().info(msg)
        return event

    def warn_event(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a warning event."""
        event = self._event_logger.warn(msg, source=source, data=data)
        super().warning(msg)
        return event

    def error_event(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None,
        stack: str | None = None
    ) -> ConsoleEvent:
        """Log an error event."""
        event = self._event_logger.error(msg, source=source, data=data, stack=stack)
        super().error(msg, exc_info=stack is not None)
        return event

    def debug_event(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a debug event."""
        event = self._event_logger.debug(msg, source=source, data=data)
        super().debug(msg)
        return event
    # Override standard logging methods to also log events

    def info(self, msg: object, *args, **kwargs) -> None:
        """Log info message (standard logger + event logger)."""
        msg_str = str(msg) % args if args else str(msg)
        self._event_logger.info(msg_str, source=self.name)
        super().info(msg, *args, **kwargs)

    def warning(self, msg: object, *args, **kwargs) -> None:
        """Log warning message (standard logger + event logger)."""
        msg_str = str(msg) % args if args else str(msg)
        self._event_logger.warn(msg_str, source=self.name)
        super().warning(msg, *args, **kwargs)

    def error(self, msg: object, *args, **kwargs) -> None:
        """Log error message (standard logger + event logger)."""
        msg_str = str(msg) % args if args else str(msg)
        self._event_logger.error(msg_str, source=self.name)
        super().error(msg, *args, **kwargs)

    def debug(self, msg: object, *args, **kwargs) -> None:
        """Log debug message (standard logger + event logger)."""
        msg_str = str(msg) % args if args else str(msg)
        self._event_logger.debug(msg_str, source=self.name)
        super().debug(msg, *args, **kwargs)

    def critical(self, msg: object, *args, **kwargs) -> None:
        """Log critical message (standard logger + event logger)."""
        msg_str = str(msg) % args if args else str(msg)
        self._event_logger.error(msg_str, source=self.name)
        super().critical(msg, *args, **kwargs)
    # Event logger methods (delegate to internal event logger)

    def get_events(
        self,
        event_type: ConsoleEventType | None = None,
        level: LogLevel | None = None,
        source: str | None = None,
        limit: int | None = None
    ) -> list[dict]:
        """Get logged events as dictionaries."""
        return self._event_logger.get_events(event_type=event_type, level=level, source=source, limit=limit)

    def clear_events(self) -> None:
        """Clear all logged events."""
        self._event_logger.clear()

    def count_events(self) -> int:
        """Get the number of logged events."""
        return self._event_logger.count()
    @property

    def event_logger(self) -> ConsoleEventLogger:
        """Get the internal event logger instance."""
        return self._event_logger
__all__ = [
    'CliEventLogger',
]
