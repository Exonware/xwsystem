#!/usr/bin/env python3
#exonware/xwsystem/src/exonware/xwsystem/monitoring/contracts.py
"""
Company: eXonware.com
Author: eXonware Backend Team
Email: connect@exonware.com
Version: 0.9.0.14
Generation Date: September 04, 2025
Monitoring protocol interfaces for XWSystem.
"""

from typing import Any, Protocol, runtime_checkable

from collections.abc import Iterator, Callable
import time
# Import enums from types module
from .defs import (
    MetricType,
    HealthStatus,
    AlertLevel,
    MonitoringMode,
    PerformanceLevel,
    CircuitState
)
# ============================================================================
# PERFORMANCE INTERFACES
# ============================================================================
@runtime_checkable

class IPerformance(Protocol):
    """
    Interface for performance monitoring.
    Enforces consistent performance monitoring across XWSystem.
    """

    def start_timer(self, operation: str) -> str:
        """
        Start performance timer.
        Args:
            operation: Operation name
        Returns:
            Timer ID
        """
        ...

    def end_timer(self, timer_id: str) -> float:
        """
        End performance timer.
        Args:
            timer_id: Timer ID
        Returns:
            Elapsed time in seconds
        """
        ...

    def get_metrics(self) -> dict[str, Any]:
        """
        Get performance metrics.
        Returns:
            Performance metrics dictionary
        """
        ...

    def reset_metrics(self) -> None:
        """
        Reset performance metrics.
        """
        ...

    def record_metric(self, name: str, value: float, metric_type: MetricType = MetricType.GAUGE) -> None:
        """
        Record performance metric.
        Args:
            name: Metric name
            value: Metric value
            metric_type: Type of metric
        """
        ...

    def get_metric(self, name: str) -> float | None:
        """
        Get performance metric value.
        Args:
            name: Metric name
        Returns:
            Metric value or None
        """
        ...

    def get_performance_level(self) -> PerformanceLevel:
        """
        Get current performance level.
        Returns:
            Current performance level
        """
        ...

    def set_performance_threshold(self, metric: str, threshold: float) -> None:
        """
        Set performance threshold.
        Args:
            metric: Metric name
            threshold: Threshold value
        """
        ...

    def is_performance_acceptable(self) -> bool:
        """
        Check if performance is acceptable.
        Returns:
            True if performance is acceptable
        """
        ...
# ============================================================================
# MONITORABLE INTERFACES
# ============================================================================
@runtime_checkable

class IMonitorable(Protocol):
    """
    Interface for monitorable objects.
    Enforces consistent monitoring behavior across XWSystem.
    """

    def start_monitoring(self) -> None:
        """
        Start monitoring.
        """
        ...

    def stop_monitoring(self) -> None:
        """
        Stop monitoring.
        """
        ...

    def get_status(self) -> HealthStatus:
        """
        Get current status.
        Returns:
            Current health status
        """
        ...

    def get_health(self) -> dict[str, Any]:
        """
        Get health information.
        Returns:
            Health information dictionary
        """
        ...

    def is_monitoring(self) -> bool:
        """
        Check if monitoring is active.
        Returns:
            True if monitoring is active
        """
        ...

    def get_monitoring_info(self) -> dict[str, Any]:
        """
        Get monitoring information.
        Returns:
            Monitoring information dictionary
        """
        ...

    def set_monitoring_interval(self, interval: float) -> None:
        """
        Set monitoring interval.
        Args:
            interval: Monitoring interval in seconds
        """
        ...

    def get_monitoring_interval(self) -> float:
        """
        Get monitoring interval.
        Returns:
            Monitoring interval in seconds
        """
        ...
# ============================================================================
# METRICS INTERFACES
# ============================================================================
@runtime_checkable

class IMetrics(Protocol):
    """
    Interface for metrics collection.
    Enforces consistent metrics behavior across XWSystem.
    """

    def collect_metrics(self) -> dict[str, Any]:
        """
        Collect all metrics.
        Returns:
            Dictionary of collected metrics
        """
        ...

    def add_metric(self, name: str, value: Any, labels: dict[str, str] | None = None) -> None:
        """
        Add metric.
        Args:
            name: Metric name
            value: Metric value
            labels: Optional metric labels
        """
        ...

    def get_metric(self, name: str) -> Any | None:
        """
        Get metric value.
        Args:
            name: Metric name
        Returns:
            Metric value or None
        """
        ...

    def remove_metric(self, name: str) -> bool:
        """
        Remove metric.
        Args:
            name: Metric name to remove
        Returns:
            True if removed
        """
        ...

    def list_metrics(self) -> list[str]:
        """
        List all metric names.
        Returns:
            List of metric names
        """
        ...

    def export_metrics(self, format: str = "json") -> str:
        """
        Export metrics in specified format.
        Args:
            format: Export format
        Returns:
            Exported metrics string
        """
        ...

    def clear_metrics(self) -> None:
        """
        Clear all metrics.
        """
        ...

    def get_metrics_summary(self) -> dict[str, Any]:
        """
        Get metrics summary.
        Returns:
            Metrics summary dictionary
        """
        ...
# ============================================================================
# HEALTH CHECK INTERFACES
# ============================================================================
@runtime_checkable

class IHealthCheck(Protocol):
    """
    Interface for health checks.
    Enforces consistent health checking across XWSystem.
    """

    def check_health(self) -> HealthStatus:
        """
        Perform health check.
        Returns:
            Health status
        """
        ...

    def get_health_details(self) -> dict[str, Any]:
        """
        Get detailed health information.
        Returns:
            Health details dictionary
        """
        ...

    def add_health_check(self, name: str, check_func: Callable[[], HealthStatus]) -> None:
        """
        Add health check function.
        Args:
            name: Health check name
            check_func: Health check function
        """
        ...

    def remove_health_check(self, name: str) -> bool:
        """
        Remove health check.
        Args:
            name: Health check name
        Returns:
            True if removed
        """
        ...

    def list_health_checks(self) -> list[str]:
        """
        List all health check names.
        Returns:
            List of health check names
        """
        ...

    def run_health_checks(self) -> dict[str, HealthStatus]:
        """
        Run all health checks.
        Returns:
            Dictionary of health check results
        """
        ...

    def get_overall_health(self) -> HealthStatus:
        """
        Get overall health status.
        Returns:
            Overall health status
        """
        ...

    def set_health_threshold(self, check_name: str, threshold: float) -> None:
        """
        Set health check threshold.
        Args:
            check_name: Health check name
            threshold: Threshold value
        """
        ...
# ============================================================================
# ALERTING INTERFACES
# ============================================================================
@runtime_checkable

class IAlerting(Protocol):
    """
    Interface for alerting.
    Enforces consistent alerting behavior across XWSystem.
    """

    def create_alert(self, message: str, level: AlertLevel, source: str = "") -> str:
        """
        Create alert.
        Args:
            message: Alert message
            level: Alert level
            source: Alert source
        Returns:
            Alert ID
        """
        ...

    def get_alert(self, alert_id: str) -> dict[str, Any] | None:
        """
        Get alert by ID.
        Args:
            alert_id: Alert ID
        Returns:
            Alert information or None
        """
        ...

    def list_alerts(self, level: AlertLevel | None = None) -> list[dict[str, Any]]:
        """
        List alerts.
        Args:
            level: Filter by alert level
        Returns:
            List of alert information
        """
        ...

    def acknowledge_alert(self, alert_id: str, user: str = "") -> bool:
        """
        Acknowledge alert.
        Args:
            alert_id: Alert ID
            user: User acknowledging alert
        Returns:
            True if acknowledged
        """
        ...

    def resolve_alert(self, alert_id: str, resolution: str = "") -> bool:
        """
        Resolve alert.
        Args:
            alert_id: Alert ID
            resolution: Resolution description
        Returns:
            True if resolved
        """
        ...

    def clear_alert(self, alert_id: str) -> bool:
        """
        Clear alert.
        Args:
            alert_id: Alert ID
        Returns:
            True if cleared
        """
        ...

    def get_alert_stats(self) -> dict[str, int]:
        """
        Get alert statistics.
        Returns:
            Alert statistics dictionary
        """
        ...

    def set_alert_threshold(self, metric: str, threshold: float, level: AlertLevel) -> None:
        """
        Set alert threshold.
        Args:
            metric: Metric name
            threshold: Threshold value
            level: Alert level
        """
        ...
# ============================================================================
# SYSTEM MONITORING INTERFACES
# ============================================================================
@runtime_checkable

class ISystemMonitor(Protocol):
    """
    Interface for system monitoring.
    Enforces consistent system monitoring across XWSystem.
    """

    def get_cpu_usage(self) -> float:
        """
        Get CPU usage percentage.
        Returns:
            CPU usage percentage
        """
        ...

    def get_memory_usage(self) -> dict[str, Any]:
        """
        Get memory usage information.
        Returns:
            Memory usage dictionary
        """
        ...

    def get_disk_usage(self) -> dict[str, Any]:
        """
        Get disk usage information.
        Returns:
            Disk usage dictionary
        """
        ...

    def get_network_usage(self) -> dict[str, Any]:
        """
        Get network usage information.
        Returns:
            Network usage dictionary
        """
        ...

    def get_process_info(self) -> list[dict[str, Any]]:
        """
        Get process information.
        Returns:
            List of process information
        """
        ...

    def get_system_load(self) -> float:
        """
        Get system load average.
        Returns:
            System load average
        """
        ...

    def get_uptime(self) -> float:
        """
        Get system uptime.
        Returns:
            Uptime in seconds
        """
        ...

    def get_system_info(self) -> dict[str, Any]:
        """
        Get system information.
        Returns:
            System information dictionary
        """
        ...
# ============================================================================
# PERFORMANCE PROFILING INTERFACES
# ============================================================================
@runtime_checkable

class IProfiler(Protocol):
    """
    Interface for performance profiling.
    Enforces consistent profiling behavior across XWSystem.
    """

    def start_profiling(self, name: str) -> str:
        """
        Start profiling session.
        Args:
            name: Profiling session name
        Returns:
            Profiling session ID
        """
        ...

    def stop_profiling(self, session_id: str) -> dict[str, Any]:
        """
        Stop profiling session.
        Args:
            session_id: Profiling session ID
        Returns:
            Profiling results
        """
        ...

    def profile_function(self, func: Callable, *args, **kwargs) -> tuple[Any, dict[str, Any]]:
        """
        Profile function execution.
        Args:
            func: Function to profile
            *args: Function arguments
            **kwargs: Function keyword arguments
        Returns:
            Tuple of (result, profiling_data)
        """
        ...

    def get_profiling_results(self, session_id: str) -> dict[str, Any] | None:
        """
        Get profiling results.
        Args:
            session_id: Profiling session ID
        Returns:
            Profiling results or None
        """
        ...

    def list_profiling_sessions(self) -> list[str]:
        """
        List profiling sessions.
        Returns:
            List of session IDs
        """
        ...

    def clear_profiling_data(self) -> None:
        """
        Clear profiling data.
        """
        ...

    def export_profiling_data(self, session_id: str, format: str = "json") -> str:
        """
        Export profiling data.
        Args:
            session_id: Profiling session ID
            format: Export format
        Returns:
            Exported profiling data
        """
        ...
# ============================================================================
# MONITORING CONFIGURATION INTERFACES
# ============================================================================
@runtime_checkable

class IMonitoringConfig(Protocol):
    """
    Interface for monitoring configuration.
    Enforces consistent monitoring configuration across XWSystem.
    """

    def set_monitoring_mode(self, mode: MonitoringMode) -> None:
        """
        Set monitoring mode.
        Args:
            mode: Monitoring mode
        """
        ...

    def get_monitoring_mode(self) -> MonitoringMode:
        """
        Get monitoring mode.
        Returns:
            Current monitoring mode
        """
        ...

    def set_metric_interval(self, metric: str, interval: float) -> None:
        """
        Set metric collection interval.
        Args:
            metric: Metric name
            interval: Collection interval in seconds
        """
        ...

    def get_metric_interval(self, metric: str) -> float | None:
        """
        Get metric collection interval.
        Args:
            metric: Metric name
        Returns:
            Collection interval or None
        """
        ...

    def enable_metric(self, metric: str) -> None:
        """
        Enable metric collection.
        Args:
            metric: Metric name
        """
        ...

    def disable_metric(self, metric: str) -> None:
        """
        Disable metric collection.
        Args:
            metric: Metric name
        """
        ...

    def is_metric_enabled(self, metric: str) -> bool:
        """
        Check if metric is enabled.
        Args:
            metric: Metric name
        Returns:
            True if enabled
        """
        ...

    def get_monitoring_config(self) -> dict[str, Any]:
        """
        Get monitoring configuration.
        Returns:
            Monitoring configuration dictionary
        """
        ...

    def set_monitoring_config(self, config: dict[str, Any]) -> None:
        """
        Set monitoring configuration.
        Args:
            config: Monitoring configuration
        """
        ...
# ============================================================================
# MONITORING PROTOCOLS
# ============================================================================
@runtime_checkable

class IMonitorableSimple(Protocol):
    """Protocol for objects that support performance monitoring (simpler interface than IMonitorable)."""

    def start_monitoring(self) -> None:
        """Start performance monitoring."""
        ...

    def stop_monitoring(self) -> None:
        """Stop performance monitoring."""
        ...

    def get_stats(self) -> dict[str, Any]:
        """Get performance statistics."""
        ...
