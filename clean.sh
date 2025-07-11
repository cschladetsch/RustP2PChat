#!/bin/bash

# Clean script for rust-p2p-chat
# Removes build artifacts and temporary files

echo "Cleaning build artifacts..."

# Clean Cargo build artifacts
cargo clean

# Remove any temporary files
find . -name "*.swp" -o -name "*.swo" -o -name "*~" -o -name "*.tmp" -o -name "*.bak" | xargs rm -f 2>/dev/null

# Remove .DS_Store files (macOS)
find . -name ".DS_Store" | xargs rm -f 2>/dev/null

echo "Clean complete!"