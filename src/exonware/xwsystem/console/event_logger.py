#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/event_logger.py
#exonware/xwsystem/console/event_logger.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.40
Generation Date: 2025-01-27
Console Event Logger implementation for structured logging.
"""

import time
import threading
from typing import Any
from .base import AEventLogger
from .defs import LogLevel, ConsoleEventType, ConsoleEvent


class ConsoleEventLogger(AEventLogger):
    """
    Console event logger for structured logging.
    This logger generates structured events with consistent formatting.
    All other loggers in xwsystem should reuse this for structured logging.
    Example:
        logger = ConsoleEventLogger()
        logger.log("App initialized", source="app.init")
        logger.error("Failed to connect", source="api.client", data={"endpoint": "/api/users"})
        events = logger.get_events()  # Returns list of ConsoleEvent dicts
    """
    # Default colors for each event type
    DEFAULT_COLORS = {
        'log': '#ffffff',
        'info': '#4aa3ff',
        'warn': '#ffb020',
        'error': '#ff4d4f',
        'debug': '#b38cff',
        'trace': '#ff85c0',
        'group': '#ffd666',
        'groupCollapsed': '#ffd666',
        'groupEnd': '#ffd666',
        'table': '#40a9ff',
        'success': '#2ecc71',
        'system': '#888',
    }
    # Default labels for each event type
    DEFAULT_LABELS = {
        'log': 'LOG',
        'info': 'INFO',
        'warn': 'WARN',
        'error': 'ERROR',
        'debug': 'DEBUG',
        'trace': 'TRACE',
        'group': 'GROUP',
        'groupCollapsed': 'GROUP',
        'groupEnd': 'END',
        'table': 'TABLE',
        'success': 'OK',
        'system': 'SYSTEM',
    }
    # Type to level mapping (when level not explicitly provided)
    TYPE_TO_LEVEL: dict[ConsoleEventType, LogLevel] = {
        'error': 'error',
        'warn': 'warn',
        'info': 'info',
        'debug': 'debug',
        'trace': 'debug',
        'log': 'log',
        'success': 'log',
        'system': 'log',
        'table': 'log',
        'group': 'log',
        'groupCollapsed': 'log',
        'groupEnd': 'log',
    }

    def __init__(
        self,
        max_entries: int = 1000,
        use_milliseconds: bool = True,
        default_source: str | None = None
    ):
        """
        Initialize console event logger.
        Args:
            max_entries: Maximum number of events to keep in memory
            use_milliseconds: If True, timestamps in milliseconds; if False, in seconds
            default_source: Default source name for all events (can be overridden per event)
        """
        self.max_entries = max_entries
        self.use_milliseconds = use_milliseconds
        self.default_source = default_source
        self._events: list[ConsoleEvent] = []
        self._event_id_counter = 0
        self._lock = threading.Lock()

    def _get_timestamp(self) -> float:
        """Get current timestamp in the configured format."""
        if self.use_milliseconds:
            return time.time() * 1000  # Milliseconds
        return time.time()  # Seconds

    def _get_next_id(self) -> int:
        """Get next event ID (thread-safe)."""
        with self._lock:
            self._event_id_counter += 1
            return self._event_id_counter

    def _add_event(
        self,
        event_type: ConsoleEventType,
        msg: str,
        source: str | None = None,
        level: LogLevel | None = None,
        color: str | None = None,
        label: str | None = None,
        timestamp: float | int | str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """
        Add an event to the log (internal method).
        Args:
            event_type: Type of console event
            msg: Message text
            source: Source/operation/module name (uses default_source if not provided)
            level: Explicit log level (derived from type if not provided)
            color: Color for display (uses default for type if not provided)
            label: Label for badge (uses default for type if not provided)
            timestamp: Timestamp (uses current time if not provided)
            data: Additional structured data
        Returns:
            Created ConsoleEvent
        """
        event_id = self._get_next_id()
        event_timestamp = timestamp if timestamp is not None else self._get_timestamp()
        event_source = source if source is not None else self.default_source
        event_level = level if level is not None else self.TYPE_TO_LEVEL.get(event_type, 'log')
        event_color = color if color is not None else self.DEFAULT_COLORS.get(event_type, '#ffffff')
        event_label = label if label is not None else self.DEFAULT_LABELS.get(event_type, 'LOG')
        event = ConsoleEvent(
            id=event_id,
            type=event_type,
            timestamp=event_timestamp,
            color=event_color,
            label=event_label,
            msg=msg,
            source=event_source,
            level=event_level,
            data=data
        )
        with self._lock:
            self._events.append(event)
            # Limit entries
            if len(self._events) > self.max_entries:
                self._events = self._events[-self.max_entries:]
        return event
    # Public API methods matching common logging patterns

    def log(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None,
        color: str | None = None,
        label: str | None = None
    ) -> ConsoleEvent:
        """Log a general message."""
        return self._add_event('log', msg, source=source, data=data, color=color, label=label)

    def info(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log an info message."""
        return self._add_event('info', msg, source=source, data=data)

    def warn(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a warning message."""
        return self._add_event('warn', msg, source=source, data=data)

    def error(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None,
        stack: str | None = None
    ) -> ConsoleEvent:
        """Log an error message."""
        error_data = data
        if stack is not None:
            error_data = error_data or {}
            if isinstance(error_data, dict):
                error_data['stack'] = stack
        return self._add_event('error', msg, source=source, data=error_data)

    def debug(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a debug message."""
        return self._add_event('debug', msg, source=source, data=data)

    def trace(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a trace message."""
        return self._add_event('trace', msg, source=source, data=data)

    def success(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a success message."""
        return self._add_event('success', msg, source=source, data=data)

    def system(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a system message."""
        return self._add_event('system', msg, source=source, data=data)

    def group(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Start a log group."""
        return self._add_event('group', msg, source=source, data=data)

    def group_collapsed(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Start a collapsed log group."""
        return self._add_event('groupCollapsed', msg, source=source, data=data)

    def group_end(
        self,
        msg: str = "",
        source: str | None = None
    ) -> ConsoleEvent:
        """End a log group."""
        return self._add_event('groupEnd', msg, source=source)

    def table(
        self,
        msg: str,
        source: str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """Log a table message."""
        return self._add_event('table', msg, source=source, data=data)

    def add_event(
        self,
        event_type: ConsoleEventType,
        msg: str,
        source: str | None = None,
        level: LogLevel | None = None,
        color: str | None = None,
        label: str | None = None,
        timestamp: float | int | str | None = None,
        data: Any | None = None
    ) -> ConsoleEvent:
        """
        Add a custom event with full control over all fields.
        Args:
            event_type: Type of console event
            msg: Message text
            source: Source/operation/module name
            level: Explicit log level
            color: Color for display
            label: Label for badge
            timestamp: Timestamp (Unix timestamp or ISO string)
            data: Additional structured data
        Returns:
            Created ConsoleEvent
        """
        return self._add_event(
            event_type, msg,
            source=source,
            level=level,
            color=color,
            label=label,
            timestamp=timestamp,
            data=data
        )

    def get_events(
        self,
        event_type: ConsoleEventType | None = None,
        level: LogLevel | None = None,
        source: str | None = None,
        limit: int | None = None
    ) -> list[dict]:
        """
        Get logged events as dictionaries.
        Args:
            event_type: Filter by event type
            level: Filter by log level
            source: Filter by source
            limit: Maximum number of events to return (most recent)
        Returns:
            List of event dictionaries in ConsoleEvent format
        """
        with self._lock:
            events = self._events.copy()
        # Apply filters
        if event_type is not None:
            events = [e for e in events if e.type == event_type]
        if level is not None:
            events = [e for e in events if (e.level or self.TYPE_TO_LEVEL.get(e.type, 'log')) == level]
        if source is not None:
            events = [e for e in events if e.source == source]
        # Apply limit (most recent)
        if limit is not None and limit > 0:
            events = events[-limit:]
        # Convert to dictionaries
        return [e.to_dict() for e in events]

    def get_events_raw(self) -> list[ConsoleEvent]:
        """
        Get logged events as ConsoleEvent objects.
        Returns:
            List of ConsoleEvent objects
        """
        with self._lock:
            return self._events.copy()

    def clear(self) -> None:
        """Clear all logged events."""
        with self._lock:
            self._events.clear()
            self._event_id_counter = 0

    def count(self) -> int:
        """Get the number of logged events."""
        with self._lock:
            return len(self._events)
__all__ = [
    'ConsoleEventLogger',
]
