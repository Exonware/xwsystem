// #exonware/xwsystem/rust/src/cli/console.rs
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! Console utilities for CLI operations.


use std::collections::HashMap;
use std::env;
use std::io::{self, Write};

use crate::cli::defs::ColorType;
use crate::cli::contracts::IConsole;
use crate::cli::errors::ConsoleError;

/// Console implementation for CLI operations.
pub struct Console {
    supports_color: bool,
    is_interactive: bool,
}

impl IConsole for Console {
    fn print(&self, text: String, color: Option<ColorType>) -> () {
        self.print(text, color, HashMap::new())
    }

    fn input(&self, prompt: String) -> String {
        self.input(prompt, HashMap::new())
    }

    fn clear(&self) -> () {
        self.clear()
    }
}

impl Console {
    /// Initialize console.
    pub fn new() -> Self {
        let supports_color = Self::_check_color_support();
        let is_interactive = Self::_check_interactive();
        Self {
            supports_color,
            is_interactive,
        }
    }

    // Check if we're in a terminal that supports color
    /// Check if terminal supports color.
    fn _check_color_support() -> bool {
        // Check environment variables
        if env::var("NO_COLOR").is_ok() {
            return false;
        }
        
        // Check TERM environment variable
        if let Ok(term) = env::var("TERM") {
            let term_lower = term.to_lowercase();
            if term_lower == "dumb" {
                return false;
            }
            if term_lower.contains("color") || 
               matches!(term_lower.as_str(), "xterm" | "xterm-256color" | "screen" | "tmux") {
                return true;
            }
        }
        
        // Check if stdout is a TTY (simplified - in real implementation would use atty crate)
        // For now, assume true if TERM is set
        env::var("TERM").is_ok()
    }

    /// Check if console is interactive.
    fn _check_interactive() -> bool {
        // Simplified check - in real implementation would use atty crate
        // For now, check if stdin appears to be a TTY
        // This is a basic implementation
        true // Assume interactive for now
    }

    /// Print text to console.
    pub fn print(&self, text: String, color: Option<ColorType>, _kwargs: HashMap<String, String>) -> () {
        let output = if let Some(col) = color {
            if self.supports_color {
                self._apply_color(text, col)
            } else {
                text
            }
        } else {
            text
        };
        
        println!("{}", output);
    }

    /// Get input from user.
    pub fn input(&self, prompt: String, _kwargs: HashMap<String, String>) -> String {
        if !self.is_interactive {
            panic!("{}", ConsoleError::new("Console is not interactive"));
        }
        
        print!("{}", prompt);
        io::stdout().flush().unwrap_or(());
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            panic!("{}", ConsoleError::new("Failed to get input"))
        });
        input.trim().to_string()
    }

    /// Clear console screen.
    pub fn clear(&self) -> () {
        #[cfg(windows)]
        {
            std::process::Command::new("cmd")
                .args(&["/C", "cls"])
                .status()
                .unwrap_or_else(|_| {
                    panic!("{}", ConsoleError::new("Failed to clear console"))
                });
        }
        
        #[cfg(not(windows))]
        {
            std::process::Command::new("clear")
                .status()
                .unwrap_or_else(|_| {
                    panic!("{}", ConsoleError::new("Failed to clear console"))
                });
        }
    }

    /// Apply color to text.
    fn _apply_color(&self, text: String, color: ColorType) -> String {
        let color_codes: std::collections::HashMap<ColorType, &str> = [
            (ColorType::Reset, "\x1b[0m"),
            (ColorType::Bold, "\x1b[1m"),
            (ColorType::Dim, "\x1b[2m"),
            (ColorType::Underline, "\x1b[4m"),
            (ColorType::Red, "\x1b[31m"),
            (ColorType::Green, "\x1b[32m"),
            (ColorType::Yellow, "\x1b[33m"),
            (ColorType::Blue, "\x1b[34m"),
            (ColorType::Magenta, "\x1b[35m"),
            (ColorType::Cyan, "\x1b[36m"),
            (ColorType::White, "\x1b[37m"),
            (ColorType::Gray, "\x1b[90m"),
        ].iter().cloned().collect();
        
        if let Some(code) = color_codes.get(&color) {
            format!("{}{}\x1b[0m", code, text)
        } else {
            text
        }
    }

    /// Get console size.
    pub fn get_size(&self) -> (i64, i64) {
        // Try to get terminal size, default to 80x24
        #[cfg(windows)]
        {
            // Windows implementation would use Windows API
            (80, 24)
        }
        
        #[cfg(not(windows))]
        {
            use std::process::Command;
            // Try to get size using stty or tput
            (80, 24) // Simplified - would need proper terminal size detection
        }
    }

    /// Check if console is interactive.
    pub fn is_interactive(&self) -> bool {
        self.is_interactive
    }

    /// Check if console supports color.
    pub fn supports_color(&self) -> bool {
        self.supports_color
    }

}
