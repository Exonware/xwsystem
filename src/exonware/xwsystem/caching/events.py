#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/caching/events.py
#exonware/xwsystem/caching/events.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.35
Generation Date: 01-Nov-2025
Event system for caching module.
Extensibility Priority #5 - Event-driven architecture for custom behaviors.
"""

from typing import Any

from collections.abc import Callable
from enum import Enum
from ..config.logging_setup import get_logger
from ..console import ConsoleEventLogger
logger = get_logger("xwsystem.caching.events")


class CacheEvent(Enum):
    """Cache events for hook registration."""
    HIT = "hit"
    MISS = "miss"
    PUT = "put"
    DELETE = "delete"
    EVICT = "evict"
    EXPIRE = "expire"
    CLEAR = "clear"
    ERROR = "error"


class CacheEventEmitter:
    """
    Mixin class for event emission in caches.
    Provides event hook registration and emission functionality.
    Example:
        class EventDrivenCache(CacheEventEmitter, LRUCache):
            def get(self, key):
                value = super().get(key)
                if value is not None:
                    self._emit(CacheEvent.HIT, key=key, value=value)
                else:
                    self._emit(CacheEvent.MISS, key=key)
                return value
    """

    def __init__(self):
        """Initialize event emitter."""
        self._hooks: dict[CacheEvent, list[Callable]] = {
            event: [] for event in CacheEvent
        }
        self._event_stats: dict[CacheEvent, int] = {
            event: 0 for event in CacheEvent
        }

    def on(self, event: CacheEvent, callback: Callable) -> None:
        """
        Register event callback.
        Args:
            event: Event to listen for
            callback: Callback function(event, **kwargs)
        Example:
            def on_cache_hit(event, key, value):
                print(f"Cache hit for {key}")
            cache.on(CacheEvent.HIT, on_cache_hit)
        """
        if event not in self._hooks:
            raise ValueError(f"Invalid event: {event}. Valid events: {list(CacheEvent)}")
        if callback not in self._hooks[event]:
            self._hooks[event].append(callback)
            logger.debug(f"Registered callback for event: {event.value}")

    def off(self, event: CacheEvent, callback: Callable) -> bool:
        """
        Unregister event callback.
        Args:
            event: Event to stop listening for
            callback: Callback function to remove
        Returns:
            True if callback was removed
        """
        if event in self._hooks and callback in self._hooks[event]:
            self._hooks[event].remove(callback)
            logger.debug(f"Unregistered callback for event: {event.value}")
            return True
        return False

    def clear_hooks(self, event: CacheEvent | None = None) -> None:
        """
        Clear event hooks.
        Args:
            event: Specific event to clear (None = clear all)
        """
        if event:
            self._hooks[event].clear()
        else:
            for event_type in self._hooks:
                self._hooks[event_type].clear()

    def _emit(self, event: CacheEvent, **kwargs) -> None:
        """
        Emit event to registered callbacks.
        Args:
            event: Event to emit
            **kwargs: Event data passed to callbacks
        """
        self._event_stats[event] += 1
        for callback in self._hooks.get(event, []):
            try:
                callback(event=event, **kwargs)
            except Exception as e:
                logger.error(f"Event callback failed for {event.value}: {e}")
                self._emit(CacheEvent.ERROR, error=e, event=event, **kwargs)

    def get_event_stats(self) -> dict[str, int]:
        """
        Get event emission statistics.
        Returns:
            Dictionary of event counts
        """
        return {event.value: count for event, count in self._event_stats.items()}


class EventLogger:
    """
    Built-in event logger for debugging.
    Logs all cache events for debugging and monitoring.
    Uses ConsoleEventLogger internally.
    """
    # Map CacheEvent to ConsoleEventType
    _CACHE_EVENT_TO_CONSOLE_TYPE = {
        CacheEvent.HIT: 'info',
        CacheEvent.MISS: 'warn',
        CacheEvent.PUT: 'log',
        CacheEvent.DELETE: 'log',
        CacheEvent.EVICT: 'warn',
        CacheEvent.EXPIRE: 'warn',
        CacheEvent.CLEAR: 'system',
        CacheEvent.ERROR: 'error',
    }

    def __init__(self, log_level: str = "DEBUG", max_entries: int = 1000):
        """
        Initialize event logger.
        Args:
            log_level: Logging level for events (passed through; ConsoleEventLogger may ignore)
            max_entries: Maximum number of events to keep
        """
        self.log_level = log_level
        # Use ConsoleEventLogger internally
        self._console_logger = ConsoleEventLogger(
            max_entries=max_entries,
            use_milliseconds=True,
            default_source="cache.events"
        )

    def __call__(self, event: CacheEvent, **kwargs):
        """Log event."""
        # Map CacheEvent to ConsoleEventType
        console_type = self._CACHE_EVENT_TO_CONSOLE_TYPE.get(event, 'log')
        # Format message based on event type
        key = kwargs.get('key', '?')
        if event == CacheEvent.HIT:
            msg = f"Cache HIT: {key}"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.MISS:
            msg = f"Cache MISS: {key}"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.PUT:
            msg = f"Cache PUT: {key}"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.DELETE:
            msg = f"Cache DELETE: {key}"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.EVICT:
            msg = f"Cache EVICT: {key}"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.EXPIRE:
            msg = f"Cache EXPIRE: {key}"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.CLEAR:
            msg = "Cache CLEAR"
            logger.debug(f"[EVENT] {msg}")
        elif event == CacheEvent.ERROR:
            error_msg = kwargs.get('error', 'Unknown')
            msg = f"Cache ERROR: {error_msg}"
            logger.error(f"[EVENT] {msg}")
        else:
            msg = f"Cache {event.value.upper()}: {key}"
        # Log to ConsoleEventLogger
        # Include original cache event type in data for filtering
        event_data = kwargs.copy()
        event_data['_cache_event'] = event.value
        self._console_logger.add_event(
            event_type=console_type,
            msg=msg,
            source="cache.events",
            data=event_data
        )

    def get_events(self, event_type: CacheEvent | None = None) -> list[dict[str, Any]]:
        """
        Get logged events, optionally filtered by type.
        Returns events in a simplified format.
        For XWUIConsole format, use get_console_events() instead.
        """
        # Get events from ConsoleEventLogger
        console_events = self._console_logger.get_events_raw()
        # Convert to simplified format
        result = []
        for event in console_events:
            # Extract original event type from data if available
            event_data = event.data or {}
            cache_event_value = event_data.get('_cache_event', None)
            # Filter by event_type if specified
            if event_type is not None:
                if cache_event_value != event_type.value:
                    continue
            # Convert to old format
            old_format = {
                'timestamp': event.timestamp / 1000 if isinstance(event.timestamp, (int, float)) and event.timestamp > 1e10 else event.timestamp,
                'event': cache_event_value or event.type,
                'data': event.data
            }
            result.append(old_format)
        return result

    def get_console_events(self) -> list[dict[str, Any]]:
        """
        Get logged events in structured format.
        Returns:
            List of event dictionaries in ConsoleEvent format
        """
        return self._console_logger.get_events()

    def clear(self) -> None:
        """Clear event log."""
        self._console_logger.clear()
    @property

    def console_logger(self) -> ConsoleEventLogger:
        """
        Get the underlying ConsoleEventLogger instance.
        Returns:
            ConsoleEventLogger instance for direct access
        """
        return self._console_logger
__all__ = [
    'CacheEvent',
    'CacheEventEmitter',
    'EventLogger',
]
