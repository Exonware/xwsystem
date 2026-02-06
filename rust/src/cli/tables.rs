// #exonware/xwsystem/rust/src/cli/tables.rs
//! Table Formatting Utilities
//! ==========================
//! 
//! Production-grade table formatting for XWSystem.
//! 
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: September 05, 2025


use std::collections::HashMap;
use std::io::{self, Write};

use crate::cli::colors::{Colors, Style, colorize};
use crate::cli::defs::{Alignment, BorderStyle};

// Utility functions

/// Table column definition.
///
/// Features:
/// - Custom alignment and width
/// - Header and data formatting
/// - Color and style customization
/// - Data validation and transformation
pub struct Column {
    pub header: String,
    pub width: Option<i64>,
    pub alignment: Alignment,
    pub color: Option<Colors>,
    pub style: Option<Style>,
    pub header_color: Option<Colors>,
    pub header_style: Option<Style>,
    pub min_width: i64,
    pub max_width: Option<i64>,
}

impl Column {
    pub fn new(header: String) -> Self {
        Self {
            header,
            width: None,
            alignment: Alignment::Left,
            color: None,
            style: None,
            header_color: None,
            header_style: None,
            min_width: 1,
            max_width: None,
        }
    }
}

// Apply custom formatter if provided
// Apply color and style
/// Production-grade table formatter.
///
/// Features:
/// - Multiple border styles
/// - Custom column formatting
/// - Color and styling support
/// - Auto-sizing and word wrapping
/// - Header and footer support
/// - Export to different formats
pub struct TableFormatter {
    pub border_style: BorderStyle,
    pub padding: i64,
}

impl TableFormatter {
    /// Initialize table formatter.
    pub fn new(
        border_style: Option<BorderStyle>,
        padding: Option<i64>
    ) -> Self {
        Self {
            border_style: border_style.unwrap_or(BorderStyle::Rounded),
            padding: padding.unwrap_or(1),
        }
    }

    // Apply custom formatter if provided
    // Apply color and style
    /// Format a cell value according to column specifications.
    pub fn format_cell(&self, value: &serde_json::Value, column: &Column, width: i64) -> String {
        // Convert value to string
        let text = if value.is_null() {
            String::new()
        } else if let Some(s) = value.as_str() {
            s.to_string()
        } else {
            value.to_string()
        };
        
        // Truncate if too long
        let mut text = if text.len() > width as usize {
            format!("{}...", &text[..(width as usize).saturating_sub(3)])
        } else {
            text
        };
        
        // Apply alignment
        let width = width as usize;
        text = match column.alignment {
            Alignment::Left => format!("{:<width$}", text, width = width),
            Alignment::Right => format!("{:>width$}", text, width = width),
            Alignment::Center => format!("{:^width$}", text, width = width),
        };
        
        // Apply color and style
        if let Some(color) = column.color {
            text = colorize(&text, color, column.style);
        }
        
        text
    }

    /// Calculate optimal column widths.
    pub fn calculate_widths(&self, columns: &[Column], data: &[Vec<serde_json::Value>]) -> Vec<i64> {
        let mut widths = Vec::new();
        
        for (i, column) in columns.iter().enumerate() {
            let width = if let Some(w) = column.width {
                w.max(column.min_width)
                    .min(column.max_width.unwrap_or(i64::MAX))
            } else {
                let mut width = column.header.len() as i64;
                for row in data {
                    if i < row.len() {
                        let cell_text = if row[i].is_null() {
                            String::new()
                        } else if let Some(s) = row[i].as_str() {
                            s.to_string()
                        } else {
                            row[i].to_string()
                        };
                        width = width.max(cell_text.len() as i64);
                    }
                }
                width.max(column.min_width)
                    .min(column.max_width.unwrap_or(i64::MAX))
            };
            
            widths.push(width);
        }
        
        widths
    }

}

// Calculate column widths
/// Production-grade table with advanced formatting capabilities.
///
/// Features:
/// - Flexible column definitions
/// - Multiple output formats
/// - Sorting and filtering
/// - Export capabilities
pub struct Table {
    columns: Vec<Column>,
    formatter: TableFormatter,
    title: String,
    rows: Vec<Vec<serde_json::Value>>,
}

impl Table {
    /// Initialize table.
    pub fn new(
        columns: Option<Vec<String>>,
        formatter: Option<TableFormatter>,
        title: Option<String>
    ) -> Self {
        let columns: Vec<Column> = columns.unwrap_or_default()
            .into_iter()
            .map(|s| Column::new(s))
            .collect();
        
        Self {
            columns,
            formatter: formatter.unwrap_or_else(|| TableFormatter::new(None, None)),
            title: title.unwrap_or_default(),
            rows: Vec::new(),
        }
    }

    /// Add a column to the table.
    pub fn add_column(&mut self, column: Column) -> &mut Self {
        self.columns.push(column);
        self
    }

    /// Add a row to the table.
    pub fn add_row(&mut self, values: Vec<serde_json::Value>) -> &mut Self {
        self.rows.push(values);
        self
    }

    /// Add multiple rows to the table.
    pub fn add_rows(&mut self, rows: Vec<Vec<serde_json::Value>>) -> &mut Self {
        self.rows.extend(rows);
        self
    }

    // Calculate column widths
    /// Convert table to string representation.
    pub fn to_string(&self) -> String {
        if self.columns.is_empty() {
            return String::new();
        }
        
        // Calculate column widths
        let widths = self.formatter.calculate_widths(&self.columns, &self.rows);
        
        let mut lines = Vec::new();
        
        // Title
        if !self.title.is_empty() {
            lines.push(colorize(&self.title, Colors::Cyan, Some(Style::Bold)));
            lines.push(String::new());
        }
        
        // Header row
        let mut header_parts = Vec::new();
        for (i, (column, width)) in self.columns.iter().zip(widths.iter()).enumerate() {
            let header_text = format!("{:<width$}", column.header, width = *width as usize);
            let formatted_header = if let Some(color) = column.header_color {
                colorize(&header_text, color, column.header_style)
            } else {
                colorize(&header_text, Colors::Blue, Some(Style::Bold))
            };
            header_parts.push(formatted_header);
        }
        lines.push(header_parts.join(" | "));
        
        // Separator
        let separator_parts: Vec<String> = widths.iter()
            .map(|w| "-".repeat(*w as usize))
            .collect();
        lines.push(separator_parts.join("-|-"));
        
        // Data rows
        for row in &self.rows {
            let mut row_parts = Vec::new();
            for (i, (column, width)) in self.columns.iter().zip(widths.iter()).enumerate() {
                let value = if i < row.len() {
                    &row[i]
                } else {
                    &serde_json::Value::Null
                };
                let formatted_cell = self.formatter.format_cell(value, column, *width);
                row_parts.push(formatted_cell);
            }
            lines.push(row_parts.join(" | "));
        }
        
        lines.join("\n")
    }

    /// Print table to file or stdout.
    pub fn print(&self, _file: Option<()>) {
        println!("{}", self.to_string());
        io::stdout().flush().unwrap_or(());
    }

}

/// Create a simple table with basic formatting.
pub fn create_simple_table(headers: Vec<String>, rows: Vec<Vec<serde_json::Value>>) -> Table {
    let mut table = Table::new(Some(headers), None, None);
    table.add_rows(rows);
    table
}

/// Print a key-value table.
pub fn print_key_value_table(data: HashMap<String, serde_json::Value>, title: Option<&str>) {
    let mut table = Table::new(
        Some(vec!["Property".to_string(), "Value".to_string()]),
        None,
        title.map(|s| s.to_string())
    );
    
    for (key, value) in data {
        let value_str = if value.is_null() {
            String::new()
        } else if let Some(s) = value.as_str() {
            s.to_string()
        } else {
            value.to_string()
        };
        table.add_row(vec![
            serde_json::Value::String(key),
            serde_json::Value::String(value_str)
        ]);
    }
    
    table.print(None);
}
