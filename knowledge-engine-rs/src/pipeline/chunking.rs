//! src/pipeline/chunking.rs
//!
//! The second stage of the data pipeline. This module takes the content of a
//! source document and splits it into smaller, manageable chunks suitable for
//! embedding.

use crate::pipeline::ingestion::SourceDocument;
use text_splitter::TextSplitter;
use tracing::info;

const MAX_CHUNK_SIZE: usize = 1000; // The target size for each text chunk in characters.

/// Represents a single piece of text derived from a source document.
pub struct TextChunk {
    pub document_id: i64,
    pub chunk_text: String,
}

/// Splits a collection of source documents into text chunks.
///
/// This function iterates through each source document and uses the `text-splitter`
/// crate to break its content down into chunks of a predefined maximum size.
///
/// # Arguments
///
/// * `documents` - A slice of `SourceDocument`s to be processed.
///
/// # Returns
///
/// A `Vec` of `TextChunk` structs ready for the next pipeline stage (indexing).
pub fn chunk_documents(documents: &[SourceDocument]) -> Vec<TextChunk> {
    info!("Starting document chunking process...");
    let splitter = TextSplitter::default().with_trim_chunks(true);
    let mut all_chunks = Vec::new();

    for doc in documents {
        let chunks: Vec<_> = splitter
            .chunks(&doc.content, MAX_CHUNK_SIZE)
            .map(|text| TextChunk {
                document_id: doc.id,
                chunk_text: text.to_string(),
            })
            .collect();
        info!(
            "Split document '{}' (ID: {}) into {} chunks.",
            doc.path.display(),
            doc.id,
            chunks.len()
        );
        all_chunks.extend(chunks);
    }

    info!("Document chunking complete. Generated {} total chunks.", all_chunks.len());
    all_chunks
}
