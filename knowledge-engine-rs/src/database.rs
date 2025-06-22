//! src/database.rs
//!
//! This module is responsible for all database interactions. It manages the
//! connection pool and runs schema migrations to ensure the database is
//! up-to-date.

use crate::error::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;
use tracing::info;

/// Establishes a connection to the SQLite database and runs migrations.
///
/// This function creates the database file if it doesn't exist, sets up a
/// connection pool for efficient access, and applies any pending database
/// schema migrations from the `migrations` directory.
///
/// # Arguments
///
/// * `database_url` - The connection string for the SQLite database.
///
/// # Returns
///
/// A `Result` containing the `SqlitePool` on success.
pub async fn init_db(database_url: &str) -> Result<SqlitePool> {
    info!("Initializing database connection...");

    // Create the database file if it does not exist.
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true);

    // Create a connection pool.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    info!("Database connection established. Running migrations...");

    // Run migrations to apply the schema.
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database migrations completed successfully.");
    Ok(pool)
}
