//! src/output_assembler.rs
//!
//! This module is responsible for taking the final processed storyboard frames
//! and assembling them into a single, user-viewable output file, such as HTML.

use crate::error::Result;
use crate::pipeline::stage_3_image_generation::StoryboardFrame;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::path::Path;
use tracing::info;

/// Assembles a storyboard from a collection of frames and saves it as an HTML file.
pub fn assemble_storyboard_html(frames: &[StoryboardFrame], output_path: &Path) -> Result<()> {
    info!("Assembling final storyboard HTML at '{}'...", output_path.display());

    let mut html_content = String::new();
    html_content.push_str(HTML_HEADER);

    for frame in frames {
        // Corrected: Use the modern Engine API for encoding.
        let image_base64 = STANDARD.encode(&frame.image_data);
        let image_src = format!("data:image/png;base64,{}", image_base64);

        // Corrected: Use all fields from the frame to create a richer output.
        let frame_html = format!(
            r#"
            <div class="frame">
                <div class="image-container">
                    <img src="{}" alt="{}" title="Visual Prompt: {}">
                </div>
                <div class="text-container">
                    <h4>{}</h4>
                    <p>{}</p>
                </div>
            </div>
            "#,
            image_src,
            frame.scene_description, // Use description for alt text
            frame.image_prompt,      // Use full prompt for tooltip
            frame.scene_description, // Display description as a sub-header
            frame.original_text.trim()
        );
        html_content.push_str(&frame_html);
    }

    html_content.push_str(HTML_FOOTER);

    if let Some(parent_dir) = output_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    fs::write(output_path, html_content)?;

    info!("Successfully assembled and saved storyboard.");
    Ok(())
}

// Corrected: Added styling for the new h4 element.
const HTML_HEADER: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AI Generated Storyboard</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; line-height: 1.6; background-color: #f4f4f9; color: #333; margin: 0; padding: 2rem; }
        .container { max-width: 900px; margin: auto; background: #fff; padding: 2rem; box-shadow: 0 4px 12px rgba(0,0,0,0.1); border-radius: 8px; }
        h1 { text-align: center; color: #1a1a1a; }
        .frame { display: flex; flex-wrap: wrap; gap: 2rem; margin-bottom: 3rem; padding-bottom: 2rem; border-bottom: 1px solid #e0e0e0; align-items: center; }
        .frame:last-child { border-bottom: none; }
        .image-container { flex: 1; min-width: 300px; }
        .text-container { flex: 1.5; min-width: 300px; }
        img { max-width: 100%; height: auto; border-radius: 6px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
        h4 { margin-top: 0; margin-bottom: 0.5rem; color: #555; font-weight: bold; }
        p { font-size: 1.1rem; color: #444; font-style: italic; }
    </style>
</head>
<body>
    <div class="container">
        <h1>AI Generated Storyboard</h1>
"#;

const HTML_FOOTER: &str = r#"
    </div>
</body>
</html>
"#;
