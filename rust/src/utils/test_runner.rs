// #exonware/xwsystem/rust/src/utils/test_runner.rs
//! #exonware/xwsystem/src/exonware/xwsystem/utils/test_runner.py
//! 
//! Reusable pytest runner utilities for all eXonware libraries.
//! 
//! Implements the hierarchical runner utilities described in:
//! - docs/guides/GUIDE_DEV.md
//! - docs/guides/GUIDE_TEST.md
//! 
//! Company: eXonware.com
//! Author: eXonware Backend Team
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 28-Dec-2025


use std::collections::HashMap;

use crate::__future__::{annotations};
use std::path::{Path};

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

/// Capture output for terminal and Markdown file simultaneously.
pub struct DualOutput {
    pub output_file: PathBuf,
    _markdown_lines: Vec<String>,
}

impl DualOutput {
    /// Constructor
    pub fn new(output_file: PathBuf) -> Self {
        Self {
            output_file,
            _markdown_lines: Vec::new(),
        }
    }

    pub fn print(&mut self, text: &str, markdown_format: Option<&str>, emoji: Option<&str>) {
        let prefix = emoji.map(|e| format!("{} ", e)).unwrap_or_default();
        let line = format!("{}{}", prefix, text);
        println!("{}", line);
        self._markdown_lines.push(
            markdown_format.unwrap_or(&line).to_string()
        );
    }

    pub fn save(&self, header_info: Option<&HashMap<String, String>>) -> Result<(), std::io::Error> {
        let header_info = header_info.unwrap_or(&HashMap::new());
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let now_str = format!("{}", now); // Simple timestamp
        let header = format!(
            "# Test Runner Output\n\n\
            **Library:** {}  \n\
            **Layer:** {}  \n\
            **Generated:** {}  \n\
            **Runner:** {}  \n\n\
            ---\n\n",
            header_info.get("library").unwrap_or(&String::new()),
            header_info.get("layer").unwrap_or(&String::new()),
            now_str,
            header_info.get("runner").unwrap_or(&"TestRunner".to_string())
        );
        
        let mut file = fs::File::create(&self.output_file)?;
        file.write_all(header.as_bytes())?;
        file.write_all(self._markdown_lines.join("\n").as_bytes())?;
        file.write_all(b"\n")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TestRunResult {
    pub exit_code: i32,
}

impl TestRunResult {
    pub fn new(exit_code: i32) -> Self {
        Self { exit_code }
    }
}

/// Simple reusable test runner for a single directory/layer.
pub struct TestRunner {
    pub library_name: String,
    pub layer_name: String,
    pub description: String,
    pub test_dir: PathBuf,
    pub markers: Vec<String>,
    pub output_file: PathBuf,
    pub output: DualOutput,
}

impl TestRunner {
    /// Constructor
    pub fn new(
        library_name: String,
        layer_name: String,
        description: String,
        test_dir: PathBuf,
        markers: Option<Vec<String>>,
        output_file: Option<PathBuf>,
    ) -> Self {
        let output_file = output_file.unwrap_or_else(|| test_dir.join("runner_out.md"));
        Self {
            library_name,
            layer_name,
            description,
            test_dir,
            markers: markers.unwrap_or_default(),
            output_file: output_file.clone(),
            output: DualOutput::new(output_file),
        }
    }

    pub fn run(&mut self) -> i32 {
        _configure_windows_utf8();

        print_header(&self.description, Some(&mut self.output));
        self.output.print(
            &format!("Directory: {}", format_path(&self.test_dir)),
            Some(&format!("**Directory:** `{}`", format_path(&self.test_dir))),
            Some("📂"),
        );

        let result = run_cargo_test(&self.test_dir, &self.markers);
        let ok = result.exit_code == 0;
        print_status(ok, if ok { "PASSED" } else { "FAILED" }, Some(&mut self.output));

        let mut header_info = HashMap::new();
        header_info.insert("library".to_string(), self.library_name.clone());
        header_info.insert("layer".to_string(), self.layer_name.clone());
        header_info.insert("runner".to_string(), "TestRunner".to_string());
        
        let _ = self.output.save(Some(&header_info));
        result.exit_code
    }
}

/// Configure UTF-8 for Windows consoles (GUIDE_TEST.md compliance).
pub fn _configure_windows_utf8() {
    #[cfg(windows)]
    {
        // On Windows, Rust already handles UTF-8 properly in most cases
        // This is a placeholder for any Windows-specific UTF-8 configuration
    }
}

/// Format a path as a full absolute path string.
pub fn format_path(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .to_string()
}

pub fn print_header(title: &str, output: Option<&mut DualOutput>) {
    let sep = "=".repeat(80);
    if let Some(out) = output {
        out.print(&sep, None, None);
        out.print(title, None, None);
        out.print(&sep, None, None);
    } else {
        println!("{}", sep);
        println!("{}", title);
        println!("{}", sep);
    }
}

pub fn print_section(title: &str, output: Option<&mut DualOutput>) {
    if let Some(out) = output {
        out.print(&format!("\n{}", title), Some(&format!("\n## {}\n", title)), None);
    } else {
        println!("\n{}\n{}\n{}\n", "=".repeat(80), title, "=".repeat(80));
    }
}

pub fn print_status(success: bool, message: &str, output: Option<&mut DualOutput>) {
    let emoji = if success { "✅" } else { "❌" };
    if let Some(out) = output {
        out.print(message, Some(&format!("**Result:** {} {}", emoji, message)), Some(emoji));
    } else {
        println!("{} {}", emoji, message);
    }
}

/// Run cargo test as a subprocess with standard eXonware flags.
pub fn run_cargo_test(test_dir: &Path, _markers: &[String]) -> TestRunResult {
    use std::process::Command;
    
    let result = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(test_dir.join("Cargo.toml"))
        .arg("--")
        .arg("--nocapture")
        .current_dir(test_dir)
        .status();
    
    match result {
        Ok(status) => TestRunResult::new(status.code().unwrap_or(1)),
        Err(_) => TestRunResult::new(1),
    }
}
