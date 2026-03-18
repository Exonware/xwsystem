#exonware/xwsystem/src/exonware/xwsystem/ipc/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.9
Generation Date: September 04, 2025
IPC module contracts - interfaces and enums for inter-process communication.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import AsyncGenerator, Callable
from multiprocessing import Process
# Import enums from types module
from .defs import (
    IPCType,
    MessageType,
    ProcessState,
    QueueType,
    SharedMemoryType,
    MessageQueueType
)
@runtime_checkable

class IMessageQueue(Protocol):
    """Interface for message queue operations."""

    async def put(self, message: Any, message_type: MessageType = MessageType.DATA) -> None:
        """Put message in queue."""
        ...

    async def get(self, timeout: float | None = None) -> Any:
        """Get message from queue."""
        ...

    async def get_nowait(self) -> Any:
        """Get message without waiting."""
        ...

    def empty(self) -> bool:
        """Check if queue is empty."""
        ...

    def full(self) -> bool:
        """Check if queue is full."""
        ...

    def size(self) -> int:
        """Get queue size."""
        ...
@runtime_checkable

class IPipe(Protocol):
    """Interface for pipe operations."""

    async def send(self, data: Any) -> None:
        """Send data through pipe."""
        ...

    async def recv(self, timeout: float | None = None) -> Any:
        """Receive data from pipe."""
        ...

    async def recv_nowait(self) -> Any:
        """Receive data without waiting."""
        ...

    def close(self) -> None:
        """Close pipe."""
        ...

    def closed(self) -> bool:
        """Check if pipe is closed."""
        ...
@runtime_checkable

class ISharedMemory(Protocol):
    """Interface for shared memory operations."""

    def create(self, name: str, size: int) -> None:
        """Create shared memory segment."""
        ...

    def attach(self, name: str) -> None:
        """Attach to shared memory segment."""
        ...

    def detach(self) -> None:
        """Detach from shared memory segment."""
        ...

    def unlink(self) -> None:
        """Unlink shared memory segment."""
        ...

    def read(self, offset: int = 0, size: int | None = None) -> bytes:
        """Read from shared memory."""
        ...

    def write(self, data: bytes, offset: int = 0) -> None:
        """Write to shared memory."""
        ...
@runtime_checkable

class IProcessManager(Protocol):
    """Interface for process management."""

    def create_process(self, target: Callable, args: tuple = (), kwargs: dict | None = None) -> Process:
        """Create new process."""
        ...

    def start_process(self, process: Process) -> None:
        """Start process."""
        ...

    def stop_process(self, process: Process, timeout: float | None = None) -> None:
        """Stop process."""
        ...

    def get_process_state(self, process: Process) -> ProcessState:
        """Get process state."""
        ...

    def is_process_alive(self, process: Process) -> bool:
        """Check if process is alive."""
        ...
@runtime_checkable

class IProcessPool(Protocol):
    """Interface for process pool operations."""

    def submit(self, func: Callable, *args, **kwargs) -> Any:
        """Submit task to pool."""
        ...

    async def submit_async(self, func: Callable, *args, **kwargs) -> Any:
        """Submit async task to pool."""
        ...

    def map(self, func: Callable, iterable: list[Any]) -> list[Any]:
        """Map function over iterable."""
        ...

    async def map_async(self, func: Callable, iterable: list[Any]) -> list[Any]:
        """Map function over iterable asynchronously."""
        ...

    def close(self) -> None:
        """Close process pool."""
        ...

    def join(self) -> None:
        """Join all processes."""
        ...
