//! src/config.rs
//!
//! Manages the application's configuration, primarily loading the
//! OpenAI API key from the environment.

use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;

/// Loads the OpenAI API key from the environment.
///
/// # Returns
///
/// A `Result` containing the API key as a `String` on success.
///
/// # Errors
///
/// Returns `Error::Config` if the `OPENAI_API_KEY` environment variable is not set.
pub fn get_api_key() -> Result<String> {
    // Load environment variables from a .env file if it exists.
    dotenv().ok();

    env::var("OPENAI_API_KEY").map_err(|_| {
        Error::Config(
            "The OPENAI_API_KEY environment variable must be set.".to_string(),
        )
    })
}
