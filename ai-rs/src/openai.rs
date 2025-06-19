//! src/openai.rs
//!
//! This module is the core of the chatbot's AI capabilities.
//! It handles all interactions with the OpenAI Chat Completions API.
//! This includes:
//! 1. Defining data structures that map directly to the API's JSON format.
//! 2. Constructing and sending HTTP requests with the correct headers and body.
//! 3. Parsing the API's response to extract the chatbot's message.

use crate::error::{Error, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

// A client to interact with the OpenAI API.
// It holds the HTTP client and the API key for making authenticated requests.
#[derive(Clone)]
pub struct Client {
    http_client: reqwest::Client,
    api_key: String,
}

impl Client {
    /// Creates a new `Client`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The OpenAI API key to be used for authentication.
    ///
    /// # Returns
    ///
    /// A new `Client` instance.
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Sends a chat completion request to the OpenAI API.
    ///
    /// This function takes a history of messages, sends them to the API,
    /// and returns the model's response.
    ///
    /// # Arguments
    ///
    /// * `messages` - A slice of `Message` structs representing the conversation history.
    ///
    /// # Returns
    ///
    /// A `Result` containing the content of the AI's response as a `String`.
    pub async fn chat_completion(&self, messages: &[Message]) -> Result<String> {
        println!("{}", "Sending request to OpenAI...".cyan());

        let body = ChatCompletionRequest {
            model: "gpt-3.5-turbo".to_string(), // Or "gpt-4" if you have access
            messages: messages.to_vec(),
        };

        let response: ChatCompletionResponse = self
            .http_client
            .post(OPENAI_API_URL)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await? // The '?' operator propagates errors from reqwest
            .error_for_status()? // Ensure non-successful HTTP responses are caught
            .json()
            .await?; // The '?' operator propagates errors from JSON parsing

        // Extract the content from the first choice in the response.
        if let Some(choice) = response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            // If the API returns no choices, it's an unexpected scenario.
            // We map this to our custom OpenAI error type.
            Err(Error::OpenAI("No response choices found".to_string()))
        }
    }
}

//========= API Data Structures =========//
// These structs are designed to match the JSON format of the OpenAI API.
// `serde`'s `derive` macros handle the serialization and deserialization.

/// Represents the overall request sent to the Chat Completions API.
#[derive(Serialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

/// Represents a single message in the conversation.
/// This can be from the "system", "user", or "assistant".
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Represents the response received from the API.
#[derive(Deserialize, Debug)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

/// Represents a single completion choice. The API can sometimes return
/// multiple choices, but we will typically only use the first one.
#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}
