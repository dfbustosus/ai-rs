//! src/main.rs
//!
//! The main entry point for the Explainable Credit Risk Assessment Engine.
//! This module orchestrates the entire workflow, from parsing command-line
//! arguments to printing the final, structured risk assessment.

// Declare the module hierarchy for the compiler.
mod config;
mod error;
mod logger;
mod models;
mod openai_client;
mod risk_analyzer;
mod validator;

use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use risk_analyzer::RiskAnalyzer;
use std::path::PathBuf;
use tracing::{error, info};

/// Defines the command-line arguments accepted by the application.
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version,
    about = "An AI-powered engine for explainable credit risk assessment."
)]
struct Args {
    /// The path to the applicant's profile JSON file.
    #[arg(required = true)]
    input_file: PathBuf,
}

/// The main asynchronous function that orchestrates the application.
#[tokio::main]
async fn main() {
    // Initialize the logger from our dedicated logger module.
    logger::init();

    // Execute the core application logic and handle any resulting errors.
    if let Err(e) = run().await {
        error!("\n{} {}", "Error:".red().bold(), e.to_string().red());
        std::process::exit(1);
    }
}

/// The primary logic function for the application.
async fn run() -> Result<()> {
    // Parse the command-line arguments provided by the user.
    let args = Args::parse();
    info!("Received request to analyze profile: {}", args.input_file.display());

    // --- Initialization & Validation ---
    let profile = validator::load_and_validate_profile(&args.input_file)?;
    info!(applicant_id = %profile.applicant_id, "Applicant profile successfully validated.");

    let api_key = config::get_api_key()?;
    let openai_client = openai_client::OpenAIClient::new(api_key);
    let analyzer = RiskAnalyzer::new(openai_client);

    // --- Assessment ---
    let assessment = analyzer.assess(&profile).await?;

    // --- Display Results ---
    println!(
        "\n{}",
        "--- Credit Risk Assessment Complete ---".bold().cyan()
    );
    let results_json = serde_json::to_string_pretty(&assessment)?;
    println!("{}", results_json);

    Ok(())
}
