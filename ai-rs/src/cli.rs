//! src/cli.rs
//!
//! This module provides the command-line interface for the chatbot.
//! It is responsible for:
//! 1. The main interactive loop that reads user input.
//! 2. Maintaining the conversation history.
//! 3. Displaying messages from the user and the assistant.
//! 4. Handling special commands like "exit".

use crate::error::Result;
// Corrected line: removed the unused `self` import.
use crate::openai::{Client, Message};
use colored::Colorize;
use std::io::{self, Write};

/// The main entry point for the command-line interface.
///
/// This function orchestrates the chat session. It initializes the OpenAI client,
/// sets up the initial "system" prompt, and enters an infinite loop to
/// process user input and display AI responses.
///
/// # Arguments
///
/// * `client` - The OpenAI `Client` used to communicate with the API.
///
/// # Returns
///
/// A `Result<()>` which will be `Ok(())` on successful exit, or an `Err`
/// if a critical I/O or API error occurs.
pub async fn run(client: Client) -> Result<()> {
    // Initialize the conversation history with a system message.
    // This sets the context and persona for the chatbot.
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: "You are a helpful assistant.".to_string(),
    }];

    println!("\n{}", "Chatbot session started.".blue().bold());
    println!("{}", "Type 'exit' to end the session.".blue());

    loop {
        // Prompt the user for input.
        print!("\n{}", "You: ".green().bold());
        // We must flush stdout to ensure the prompt is displayed before reading input.
        io::stdout().flush()?;

        let mut user_input = String::new();
        // Read the line from stdin. The `?` will propagate any I/O errors.
        io::stdin().read_line(&mut user_input)?;

        let user_input = user_input.trim();

        // Check for the exit command.
        if user_input.eq_ignore_ascii_case("exit") {
            println!("{}", "Ending session. Goodbye!".blue().bold());
            break;
        }

        // Add the user's message to the conversation history.
        messages.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });

        // Send the entire conversation history to the OpenAI API.
        let ai_response_content = client.chat_completion(&messages).await?;

        // Print the assistant's response.
        println!(
            "{}{}",
            "Assistant: ".yellow().bold(),
            ai_response_content.yellow()
        );

        // Add the assistant's response to the history for the next turn.
        messages.push(Message {
            role: "assistant".to_string(),
            content: ai_response_content,
        });
    }

    Ok(())
}
