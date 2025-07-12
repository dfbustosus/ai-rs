//! src/pipeline/stage_1_scene_detection.rs
//!
//! The first stage of the narrative visualization pipeline. This module is
//! responsible for taking a block of source text and using an AI model to
//! decompose it into a structured list of distinct scenes.

use crate::error::{Error, Result};
use crate::openai_client::OpenAIClient;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use tracing::{info, instrument};

/// A lazily-compiled regular expression to robustly extract a JSON object
/// from within the AI's response, which might include markdown code fences.
static JSON_EXTRACTOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)\s*\{.*\}\s*").unwrap());

/// Represents a single, distinct scene identified by the AI.
#[derive(Deserialize, Debug)]
pub struct Scene {
    pub description: String,
    #[serde(rename = "originalText")]
    pub original_text: String,
}

/// The top-level structure that the AI is instructed to return.
#[derive(Deserialize, Debug)]
struct SceneDetectionResponse {
    scenes: Vec<Scene>,
}

/// Analyzes a narrative text and breaks it down into distinct scenes.
#[instrument(skip_all)]
pub async fn detect_scenes(client: &OpenAIClient, narrative_text: &str) -> Result<Vec<Scene>> {
    info!("Starting scene detection.");

    let system_prompt = "You are an expert film director and script analyst. Your task is to read the provided narrative text and break it down into distinct, visually coherent scenes or 'shots'. Each scene should represent a single, continuous moment or a specific visual focus.";
    let user_prompt = build_user_prompt(narrative_text);

    let response_text = client.get_completion(system_prompt, &user_prompt).await?;
    info!(raw_response = %response_text, "Received raw response from API.");

    // Corrected: Robustly extract the JSON part of the response using regex.
    let json_text = JSON_EXTRACTOR
        .find(&response_text)
        .map(|m| m.as_str())
        .ok_or_else(|| {
            Error::Pipeline(
                "Could not find a valid JSON object in the AI's response.".to_string(),
            )
        })?;

    let parsed_response: SceneDetectionResponse = serde_json::from_str(json_text).map_err(|e| {
        Error::Pipeline(format!(
            "Failed to parse scene detection response: {}. Extracted text: '{}'",
            e, json_text
        ))
    })?;

    if parsed_response.scenes.is_empty() {
        Err(Error::Pipeline("Scene detection returned no scenes.".to_string()))
    } else {
        info!("Successfully detected {} scenes.", parsed_response.scenes.len());
        Ok(parsed_response.scenes)
    }
}

/// Constructs the detailed user prompt for the scene detection task.
fn build_user_prompt(narrative_text: &str) -> String {
    let output_schema = serde_json::json!({
      "scenes": [
        {
          "description": "A concise, one-sentence description of the key visual elements and action in this specific scene.",
          "originalText": "The exact, unmodified segment of the original text that corresponds to this scene."
        }
      ]
    });

    format!(
        "Analyze the following narrative text. Decompose it into a sequence of scenes.

        Your final output must be a single, valid JSON object. The root object should have a single key, \"scenes\", which contains an array of scene objects. Each scene object must strictly adhere to the following schema:
        ```json
        {}
        ```

        Ensure that the `originalText` fields, when concatenated, perfectly reconstruct the original narrative without any modifications, additions, or omissions. Do not include any text or explanations outside of the JSON object.

        Narrative Text to Analyze:
        \"\"\"
        {}
        \"\"\"
        ",
        serde_json::to_string_pretty(&output_schema).unwrap(),
        narrative_text
    )
}
