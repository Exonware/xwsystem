#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/threading/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.10
Generation Date: September 04, 2025
Threading protocol interfaces for XWSystem.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import Iterator, Callable, Coroutine, Awaitable
import threading
import asyncio
# Import enums from types module
from .defs import (
    ThreadState,
    LockType,
    ThreadPriority,
    AsyncState,
    ConcurrencyMode
)
# ============================================================================
# LOCK INTERFACES
# ============================================================================
@runtime_checkable

class ILockable(Protocol):
    """
    Interface for lockable objects.
    Enforces consistent locking behavior across XWSystem.
    """

    def acquire_lock(self, blocking: bool = True, timeout: float | None = None) -> bool:
        """
        Acquire lock.
        Args:
            blocking: Whether to block until lock is acquired
            timeout: Timeout in seconds
        Returns:
            True if lock acquired
        """
        ...

    def release_lock(self) -> None:
        """
        Release lock.
        """
        ...

    def is_locked(self) -> bool:
        """
        Check if object is locked.
        Returns:
            True if locked
        """
        ...

    def try_lock(self, timeout: float | None = None) -> bool:
        """
        Try to acquire lock without blocking.
        Args:
            timeout: Timeout in seconds
        Returns:
            True if lock acquired
        """
        ...

    def get_lock_type(self) -> LockType:
        """
        Get lock type.
        Returns:
            Lock type
        """
        ...

    def get_lock_owner(self) -> str | None:
        """
        Get lock owner thread ID.
        Returns:
            Thread ID of lock owner or None
        """
        ...

    def get_lock_count(self) -> int:
        """
        Get lock acquisition count.
        Returns:
            Number of times lock has been acquired
        """
        ...
# ============================================================================
# ASYNC INTERFACES
# ============================================================================
@runtime_checkable

class IAsync(Protocol):
    """
    Interface for async operations.
    Enforces consistent async behavior across XWSystem.
    """

    async def async_method(self, *args, **kwargs) -> Any:
        """
        Execute method asynchronously.
        Args:
            *args: Positional arguments
            **kwargs: Keyword arguments
        Returns:
            Method result
        """
        ...

    def await_result(self, coroutine: Coroutine) -> Any:
        """
        Await coroutine result.
        Args:
            coroutine: Coroutine to await
        Returns:
            Coroutine result
        """
        ...

    def is_async(self) -> bool:
        """
        Check if object supports async operations.
        Returns:
            True if async supported
        """
        ...

    def get_future(self) -> asyncio.Future:
        """
        Get future for async operation.
        Returns:
            Future object
        """
        ...

    def get_async_state(self) -> AsyncState:
        """
        Get async operation state.
        Returns:
            Current async state
        """
        ...

    def cancel_async(self) -> bool:
        """
        Cancel async operation.
        Returns:
            True if cancelled
        """
        ...

    def is_async_completed(self) -> bool:
        """
        Check if async operation is completed.
        Returns:
            True if completed
        """
        ...

    def get_async_result(self) -> Any:
        """
        Get async operation result.
        Returns:
            Async operation result
        """
        ...
# ============================================================================
# THREAD SAFETY INTERFACES
# ============================================================================
@runtime_checkable

class IThreadSafe(Protocol):
    """
    Interface for thread-safe objects.
    Enforces consistent thread safety across XWSystem.
    """

    def thread_safe_method(self, *args, **kwargs) -> Any:
        """
        Execute method in thread-safe manner.
        Args:
            *args: Positional arguments
            **kwargs: Keyword arguments
        Returns:
            Method result
        """
        ...

    def get_thread_id(self) -> int:
        """
        Get current thread ID.
        Returns:
            Thread ID
        """
        ...

    def is_thread_safe(self) -> bool:
        """
        Check if object is thread-safe.
        Returns:
            True if thread-safe
        """
        ...

    def get_thread_count(self) -> int:
        """
        Get number of active threads.
        Returns:
            Number of active threads
        """
        ...

    def get_thread_info(self) -> dict[str, Any]:
        """
        Get thread information.
        Returns:
            Thread information dictionary
        """
        ...

    def wait_for_threads(self, timeout: float | None = None) -> bool:
        """
        Wait for all threads to complete.
        Args:
            timeout: Timeout in seconds
        Returns:
            True if all threads completed
        """
        ...

    def interrupt_threads(self) -> None:
        """
        Interrupt all threads.
        """
        ...

    def join_threads(self) -> None:
        """
        Join all threads.
        """
        ...
# ============================================================================
# THREAD POOL INTERFACES
# ============================================================================
@runtime_checkable

class IThreadPool(Protocol):
    """
    Interface for thread pool management.
    Enforces consistent thread pool behavior across XWSystem.
    """

    def submit_task(self, func: Callable, *args, **kwargs) -> Any:
        """
        Submit task to thread pool.
        Args:
            func: Function to execute
            *args: Function arguments
            **kwargs: Function keyword arguments
        Returns:
            Task result or future
        """
        ...

    def submit_async_task(self, func: Callable, *args, **kwargs) -> asyncio.Future:
        """
        Submit async task to thread pool.
        Args:
            func: Async function to execute
            *args: Function arguments
            **kwargs: Function keyword arguments
        Returns:
            Future object
        """
        ...

    def get_pool_size(self) -> int:
        """
        Get thread pool size.
        Returns:
            Number of threads in pool
        """
        ...

    def set_pool_size(self, size: int) -> None:
        """
        Set thread pool size.
        Args:
            size: New pool size
        """
        ...

    def get_active_count(self) -> int:
        """
        Get number of active threads.
        Returns:
            Number of active threads
        """
        ...

    def get_queue_size(self) -> int:
        """
        Get task queue size.
        Returns:
            Number of queued tasks
        """
        ...

    def shutdown(self, wait: bool = True) -> None:
        """
        Shutdown thread pool.
        Args:
            wait: Whether to wait for tasks to complete
        """
        ...

    def is_shutdown(self) -> bool:
        """
        Check if thread pool is shutdown.
        Returns:
            True if shutdown
        """
        ...
# ============================================================================
# CONCURRENCY CONTROL INTERFACES
# ============================================================================
@runtime_checkable

class IConcurrencyControl(Protocol):
    """
    Interface for concurrency control.
    Enforces consistent concurrency control across XWSystem.
    """

    def acquire_resource(self, resource_id: str, timeout: float | None = None) -> bool:
        """
        Acquire resource for exclusive access.
        Args:
            resource_id: Resource identifier
            timeout: Timeout in seconds
        Returns:
            True if resource acquired
        """
        ...

    def release_resource(self, resource_id: str) -> None:
        """
        Release resource.
        Args:
            resource_id: Resource identifier
        """
        ...

    def is_resource_available(self, resource_id: str) -> bool:
        """
        Check if resource is available.
        Args:
            resource_id: Resource identifier
        Returns:
            True if available
        """
        ...

    def get_resource_owner(self, resource_id: str) -> str | None:
        """
        Get resource owner.
        Args:
            resource_id: Resource identifier
        Returns:
            Owner thread ID or None
        """
        ...

    def wait_for_resource(self, resource_id: str, timeout: float | None = None) -> bool:
        """
        Wait for resource to become available.
        Args:
            resource_id: Resource identifier
            timeout: Timeout in seconds
        Returns:
            True if resource became available
        """
        ...

    def get_concurrency_mode(self) -> ConcurrencyMode:
        """
        Get concurrency mode.
        Returns:
            Current concurrency mode
        """
        ...

    def set_concurrency_mode(self, mode: ConcurrencyMode) -> None:
        """
        Set concurrency mode.
        Args:
            mode: Concurrency mode to set
        """
        ...
# ============================================================================
# SYNCHRONIZATION INTERFACES
# ============================================================================
@runtime_checkable

class ISynchronization(Protocol):
    """
    Interface for synchronization primitives.
    Enforces consistent synchronization across XWSystem.
    """

    def wait(self, timeout: float | None = None) -> bool:
        """
        Wait for condition.
        Args:
            timeout: Timeout in seconds
        Returns:
            True if condition met
        """
        ...

    def notify(self) -> None:
        """
        Notify waiting threads.
        """
        ...

    def notify_all(self) -> None:
        """
        Notify all waiting threads.
        """
        ...

    def signal(self) -> None:
        """
        Signal condition.
        """
        ...

    def reset(self) -> None:
        """
        Reset synchronization primitive.
        """
        ...

    def is_set(self) -> bool:
        """
        Check if condition is set.
        Returns:
            True if set
        """
        ...

    def get_waiting_count(self) -> int:
        """
        Get number of waiting threads.
        Returns:
            Number of waiting threads
        """
        ...
# ============================================================================
# DEADLOCK DETECTION INTERFACES
# ============================================================================
@runtime_checkable

class IDeadlockDetection(Protocol):
    """
    Interface for deadlock detection.
    Enforces consistent deadlock detection across XWSystem.
    """

    def detect_deadlock(self) -> list[dict[str, Any]]:
        """
        Detect deadlocks.
        Returns:
            List of deadlock information
        """
        ...

    def is_deadlocked(self) -> bool:
        """
        Check if system is deadlocked.
        Returns:
            True if deadlocked
        """
        ...

    def resolve_deadlock(self, deadlock_info: dict[str, Any]) -> bool:
        """
        Resolve deadlock.
        Args:
            deadlock_info: Deadlock information
        Returns:
            True if resolved
        """
        ...

    def get_lock_graph(self) -> dict[str, list[str]]:
        """
        Get lock dependency graph.
        Returns:
            Lock dependency graph
        """
        ...

    def add_lock_dependency(self, resource1: str, resource2: str) -> None:
        """
        Add lock dependency.
        Args:
            resource1: First resource
            resource2: Second resource
        """
        ...

    def remove_lock_dependency(self, resource1: str, resource2: str) -> None:
        """
        Remove lock dependency.
        Args:
            resource1: First resource
            resource2: Second resource
        """
        ...

    def get_deadlock_prevention_mode(self) -> bool:
        """
        Get deadlock prevention mode.
        Returns:
            True if prevention enabled
        """
        ...

    def set_deadlock_prevention_mode(self, enabled: bool) -> None:
        """
        Set deadlock prevention mode.
        Args:
            enabled: Whether to enable prevention
        """
        ...
# ============================================================================
# THREAD MONITORING INTERFACES
# ============================================================================
@runtime_checkable

class IThreadMonitor(Protocol):
    """
    Interface for thread monitoring.
    Enforces consistent thread monitoring across XWSystem.
    """

    def get_thread_stats(self) -> dict[str, Any]:
        """
        Get thread statistics.
        Returns:
            Thread statistics dictionary
        """
        ...

    def get_thread_list(self) -> list[dict[str, Any]]:
        """
        Get list of all threads.
        Returns:
            List of thread information
        """
        ...

    def get_thread_by_id(self, thread_id: int) -> dict[str, Any] | None:
        """
        Get thread by ID.
        Args:
            thread_id: Thread ID
        Returns:
            Thread information or None
        """
        ...

    def monitor_thread_performance(self, thread_id: int) -> dict[str, Any]:
        """
        Monitor thread performance.
        Args:
            thread_id: Thread ID
        Returns:
            Performance metrics
        """
        ...

    def detect_thread_leaks(self) -> list[int]:
        """
        Detect thread leaks.
        Returns:
            List of leaked thread IDs
        """
        ...

    def cleanup_thread_leaks(self, thread_ids: list[int]) -> int:
        """
        Cleanup thread leaks.
        Args:
            thread_ids: Thread IDs to cleanup
        Returns:
            Number of threads cleaned up
        """
        ...

    def get_thread_priority(self, thread_id: int) -> ThreadPriority:
        """
        Get thread priority.
        Args:
            thread_id: Thread ID
        Returns:
            Thread priority
        """
        ...

    def set_thread_priority(self, thread_id: int, priority: ThreadPriority) -> bool:
        """
        Set thread priority.
        Args:
            thread_id: Thread ID
            priority: New priority
        Returns:
            True if set successfully
        """
        ...
# ============================================================================
# ASYNC CONTEXT MANAGER INTERFACES
# ============================================================================
@runtime_checkable

class IAsyncContextManager(Protocol):
    """
    Interface for async context managers.
    Enforces consistent async context management across XWSystem.
    """

    async def __aenter__(self) -> IAsyncContextManager:
        """
        Async context manager entry.
        Returns:
            Self
        """
        ...

    async def __aexit__(self, exc_type: type, exc_val: Exception, exc_tb: Any) -> bool:
        """
        Async context manager exit.
        Args:
            exc_type: Exception type
            exc_val: Exception value
            exc_tb: Exception traceback
        Returns:
            True if exception handled
        """
        ...

    def is_async_context_active(self) -> bool:
        """
        Check if async context is active.
        Returns:
            True if active
        """
        ...

    def get_async_context_info(self) -> dict[str, Any]:
        """
        Get async context information.
        Returns:
            Context information dictionary
        """
        ...
