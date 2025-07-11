# Multi-stage Dockerfile for Rust P2P Chat
# This creates a minimal final image with just the binary

# Stage 1: Build stage
FROM rust:1.75-slim AS builder

# Install required dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/app

# Copy Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application
RUN cargo build --release

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN useradd -m -u 1001 -s /bin/bash rustchat

# Copy the binary from the build stage
COPY --from=builder /usr/src/app/target/release/rust-p2p-chat /usr/local/bin/rust-p2p-chat

# Create directories for config and data
RUN mkdir -p /home/rustchat/.config/rustchat/p2p-chat && \
    chown -R rustchat:rustchat /home/rustchat

# Switch to non-root user
USER rustchat

# Set the working directory
WORKDIR /home/rustchat

# Expose the default port
EXPOSE 8080

# Set default environment variables
ENV RUST_LOG=info

# Default command (can be overridden)
ENTRYPOINT ["rust-p2p-chat"]
CMD ["--help"]