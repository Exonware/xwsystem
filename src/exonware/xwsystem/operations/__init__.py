#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/operations/__init__.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.30
Generation Date: October 26, 2025
Universal operations library for data manipulation.
"""

from ..config.logging_setup import get_logger
logger = get_logger("xwsystem.operations")
# Import from submodules
from .defs import MergeStrategy, DiffMode, PatchOperation, DiffResult, PatchResult
from .base import (
    OperationError, MergeError, DiffError, PatchError
)
from .contracts import (
    IOperation, IMergeOperation, IDiffOperation, IPatchOperation
)
from .merge import MergeOperation, deep_merge
from .diff import DiffOperation, generate_diff
from .patch import PatchOperationImpl, apply_patch
__all__ = [
    # Enums and data classes
    "MergeStrategy",
    "DiffMode", 
    "PatchOperation",
    "DiffResult",
    "PatchResult",
    # Exceptions
    "OperationError",
    "MergeError", 
    "DiffError",
    "PatchError",
    # Interfaces
    "IOperation",
    "IMergeOperation",
    "IDiffOperation",
    "IPatchOperation",
    # Operations
    "MergeOperation",
    "DiffOperation",
    "PatchOperationImpl",
    # Convenience functions
    "deep_merge",
    "generate_diff",
    "apply_patch",
]
