#exonware/xwsystem/src/exonware/xwsystem/utils/dt/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.41
Generation Date: September 04, 2025
DateTime module contracts - interfaces and enums for date/time functionality.
"""

from typing import Any, Protocol, runtime_checkable
from datetime import datetime, date, time, timezone
# Import enums from types module
from .defs import (
    TimeFormat,
    DateFormat,
    TimezoneType,
    HumanizeUnit,
    DateTimeFormat,
    HumanizeStyle
)
@runtime_checkable

class IDateTimeFormatter(Protocol):
    """Interface for date/time formatting."""

    def format_datetime(self, dt: datetime, format_type: TimeFormat) -> str:
        """Format datetime object."""
        ...

    def format_date(self, d: date, format_type: DateFormat) -> str:
        """Format date object."""
        ...

    def format_time(self, t: time, format_type: TimeFormat) -> str:
        """Format time object."""
        ...
@runtime_checkable

class IDateTimeParser(Protocol):
    """Interface for date/time parsing."""

    def parse_datetime(self, date_string: str, format_type: TimeFormat | None = None) -> datetime:
        """Parse datetime string."""
        ...

    def parse_date(self, date_string: str, format_type: DateFormat | None = None) -> date:
        """Parse date string."""
        ...

    def parse_time(self, time_string: str, format_type: TimeFormat | None = None) -> time:
        """Parse time string."""
        ...
@runtime_checkable

class IDateTimeHumanizer(Protocol):
    """Interface for humanizing time differences."""

    def humanize(self, dt: datetime, reference: datetime | None = None) -> str:
        """Humanize datetime relative to reference."""
        ...

    def natural_time(self, dt: datetime, reference: datetime | None = None) -> str:
        """Get natural time representation."""
        ...
@runtime_checkable

class ITimezoneUtils(Protocol):
    """Interface for timezone utilities."""

    def get_timezone(self, tz_name: str) -> timezone:
        """Get timezone object."""
        ...

    def convert_timezone(self, dt: datetime, target_tz: timezone) -> datetime:
        """Convert datetime to target timezone."""
        ...

    def get_local_timezone(self) -> timezone:
        """Get local timezone."""
        ...
