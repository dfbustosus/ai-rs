//! src/config.rs
//!
//! This module handles the application's configuration. It is responsible
//! for loading secrets and settings from the environment, primarily the
//! OpenAI API key from a `.env` file.

use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;

const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

/// Retrieves the OpenAI API key from the environment.
///
/// This function loads the `.env` file from the project directory and then
/// attempts to read the `OPENAI_API_KEY` environment variable.
///
/// # Returns
///
/// A `Result` containing the API key as a `String` if successful.
///
/// # Errors
///
/// Returns `Error::Config` if the `OPENAI_API_KEY` is not set.
pub fn api_key() -> Result<String> {
    // Attempt to load the .env file. Fails silently if not present.
    dotenv().ok();

    // Read the variable, mapping the `VarError` to our custom `Error::Config`.
    env::var(OPENAI_API_KEY).map_err(|_| {
        Error::Config(format!("{} is not set in the .env file", OPENAI_API_KEY))
    })
}
