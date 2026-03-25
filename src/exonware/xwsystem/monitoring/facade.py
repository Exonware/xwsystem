#exonware/xwsystem/src/exonware/xwsystem/monitoring/facade.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.19
Generation Date: January 2026
XWMonitor - Unified Monitoring Facade
Simplified API for monitoring operations:
- Performance monitoring
- Memory monitoring
- Circuit breakers
- System monitoring
"""

from typing import Any

from collections.abc import Callable
from contextlib import contextmanager
from functools import wraps
from .performance_monitor import (
    PerformanceMonitor,
    performance_context,
    create_performance_monitor,
)
from .memory_monitor import (
    MemoryMonitor,
    start_memory_monitoring,
    stop_memory_monitoring,
    get_memory_stats,
    get_memory_monitor,
)
from .error_recovery import (
    CircuitBreaker,
    CircuitBreakerConfig,
    circuit_breaker,
    retry_with_backoff,
)
from .system_monitor import (
    SystemMonitor,
    get_cpu_usage,
    get_memory_usage,
    get_system_info,
)
from ..config.logging_setup import get_logger
logger = get_logger(__name__)


class XWMonitor:
    """
    Unified monitoring facade - simple API for monitoring operations.
    Examples:
        >>> # Performance monitoring
        >>> with XWMonitor.performance("operation"):
        ...     result = expensive_operation()
        >>> # Memory monitoring
        >>> monitor = XWMonitor.memory(auto_cleanup=True)
        >>> monitor.start()
        >>> # Circuit breaker
        >>> @XWMonitor.circuit_breaker(failures=5, timeout=60)
        >>> def external_api_call():
        ...     return requests.get("https://api.example.com")
        >>> # System monitoring
        >>> cpu = XWMonitor.cpu_usage()
        >>> memory = XWMonitor.memory_usage()
        >>> disk = XWMonitor.disk_usage()
    """
    @staticmethod
    @contextmanager

    def performance(operation_name: str):
        """
        Context manager for performance monitoring.
        Args:
            operation_name: Name of the operation being monitored
        """
        monitor = create_performance_monitor()
        with performance_context(monitor, operation_name):
            yield
    @staticmethod

    def memory(auto_cleanup: bool = False) -> MemoryMonitor:
        """
        Create memory monitor instance.
        Args:
            auto_cleanup: Enable automatic memory cleanup
        Returns:
            MemoryMonitor instance
        """
        monitor = get_memory_monitor()
        if auto_cleanup:
            monitor.enable_auto_cleanup()
        return monitor
    @staticmethod

    def start_memory_monitoring(auto_cleanup: bool = False) -> None:
        """Start memory monitoring."""
        start_memory_monitoring(auto_cleanup=auto_cleanup)
    @staticmethod

    def stop_memory_monitoring() -> None:
        """Stop memory monitoring."""
        stop_memory_monitoring()
    @staticmethod

    def get_memory_stats() -> dict:
        """Get current memory statistics."""
        return get_memory_stats()
    @staticmethod

    def circuit_breaker(
        failures: int = 5,
        timeout: int = 60,
        **kwargs
    ):
        """
        Decorator for circuit breaker pattern.
        Args:
            failures: Number of failures before opening circuit
            timeout: Timeout in seconds before attempting recovery
            **kwargs: Additional circuit breaker options
        Returns:
            Decorator function
        """
        config = CircuitBreakerConfig(
            failure_threshold=failures,
            recovery_timeout=timeout,
            **kwargs
        )
        return circuit_breaker(config=config)
    @staticmethod

    def retry(max_attempts: int = 3, backoff: float = 1.0, **kwargs):
        """
        Decorator for retry logic.
        Args:
            max_attempts: Maximum retry attempts
            backoff: Backoff multiplier
            **kwargs: Additional retry options
        Returns:
            Decorator function
        """
        return retry_with_backoff(max_attempts=max_attempts, backoff=backoff, **kwargs)
    @staticmethod

    def cpu_usage() -> float:
        """Get current CPU usage percentage."""
        return get_cpu_usage()
    @staticmethod

    def memory_usage() -> dict:
        """Get current memory usage statistics."""
        return get_memory_usage()
    @staticmethod

    def disk_usage(path: str = "/") -> dict:
        """Get disk usage statistics."""
        import shutil
        usage = shutil.disk_usage(path)
        return {
            "total": usage.total,
            "used": usage.used,
            "free": usage.free,
            "percent": (usage.used / usage.total) * 100
        }
    @staticmethod

    def system_info() -> dict:
        """Get system information."""
        return get_system_info()
