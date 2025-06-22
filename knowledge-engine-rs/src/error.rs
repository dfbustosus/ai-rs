//! src/error.rs
//!
//! This module defines the unified error type for the knowledge engine.
//! It consolidates all possible failure scenarios, including database
//! errors, into a single enum for consistent error handling.

use thiserror::Error;

/// The primary error enum for the application.
#[derive(Error, Debug)]
pub enum Error {
    /// For errors related to loading or validating configuration.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Wraps errors from standard I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Wraps errors originating from the `sqlx` database toolkit.
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Wraps errors from the `sqlx` migration process.
    #[error("Database migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// Wraps errors from the `reqwest` HTTP client.
    #[error("HTTP request error")]
    Reqwest(#[from] reqwest::Error),

    /// For errors returned specifically by the OpenAI API.
    #[error("OpenAI API error: {0}")]
    OpenAI(String),

    /// For errors during JSON serialization or deserialization.
    #[error("JSON processing error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    /// For general processing errors, like parsing files.
    #[error("Processing error: {0}")]
    Processing(String),
}

/// A convenient type alias for `Result<T, E>` using our custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
