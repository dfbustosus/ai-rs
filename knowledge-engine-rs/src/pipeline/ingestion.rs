//! src/pipeline/ingestion.rs
//!
//! The first stage of the knowledge engine's data pipeline. This module
//! is responsible for discovering documents, tracking their state via content
//! hashing, and storing their metadata in the database.

use crate::error::{Error, Result};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;

/// Represents a source document to be processed by the pipeline.
pub struct SourceDocument {
    pub id: i64,
    pub path: PathBuf,
    pub content: String,
}

#[derive(sqlx::FromRow)]
struct DocIdAndHash {
    id: i64,
    content_hash: String,
}

/// Scans a directory for documents and ingests new or updated ones.
pub async fn ingest_documents(
    pool: &SqlitePool,
    documents_path: &Path,
) -> Result<Vec<SourceDocument>> {
    info!("Starting document ingestion from '{}'...", documents_path.display());
    let mut documents_to_process = Vec::new();

    for entry in WalkDir::new(documents_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        
        let content = match path.extension().and_then(|s| s.to_str()) {
            Some("pdf") => extract_pdf_text(path)?,
            Some("txt") | Some("md") => fs::read_to_string(path)?,
            _ => {
                warn!("Unsupported file type, skipping: {}", path.display());
                continue;
            }
        };

        let hash = calculate_hash(&content);
        let path_str = path.to_string_lossy().to_string();

        let existing_doc: Option<DocIdAndHash> = sqlx::query_as(
            "SELECT id, content_hash FROM documents WHERE file_path = ?",
        )
        .bind(&path_str)
        .fetch_optional(pool)
        .await?;

        match existing_doc {
            Some(doc) if doc.content_hash == hash => continue,
            Some(doc) => {
                warn!("Document '{}' has changed and will be re-ingested.", path.display());
                sqlx::query("UPDATE documents SET content_hash = ? WHERE id = ?")
                    .bind(&hash)
                    .bind(doc.id)
                    .execute(pool)
                    .await?;
                sqlx::query("DELETE FROM chunks WHERE document_id = ?")
                    .bind(doc.id)
                    .execute(pool)
                    .await?;
                documents_to_process.push(SourceDocument {
                    id: doc.id,
                    path: path.to_path_buf(),
                    content,
                });
            }
            None => {
                info!("Ingesting new document: '{}'", path.display());
                let result = sqlx::query(
                    "INSERT INTO documents (file_path, content_hash) VALUES (?, ?)",
                )
                .bind(&path_str)
                .bind(&hash)
                .execute(pool)
                .await?;
                documents_to_process.push(SourceDocument {
                    id: result.last_insert_rowid(),
                    path: path.to_path_buf(),
                    content,
                });
            }
        }
    }

    info!("Document ingestion complete. Found {} new or updated documents to process.", documents_to_process.len());
    Ok(documents_to_process)
}

fn extract_pdf_text(path: &Path) -> Result<String> {
    pdf_extract::extract_text(path)
        .map_err(|e| Error::Processing(format!("Failed to extract text from PDF '{}': {}", path.display(), e)))
}

fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}
