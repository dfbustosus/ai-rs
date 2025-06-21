#  Conversation Distillation Engine
A command-line tool built in Rust that performs advanced analysis on conversation transcripts. 

It leverages the OpenAI API to move beyond simple summarization and into the realm of purpose-driven content synthesis.

The engine takes a structured conversation transcript and "distills" it into a new format based on a user-selected tone profile. This allows it to transform a single conversation into multiple, distinct outputs tailored for different audiences, such as a formal executive briefing, a technical developer debrief, or a friendly customer-facing summary.

# Key Features
- **Purpose-Driven Synthesis:** Instead of just shortening text, the engine completely reframes it according to predefined goals, demonstrating a sophisticated use of AI for content generation.
- **Configuration-Driven Tone Profiles:** The "personas" the AI adopts (e.g., business analyst, senior engineer) are defined in an external config/tone_profiles.json file. This makes the engine a highly flexible and adaptable platform that can be customized for any use case without code changes.
- **Explainable AI (XAI) Ready:** The underlying architecture, which separates system prompts from user prompts, can be easily extended to incorporate Chain of Thought (CoT) or other explainability patterns.
- **Robust & Modular Architecture:** Engineered with a strict separation of concerns. Each component (logging, configuration, parsing, API client, and core logic) is isolated in its own module for maximum clarity, testability, and maintainability.
- **Secure and Modern:** Implements best practices, including secure API key management via .env files and a pure-Rust TLS implementation (rustls) for a more reliable and secure build process.

# Project Structure
The codebase is organized to be clear, scalable, and easy to maintain. Each module has a single, well-defined responsibility.

```bash
conversation-distiller-rs/
├── .env
├── .gitignore
├── Cargo.toml
└── config/
|   └── tone_profiles.json # Defines the various summary formats and tones.
└── input/
|   └── sample_conversation.json # An example input conversation transcript.
└── src/
    ├── main.rs                 # Entry point, CLI parsing, and orchestration.
    ├── error.rs                # Unified error handling module.
    ├── config.rs               # Manages loading and validation of tone profiles.
    ├── conversation_parser.rs  # Handles loading and parsing of input transcripts.
    ├── openai_client.rs        # Dedicated client for OpenAI API communication.
    └── distiller_engine.rs     # The core logic: assembles prompts and calls the AI.

```
# Getting Started
Follow these steps to set up and run the distillation engine on your local machine.

1. Create Configuration and Input Directories

In the root of the project, create the necessary directories:

```bash
mkdir config
mkdir input
```

2. Define Tone Profiles

Inside the config directory, create a file named tone_profiles.json with the following content. This file defines the different summary formats.

```json
{
  "profiles": [
    {
      "name": "executive_briefing",
      "description": "A formal, concise summary for a manager, focusing on the core issue and resolution.",
      "system_prompt": "You are a senior business analyst. Your task is to distill the following conversation into a formal, one-paragraph executive briefing. Focus on the customer's core problem, the steps taken to resolve it, and the final outcome. Omit all pleasantries and technical jargon."
    },
    {
      "name": "developer_debrief",
      "description": "A technical, bulleted list for an engineering team, detailing issues and action items.",
      "system_prompt": "You are a senior software engineer. Your task is to analyze the following conversation and produce a technical debrief for the development team. Create a bulleted list outlining the specific technical issues encountered, the root cause if identified, and any action items required. Be precise and focus only on the technical details."
    },
    {
      "name": "customer_facing_summary",
      "description": "A friendly, non-technical summary to be sent to the customer.",
      "system_prompt": "You are a customer success advocate. Your task is to write a friendly, polite, and non-technical summary of the following conversation for the customer. Confirm the issue they reported and briefly explain the resolution in simple terms. Ensure the tone is reassuring and positive."
    }
  ]
}
```

3. Provide a Sample Conversation

Inside the input directory, create a file named `sample_conversation.json`.

```json
{
  "conversation": [
    {
      "speaker": "User",
      "text": "Hi, I'm having trouble with my account. I can't seem to log in. It keeps saying 'Authentication Error'."
    },
    {
      "speaker": "Bot",
      "text": "Hello! I'm sorry to hear you're having trouble. Can you please confirm the email address you are using to log in?"
    },
    {
      "speaker": "User",
      "text": "Yes, it's user@example.com."
    },
    {
      "speaker": "Bot",
      "text": "Thank you. I see that email address in our system. It looks like there was a temporary issue with our authentication service that was preventing some logins. The engineering team has just deployed a fix. Could you please try logging in again now?"
    },
    {
      "speaker": "User",
      "text": "Okay, let me try... It works! I'm in. Thanks for your help."
    },
    {
      "speaker": "Bot",
      "text": "That's wonderful news! I'm glad we could get that sorted out for you. Is there anything else I can assist you with today?"
    },
    {
      "speaker": "User",
      "text": "No, that's all. Thanks again!"
    }
  ]
}
```

4. Set Up Your API Key

Create a file named `.env` in the root of the project and add your OpenAI API key:

```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
```

5. Build and Run the Engine

You can now run the tool using cargo run. You must specify the input file and the desired profile name. The `--` separator is crucial to distinguish arguments for Cargo from arguments for your application.

Results
Here are some real examples of running the engine with the provided sample data.

Example 1: Generating an Executive Briefing
This command will process the sample conversation and apply the executive_briefing tone profile.

Command:

```bash
cargo run -- --input-file input/sample_conversation.json --profile-name executive_briefing
```

# Expected Output:

```bash
--- Distilled Summary: executive_briefing ---
A customer encountered an 'Authentication Error' when attempting to log into their account. The issue was traced to a temporary fault in the authentication service. The engineering team deployed a fix, after which the customer confirmed they were able to log in successfully, fully resolving the incident.
--- End of Summary ---
```

Example 2: Generating a Developer Debrief
This command uses the same conversation but applies the developer_debrief profile, resulting in a completely different, technically-focused output.

Command:

```bash
cargo run -- --input-file input/sample_conversation.json --profile-name developer_debrief
```

# Expected Output:

```bash
--- Distilled Summary: developer_debrief ---
- **Issue:** User 'user@example.com' reported login failures presenting as 'Authentication Error'.
- **Root Cause:** A temporary, unspecified issue within the authentication service.
- **Resolution:** A fix was deployed by the engineering team. No specific details on the fix were provided in the transcript.
- **Action Items:** None. The issue is confirmed resolved by the user. Monitor service for any recurring authentication problems.
--- End of Summary ---
```
