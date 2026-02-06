// #exonware/xwsystem/rust/src/caching/rate_limiter.rs
//exonware/xwsystem/caching/rate_limiter.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 01-Nov-2025
//! 
//! Rate limiting for caching module - Security Priority #1.
//! Prevents DoS attacks via cache flooding.


use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::caching::errors::CacheRateLimitError;

// Refill tokens based on elapsed time
// Check if enough tokens available
// Calculate rate from recent timestamps
/// Token bucket rate limiter for cache operations.
///
/// Prevents DoS attacks by limiting the rate of cache operations.
///
/// Features:
/// - Token bucket algorithm
/// - Thread-safe implementation
/// - Configurable burst capacity
/// - Graceful degradation
pub struct RateLimiter {
    max_ops_per_second: i64,
    burst_capacity: i64,
    time_window: f64,
    tokens: Mutex<f64>,
    last_update: Mutex<f64>,
    total_requests: Mutex<i64>,
    rejected_requests: Mutex<i64>,
    timestamps: Mutex<VecDeque<f64>>,
}

impl RateLimiter {
    /// Initialize rate limiter.
    ///
    /// Args:
    /// max_ops_per_second: Maximum operations per second
    /// burst_capacity: Maximum burst size (default: max_ops_per_second)
    /// time_window: Time window in seconds
    pub fn new(
        max_ops_per_second: Option<i64>,
        burst_capacity: Option<i64>,
        time_window: Option<f64>
    ) -> Self {
        let max_ops = max_ops_per_second.unwrap_or(10000);
        if max_ops <= 0 {
            panic!("max_ops_per_second must be positive, got {}", max_ops);
        }
        
        let burst = burst_capacity.unwrap_or(max_ops);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        Self {
            max_ops_per_second: max_ops,
            burst_capacity: burst,
            time_window: time_window.unwrap_or(1.0),
            tokens: Mutex::new(burst as f64),
            last_update: Mutex::new(now),
            total_requests: Mutex::new(0),
            rejected_requests: Mutex::new(0),
            timestamps: Mutex::new(VecDeque::with_capacity(max_ops as usize)),
        }
    }

    /// Acquire tokens for an operation.
    ///
    /// Args:
    /// tokens: Number of tokens to acquire
    ///
    /// Returns:
    /// True if tokens acquired successfully
    ///
    /// Raises:
    /// CacheRateLimitError: If rate limit is exceeded
    pub fn acquire(&self, tokens: Option<i64>) -> Result<bool, CacheRateLimitError> {
        let token_count = tokens.unwrap_or(1) as f64;
        let mut tokens_guard = self.tokens.lock().unwrap();
        let mut last_update_guard = self.last_update.lock().unwrap();
        let mut total_requests_guard = self.total_requests.lock().unwrap();
        let mut rejected_requests_guard = self.rejected_requests.lock().unwrap();
        let mut timestamps_guard = self.timestamps.lock().unwrap();
        
        *total_requests_guard += 1;
        
        // Refill tokens based on elapsed time
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let elapsed = now - *last_update_guard;
        *tokens_guard = (*tokens_guard + (elapsed * self.max_ops_per_second as f64))
            .min(self.burst_capacity as f64);
        *last_update_guard = now;
        
        // Check if enough tokens available
        if *tokens_guard >= token_count {
            *tokens_guard -= token_count;
            timestamps_guard.push_back(now);
            if timestamps_guard.len() > self.max_ops_per_second as usize {
                timestamps_guard.pop_front();
            }
            Ok(true)
        } else {
            *rejected_requests_guard += 1;
            let current_rate = self._get_current_rate_internal(&timestamps_guard, now);
            Err(CacheRateLimitError::new(format!(
                "Rate limit exceeded: {} ops/sec. Current rate: {:.0} ops/sec. Please slow down and try again later.",
                self.max_ops_per_second, current_rate
            )))
        }
    }

    /// Try to acquire tokens without raising exception.
    ///
    /// Args:
    /// tokens: Number of tokens to acquire
    ///
    /// Returns:
    /// True if tokens acquired, False otherwise
    pub fn try_acquire(&self, tokens: Option<i64>) -> bool {
        self.acquire(tokens).unwrap_or(false)
    }

    /// Get current operations per second rate.
    ///
    /// Returns:
    /// Current ops/sec rate
    pub fn get_current_rate(&self) -> f64 {
        let timestamps_guard = self.timestamps.lock().unwrap();
        let last_update_guard = self.last_update.lock().unwrap();
        let now = *last_update_guard;
        self._get_current_rate_internal(&timestamps_guard, now)
    }

    fn _get_current_rate_internal(&self, timestamps: &VecDeque<f64>, now: f64) -> f64 {
        if timestamps.len() < 2 {
            return 0.0;
        }
        
        // Calculate rate from recent timestamps
        let recent_timestamps: Vec<f64> = timestamps.iter()
            .filter(|&&ts| now - ts <= self.time_window)
            .copied()
            .collect();
        
        if recent_timestamps.len() < 2 {
            return 0.0;
        }
        
        let time_span = now - recent_timestamps[0];
        if time_span > 0.0 {
            recent_timestamps.len() as f64 / time_span
        } else {
            0.0
        }
    }

    /// Get rate limiter statistics.
    ///
    /// Returns:
    /// Dictionary with statistics
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let total_requests = *self.total_requests.lock().unwrap();
        let rejected_requests = *self.rejected_requests.lock().unwrap();
        let tokens = *self.tokens.lock().unwrap();
        let current_rate = self.get_current_rate();
        
        let mut stats = HashMap::new();
        stats.insert("max_ops_per_second".to_string(), serde_json::Value::Number(serde_json::Number::from(self.max_ops_per_second)));
        stats.insert("burst_capacity".to_string(), serde_json::Value::Number(serde_json::Number::from(self.burst_capacity)));
        stats.insert("current_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(tokens).unwrap()));
        stats.insert("current_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(current_rate).unwrap()));
        stats.insert("total_requests".to_string(), serde_json::Value::Number(serde_json::Number::from(total_requests)));
        stats.insert("rejected_requests".to_string(), serde_json::Value::Number(serde_json::Number::from(rejected_requests)));
        
        let acceptance_rate = if total_requests > 0 {
            (total_requests - rejected_requests) as f64 / total_requests as f64
        } else {
            1.0
        };
        stats.insert("acceptance_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(acceptance_rate).unwrap()));
        
        stats
    }

    /// Reset rate limiter state.
    pub fn reset(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        *self.tokens.lock().unwrap() = self.burst_capacity as f64;
        *self.last_update.lock().unwrap() = now;
        *self.total_requests.lock().unwrap() = 0;
        *self.rejected_requests.lock().unwrap() = 0;
        self.timestamps.lock().unwrap().clear();
    }
}

/// Fixed window rate limiter.
///
/// Simpler than token bucket but can have burst issues at window boundaries.
pub struct FixedWindowRateLimiter {
    max_ops: i64,
    window_seconds: f64,
    current_window_start: Mutex<f64>,
    current_window_count: Mutex<i64>,
    total_requests: Mutex<i64>,
    rejected_requests: Mutex<i64>,
}

impl FixedWindowRateLimiter {
    /// Initialize fixed window rate limiter.
    ///
    /// Args:
    /// max_ops: Maximum operations per window
    /// window_seconds: Window size in seconds
    pub fn new(
        max_ops: Option<i64>,
        window_seconds: Option<f64>
    ) -> Self {
        let max = max_ops.unwrap_or(1000);
        let window = window_seconds.unwrap_or(1.0);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        Self {
            max_ops: max,
            window_seconds: window,
            current_window_start: Mutex::new(now),
            current_window_count: Mutex::new(0),
            total_requests: Mutex::new(0),
            rejected_requests: Mutex::new(0),
        }
    }

    /// Acquire permission for an operation.
    ///
    /// Returns:
    /// True if operation allowed
    ///
    /// Raises:
    /// CacheRateLimitError: If rate limit exceeded
    pub fn acquire(&self) -> Result<bool, CacheRateLimitError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let mut window_start = self.current_window_start.lock().unwrap();
        let mut window_count = self.current_window_count.lock().unwrap();
        let mut total_requests = self.total_requests.lock().unwrap();
        let mut rejected_requests = self.rejected_requests.lock().unwrap();
        
        *total_requests += 1;
        
        // Check if we need to start a new window
        if now - *window_start >= self.window_seconds {
            *window_start = now;
            *window_count = 0;
        }
        
        // Check if within limit
        if *window_count < self.max_ops {
            *window_count += 1;
            Ok(true)
        } else {
            *rejected_requests += 1;
            Err(CacheRateLimitError::new(format!(
                "Rate limit exceeded: {} ops per {} seconds. Please wait for the next window.",
                self.max_ops, self.window_seconds
            )))
        }
    }

    /// Try to acquire permission without raising exception.
    pub fn try_acquire(&self) -> bool {
        self.acquire().unwrap_or(false)
    }

    /// Get rate limiter statistics.
    pub fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let total_requests = *self.total_requests.lock().unwrap();
        let rejected_requests = *self.rejected_requests.lock().unwrap();
        let window_count = *self.current_window_count.lock().unwrap();
        
        let mut stats = HashMap::new();
        stats.insert("max_ops".to_string(), serde_json::Value::Number(serde_json::Number::from(self.max_ops)));
        stats.insert("window_seconds".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(self.window_seconds).unwrap()));
        stats.insert("current_window_count".to_string(), serde_json::Value::Number(serde_json::Number::from(window_count)));
        stats.insert("total_requests".to_string(), serde_json::Value::Number(serde_json::Number::from(total_requests)));
        stats.insert("rejected_requests".to_string(), serde_json::Value::Number(serde_json::Number::from(rejected_requests)));
        
        let acceptance_rate = if total_requests > 0 {
            (total_requests - rejected_requests) as f64 / total_requests as f64
        } else {
            1.0
        };
        stats.insert("acceptance_rate".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(acceptance_rate).unwrap()));
        
        stats
    }

    /// Reset rate limiter state.
    pub fn reset(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        *self.current_window_start.lock().unwrap() = now;
        *self.current_window_count.lock().unwrap() = 0;
        *self.total_requests.lock().unwrap() = 0;
        *self.rejected_requests.lock().unwrap() = 0;
    }
}


// =============================================================================
// EXPORT ALL (from __all__)
// =============================================================================
// Types exported via mod.rs
