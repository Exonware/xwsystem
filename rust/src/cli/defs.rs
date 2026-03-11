// #exonware/xwsystem/rust/src/cli/defs.rs
//exonware/xwsystem/cli/types.py
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! CLI types and enums for XWSystem.

/// Color types for CLI output.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorType {
    Reset,
    Bold,
    Dim,
    Underline,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
}

/// Progress bar styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgressStyle {
    Bar,
    Spinner,
    Percentage,
    Counter,
}

/// Table display styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableStyle {
    Simple,
    Grid,
    Fancy,
    Minimal,
}

/// Prompt input types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptType {
    Text,
    Password,
    Confirm,
    Select,
    Multiselect,
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Table border styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Ascii,
    Simple,
    Rounded,
    Double,
    Thick,
}

/// ANSI color codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    Reset,
}

impl Colors {
    /// Get the ANSI escape code for this color.
    pub fn value(&self) -> &'static str {
        match self {
            Colors::Black => "\x1b[30m",
            Colors::Red => "\x1b[31m",
            Colors::Green => "\x1b[32m",
            Colors::Yellow => "\x1b[33m",
            Colors::Blue => "\x1b[34m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan => "\x1b[36m",
            Colors::White => "\x1b[37m",
            Colors::BrightBlack => "\x1b[90m",
            Colors::BrightRed => "\x1b[91m",
            Colors::BrightGreen => "\x1b[92m",
            Colors::BrightYellow => "\x1b[93m",
            Colors::BrightBlue => "\x1b[94m",
            Colors::BrightMagenta => "\x1b[95m",
            Colors::BrightCyan => "\x1b[96m",
            Colors::BrightWhite => "\x1b[97m",
            Colors::BgBlack => "\x1b[40m",
            Colors::BgRed => "\x1b[41m",
            Colors::BgGreen => "\x1b[42m",
            Colors::BgYellow => "\x1b[43m",
            Colors::BgBlue => "\x1b[44m",
            Colors::BgMagenta => "\x1b[45m",
            Colors::BgCyan => "\x1b[46m",
            Colors::BgWhite => "\x1b[47m",
            Colors::Reset => "\x1b[0m",
        }
    }
}

/// ANSI style codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Style {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Strikethrough,
}

impl Style {
    /// Get the ANSI escape code for this style.
    pub fn value(&self) -> &'static str {
        match self {
            Style::Reset => "\x1b[0m",
            Style::Bold => "\x1b[1m",
            Style::Dim => "\x1b[2m",
            Style::Italic => "\x1b[3m",
            Style::Underline => "\x1b[4m",
            Style::Blink => "\x1b[5m",
            Style::Reverse => "\x1b[7m",
            Style::Strikethrough => "\x1b[9m",
        }
    }
}

/// Types of command-line arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgumentType {
    String,
    #[serde(rename = "int")]
    Integer,
    Float,
    #[serde(rename = "bool")]
    Boolean,
    File,
    #[serde(rename = "dir")]
    Directory,
    Choice,
}
