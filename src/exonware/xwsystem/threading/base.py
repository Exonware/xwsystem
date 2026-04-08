#exonware/xwsystem/src/exonware/xwsystem/threading/base.py
#exonware/xwsystem/threading/base.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.35
Generation Date: September 04, 2025
Threading module base classes - abstract classes for threading functionality.
"""

from abc import ABC, abstractmethod
from typing import Any

from collections.abc import Callable, Coroutine
from .contracts import ThreadState, LockType, AsyncPrimitiveType, ConcurrencyMode


class AThreadBase(ABC):
    """Abstract base class for thread operations."""

    def __init__(self, name: str | None = None, daemon: bool = False):
        """
        Initialize thread base.
        Args:
            name: Thread name
            daemon: Whether thread is daemon
        """
        self.name = name
        self.daemon = daemon
        self._state = ThreadState.NEW
        self._target: Callable | None = None
        self._args: tuple = ()
        self._kwargs: dict[str, Any] = {}
    @abstractmethod

    def start(self) -> None:
        """Start thread."""
        pass
    @abstractmethod

    def stop(self) -> None:
        """Stop thread."""
        pass
    @abstractmethod

    def join(self, timeout: float | None = None) -> None:
        """Join thread."""
        pass
    @abstractmethod

    def is_alive(self) -> bool:
        """Check if thread is alive."""
        pass
    @abstractmethod

    def is_running(self) -> bool:
        """Check if thread is running."""
        pass
    @abstractmethod

    def get_state(self) -> ThreadState:
        """Get thread state."""
        pass
    @abstractmethod

    def get_ident(self) -> int:
        """Get thread identifier."""
        pass
    @abstractmethod

    def set_target(self, target: Callable, *args, **kwargs) -> None:
        """Set thread target function."""
        pass
    @abstractmethod

    def get_result(self) -> Any:
        """Get thread result."""
        pass
    @abstractmethod

    def get_exception(self) -> Exception | None:
        """Get thread exception."""
        pass


class ALockBase(ABC):
    """Abstract base class for lock operations."""

    def __init__(self, lock_type: LockType = LockType.MUTEX):
        """
        Initialize lock base.
        Args:
            lock_type: Type of lock
        """
        self.lock_type = lock_type
        self._locked = False
        self._owner: int | None = None
        self._waiting_threads: list[int] = []
    @abstractmethod

    def acquire(self, blocking: bool = True, timeout: float | None = None) -> bool:
        """Acquire lock."""
        pass
    @abstractmethod

    def release(self) -> None:
        """Release lock."""
        pass
    @abstractmethod

    def is_locked(self) -> bool:
        """Check if lock is locked."""
        pass
    @abstractmethod

    def is_owned(self) -> bool:
        """Check if lock is owned by current thread."""
        pass
    @abstractmethod

    def get_owner(self) -> int | None:
        """Get lock owner thread ID."""
        pass
    @abstractmethod

    def get_waiting_count(self) -> int:
        """Get number of waiting threads."""
        pass
    @abstractmethod

    def try_acquire(self) -> bool:
        """Try to acquire lock without blocking."""
        pass
    @abstractmethod

    def __enter__(self):
        """Context manager entry."""
        pass
    @abstractmethod

    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit."""
        pass


class AAsyncPrimitiveBase(ABC):
    """Abstract base class for async primitives."""

    def __init__(self, primitive_type: AsyncPrimitiveType = AsyncPrimitiveType.EVENT):
        """
        Initialize async primitive.
        Args:
            primitive_type: Type of async primitive
        """
        self.primitive_type = primitive_type
        self._state = False
        self._waiting_coroutines: list[Coroutine] = []
    @abstractmethod

    async def wait(self, timeout: float | None = None) -> bool:
        """Wait for primitive."""
        pass
    @abstractmethod

    async def set(self) -> None:
        """Set primitive."""
        pass
    @abstractmethod

    async def clear(self) -> None:
        """Clear primitive."""
        pass
    @abstractmethod

    async def is_set(self) -> bool:
        """Check if primitive is set."""
        pass
    @abstractmethod

    async def wait_for(self, condition: Callable, timeout: float | None = None) -> bool:
        """Wait for condition."""
        pass
    @abstractmethod

    async def notify(self) -> None:
        """Notify waiting coroutines."""
        pass
    @abstractmethod

    async def notify_all(self) -> None:
        """Notify all waiting coroutines."""
        pass
    @abstractmethod

    async def get_waiting_count(self) -> int:
        """Get number of waiting coroutines."""
        pass


class ASafeFactoryBase(ABC):
    """Abstract base class for safe factory operations."""

    def __init__(self, concurrency_mode: ConcurrencyMode = ConcurrencyMode.THREAD_SAFE):
        """
        Initialize safe factory.
        Args:
            concurrency_mode: Concurrency mode
        """
        self.concurrency_mode = concurrency_mode
        self._instances: dict[str, Any] = {}
        self._locks: dict[str, ALockBase] = {}
    @abstractmethod

    def create_instance(self, instance_type: str, *args, **kwargs) -> Any:
        """Create thread-safe instance."""
        pass
    @abstractmethod

    def get_instance(self, instance_id: str) -> Any | None:
        """Get instance by ID."""
        pass
    @abstractmethod

    def remove_instance(self, instance_id: str) -> bool:
        """Remove instance."""
        pass
    @abstractmethod

    def list_instances(self) -> list[str]:
        """List all instance IDs."""
        pass
    @abstractmethod

    def clear_instances(self) -> None:
        """Clear all instances."""
        pass
    @abstractmethod

    def get_instance_count(self) -> int:
        """Get instance count."""
        pass
    @abstractmethod

    def is_instance_exists(self, instance_id: str) -> bool:
        """Check if instance exists."""
        pass
    @abstractmethod

    def get_instance_info(self, instance_id: str) -> dict[str, Any]:
        """Get instance information."""
        pass


class AThreadPoolBase(ABC):
    """Abstract base class for thread pool operations."""

    def __init__(self, max_workers: int = 4):
        """
        Initialize thread pool.
        Args:
            max_workers: Maximum number of worker threads
        """
        self.max_workers = max_workers
        self._workers: list[AThreadBase] = []
        self._task_queue: list[tuple] = []
        self._result_queue: list[Any] = []
        self._shutdown = False
    @abstractmethod

    def submit(self, func: Callable, *args, **kwargs) -> Any:
        """Submit task to thread pool."""
        pass
    @abstractmethod

    def map(self, func: Callable, iterable: list[Any]) -> list[Any]:
        """Map function over iterable using thread pool."""
        pass
    @abstractmethod

    def shutdown(self, wait: bool = True) -> None:
        """Shutdown thread pool."""
        pass
    @abstractmethod

    def is_shutdown(self) -> bool:
        """Check if thread pool is shutdown."""
        pass
    @abstractmethod

    def get_active_count(self) -> int:
        """Get number of active threads."""
        pass
    @abstractmethod

    def get_pending_count(self) -> int:
        """Get number of pending tasks."""
        pass
    @abstractmethod

    def get_completed_count(self) -> int:
        """Get number of completed tasks."""
        pass
    @abstractmethod

    def get_worker_count(self) -> int:
        """Get number of worker threads."""
        pass


class AConcurrencyManagerBase(ABC):
    """Abstract base class for concurrency management."""

    def __init__(self):
        """Initialize concurrency manager."""
        self._threads: dict[str, AThreadBase] = {}
        self._locks: dict[str, ALockBase] = {}
        self._async_primitives: dict[str, AAsyncPrimitiveBase] = {}
        self._deadlock_detector_enabled = False
    @abstractmethod

    def create_thread(self, name: str, target: Callable, *args, **kwargs) -> AThreadBase:
        """Create new thread."""
        pass
    @abstractmethod

    def create_lock(self, name: str, lock_type: LockType = LockType.MUTEX) -> ALockBase:
        """Create new lock."""
        pass
    @abstractmethod

    def create_async_primitive(self, name: str, primitive_type: AsyncPrimitiveType) -> AAsyncPrimitiveBase:
        """Create new async primitive."""
        pass
    @abstractmethod

    def get_thread(self, name: str) -> AThreadBase | None:
        """Get thread by name."""
        pass
    @abstractmethod

    def get_lock(self, name: str) -> ALockBase | None:
        """Get lock by name."""
        pass
    @abstractmethod

    def get_async_primitive(self, name: str) -> AAsyncPrimitiveBase | None:
        """Get async primitive by name."""
        pass
    @abstractmethod

    def remove_thread(self, name: str) -> bool:
        """Remove thread."""
        pass
    @abstractmethod

    def remove_lock(self, name: str) -> bool:
        """Remove lock."""
        pass
    @abstractmethod

    def remove_async_primitive(self, name: str) -> bool:
        """Remove async primitive."""
        pass
    @abstractmethod

    def detect_deadlocks(self) -> list[list[str]]:
        """Detect potential deadlocks."""
        pass
    @abstractmethod

    def enable_deadlock_detection(self) -> None:
        """Enable deadlock detection."""
        pass
    @abstractmethod

    def disable_deadlock_detection(self) -> None:
        """Disable deadlock detection."""
        pass
    @abstractmethod

    def get_concurrency_stats(self) -> dict[str, Any]:
        """Get concurrency statistics."""
        pass
    @abstractmethod

    def shutdown_all(self) -> None:
        """Shutdown all managed resources."""
        pass
