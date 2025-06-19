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
