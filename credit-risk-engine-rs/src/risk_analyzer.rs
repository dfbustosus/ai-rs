//! src/risk_analyzer.rs
//!
//! This module contains the core logic for the risk assessment engine.
//! It is responsible for constructing the detailed prompt for the OpenAI API,
//! sending the request, and parsing the structured JSON response.

use crate::error::Result;
use crate::models::{ApplicantProfile, RiskAssessment};
use crate::openai_client::OpenAIClient;
use tracing::{info, instrument};

/// The main engine responsible for analyzing credit risk.
pub struct RiskAnalyzer {
    client: OpenAIClient,
}

impl RiskAnalyzer {
    /// Creates a new instance of the `RiskAnalyzer`.
    pub fn new(client: OpenAIClient) -> Self {
        Self { client }
    }

    /// Performs a comprehensive risk assessment for a given applicant profile.
    ///
    /// This function orchestrates the entire analysis process:
    /// 1. Serializes the applicant's profile into a JSON string.
    /// 2. Constructs a sophisticated, multi-part system prompt.
    /// 3. Sends the request to the OpenAI API.
    /// 4. Parses the returned JSON string into a `RiskAssessment` struct.
    ///
    /// # Arguments
    ///
    /// * `profile` - A validated `ApplicantProfile`.
    ///
    /// # Returns
    ///
    /// A `Result` containing the structured `RiskAssessment`.
    #[instrument(skip(self, profile))]
    pub async fn assess(&self, profile: &ApplicantProfile) -> Result<RiskAssessment> {
        info!(applicant_id = %profile.applicant_id, "Starting risk assessment.");

        let profile_json = serde_json::to_string_pretty(profile)?;
        let system_prompt = self.build_system_prompt();
        
        let response_text = self.client.send_request(&system_prompt, &profile_json).await?;

        // Parse the JSON string response from the AI into our target struct.
        let assessment: RiskAssessment = serde_json::from_str(&response_text)?;
        info!(applicant_id = %profile.applicant_id, "Successfully completed and parsed risk assessment.");

        Ok(assessment)
    }

    /// Constructs the detailed system prompt that guides the AI's analysis.
    ///
    /// This prompt is critical. It defines the AI's persona, its task, the
    /// factors it must consider, and the exact JSON schema it must use for
    /// its response. This is the core of our "prompt engineering".
    fn build_system_prompt(&self) -> String {
        let output_schema = serde_json::json!({
            "riskScore": "A number from 1 (lowest risk) to 10 (highest risk).",
            "recommendation": "Enum, one of: 'APPROVE', 'DENY', 'MANUAL_REVIEW'.",
            "positiveFactors": ["A list of strings explaining strengths."],
            "negativeFactors": ["A list of strings explaining weaknesses."],
            "detailedRationale": "A paragraph explaining the final recommendation."
        });

        format!(
            "You are an expert credit risk analyst for a financial institution. Your task is to perform a detailed risk assessment of the loan applicant whose data is provided below in JSON format.

            Analyze all aspects of the applicant's profile, including their income-to-debt ratio, credit score, employment stability, and the purpose of the loan.

            Your final output must be a single, valid JSON object that strictly adheres to the following schema:
            ```json
            {}
            ```

            Do not include any text, explanations, or markdown formatting outside of this single JSON object.",
            serde_json::to_string_pretty(&output_schema).unwrap()
        )
    }
}
