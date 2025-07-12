//! src/error.rs
//!
//! This module defines the unified error type for the narrative visualizer.
//! It consolidates all possible failure scenarios into a single, comprehensive
//! enum for consistent error handling throughout the application.

use thiserror::Error;

/// The primary error enum for the application.
#[derive(Error, Debug)]
pub enum Error {
    /// For errors related to loading or validating configuration.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Wraps errors from standard I/O operations (e.g., file reading/writing).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Wraps errors from the `reqwest` HTTP client.
    #[error("HTTP request error")]
    Reqwest(#[from] reqwest::Error),

    /// For errors returned specifically by the OpenAI API.
    #[error("OpenAI API error: {0}")]
    OpenAI(String),

    /// For errors during JSON serialization or deserialization.
    #[error("JSON processing error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// For errors related to decoding base64 image data.
    #[error("Base64 decoding error: {0}")]
    Base64Decode(#[from] base64::DecodeError),

    /// For when a required step in the pipeline produces no output.
    #[error("Pipeline error: {0}")]
    Pipeline(String),
}

/// A convenient type alias for `Result<T, E>` using our custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
