//! src/config.rs
//!
//! This module handles loading and accessing the tone profile configurations
//! from the external JSON file.

use crate::error::Result;
use serde::Deserialize;
use std::fs;

const TONE_PROFILES_PATH: &str = "config/tone_profiles.json";

/// Represents a single, named tone profile loaded from the configuration.
#[derive(Deserialize, Debug, Clone)]
pub struct ToneProfile {
    pub name: String,
    pub description: String,
    pub system_prompt: String,
}

/// Represents the top-level structure of the tone profiles configuration file.
#[derive(Deserialize, Debug, Clone)]
pub struct ToneProfileConfig {
    pub profiles: Vec<ToneProfile>,
}

/// Loads the tone profile configuration from the predefined file path.
///
/// # Returns
///
/// A `Result` containing the loaded `ToneProfileConfig`.
///
/// # Errors
///
/// Returns an `Error` if the file cannot be read or if the JSON is malformed.
pub fn load_tone_profiles() -> Result<ToneProfileConfig> {
    let file_content = fs::read_to_string(TONE_PROFILES_PATH)?;
    let config: ToneProfileConfig = serde_json::from_str(&file_content)?;
    Ok(config)
}
