//! src/pipeline/indexing.rs
//!
//! The final stage of the data processing pipeline. This module is responsible
//! for taking text chunks, generating vector embeddings for them using the
//! OpenAI API, and storing them in the database for later retrieval.

use crate::error::Result;
use crate::openai_client::OpenAIClient;
use crate::pipeline::chunking::TextChunk;
use sqlx::SqlitePool;
use tracing::{info, instrument};

/// Indexes a collection of text chunks by generating and storing their embeddings.
#[instrument(skip_all)]
pub async fn index_chunks(
    pool: &SqlitePool,
    client: &OpenAIClient,
    chunks: &[TextChunk],
) -> Result<()> {
    info!("Starting chunk indexing process for {} chunks...", chunks.len());
    let mut transaction = pool.begin().await?;

    for chunk in chunks {
        let embedding_vec = client.get_embedding(&chunk.chunk_text).await?;

        let embedding_bytes: Vec<u8> = embedding_vec
            .iter()
            .flat_map(|&f| f.to_ne_bytes())
            .collect();

        // Use a runtime-checked query to avoid compile-time database access.
        sqlx::query(
            "INSERT INTO chunks (document_id, chunk_text, embedding) VALUES (?, ?, ?)",
        )
        .bind(chunk.document_id)
        .bind(&chunk.chunk_text)
        .bind(&embedding_bytes)
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;

    info!("Successfully indexed {} chunks into the database.", chunks.len());
    Ok(())
}
