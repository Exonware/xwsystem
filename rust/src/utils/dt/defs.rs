// #exonware/xwsystem/rust/src/utils/dt/defs.rs
//exonware/xwsystem/datetime/types.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 07-Sep-2025
//! 
//! DateTime types and enums for XWSystem.

/// Time format types.
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFormat {
    Iso,
    Rfc2822,
    Rfc3339,
    Custom,
}

/// Date format types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateFormat {
    Iso,
    Us,
    Eu,
    Custom,
}

/// Timezone types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimezoneType {
    Utc,
    Local,
    Custom,
}

/// Humanize time units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanizeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

/// DateTime format types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DateTimeFormat {
    Iso,
    Rfc2822,
    Rfc3339,
    Us,
    Eu,
    Custom,
}

/// Humanize styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanizeStyle {
    Relative,
    Absolute,
    Natural,
    Precise,
}
