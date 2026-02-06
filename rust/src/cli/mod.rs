// #exonware/xwsystem/rust/src/cli/mod.rs
//! Command Line Interface (CLI) Utilities
//! ======================================
//! 
//! Production-grade CLI utilities for XSystem.
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generated: 2025-01-27

pub mod args;
pub mod base;
pub mod colors;
pub mod console;
pub mod contracts;
pub mod defs;
pub mod errors;
pub mod progress;
pub mod prompts;
pub mod tables;

pub use args::{Argument, ArgumentParser, Command};
pub use base::BaseCLI;
pub use colors::{ColoredOutput, Colors, Style, colorize};
pub use console::Console;
pub use defs::{Alignment, ArgumentType, BorderStyle, ColorType, ProgressStyle, PromptType, TableStyle};
pub use progress::{MultiProgress, ProgressBar, ProgressConfig, SpinnerProgress};
pub use tables::{Column, Table, TableFormatter};
