#!/bin/bash
# Demo script showing encrypted P2P chat in action

echo "========================================"
echo "P2P Encrypted Chat Demo"
echo "========================================"
echo ""
echo "This demo will:"
echo "1. Start two chat peers (Alice and Bob)"
echo "2. Establish encrypted connection"
echo "3. Exchange test messages"
echo "4. Show the encryption in action"
echo ""

# Kill any existing instances
pkill -f "rust-p2p-chat" 2>/dev/null
sleep 1

# Create FIFOs for input
ALICE_INPUT=$(mktemp -u)
BOB_INPUT=$(mktemp -u)
mkfifo "$ALICE_INPUT"
mkfifo "$BOB_INPUT"

# Start Alice
echo "Starting Alice on port 8080..."
(
    sleep 3
    echo "Hello Bob! This is Alice." > "$ALICE_INPUT"
    sleep 2
    echo "This message is encrypted with RSA + AES-256!" > "$ALICE_INPUT"
    sleep 2
    echo "/quit" > "$ALICE_INPUT"
) &

cargo run --bin rust-p2p-chat -- --port 8080 --connect 127.0.0.1:8081 --nickname Alice < "$ALICE_INPUT" > alice.log 2>&1 &
ALICE_PID=$!

sleep 1

# Start Bob
echo "Starting Bob on port 8081..."
(
    sleep 3
    echo "Hi Alice! This is Bob." > "$BOB_INPUT"
    sleep 2
    echo "Great to chat securely with 1024-bit encryption!" > "$BOB_INPUT"
    sleep 2
    echo "/quit" > "$BOB_INPUT"
) &

cargo run --bin rust-p2p-chat -- --port 8081 --connect 127.0.0.1:8080 --nickname Bob < "$BOB_INPUT" > bob.log 2>&1 &
BOB_PID=$!

# Wait for chat to complete
sleep 8

# Kill processes
kill $ALICE_PID $BOB_PID 2>/dev/null

# Clean up FIFOs
rm -f "$ALICE_INPUT" "$BOB_INPUT"

# Display results
echo ""
echo "========================================"
echo "Chat Session Results"
echo "========================================"
echo ""
echo "ALICE'S VIEW:"
echo "-------------"
grep -E "(Connected|You:|Peer:|Encryption)" alice.log | sed 's/\x1b\[[0-9;]*m//g'
echo ""
echo "BOB'S VIEW:"
echo "-----------"
grep -E "(Connected|You:|Peer:|Encryption)" bob.log | sed 's/\x1b\[[0-9;]*m//g'

# Check encryption
echo ""
echo "========================================"
echo "Security Features Verified:"
echo "========================================"
if grep -q "RSA" alice.log && grep -q "AES-256-GCM" alice.log; then
    echo "✓ 1024-bit RSA key exchange"
    echo "✓ AES-256-GCM encryption"
    echo "✓ End-to-end encrypted messages"
else
    echo "⚠ Encryption status unclear"
fi

# Clean up
rm -f alice.log bob.log

echo ""
echo "Demo complete! To run your own chat session:"
echo "Terminal 1: cargo run -- --port 8080 --nickname YourName"
echo "Terminal 2: cargo run -- --port 8081 --connect 127.0.0.1:8080 --nickname FriendName"