//! src/files.rs
//!
//! This module is responsible for file system operations, specifically
//! discovering all Rust source files within a given directory path.

use crate::error::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir; // Corrected: Changed to use WalkDir with a capital 'D'.

/// Recursively finds all Rust source files (`.rs`) in a given directory.
///
/// This function walks the directory tree starting from the provided `root_path`,
/// filtering for files that have the `.rs` extension. It ignores directories
/// and files that are not Rust source files.
///
/// # Arguments
///
/// * `root_path` - A reference to the path of the directory to search.
///
/// # Returns
///
/// A `Result` containing a `Vec<PathBuf>` of all found Rust files.
/// Each `PathBuf` is the full path to a `.rs` file.
///
/// # Errors
///
/// This function can return an error if the directory traversal fails
/// (e.g., due to permissions issues), which will be wrapped in our
/// custom `Error::Walkdir` variant.
pub fn find_rust_files(root_path: &Path) -> Result<Vec<PathBuf>> {
    println!("-> Discovering Rust files in '{}'...", root_path.display());

    // Corrected: Changed to use WalkDir with a capital 'D'.
    let walker = WalkDir::new(root_path);

    // Process the iterator to collect valid Rust file paths.
    // `filter_map` is used to both filter and map the entries.
    // `e.ok()` converts a `Result<DirEntry>` into an `Option<DirEntry>`,
    // effectively skipping any entries that result in an error during traversal.
    let rust_files: Vec<PathBuf> = walker
        .into_iter()
        .filter_map(|e| e.ok()) // Ignore any errors during iteration.
        .filter(|e| {
            // We are only interested in files that have the ".rs" extension.
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .map_or(false, |ext| ext == "rs")
        })
        .map(|e| e.into_path()) // Convert the DirEntry into a PathBuf.
        .collect();

    println!("-> Found {} Rust file(s).", rust_files.len());

    Ok(rust_files)
}
