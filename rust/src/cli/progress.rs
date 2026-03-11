// #exonware/xwsystem/rust/src/cli/progress.rs
//! Progress Bar Utilities
//! =====================
//! 
//! Production-grade progress indicators for XSystem.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 05, 2025


use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;

use crate::cli::colors::{Colors, Style, colorize};

// Utility functions

/// Configuration for progress indicators.
#[derive(Clone)]
pub struct ProgressConfig {
    pub width: i64,
    pub show_percentage: bool,
    pub show_count: bool,
    pub show_rate: bool,
    pub show_eta: bool,
    pub refresh_rate: f64,
    pub color: Colors,
    pub style: Option<Style>,
}

impl Default for ProgressConfig {
    fn default() -> Self {
        ProgressConfig {
            width: 50,
            show_percentage: true,
            show_count: true,
            show_rate: true,
            show_eta: true,
            refresh_rate: 0.1,
            color: Colors::Green,
            style: Some(Style::Bold),
        }
    }
}

impl ProgressConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

// Update rate calculation
// Update rate every second
// Check if we should refresh display
// Keep only recent samples (last 10 seconds)
// Add newline if complete
/// Production-grade progress bar with ETA and rate calculation.
///
/// Features:
/// - Real-time progress tracking
/// - ETA (Estimated Time of Arrival) calculation
/// - Rate calculation (items/second)
/// - Customizable appearance
/// - Thread-safe updates
/// - Context manager support
pub struct ProgressBar {
    total: i64,
    description: String,
    config: ProgressConfig,
    current: Arc<Mutex<i64>>,
    start_time: Instant,
    last_update: Arc<Mutex<Instant>>,
    rate_samples: Arc<Mutex<Vec<f64>>>,
    last_rate_update: Arc<Mutex<Instant>>,
    closed: Arc<Mutex<bool>>,
}

impl ProgressBar {
    /// Initialize progress bar.
    ///
    ///
    /// Args:
    /// total: Total number of items to process
    /// description: Description text
    /// config: Progress configuration
    /// file: Output file (defaults to stderr)
    pub fn new(
        total: i64,
        description: Option<String>,
        config: Option<ProgressConfig>,
        _file: Option<()>
    ) -> Self {
        let now = Instant::now();
        Self {
            total,
            description: description.unwrap_or_default(),
            config: config.unwrap_or_else(ProgressConfig::new),
            current: Arc::new(Mutex::new(0)),
            start_time: now,
            last_update: Arc::new(Mutex::new(now)),
            rate_samples: Arc::new(Mutex::new(Vec::new())),
            last_rate_update: Arc::new(Mutex::new(now)),
            closed: Arc::new(Mutex::new(false)),
        }
    }

    // Update rate calculation
    // Update rate every second
    // Check if we should refresh display
    /// Update progress by n items.
    ///
    ///
    /// Args:
    /// n: Number of items to add to progress
    pub fn update(&self, n: Option<i64>) {
        let n = n.unwrap_or(1);
        let mut current = self.current.lock().unwrap();
        let mut closed = self.closed.lock().unwrap();
        
        if *closed {
            return;
        }
        
        *current = (*current + n).min(self.total);
        let current_val = *current;
        drop(current);
        drop(closed);
        
        let now = Instant::now();
        let mut last_rate_update = self.last_rate_update.lock().unwrap();
        
        // Update rate calculation
        if now.duration_since(*last_rate_update).as_secs_f64() >= 1.0 {
            self._update_rate(now);
            *last_rate_update = now;
        }
        drop(last_rate_update);
        
        // Check if we should refresh display
        let mut last_update = self.last_update.lock().unwrap();
        if now.duration_since(*last_update).as_secs_f64() >= self.config.refresh_rate || current_val >= self.total {
            self._refresh_display(current_val);
            *last_update = now;
        }
    }

    /// Set absolute progress.
    ///
    ///
    /// Args:
    /// current: Current progress value
    pub fn set_progress(&self, current: i64) {
        let mut current_val = self.current.lock().unwrap();
        let mut closed = self.closed.lock().unwrap();
        
        if *closed {
            return;
        }
        
        *current_val = current.max(0).min(self.total);
        let val = *current_val;
        drop(current_val);
        drop(closed);
        
        self._refresh_display(val);
    }

    // Keep only recent samples (last 10 seconds)
    /// Update rate calculation.
    fn _update_rate(&self, current_time: Instant) {
        let elapsed = current_time.duration_since(self.start_time).as_secs_f64();
        if elapsed > 0.0 {
            let current = *self.current.lock().unwrap();
            let current_rate = current as f64 / elapsed;
            
            let mut samples = self.rate_samples.lock().unwrap();
            samples.push(current_rate);
            
            // Keep only recent samples (last 10 seconds)
            if samples.len() > 10 {
                samples.remove(0);
            }
        }
    }

    /// Get current processing rate.
    fn _get_rate(&self) -> f64 {
        let samples = self.rate_samples.lock().unwrap();
        if samples.is_empty() {
            let elapsed = Instant::now().duration_since(self.start_time).as_secs_f64();
            let current = *self.current.lock().unwrap();
            if elapsed > 0.0 {
                return current as f64 / elapsed;
            }
            return 0.0;
        }
        samples.iter().sum::<f64>() / samples.len() as f64
    }

    /// Get estimated time to completion.
    fn _get_eta(&self) -> Option<f64> {
        let current = *self.current.lock().unwrap();
        if current == 0 {
            return None;
        }
        
        let rate = self._get_rate();
        if rate <= 0.0 {
            return None;
        }
        
        let remaining = self.total - current;
        Some(remaining as f64 / rate)
    }

    /// Format time duration.
    fn _format_time(&self, seconds: Option<f64>) -> String {
        if let Some(secs) = seconds {
            if secs < 0.0 {
                return "??:??".to_string();
            }
            
            if secs < 60.0 {
                format!("{:02}s", secs as i64)
            } else if secs < 3600.0 {
                let minutes = (secs / 60.0) as i64;
                let secs = (secs % 60.0) as i64;
                format!("{:02}:{:02}", minutes, secs)
            } else {
                let hours = (secs / 3600.0) as i64;
                let minutes = ((secs % 3600.0) / 60.0) as i64;
                format!("{:02}:{:02}h", hours, minutes)
            }
        } else {
            "??:??".to_string()
        }
    }

    /// Format processing rate.
    fn _format_rate(&self, rate: f64) -> String {
        if rate >= 1000.0 {
            format!("{:.1}K/s", rate / 1000.0)
        } else if rate >= 1.0 {
            format!("{:.1}/s", rate)
        } else {
            format!("{:.2}/s", rate)
        }
    }

    // Add newline if complete
    /// Refresh the progress bar display.
    pub(crate) fn _refresh_display(&self, current: i64) {
        let closed = *self.closed.lock().unwrap();
        if closed {
            return;
        }
        
        // Calculate percentage
        let percentage = if self.total > 0 {
            (current as f64 / self.total as f64 * 100.0)
        } else {
            0.0
        };
        
        // Create progress bar
        let filled_width = if self.total > 0 {
            ((self.config.width as f64 * current as f64 / self.total as f64) as i64).min(self.config.width)
        } else {
            0
        };
        let empty_width = self.config.width - filled_width;
        
        let filled = "█".repeat(filled_width as usize);
        let empty = "░".repeat(empty_width as usize);
        let bar = format!("{}{}", filled, empty);
        
        // Apply color
        let colored_bar = colorize(&bar, self.config.color, self.config.style);
        
        // Build status line
        let mut parts = Vec::new();
        
        if !self.description.is_empty() {
            parts.push(self.description.clone());
        }
        
        parts.push(format!("|{}|", colored_bar));
        
        if self.config.show_percentage {
            parts.push(format!("{:5.1}%", percentage));
        }
        
        if self.config.show_count {
            parts.push(format!("({}/{})", current, self.total));
        }
        
        if self.config.show_rate {
            let rate = self._get_rate();
            parts.push(format!("[{}]", self._format_rate(rate)));
        }
        
        if self.config.show_eta {
            let eta = self._get_eta();
            parts.push(format!("ETA: {}", self._format_time(eta)));
        }
        
        // Write to output
        let line = parts.join(" ");
        print!("\r{}", line);
        io::stdout().flush().unwrap_or(());
        
        // Add newline if complete
        if current >= self.total {
            println!();
            io::stdout().flush().unwrap_or(());
        }
    }

    /// Close the progress bar.
    pub fn close(&self) {
        let mut closed = self.closed.lock().unwrap();
        if !*closed {
            *closed = true;
            let current = *self.current.lock().unwrap();
            if current < self.total {
                println!();
                io::stdout().flush().unwrap_or(());
            }
        }
    }

}

/// Spinning progress indicator for indeterminate tasks.
///
/// Features:
/// - Multiple spinner styles
/// - Custom messages
/// - Thread-safe operation
/// - Context manager support
pub struct SpinnerProgress {
    message: Arc<Mutex<String>>,
    frames: Vec<&'static str>,
    speed: f64,
    running: Arc<Mutex<bool>>,
    frame_index: Arc<Mutex<usize>>,
}

impl SpinnerProgress {
    const SPINNERS: &'static [(&'static str, &'static [&'static str])] = &[
        ("dots", &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        ("line", &["|", "/", "-", "\\"]),
        ("arrows", &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"]),
        ("bounce", &["⠁", "⠂", "⠄", "⠂"]),
    ];

impl SpinnerProgress {
    /// Initialize spinner.
    ///
    ///
    /// Args:
    /// message: Message to display
    /// spinner: Spinner style name
    /// speed: Animation speed in seconds
    /// file: Output file
    pub fn new(
        message: Option<String>,
        spinner: Option<String>,
        speed: Option<f64>,
        _file: Option<()>
    ) -> Self {
        let spinner_name = spinner.unwrap_or_else(|| "dots".to_string());
        let frames = Self::SPINNERS
            .iter()
            .find(|(name, _)| *name == spinner_name)
            .map(|(_, frames)| frames.to_vec())
            .unwrap_or_else(|| Self::SPINNERS[0].1.to_vec());
        
        Self {
            message: Arc::new(Mutex::new(message.unwrap_or_else(|| "Processing...".to_string()))),
            frames,
            speed: speed.unwrap_or(0.1),
            running: Arc::new(Mutex::new(false)),
            frame_index: Arc::new(Mutex::new(0)),
        }
    }

    /// Start the spinner animation.
    pub fn start(&self) -> &Self {
        let mut running = self.running.lock().unwrap();
        if !*running {
            *running = true;
            let message = Arc::clone(&self.message);
            let frames = self.frames.clone();
            let speed = self.speed;
            let running_clone = Arc::clone(&self.running);
            let frame_index = Arc::clone(&self.frame_index);
            
            thread::spawn(move || {
                let mut idx = 0;
                while *running_clone.lock().unwrap() {
                    let frame = frames[idx % frames.len()];
                    let colored_frame = colorize(frame, Colors::Cyan, Some(Style::Bold));
                    let msg = message.lock().unwrap();
                    print!("\r{} {}", colored_frame, *msg);
                    io::stdout().flush().unwrap_or(());
                    drop(msg);
                    
                    idx += 1;
                    *frame_index.lock().unwrap() = idx;
                    thread::sleep(Duration::from_secs_f64(speed));
                }
            });
        }
        self
    }

    /// Stop the spinner animation.
    pub fn stop(&self) {
        let mut running = self.running.lock().unwrap();
        if *running {
            *running = false;
            let msg = self.message.lock().unwrap();
            let clear_len = msg.len() + 10;
            print!("\r{}\r", " ".repeat(clear_len));
            io::stdout().flush().unwrap_or(());
        }
    }

    /// Update the spinner message.
    pub fn update_message(&self, message: String) {
        *self.message.lock().unwrap() = message;
    }

}

/// Multiple progress bars manager.
///
/// Features:
/// - Multiple concurrent progress bars
/// - Dynamic add/remove bars
/// - Synchronized display updates
/// - Thread-safe operations
pub struct MultiProgress {
    bars: Arc<Mutex<std::collections::HashMap<String, Arc<ProgressBar>>>>,
    active: Arc<Mutex<bool>>,
}

impl MultiProgress {
    /// Initialize multi-progress manager.
    ///
    ///
    /// Args:
    /// file: Output file
    pub fn new(_file: Option<()>) -> Self {
        Self {
            bars: Arc::new(Mutex::new(std::collections::HashMap::new())),
            active: Arc::new(Mutex::new(false)),
        }
    }

    /// Add a new progress bar.
    ///
    ///
    /// Args:
    /// bar_id: Unique identifier for the bar
    /// total: Total items for this bar
    /// description: Description text
    /// config: Progress configuration
    ///
    ///
    /// Returns:
    /// ProgressBar instance
    pub fn add_bar(&self, bar_id: String, total: i64, description: Option<String>, config: Option<ProgressConfig>) -> Arc<ProgressBar> {
        let mut bars = self.bars.lock().unwrap();
        if bars.contains_key(&bar_id) {
            panic!("Progress bar '{}' already exists", bar_id);
        }
        
        let bar = Arc::new(ProgressBar::new(total, description, config, None));
        bars.insert(bar_id, Arc::clone(&bar));
        *self.active.lock().unwrap() = true;
        bar
    }

    /// Remove a progress bar.
    pub fn remove_bar(&self, bar_id: String) {
        let mut bars = self.bars.lock().unwrap();
        bars.remove(&bar_id);
        
        if bars.is_empty() {
            *self.active.lock().unwrap() = false;
        }
    }

    /// Update all progress bar displays.
    pub fn update_display(&self) {
        let active = *self.active.lock().unwrap();
        if !active {
            return;
        }
        
        let bars = self.bars.lock().unwrap();
        let num_bars = bars.len();
        if num_bars > 0 {
            // Move cursor up to overwrite previous bars
            print!("\x1b[{}A", num_bars);
        }
        
        for bar in bars.values() {
            let current = *bar.current.lock().unwrap();
            bar._refresh_display(current);
            println!();
        }
        
        io::stdout().flush().unwrap_or(());
    }

    /// Custom write method for progress bars to use.
    pub fn write(&self, _text: String) {
        // Collect output for synchronized display
    }

    /// Custom flush method.
    pub fn flush(&self) {
        // Flush is handled by update_display
    }

    /// Close all progress bars.
    pub fn close_all(&self) {
        let mut bars = self.bars.lock().unwrap();
        for bar in bars.values() {
            bar.close();
        }
        bars.clear();
        *self.active.lock().unwrap() = false;
    }

}

/// Create a simple progress bar.
pub fn create_progress_bar(total: i64, description: Option<&str>) -> ProgressBar {
    ProgressBar::new(total, description.map(|s| s.to_string()), None, None)
}

/// Create a simple spinner.
pub fn create_spinner(message: Option<&str>) -> SpinnerProgress {
    SpinnerProgress::new(message.map(|s| s.to_string()), None, None, None)
}
