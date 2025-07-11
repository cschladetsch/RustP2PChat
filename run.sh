#!/bin/bash

# Simple run script for rust-p2p-chat
# Usage: ./run.sh [args...]

# Build if binary doesn't exist
if [ ! -f "./target/debug/rust-p2p-chat" ]; then
    echo "Binary not found, building..."
    ./build.sh
fi

# Run with all provided arguments
./target/debug/rust-p2p-chat "$@"