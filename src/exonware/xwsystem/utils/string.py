#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/utils/string.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.37
Generation Date: January 2025
String utility functions for xwsystem.
"""



def find_nth_occurrence(text: str, key: str, n: int) -> int:
    """
    Find the nth occurrence of a substring in text.
    Args:
        text: Text to search
        key: Substring to find
        n: Occurrence number (1-based)
    Returns:
        Index of nth occurrence, or -1 if not found
    Examples:
        >>> find_nth_occurrence("hello world hello", "hello", 1)
        0
        >>> find_nth_occurrence("hello world hello", "hello", 2)
        12
        >>> find_nth_occurrence("hello world hello", "hello", 3)
        -1
    """
    if not text or not key or n < 1:
        return -1
    start = text.find(key)
    while start >= 0 and n > 1:
        start = text.find(key, start + len(key))
        n -= 1
    return start
__all__ = [
    'find_nth_occurrence',
]
