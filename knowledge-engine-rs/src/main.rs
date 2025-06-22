//! src/main.rs
//!
//! The main entry point for the AI-Powered Knowledge Synthesis and Query Engine.
//! This module orchestrates the two primary workflows: ingesting documents into
//! the knowledge base and querying that knowledge base to answer questions.

// Declare the module hierarchy for the compiler.
mod config;
mod database;
mod error;
mod openai_client;
mod pipeline;
mod query_engine;

use crate::error::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

/// Defines the command-line interface for the application using clap.
#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version,
    about = "An AI-powered knowledge engine to ingest and query documents."
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Defines the available subcommands: `ingest` and `query`.
#[derive(Subcommand, Debug)]
enum Command {
    /// Ingests documents from a specified path into the knowledge base.
    Ingest {
        /// The path to the directory containing documents to ingest.
        #[arg(default_value = "./documents")]
        path: PathBuf,
    },
    /// Asks a question against the knowledge base.
    Query {
        /// The question to ask.
        #[arg(required = true)]
        question: String,
    },
}

/// The main asynchronous function that orchestrates the application.
#[tokio::main]
async fn main() {
    init_logger();

    if let Err(e) = run().await {
        error!(error = ?e, "A critical error occurred. Exiting.");
        std::process::exit(1);
    }
}

/// The primary logic runner for the application.
async fn run() -> Result<()> {
    let args = Args::parse();
    let config = config::load()?;

    let db_pool = database::init_db(&config.database_url).await?;
    let client = openai_client::OpenAIClient::new(config.openai_api_key);

    match args.command {
        Command::Ingest { path } => {
            info!("Starting 'ingest' command for path: '{}'", path.display());
            let source_docs = pipeline::ingestion::ingest_documents(&db_pool, &path).await?;
            if source_docs.is_empty() {
                info!("{}", "No new or updated documents to process.".green());
                return Ok(());
            }
            let chunks = pipeline::chunking::chunk_documents(&source_docs);
            pipeline::indexing::index_chunks(&db_pool, &client, &chunks).await?;
            info!("{}", "Ingestion process completed successfully.".green().bold());
        }
        Command::Query { question } => {
            info!("Starting 'query' command with question: '{}'", question);
            let query_engine = query_engine::QueryEngine::new(db_pool, client);
            let answer = query_engine.answer_question(&question).await?;

            println!("\n{}", "Answer:".bold().cyan());
            println!("{}", answer);
        }
    }

    Ok(())
}

/// Initializes the logging system.
fn init_logger() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}
