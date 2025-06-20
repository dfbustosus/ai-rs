//! src/error.rs
//!
//! This module defines the unified error type for the entire application.
//! It is designed to handle all potential failure modes, from I/O and
//! configuration issues to network errors and code-parsing failures.

use thiserror::Error;

/// The primary error enum for the application.
///
/// The `#[derive(Error, Debug)]` macro provides the necessary trait
/// implementations for this to function as a standard error type. Each
/// variant represents a distinct category of failure.
#[derive(Error, Debug)]
pub enum Error {
    /// For errors related to configuration, like a missing API key.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Wraps errors from standard I/O operations (e.g., file reading).
    /// The `{0}` includes the underlying error message in the output.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Wraps errors from the `walkdir` crate during file system traversal.
    #[error("Directory traversal error: {0}")]
    Walkdir(#[from] walkdir::Error),

    /// Wraps errors from the `syn` crate during Rust code parsing.
    #[error("Code parsing error: {0}")]
    Syn(#[from] syn::Error),

    /// Wraps errors from the `reqwest` HTTP client.
    #[error("HTTP request error")]
    Reqwest(#[from] reqwest::Error),

    /// For errors returned specifically by the OpenAI API.
    #[error("OpenAI API error: {0}")]
    OpenAI(String),

    /// For errors during JSON serialization or deserialization.
    #[error("JSON serialization/deserialization error")]
    SerdeJson(#[from] serde_json::Error),
}

/// A convenient type alias for `Result<T, E>` using our custom `Error` type.
pub type Result<T> = std::result::Result<T, Error>;
