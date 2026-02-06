// #exonware/xwsystem/rust/src/monitoring/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Monitoring protocol interfaces for XWSystem.


use std::collections::HashMap;

use crate::defs::{AlertLevel, CircuitState, HealthStatus, MetricType, MonitoringMode, PerformanceLevel};

// ============================================================================

// MONITORABLE INTERFACES

// ============================================================================

// ============================================================================

// METRICS INTERFACES

// ============================================================================

// ============================================================================

// HEALTH CHECK INTERFACES

// ============================================================================

// ============================================================================

// ALERTING INTERFACES

// ============================================================================

// ============================================================================

// SYSTEM MONITORING INTERFACES

// ============================================================================

// ============================================================================

// PERFORMANCE PROFILING INTERFACES

// ============================================================================

// ============================================================================

// MONITORING CONFIGURATION INTERFACES

// ============================================================================

// ============================================================================

// MONITORING PROTOCOLS

// ============================================================================

/// Interface for performance monitoring.
///
/// Enforces consistent performance monitoring across XWSystem.
pub trait IPerformance {
    /// Start performance timer.
    /// Args:
    /// operation: Operation name
    /// Returns:
    /// Timer ID
    fn start_timer(&self, operation: String) -> String;

    /// End performance timer.
    /// Args:
    /// timer_id: Timer ID
    /// Returns:
    /// Elapsed time in seconds
    fn end_timer(&self, timer_id: String) -> f64;

    /// Get performance metrics.
    /// Returns:
    /// Performance metrics dictionary
    fn get_metrics(&self) -> HashMap<String, serde_json::Value>;

    /// Reset performance metrics.
    fn reset_metrics(&self) -> ();

    /// Record performance metric.
    /// Args:
    /// name: Metric name
    /// value: Metric value
    /// metric_type: Type of metric
    fn record_metric(&self, name: String, value: f64, metric_type: MetricType) -> ();

    /// Get performance metric value.
    /// Args:
    /// name: Metric name
    /// Returns:
    /// Metric value or None
    fn get_metric(&self, name: String) -> Option<f64>;

    /// Get current performance level.
    /// Returns:
    /// Current performance level
    fn get_performance_level(&self) -> PerformanceLevel;

    /// Set performance threshold.
    /// Args:
    /// metric: Metric name
    /// threshold: Threshold value
    fn set_performance_threshold(&self, metric: String, threshold: f64) -> ();

    /// Check if performance is acceptable.
    /// Returns:
    /// True if performance is acceptable
    fn is_performance_acceptable(&self) -> bool;

}

/// Interface for monitorable objects.
///
/// Enforces consistent monitoring behavior across XWSystem.
pub trait IMonitorable {
    /// Start monitoring.
    fn start_monitoring(&self) -> ();

    /// Stop monitoring.
    fn stop_monitoring(&self) -> ();

    /// Get current status.
    /// Returns:
    /// Current health status
    fn get_status(&self) -> HealthStatus;

    /// Get health information.
    /// Returns:
    /// Health information dictionary
    fn get_health(&self) -> HashMap<String, serde_json::Value>;

    /// Check if monitoring is active.
    /// Returns:
    /// True if monitoring is active
    fn is_monitoring(&self) -> bool;

    /// Get monitoring information.
    /// Returns:
    /// Monitoring information dictionary
    fn get_monitoring_info(&self) -> HashMap<String, serde_json::Value>;

    /// Set monitoring interval.
    /// Args:
    /// interval: Monitoring interval in seconds
    fn set_monitoring_interval(&self, interval: f64) -> ();

    /// Get monitoring interval.
    /// Returns:
    /// Monitoring interval in seconds
    fn get_monitoring_interval(&self) -> f64;

}

/// Interface for metrics collection.
///
/// Enforces consistent metrics behavior across XWSystem.
pub trait IMetrics {
    /// Collect all metrics.
    /// Returns:
    /// Dictionary of collected metrics
    fn collect_metrics(&self) -> HashMap<String, serde_json::Value>;

    /// Add metric.
    /// Args:
    /// name: Metric name
    /// value: Metric value
    /// labels: Optional metric labels
    fn add_metric(&self, name: String, value: serde_json::Value, labels: Option<HashMap<String, String>>) -> ();

    /// Get metric value.
    /// Args:
    /// name: Metric name
    /// Returns:
    /// Metric value or None
    fn get_metric(&self, name: String) -> Option<serde_json::Value>;

    /// Remove metric.
    /// Args:
    /// name: Metric name to remove
    /// Returns:
    /// True if removed
    fn remove_metric(&self, name: String) -> bool;

    /// List all metric names.
    /// Returns:
    /// List of metric names
    fn list_metrics(&self) -> Vec<String>;

    /// Export metrics in specified format.
    /// Args:
    /// format: Export format
    /// Returns:
    /// Exported metrics string
    fn export_metrics(&self, format: String) -> String;

    /// Clear all metrics.
    fn clear_metrics(&self) -> ();

    /// Get metrics summary.
    /// Returns:
    /// Metrics summary dictionary
    fn get_metrics_summary(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for health checks.
///
/// Enforces consistent health checking across XWSystem.
pub trait IHealthCheck {
    /// Perform health check.
    /// Returns:
    /// Health status
    fn check_health(&self) -> HealthStatus;

    /// Get detailed health information.
    /// Returns:
    /// Health details dictionary
    fn get_health_details(&self) -> HashMap<String, serde_json::Value>;

    /// Add health check function.
    /// Args:
    /// name: Health check name
    /// check_func: Health check function
    fn add_health_check(&self, name: String, check_func: fn()) -> ();

    /// Remove health check.
    /// Args:
    /// name: Health check name
    /// Returns:
    /// True if removed
    fn remove_health_check(&self, name: String) -> bool;

    /// List all health check names.
    /// Returns:
    /// List of health check names
    fn list_health_checks(&self) -> Vec<String>;

    /// Run all health checks.
    /// Returns:
    /// Dictionary of health check results
    fn run_health_checks(&self) -> HashMap<String, HealthStatus>;

    /// Get overall health status.
    /// Returns:
    /// Overall health status
    fn get_overall_health(&self) -> HealthStatus;

    /// Set health check threshold.
    /// Args:
    /// check_name: Health check name
    /// threshold: Threshold value
    fn set_health_threshold(&self, check_name: String, threshold: f64) -> ();

}

/// Interface for alerting.
///
/// Enforces consistent alerting behavior across XWSystem.
pub trait IAlerting {
    /// Create alert.
    /// Args:
    /// message: Alert message
    /// level: Alert level
    /// source: Alert source
    /// Returns:
    /// Alert ID
    fn create_alert(&self, message: String, level: AlertLevel, source: String) -> String;

    /// Get alert by ID.
    /// Args:
    /// alert_id: Alert ID
    /// Returns:
    /// Alert information or None
    fn get_alert(&self, alert_id: String) -> Option<HashMap<String, serde_json::Value>>;

    /// List alerts.
    /// Args:
    /// level: Filter by alert level
    /// Returns:
    /// List of alert information
    fn list_alerts(&self, level: Option<AlertLevel>) -> Vec<HashMap<String, serde_json::Value>>;

    /// Acknowledge alert.
    /// Args:
    /// alert_id: Alert ID
    /// user: User acknowledging alert
    /// Returns:
    /// True if acknowledged
    fn acknowledge_alert(&self, alert_id: String, user: String) -> bool;

    /// Resolve alert.
    /// Args:
    /// alert_id: Alert ID
    /// resolution: Resolution description
    /// Returns:
    /// True if resolved
    fn resolve_alert(&self, alert_id: String, resolution: String) -> bool;

    /// Clear alert.
    /// Args:
    /// alert_id: Alert ID
    /// Returns:
    /// True if cleared
    fn clear_alert(&self, alert_id: String) -> bool;

    /// Get alert statistics.
    /// Returns:
    /// Alert statistics dictionary
    fn get_alert_stats(&self) -> HashMap<String, i64>;

    /// Set alert threshold.
    /// Args:
    /// metric: Metric name
    /// threshold: Threshold value
    /// level: Alert level
    fn set_alert_threshold(&self, metric: String, threshold: f64, level: AlertLevel) -> ();

}

/// Interface for system monitoring.
///
/// Enforces consistent system monitoring across XWSystem.
pub trait ISystemMonitor {
    /// Get CPU usage percentage.
    /// Returns:
    /// CPU usage percentage
    fn get_cpu_usage(&self) -> f64;

    /// Get memory usage information.
    /// Returns:
    /// Memory usage dictionary
    fn get_memory_usage(&self) -> HashMap<String, serde_json::Value>;

    /// Get disk usage information.
    /// Returns:
    /// Disk usage dictionary
    fn get_disk_usage(&self) -> HashMap<String, serde_json::Value>;

    /// Get network usage information.
    /// Returns:
    /// Network usage dictionary
    fn get_network_usage(&self) -> HashMap<String, serde_json::Value>;

    /// Get process information.
    /// Returns:
    /// List of process information
    fn get_process_info(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Get system load average.
    /// Returns:
    /// System load average
    fn get_system_load(&self) -> f64;

    /// Get system uptime.
    /// Returns:
    /// Uptime in seconds
    fn get_uptime(&self) -> f64;

    /// Get system information.
    /// Returns:
    /// System information dictionary
    fn get_system_info(&self) -> HashMap<String, serde_json::Value>;

}

/// Interface for performance profiling.
///
/// Enforces consistent profiling behavior across XWSystem.
pub trait IProfiler {
    /// Start profiling session.
    /// Args:
    /// name: Profiling session name
    /// Returns:
    /// Profiling session ID
    fn start_profiling(&self, name: String) -> String;

    /// Stop profiling session.
    /// Args:
    /// session_id: Profiling session ID
    /// Returns:
    /// Profiling results
    fn stop_profiling(&self, session_id: String) -> HashMap<String, serde_json::Value>;

    /// Profile function execution.
    /// Args:
    /// func: Function to profile
    /// *args: Function arguments
    /// **kwargs: Function keyword arguments
    /// Returns:
    /// Tuple of (result, profiling_data)
    fn profile_function(&self, func: fn()) -> (serde_json::Value, HashMap<String, serde_json::Value>);

    /// Get profiling results.
    /// Args:
    /// session_id: Profiling session ID
    /// Returns:
    /// Profiling results or None
    fn get_profiling_results(&self, session_id: String) -> Option<HashMap<String, serde_json::Value>>;

    /// List profiling sessions.
    /// Returns:
    /// List of session IDs
    fn list_profiling_sessions(&self) -> Vec<String>;

    /// Clear profiling data.
    fn clear_profiling_data(&self) -> ();

    /// Export profiling data.
    /// Args:
    /// session_id: Profiling session ID
    /// format: Export format
    /// Returns:
    /// Exported profiling data
    fn export_profiling_data(&self, session_id: String, format: String) -> String;

}

/// Interface for monitoring configuration.
///
/// Enforces consistent monitoring configuration across XWSystem.
pub trait IMonitoringConfig {
    /// Set monitoring mode.
    /// Args:
    /// mode: Monitoring mode
    fn set_monitoring_mode(&self, mode: MonitoringMode) -> ();

    /// Get monitoring mode.
    /// Returns:
    /// Current monitoring mode
    fn get_monitoring_mode(&self) -> MonitoringMode;

    /// Set metric collection interval.
    /// Args:
    /// metric: Metric name
    /// interval: Collection interval in seconds
    fn set_metric_interval(&self, metric: String, interval: f64) -> ();

    /// Get metric collection interval.
    /// Args:
    /// metric: Metric name
    /// Returns:
    /// Collection interval or None
    fn get_metric_interval(&self, metric: String) -> Option<f64>;

    /// Enable metric collection.
    /// Args:
    /// metric: Metric name
    fn enable_metric(&self, metric: String) -> ();

    /// Disable metric collection.
    /// Args:
    /// metric: Metric name
    fn disable_metric(&self, metric: String) -> ();

    /// Check if metric is enabled.
    /// Args:
    /// metric: Metric name
    /// Returns:
    /// True if enabled
    fn is_metric_enabled(&self, metric: String) -> bool;

    /// Get monitoring configuration.
    /// Returns:
    /// Monitoring configuration dictionary
    fn get_monitoring_config(&self) -> HashMap<String, serde_json::Value>;

    /// Set monitoring configuration.
    /// Args:
    /// config: Monitoring configuration
    fn set_monitoring_config(&self, config: HashMap<String, serde_json::Value>) -> ();

}

/// Protocol for objects that support performance monitoring (simpler interface than IMonitorable).
pub trait IMonitorableSimple {
    /// Start performance monitoring.
    fn start_monitoring(&self) -> ();

    /// Stop performance monitoring.
    fn stop_monitoring(&self) -> ();

    /// Get performance statistics.
    fn get_stats(&self) -> HashMap<String, serde_json::Value>;

}
