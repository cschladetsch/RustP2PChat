# Multi-stage Dockerfile for Rust P2P Chat
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -m -u 1001 -s /bin/bash rustchat

# Copy the binary
COPY --from=builder /app/target/release/rust-p2p-chat /usr/local/bin/rust-p2p-chat

# Make it executable
RUN chmod +x /usr/local/bin/rust-p2p-chat

# Create directories for the app
RUN mkdir -p /home/rustchat/.config/rustchat/p2p-chat && \
    chown -R rustchat:rustchat /home/rustchat

# Switch to non-root user
USER rustchat
WORKDIR /home/rustchat

# Expose default port
EXPOSE 8080

# Set entrypoint
ENTRYPOINT ["rust-p2p-chat"]