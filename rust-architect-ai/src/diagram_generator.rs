//! src/diagram_generator.rs
//!
//! This module is the core engine of the application. It takes the aggregated
//! source code context, constructs a sophisticated prompt to request an
//! architectural diagram, and parses the AI's response to extract the
//! final diagram syntax.

use crate::error::{Error, Result};
use crate::openai_client::OpenAIClient;
use clap::ValueEnum;
use once_cell::sync::Lazy;
use regex::Regex;
use tracing::info;

/// A lazily-compiled regular expression to robustly extract diagram syntax
/// from within a Markdown code block (e.g., ```mermaid ... ```).
static DIAGRAM_EXTRACTOR: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```(?:mermaid|plantuml)?\s*(.*)\s*```").unwrap());

/// Defines the types of diagrams the application can generate.
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum DiagramType {
    Component,
    Sequence,
}

/// The primary struct responsible for generating diagrams.
pub struct DiagramGenerator {
    client: OpenAIClient,
}

impl DiagramGenerator {
    /// Creates a new instance of the `DiagramGenerator`.
    pub fn new(client: OpenAIClient) -> Self {
        Self { client }
    }

    /// Generates an architectural diagram from the provided source code context.
    pub async fn generate_diagram(
        &self,
        project_context: &str,
        diagram_type: DiagramType,
        entry_function: Option<String>,
    ) -> Result<String> {
        info!("Generating '{:?}' diagram...", diagram_type);

        if project_context.is_empty() {
            return Err(Error::Config(
                "Project context is empty. No files to analyze.".to_string(),
            ));
        }

        let prompt = self.build_prompt(project_context, diagram_type, entry_function);

        let response_text = self.client.send_request(prompt).await?;
        info!("Received diagram response from AI.");

        let diagram_syntax = DIAGRAM_EXTRACTOR
            .captures(&response_text)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
            .ok_or_else(|| {
                Error::OpenAI(
                    "AI response did not contain a valid diagram code block.".to_string(),
                )
            })?;

        info!("Successfully extracted diagram syntax.");
        Ok(diagram_syntax)
    }

    /// Constructs a specialized prompt based on the desired diagram type.
    fn build_prompt(
        &self,
        project_context: &str,
        diagram_type: DiagramType,
        entry_function: Option<String>,
    ) -> String {
        let base_prompt = "You are an expert software architect with deep knowledge of Rust. Your task is to analyze the entire provided codebase and generate a diagram.";
        let output_format = "Your final output must ONLY be the Mermaid syntax, enclosed in a ```mermaid code block. Do not include any other text, explanations, or introductory sentences.";

        let specific_instructions: String = match diagram_type {
            DiagramType::Component => {
                // Corrected: Convert the string literal to an owned String
                // so that both match arms have the same type.
                "Generate a high-level component diagram showing the main modules, structs, and their primary relationships. Focus on the most significant architectural interactions, not every single function call. The output must be a Mermaid `graph TD`.".to_string()
            }
            DiagramType::Sequence => {
                let func_name = entry_function.as_deref().unwrap_or("[unspecified function]");
                format!("Generate a sequence diagram illustrating the flow of calls starting from the public function `{func_name}`. Trace the interactions between different modules and structs. The output must be a Mermaid `sequenceDiagram`.", func_name = func_name)
            }
        };

        format!(
            "{base_prompt}\n\nInstructions:\n1. Analyze the entire codebase provided below.\n2. {specific_instructions}\n3. {output_format}\n\nSTART OF CODEBASE CONTEXT\n---\n{project_context}\n---\nEND OF CODEBASE CONTEXT"
        )
    }
}
