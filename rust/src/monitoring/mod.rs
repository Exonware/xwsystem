// #exonware/xwsystem/rust/src/monitoring/mod.rs
//! XSystem Monitoring Package
//! 
//! Provides performance monitoring, metrics collection, system observation utilities,
//! memory monitoring, error recovery, resilience mechanisms, and distributed tracing.

pub mod base;
pub mod defs;
pub mod error_recovery;
pub mod errors;
pub mod memory_monitor;
pub mod metrics;
pub mod performance_manager_generic;
pub mod performance_monitor;
pub mod performance_validator;
pub mod tracing;

pub use base::{ATracingProvider};

pub use defs::{SpanKind};

pub use error_recovery::{CircuitBreaker, CircuitBreakerConfig, CircuitState, ErrorContext, ErrorRecoveryManager, circuit_breaker, get_error_recovery_manager, graceful_degradation, handle_error, retry_with_backoff};

pub use errors::{DistributedTracingError, SpanError, TraceContextError, TracingError};

pub use memory_monitor::{MemoryLeakReport, MemoryMonitor, MemorySnapshot, force_memory_cleanup, get_memory_monitor, get_memory_stats, register_object_for_monitoring, start_memory_monitoring, stop_memory_monitoring, unregister_object_from_monitoring};

pub use metrics::{GenericMetrics, OperationMetrics, create_component_metrics, get_metrics, reset_metrics};

pub use performance_manager_generic::{GenericPerformanceManager, HealthStatus, PerformanceRecommendation};

pub use performance_monitor::{PerformanceMonitor, PerformanceStats, calculate_performance_summary, create_performance_monitor, enhanced_error_context, format_performance_report, performance_context};

pub use performance_validator::{PerformanceMetric, PerformanceReport, PerformanceThreshold, PerformanceValidator, get_performance_statistics, get_performance_validator, performance_monitor, record_performance_metric, start_performance_validation, stop_performance_validation, validate_performance};

pub use tracing::{DistributedTracing, JaegerTracer, OpenTelemetryTracer, SpanContext, TraceContext, TracingManager, configure_tracing, get_tracing_manager};
