#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/operations/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.1.0.6
Generation Date: October 26, 2025
Base classes and contracts for operations.
"""

from typing import Optional


class OperationError(Exception):
    """Base exception for operation errors."""

    def __init__(self, message: str, operation: Optional[str] = None):
        super().__init__(message)
        self.operation = operation


class MergeError(OperationError):
    """Error during merge operations."""
    pass


class DiffError(OperationError):
    """Error during diff operations."""
    pass


class PatchError(OperationError):
    """Error during patch operations."""
    pass
__all__ = [
    "OperationError",
    "MergeError", 
    "DiffError",
    "PatchError",
]
