#!/usr/bin/env python3
"""
#exonware/xwsystem/src/exonware/xwsystem/utils/__init__.py
Utilities package for xwsystem.
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.22
Generation Date: 28-Dec-2025
"""
# String utilities

from .string import find_nth_occurrence
# Web utilities
from .web import validate_url_accessible, extract_webpage_text
# DateTime utilities (re-export from dt submodule)
from .dt import (
    get_datetime, get_date, get_date_from_to_month, calculate_duration_days,
    parse_timestamp_milliseconds
)
__all__ = [
    # String utilities
    'find_nth_occurrence',
    # Web utilities
    'validate_url_accessible',
    'extract_webpage_text',
    # DateTime utilities
    'get_datetime',
    'get_date',
    'get_date_from_to_month',
    'calculate_duration_days',
    'parse_timestamp_milliseconds',
]
