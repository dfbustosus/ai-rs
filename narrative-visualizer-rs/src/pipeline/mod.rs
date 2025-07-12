//! src/pipeline/mod.rs
//!
//! This module defines the stages of the narrative visualization pipeline.
//! Each submodule represents a distinct step in transforming raw text into
//! a visual storyboard.

pub mod stage_1_scene_detection;
pub mod stage_2_prompt_generation;
pub mod stage_3_image_generation;
