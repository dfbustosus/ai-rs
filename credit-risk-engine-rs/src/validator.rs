//! src/validator.rs
//!
//! This module is responsible for loading and validating applicant profile
//! data. It ensures that any data fed into the risk analysis engine conforms
//! to the predefined business rules.

use crate::error::Result;
use crate::models::ApplicantProfile;
use std::fs;
use std::path::Path;
use validator::Validate;

/// Loads an applicant profile from a JSON file and validates its contents.
///
/// # Arguments
///
/// * `file_path` - A reference to the path of the applicant's JSON profile.
///
/// # Returns
///
/// A `Result` containing the validated `ApplicantProfile` on success.
///
/// # Errors
///
/// Returns an `Error` if the file cannot be read, the JSON is malformed,
/// or the data fails any of the validation rules defined in `ApplicantProfile`.
pub fn load_and_validate_profile(file_path: &Path) -> Result<ApplicantProfile> {
    // 1. Read the file content.
    let file_content = fs::read_to_string(file_path)?;

    // 2. Deserialize the JSON into our struct.
    let profile: ApplicantProfile = serde_json::from_str(&file_content)?;

    // 3. Run the validation rules.
    // The `validate()` method comes from the `Validate` trait.
    // If validation fails, it will return an `Err` which is automatically
    // converted into our `Error::Validation` variant by the `?` operator.
    profile.validate()?;

    // 4. Return the valid profile.
    Ok(profile)
}
