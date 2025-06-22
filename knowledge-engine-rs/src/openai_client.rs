//! src/openai_client.rs
//!
//! This module provides a dedicated client for interacting with the OpenAI API.
//! It is designed to handle different types of requests, including text
//! embeddings and generative completions.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

const OPENAI_API_URL: &str = "https://api.openai.com/v1";
const EMBEDDING_MODEL: &str = "text-embedding-3-small";
const GENERATIVE_MODEL: &str = "gpt-4o";

/// A client for making requests to the OpenAI API.
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

    /// Generates a vector embedding for a given piece of text.
    #[instrument(skip(self, text))]
    pub async fn get_embedding(&self, text: &str) -> Result<Vec<f32>> {
        info!("Requesting embedding from OpenAI API.");

        let body = EmbeddingRequest {
            input: text.to_string(),
            model: EMBEDDING_MODEL.to_string(),
        };

        let response: EmbeddingResponse = self
            .http_client
            .post(format!("{}/embeddings", OPENAI_API_URL))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(embedding_data) = response.data.into_iter().next() {
            info!("Successfully generated text embedding.");
            Ok(embedding_data.embedding)
        } else {
            Err(Error::OpenAI(
                "API response did not contain any embedding data.".to_string(),
            ))
        }
    }

    /// Generates a conversational completion based on a system and user prompt.
    #[instrument(skip(self, system_prompt, user_prompt))]
    pub async fn get_completion(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        info!("Requesting completion from OpenAI API.");

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];

        let body = ChatCompletionRequest {
            model: GENERATIVE_MODEL.to_string(),
            messages,
        };

        let response: ChatCompletionResponse = self
            .http_client
            .post(format!("{}/chat/completions", OPENAI_API_URL))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(choice) = response.choices.into_iter().next() {
            info!("Successfully received completion from API.");
            Ok(choice.message.content)
        } else {
            Err(Error::OpenAI(
                "API response did not contain any choices.".to_string(),
            ))
        }
    }
}

//========= API Data Structures =========//

#[derive(Serialize)]
struct EmbeddingRequest {
    input: String,
    model: String,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

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
