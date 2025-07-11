#!/bin/bash

# Simple build script for rust-p2p-chat
# Usage: ./build.sh [--release]

if [ "$1" == "--release" ]; then
    echo "Building in release mode..."
    cargo build --release
else
    echo "Building in debug mode..."
    cargo build
fi