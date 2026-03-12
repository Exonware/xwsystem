#exonware/xwsystem/src/exonware/xwsystem/threading/facade.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.2
Generation Date: January 2026
XWConcurrency - Unified Concurrency Facade
Simplified API for threading and concurrency:
- Thread-safe factories
- Async primitives
- Thread pools
- Process pools
"""

from typing import Any, Optional, Callable, List
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor
from .safe_factory import ThreadSafeFactory, MethodGenerator
from .async_primitives import (
    AsyncLock,
    AsyncSemaphore,
    AsyncEvent,
    AsyncQueue,
    AsyncCondition,
    AsyncResourcePool,
)
from ..config.logging_setup import get_logger
logger = get_logger(__name__)


class XWConcurrency:
    """
    Unified concurrency facade - simple API for threading operations.
    Examples:
        >>> # Thread-safe factory
        >>> factory = XWConcurrency.Factory()
        >>> factory.register("handler", MyHandler, thread_safe=True)
        >>> # Async primitives
        >>> lock = XWConcurrency.Lock()
        >>> queue = XWConcurrency.Queue()
        >>> semaphore = XWConcurrency.Semaphore(5)
        >>> # Thread pool
        >>> with XWConcurrency.Pool(workers=4) as pool:
        ...     results = pool.map(process_item, items)
        >>> # Process pool
        >>> with XWConcurrency.ProcessPool(workers=4) as pool:
        ...     results = pool.map(cpu_intensive_task, items)
    """
    class Factory:
        """Thread-safe factory."""
        def __init__(self):
            self._factory = ThreadSafeFactory()
        def register(self, name: str, handler_class: type, thread_safe: bool = True) -> None:
            """Register handler with factory."""
            # ThreadSafeFactory.register doesn't take thread_safe parameter
            self._factory.register(name, handler_class)
        def get(self, name: str, *args, **kwargs) -> Any:
            """Get handler instance."""
            # ThreadSafeFactory uses create() method, not get()
            handler_class = self._factory.get_handler(name)
            if handler_class:
                return handler_class(*args, **kwargs)
            return None
    @staticmethod

    def Lock() -> AsyncLock:
        """Create async lock."""
        return AsyncLock()
    @staticmethod

    def Semaphore(value: int = 1) -> AsyncSemaphore:
        """Create async semaphore."""
        return AsyncSemaphore(value)
    @staticmethod

    def Event() -> AsyncEvent:
        """Create async event."""
        return AsyncEvent()
    @staticmethod

    def Queue(maxsize: int = 0) -> AsyncQueue:
        """Create async queue."""
        return AsyncQueue(maxsize)
    @staticmethod

    def Condition(lock: Optional[AsyncLock] = None) -> AsyncCondition:
        """Create async condition."""
        return AsyncCondition(lock)
    @staticmethod

    def ResourcePool(factory: Callable, max_size: int = 10) -> AsyncResourcePool:
        """Create async resource pool."""
        return AsyncResourcePool(factory, max_size)
    class Pool:
        """Thread pool executor."""
        def __init__(self, workers: int = 4):
            """
            Initialize thread pool.
            Args:
                workers: Number of worker threads
            """
            self.workers = workers
            self._pool: Optional[ThreadPoolExecutor] = None
        def __enter__(self):
            """Context manager entry."""
            self._pool = ThreadPoolExecutor(max_workers=self.workers)
            return self
        def __exit__(self, exc_type, exc_val, exc_tb):
            """Context manager exit."""
            if self._pool:
                self._pool.shutdown(wait=True)
        def map(self, func: Callable, items: List[Any]) -> List[Any]:
            """Map function over items using thread pool."""
            if not self._pool:
                raise RuntimeError("Pool not started. Use as context manager.")
            return list(self._pool.map(func, items))
        def submit(self, func: Callable, *args, **kwargs):
            """Submit task to thread pool."""
            if not self._pool:
                raise RuntimeError("Pool not started. Use as context manager.")
            return self._pool.submit(func, *args, **kwargs)
    class ProcessPool:
        """Process pool executor."""
        def __init__(self, workers: int = 4):
            """
            Initialize process pool.
            Args:
                workers: Number of worker processes
            """
            self.workers = workers
            self._pool: Optional[ProcessPoolExecutor] = None
        def __enter__(self):
            """Context manager entry."""
            self._pool = ProcessPoolExecutor(max_workers=self.workers)
            return self
        def __exit__(self, exc_type, exc_val, exc_tb):
            """Context manager exit."""
            if self._pool:
                self._pool.shutdown(wait=True)
        def map(self, func: Callable, items: List[Any]) -> List[Any]:
            """Map function over items using process pool."""
            if not self._pool:
                raise RuntimeError("Pool not started. Use as context manager.")
            return list(self._pool.map(func, items))
        def submit(self, func: Callable, *args, **kwargs):
            """Submit task to process pool."""
            if not self._pool:
                raise RuntimeError("Pool not started. Use as context manager.")
            return self._pool.submit(func, *args, **kwargs)
