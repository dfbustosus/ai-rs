//! src/logger.rs
//!
//! This module is responsible for initializing the application's logging infrastructure.
//! We use the `tracing` and `tracing_subscriber` crates to provide structured,
//! level-based logging.

use tracing_subscriber::{fmt, EnvFilter};

/// Initializes the logging system for the application.
///
/// This function sets up a `tracing` subscriber that formats log messages
/// and filters them based on the `RUST_LOG` environment variable. If `RUST_LOG`
/// is not set, it defaults to showing `info`-level logs and above for all modules.
pub fn init() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt::Subscriber::builder()
        .with_env_filter(filter)
        .with_target(true)  // Include the module path in log messages.
        .with_level(true)   // Include the log level in log messages.
        .init();
}
