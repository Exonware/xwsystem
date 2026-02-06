// #exonware/xwsystem/rust/src/utils/dt/contracts.rs
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 04, 2025
//! 
//! DateTime module contracts - interfaces and enums for date/time functionality.


use crate::defs::{DateFormat, DateTimeFormat, HumanizeStyle, HumanizeUnit, TimeFormat, TimezoneType};

/// Interface for date/time formatting.
pub trait IDateTimeFormatter {
    /// Format datetime object.
    fn format_datetime(&self, dt: datetime, format_type: TimeFormat) -> String;

    /// Format date object.
    fn format_date(&self, d: date, format_type: DateFormat) -> String;

    /// Format time object.
    fn format_time(&self, t: time, format_type: TimeFormat) -> String;

}

/// Interface for date/time parsing.
pub trait IDateTimeParser {
    /// Parse datetime string.
    fn parse_datetime(&self, date_string: String, format_type: Option<TimeFormat>) -> datetime;

    /// Parse date string.
    fn parse_date(&self, date_string: String, format_type: Option<DateFormat>) -> date;

    /// Parse time string.
    fn parse_time(&self, time_string: String, format_type: Option<TimeFormat>) -> time;

}

/// Interface for humanizing time differences.
pub trait IDateTimeHumanizer {
    /// Humanize datetime relative to reference.
    fn humanize(&self, dt: datetime, reference: Option<datetime>) -> String;

    /// Get natural time representation.
    fn natural_time(&self, dt: datetime, reference: Option<datetime>) -> String;

}

/// Interface for timezone utilities.
pub trait ITimezoneUtils {
    /// Get timezone object.
    fn get_timezone(&self, tz_name: String) -> timezone;

    /// Convert datetime to target timezone.
    fn convert_timezone(&self, dt: datetime, target_tz: timezone) -> datetime;

    /// Get local timezone.
    fn get_local_timezone(&self) -> timezone;

}
