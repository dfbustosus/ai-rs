//! src/analyzer.rs
//!
//! This module contains the core logic for the code analysis process.
//! It orchestrates reading files, sending them to the OpenAI client for
//! review, and displaying the results.

use crate::error::Result;
use crate::openai;
use colored::Colorize;
use std::fs;
use std::path::Path;

/// Analyzes a single Rust source file using the OpenAI API.
///
/// This function performs the following steps:
/// 1. Prints the name of the file being analyzed.
/// 2. Reads the file's content into a string.
/// 3. Passes the content to the provided OpenAI client.
/// 4. Prints the AI-generated analysis in a formatted block.
///
/// # Arguments
///
/// * `client` - An instance of `openai::Client` to communicate with the API.
/// * `file_path` - A reference to the path of the file to be analyzed.
///
/// # Returns
///
/// A `Result<()>` which will be `Ok(())` on success, or an `Err` if any
/// step (file reading, API communication) fails.
pub async fn analyze_file(client: &openai::Client, file_path: &Path) -> Result<()> {
    // Print a header for the file being analyzed.
    println!("\n{}", "==================================================".blue());
    println!(
        "{} {}",
        "Analyzing:".blue().bold(),
        file_path.display().to_string().bright_white()
    );
    println!("{}", "==================================================".blue());

    // Read the file content into a string. The `?` operator will propagate
    // any I/O errors, which our main function will handle.
    let file_content = fs::read_to_string(file_path)?;

    // Use the client to send the code for analysis. This is an async operation.
    let analysis_result = client.analyze_code(&file_content).await?;

    // Print the analysis received from the AI.
    println!("{}", "Analysis:".green().bold());
    println!("{}", analysis_result.trim());

    Ok(())
}
