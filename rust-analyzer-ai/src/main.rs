//! src/main.rs
//!
//! This is the main entry point for the AI-Powered Rust Code Analyzer.
//! It ties together all other modules to create a cohesive command-line tool.
//! Its primary responsibilities are:
//! 1. Parsing and validating command-line arguments (the file/directory path).
//! 2. Initializing configuration and the OpenAI client.
//! 3. Discovering target Rust files.
//! 4. Iterating through the files and triggering the analysis for each one.
//! 5. Handling and reporting any errors that occur during the process.

// Declare the module hierarchy for the compiler.
mod analyzer;
mod config;
mod error;
mod files;
mod openai;

use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;

/// Defines the command-line arguments for our application.
/// `clap` will automatically generate a help message, parse arguments,
/// and provide validation based on this struct.
#[derive(Parser, Debug)]
#[command(
    author = "David BU",
    version,
    about = "An AI-powered assistant to analyze and suggest improvements for Rust code."
)]
struct Args {
    /// The path to the Rust source file or project directory to analyze.
    #[arg(required = true)]
    path: PathBuf,
}

/// The main asynchronous function that runs our application.
#[tokio::main]
async fn main() -> Result<()> {
    // Parse the command-line arguments. `clap` will handle errors and exit
    // if the arguments are invalid.
    let args = Args::parse();

    // Use a single, top-level try block to catch and handle any errors
    // that bubble up from our application's logic.
    if let Err(e) = run_analyzer(args).await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        // Ensure the process exits with a non-zero status code on error.
        std::process::exit(1);
    }

    Ok(())
}

/// The core logic runner for the analyzer.
///
/// This function is separated from `main` to allow for clean error handling
/// using the `?` operator.
async fn run_analyzer(args: Args) -> Result<()> {
    // --- Initialization ---
    println!("{}", "Initializing analyzer...".cyan());
    let api_key = config::api_key()?;
    let client = openai::Client::new(api_key);

    // --- File Discovery ---
    let mut files_to_analyze = Vec::new();

    if args.path.is_dir() {
        // If the path is a directory, find all `.rs` files within it.
        files_to_analyze = files::find_rust_files(&args.path)?;
    } else if args.path.is_file() {
        // If it's a single file, just add it to the list.
        files_to_analyze.push(args.path);
    } else {
        // If the path doesn't exist, print an error.
        eprintln!(
            "{} Path '{}' is not a valid file or directory.",
            "Error:".red().bold(),
            args.path.display()
        );
        std::process::exit(1);
    }

    if files_to_analyze.is_empty() {
        println!("{}", "No Rust files to analyze. Exiting.".yellow());
        return Ok(());
    }

    // --- Analysis Loop ---
    for file_path in files_to_analyze {
        // Analyze each file. If an error occurs for a single file,
        // we print it and continue to the next one.
        if let Err(e) = analyzer::analyze_file(&client, &file_path).await {
            eprintln!(
                "{} Could not analyze file '{}': {}",
                "Warning:".yellow().bold(),
                file_path.display(),
                e
            );
        }
    }

    println!(
        "\n{}",
        "Analysis complete. All files have been processed.".green().bold()
    );
    Ok(())
}
