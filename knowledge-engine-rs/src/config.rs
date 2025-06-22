//! src/config.rs
//!
//! Manages the application's configuration, such as the database URL
//! and the OpenAI API key.

use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::env;

const DATABASE_URL_KEY: &str = "DATABASE_URL";
const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

/// A struct to hold all application configuration.
pub struct Config {
    pub database_url: String,
    pub openai_api_key: String,
}

/// Loads the application configuration from environment variables.
pub fn load() -> Result<Config> {
    // Load .env file if it exists.
    dotenv().ok();

    let database_url = env::var(DATABASE_URL_KEY)
        .map_err(|_| Error::Config(format!("{} must be set", DATABASE_URL_KEY)))?;

    let openai_api_key = env::var(OPENAI_API_KEY)
        .map_err(|_| Error::Config(format!("{} must be set", OPENAI_API_KEY)))?;

    Ok(Config {
        database_url,
        openai_api_key,
    })
}
