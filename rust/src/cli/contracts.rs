// #exonware/xwsystem/rust/src/cli/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! CLI module contracts - interfaces and enums for command-line interface functionality.


use crate::cli::defs::{Alignment, ArgumentType, BorderStyle, ColorType, Colors, ProgressStyle, PromptType, Style, TableStyle};

/// Interface for console operations.
pub trait IConsole {
    /// Print text to console.
    fn print(&self, text: String, color: Option<ColorType>) -> ();

    /// Get input from user.
    fn input(&self, prompt: String) -> String;

    /// Clear console screen.
    fn clear(&self) -> ();

}

/// Interface for progress bar operations.
pub trait IProgressBar {
    /// Start progress bar.
    fn start(&self, total: i64, description: String) -> ();

    /// Update progress.
    fn update(&self, increment: i64) -> ();

    /// Finish progress bar.
    fn finish(&self) -> ();

}

/// Interface for table operations.
pub trait ITable {
    /// Add row to table.
    fn add_row(&self) -> ();

    /// Render table as string.
    fn render(&self) -> String;

}

/// Interface for user prompts.
pub trait IPrompt {
    /// Ask user a question.
    fn ask(&self, question: String) -> serde_json::Value;

}

/// Interface for argument parsing.
pub trait IArgumentParser {
    /// Add argument to parser.
    fn add_argument(&self) -> ();

    /// Parse command line arguments.
    fn parse_args(&self, args: Option<Vec<String>>) -> serde_json::Value;

}

/// Interface for CLI operations.
pub trait ICLI {
    /// Get CLI name.
    // Python decorators: @property
    fn name(&self) -> &str;

    /// Get CLI version.
    // Python decorators: @property
    fn version(&self) -> &str;

    /// Add a command to the CLI.
    fn add_command(&self, name: String, command: serde_json::Value) -> ();

    /// Add an option to the CLI.
    fn add_option(&self, name: String, option: serde_json::Value) -> ();

    /// Run the CLI.
    fn run(&self, args: Option<Vec<String>>) -> i64;

    /// Get help text.
    fn get_help(&self) -> String;

}

/// Interface for progress operations.
pub trait IProgress {
    /// Start progress tracking.
    fn start(&self, total: i64, description: String) -> ();

    /// Update progress.
    fn update(&self, increment: i64) -> ();

    /// Finish progress tracking.
    fn finish(&self) -> ();

}

/// Interface for user prompts.
pub trait IPrompts {
    /// Ask user a question.
    fn ask(&self, question: String) -> serde_json::Value;

    /// Ask for confirmation.
    fn confirm(&self, message: String, default: bool) -> bool;

    /// Ask user to select from choices.
    fn select(&self, message: String, choices: Vec<String>, default: Option<String>) -> String;

}

/// Interface for table formatting.
pub trait ITableFormatter {
    /// Add row to table.
    fn add_row(&self) -> ();

    /// Render table as string.
    fn render(&self) -> String;

    /// Clear all rows.
    fn clear(&self) -> ();

}
