#!/bin/bash

echo "=== Rust P2P Chat Demo ==="
echo
echo "This demo shows the new features added to the P2P chat application."
echo

# Build the project
echo "Building the project..."
cargo build --release

echo
echo "Features implemented:"
echo "1. Custom error types for better error handling"
echo "2. Message protocol with different types (text, files, commands)"
echo "3. File transfer capability"
echo "4. Command system (/help, /quit, /send)"
echo "5. Configuration file support"
echo "6. CLI arguments with clap"
echo "7. Colorful terminal output"
echo "8. Message buffering up to 8KB"
echo "9. Heartbeat mechanism"
echo "10. Prepared for TLS encryption (can be enabled)"
echo

echo "To test the application:"
echo "1. In terminal 1: ./target/release/rust-p2p-chat --port 8080"
echo "2. In terminal 2: ./target/release/rust-p2p-chat --connect 127.0.0.1:8080"
echo
echo "Available commands in chat:"
echo "  /help      - Show help"
echo "  /quit      - Exit chat"
echo "  /send file - Send a file"
echo
echo "You can also:"
echo "- Set a nickname: --nickname Alice"
echo "- Enable debug logging: --debug"
echo "- Generate config: rust-p2p-chat config"