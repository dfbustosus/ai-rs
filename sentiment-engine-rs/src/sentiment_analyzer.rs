//! src/sentiment_analyzer.rs
//!
//! This module contains the core analysis logic. It constructs the prompts,
//! interacts with the OpenAI client, and parses the structured response.

use crate::config::SentimentConfig;
use crate::error::{Error, Result};
use crate::openai_client::OpenAIClient;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use tracing::info;

/// A lazily-compiled regular expression to robustly extract a JSON object
/// from within a Markdown code block.
/// The `(?s)` flag allows `.` to match newlines.
static JSON_EXTRACTOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```(?:json)?\s*(\{.*\})\s*```").unwrap());

/// The main analyzer struct, holding the necessary components for analysis.
pub struct SentimentAnalyzer {
    client: OpenAIClient,
    config: SentimentConfig,
}

/// The structure of the JSON object we expect to receive from the AI.
#[derive(Deserialize, Debug)]
pub struct AnalysisResult {
    #[serde(rename = "chainOfThought")]
    pub chain_of_thought: String,
    pub sentiment: String,
}

impl SentimentAnalyzer {
    /// Creates a new instance of the `SentimentAnalyzer`.
    pub fn new(client: OpenAIClient, config: SentimentConfig) -> Self {
        Self { client, config }
    }

    /// Analyzes the provided text to determine its sentiment.
    ///
    /// This function builds a detailed prompt, sends it to the OpenAI API,
    /// and parses the resulting JSON object into an `AnalysisResult`.
    ///
    /// # Arguments
    ///
    /// * `text_to_analyze` - A string slice of the text to be analyzed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the structured `AnalysisResult`.
    pub async fn analyze(&self, text_to_analyze: &str) -> Result<AnalysisResult> {
        info!("Starting sentiment analysis.");

        // Build the detailed prompt for the AI.
        let prompt = self.build_prompt(text_to_analyze);
        info!(prompt = %prompt, "Constructed analysis prompt.");

        // Send the request to the OpenAI client.
        let response_text = self.client.send_request(prompt).await?;
        info!(response = %response_text, "Received response from API.");

        // Use the robust regex-based method to extract the JSON payload.
        // If the regex does not find a match, or if the AI simply returns raw JSON,
        // we fall back to parsing the raw text.
        let json_text = JSON_EXTRACTOR
            .captures(&response_text)
            .and_then(|caps| caps.get(1).map(|m| m.as_str()))
            .unwrap_or(&response_text);

        serde_json::from_str(json_text.trim()).map_err(|e| {
            Error::InvalidResponseFormat(format!(
                "Failed to parse JSON response: {}. Response text: '{}'",
                e, response_text
            ))
        })
    }

    /// Constructs the detailed prompt for the AI model.
    ///
    /// This function creates a prompt that instructs the model to follow a specific
    /// reasoning process (Chain of Thought) and to format its output as a JSON object.
    fn build_prompt(&self, text_to_analyze: &str) -> String {
        let labels_description = self
            .config
            .labels
            .iter()
            .map(|label| format!("- \"{}\": {}", label.name, label.description))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "You are an expert sentiment analysis engine. Your task is to analyze the provided text \
            and classify it according to one of the following predefined sentiment labels. You must \
            provide your reasoning process and then the final classification in a specific JSON format.

            Sentiment Labels:
            {labels_description}

            Follow these steps precisely:
            1.  **Chain of Thought**: First, write a step-by-step reasoning process explaining your analysis. \
                Consider the explicit words, the context, and the likely intent of the author. This reasoning \
                must be detailed.
            2.  **Sentiment Classification**: After your reasoning, choose the single best sentiment label \
                from the provided list that accurately describes the text.

            Your final output must be a single, valid JSON object with two keys: \"chainOfThought\" and \"sentiment\". \
            Do not include any other text or explanations outside of the JSON object.

            Text to Analyze:
            \"\"\"
            {text_to_analyze}
            \"\"\""
        )
    }
}
