//! src/main.rs
//!
//! The main entry point for the AI-Powered Architecture Illustrator.
//! This module orchestrates the entire workflow, from parsing command-line
//! arguments to generating and saving the final architectural diagram.

// Declare the module hierarchy for the compiler.
mod config;
mod diagram_generator;
mod error;
mod logger;
mod openai_client;
mod project_scanner;

use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use diagram_generator::DiagramType;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};

/// Defines the command-line arguments for the application.
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version,
    about = "An AI-powered tool to automatically generate architectural diagrams from Rust code."
)]
struct Args {
    /// The path to the Rust project directory to analyze.
    #[arg(short, long, default_value = ".")]
    project_path: PathBuf,

    /// The path to the output file for the generated diagram.
    #[arg(short, long, default_value = "output/architecture.md")]
    output: PathBuf,

    /// The type of diagram to generate.
    #[arg(long, value_enum, default_value_t = DiagramType::Component)]
    diagram_type: DiagramType,

    /// (For Sequence Diagrams) The name of the public function to use as the entry point.
    #[arg(long)]
    function_name: Option<String>,
}

/// The main asynchronous function that runs the application.
#[tokio::main]
async fn main() {
    // Initialize the logging system as the very first step.
    logger::init();

    // Execute the core application logic and handle any fatal errors.
    if let Err(e) = run().await {
        error!(error = ?e, "A critical error occurred and the application must exit.");
        std::process::exit(1);
    }
}

/// The primary logic runner for the application, separated from `main`
/// for clean error handling with the `?` operator.
async fn run() -> Result<()> {
    let args = Args::parse();
    info!("Starting architectural analysis for project at '{}'.", args.project_path.display());

    // --- Validation for Sequence Diagram ---
    if args.diagram_type == DiagramType::Sequence && args.function_name.is_none() {
        error!("The '--function-name' argument is required when generating a sequence diagram.");
        std::process::exit(1);
    }

    // --- Initialization ---
    let api_key = config::get_api_key()?;
    let client = openai_client::OpenAIClient::new(api_key);
    let generator = diagram_generator::DiagramGenerator::new(client);

    // --- Project Scanning ---
    let project_context = project_scanner::scan_project(&args.project_path)?;

    if project_context.is_empty() {
        info!("{}", "No Rust files were found in the specified directory. Exiting.".yellow());
        return Ok(());
    }

    // --- Diagram Generation ---
    let diagram = generator
        .generate_diagram(&project_context, args.diagram_type, args.function_name)
        .await?;

    // --- Output ---
    // Ensure the output directory exists before writing the file.
    if let Some(parent_dir) = args.output.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    fs::write(&args.output, &diagram)?;

    info!(
        "{}",
        format!(
            "Successfully generated diagram and saved it to '{}'.",
            args.output.display()
        )
        .green()
        .bold()
    );

    Ok(())
}
