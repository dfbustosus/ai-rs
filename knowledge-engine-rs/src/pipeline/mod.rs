//! src/pipeline/mod.rs
//!
//! This module defines the stages of the data processing pipeline for the
//! knowledge engine. Each submodule represents a distinct step in taking
//! raw documents and preparing them for querying.

pub mod chunking;
pub mod ingestion;
pub mod indexing;
