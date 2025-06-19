//! src/config.rs
//!
//! This module is responsible for managing the application's configuration.
//! Its primary purpose is to load secrets and settings from the environment,
//! most notably the OpenAI API key from a `.env` file.

use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

/// Retrieves the OpenAI API key from the environment.
///
/// This function first loads the `.env` file from the current directory,
/// then attempts to read the `OPENAI_API_KEY` environment variable.
///
/// # Returns
///
/// A `Result` containing the API key as a `String` on success.
///
/// # Errors
///
/// Returns `Error::Config` if the `OPENAI_API_KEY` environment variable is not set.
/// This error is specifically crafted in our `error.rs` module.
pub fn api_key() -> Result<String> {
    // Load environment variables from the .env file in the project root.
    // This will do nothing if the file doesn't exist, which is fine.
    dotenv().ok();

    // Attempt to read the OPENAI_API_KEY from the environment.
    // `env::var` returns a `Result`, which we can elegantly handle with `map_err`.
    env::var(OPENAI_API_KEY).map_err(|_| {
        // If the variable is not found, we create a specific, user-friendly error.
        Error::Config(format!("{} is not set in the .env file", OPENAI_API_KEY))
    })
}
