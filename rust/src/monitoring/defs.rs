// #exonware/xwsystem/rust/src/monitoring/defs.rs
//exonware/xwsystem/monitoring/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Monitoring types and enums for XWSystem.


use serde::{Serialize, Deserialize};

use crate::shared::defs::{PerformanceLevel};

/// Metric types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    Timer,
    Rate,
}

/// Health status levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
    Degraded,
}

/// Alert levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
    Emergency,
}

/// Monitoring modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonitoringMode {
    Passive,
    Active,
    Continuous,
    #[serde(rename = "on_demand")]
    OnDemand,
}

// Failing, reject requests
// Testing if service recovered
/// Circuit breaker states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    Closed,
    Open,
    #[serde(rename = "half_open")]
    HalfOpen,
}

/// Span kinds for different operation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpanKind {
    #[serde(rename = "INTERNAL")]
    Internal,
    #[serde(rename = "SERVER")]
    Server,
    #[serde(rename = "CLIENT")]
    Client,
    #[serde(rename = "PRODUCER")]
    Producer,
    #[serde(rename = "CONSUMER")]
    Consumer,
}
