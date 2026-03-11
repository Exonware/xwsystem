// #exonware/xwsystem/rust/src/caching/metrics_exporter.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Metrics exporters for cache monitoring.
//! Performance Priority #4 - Observability and monitoring integration.


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::caching::base::ACache;

// HELP and TYPE declarations
// Additional metrics based on cache type
/// Export cache metrics in Prometheus format.
///
/// Provides cache statistics in Prometheus-compatible format for monitoring.
pub struct PrometheusExporter {
    cache: Arc<Mutex<dyn ACache + Send + Sync>>,
    metric_prefix: String,
}

impl PrometheusExporter {
    /// Initialize Prometheus exporter.
    ///
    /// Args:
    /// cache: Cache instance to export metrics from
    /// metric_prefix: Prefix for metric names
    pub fn new<C>(cache: C, metric_prefix: Option<String>) -> Self
    where
        C: ACache + Send + Sync + 'static,
    {
        Self {
            cache: Arc::new(Mutex::new(cache)),
            metric_prefix: metric_prefix.unwrap_or_else(|| "xwcache".to_string()),
        }
    }

    /// Export metrics in Prometheus format.
    ///
    /// Returns:
    /// Prometheus-formatted metrics string
    pub fn export_metrics(&self) -> String {
        let stats = self.cache.lock().unwrap().get_stats();
        
        let cache_name = stats.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let cache_type = stats.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        
        let mut lines = Vec::new();
        
        // HELP and TYPE declarations
        let size = stats.get("size")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        lines.push(format!("# HELP {}_size Current number of entries in cache", self.metric_prefix));
        lines.push(format!("# TYPE {}_size gauge", self.metric_prefix));
        lines.push(format!(r#"{}_size{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, size));
        
        let capacity = stats.get("capacity")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        lines.push(format!("# HELP {}_capacity Maximum cache capacity", self.metric_prefix));
        lines.push(format!("# TYPE {}_capacity gauge", self.metric_prefix));
        lines.push(format!(r#"{}_capacity{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, capacity));
        
        let hits = stats.get("hits")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        lines.push(format!("# HELP {}_hits_total Total cache hits", self.metric_prefix));
        lines.push(format!("# TYPE {}_hits_total counter", self.metric_prefix));
        lines.push(format!(r#"{}_hits_total{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, hits));
        
        let misses = stats.get("misses")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        lines.push(format!("# HELP {}_misses_total Total cache misses", self.metric_prefix));
        lines.push(format!("# TYPE {}_misses_total counter", self.metric_prefix));
        lines.push(format!(r#"{}_misses_total{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, misses));
        
        let evictions = stats.get("evictions")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        lines.push(format!("# HELP {}_evictions_total Total cache evictions", self.metric_prefix));
        lines.push(format!("# TYPE {}_evictions_total counter", self.metric_prefix));
        lines.push(format!(r#"{}_evictions_total{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, evictions));
        
        let hit_rate = stats.get("hit_rate")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        lines.push(format!("# HELP {}_hit_rate Cache hit rate (0.0 to 1.0)", self.metric_prefix));
        lines.push(format!("# TYPE {}_hit_rate gauge", self.metric_prefix));
        lines.push(format!(r#"{}_hit_rate{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, hit_rate));
        
        // Additional metrics based on cache type
        if let Some(memory_mb) = stats.get("memory_used_mb").and_then(|v| v.as_f64()) {
            lines.push(format!("# HELP {}_memory_mb Memory used in megabytes", self.metric_prefix));
            lines.push(format!("# TYPE {}_memory_mb gauge", self.metric_prefix));
            lines.push(format!(r#"{}_memory_mb{{cache="{}",type="{}"}} {}"#, self.metric_prefix, cache_name, cache_type, memory_mb));
        }
        
        lines.join("\n") + "\n"
    }

    /// Export metrics as dictionary.
    ///
    /// Returns:
    /// Dictionary of metrics
    pub fn export_dict(&self) -> HashMap<String, serde_json::Value> {
        let stats = self.cache.lock().unwrap().get_stats();
        
        let mut result = HashMap::new();
        result.insert("cache_name".to_string(), stats.get("name").cloned().unwrap_or(serde_json::Value::String("unknown".to_string())));
        result.insert("cache_type".to_string(), stats.get("type").cloned().unwrap_or(serde_json::Value::String("unknown".to_string())));
        result.insert("size".to_string(), stats.get("size").cloned().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0))));
        result.insert("capacity".to_string(), stats.get("capacity").cloned().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0))));
        result.insert("hits".to_string(), stats.get("hits").cloned().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0))));
        result.insert("misses".to_string(), stats.get("misses").cloned().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0))));
        result.insert("evictions".to_string(), stats.get("evictions").cloned().unwrap_or(serde_json::Value::Number(serde_json::Number::from(0))));
        result.insert("hit_rate".to_string(), stats.get("hit_rate").cloned().unwrap_or(serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap())));
        result.insert("stats".to_string(), serde_json::Value::Object(serde_json::Map::from_iter(
            stats.iter().map(|(k, v)| (k.clone(), v.clone()))
        )));
        result
    }
}

/// Collect statistics from multiple caches.
pub struct StatsCollector {
    caches: Mutex<HashMap<String, Arc<Mutex<dyn ACache + Send + Sync>>>>,
}

impl StatsCollector {
    /// Initialize stats collector.
    pub fn new() -> Self {
        Self {
            caches: Mutex::new(HashMap::new()),
        }
    }

    /// Register cache for monitoring.
    ///
    /// Args:
    /// name: Unique name for this cache
    /// cache: Cache instance
    pub fn register<C>(&self, name: String, cache: C)
    where
        C: ACache + Send + Sync + 'static,
    {
        let mut caches = self.caches.lock().unwrap();
        caches.insert(name, Arc::new(Mutex::new(cache)));
    }

    /// Unregister cache.
    ///
    /// Args:
    /// name: Name of cache to unregister
    pub fn unregister(&self, name: String) {
        let mut caches = self.caches.lock().unwrap();
        caches.remove(&name);
    }

    /// Collect stats from all registered caches.
    ///
    /// Returns:
    /// Dictionary of cache stats
    pub fn collect_all(&self) -> HashMap<String, HashMap<String, serde_json::Value>> {
        let caches = self.caches.lock().unwrap();
        let mut all_stats = HashMap::new();
        
        for (name, cache) in caches.iter() {
            if let Ok(stats) = cache.lock() {
                all_stats.insert(name.clone(), stats.get_stats());
            }
        }
        
        all_stats
    }

    /// Export all cache metrics in Prometheus format.
    ///
    /// Returns:
    /// Prometheus-formatted metrics
    pub fn export_prometheus(&self) -> String {
        let caches = self.caches.lock().unwrap();
        let mut metrics_lines = Vec::new();
        
        for (name, cache) in caches.iter() {
            let exporter = PrometheusExporter {
                cache: cache.clone(),
                metric_prefix: format!("xwcache_{}", name),
            };
            metrics_lines.push(exporter.export_metrics());
        }
        
        metrics_lines.join("\n")
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
