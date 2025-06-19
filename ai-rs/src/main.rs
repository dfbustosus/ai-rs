//! src/main.rs
//!
//! This is the main entry point for the `ai-rs` application.
//! Its responsibilities are:
//! 1. Declaring the module hierarchy to the Rust compiler.
//! 2. Initializing the configuration by loading the API key.
//! 3. Creating the OpenAI client instance.
//! 4. Running the command-line interface.
//! 5. Handling any top-level errors that might occur during startup or runtime.

// Declare the modules that make up our application.
// This tells Rust to look for `error.rs`, `config.rs`, etc., and include them.
mod cli;
mod config;
mod error;
mod openai;

// The `Error` type is not used directly, so it can be removed from the import.
use crate::error::Result;
use colored::Colorize;
use openai::Client;

// The `tokio::main` attribute transforms our `async main` function into a
// synchronous `main` function that sets up and runs the Tokio async runtime.
#[tokio::main]
async fn main() -> Result<()> {
    // Attempt to load the API key from the environment.
    // The `?` operator will propagate any error from `config::api_key()`,
    // causing the program to exit if the key isn't found.
    let api_key = config::api_key()?;

    // Create a new OpenAI client with the loaded key.
    let client = Client::new(api_key);

    // Start the command-line interface. If an error occurs, print it
    // and then propagate the error out of `main`.
    if let Err(e) = cli::run(client).await {
        // Use the `colored` crate to make the error message stand out.
        eprintln!("{} {}", "Error:".red().bold(), e);
        // Return the error to ensure the process exits with a non-zero status code.
        return Err(e);
    }

    // Return Ok(()) to indicate successful execution.
    Ok(())
}
