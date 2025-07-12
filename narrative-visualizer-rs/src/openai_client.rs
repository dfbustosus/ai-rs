//! src/openai_client.rs
//!
//! This module provides a dedicated, multi-modal client for interacting with
//! the OpenAI API. It supports both text generation via the Chat Completions
//! endpoint and image generation via the DALL-E 3 endpoint.

use crate::error::{Error, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

const OPENAI_API_URL: &str = "https://api.openai.com/v1";
const TEXT_MODEL: &str = "gpt-4o";
const IMAGE_MODEL: &str = "dall-e-3";

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

    /// Sends a text-based request to the Chat Completions API.
    #[instrument(skip(self, system_prompt, user_prompt))]
    pub async fn get_completion(&self, system_prompt: &str, user_prompt: &str) -> Result<String> {
        info!("Requesting text completion from OpenAI API.");

        let messages = vec![
            Message { role: "system".to_string(), content: system_prompt.to_string() },
            Message { role: "user".to_string(), content: user_prompt.to_string() },
        ];

        let body = ChatCompletionRequest {
            model: TEXT_MODEL.to_string(),
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
            info!("Successfully received text completion.");
            Ok(choice.message.content)
        } else {
            Err(Error::OpenAI("API response did not contain any text choices.".to_string()))
        }
    }

    /// Sends a request to the Image Generation API to synthesize an image.
    #[instrument(skip(self, prompt))]
    pub async fn generate_image(&self, prompt: &str) -> Result<Vec<u8>> {
        info!("Requesting image generation from OpenAI API.");

        let body = ImageGenerationRequest {
            model: IMAGE_MODEL.to_string(),
            prompt: prompt.to_string(),
            n: 1,
            size: "1024x1024".to_string(),
            response_format: "b64_json".to_string(),
        };

        let response: ImageGenerationResponse = self
            .http_client
            .post(format!("{}/images/generations", OPENAI_API_URL))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        if let Some(image_data) = response.data.into_iter().next() {
            info!("Successfully received image data.");
            // Corrected: Use the modern Engine API for decoding.
            let image_bytes = STANDARD.decode(&image_data.b64_json)?;
            Ok(image_bytes)
        } else {
            Err(Error::OpenAI("API response did not contain any image data.".to_string()))
        }
    }
}

//========= API Data Structures =========//

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Serialize)]
struct ImageGenerationRequest {
    model: String,
    prompt: String,
    n: u32,
    size: String,
    response_format: String,
}

#[derive(Deserialize)]
struct ImageGenerationResponse {
    data: Vec<ImageData>,
}

#[derive(Deserialize)]
struct ImageData {
    b64_json: String,
}
