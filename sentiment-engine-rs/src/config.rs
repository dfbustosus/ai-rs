//! src/config.rs
//!
//! This module is responsible for loading and managing the application's
//! configuration, specifically the sentiment labels defined in the external
//! JSON file.

use crate::{constants, error::Result};
use serde::Deserialize;
use std::fs;

/// Represents a single sentiment category loaded from the configuration file.
#[derive(Deserialize, Debug, Clone)]
pub struct SentimentLabel {
    pub name: String,
    pub description: String,
}

/// Represents the top-level structure of the sentiment configuration file.
#[derive(Deserialize, Debug, Clone)]
pub struct SentimentConfig {
    pub labels: Vec<SentimentLabel>,
}

/// Loads the sentiment configuration from the file specified in `constants`.
///
/// This function reads the JSON file, parses it into our `SentimentConfig`
/// struct, and returns the configuration. This approach makes the sentiment
/// rules easily extensible without requiring code changes.
///
/// # Returns
///
/// A `Result` containing the loaded `SentimentConfig`.
///
/// # Errors
///
/// Returns `Error::Io` if the file cannot be read, or `Error::SerdeJson`
/// if the file content is not valid JSON.
pub fn load() -> Result<SentimentConfig> {
    let config_path = constants::SENTIMENT_CONFIG_PATH;
    let file_content = fs::read_to_string(config_path)?;
    let config: SentimentConfig = serde_json::from_str(&file_content)?;
    Ok(config)
}
