#!/bin/bash
# Test P2P chat using tmux split panes

# Kill any existing tmux session named "p2p-chat"
tmux kill-session -t p2p-chat 2>/dev/null

# Create new tmux session with horizontal split
tmux new-session -d -s p2p-chat
tmux split-window -h -t p2p-chat

# Run peer 1 in left pane
tmux send-keys -t p2p-chat:0.0 'cargo run -- 8080 127.0.0.1:8081' C-m

# Wait a moment
sleep 1

# Run peer 2 in right pane
tmux send-keys -t p2p-chat:0.1 'cargo run -- 8081 127.0.0.1:8080' C-m

# Attach to the session
tmux attach-session -t p2p-chat