//! src/pipeline/stage_3_image_generation.rs
//!
//! The third and final stage of the narrative visualization pipeline. This
//! module takes the visually descriptive prompts and uses an AI image
//! generation model to synthesize an image for each scene.

use crate::error::Result;
use crate::openai_client::OpenAIClient;
use crate::pipeline::stage_2_prompt_generation::VisualPrompt;
use tracing::{info, instrument};

/// Represents a fully processed scene, containing the original text, its
/// description, the prompt used for image generation, and the raw byte data
/// of the generated image itself.
#[derive(Debug)]
pub struct StoryboardFrame {
    pub original_text: String,
    pub scene_description: String,
    pub image_prompt: String,
    pub image_data: Vec<u8>, // The raw PNG/JPEG data of the generated image.
}

/// Takes a list of visual prompts and generates an image for each one.
///
/// # Arguments
///
/// * `client` - An instance of the `OpenAIClient`.
/// * `prompts` - A slice of `VisualPrompt` structs from the previous pipeline stage.
///
/// # Returns
///
/// A `Result` containing a `Vec<StoryboardFrame>` on success.
#[instrument(skip_all)]
pub async fn generate_images(
    client: &OpenAIClient,
    prompts: &[VisualPrompt],
) -> Result<Vec<StoryboardFrame>> {
    info!("Starting image generation for {} prompts.", prompts.len());

    let mut storyboard_frames = Vec::new();

    for (index, prompt) in prompts.iter().enumerate() {
        info!("Generating image for scene {}/{}...", index + 1, prompts.len());

        // Call the AI to synthesize an image based on the detailed prompt.
        let image_data = client.generate_image(&prompt.image_prompt).await?;

        storyboard_frames.push(StoryboardFrame {
            original_text: prompt.original_text.clone(),
            scene_description: prompt.scene_description.clone(),
            image_prompt: prompt.image_prompt.clone(),
            image_data,
        });
    }

    info!("Successfully generated {} images.", storyboard_frames.len());
    Ok(storyboard_frames)
}
