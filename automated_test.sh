#!/bin/bash
# Automated test script for P2P chat

echo "Starting Automated P2P Chat Test..."
echo "==================================="

# Kill any existing instances
pkill -f "rust-p2p-chat" 2>/dev/null
sleep 1

# Create named pipes for communication
PIPE1=$(mktemp -u)
PIPE2=$(mktemp -u)
mkfifo "$PIPE1"
mkfifo "$PIPE2"

# Start first peer with input from pipe
echo "Starting Peer 1 on port 8080..."
(echo "Hello from Peer 1!" > "$PIPE1"; sleep 2; echo "Test message 2" > "$PIPE1"; sleep 2; echo "/quit" > "$PIPE1") &
cargo run --bin rust-p2p-chat -- --port 8080 --connect 127.0.0.1:8081 --nickname Alice < "$PIPE1" > peer1.log 2>&1 &
PEER1_PID=$!

sleep 2

# Start second peer with input from pipe
echo "Starting Peer 2 on port 8081..."
(echo "Hello from Peer 2!" > "$PIPE2"; sleep 2; echo "Response from Peer 2" > "$PIPE2"; sleep 2; echo "/quit" > "$PIPE2") &
cargo run --bin rust-p2p-chat -- --port 8081 --connect 127.0.0.1:8080 --nickname Bob < "$PIPE2" > peer2.log 2>&1 &
PEER2_PID=$!

# Wait for messages to be exchanged
sleep 5

# Kill processes
kill $PEER1_PID $PEER2_PID 2>/dev/null

# Clean up pipes
rm -f "$PIPE1" "$PIPE2"

# Display results
echo ""
echo "Peer 1 Log:"
echo "-----------"
cat peer1.log | grep -E "(Hello|Test|Response|Connected|Peer:)" || echo "No messages found"
echo ""
echo "Peer 2 Log:"
echo "-----------"
cat peer2.log | grep -E "(Hello|Test|Response|Connected|Peer:)" || echo "No messages found"

# Check if connection was successful
if grep -q "Connected to peer" peer1.log && grep -q "Connected to peer" peer2.log; then
    echo ""
    echo "✓ Connection test PASSED"
else
    echo ""
    echo "✗ Connection test FAILED"
fi

# Clean up log files
rm -f peer1.log peer2.log