//! src/main.rs
//!
//! The main entry point for the Intelligent Conversation Distillation Engine.
//! This module ties all other components together to form a cohesive
//! command-line application.

// Declare the module hierarchy for the compiler.
mod config;
mod conversation_parser;
mod distiller_engine;
mod error;
mod logger;
mod openai_client;

use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use tracing::{error, info};

/// Defines the command-line arguments accepted by the application.
#[derive(Parser, Debug)]
#[command(
    author = "David BU",
    version,
    about = "An intelligent engine to distill conversations into purpose-driven summaries."
)]
struct Args {
    /// The path to the input conversation JSON file.
    #[arg(short, long)]
    input_file: PathBuf,

    /// The name of the tone profile to use for the summary (e.g., 'executive_briefing').
    #[arg(short, long)]
    profile_name: String,
}

/// The main asynchronous function that orchestrates the application.
#[tokio::main]
async fn main() {
    // Initialize the logging system immediately.
    logger::init();

    // Execute the core application logic and handle any resulting errors.
    if let Err(e) = run().await {
        error!("\n{}\n", e.to_string().red());
        std::process::exit(1);
    }
}

/// The primary logic function for the application.
async fn run() -> Result<()> {
    // Parse the command-line arguments provided by the user.
    let args = Args::parse();
    info!(
        input_file = %args.input_file.display(),
        profile = %args.profile_name,
        "Starting distillation process."
    );

    // --- Initialization ---
    // Load the available tone profiles from the configuration file.
    let tone_profiles = config::load_tone_profiles()?;
    info!("Successfully loaded {} tone profiles.", tone_profiles.profiles.len());

    // Find the specific profile requested by the user.
    let selected_profile = tone_profiles
        .profiles
        .iter()
        .find(|p| p.name == args.profile_name)
        .cloned() // Clone the found profile to get an owned version.
        .ok_or_else(|| {
            // If the profile is not found, construct a helpful error message
            // listing all available profiles and their descriptions.
            let available_profiles_info = tone_profiles
                .profiles
                .iter()
                .map(|p| format!("  - {}: {}", p.name.cyan(), p.description))
                .collect::<Vec<_>>()
                .join("\n");
            
            let error_message = format!(
                "Profile '{}' not found.\n\nAvailable profiles:\n{}",
                args.profile_name, available_profiles_info
            );

            error::Error::Config(error_message)
        })?;

    info!("Using selected profile: '{}'", selected_profile.name);

    // Load the conversation transcript from the specified input file.
    let conversation = conversation_parser::load_conversation(&args.input_file)?;
    info!("Successfully loaded conversation with {} turns.", conversation.conversation.len());

    // Load the OpenAI API key and create the client.
    let api_key = load_api_key()?;
    let openai_client = openai_client::OpenAIClient::new(api_key);

    // Create the distiller engine instance.
    let engine = distiller_engine::DistillerEngine::new(openai_client);

    // --- Distillation ---
    // Perform the distillation using the selected conversation and profile.
    let summary = engine.distill(&conversation, &selected_profile).await?;

    // --- Display Results ---
    print_summary(&selected_profile.name, &summary);

    Ok(())
}

/// Loads the OpenAI API key from the environment variables.
fn load_api_key() -> Result<String> {
    dotenvy::dotenv().ok();
    std::env::var("OPENAI_API_KEY")
        .map_err(|_| error::Error::Config("OPENAI_API_KEY not found in environment.".to_string()))
}

/// Prints the final summary to the console in a formatted block.
fn print_summary(profile_name: &str, summary: &str) {
    println!(
        "\n{}",
        format!("--- Distilled Summary: {} ---", profile_name)
            .bold()
            .cyan()
    );
    println!("{}", summary);
    println!(
        "{}",
        "--- End of Summary ---".bold().cyan()
    );
}
