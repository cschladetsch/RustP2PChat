#!/bin/bash
#
# Script Name: quick_test.sh
# Description: Quick interactive test of P2P chat functionality
# Usage: ./quick_test.sh [--port1 PORT] [--port2 PORT]
#
set -euo pipefail

# Default ports
PORT1=${1:-8080}
PORT2=${2:-8081}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Error handling function
error_exit() {
    echo -e "${RED}Error: $1${NC}" >&2
    cleanup
    exit 1
}

# Cleanup function
cleanup() {
    echo -e "\n${YELLOW}Cleaning up...${NC}"
    pkill -f "rust-p2p-chat" 2>/dev/null || true
    kill ${PEER1_PID:-} 2>/dev/null || true
}

# Validate port numbers
validate_port() {
    local port=$1
    if ! [[ "$port" =~ ^[0-9]+$ ]] || [ "$port" -lt 1024 ] || [ "$port" -gt 65535 ]; then
        error_exit "Invalid port: $port (must be 1024-65535)"
    fi
}

# Check if cargo is available
command -v cargo >/dev/null 2>&1 || error_exit "cargo is not installed or not in PATH"

echo -e "${GREEN}Starting P2P Chat Test...${NC}"
echo "========================="

# Validate ports
validate_port "$PORT1"
validate_port "$PORT2"

# Kill any existing instances
cleanup

# Set up signal handling
trap cleanup INT TERM EXIT

# Start first peer in background
echo -e "${YELLOW}Starting Peer 1 on port $PORT1...${NC}"
if ! cargo run --bin rust-p2p-chat -- --port "$PORT1" --connect "127.0.0.1:$PORT2" &; then
    error_exit "Failed to start Peer 1"
fi
PEER1_PID=$!

sleep 2

# Start second peer in foreground
echo -e "${YELLOW}Starting Peer 2 on port $PORT2...${NC}"
echo -e "${GREEN}You can now type messages in Peer 2!${NC}"
echo -e "${GREEN}To see Peer 1's messages, check the other terminal${NC}"
echo -e "${GREEN}Press Ctrl+C to stop both peers${NC}"

if ! cargo run --bin rust-p2p-chat -- --port "$PORT2" --connect "127.0.0.1:$PORT1"; then
    error_exit "Failed to start Peer 2"
fi

echo -e "${GREEN}Test completed successfully!${NC}"