//! src/openai_client.rs
//!
//! This module provides the client for interacting with the OpenAI API.
//! It encapsulates the logic for constructing requests, sending them,
//! and parsing the responses for the sentiment analysis task.

use crate::{constants, error::Result};
use serde::{Deserialize, Serialize};

/// The client for making requests to the OpenAI Chat Completions API.
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

    /// Sends a request to the OpenAI API to perform sentiment analysis.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The fully constructed prompt to send to the model.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of the AI's response as a `String`.
    pub async fn send_request(&self, prompt: String) -> Result<String> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: prompt,
        }];

        let body = ChatCompletionRequest {
            model: constants::AI_MODEL_NAME.to_string(),
            messages,
        };

        // Send the request and handle potential errors.
        let response: ChatCompletionResponse = self
            .http_client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        // Extract the message content from the first choice in the response.
        if let Some(choice) = response.choices.into_iter().next() {
            Ok(choice.message.content)
        } else {
            Err(crate::error::Error::OpenAI(
                "No response choices were returned from the API.".to_string(),
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
