#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/console/errors.py
#exonware/xwsystem/console/errors.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.1.0.10
Generation Date: 2025-01-27
Console module errors - exception classes for console functionality.
"""


class ConsoleError(Exception):
    """Base exception for console errors."""
    pass


class EventLoggerError(ConsoleError):
    """Raised when event logger operation fails."""
    pass


class ConsoleWriterError(ConsoleError):
    """Raised when console writer operation fails."""
    pass
__all__ = [
    'ConsoleError',
    'EventLoggerError',
    'ConsoleWriterError',
]
