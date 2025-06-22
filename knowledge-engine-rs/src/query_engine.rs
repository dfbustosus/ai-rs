//! src/query_engine.rs
//!
//! This module contains the logic for querying the knowledge base. It handles
//! embedding the user's query, finding relevant chunks from the database
//! using vector similarity, and synthesizing a final answer using a
//! generative model.

use crate::error::{Error, Result};
use crate::openai_client::OpenAIClient;
use sqlx::{FromRow, SqlitePool};
use tracing::{info, instrument};

const SIMILARITY_TOP_K: usize = 5; // The number of most relevant chunks to retrieve.

/// A struct to hold a chunk retrieved from the database, including its text
/// and pre-calculated similarity score to the user's query.
#[derive(Debug)]
struct RelevantChunk {
    text: String,
    similarity: f32,
}

/// Represents a record from the `chunks` table.
#[derive(FromRow)]
struct ChunkRecord {
    chunk_text: String,
    embedding: Vec<u8>,
}

/// The main engine for processing user queries against the knowledge base.
pub struct QueryEngine {
    pool: SqlitePool,
    client: OpenAIClient,
}

impl QueryEngine {
    /// Creates a new instance of the `QueryEngine`.
    pub fn new(pool: SqlitePool, client: OpenAIClient) -> Self {
        Self { pool, client }
    }

    /// Answers a user's question by querying the knowledge base.
    #[instrument(skip(self, question))]
    pub async fn answer_question(&self, question: &str) -> Result<String> {
        info!("Answering question: '{}'", question);

        let question_embedding = self.client.get_embedding(question).await?;
        let relevant_chunks = self.find_relevant_chunks(&question_embedding).await?;

        if relevant_chunks.is_empty() {
            return Ok("I could not find any relevant information in the knowledge base to answer your question.".to_string());
        }

        let system_prompt = "You are a helpful AI assistant. Answer the user's question based *only* on the context provided. If the context does not contain the answer, state that you cannot answer from the given information.";
        let user_prompt = self.build_synthesis_prompt(question, &relevant_chunks);

        self.client.get_completion(system_prompt, &user_prompt).await
    }

    /// Finds the most relevant text chunks from the database using vector similarity.
    async fn find_relevant_chunks(&self, question_embedding: &[f32]) -> Result<Vec<RelevantChunk>> {
        info!("Searching for relevant chunks in the database...");
        let all_chunks: Vec<ChunkRecord> =
            sqlx::query_as("SELECT chunk_text, embedding FROM chunks")
                .fetch_all(&self.pool)
                .await?;

        let mut scored_chunks = Vec::new();

        for chunk_record in all_chunks {
            let chunk_embedding = deserialize_embedding(&chunk_record.embedding)?;
            let similarity = cosine_similarity(question_embedding, &chunk_embedding);
            scored_chunks.push(RelevantChunk {
                text: chunk_record.chunk_text,
                similarity,
            });
        }

        scored_chunks.sort_unstable_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());
        scored_chunks.truncate(SIMILARITY_TOP_K);

        info!("Found {} relevant chunks.", scored_chunks.len());
        Ok(scored_chunks)
    }

    /// Builds the final prompt for the generative model to synthesize an answer.
    fn build_synthesis_prompt(&self, question: &str, chunks: &[RelevantChunk]) -> String {
        let context = chunks
            .iter()
            .map(|c| c.text.as_str())
            .collect::<Vec<_>>()
            .join("\n---\n");

        format!(
            "CONTEXT:
            ---
            {}
            ---
            QUESTION: {}
            
            ANSWER:",
            context, question
        )
    }
}

//========= Vector Math Helpers =========//

fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product = v1.iter().zip(v2).map(|(x, y)| x * y).sum::<f32>();
    let norm_v1 = (v1.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
    let norm_v2 = (v2.iter().map(|x| x.powi(2)).sum::<f32>()).sqrt();
    if norm_v1 == 0.0 || norm_v2 == 0.0 {
        return 0.0;
    }
    dot_product / (norm_v1 * norm_v2)
}

fn deserialize_embedding(bytes: &[u8]) -> Result<Vec<f32>> {
    if bytes.len() % 4 != 0 {
        return Err(Error::Processing(
            "Invalid embedding data in database: not a multiple of 4 bytes.".to_string(),
        ));
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|c| f32::from_ne_bytes(c.try_into().unwrap()))
        .collect())
}
