//! src/main.rs
//!
//! The main entry point for the Narrative Visualization Engine.
//! This module orchestrates the entire multi-stage pipeline, from parsing
//! command-line arguments to generating the final storyboard file.

// Declare the module hierarchy for the compiler.
mod config;
mod error;
mod openai_client;
mod output_assembler;
mod pipeline;

use crate::error::Result;
use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

/// Defines the command-line arguments for the application.
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version,
    about = "An AI-powered engine to generate visual storyboards from narrative text."
)]
struct Args {
    /// The path to the input narrative text file.
    #[arg(short, long)]
    input_file: PathBuf,

    /// The path for the output HTML storyboard file.
    #[arg(short, long, default_value = "output/storyboard.html")]
    output_file: PathBuf,
}

/// The main asynchronous function that orchestrates the application.
#[tokio::main]
async fn main() {
    init_logger();

    // Execute the core application logic and handle any resulting errors.
    if let Err(e) = run().await {
        error!("\n{} {}", "Error:".red().bold(), e.to_string().red());
        std::process::exit(1);
    }
}

/// The primary logic function for the application.
async fn run() -> Result<()> {
    // Parse the command-line arguments.
    let args = Args::parse();
    info!(
        "Starting narrative visualization for '{}'.",
        args.input_file.display()
    );

    // --- Initialization ---
    let api_key = config::get_api_key()?;
    let client = openai_client::OpenAIClient::new(api_key);

    // Load the source narrative text from the input file.
    let narrative_text = fs::read_to_string(&args.input_file)?;
    if narrative_text.trim().is_empty() {
        return Err(error::Error::Pipeline(
            "Input file is empty.".to_string(),
        ));
    }

    // --- Execute Pipeline ---
    // Stage 1: Decompose the narrative into distinct scenes.
    let scenes = pipeline::stage_1_scene_detection::detect_scenes(&client, &narrative_text).await?;

    // Stage 2: Generate rich visual prompts for each scene.
    let visual_prompts =
        pipeline::stage_2_prompt_generation::generate_visual_prompts(&client, &scenes).await?;

    // Stage 3: Synthesize an image for each visual prompt.
    let storyboard_frames =
        pipeline::stage_3_image_generation::generate_images(&client, &visual_prompts).await?;

    // --- Assemble Output ---
    // Combine the text and generated images into a final HTML storyboard.
    output_assembler::assemble_storyboard_html(&storyboard_frames, &args.output_file)?;

    info!(
        "{}",
        format!(
            "Successfully generated storyboard at '{}'",
            args.output_file.display()
        )
        .green()
        .bold()
    );

    Ok(())
}

/// Initializes the global logger for the application.
fn init_logger() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}
