# Explainable Sentiment Analysis Engine
This tool built in Rust, that performs advanced sentiment analysis using the OpenAI API. Its core feature is explainability; it does not just provide a classification label but also generates a detailed "Chain of Thought" reasoning process to justify its conclusion.

This engine is designed for scenarios where transparency and auditability are critical. It uses a predefined, configurable set of rules, making it adaptable to various business and research contexts.

# Key Features
1. **Explainable AI (XAI):** Leverages Chain of Thought (CoT) prompting to make the AI's reasoning process transparent and auditable.

2. **Configuration-Driven Rules:** Sentiment categories are not hardcoded. They are loaded from an external `sentiment_labels.json` file, allowing for easy customization without code changes.

3. **Robust & Modular Architecture:** Built with a strict separation of concerns. Each component (logging, configuration, API client, analysis logic) is isolated in its own module for maximum clarity, testability, and maintainability.

4. **Structured Logging:** Implements the tracing framework for professional, leveled logging, crucial for debugging and monitoring.

5. **Secure API Key Management:** Loads the OpenAI API key from a standard `.env` file to ensure secrets are never committed to source control.

5. **Modern Rust Practices:** Uses a pure-Rust TLS implementation (rustls) to avoid C-library dependencies, ensuring a more reliable and secure build process.

# Project Structure
The codebase is organized to be clear, scalable, and easy to maintain.

```bash
sentiment-engine-rs/
├── .env
├── .gitignore
├── Cargo.toml
└── config/
|   └── sentiment_labels.json # Defines the predetermined sentiment categories.
└── src/
    ├── main.rs               # Entry point, CLI parsing, and orchestration.
    ├── error.rs              # Unified, robust error handling.
    ├── config.rs             # Manages application configuration.
    ├── constants.rs          # Defines global constants like model names.
    ├── logger.rs             # A dedicated logging setup module.
    ├── openai_client.rs      # Handles all communication with the OpenAI API.
    └── sentiment_analyzer.rs # Core logic: loads labels, builds CoT prompts, parses results.
```

# Setup and Usage
Follow these steps to set up and run the sentiment engine.

1. Create the Configuration Directory

In the root of the project, create a directory named config:
```bash
mkdir config
```

2. Define Sentiment Labels

Inside the config directory, create a file named `sentiment_labels.json` with the following content. You can modify this to fit your specific needs.
```json
{
  "labels": [
    {
      "name": "Positive",
      "description": "The text expresses a clearly positive, happy, or favorable sentiment."
    },
    {
      "name": "Negative",
      "description": "The text expresses a clearly negative, unhappy, or unfavorable sentiment."
    },
    {
      "name": "Neutral",
      "description": "The text is objective, factual, or does not express a strong emotion."
    },
    {
      "name": "Inquisitive",
      "description": "The text is primarily asking a question or expressing curiosity."
    },
    {
      "name": "Urgent",
      "description": "The text conveys a sense of urgency, requiring immediate attention or action."
    }
  ]
}
```

3. Set Up Your API Key

Create a file named `.env` in the root of the project and add your OpenAI API key:

```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
```

4. Run the Application

Use cargo run to execute the program. You must provide the text you want to analyze as a command-line argument, enclosed in quotes. The -- separator is crucial to distinguish arguments for Cargo from arguments for your application.

Example Command:

```bash
cargo run -- "This new feature is absolutely fantastic and works better than I expected!"
```

The tool will then output the detailed "Chain of Thought" reasoning, followed by the final sentiment classification.

# Results
## Example 1
```bash
cargo run -- "I love this new product, it works perfectly"


    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/sentiment-engine-rs 'I love this new product, it works perfectly'`
2025-06-20T00:41:34.496147Z  INFO sentiment_engine_rs: Received text for analysis. text=I love this new product, it works perfectly
2025-06-20T00:41:34.496301Z  INFO sentiment_engine_rs: Successfully loaded 5 sentiment labels.
2025-06-20T00:41:34.496772Z  INFO sentiment_engine_rs::sentiment_analyzer: Starting sentiment analysis.
2025-06-20T00:41:34.496810Z  INFO sentiment_engine_rs::sentiment_analyzer: Constructed analysis prompt. prompt=You are an expert sentiment analysis engine. Your task is to analyze the provided text and classify it according to one of the following predefined sentiment labels. You must provide your reasoning process and then the final classification in a specific JSON format.

            Sentiment Labels:
            - "Positive": The text expresses a clearly positive, happy, or favorable sentiment.
- "Negative": The text expresses a clearly negative, unhappy, or unfavorable sentiment.
- "Neutral": The text is objective, factual, or does not express a strong emotion.
- "Inquisitive": The text is primarily asking a question or expressing curiosity.
- "Urgent": The text conveys a sense of urgency, requiring immediate attention or action.

            Follow these steps precisely:
            1.  **Chain of Thought**: First, write a step-by-step reasoning process explaining your analysis. Consider the explicit words, the context, and the likely intent of the author. This reasoning must be detailed.
            2.  **Sentiment Classification**: After your reasoning, choose the single best sentiment label from the provided list that accurately describes the text.

            Your final output must be a single, valid JSON object with two keys: "chainOfThought" and "sentiment". Do not include any other text or explanations outside of the JSON object.

            Text to Analyze:
            """
            I love this new product, it works perfectly
            """
2025-06-20T00:41:36.750839Z  INFO sentiment_engine_rs::sentiment_analyzer: Received response from API. response=```json
{
    "chainOfThought": "The text clearly expresses a positive sentiment. The use of the word 'love' indicates a strong, favorable emotion towards the subject, which in this case is a 'new product'. Additionally, the phrase 'it works perfectly' further reinforces a positive evaluation. There are no elements of negativity, urgency, or inquiry present in the text. The intent of the author appears to be to convey satisfaction and approval of the product.",
    "sentiment": "Positive"
}


Sentiment Analysis Complete

Reasoning (Chain of Thought):
The text clearly expresses a positive sentiment. The use of the word 'love' indicates a strong, favorable emotion towards the subject, which in this case is a 'new product'. Additionally, the phrase 'it works perfectly' further reinforces a positive evaluation. There are no elements of negativity, urgency, or inquiry present in the text. The intent of the author appears to be to convey satisfaction and approval of the product.

Final Classification:
Positive
```
## Example 2
```bash
cargo run -- "The service was terrible and I am very disappointed."
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/sentiment-engine-rs 'The service was terrible and I am very disappointed.'`
2025-06-20T00:41:49.595538Z  INFO sentiment_engine_rs: Received text for analysis. text=The service was terrible and I am very disappointed.
2025-06-20T00:41:49.595730Z  INFO sentiment_engine_rs: Successfully loaded 5 sentiment labels.
2025-06-20T00:41:49.596285Z  INFO sentiment_engine_rs::sentiment_analyzer: Starting sentiment analysis.
2025-06-20T00:41:49.596328Z  INFO sentiment_engine_rs::sentiment_analyzer: Constructed analysis prompt. prompt=You are an expert sentiment analysis engine. Your task is to analyze the provided text and classify it according to one of the following predefined sentiment labels. You must provide your reasoning process and then the final classification in a specific JSON format.

            Sentiment Labels:
            - "Positive": The text expresses a clearly positive, happy, or favorable sentiment.
- "Negative": The text expresses a clearly negative, unhappy, or unfavorable sentiment.
- "Neutral": The text is objective, factual, or does not express a strong emotion.
- "Inquisitive": The text is primarily asking a question or expressing curiosity.
- "Urgent": The text conveys a sense of urgency, requiring immediate attention or action.

            Follow these steps precisely:
            1.  **Chain of Thought**: First, write a step-by-step reasoning process explaining your analysis. Consider the explicit words, the context, and the likely intent of the author. This reasoning must be detailed.
            2.  **Sentiment Classification**: After your reasoning, choose the single best sentiment label from the provided list that accurately describes the text.

            Your final output must be a single, valid JSON object with two keys: "chainOfThought" and "sentiment". Do not include any other text or explanations outside of the JSON object.

            Text to Analyze:
            """
            The service was terrible and I am very disappointed.
            """
2025-06-20T00:41:53.081797Z  INFO sentiment_engine_rs::sentiment_analyzer: Received response from API. response=```json
{
  "chainOfThought": "The text explicitly uses the word 'terrible' to describe the service, which is a strong negative descriptor. Additionally, the use of 'very disappointed' further emphasizes the speaker's dissatisfaction and negative emotional response to the experience. The context reveals that the author is expressing an unfavorable opinion with emotional discontent likely due to personal experience. The combination of these terms and the conveyed emotion aligns with a negative sentiment, as the overall tone and context are aligned with expressing unhappiness and disappointment.",
  "sentiment": "Negative"
}
```

Sentiment Analysis Complete

Reasoning (Chain of Thought):
The text explicitly uses the word 'terrible' to describe the service, which is a strong negative descriptor. Additionally, the use of 'very disappointed' further emphasizes the speaker's dissatisfaction and negative emotional response to the experience. The context reveals that the author is expressing an unfavorable opinion with emotional discontent likely due to personal experience. The combination of these terms and the conveyed emotion aligns with a negative sentiment, as the overall tone and context are aligned with expressing unhappiness and disappointment.

Final Classification:
Negative
```