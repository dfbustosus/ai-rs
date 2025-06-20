//! src/error.rs
//!
//! Defines the unified error type for the sentiment analysis engine. This
//! approach ensures consistent error handling across all modules.

use thiserror::Error;

/// The primary error enum for the application. Each variant represents a
/// distinct category of potential failure.
#[derive(Error, Debug)]
pub enum Error {
    /// For errors related to configuration loading or parsing.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Wraps errors from standard I/O operations (e.g., file reading).
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

    /// For when the AI's response is not in the expected format.
    #[error("Invalid response format from AI: {0}")]
    InvalidResponseFormat(String),
}

/// A convenient type alias for `Result<T, E>` using our custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
