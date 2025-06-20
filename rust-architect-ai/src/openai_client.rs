//! src/openai_client.rs
//!
//! This module provides a dedicated client for interacting with the OpenAI API.
//! It encapsulates all the logic for creating and sending requests, as well as
//! handling the responses in a structured way.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const AI_MODEL_NAME: &str = "gpt-4o";

/// A client for making requests to the OpenAI Chat Completions API.
#[derive(Clone)]
pub struct OpenAIClient {
    http_client: reqwest::Client,
    api_key: String,
}

impl OpenAIClient {
    /// Creates a new instance of the `OpenAIClient`.
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Sends a request to the OpenAI API with a given prompt.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The complete prompt to be sent to the language model.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of the AI's response as a `String`.
    pub async fn send_request(&self, prompt: String) -> Result<String> {
        info!("Sending request to OpenAI API...");

        let messages = vec![Message {
            role: "user".to_string(),
            content: prompt,
        }];

        let body = ChatCompletionRequest {
            model: AI_MODEL_NAME.to_string(),
            messages,
        };

        // Send the request and handle potential errors robustly.
        let response: ChatCompletionResponse = self
            .http_client
            .post(OPENAI_API_URL)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()? // This is crucial for catching non-2xx responses.
            .json()
            .await?;

        // Extract the message content from the first choice in the response.
        if let Some(choice) = response.choices.into_iter().next() {
            info!("Successfully received response from OpenAI API.");
            Ok(choice.message.content)
        } else {
            Err(Error::OpenAI(
                "API response did not contain any choices.".to_string(),
            ))
        }
    }
}

//========= API Data Structures =========//

#[derive(Serialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}
