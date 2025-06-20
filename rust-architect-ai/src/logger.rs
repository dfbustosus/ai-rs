//! src/logger.rs
//!
//! Initializes the application's logging infrastructure using the `tracing`
//! and `tracing_subscriber` crates for structured, level-based logging.

use tracing_subscriber::{fmt, EnvFilter};

/// Initializes the global logger for the application.
///
/// This setup filters logs based on the `RUST_LOG` environment variable,
/// defaulting to the "info" level if it's not set. This provides a clean
/// and configurable way to manage application output.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(false) // Keep the output clean for this tool.
        .without_time()     // We don't need timestamps for this CLI.
        .init();
}
