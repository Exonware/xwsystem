// #exonware/xwsystem/rust/src/cli/colors.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 05, 2025
//! 
//! Colored terminal output utilities with cross-platform support.


use std::collections::HashMap;
use std::env;

use crate::cli::defs::{Colors, Style};

// Global colored output instance

// Global colored output instance

// Convenience functions

// Check environment variables
// Check if stdout is a TTY
// Check TERM environment variable
// Lazy installation ensures colorama is always available
// Works on all platforms
// Convert string colors to enum
// Simple gradient: alternate between start and end colors
/// Cross-platform colored terminal output.
///
/// Features:
/// - Automatic color detection
/// - Windows support via colorama
/// - Graceful fallback when colors not supported
/// - Rich formatting options
pub struct ColoredOutput {
    force_color: Option<bool>,
    auto_reset: bool,
    supports_color: bool,
}

impl ColoredOutput {
    /// Initialize colored output.
    ///
    ///
    /// Args:
    /// force_color: Force color output (None = auto-detect)
    /// auto_reset: Automatically reset colors after each output
    pub fn new(
        force_color: Option<bool>,
        auto_reset: Option<bool>
    ) -> Self {
        let supports_color = Self::_detect_color_support(force_color);
        Self {
            force_color,
            auto_reset: auto_reset.unwrap_or(true),
            supports_color,
        }
    }

    // Check environment variables
    // Check if stdout is a TTY
    // Check TERM environment variable
    // Lazy installation ensures colorama is always available
    // Works on all platforms
    /// Detect if terminal supports colors.
    fn _detect_color_support(force_color: Option<bool>) -> bool {
        if let Some(force) = force_color {
            return force;
        }
        
        // Check environment variables
        if env::var("NO_COLOR").is_ok() {
            return false;
        }
        
        if env::var("FORCE_COLOR").is_ok() {
            return true;
        }
        
        // Check TERM environment variable
        if let Ok(term) = env::var("TERM") {
            let term_lower = term.to_lowercase();
            if term_lower.contains("color") || 
               matches!(term_lower.as_str(), "xterm" | "xterm-256color" | "screen" | "tmux") {
                return true;
            }
        }
        
        // Assume color support if TERM is set
        env::var("TERM").is_ok()
    }

    /// Check if colored output is supported.
    pub fn supports_color(&self) -> bool {
        self.supports_color
    }

    // Convert string colors to enum
    /// Apply color and style to text.
    ///
    ///
    /// Args:
    /// text: Text to colorize
    /// color: Color to apply
    /// style: Optional style to apply
    ///
    ///
    /// Returns:
    /// Colored text (or plain text if colors not supported)
    pub fn colorize(&self, text: String, color: Colors, style: Option<Style>) -> String {
        if !self.supports_color {
            return text;
        }
        
        let mut color_str = color.value().to_string();
        if let Some(s) = style {
            color_str.push_str(s.value());
        }
        
        if self.auto_reset {
            format!("{}{}{}", color_str, text, Colors::Reset.value())
        } else {
            format!("{}{}", color_str, text)
        }
    }

    /// Print colored text.
    ///
    ///
    /// Args:
    /// text: Text to print
    /// color: Color to apply
    /// style: Optional style to apply
    /// **kwargs: Additional arguments for print()
    pub fn print_colored(&self, text: String, color: Colors, style: Option<Style>, _kwargs: HashMap<String, String>) -> () {
        let colored_text = self.colorize(text, color, style);
        println!("{}", colored_text);
    }

    /// Print success message in green.
    pub fn success(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(format!("✓ {}", text), Colors::Green, None, kwargs);
    }

    /// Print error message in red.
    pub fn error(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(format!("✗ {}", text), Colors::Red, None, kwargs);
    }

    /// Print warning message in yellow.
    pub fn warning(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(format!("⚠ {}", text), Colors::Yellow, None, kwargs);
    }

    /// Print info message in blue.
    pub fn info(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(format!("ℹ {}", text), Colors::Blue, None, kwargs);
    }

    /// Print debug message in dim style.
    pub fn debug(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(format!("🔧 {}", text), Colors::BrightBlack, Some(Style::Dim), kwargs);
    }

    /// Print header text in bold.
    pub fn header(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(text, Colors::White, Some(Style::Bold), kwargs);
    }

    /// Print subheader text in cyan.
    pub fn subheader(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(text, Colors::Cyan, Some(Style::Bold), kwargs);
    }

    /// Print highlighted text in magenta.
    pub fn highlight(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(text, Colors::Magenta, Some(Style::Bold), kwargs);
    }

    /// Print muted text in dim style.
    pub fn muted(&self, text: String, kwargs: HashMap<String, String>) -> () {
        self.print_colored(text, Colors::BrightBlack, Some(Style::Dim), kwargs);
    }

    /// Print text with rainbow colors (character by character).
    pub fn rainbow(&self, text: String, kwargs: HashMap<String, String>) -> () {
        if !self.supports_color {
            println!("{}", text);
            return;
        }
        
        let rainbow_colors = vec![
            Colors::Red, Colors::Yellow, Colors::Green,
            Colors::Cyan, Colors::Blue, Colors::Magenta
        ];
        
        let colored_chars: Vec<String> = text.chars()
            .enumerate()
            .map(|(i, char)| {
                let color = rainbow_colors[i % rainbow_colors.len()];
                self.colorize(char.to_string(), color, None)
            })
            .collect();
        
        println!("{}", colored_chars.join(""));
    }

    // Simple gradient: alternate between start and end colors
    /// Print text with gradient colors (simplified version).
    pub fn gradient(&self, text: String, start_color: Colors, end_color: Colors, kwargs: HashMap<String, String>) -> () {
        if !self.supports_color {
            println!("{}", text);
            return;
        }
        
        let colored_chars: Vec<String> = text.chars()
            .enumerate()
            .map(|(i, char)| {
                let color = if i % 2 == 0 { start_color } else { end_color };
                self.colorize(char.to_string(), color, None)
            })
            .collect();
        
        println!("{}", colored_chars.join(""));
    }

    /// Generate a colored progress bar.
    ///
    ///
    /// Args:
    /// current: Current progress value
    /// total: Total progress value
    /// width: Width of progress bar in characters
    /// fill_color: Color for filled portion
    /// empty_color: Color for empty portion
    ///
    ///
    /// Returns:
    /// Colored progress bar string
    pub fn progress_bar(&self, current: i64, total: i64, width: Option<i64>, fill_color: Option<Colors>, empty_color: Option<Colors>) -> String {
        let width = width.unwrap_or(50);
        let fill_color = fill_color.unwrap_or(Colors::Green);
        let empty_color = empty_color.unwrap_or(Colors::BrightBlack);
        
        let percent = if total == 0 {
            0.0
        } else {
            (current as f64 / total as f64 * 100.0).min(100.0).max(0.0)
        };
        
        let filled_width = ((width as f64 * percent / 100.0) as i64).min(width);
        let empty_width = width - filled_width;
        
        let filled_bar = self.colorize("█".repeat(filled_width as usize), fill_color, None);
        let empty_bar = self.colorize("░".repeat(empty_width as usize), empty_color, None);
        
        format!("[{}{}] {:5.1}% ({}/{})", filled_bar, empty_bar, percent, current, total)
    }

}

// Global colored output instance
static mut _COLORED_OUTPUT: Option<ColoredOutput> = None;

fn get_global_instance() -> &'static ColoredOutput {
    unsafe {
        if _COLORED_OUTPUT.is_none() {
            _COLORED_OUTPUT = Some(ColoredOutput::new(None, None));
        }
        _COLORED_OUTPUT.as_ref().unwrap()
    }
}

// Convenience functions
/// Colorize text using global instance.
pub fn colorize(text: &str, color: Colors, style: Option<Style>) -> String {
    get_global_instance().colorize(text.to_string(), color, style)
}

/// Print colored text using global instance.
pub fn print_colored(text: &str, color: Colors, style: Option<Style>, kwargs: HashMap<String, String>) -> () {
    get_global_instance().print_colored(text.to_string(), color, style, kwargs);
}

/// Print success message.
pub fn success(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().success(text.to_string(), kwargs);
}

/// Print error message.
pub fn error(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().error(text.to_string(), kwargs);
}

/// Print warning message.
pub fn warning(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().warning(text.to_string(), kwargs);
}

/// Print info message.
pub fn info(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().info(text.to_string(), kwargs);
}

/// Print debug message.
pub fn debug(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().debug(text.to_string(), kwargs);
}

/// Print header text.
pub fn header(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().header(text.to_string(), kwargs);
}

/// Print subheader text.
pub fn subheader(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().subheader(text.to_string(), kwargs);
}

/// Print highlighted text.
pub fn highlight(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().highlight(text.to_string(), kwargs);
}

/// Print muted text.
pub fn muted(text: &str, kwargs: HashMap<String, String>) -> () {
    get_global_instance().muted(text.to_string(), kwargs);
}

/// Check if colored output is supported.
pub fn supports_color() -> bool {
    get_global_instance().supports_color()
}
