// #exonware/xwsystem/rust/src/utils/paths.rs
//! Path utilities for XSystem.
//! Provides common path operations and project root detection.

use std::env;
use std::path::{Path, PathBuf};

/// Get the project root directory.
pub fn get_project_root(
    from_file: Option<&str>,
    levels_up: Option<usize>,
) -> Result<PathBuf, std::io::Error> {
    let levels = levels_up.unwrap_or(7);
    let start_path = if let Some(file) = from_file {
        PathBuf::from(file)
    } else {
        // Fallback to current working directory
        env::current_dir()?
    };

    let mut current_path = start_path.canonicalize()?;

    // If it's a file, start from its parent directory
    if current_path.is_file() {
        current_path = current_path.parent()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Cannot get parent of file path"
            ))?
            .to_path_buf();
    }

    // Traverse up the specified number of levels
    for _ in 0..levels {
        current_path = current_path.parent()
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Project root not found after traversing {} levels up from {:?}",
                    levels,
                    from_file
                )
            ))?
            .to_path_buf();
        
        if !current_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Project root not found after traversing {} levels up from {:?}",
                    levels,
                    from_file
                )
            ));
        }
    }

    Ok(current_path)
}

/// Get the src directory path within the project.
pub fn get_src_path(
    from_file: Option<&str>,
    levels_up: Option<usize>,
) -> Result<PathBuf, std::io::Error> {
    let project_root = get_project_root(from_file, levels_up)?;
    let src_path = project_root.join("src");

    if !src_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("src directory not found at {:?}", src_path)
        ));
    }

    Ok(src_path)
}

/// Setup project and src paths.
/// Note: In Rust, we don't modify a global path like Python's sys.path.
/// This function just returns the paths for the caller to use.
pub fn setup_paths(
    from_file: Option<&str>,
    levels_up: Option<usize>,
) -> Result<(PathBuf, PathBuf), std::io::Error> {
    let project_root = get_project_root(from_file, levels_up)?;
    let src_path = get_src_path(from_file, levels_up)?;
    Ok((project_root, src_path))
}

/// Utility class for path operations.
pub struct PathUtils;

impl PathUtils {
    /// Get the project root directory.
    pub fn get_project_root(
        from_file: Option<&str>,
        levels_up: Option<usize>,
    ) -> Result<PathBuf, std::io::Error> {
        get_project_root(from_file, levels_up)
    }

    /// Get the src directory path.
    pub fn get_src_path(
        from_file: Option<&str>,
        levels_up: Option<usize>,
    ) -> Result<PathBuf, std::io::Error> {
        get_src_path(from_file, levels_up)
    }

    /// Setup project and src paths.
    pub fn setup_paths(
        from_file: Option<&str>,
        levels_up: Option<usize>,
    ) -> Result<(PathBuf, PathBuf), std::io::Error> {
        setup_paths(from_file, levels_up)
    }

    /// Normalize a path string.
    pub fn normalize_path(path: &str) -> Result<PathBuf, std::io::Error> {
        PathBuf::from(path).canonicalize()
    }

    /// Ensure directory exists.
    pub fn ensure_dir(path: &Path) -> Result<PathBuf, std::io::Error> {
        std::fs::create_dir_all(path)?;
        Ok(path.to_path_buf())
    }

    /// Check if path is relative to another path.
    pub fn is_relative_to(path: &Path, other: &Path) -> bool {
        path.strip_prefix(other).is_ok()
    }
}
