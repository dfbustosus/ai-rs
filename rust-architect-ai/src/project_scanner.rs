//! src/project_scanner.rs
//!
//! This module is responsible for scanning a project directory, discovering
//! all relevant Rust source files, and aggregating their content into a
//! single context for analysis.

use crate::error::Result;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;
use walkdir::WalkDir;

/// Scans the given project path, finds all Rust files, and consolidates
/// their content into a single string.
///
/// Each file's content is prefixed with a clear header indicating its path,
/// providing essential context for the AI model's analysis.
///
/// # Arguments
///
/// * `root_path` - A reference to the root directory of the project to scan.
///
/// # Returns
///
/// A `Result` containing a single `String` with the combined content of all
/// found `.rs` files.
///
/// # Errors
///
/// This function can return an `Error` if directory traversal or file
/// reading fails.
pub fn scan_project(root_path: &Path) -> Result<String> {
    info!("Starting project scan at '{}'...", root_path.display());

    let rust_files = find_rust_files(root_path)?;
    let total_files = rust_files.len();
    info!("Found {} Rust source files to process.", total_files);

    if total_files == 0 {
        return Ok(String::new());
    }

    let mut combined_context = String::new();

    for (index, file_path) in rust_files.iter().enumerate() {
        info!(
            "Processing file {}/{}: {}",
            index + 1,
            total_files,
            file_path.display()
        );
        let file_content = fs::read_to_string(file_path)?;

        // Create a structured header for each file to provide clear context to the AI.
        let file_header = format!(
            "\n\n======================================\n// FILE: {}\n======================================\n\n",
            file_path.strip_prefix(root_path).unwrap_or(file_path).display()
        );

        combined_context.push_str(&file_header);
        combined_context.push_str(&file_content);
    }

    info!("Project scan complete. All file contents have been aggregated.");
    Ok(combined_context)
}

/// Discovers all Rust files (`.rs`) within a given directory, ignoring common
/// target and git directories.
fn find_rust_files(root_path: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    let walker = WalkDir::new(root_path)
        .into_iter()
        // Use filter_entry to efficiently prune entire directories.
        .filter_entry(|e| !is_ignored_dir(e.path()));

    for entry in walker {
        let entry = entry?; // Propagate errors from walking the directory.
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

/// A helper function to determine if a directory should be ignored.
fn is_ignored_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == "target" || name == ".git")
        .unwrap_or(false)
}
