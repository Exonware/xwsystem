// #exonware/xwsystem/rust/src/monitoring/base.rs
//exonware/xwsystem/monitoring/base.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Monitoring module base classes - abstract classes for monitoring functionality.


use std::collections::HashMap;

use crate::defs::{AlertLevel, HealthStatus, MetricType, SpanKind};

// ============================================================================

// TRACING BASE CLASSES (Moved from enterprise)

// ============================================================================

/// Abstract base class for performance monitoring.
pub trait APerformanceMonitorBase {
    /// Start performance monitoring.
    fn start_monitoring(&self) -> ();

    /// Stop performance monitoring.
    fn stop_monitoring(&self) -> ();

    /// Check if monitoring is active.
    fn is_monitoring(&self) -> bool;

    /// Record performance metric.
    fn record_metric(&self, metric_name: String, value: f64, metric_type: MetricType) -> ();

    /// Get metric value.
    fn get_metric(&self, metric_name: String) -> Option<f64>;

    /// Get all metrics.
    fn get_all_metrics(&self) -> HashMap<String, f64>;

    /// Set metric threshold.
    fn set_threshold(&self, metric_name: String, threshold: f64) -> ();

    /// Check if any metrics exceed thresholds.
    fn check_thresholds(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Get performance summary.
    fn get_performance_summary(&self) -> HashMap<String, serde_json::Value>;

}

/// Abstract base class for memory monitoring.
pub trait AMemoryMonitorBase {
    /// Start memory monitoring.
    fn start_monitoring(&self) -> ();

    /// Stop memory monitoring.
    fn stop_monitoring(&self) -> ();

    /// Take memory snapshot.
    fn take_snapshot(&self) -> HashMap<String, serde_json::Value>;

    /// Get current memory usage.
    fn get_memory_usage(&self) -> HashMap<String, f64>;

    /// Detect potential memory leaks.
    fn detect_memory_leaks(&self) -> Vec<HashMap<String, serde_json::Value>>;

    /// Get memory statistics.
    fn get_memory_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Cleanup memory and return bytes freed.
    fn cleanup_memory(&self) -> i64;

    /// Get garbage collection statistics.
    fn get_garbage_collection_stats(&self) -> HashMap<String, serde_json::Value>;

    /// Force garbage collection.
    fn force_garbage_collection(&self) -> ();

}

/// Abstract base class for metrics collection.
pub trait AMetricsBase {
    /// Increment counter metric.
    fn increment_counter(&self, name: String, value: i64) -> ();

    /// Set gauge metric.
    fn set_gauge(&self, name: String, value: f64) -> ();

    /// Record histogram metric.
    fn record_histogram(&self, name: String, value: f64) -> ();

    /// Record timer metric.
    fn record_timer(&self, name: String, duration: f64) -> ();

    /// Get counter value.
    fn get_counter(&self, name: String) -> i64;

    /// Get gauge value.
    fn get_gauge(&self, name: String) -> f64;

    /// Get histogram statistics.
    fn get_histogram_stats(&self, name: String) -> HashMap<String, f64>;

    /// Get timer statistics.
    fn get_timer_stats(&self, name: String) -> HashMap<String, f64>;

    /// Export all metrics.
    fn export_metrics(&self) -> HashMap<String, serde_json::Value>;

    /// Reset all metrics.
    fn reset_metrics(&self) -> ();

}

/// Abstract base class for error recovery.
pub trait AErrorRecoveryBase {
    /// Register error recovery strategy.
    fn register_recovery_strategy(&self, error_type: String, strategy: fn()) -> ();

    /// Handle error with recovery strategy.
    fn handle_error(&self, error: Exception, context: HashMap<String, serde_json::Value>) -> bool;

    /// Create circuit breaker.
    fn create_circuit_breaker(&self, name: String, failure_threshold: i64, recovery_timeout: i64) -> ();

    /// Check if circuit breaker is open.
    fn is_circuit_open(&self, name: String) -> bool;

    /// Record successful operation.
    fn record_success(&self, name: String) -> ();

    /// Record failed operation.
    fn record_failure(&self, name: String) -> ();

    /// Get circuit breaker state.
    fn get_circuit_state(&self, name: String) -> HashMap<String, serde_json::Value>;

}

/// Abstract base class for system monitoring.
pub trait ASystemMonitorBase {
    /// Start system monitoring.
    fn start_system_monitoring(&self) -> ();

    /// Stop system monitoring.
    fn stop_system_monitoring(&self) -> ();

    /// Get CPU usage percentage.
    fn get_cpu_usage(&self) -> f64;

    /// Get memory usage statistics.
    fn get_memory_usage(&self) -> HashMap<String, f64>;

    /// Get disk usage statistics.
    fn get_disk_usage(&self) -> HashMap<String, f64>;

    /// Get network usage statistics.
    fn get_network_usage(&self) -> HashMap<String, f64>;

    /// Get system load average.
    fn get_system_load(&self) -> f64;

    /// Get number of running processes.
    fn get_process_count(&self) -> i64;

    /// Get system uptime in seconds.
    fn get_system_uptime(&self) -> f64;

    /// Get overall system health status.
    fn get_system_health(&self) -> HealthStatus;

    /// Get comprehensive system information.
    fn get_system_info(&self) -> HashMap<String, serde_json::Value>;

}

/// Abstract base class for tracing providers.
pub trait ATracingProvider {
    /// Start a new span.
    fn start_span(&self, name: String, kind: String, parent: Option<String>, attributes: Option<HashMap<String, serde_json::Value>>) -> String;

    /// Finish a span.
    fn finish_span(&self, span: String, status: String, error: Option<Exception>) -> ();

    /// Add attribute to span.
    fn add_span_attribute(&self, span: String, key: String, value: serde_json::Value) -> ();

    /// Add event to span.
    fn add_span_event(&self, span: String, name: String, attributes: Option<HashMap<String, serde_json::Value>>) -> ();

}
