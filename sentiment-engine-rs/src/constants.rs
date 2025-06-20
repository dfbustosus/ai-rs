//! src/constants.rs
//!
//! This module defines global, immutable constants for the application.
//! Centralizing these values makes them easy to manage and update.

/// The OpenAI model to be used for sentiment analysis and reasoning.
/// We use "gpt-4o" for its advanced instruction-following and reasoning capabilities.
pub const AI_MODEL_NAME: &str = "gpt-4o";

/// The path to the JSON file that defines the sentiment labels.
/// This path is relative to the root of the project.
pub const SENTIMENT_CONFIG_PATH: &str = "config/sentiment_labels.json";
