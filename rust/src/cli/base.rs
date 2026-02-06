// #exonware/xwsystem/rust/src/cli/base.rs
//exonware/xwsystem/cli/base.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! CLI module base classes - abstract classes for command-line interface functionality.


use std::collections::HashMap;
use std::cell::RefCell;

use crate::contracts::{ColorType, ICLI, ProgressStyle, PromptType, TableStyle};
use crate::version::{__version__};

/// Abstract base class for console operations.
pub trait AConsoleBase {
    /// Print text to console.
    fn print(&self, text: String, color: Option<ColorType>) -> ();

    /// Get input from user.
    fn input(&self, prompt: String) -> String;

    /// Clear console screen.
    fn clear(&self) -> ();

    /// Get console size.
    fn get_size(&self) -> (i64, i64);

    /// Check if console is interactive.
    fn is_interactive(&self) -> bool;

}

/// Abstract base class for progress bar operations.
pub trait AProgressBarBase {
    /// Start progress bar.
    fn start(&self) -> ();

    /// Update progress.
    fn update(&self, increment: i64) -> ();

    /// Finish progress bar.
    fn finish(&self) -> ();

    /// Set progress description.
    fn set_description(&self, description: String) -> ();

}

/// Abstract base class for table operations.
pub trait ATableBase {
    /// Add row to table.
    fn add_row(&self) -> ();

    /// Render table as string.
    fn render(&self) -> String;

    /// Clear all rows.
    fn clear(&self) -> ();

    /// Get number of rows.
    fn get_row_count(&self) -> i64;

}

/// Abstract base class for user prompts.
pub trait APromptBase {
    /// Ask user a question.
    fn ask(&self, question: String, prompt_type: PromptType) -> serde_json::Value;

    /// Ask for confirmation.
    fn confirm(&self, message: String, default: bool) -> bool;

    /// Ask user to select from choices.
    fn select(&self, message: String, choices: Vec<String>, default: Option<String>) -> String;

    /// Ask user to select multiple choices.
    fn multiselect(&self, message: String, choices: Vec<String>, default: Option<Vec<String>>) -> Vec<String>;

}

/// Abstract base class for argument parsing.
pub trait AArgumentParserBase {
    /// Add argument to parser.
    fn add_argument(&self) -> ();

    /// Parse command line arguments.
    fn parse_args(&self, args: Option<Vec<String>>) -> serde_json::Value;

    /// Print help message.
    fn print_help(&self) -> ();

    /// Print usage message.
    fn print_usage(&self) -> ();

}

/// Abstract base class for color operations.
pub trait AColorBase {
    /// Apply color to text.
    fn colorize(&self, text: String, color: ColorType) -> String;

    /// Check if color is supported.
    fn supports_color(&self) -> bool;

    /// Get color codes mapping.
    fn get_color_codes(&self) -> HashMap<ColorType, String>;

}

// Basic implementation - can be overridden
/// Base CLI implementation.
pub struct BaseCLI {
    name: String,
    version: String,
    commands: RefCell<std::collections::HashMap<String, serde_json::Value>>,
    options: RefCell<std::collections::HashMap<String, serde_json::Value>>,
}

impl ICLI for BaseCLI {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn add_command(&self, name: String, command: serde_json::Value) -> () {
        self.commands.borrow_mut().insert(name, command);
    }

    fn add_option(&self, name: String, option: serde_json::Value) -> () {
        self.options.borrow_mut().insert(name, option);
    }

    fn run(&self, args: Option<Vec<String>>) -> i64 {
        BaseCLI::run(self, args)
    }

    fn get_help(&self) -> String {
        BaseCLI::get_help(self)
    }
}

impl BaseCLI {
    /// Initialize the CLI.
    ///
    ///
    /// Args:
    /// name: CLI name
    /// version: CLI version (defaults to package version)
    pub fn new(
        name: Option<String>,
        version: Option<String>
    ) -> Self {
        Self {
            name: name.unwrap_or_else(|| "xwsystem".to_string()),
            version: version.unwrap_or_else(|| __version__.to_string()),
            commands: RefCell::new(std::collections::HashMap::new()),
            options: RefCell::new(std::collections::HashMap::new()),
        }
    }

    /// Get CLI name.
    // Python decorators: @property
    pub fn name(&self) -> &str
    {
        &self.name
    }

    /// Get CLI version.
    // Python decorators: @property
    pub fn version(&self) -> &str
    {
        &self.version
    }

    /// Add a command to the CLI.
    ///
    ///
    /// Args:
    /// name: Command name
    /// command: Command implementation
    pub fn add_command(&mut self, name: String, command: serde_json::Value) -> ()
    {
        self.commands.borrow_mut().insert(name, command);
    }

    /// Add an option to the CLI.
    ///
    ///
    /// Args:
    /// name: Option name
    /// option: Option implementation
    pub fn add_option(&mut self, name: String, option: serde_json::Value) -> ()
    {
        self.options.borrow_mut().insert(name, option);
    }

    // Basic implementation - can be overridden
    /// Run the CLI.
    ///
    ///
    /// Args:
    /// args: Command line arguments
    ///
    ///
    /// Returns:
    /// Exit code
    pub fn run(&self, args: Option<Vec<String>>) -> i64
    {
        let args = args.unwrap_or_default();
        
        if args.is_empty() || args[0] == "-h" || args[0] == "--help" {
            println!("{}", self.get_help());
            return 0;
        }
        
        let command_name = &args[0];
        if let Some(_command) = self.commands.borrow().get(command_name) {
            // In a real implementation, we would call the command's run method
            // For now, just return success
            return 0;
        } else {
            println!("Unknown command: {}", command_name);
            println!("{}", self.get_help());
            return 1;
        }
    }

    /// Get help text.
    pub fn get_help(&self) -> String
    {
        let mut help_text = format!("{} v{} - XWSystem CLI\n\n", self.name, self.version);
        help_text.push_str("Available commands:\n");
        for cmd_name in self.commands.borrow().keys() {
            help_text.push_str(&format!("  {}\n", cmd_name));
        }
        help_text.push_str("\nUse -h or --help for this help message.");
        help_text
    }

}
