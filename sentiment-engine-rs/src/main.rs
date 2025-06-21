//! src/main.rs
//!
//! The main entry point for the Explainable Sentiment Analysis Engine.
//! This module ties all other components together to form a cohesive
//! command-line application.

// Declare the module hierarchy for the compiler.
mod config;
mod constants;
mod error;
mod logger;
mod openai_client;
mod sentiment_analyzer;

use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use sentiment_analyzer::{AnalysisResult, SentimentAnalyzer};
use tracing::{error, info};

/// Defines the command-line arguments accepted by the application.
/// `clap` uses this struct to generate help messages and parse input.
#[derive(Parser, Debug)]
#[command(
    author = "David BU",
    version,
    about = "An explainable sentiment analysis engine powered by AI."
)]
struct Args {
    /// The text to analyze for sentiment.
    #[arg(required = true)]
    text: String,
}

/// The main asynchronous function that orchestrates the application.
#[tokio::main]
async fn main() {
    // Initialize the logging system immediately.
    logger::init();

    // Execute the core application logic and handle any resulting errors.
    if let Err(e) = run().await {
        error!(error = ?e, "A critical error occurred. Exiting.");
        std::process::exit(1);
    }
}

/// The primary logic function for the application.
///
/// This function is separated from `main` to allow for clean error
/// handling using the `?` operator. It initializes all components and
/// runs the analysis.
async fn run() -> Result<()> {
    // Parse the command-line arguments provided by the user.
    let args = Args::parse();
    info!(text = %args.text, "Received text for analysis.");

    // --- Initialization ---
    // Load the sentiment category configuration from the JSON file.
    let sentiment_config = config::load()?;
    info!("Successfully loaded {} sentiment labels.", sentiment_config.labels.len());

    // Load the OpenAI API key from the environment.
    let api_key = load_api_key()?;
    let openai_client = openai_client::OpenAIClient::new(api_key);

    // Create the analyzer instance.
    let analyzer = SentimentAnalyzer::new(openai_client, sentiment_config);

    // --- Analysis ---
    // Perform the sentiment analysis on the user-provided text.
    let analysis_result = analyzer.analyze(&args.text).await?;

    // --- Display Results ---
    // Print the results to the console in a clear, formatted way.
    print_results(&analysis_result);

    Ok(())
}

/// Loads the OpenAI API key from the environment variables.
fn load_api_key() -> Result<String> {
    dotenvy::dotenv().ok();
    std::env::var("OPENAI_API_KEY")
        .map_err(|_| crate::error::Error::Config("OPENAI_API_KEY not found in environment.".to_string()))
}

/// Prints the final analysis results to the console.
fn print_results(result: &AnalysisResult) {
    println!("\n{}", "Sentiment Analysis Complete".bold().underline());
    println!("\n{}", "Reasoning (Chain of Thought):".cyan().bold());
    println!("{}", result.chain_of_thought);
    println!("\n{}", "Final Classification:".green().bold());
    println!("{}", result.sentiment);
}
