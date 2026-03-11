#exonware/xwsystem/src/exonware/xwsystem/operations/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Operations Contracts
Protocol definitions for data operations.
"""

from typing import Any, Protocol, runtime_checkable
from .defs import MergeStrategy, DiffMode, DiffResult, PatchResult
@runtime_checkable


class IOperation(Protocol):
    """Base protocol for all operations."""

    def execute(self, *args, **kwargs) -> Any:
        """Execute the operation."""
        ...
@runtime_checkable


class IMergeOperation(IOperation, Protocol):
    """Protocol for merge operations."""

    def merge(
        self,
        target: Any,
        source: Any,
        strategy: MergeStrategy = MergeStrategy.DEEP
    ) -> Any:
        """Merge source into target."""
        ...
@runtime_checkable


class IDiffOperation(IOperation, Protocol):
    """Protocol for diff operations."""

    def diff(
        self,
        original: Any,
        modified: Any,
        mode: DiffMode = DiffMode.FULL
    ) -> DiffResult:
        """Generate diff between original and modified."""
        ...
@runtime_checkable


class IPatchOperation(IOperation, Protocol):
    """Protocol for patch operations."""

    def apply_patch(
        self,
        data: Any,
        operations: list[dict[str, Any]]
    ) -> PatchResult:
        """Apply patch operations to data."""
        ...

    def patch(
        self,
        data: Any,
        operations: list[dict[str, Any]]
    ) -> Any:
        """Apply patch operations to data."""
        ...
