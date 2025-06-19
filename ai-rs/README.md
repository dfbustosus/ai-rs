# Rust OpenAI Chatbot
This is a robust command-line chatbot built in Rust that connects to the OpenAI API. You can have a continuous conversation with it right from your terminal.

The main goal here was to build a solid foundation, focusing on good practices like modular design and security, rather than just hacking a script together.

# Key Features
1. **Interactive Chat**: Have a back-and-forth conversation. The bot remembers the context of the current session.

2. **Secure API Key Storage**: Your OpenAI key is loaded from a `.env` file, so it never gets checked into version control.

3. **Modular Codebase**: The project is split into logical parts (API client, CLI, configuration, error handling) to make it easy to understand, test, and expand.

4. **Robust Error Handling**: Uses a custom error type to handle things gracefully, whether it's a missing API key or a network hiccup.

5. **Clean CLI**: The terminal output is colored for better readability.

# Project Structure
I set it up this way to keep things organized. Each file has one job.
```
ai-rs/
├── .env           # Where you put your secret API key (ignored by git)
├── .gitignore     # Tells git to ignore .env and build files
├── Cargo.toml     # Manages project dependencies
└── src/
    ├── main.rs    # The entry point that wires everything together
    ├── error.rs   # Defines our custom Result/Error types
    ├── config.rs  # Logic for loading the API key
    ├── openai.rs  # All the code for talking to the OpenAI API
    └── cli.rs     # Handles the command-line user interface and chat loop
```

# Getting Started
Here's how to get it running on your machine.

1. Set up your API Key

Create a file named `.env` in the root of the project directory. Add your OpenAI API key to it like this:

```bash
OPENAI_API_KEY="your-secret-api-key-goes-here"
```

2. Build and Run

Once the `.env` file is in place, you can run the application using Cargo:

```bash
cargo run
```

The first time you run it, Cargo will download and compile all the necessary libraries. After that, it will launch the chatbot, and you can start talking! Type exit and press Enter to end the session.


# Containerization with Docker
For reproducible builds and easy deployment, you can also run this application inside a Docker container.

1. Create a Dockerfile

Create a file named `Dockerfile` in the root of your project and add the following content. This uses a multi-stage build to create a small, optimized final image.

```dockerfile
# =================================================================================================
# A Simple & Reliable Dockerfile for Rust Applications
#
# This Dockerfile uses a straightforward, multi-stage build process that is both
# reliable and easy to understand. It avoids environment conflicts by using a
# consistent base (Debian) for both the build and final runtime stages.
# =================================================================================================

# =================================================================================================
# STAGE 1: The Builder
#
# This stage uses a full Rust development environment (based on Debian) to compile
# the application into a standard, release-optimized Linux executable.
# =================================================================================================
FROM rust:1.87.0 AS builder

# Set up the working directory.
WORKDIR /usr/src/app

# Copy the entire project context into the builder.
# A .dockerignore file should be used to prevent copying the `target` directory.
COPY . .

# Build the application in release mode. This creates a dynamically-linked
# executable for the default Linux target (x86_64-unknown-linux-gnu).
# Because we are using the `rustls` feature in Cargo.toml, this binary will not
# depend on the system's OpenSSL library, which avoids many common issues.
RUN cargo build --release


# =================================================================================================
# STAGE 2: The Runner
#
# This stage creates the final, lean image for our application. We use a minimal
# Debian base image to ensure it is compatible with the executable from the builder.
# =================================================================================================
FROM debian:bookworm-slim

# Step 1: Install the root SSL certificates.
# The 'slim' image is minimal and does not include these by default. They are
# essential for our application to make secure HTTPS requests, as the `rustls`
# library relies on the operating system's trust store. We also clean up the
# apt cache to keep the final image size as small as possible.
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Step 2: Copy the compiled binary from the builder stage into the final image.
# We place it in a standard location for executables.
COPY --from=builder /usr/src/app/target/release/ai-rs /usr/local/bin/ai-rs

# Step 3: Set the entrypoint.
# This command will be executed when the container starts.
ENTRYPOINT ["/usr/local/bin/ai-rs"]
```

2. Create a `.dockerignore` file

To keep the build context small and prevent secrets from leaking into the image, create a `.dockerignore` file with the following:

```bash
.git
.gitignore
target/
.env
```

3. Build and Run the Container

With Docker running, build the image from your terminal:

```bash
docker build -t ai-rs .
```
Now, run the container. The `-it` flag gives you an interactive terminal, and --env-file securely passes your API key from the .env file to the container.

```bash
docker run -it --rm --env-file .env ai-rs
```