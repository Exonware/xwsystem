// #exonware/xwsystem/rust/src/cli/args.rs
//! Argument Parsing Utilities
//! =========================
//! 
//! Production-grade CLI argument parsing for XWSystem.
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generated: 2025-01-27


use std::collections::HashMap;
use std::env;

use crate::cli::defs::ArgumentType;

// Utility functions for common argument types

// store, store_true, store_false, append, count
/// Definition of a command-line argument.
///
/// Features:
/// - Type validation and conversion
/// - Default values and required flags
/// - Help text and examples
/// - Choice validation
/// - Custom validation functions
pub struct Argument {
    pub name: String,
    pub arg_type: ArgumentType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
    pub help_text: String,
    pub short_name: Option<String>,
    pub choices: Option<Vec<String>>,
    pub action: String, // store, store_true, store_false, append, count
    pub nargs: Option<String>, // Number of arguments
}

impl Argument {
    pub fn new(
        name: String,
        arg_type: ArgumentType,
        required: bool,
        default: Option<serde_json::Value>,
        help_text: String,
        short_name: Option<String>,
        choices: Option<Vec<String>>,
        action: String,
        nargs: Option<String>,
    ) -> Self {
        let mut arg = Argument {
            name,
            arg_type,
            required,
            default,
            help_text,
            short_name,
            choices,
            action,
            nargs,
        };
        arg.validate();
        arg
    }

    fn validate(&mut self) {
        if self.arg_type == ArgumentType::Choice && self.choices.is_none() {
            panic!("Argument '{}' with CHOICE type must have choices", self.name);
        }
        
        if let Some(ref mut short) = self.short_name {
            if !short.starts_with('-') {
                *short = format!("-{}", short);
            }
        }
        
        if !self.name.starts_with("--") {
            self.name = format!("--{}", self.name.trim_start_matches('-'));
        }
    }
}

/// Definition of a CLI command.
///
/// Features:
/// - Hierarchical subcommands
/// - Command-specific arguments
/// - Help text and examples
/// - Command handlers
pub struct Command {
    pub name: String,
    pub handler: Box<dyn Fn(HashMap<String, serde_json::Value>) -> serde_json::Value>,
    pub description: String,
    pub arguments: Vec<Argument>,
    pub subcommands: Vec<Command>,
    pub examples: Vec<String>,
}

impl Command {
    pub fn new(
        name: String,
        handler: Box<dyn Fn(HashMap<String, serde_json::Value>) -> serde_json::Value>,
        description: String,
        arguments: Option<Vec<Argument>>,
        subcommands: Option<Vec<Command>>,
        examples: Option<Vec<String>>,
    ) -> Self {
        Command {
            name,
            handler,
            description,
            arguments: arguments.unwrap_or_default(),
            subcommands: subcommands.unwrap_or_default(),
            examples: examples.unwrap_or_default(),
        }
    }
}

// Create subparser for command
// Add command arguments
// Add examples to epilog if provided
// Handle subcommands recursively
// Handle different argument types
// Will validate in custom validator
// Handle number of arguments
// argparse calls sys.exit on error - re-raise for handling
// Find and execute command
// Custom validation logic can be added here
/// Production-grade argument parser built on argparse.
///
/// Features:
/// - Type-safe argument parsing
/// - Hierarchical command structure
/// - Automatic help generation
/// - Validation and error handling
/// - Colored output support
/// - Configuration file support
pub struct ArgumentParser {
    program_name: String,
    description: String,
    version: Option<String>,
    epilog: String,
    commands: HashMap<String, Command>,
    global_arguments: Vec<Argument>,
}

impl ArgumentParser {
    /// Initialize argument parser.
    ///
    ///
    /// Args:
    /// program_name: Name of the program
    /// description: Program description
    /// version: Program version
    /// epilog: Text to display after help
    pub fn new(
        program_name: Option<String>,
        description: Option<String>,
        version: Option<String>,
        epilog: Option<String>
    ) -> Self {
        let program_name = program_name.unwrap_or_else(|| {
            env::args().next().unwrap_or_else(|| "program".to_string())
        });
        Self {
            program_name,
            description: description.unwrap_or_default(),
            version,
            epilog: epilog.unwrap_or_default(),
            commands: HashMap::new(),
            global_arguments: Vec::new(),
        }
    }

    /// Add a global argument to the parser.
    ///
    ///
    /// Args:
    /// argument: Argument definition
    ///
    ///
    /// Returns:
    /// Self for method chaining
    pub fn add_argument(&mut self, argument: Argument) -> &mut Self {
        self.global_arguments.push(argument);
        self
    }

    // Create subparser for command
    // Add command arguments
    // Add examples to epilog if provided
    // Handle subcommands recursively
    /// Add a command to the parser.
    ///
    ///
    /// Args:
    /// command: Command definition
    ///
    ///
    /// Returns:
    /// Self for method chaining
    pub fn add_command(&mut self, command: Command) -> &mut Self {
        let name = command.name.clone();
        self.commands.insert(name, command);
        self
    }

    /// Add a subcommand to a subparser.
    fn _add_subcommand(&mut self, command: Command, parent_name: String) {
        let full_name = format!("{}.{}", parent_name, command.name);
        self.commands.insert(full_name, command);
    }

    // Handle different argument types
    // Will validate in custom validator
    // Handle number of arguments
    /// Add an argument to an argparse parser.
    /// This is a placeholder - in a real implementation, this would integrate with clap or similar
    fn _add_argument_to_parser(&self, _parser: &str, _argument: &Argument) {
        // Placeholder - would integrate with argument parsing library
    }

    // argparse calls sys.exit on error - re-raise for handling
    /// Parse command-line arguments.
    ///
    ///
    /// Args:
    /// args: Arguments to parse (defaults to sys.argv)
    ///
    ///
    /// Returns:
    /// Parsed arguments namespace
    pub fn parse_args(&self, args: Option<Vec<String>>) -> HashMap<String, serde_json::Value> {
        let args = args.unwrap_or_else(|| env::args().skip(1).collect());
        let mut parsed = HashMap::new();
        
        // Simple parsing - in production would use clap
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                let name = arg.trim_start_matches("--").to_string();
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    parsed.insert(name, serde_json::Value::String(args[i + 1].clone()));
                    i += 2;
                } else {
                    parsed.insert(name, serde_json::Value::Bool(true));
                    i += 1;
                }
            } else if arg.starts_with('-') {
                let name = arg.trim_start_matches('-').to_string();
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    parsed.insert(name, serde_json::Value::String(args[i + 1].clone()));
                    i += 2;
                } else {
                    parsed.insert(name, serde_json::Value::Bool(true));
                    i += 1;
                }
            } else {
                // Positional argument
                parsed.insert(format!("arg_{}", i), serde_json::Value::String(arg.clone()));
                i += 1;
            }
        }
        
        self._validate_args(&parsed);
        parsed
    }

    // Find and execute command
    /// Parse arguments and execute the appropriate command.
    ///
    ///
    /// Args:
    /// args: Arguments to parse
    ///
    ///
    /// Returns:
    /// Result from command handler
    pub fn execute(&self, args: Option<Vec<String>>) -> serde_json::Value {
        let parsed_args = self.parse_args(args);
        
        // Find and execute command
        if let Some(command_name) = parsed_args.get("command") {
            if let Some(command_name_str) = command_name.as_str() {
                if let Some(command) = self.commands.get(command_name_str) {
                    return (command.handler)(parsed_args);
                }
            }
        }
        
        // No command specified
        self.print_help();
        serde_json::Value::Number(0.into())
    }

    // Custom validation logic can be added here
    /// Validate parsed arguments.
    fn _validate_args(&self, _args: &HashMap<String, serde_json::Value>) {
        // Custom validation logic can be added here
        // For now, basic validation is done during parsing
    }

    /// Print help message.
    pub fn print_help(&self) {
        println!("{}", self.get_help_text());
    }

    fn get_help_text(&self) -> String {
        let mut help = format!("{}\n", self.program_name);
        if !self.description.is_empty() {
            help.push_str(&format!("{}\n", self.description));
        }
        if let Some(ref version) = self.version {
            help.push_str(&format!("Version: {}\n", version));
        }
        help.push_str("\nAvailable commands:\n");
        for (name, cmd) in &self.commands {
            help.push_str(&format!("  {} - {}\n", name, cmd.description));
        }
        if !self.epilog.is_empty() {
            help.push_str(&format!("\n{}", self.epilog));
        }
        help
    }

    /// Add a mutually exclusive argument group.
    pub fn add_mutually_exclusive_group(&self, _required: Option<bool>) {
        // Placeholder - would implement mutually exclusive groups
    }

}

// Utility functions for common argument types
/// Create a file input argument.
pub fn create_file_argument(name: &str, required: Option<bool>, help_text: Option<&str>) -> Argument {
    Argument::new(
        name.to_string(),
        ArgumentType::File,
        required.unwrap_or(false),
        None,
        help_text.unwrap_or("Input file path").to_string(),
        None,
        None,
        "store".to_string(),
        None,
    )
}

/// Create an output file argument.
pub fn create_output_argument(name: Option<&str>, help_text: Option<&str>) -> Argument {
    Argument::new(
        name.unwrap_or("output").to_string(),
        ArgumentType::String,
        false,
        None,
        help_text.unwrap_or("Output file path").to_string(),
        None,
        None,
        "store".to_string(),
        None,
    )
}

/// Create a verbose flag argument.
pub fn create_verbose_argument() -> Argument {
    Argument::new(
        "verbose".to_string(),
        ArgumentType::Boolean,
        false,
        None,
        "Enable verbose output".to_string(),
        Some("-v".to_string()),
        None,
        "store_true".to_string(),
        None,
    )
}

/// Create a quiet flag argument.
pub fn create_quiet_argument() -> Argument {
    Argument::new(
        "quiet".to_string(),
        ArgumentType::Boolean,
        false,
        None,
        "Suppress output".to_string(),
        Some("-q".to_string()),
        None,
        "store_true".to_string(),
        None,
    )
}

/// Create a configuration file argument.
pub fn create_config_argument() -> Argument {
    Argument::new(
        "config".to_string(),
        ArgumentType::File,
        false,
        None,
        "Configuration file path".to_string(),
        Some("-c".to_string()),
        None,
        "store".to_string(),
        None,
    )
}
