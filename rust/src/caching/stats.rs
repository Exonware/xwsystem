// #exonware/xwsystem/rust/src/caching/stats.rs
//exonware/xwsystem/caching/stats.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Statistics formatting utilities for caching module.
//! Usability Priority #2 - Human-readable statistics display.


use std::collections::HashMap;

use crate::caching::utils::format_bytes;

/// Format cache statistics as human-readable string.
///
///
/// Args:
/// stats: Cache statistics dictionary
/// style: Display style ('box', 'table', 'compact')
///
///
/// Returns:
/// Formatted statistics string
pub fn format_cache_stats(stats: HashMap<String, serde_json::Value>, style: Option<&str>) -> String {
    let style = style.unwrap_or("compact");
    match style {
        "box" => _format_box_style(stats),
        "table" => _format_table_style(stats),
        "compact" => _format_compact_style(stats),
        _ => _format_compact_style(stats),
    }
}

// Calculate fill percentage
// Add TTL info if present
// Add memory info if present
/// Format statistics in box style with borders.
pub fn _format_box_style(stats: HashMap<String, serde_json::Value>) -> String {
    let mut lines = Vec::new();
    lines.push("┌─────────────────────────────────────┐".to_string());
    
    // Calculate fill percentage
    let capacity = stats.get("capacity")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let size = stats.get("size")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let fill_pct = if capacity > 0 {
        (size as f64 / capacity as f64) * 100.0
    } else {
        0.0
    };
    
    for (key, value) in &stats {
        let value_str = match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            _ => serde_json::to_string(value).unwrap_or_else(|_| "N/A".to_string()),
        };
        lines.push(format!("│ {:20} : {:15} │", key, value_str));
    }
    
    // Add TTL info if present
    if stats.contains_key("ttl") {
        if let Some(ttl) = stats.get("ttl").and_then(|v| v.as_f64()) {
            lines.push(format!("│ {:20} : {:15.2} │", "TTL (seconds)", ttl));
        }
    }
    
    // Add fill percentage
    lines.push(format!("│ {:20} : {:15.2}% │", "Fill Percentage", fill_pct));
    
    lines.push("└─────────────────────────────────────┘".to_string());
    lines.join("\n")
}

/// Format statistics in table style.
pub fn _format_table_style(stats: HashMap<String, serde_json::Value>) -> String {
    let mut lines = Vec::new();
    lines.push("┌─────────────────────┬─────────────────┐".to_string());
    lines.push("│ Metric               │ Value           │".to_string());
    lines.push("├─────────────────────┼─────────────────┤".to_string());
    
    for (key, value) in &stats {
        let value_str = match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            _ => serde_json::to_string(value).unwrap_or_else(|_| "N/A".to_string()),
        };
        lines.push(format!("│ {:19} │ {:15} │", key, value_str));
    }
    
    lines.push("└─────────────────────┴─────────────────┘".to_string());
    lines.join("\n")
}

/// Format statistics in compact one-line style.
pub fn _format_compact_style(stats: HashMap<String, serde_json::Value>) -> String {
    let mut parts = Vec::new();
    
    if let Some(name) = stats.get("name").and_then(|v| v.as_str()) {
        parts.push(format!("name={}", name));
    }
    
    if let Some(size) = stats.get("size").and_then(|v| v.as_i64()) {
        if let Some(capacity) = stats.get("capacity").and_then(|v| v.as_i64()) {
            parts.push(format!("size={}/{}", size, capacity));
        } else {
            parts.push(format!("size={}", size));
        }
    }
    
    if let Some(hits) = stats.get("hits").and_then(|v| v.as_i64()) {
        if let Some(misses) = stats.get("misses").and_then(|v| v.as_i64()) {
            parts.push(format!("hits={} misses={}", hits, misses));
        }
    }
    
    if let Some(hit_rate) = stats.get("hit_rate").and_then(|v| v.as_f64()) {
        parts.push(format!("hit_rate={:.2}%", hit_rate * 100.0));
    }
    
    parts.join(", ")
}

/// Format statistics for multiple caches in a table.
///
///
/// Args:
/// cache_list: List of cache objects with get_stats() method
///
///
/// Returns:
/// Formatted table string
pub fn format_cache_stats_table(cache_list: Vec<serde_json::Value>) -> String {
    if cache_list.is_empty() {
        return "No caches to display".to_string();
    }
    
    let mut lines = Vec::new();
    lines.push("┌──────────┬────────┬──────┬───────┬──────────┬─────────┐".to_string());
    lines.push("│ Name     │ Type   │ Size │ Hits  │ Misses   │ Hit Rate│".to_string());
    lines.push("├──────────┼────────┼──────┼───────┼──────────┼─────────┤".to_string());
    
    for cache_val in cache_list {
        if let serde_json::Value::Object(stats) = cache_val {
            let name = stats.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("N/A");
            let cache_type = stats.get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("N/A");
            let size = stats.get("size")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let hits = stats.get("hits")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let misses = stats.get("misses")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let hit_rate = stats.get("hit_rate")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            
            lines.push(format!(
                "│ {:8} │ {:6} │ {:4} │ {:5} │ {:8} │ {:7.2}% │",
                name, cache_type, size, hits, misses, hit_rate * 100.0
            ));
        }
    }
    
    lines.push("└──────────┴────────┴──────┴───────┴──────────┴─────────┘".to_string());
    lines.join("\n")
}

/// Get one-line summary of cache statistics.
///
///
/// Args:
/// stats: Cache statistics dictionary
///
///
/// Returns:
/// One-line summary string
pub fn get_stats_summary(stats: HashMap<String, serde_json::Value>) -> String {
    let name = stats.get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("cache");
    let size = stats.get("size")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let capacity = stats.get("capacity")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let hits = stats.get("hits")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let misses = stats.get("misses")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let hit_rate = stats.get("hit_rate")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    
    format!(
        "{}: {}/{} items, {} hits, {} misses, {:.2}% hit rate",
        name, size, capacity, hits, misses, hit_rate * 100.0
    )
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Functions exported via mod.rs
