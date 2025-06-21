#!/bin/bash
# Quick test script for P2P chat

echo "Starting P2P Chat Test..."
echo "========================="

# Kill any existing instances
pkill -f "rust-p2p-chat" 2>/dev/null

# Start first peer in background
echo "Starting Peer 1 on port 8080..."
cargo run --bin rust-p2p-chat -- --port 8080 --connect 127.0.0.1:8081 &
PEER1_PID=$!

sleep 2

# Start second peer in foreground
echo "Starting Peer 2 on port 8081..."
echo "You can now type messages in Peer 2!"
echo "To see Peer 1's messages, check the other terminal"
echo "Press Ctrl+C to stop both peers"

# Trap to clean up both processes
trap "kill $PEER1_PID 2>/dev/null; exit" INT TERM

cargo run --bin rust-p2p-chat -- --port 8081 --connect 127.0.0.1:8080

# Clean up
kill $PEER1_PID 2>/dev/null