#exonware/xwsystem/src/exonware/xwsystem/utils/dt/formatting.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.5
Generation Date: September 04, 2025
DateTime formatting utilities for XSystem.
"""

from datetime import datetime, date, time
from typing import Optional, Any
from .contracts import IDateTimeFormatter, TimeFormat, DateFormat
from .errors import DateTimeFormatError


class DateTimeFormatter(IDateTimeFormatter):
    """DateTime formatter implementation."""

    def __init__(self):
        """Initialize formatter."""
        self._formats = {
            TimeFormat.ISO: "%Y-%m-%dT%H:%M:%S",
            TimeFormat.RFC2822: "%a, %d %b %Y %H:%M:%S %z",
            TimeFormat.RFC3339: "%Y-%m-%dT%H:%M:%S%z",
            DateFormat.ISO: "%Y-%m-%d",
            DateFormat.US: "%m/%d/%Y",
            DateFormat.EU: "%d/%m/%Y"
        }

    def format_datetime(self, dt: datetime, format_type: TimeFormat) -> str:
        """Format datetime object."""
        try:
            if format_type == TimeFormat.ISO:
                return dt.isoformat()
            elif format_type == TimeFormat.RFC2822:
                return dt.strftime("%a, %d %b %Y %H:%M:%S %z")
            elif format_type == TimeFormat.RFC3339:
                return dt.isoformat()
            else:
                format_string = self._formats.get(format_type, "%Y-%m-%d %H:%M:%S")
                return dt.strftime(format_string)
        except Exception as e:
            raise DateTimeFormatError(f"Failed to format datetime: {e}")

    def format_date(self, d: date, format_type: DateFormat) -> str:
        """Format date object."""
        try:
            if format_type == DateFormat.ISO:
                return d.isoformat()
            else:
                format_string = self._formats.get(format_type, "%Y-%m-%d")
                return d.strftime(format_string)
        except Exception as e:
            raise DateTimeFormatError(f"Failed to format date: {e}")

    def format_time(self, t: time, format_type: TimeFormat) -> str:
        """Format time object."""
        try:
            if format_type == TimeFormat.ISO:
                return t.isoformat()
            else:
                return t.strftime("%H:%M:%S")
        except Exception as e:
            raise DateTimeFormatError(f"Failed to format time: {e}")

    def format_custom(self, dt: datetime, format_string: str) -> str:
        """Format with custom format string."""
        try:
            return dt.strftime(format_string)
        except Exception as e:
            raise DateTimeFormatError(f"Failed to format with custom format: {e}")

    def get_available_formats(self) -> dict[str, str]:
        """Get available format types."""
        return {
            "ISO": "ISO 8601 format",
            "RFC2822": "RFC 2822 format",
            "RFC3339": "RFC 3339 format",
            "US": "US date format (MM/DD/YYYY)",
            "EU": "European date format (DD/MM/YYYY)"
        }
# Utility functions

def format_datetime(dt: datetime, format_type: str = "iso") -> str:
    """Format datetime with specified type."""
    formatter = DateTimeFormatter()
    if format_type.lower() == "iso":
        return formatter.format_datetime(dt, TimeFormat.ISO)
    elif format_type.lower() == "rfc2822":
        return formatter.format_datetime(dt, TimeFormat.RFC2822)
    elif format_type.lower() == "rfc3339":
        return formatter.format_datetime(dt, TimeFormat.RFC3339)
    else:
        return formatter.format_custom(dt, format_type)


def format_date(d: date, format_type: str = "iso") -> str:
    """Format date with specified type."""
    formatter = DateTimeFormatter()
    if format_type.lower() == "iso":
        return formatter.format_date(d, DateFormat.ISO)
    elif format_type.lower() == "us":
        return formatter.format_date(d, DateFormat.US)
    elif format_type.lower() == "eu":
        return formatter.format_date(d, DateFormat.EU)
    else:
        return formatter.format_custom(datetime.combine(d, time()), format_type)


def format_time(t: time, format_type: str = "iso") -> str:
    """Format time with specified type."""
    formatter = DateTimeFormatter()
    return formatter.format_time(t, TimeFormat.ISO)


def format_iso8601(dt: datetime) -> str:
    """Format datetime in ISO 8601 format."""
    return dt.isoformat()


def format_relative(dt: datetime) -> str:
    """Format datetime as relative time."""
    now = datetime.now()
    diff = now - dt
    if diff.days > 0:
        return f"{diff.days} days ago"
    elif diff.seconds > 3600:
        hours = diff.seconds // 3600
        return f"{hours} hours ago"
    elif diff.seconds > 60:
        minutes = diff.seconds // 60
        return f"{minutes} minutes ago"
    else:
        return "just now"


def get_datetime() -> str:
    """
    Get current datetime as formatted string.
    Returns:
        Current datetime in "YYYY-MM-DD HH:MM:SS" format
    Examples:
        >>> dt_str = get_datetime()
        >>> len(dt_str) == 19
        True
    """
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def get_date() -> str:
    """
    Get current date as formatted string.
    Returns:
        Current date in "YYYY-MM-DD" format
    Examples:
        >>> date_str = get_date()
        >>> len(date_str) == 10
        True
    """
    return datetime.now().strftime("%Y-%m-%d")


def get_date_from_to_month(year_month: str) -> tuple[str, str]:
    """
    Get start and end dates for a month.
    Args:
        year_month: Format "YYYY-MM"
    Returns:
        Tuple of (start_date, end_date) in "YYYY-MM-DD" format
    Examples:
        >>> start, end = get_date_from_to_month("2025-01")
        >>> start
        '2025-01-01'
        >>> end
        '2025-01-31'
    """
    import calendar
    year, month = map(int, year_month.split('-'))
    # Start of the month is day 1
    start_of_month = f"{year}-{month:02d}-01"
    # Get the last day of the month
    _, last_day = calendar.monthrange(year, month)
    end_of_month = f"{year}-{month:02d}-{last_day:02d}"
    return start_of_month, end_of_month


def calculate_duration_days(date_from: datetime, date_to: datetime) -> float:
    """
    Calculate duration between two dates in days.
    Args:
        date_from: Start date
        date_to: End date
    Returns:
        Duration in days (rounded to 2 decimal places)
    Examples:
        >>> from datetime import datetime, timedelta
        >>> start = datetime(2025, 1, 1)
        >>> end = datetime(2025, 1, 5)
        >>> calculate_duration_days(end, start)
        4.0
    """
    delta = date_from - date_to
    seconds = delta.total_seconds()
    days = seconds / 86400  # 60 * 60 * 24
    return round(days, 2)
