// #exonware/xwsystem/rust/src/io/archive/archive.rs
//exonware/xwsystem/src/exonware/xwsystem/io/archive/archive.py
//! Company: eXonware.com
//! Author: Eng. Muhammad AlShehri
//! Email: connect@exonware.com
//! Version: 0.1.0.1
//! Generation Date: 30-Oct-2025
//! 
//! Archive facade using registry pattern.
//! 
//! Like get_codec_for_file() - auto-detects format!
//! 
//! Priority 1 (Security): Safe archive operations
//! Priority 2 (Usability): Auto-detect format from extension
//! Priority 3 (Maintainability): Clean facade over registry
//! Priority 4 (Performance): Fast format lookup
//! Priority 5 (Extensibility): Add formats via registry!


use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::archive::formats::{get_archiver_by_id, get_archiver_for_file};
use crate::errors::ArchiveError;

// Get archiver from registry
// Delegate to format-specific handler
// Auto-detect format from archive extension
// Delegate to format-specific handler
/// Archive facade using registry (LIKE get_codec!).
///
/// Auto-detects format from file extension.
/// Delegates to format-specific handlers.
///
/// Examples:
/// >>> archive = Archive()
/// >>>
/// >>> # Auto-detect format
/// >>> archive.create([Path("file.txt")], Path("backup.zip"))  # Uses ZipArchiver
/// >>> archive.create([Path("file.txt")], Path("backup.tar.gz"))  # Uses TarArchiver
/// >>>
/// >>> # Future: 7z automatically supported when registered!
/// >>> archive.create([Path("file.txt")], Path("backup.7z"))  # Uses 7zArchiver
pub struct Archive;

impl Archive {
    pub fn new() -> Self {
        Archive
    }

    // Get archiver from registry
    // Delegate to format-specific handler
    /// Create archive - auto-detects handler.
    ///
    ///
    /// Args:
    /// files: List of files to archive
    /// output: Output archive path
    /// format: Format hint (default: auto-detect from output path)
    /// **opts: Format-specific options
    pub fn create(&self, files: Vec<PathBuf>, output: PathBuf, format: Option<String>, _opts: HashMap<String, String>) -> Result<(), ArchiveError> {
        // Get archiver from registry
        let archiver = if let Some(format_id) = format.as_ref() {
            if !format_id.is_empty() {
                get_archiver_by_id(format_id)
            } else {
                get_archiver_for_file(&output.to_string_lossy())
            }
        } else {
            get_archiver_for_file(&output.to_string_lossy())
        };

        let archiver = archiver.ok_or_else(|| {
            let format_str = format.as_deref().unwrap_or("auto");
            ArchiveError::new(format!("No archiver found for format: {} or {}", format_str, output.display()))
        })?;

        // Delegate to format-specific handler
        archiver.create(files, output);
        Ok(())
    }

    // Auto-detect format from archive extension
    // Delegate to format-specific handler
    /// Extract archive - auto-detects handler.
    pub fn extract(&self, archive: PathBuf, output_dir: PathBuf, members: Option<Vec<String>>, _opts: HashMap<String, String>) -> Result<Vec<PathBuf>, ArchiveError> {
        // Auto-detect format from archive extension
        let archiver = get_archiver_for_file(&archive.to_string_lossy())
            .ok_or_else(|| {
                ArchiveError::new(format!("No archiver found for: {}", archive.display()))
            })?;

        // Delegate to format-specific handler
        Ok(archiver.extract(archive, output_dir, members))
    }

    /// List archive contents - auto-detects handler.
    pub fn list_contents(&self, archive: PathBuf) -> Result<Vec<String>, ArchiveError> {
        // Auto-detect format
        let archiver = get_archiver_for_file(&archive.to_string_lossy())
            .ok_or_else(|| {
                ArchiveError::new(format!("No archiver found for: {}", archive.display()))
            })?;

        Ok(archiver.list_contents(archive))
    }

    /// Add file to archive - auto-detects handler.
    pub fn add_file(&self, archive: PathBuf, file: PathBuf, arcname: Option<String>) -> Result<(), ArchiveError> {
        // Auto-detect format
        let archiver = get_archiver_for_file(&archive.to_string_lossy())
            .ok_or_else(|| {
                ArchiveError::new(format!("No archiver found for: {}", archive.display()))
            })?;

        archiver.add_file(archive, file, arcname);
        Ok(())
    }
}

impl Default for Archive {
    fn default() -> Self {
        Self::new()
    }
}
