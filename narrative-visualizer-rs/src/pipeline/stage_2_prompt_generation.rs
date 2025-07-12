//! src/pipeline/stage_2_prompt_generation.rs
//!
//! The second stage of the narrative visualization pipeline. This module takes
//! the scenes identified in stage 1 and uses an AI model to generate rich,
//! descriptive visual prompts suitable for an image generation API.

use crate::error::Result;
use crate::openai_client::OpenAIClient;
use crate::pipeline::stage_1_scene_detection::Scene;
use tracing::{info, instrument};

/// Represents a scene that has been enriched with a detailed visual prompt
/// ready for image generation.
#[derive(Debug)]
pub struct VisualPrompt {
    pub scene_description: String,
    pub original_text: String,
    pub image_prompt: String,
}

/// Takes a list of scenes and generates a detailed visual prompt for each one.
///
/// # Arguments
///
/// * `client` - An instance of the `OpenAIClient`.
/// * `scenes` - A slice of `Scene` structs from the previous pipeline stage.
///
/// # Returns
///
/// A `Result` containing a `Vec<VisualPrompt>` on success.
#[instrument(skip_all)]
pub async fn generate_visual_prompts(
    client: &OpenAIClient,
    scenes: &[Scene],
) -> Result<Vec<VisualPrompt>> {
    info!("Starting visual prompt generation for {} scenes.", scenes.len());

    let mut visual_prompts = Vec::new();
    let system_prompt = "You are a creative visual artist and art director. Your task is to take a simple scene description and expand it into a rich, detailed, and evocative prompt for an AI image generation model like DALL-E 3. The prompt must be a single, descriptive paragraph and should specify the mood, lighting, color palette, camera angle, and artistic style.";

    for (index, scene) in scenes.iter().enumerate() {
        info!("Generating prompt for scene {}/{}...", index + 1, scenes.len());
        let user_prompt = build_user_prompt(&scene.description);

        // Call the AI to transform the simple description into a rich prompt.
        let image_prompt = client.get_completion(system_prompt, &user_prompt).await?;

        visual_prompts.push(VisualPrompt {
            scene_description: scene.description.clone(),
            original_text: scene.original_text.clone(),
            image_prompt,
        });
    }

    info!("Successfully generated {} visual prompts.", visual_prompts.len());
    Ok(visual_prompts)
}

/// Constructs the detailed user prompt for the visual prompt generation task.
fn build_user_prompt(scene_description: &str) -> String {
    format!(
        "Based on the following scene description, generate a detailed visual prompt for an image generation model.

        The prompt should be a single, descriptive paragraph. Emphasize a 'cinematic film noir' artistic style. Include details about lighting (e.g., 'dramatic chiaroscuro lighting'), color (e.g., 'monochromatic with a single splash of color'), and camera angle (e.g., 'low-angle shot').

        Scene Description:
        \"\"\"
        {}
        \"\"\"

        Generated Visual Prompt:",
        scene_description
    )
}
