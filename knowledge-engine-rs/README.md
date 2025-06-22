# Knowledge Synthesis and Query Engine
This tool implements a complete Retrieval-Augmented Generation (RAG) pipeline. The engine can ingest a corpus of unstructured documents (including .txt, .md, and .pdf files), build a semantic knowledge base, and allow a user to ask complex, natural language questions against that knowledge base.

This project is designed to be a foundational example of how to build data-intensive AI systems. It moves beyond simple API calls to tackle the real-world engineering challenges of data ingestion, processing, storage, and retrieval.

# Key Features
- **End-to-End RAG Pipeline:** Implements all stages of a modern RAG system:

- **Ingestion:** Scans directories for documents and tracks changes using content hashes.

- **Chunking:** Intelligently splits large documents into smaller, semantically coherent pieces.

- **Embedding & Indexing:** Uses OpenAI's embedding models to convert text chunks into vectors and stores them in a persistent database.

- **Querying & Synthesis:** Takes a user's question, finds the most relevant information from the database using vector similarity search, and uses a generative model to synthesize a coherent, context-aware answer.

- **Persistent Knowledge Base:** Uses a local SQLite database to store all processed data, allowing the knowledge base to grow over time without needing to re-process unchanged documents.

- **Multi-Document Support:** Capable of extracting text from various file formats, including .txt, .md, and .pdf.

- **Robust & Modular Architecture:** Engineered with a strict separation of concerns, with distinct modules for the database, the data pipeline stages, and the query engine, making the system highly maintainable and extensible.

- **Modern Rust Practices:** Built with a fully asynchronous pipeline using Tokio, and leverages best-in-class crates like sqlx for database interaction and clap for a powerful CLI.

# Project Structure
The codebase is organized to reflect the multi-stage nature of the data pipeline.

```
knowledge-engine-rs/
├── .env
├── .gitignore
├── Cargo.toml
└── data/
|   └── knowledge_base.sqlite # The persistent SQLite database (created on first run).
└── documents/
|   └── (Place your source .txt, .md, and .pdf files here)
└── migrations/
|   └── ..._initial_schema.sql # The SQL script to set up the database schema.
└── src/
    ├── main.rs                 # CLI parsing and orchestration of pipeline stages.
    ├── error.rs                # Unified error handling.
    ├── config.rs               # Application configuration.
    ├── database.rs             # Manages all interaction with the SQLite database.
    ├── pipeline/
    |   ├── mod.rs              # The pipeline module definition.
    |   ├── ingestion.rs        # Stage 1: Loads and parses documents.
    |   ├── chunking.rs         # Stage 2: Splits documents into manageable chunks.
    |   └── indexing.rs         # Stage 3: Embeds and indexes the chunks.
    ├── query_engine.rs         # The core logic for answering questions.
    └── openai_client.rs        # Handles all API calls for both embedding and generation.
```

# Setup and Usage
Follow these steps to set up and run the engine on your local machine.

1. Create Necessary Directories

In the root of the project, create the documents and migrations directories if they don't already exist.
```bash
mkdir -p documents migrations
```
2. Create the Database Migration File

Inside the migrations directory, create a file named `20250622000000_initial_schema.sql` (you can use any timestamp). Add the following SQL schema to it:

```bash
-- The `documents` table stores metadata about the original source files.
CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL UNIQUE,
    content_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- The `chunks` table stores the individual text chunks and their vector embeddings.
CREATE TABLE IF NOT EXISTS chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id INTEGER NOT NULL,
    chunk_text TEXT NOT NULL,
    embedding BLOB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (document_id) REFERENCES documents (id) ON DELETE CASCADE
);
```
3. Add Your Documents

Place any `.txt`, `.md`, or `.pdf` files you want to include in your knowledge base into the `documents/` directory.

4. Set Up Your Environment File

Create a file named `.env` in the root of the project and add your OpenAI API key and the path to your database file.

```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
DATABASE_URL="sqlite:data/knowledge_base.sqlite"
```

5. Ingest Documents into the Knowledge Base

Run the ingest command. This will scan your documents folder, process any new or updated files, and store them in the database. You only need to do this when you add or modify your source documents.

```bash
cargo run -- ingest
```

```bash
cargo run -- ingest
   Compiling knowledge-engine-rs v0.1.0 (/Users/davidusta/Desktop/ai-rs/knowledge-engine-rs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.30s
     Running `target/debug/knowledge-engine-rs ingest`
2025-06-22T19:19:46.776038Z  INFO knowledge_engine_rs::database: Initializing database connection...
2025-06-22T19:19:46.777420Z  INFO knowledge_engine_rs::database: Database connection established. Running migrations...
2025-06-22T19:19:46.782407Z  INFO knowledge_engine_rs::database: Database migrations completed successfully.
2025-06-22T19:19:46.783008Z  INFO knowledge_engine_rs: Starting 'ingest' command for path: './documents'
2025-06-22T19:19:46.783059Z  INFO knowledge_engine_rs::pipeline::ingestion: Starting document ingestion from './documents'...
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
missing char 80 in unicode map {214: "Σ", 216: "Φ", 52: "\u{f8f0}", 48: "\u{f8eb}", 49: "\u{f8f6}", 55: "\u{f8fa}", 66: "\u{f8ec}", 159: "√", 54: "\u{f8ef}", 67: "\u{f8f7}", 218: "Ω", 208: "Γ", 51: "\u{f8f9}", 61: "\u{f8fd}", 215: "Υ", 57: "\u{f8fc}", 50: "\u{f8ee}", 58: "\u{f8f3}", 211: "Λ", 60: "\u{f8f2}", 217: "Ψ", 62: "\u{f8f4}", 53: "\u{f8fb}", 59: "\u{f8fe}", 63: "\u{f8e6}", 209: "∆", 65: "\u{f8f8}", 160: " ", 64: "\u{f8ed}", 212: "Ξ", 56: "\u{f8f1}", 210: "Θ", 213: "Π"} for <</Type /Font/Subtype /Type1/BaseFont /THPNLT+CMEX9/FirstChar 80/FontDescriptor 5866 0 R/LastChar 80/ToUnicode 5908 0 R/Widths 5842 0 R>>
falling back to encoding 80 -> "∑"
missing char 80 in unicode map {214: "Σ", 216: "Φ", 52: "\u{f8f0}", 48: "\u{f8eb}", 49: "\u{f8f6}", 55: "\u{f8fa}", 66: "\u{f8ec}", 159: "√", 54: "\u{f8ef}", 67: "\u{f8f7}", 218: "Ω", 208: "Γ", 51: "\u{f8f9}", 61: "\u{f8fd}", 215: "Υ", 57: "\u{f8fc}", 50: "\u{f8ee}", 58: "\u{f8f3}", 211: "Λ", 60: "\u{f8f2}", 217: "Ψ", 62: "\u{f8f4}", 53: "\u{f8fb}", 59: "\u{f8fe}", 63: "\u{f8e6}", 209: "∆", 65: "\u{f8f8}", 160: " ", 64: "\u{f8ed}", 212: "Ξ", 56: "\u{f8f1}", 210: "Θ", 213: "Π"} for <</Type /Font/Subtype /Type1/BaseFont /THPNLT+CMEX9/FirstChar 80/FontDescriptor 5866 0 R/LastChar 80/ToUnicode 5908 0 R/Widths 5842 0 R>>
falling back to encoding 80 -> "∑"
missing char 80 in unicode map {214: "Σ", 216: "Φ", 52: "\u{f8f0}", 48: "\u{f8eb}", 49: "\u{f8f6}", 55: "\u{f8fa}", 66: "\u{f8ec}", 159: "√", 54: "\u{f8ef}", 67: "\u{f8f7}", 218: "Ω", 208: "Γ", 51: "\u{f8f9}", 61: "\u{f8fd}", 215: "Υ", 57: "\u{f8fc}", 50: "\u{f8ee}", 58: "\u{f8f3}", 211: "Λ", 60: "\u{f8f2}", 217: "Ψ", 62: "\u{f8f4}", 53: "\u{f8fb}", 59: "\u{f8fe}", 63: "\u{f8e6}", 209: "∆", 65: "\u{f8f8}", 160: " ", 64: "\u{f8ed}", 212: "Ξ", 56: "\u{f8f1}", 210: "Θ", 213: "Π"} for <</Type /Font/Subtype /Type1/BaseFont /THPNLT+CMEX9/FirstChar 80/FontDescriptor 5866 0 R/LastChar 80/ToUnicode 5908 0 R/Widths 5842 0 R>>
falling back to encoding 80 -> "∑"
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
Unicode mismatch true fi "fi" Ok("ﬁ") [64257]
Unicode mismatch true fl "fl" Ok("ﬂ") [64258]
2025-06-22T19:19:48.741891Z  INFO knowledge_engine_rs::pipeline::ingestion: Ingesting new document: './documents/attention.pdf'
2025-06-22T19:19:48.743720Z  INFO knowledge_engine_rs::pipeline::ingestion: Document ingestion complete. Found 1 new or updated documents to process.
2025-06-22T19:19:48.743739Z  INFO knowledge_engine_rs::pipeline::chunking: Starting document chunking process...
2025-06-22T19:19:48.749359Z  INFO knowledge_engine_rs::pipeline::chunking: Split document './documents/attention.pdf' (ID: 1) into 50 chunks.
2025-06-22T19:19:48.749386Z  INFO knowledge_engine_rs::pipeline::chunking: Document chunking complete. Generated 50 total chunks.
2025-06-22T19:19:48.749482Z  INFO index_chunks: knowledge_engine_rs::pipeline::indexing: Starting chunk indexing process for 50 chunks...
2025-06-22T19:19:48.749679Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:49.691260Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:49.692395Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:50.399983Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:50.400445Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:51.035227Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:51.035632Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:51.500766Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:51.501397Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:52.185885Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:52.186456Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:52.917141Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:52.917867Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:53.460347Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:53.460973Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:53.982875Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:53.983627Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:54.309972Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:54.310567Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:54.671920Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:54.672671Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:55.202267Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:55.202848Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:55.899884Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:55.900457Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:56.580834Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:56.581530Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:57.096917Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:57.097662Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:57.762451Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:57.763033Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:58.098237Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:58.098964Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:58.547914Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:58.548439Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:59.273871Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:59.274611Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:19:59.616863Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:19:59.617435Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:00.104993Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:00.105727Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:00.482183Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:00.482811Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:00.839979Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:00.840590Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:01.301714Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:01.302461Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:01.760095Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:01.760676Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:02.061962Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:02.062540Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:02.529468Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:02.530069Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:03.302757Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:03.303365Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:06.474843Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:06.475424Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:06.772664Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:06.773258Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:07.297135Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:07.297909Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:07.684460Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:07.685067Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:08.004686Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:08.005307Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:08.517446Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:08.518024Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:08.705665Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:08.706246Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:08.952326Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:08.952935Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:09.361312Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:09.361977Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:10.117363Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:10.118074Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:10.525980Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:10.526711Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:11.199291Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:11.200063Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:11.503949Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:11.504549Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:11.865849Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:11.866427Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:12.170942Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:12.171756Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:12.419264Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:12.419731Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:12.766206Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:12.766798Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:13.025607Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:13.026273Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:13.393420Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:13.393986Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:13.590095Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:13.590679Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:13.771178Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:13.771788Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:14.144294Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:14.145024Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:20:14.524889Z  INFO index_chunks:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:20:14.527313Z  INFO index_chunks: knowledge_engine_rs::pipeline::indexing: Successfully indexed 50 chunks into the database.
2025-06-22T19:20:14.527370Z  INFO knowledge_engine_rs: Ingestion process completed successfully.
```
6. Query Your Knowledge Base

Once your documents have been ingested, you can ask questions using the query command. The -- separator is crucial to distinguish arguments for Cargo from arguments for your application.

Example Command:
```bash
cargo run -- query "What is the Transformer architecture?"
```

```bash
 Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/knowledge-engine-rs query 'What is the Transformer architecture?'`
2025-06-22T19:21:11.686529Z  INFO knowledge_engine_rs::database: Initializing database connection...
2025-06-22T19:21:11.687666Z  INFO knowledge_engine_rs::database: Database connection established. Running migrations...
2025-06-22T19:21:11.688554Z  INFO knowledge_engine_rs::database: Database migrations completed successfully.
2025-06-22T19:21:11.689159Z  INFO knowledge_engine_rs: Starting 'query' command with question: 'What is the Transformer architecture?'
2025-06-22T19:21:11.689252Z  INFO answer_question: knowledge_engine_rs::query_engine: Answering question: 'What is the Transformer architecture?'
2025-06-22T19:21:11.689283Z  INFO answer_question:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:21:12.177515Z  INFO answer_question:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:21:12.177601Z  INFO answer_question: knowledge_engine_rs::query_engine: Searching for relevant chunks in the database...
2025-06-22T19:21:12.190756Z  INFO answer_question: knowledge_engine_rs::query_engine: Found 5 relevant chunks.
2025-06-22T19:21:12.190848Z  INFO answer_question:get_completion: knowledge_engine_rs::openai_client: Requesting completion from OpenAI API.
2025-06-22T19:21:15.130488Z  INFO answer_question:get_completion: knowledge_engine_rs::openai_client: Successfully received completion from API.

Answer:
The Transformer architecture is a novel neural sequence transduction model that follows the encoder-decoder structure typical in competitive models. It differentiates itself by completely relying on attention mechanisms, specifically self-attention, to draw global dependencies between the input and output sequences. Unlike earlier models, the Transformer dispenses with recurrence and convolution, allowing for significantly more parallelization. This architecture uses stacked self-attention and point-wise, fully connected layers for both its encoder and decoder. These innovations make the Transformer not only more efficient in training time but also capable of achieving state-of-the-art performance on translation tasks.
```


```bash
cargo run -- query "Explain the concept of self-attention."
```

```bash
 Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/knowledge-engine-rs query 'Explain the concept of self-attention.'`
2025-06-22T19:21:27.646576Z  INFO knowledge_engine_rs::database: Initializing database connection...
2025-06-22T19:21:27.647778Z  INFO knowledge_engine_rs::database: Database connection established. Running migrations...
2025-06-22T19:21:27.648695Z  INFO knowledge_engine_rs::database: Database migrations completed successfully.
2025-06-22T19:21:27.649273Z  INFO knowledge_engine_rs: Starting 'query' command with question: 'Explain the concept of self-attention.'
2025-06-22T19:21:27.649363Z  INFO answer_question: knowledge_engine_rs::query_engine: Answering question: 'Explain the concept of self-attention.'
2025-06-22T19:21:27.649394Z  INFO answer_question:get_embedding: knowledge_engine_rs::openai_client: Requesting embedding from OpenAI API.
2025-06-22T19:21:27.979204Z  INFO answer_question:get_embedding: knowledge_engine_rs::openai_client: Successfully generated text embedding.
2025-06-22T19:21:27.979288Z  INFO answer_question: knowledge_engine_rs::query_engine: Searching for relevant chunks in the database...
2025-06-22T19:21:27.992046Z  INFO answer_question: knowledge_engine_rs::query_engine: Found 5 relevant chunks.
2025-06-22T19:21:27.992130Z  INFO answer_question:get_completion: knowledge_engine_rs::openai_client: Requesting completion from OpenAI API.
2025-06-22T19:21:30.989324Z  INFO answer_question:get_completion: knowledge_engine_rs::openai_client: Successfully received completion from API.

Answer:
Self-attention, sometimes referred to as intra-attention, is an attention mechanism that relates different positions within a single sequence to compute a representation of the sequence. This mechanism allows a model to consider the entire input sequence and how its parts relate to each other, which can be particularly useful in tasks such as reading comprehension, abstractive summarization, textual entailment, and learning task-independent sentence representations. In the context of a model like the Transformer, self-attention is a core component that enables it to compute representations of its input and output without relying on traditional sequence-aligned recurrent neural networks (RNNs) or convolutional layers. Self-attention has several advantages, including potentially lower computational complexity, the ability to parallelize computations better, and producing more interpretable models by examining the learned attention distributions, which can capture syntactic and semantic structures in the input.
```