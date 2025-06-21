//! src/conversation_parser.rs
//!
//! This module is responsible for loading and parsing conversation transcripts
//! from JSON files into a structured format that the application can use.

use crate::error::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Represents a single turn or utterance in a conversation.
#[derive(Deserialize, Debug, Clone)]
pub struct ConversationTurn {
    pub speaker: String,
    pub text: String,
}

/// Represents the top-level structure of a conversation file.
#[derive(Deserialize, Debug, Clone)]
pub struct Conversation {
    pub conversation: Vec<ConversationTurn>,
}

/// Loads and parses a conversation transcript from the specified file path.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the conversation JSON file.
///
/// # Returns
///
/// A `Result` containing the loaded `Conversation`.
///
/// # Errors
///
/// Returns an `Error` if the file cannot be read or if the JSON is malformed.
pub fn load_conversation(file_path: &Path) -> Result<Conversation> {
    let file_content = fs::read_to_string(file_path)?;
    let conversation: Conversation = serde_json::from_str(&file_content)?;
    Ok(conversation)
}
