// #exonware/xwsystem/rust/src/config/performance_modes.rs
//! Performance mode definitions for XWSystem framework.
//! 
//! This module provides enums and utilities for managing performance optimization
//! modes across different components of the XWSystem framework.


use std::collections::HashMap;
use crate::config::defs::{AdvancedPerformanceMode, PerformanceMode};

/// Performance mode constants for backward compatibility and simple usage.
pub struct PerformanceModes;

impl PerformanceModes {
    pub const FAST: &'static str = "FAST";
    pub const BALANCED: &'static str = "BALANCED";
    pub const MEMORY_OPTIMIZED: &'static str = "MEMORY_OPTIMIZED";
}

// Lazy loading thresholds
// Security limits (always enforced)
/// Configuration profile for different performance modes.
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub mode: PerformanceMode,
    pub settings: HashMap<String, serde_json::Value>,
}

impl PerformanceProfile {
    /// Convert profile to dictionary.
    pub fn to_dict(&self) -> HashMap<String, serde_json::Value> {
        let mut dict = HashMap::new();
        dict.insert("mode".to_string(), serde_json::to_value(&self.mode).unwrap_or(serde_json::Value::Null));
        dict.insert("settings".to_string(), serde_json::to_value(&self.settings).unwrap_or(serde_json::Value::Null));
        dict
    }

    /// Create profile from dictionary.
    pub fn from_dict(data: &HashMap<String, serde_json::Value>) -> Result<Self, String> {
        let mode = data.get("mode")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or(PerformanceMode::Balanced);
        let settings = data.get("settings")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();
        Ok(Self { mode, settings })
    }
}

// 10% performance improvement threshold
// Number of operations to remember
// Check performance every second
// 80% memory usage triggers adaptation
// 70% CPU usage triggers adaptation
// Hybrid strategy settings
// Seconds between adaptations
/// Enhanced profile for ADAPTIVE mode with learning capabilities.
#[derive(Debug, Clone)]
pub struct AdaptiveProfile {
    pub base_profile: PerformanceProfile,
    pub learning_enabled: bool,
    pub adaptation_threshold: f64,
}

impl AdaptiveProfile {
    pub fn new(base_profile: PerformanceProfile) -> Self {
        Self {
            base_profile,
            learning_enabled: true,
            adaptation_threshold: 0.1,
        }
    }
}

// Phase 1: CRUISE (Fast, low-overhead monitoring)
// Sample every 50th operation
// Check system every 5 seconds
// Keep only 200 operations in cruise
// Phase 2: DEEP_DIVE (Intensive learning)
// 15% performance degradation triggers deep-dive
// Deep-dive for 500 operations
// Sample every operation in deep-dive
// Keep detailed history during deep-dive
// 80% memory usage triggers deep-dive
// 70% CPU usage triggers deep-dive
// 20% degradation triggers deep-dive
// 10% improvement threshold
// 10 seconds between adaptations
/// Smart dual-phase adaptive profile: fast cruise + intelligent deep-dive.
#[derive(Debug, Clone)]
pub struct DualAdaptiveProfile {
    pub cruise_profile: PerformanceProfile,
    pub deep_dive_profile: PerformanceProfile,
    pub current_phase: String,
}

impl DualAdaptiveProfile {
    pub fn new() -> Self {
        Self {
            cruise_profile: PerformanceProfile {
                mode: PerformanceMode::Fast,
                settings: HashMap::new(),
            },
            deep_dive_profile: PerformanceProfile {
                mode: PerformanceMode::Balanced,
                settings: HashMap::new(),
            },
            current_phase: "cruise".to_string(),
        }
    }
}

// Weighted combination of speed and efficiency
// Faster = higher score
// Less memory = higher score
/// Container for performance metrics and statistics.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub execution_time: f64,
    pub memory_usage: f64,
    pub cache_hits: i64,
    pub cache_misses: i64,
}

impl PerformanceMetrics {
    /// Calculate cache hit rate.
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Calculate overall performance score (higher is better).
    pub fn performance_score(&self) -> f64 {
        let hit_rate = self.cache_hit_rate();
        let time_score = 1.0 / (1.0 + self.execution_time);
        let memory_score = 1.0 / (1.0 + self.memory_usage);
        (hit_rate * 0.4 + time_score * 0.4 + memory_score * 0.2) * 100.0
    }
}

// Initialize mode performance tracking without causing recursion
// Keep history size manageable
// Track mode performance
// Keep only recent performance data
// Cache system metrics for a short period
// Keep only recent system metrics
// Fallback if psutil is not available
// Assume moderate usage
// Assume moderate usage
// Check cooldown period
// Check system pressure
// Check if we have enough data for learning
// Check for performance degradation
// If recent performance is significantly worse than historical average
// Filter metrics by operation type if specified
// Calculate mode performance scores
// Calculate weighted score based on recency and performance
// Adjust for system pressure
// Prefer memory-efficient modes under memory pressure
// Prefer fast modes under CPU pressure
// Return the best performing mode
// Get the base profile for the new mode
// Update the adaptive profile with base settings
// Apply hybrid optimizations if enabled
// Adjust cache sizes based on memory pressure
// Adjust lazy loading thresholds based on performance history
/// Engine for learning and adapting performance strategies.
pub struct AdaptiveLearningEngine {
    pub profile: AdaptiveProfile,
    metrics_history: Vec<PerformanceMetrics>,
}

impl AdaptiveLearningEngine {
    /// Constructor
    pub fn new(profile: AdaptiveProfile) -> Self {
        Self {
            profile,
            metrics_history: Vec::new(),
        }
    }

    /// Record performance metrics for learning.
    pub fn record_operation(&mut self, metrics: PerformanceMetrics) {
        self.metrics_history.push(metrics);
        // Keep only recent data (last 1000 entries)
        if self.metrics_history.len() > 1000 {
            self.metrics_history.remove(0);
        }
    }

    /// Get current system metrics.
    pub fn get_system_metrics(&mut self) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        // Placeholder - would use sysinfo crate in real implementation
        metrics.insert("cpu_usage".to_string(), 0.5);
        metrics.insert("memory_usage".to_string(), 0.5);
        metrics
    }

    /// Determine if adaptation is needed.
    pub fn should_adapt(&self) -> bool {
        if self.metrics_history.len() < 10 {
            return false;
        }
        // Simple check: if recent performance is worse than average
        let recent: Vec<&PerformanceMetrics> = self.metrics_history.iter().rev().take(10).collect();
        let avg_score: f64 = recent.iter().map(|m| m.performance_score()).sum::<f64>() / recent.len() as f64;
        let overall_avg: f64 = self.metrics_history.iter().map(|m| m.performance_score()).sum::<f64>() / self.metrics_history.len() as f64;
        avg_score < overall_avg * 0.9 // 10% degradation threshold
    }

    /// Determine the optimal performance mode based on learning.
    pub fn get_optimal_mode(&self, _operation_type: Option<String>) -> PerformanceMode {
        if self.metrics_history.is_empty() {
            return PerformanceMode::Balanced;
        }
        // Simple heuristic: use mode with best average score
        // In real implementation, would analyze by operation type
        PerformanceMode::Balanced
    }

    /// Adapt the profile based on the new optimal mode.
    pub fn adapt_profile(&mut self, new_mode: PerformanceMode) {
        self.profile.base_profile.mode = new_mode;
    }

    /// Get detailed adaptive learning statistics.
    pub fn get_adaptive_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("history_size".to_string(), serde_json::Value::Number(serde_json::Number::from(self.metrics_history.len())));
        stats.insert("learning_enabled".to_string(), serde_json::Value::Bool(self.profile.learning_enabled));
        stats
    }
}

// Initialize mode performance tracking
// Phase 1: CRUISE - Lightweight sampling
// Phase 2: DEEP_DIVE - Intensive sampling
// Keep cruise history small
// Track mode performance
// Keep only recent data
// Keep detailed history during deep-dive
// Track mode performance with more detail
// Keep more data during deep-dive
// Check system pressure
// Check performance degradation
// Trigger deep-dive if any threshold is exceeded
// Analyze deep-dive data and find optimal mode
// Return to cruise phase
// Calculate mode performance scores
// Use recent scores with higher weight
// Return the best performing mode
// Get the optimal profile
// Apply optimal settings to current profile
// Apply hybrid optimizations
// Adjust cache sizes based on memory pressure
// Cache system metrics based on current phase
// Keep only recent system metrics
// Fallback if psutil is not available
/// Smart dual-phase adaptive engine: fast cruise + intelligent deep-dive.
pub struct DualPhaseAdaptiveEngine {
    pub profile: DualAdaptiveProfile,
    cruise_metrics: Vec<PerformanceMetrics>,
    deep_dive_metrics: Vec<PerformanceMetrics>,
}

impl DualPhaseAdaptiveEngine {
    /// Constructor
    pub fn new(profile: DualAdaptiveProfile) -> Self {
        Self {
            profile,
            cruise_metrics: Vec::new(),
            deep_dive_metrics: Vec::new(),
        }
    }

    /// Record performance metrics with smart phase-based sampling.
    pub fn record_operation(&mut self, metrics: PerformanceMetrics) {
        // Phase 1: CRUISE - Lightweight sampling
        if self.profile.current_phase == "cruise" {
            // Sample every cruise_sample_rate operations (default 50)
            // For simplicity, we'll sample every operation but keep history small
            self._record_cruise_metric(metrics.clone());
            self._check_cruise_triggers(metrics);
        }
        // Phase 2: DEEP_DIVE - Intensive sampling
        else if self.profile.current_phase == "deep_dive" {
            // Sample every operation in deep-dive
            self._record_deep_dive_metric(metrics.clone());
            self._check_deep_dive_completion();
        }
    }

    // Keep cruise history small
    // Track mode performance
    // Keep only recent data
    /// Record metric in cruise phase (lightweight).
    pub fn _record_cruise_metric(&mut self, metrics: PerformanceMetrics) {
        self.cruise_metrics.push(metrics.clone());
        
        // Keep cruise history small
        if self.cruise_metrics.len() > 200 {
            self.cruise_metrics.remove(0);
        }
        
        // Track mode performance (simplified - would need proper mode tracking)
        // In production, you'd track mode performance separately
    }

    // Keep detailed history during deep-dive
    // Track mode performance with more detail
    // Keep more data during deep-dive
    /// Record metric in deep-dive phase (intensive).
    pub fn _record_deep_dive_metric(&mut self, metrics: PerformanceMetrics) {
        self.deep_dive_metrics.push(metrics);
        
        // Keep detailed history during deep-dive
        if self.deep_dive_metrics.len() > 1000 {
            self.deep_dive_metrics.remove(0);
        }
        
        // Track mode performance with more detail (simplified)
    }

    // Check system pressure
    // Check performance degradation
    // Trigger deep-dive if any threshold is exceeded
    /// Check if we should trigger deep-dive from cruise phase.
    pub fn _check_cruise_triggers(&mut self, _metrics: PerformanceMetrics) {
        // Check system pressure
        let system_metrics = self.get_system_metrics();
        let memory_pressure = system_metrics.get("memory_usage").copied().unwrap_or(0.5);
        let cpu_pressure = system_metrics.get("cpu_usage").copied().unwrap_or(0.3);
        
        // Check performance degradation
        let performance_degradation = self._calculate_performance_degradation();
        
        // Trigger deep-dive if any threshold is exceeded
        if memory_pressure > 0.8 || cpu_pressure > 0.7 || performance_degradation > 0.2 {
            self._trigger_deep_dive("System pressure or performance degradation detected".to_string());
        }
    }

    /// Check if deep-dive phase should complete.
    pub fn _check_deep_dive_completion(&mut self) {
        // Check if we've completed enough operations in deep-dive phase
        // In production, you'd track operations_in_phase properly
        if self.deep_dive_metrics.len() >= 500 {
            self._complete_deep_dive();
        }
    }

    /// Switch from cruise to deep-dive phase.
    pub fn _trigger_deep_dive(&mut self, reason: String) {
        self.profile.current_phase = "deep_dive".to_string();
        // In production, you'd set phase_start_time and reset operations_in_phase
        eprintln!("🔬 DUAL_ADAPTIVE: Entering DEEP_DIVE phase - {}", reason);
    }

    // Analyze deep-dive data and find optimal mode
    // Return to cruise phase
    /// Complete deep-dive and return to cruise with optimizations.
    pub fn _complete_deep_dive(&mut self) {
        // Analyze deep-dive data and find optimal mode
        let optimal_mode = self._analyze_deep_dive_data();
        
        // Apply optimizations
        self._apply_deep_dive_optimizations(optimal_mode);
        
        // Return to cruise phase
        self.profile.current_phase = "cruise".to_string();
        // In production, you'd reset phase_start_time and operations_in_phase
        eprintln!("🚗 DUAL_ADAPTIVE: Returning to CRUISE phase with optimizations");
    }

    /// Calculate recent performance degradation.
    pub fn _calculate_performance_degradation(&self) -> f64 {
        if self.cruise_metrics.len() < 10 {
            return 0.0;
        }
        
        // Get recent metrics (last 10)
        let recent: Vec<&PerformanceMetrics> = self.cruise_metrics.iter().rev().take(10).collect();
        let current_avg: f64 = recent.iter().map(|m| m.performance_score()).sum::<f64>() / recent.len() as f64;
        
        if self.cruise_metrics.len() >= 50 {
            // Get historical metrics (previous 40, before the recent 10)
            let historical: Vec<&PerformanceMetrics> = self.cruise_metrics.iter().rev().skip(10).take(40).collect();
            if !historical.is_empty() {
                let historical_avg: f64 = historical.iter().map(|m| m.performance_score()).sum::<f64>() / historical.len() as f64;
                
                if historical_avg > 0.0 {
                    return (historical_avg - current_avg) / historical_avg;
                }
            }
        }
        
        0.0
    }

    // Calculate mode performance scores
    // Use recent scores with higher weight
    // Return the best performing mode
    /// Analyze deep-dive data to find optimal performance mode.
    pub fn _analyze_deep_dive_data(&self) -> PerformanceMode {
        if self.deep_dive_metrics.len() < 20 {
            return PerformanceMode::Balanced; // Default to balanced
        }
        
        // Calculate average performance score from deep-dive metrics
        let avg_score: f64 = self.deep_dive_metrics.iter()
            .map(|m| m.performance_score())
            .sum::<f64>() / self.deep_dive_metrics.len() as f64;
        
        // Simple heuristic: choose mode based on average score
        // In production, you'd track mode performance separately
        if avg_score > 0.8 {
            PerformanceMode::Fast
        } else if avg_score > 0.5 {
            PerformanceMode::Balanced
        } else {
            PerformanceMode::MemoryOptimized
        }
    }

    // Get the optimal profile
    // Apply optimal settings to current profile
    // Apply hybrid optimizations
    /// Apply optimizations based on deep-dive analysis.
    pub fn _apply_deep_dive_optimizations(&mut self, optimal_mode: PerformanceMode) {
        // Get the optimal profile
        let optimal_profile = PerformanceProfiles::get_profile(optimal_mode, None);
        
        // Apply optimal settings to current profile
        // In production, you'd copy relevant fields from optimal_profile to self.profile
        // For now, we just update the mode in the cruise profile
        self.profile.cruise_profile.mode = optimal_mode;
        
        // Apply hybrid optimizations
        self._apply_hybrid_optimizations();
    }

    // Adjust cache sizes based on memory pressure
    /// Apply hybrid optimization strategies.
    pub fn _apply_hybrid_optimizations(&mut self) {
        let system_metrics = self.get_system_metrics();
        let memory_pressure = system_metrics.get("memory_usage").copied().unwrap_or(0.5);
        
        // Adjust cache sizes based on memory pressure
        // In production, you'd adjust actual cache sizes in the profile
        // For now, this is a placeholder that shows the logic
        if memory_pressure > 0.7 {
            // Reduce cache sizes under memory pressure
            // Would update profile.path_cache_size, etc.
        } else if memory_pressure < 0.3 {
            // Increase cache sizes when memory is available
            // Would update profile.path_cache_size, etc.
        }
    }

    // Cache system metrics based on current phase
    // Keep only recent system metrics
    // Fallback if psutil is not available
    /// Get current system metrics (cached for efficiency).
    pub fn get_system_metrics(&mut self) -> HashMap<String, f64> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        
        // Cache system metrics based on current phase
        let cache_duration = if self.profile.current_phase == "cruise" {
            10.0 // Cruise: check every 10 seconds
        } else {
            1.0 // Deep-dive: check every second
        };
        
        // In production, you'd cache the last check time and metrics
        // For now, we always return fallback metrics
        
        // Fallback if sysinfo is not available
        // In production, you'd use the sysinfo crate:
        // use sysinfo::{System, SystemExt, CpuExt};
        // let mut system = System::new_all();
        // system.refresh_all();
        // let memory = system.used_memory() as f64 / system.total_memory() as f64;
        // let cpu = system.global_cpu_info().cpu_usage() / 100.0;
        
        let mut metrics = HashMap::new();
        metrics.insert("memory_usage".to_string(), 0.5); // Assume moderate usage
        metrics.insert("cpu_usage".to_string(), 0.3); // Assume moderate usage
        metrics.insert("memory_available_mb".to_string(), 1024.0); // Assume 1GB available
        metrics.insert("timestamp".to_string(), current_time);
        metrics
    }

    /// Get detailed adaptive learning statistics.
    pub fn get_adaptive_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("current_phase".to_string(), serde_json::Value::String(self.profile.current_phase.clone()));
        stats.insert("cruise_metrics_count".to_string(), serde_json::Value::Number(serde_json::Number::from(self.cruise_metrics.len())));
        stats.insert("deep_dive_metrics_count".to_string(), serde_json::Value::Number(serde_json::Number::from(self.deep_dive_metrics.len())));
        stats
    }
}

// Start with DEFAULT profile and enhance with adaptive features
// Avoid recursion by directly creating the base profile
// Start with FAST profile for optimal cruise performance
// Avoid recursion by directly creating the base profile
// Cruise phase settings (fast, low-overhead)
// Sample every 100th operation (was 50)
// Check system every 10 seconds (was 5.0)
// Keep only 100 operations (was 200)
// Deep-dive phase settings (intensive learning)
// 25% degradation triggers deep-dive (was 0.15)
// Deep-dive for 200 operations (was 500)
// Sample every 5th operation (was 1)
// Keep detailed history (was 1000)
// 90% memory usage triggers deep-dive (was 0.8)
// 80% CPU usage triggers deep-dive (was 0.7)
// 30% degradation triggers deep-dive (was 0.2)
// 15% improvement threshold (was 0.1)
// 15 seconds between adaptations (was 10.0)
// Auto-selection based on data size
// For small data, use FAST mode but ensure thread safety is off
// For PARENT mode, we'll need to get the parent's profile
// This will be handled by the manager
// MANUAL mode requires explicit configuration
// For other types, estimate based on string representation
/// Predefined performance profiles for different optimization strategies.
pub struct PerformanceProfiles;

impl PerformanceProfiles {
    /// Get performance profile based on mode and data characteristics.
    pub fn get_profile(mode: PerformanceMode, _data_size: Option<i64>) -> PerformanceProfile {
        let mut settings = HashMap::new();
        match mode {
            PerformanceMode::Fast => {
                settings.insert("thread_safety".to_string(), serde_json::Value::Bool(false));
            }
            PerformanceMode::Balanced => {
                settings.insert("thread_safety".to_string(), serde_json::Value::Bool(true));
            }
            PerformanceMode::MemoryOptimized => {
                settings.insert("thread_safety".to_string(), serde_json::Value::Bool(true));
                settings.insert("cache_size".to_string(), serde_json::Value::Number(serde_json::Number::from(100)));
            }
        }
        PerformanceProfile { mode, settings }
    }

    /// Estimate the size/complexity of data for mode selection.
    pub fn estimate_data_size(data: &serde_json::Value) -> i64 {
        // Simple estimation based on JSON size
        serde_json::to_string(data).map(|s| s.len() as i64).unwrap_or(0)
    }
}

// Apply manual overrides
// Check if adaptation is needed
// Check if dual adaptive is needed
// For now, we'll just return the current mode, as dual adaptive is a separate engine
// The dual adaptive engine handles its own adaptation logic
/// Manager for performance mode selection and adaptation.
pub struct PerformanceModeManager {
    current_mode: PerformanceMode,
    parent_mode: Option<PerformanceMode>,
    manual_overrides: HashMap<String, serde_json::Value>,
}

impl PerformanceModeManager {
    /// Constructor
    pub fn new(default_mode: Option<PerformanceMode>) -> Self {
        Self {
            current_mode: default_mode.unwrap_or(PerformanceMode::Balanced),
            parent_mode: None,
            manual_overrides: HashMap::new(),
        }
    }

    /// Set the current performance mode.
    pub fn set_mode(&mut self, mode: PerformanceMode) {
        self.current_mode = mode;
    }

    /// Get the current performance mode.
    pub fn get_mode(&self) -> PerformanceMode {
        self.current_mode
    }

    /// Set the parent performance mode for inheritance.
    pub fn set_parent_mode(&mut self, mode: PerformanceMode) {
        self.parent_mode = Some(mode);
    }

    /// Set a manual override for specific configuration.
    pub fn set_manual_override(&mut self, key: String, value: serde_json::Value) {
        self.manual_overrides.insert(key, value);
    }

    /// Get the current performance profile.
    pub fn get_profile(&self, data_size: Option<i64>) -> PerformanceProfile {
        let effective_mode = self._get_effective_mode();
        let mut profile = PerformanceProfiles::get_profile(effective_mode, data_size);
        // Apply manual overrides
        for (key, value) in &self.manual_overrides {
            profile.settings.insert(key.clone(), value.clone());
        }
        profile
    }

    /// Get the effective mode considering inheritance and adaptation.
    fn _get_effective_mode(&self) -> PerformanceMode {
        self.parent_mode.unwrap_or(self.current_mode)
    }

    /// Record operation metrics for adaptive learning.
    pub fn record_operation(&self, _operation_type: String, _mode_used: PerformanceMode, _execution_time: f64, _memory_usage: f64, _cache_hits: i64, _cache_misses: i64) {
        // Metrics recording would be implemented here
    }

    /// Get adaptive learning statistics.
    pub fn get_adaptive_stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        stats.insert("current_mode".to_string(), serde_json::to_value(&self.current_mode).unwrap_or(serde_json::Value::Null));
        stats.insert("manual_overrides_count".to_string(), serde_json::Value::Number(serde_json::Number::from(self.manual_overrides.len())));
        stats
    }

    /// Reset the manager to default state.
    pub fn reset(&mut self) {
        self.current_mode = PerformanceMode::Balanced;
        self.parent_mode = None;
        self.manual_overrides.clear();
    }
}
