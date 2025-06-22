-- migrations/20250622_initial_schema.sql
--
-- This migration script sets up the initial database schema for the
-- knowledge engine. It creates two tables: one to track the source
-- documents and another to store the processed, embedded text chunks.

-- The `documents` table stores metadata about the original source files
-- that have been ingested into the knowledge base.
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    content_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- The `chunks` table stores the individual text chunks derived from the
-- source documents, along with their vector embeddings.
CREATE TABLE IF NOT EXISTS chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id INTEGER NOT NULL,
    chunk_text TEXT NOT NULL,
    -- The vector embedding is stored as a BLOB (Binary Large Object).
    -- We will serialize the vector (e.g., Vec<f32>) into bytes before storing it.
    embedding BLOB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE
);

