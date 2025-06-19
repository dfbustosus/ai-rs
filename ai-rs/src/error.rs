//! src/error.rs
//!
//! This module defines the unified error type for the entire application.
//! By using a single, comprehensive error enum, we can handle all possible
//! failure scenarios in a consistent and predictable manner. The `thiserror`
//! crate is leveraged to reduce boilerplate when implementing the error type.

use thiserror::Error;

// The main error enum for our application.
// The `#[derive(Error, Debug)]` macro automatically implements the necessary
// traits for this to be a proper error type.
#[derive(Error, Debug)]
pub enum Error {
    /// Represents errors originating from the configuration process.
    /// Typically, this means a required environment variable is missing.
    /// The `#[error(...)]` attribute provides the display message for this error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// A wrapper for errors that occur during I/O operations.
    /// The `#[from]` attribute allows for automatic conversion from `std::io::Error`
    /// into `Error::Io`, simplifying error handling at call sites.
    /// The `{0}` includes the underlying error message in the output.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A wrapper for errors originating from the `reqwest` HTTP client.
    /// This includes network issues, DNS failures, or timeouts.
    /// The `#[from]` attribute enables seamless conversion from `reqwest::Error`.
    #[error("HTTP request error")]
    Reqwest(#[from] reqwest::Error),

    /// Represents errors specifically from the OpenAI API.
    /// This could be an invalid API key, a malformed request, or rate limiting.
    #[error("OpenAI API error: {0}")]
    OpenAI(String),

    /// A catch-all for errors related to JSON serialization or deserialization.
    /// The `#[from]` attribute handles conversion from `serde_json::Error`.
    #[error("JSON serialization/deserialization error")]
    SerdeJson(#[from] serde_json::Error),
}

// We define a custom Result type alias.
// By convention, this is often done in the error module.
// This allows us to use `Result<T>` throughout our application
// without having to repeatedly specify `E = crate::error::Error`.
pub type Result<T> = std::result::Result<T, Error>;
