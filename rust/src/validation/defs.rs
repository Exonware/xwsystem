// #exonware/xwsystem/rust/src/validation/defs.rs
//exonware/xwsystem/validation/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! Validation types and enums for XWSystem.


use serde::{Serialize, Deserialize};

use crate::shared::defs::{ValidationLevel};

/// Validation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationType {
    Schema,
    Type,
    Range,
    Pattern,
    Custom,
}

/// Validation results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationResult {
    Valid,
    Invalid,
    Warning,
    Error,
}
