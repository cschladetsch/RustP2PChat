#!/bin/bash

# Test script for Rust P2P Chat using tmux
# This script creates a tmux session with two panes showing two peers chatting

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Rust P2P Chat - Tmux Demo ===${NC}"
echo -e "${YELLOW}This script will create a tmux session with two peers chatting${NC}"
echo

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo -e "${YELLOW}tmux is not installed. Please install it first:${NC}"
    echo "  Ubuntu/Debian: sudo apt install tmux"
    echo "  macOS: brew install tmux"
    exit 1
fi

# Kill any existing demo session
tmux kill-session -t p2p-demo 2>/dev/null

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR" || exit 1

# Check if the binary exists
if [ ! -f "target/release/rust-p2p-chat" ]; then
    echo -e "${YELLOW}Building the project first...${NC}"
    cargo build --release
    if [ $? -ne 0 ]; then
        echo -e "${YELLOW}Build failed. Please fix build errors first.${NC}"
        exit 1
    fi
fi

echo -e "${GREEN}Starting tmux session with two P2P chat peers...${NC}"
echo -e "${BLUE}  - Alice will listen on port 8080${NC}"
echo -e "${BLUE}  - Bob will connect to Alice and listen on port 8081${NC}"
echo

# Create a new tmux session with the first peer
tmux new-session -d -s p2p-demo -n chat

# Split the window vertically
tmux split-window -h -t p2p-demo:chat

# Start the first peer (listener) in the left pane
tmux send-keys -t p2p-demo:chat.0 'echo "=== PEER 1 (Alice) - Listener ===" && echo' C-m
tmux send-keys -t p2p-demo:chat.0 './target/release/rust-p2p-chat --port 8080 --nickname Alice' C-m

# Wait a moment for the first peer to start listening
sleep 2

# Start the second peer (connector) in the right pane
# Bob will connect to Alice on 8080 and listen on 8081 (to avoid port conflict)
tmux send-keys -t p2p-demo:chat.1 'echo "=== PEER 2 (Bob) - Connector ===" && echo' C-m
tmux send-keys -t p2p-demo:chat.1 './target/release/rust-p2p-chat --connect localhost:8080 --port 8081 --nickname Bob' C-m

# Wait for connection to establish
sleep 2

# Send some test messages from Alice (left pane)
tmux send-keys -t p2p-demo:chat.0 'Hello Bob! This is Alice testing the P2P chat.' C-m
sleep 1

# Send a reply from Bob (right pane)
tmux send-keys -t p2p-demo:chat.1 'Hi Alice! Bob here. The encryption is working great!' C-m
sleep 1

# Send another message from Alice
tmux send-keys -t p2p-demo:chat.0 'Let me check our connection info...' C-m
sleep 0.5
tmux send-keys -t p2p-demo:chat.0 '/info' C-m
sleep 1

# Bob sends a command too
tmux send-keys -t p2p-demo:chat.1 '/help' C-m

# Add a visual separator in the terminal
echo
echo -e "${GREEN}✓ Tmux session 'p2p-demo' created successfully!${NC}"
echo
echo -e "${BLUE}Commands to interact with the session:${NC}"
echo "  - Attach to session:  tmux attach -t p2p-demo"
echo "  - Switch panes:       Ctrl+B then arrow keys"
echo "  - Detach:            Ctrl+B then D"
echo "  - Kill session:       tmux kill-session -t p2p-demo"
echo
echo -e "${YELLOW}The peers are now connected and chatting!${NC}"

# Automatically attach to the session
tmux attach -t p2p-demo