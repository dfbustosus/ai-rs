//! src/distiller_engine.rs
//!
//! This module contains the core logic of the application. It is responsible for
//! taking a conversation and a tone profile, constructing the appropriate prompt,
//! and using the OpenAI client to generate the final, distilled summary.

use crate::config::ToneProfile;
use crate::conversation_parser::Conversation;
use crate::error::Result;
use crate::openai_client::OpenAIClient;
use tracing::{info, instrument};

/// The main engine responsible for distilling conversations.
pub struct DistillerEngine {
    client: OpenAIClient,
}

impl DistillerEngine {
    /// Creates a new instance of the `DistillerEngine`.
    pub fn new(client: OpenAIClient) -> Self {
        Self { client }
    }

    /// Distills a conversation into a summary based on a specified tone profile.
    ///
    /// # Arguments
    ///
    /// * `conversation` - The conversation transcript to be distilled.
    /// * `profile` - The tone profile that will guide the AI's response.
    ///
    /// # Returns
    ///
    /// A `Result` containing the final summary as a `String`.
    #[instrument(skip(self, conversation, profile))]
    pub async fn distill(
        &self,
        conversation: &Conversation,
        profile: &ToneProfile,
    ) -> Result<String> {
        info!(profile_name = %profile.name, "Distilling conversation.");

        // Construct the two parts of the prompt.
        let system_prompt = &profile.system_prompt;
        let user_prompt = self.format_conversation_for_prompt(conversation);

        // Use the client to get the distilled summary from the AI.
        self.client
            .send_request(system_prompt, &user_prompt)
            .await
    }

    /// Formats the conversation into a simple, readable script format for the AI.
    ///
    /// This helper function turns the structured conversation data into a plain
    /// text block that is easy for the language model to understand.
    fn format_conversation_for_prompt(&self, conversation: &Conversation) -> String {
        let mut formatted_text = String::from("CONVERSATION TRANSCRIPT:\n---\n");

        for turn in &conversation.conversation {
            let line = format!("{}: {}\n", turn.speaker, turn.text);
            formatted_text.push_str(&line);
        }

        formatted_text.push_str("---\nEND OF TRANSCRIPT");

        formatted_text
    }
}
