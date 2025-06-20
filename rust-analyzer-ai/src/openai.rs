//! src/openai.rs
//!
//! This module serves as the client for the OpenAI API. It is specifically
//! tailored to send Rust source code for analysis and retrieve actionable
//! feedback.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

// Defines the client responsible for making requests to OpenAI.
#[derive(Clone)]
pub struct Client {
    http_client: reqwest::Client,
    api_key: String,
}

impl Client {
    /// Creates a new OpenAI client.
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            api_key,
        }
    }

    /// Sends a file's content to the OpenAI API for analysis.
    ///
    /// This function constructs a specialized prompt, asking the AI model to act as
    /// a senior Rust developer and provide refactoring suggestions.
    ///
    /// # Arguments
    ///
    /// * `file_content` - A string slice containing the Rust source code to analyze.
    ///
    /// # Returns
    ///
    /// A `Result` containing the AI-generated analysis as a `String`.
    pub async fn analyze_code(&self, file_content: &str) -> Result<String> {
        let system_prompt = "You are an expert Rust programmer with over 20 years of experience. \
            You are acting as a code reviewer. Your goal is to provide concise, actionable feedback \
            to help a developer improve their code. Focus on identifying anti-patterns, suggesting \
            idiomatic Rust, improving clarity, and pointing out potential performance improvements. \
            Do not comment on code style like formatting, as that is handled by rustfmt. \
            Provide your feedback in a clear, bulleted list.";

        let user_prompt = format!(
            "Please review the following Rust code and provide refactoring suggestions:\n\n```rust\n{}\n```",
            file_content
        );

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        let body = ChatCompletionRequest {
            model: "gpt-4o".to_string(), // Using a more advanced model for better code analysis.
            messages,
        };

        // Make the API request.
        let response: ChatCompletionResponse = self
            .http_client
            .post(OPENAI_API_URL)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        // Extract the content from the API response.
        if let Some(choice) = response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err(Error::OpenAI("No analysis received from API".to_string()))
        }
    }
}

//========= API Data Structures =========//
// These structs map to the JSON format of the OpenAI Chat Completions API.

#[derive(Serialize, Debug)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
