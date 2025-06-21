//! src/error.rs
//!
//! This module defines the unified error type for the conversation distiller.
//! It consolidates all possible failure scenarios into a single, comprehensive
//! enum for consistent error handling throughout the application.

use thiserror::Error;

/// The primary error enum for the application.
#[derive(Error, Debug)]
pub enum Error {
    /// For errors related to loading or parsing configuration files.
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
}

/// A convenient type alias for `Result<T, E>` using our custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
