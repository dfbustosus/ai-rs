# Explainable Credit Risk Assessment Engine

This Rust-based engine delivers **explainable, AI-powered credit risk assessments**. It leverages the OpenAI API to analyze applicant profiles and generate structured, auditable risk evaluations—making it ideal for scenarios where transparency, trust, and compliance are critical.

---

## Key Features
- **Explainable AI:** Generates not only a risk score, but also a detailed, step-by-step reasoning for each assessment.
- **Configurable Input Profiles:** Accepts applicant data in JSON format, allowing easy integration with existing systems.
- **Robust & Modular Architecture:** Clean separation of concerns across modules (logging, validation, API client, analysis logic) for clarity and maintainability.
- **Secure API Key Management:** Loads the OpenAI API key from a standard `.env` file—secrets are never hardcoded.
- **Modern Rust Practices:** Uses async, tracing for logging, and strong type safety throughout.

---

## Project Structure
The codebase is organized for clarity and extensibility. Each module has a single, well-defined responsibility.

```bash
credit-risk-engine-rs/
├── .env                   # Stores your OpenAI API key (ignored by git)
├── Cargo.toml             # Project dependencies and metadata
├── input_profiles/
│   └── sample_applicant.json # Example applicant profile
├── src/
│   ├── main.rs            # Entry point, CLI parsing, orchestration
│   ├── config.rs          # Loads API key and config
│   ├── error.rs           # Unified error handling
│   ├── logger.rs          # Logging setup (tracing)
│   ├── models.rs          # Data models: ApplicantProfile, RiskAssessment
│   ├── openai_client.rs   # Handles OpenAI API communication
│   ├── risk_analyzer.rs   # Core logic: builds prompts, parses results
│   └── validator.rs       # Input validation and loading
└── ...
```

---

## Getting Started
Follow these steps to set up and run the credit risk engine on your local machine.

### 1. Set Up Your API Key
You must have an OpenAI API key. Create a `.env` file in the project root and add your key:

```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
```

### 2. Prepare an Applicant Profile
Create a JSON file in `input_profiles/`. Example:

```json
{
  "applicantId": "APP-12345",
  "age": 35,
  "monthlyIncome": 5500,
  "monthlyDebt": 1200,
  "employmentStatus": "Employed",
  "yearsInCurrentJob": 5,
  "creditScore": 680,
  "loanAmount": 25000,
  "loanPurpose": "Debt Consolidation",
  "hasPreviousDefaults": false,
  "additionalNotes": "Applicant has a stable employment history with a reputable company. They are looking to consolidate two high-interest credit card debts into a single, lower-interest loan."
}
```

### 3. Build and Run the Engine
Use Cargo to run the engine, passing the path to your applicant profile:

```bash
cargo run -- input_profiles/sample_applicant.json
```

The engine will output a detailed, explainable risk assessment in JSON format.

---

## Example Output
```json
{
  "riskScore": 5,
  "recommendation": "MANUAL_REVIEW",
  "positiveFactors": [
    "Stable employment status with 5 years in current job indicates reliability.",
    "Income sufficiently exceeds debt obligations, implying a manageable debt-to-income ratio.",
    "No history of defaults increases the likelihood of repayment."
  ],
  "negativeFactors": [
    "Average credit score of 680, which may not meet the most competitive rates.",
    "Significant loan amount relative to monthly income could imply a high repayment burden."
  ],
  "detailedRationale": "The applicant shows a reasonable financial profile with a balanced income-to-debt ratio, a stable employment history, and a credit score that falls within an acceptable range for lending. The purpose of the loan is practical and aimed at improving financial efficiency through debt consolidation. While the credit score is not exceptionally high, the applicant's consistent employment and lack of previous defaults strengthen the case for loan approval. However, the average credit score suggests room for manual review to ensure all potential risks are mitigated."
}
```

---

## License
MIT

---

## Contributing
Contributions are welcome! Please open issues or submit pull requests to help improve the engine.
