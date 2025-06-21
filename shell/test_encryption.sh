#!/bin/bash

echo "Starting encryption test..."

# Start server in background
cargo run -- --port 9999 > server.log 2>&1 &
SERVER_PID=$!

sleep 2

# Start client and send test message
echo -e "Hello encrypted world!\n/quit" | cargo run -- --connect 127.0.0.1:9999 > client.log 2>&1 &
CLIENT_PID=$!

# Wait a bit for messages to exchange
sleep 3

# Kill processes
kill $SERVER_PID $CLIENT_PID 2>/dev/null

echo "=== Server Log ==="
cat server.log | grep -E "(Listening|encryption|🔒|Peer:)"

echo -e "\n=== Client Log ==="
cat client.log | grep -E "(Connected|encryption|🔒|You:)"

# Cleanup
rm -f server.log client.log

echo -e "\nTest complete!"