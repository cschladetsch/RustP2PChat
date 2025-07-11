#!/bin/bash

# Docker Hub P2P Chat Demo with tmux
# This script pulls your image from Docker Hub and runs two peers in tmux

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
DOCKER_IMAGE="cschladetsch/rust-p2p-chat:latest"
SESSION_NAME="docker-chat"
NETWORK_NAME="p2p-chat-network"

echo -e "${BLUE}=== Docker Hub P2P Chat - Tmux Demo ===${NC}"
echo -e "${YELLOW}Using Docker image: ${DOCKER_IMAGE}${NC}"
echo

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo -e "${RED}tmux is not installed. Please install it first:${NC}"
    echo "  Ubuntu/Debian: sudo apt install tmux"
    echo "  macOS: brew install tmux"
    exit 1
fi

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

# Clean up any existing session
tmux kill-session -t $SESSION_NAME 2>/dev/null
docker network rm $NETWORK_NAME 2>/dev/null

echo -e "${YELLOW}Pulling image from Docker Hub...${NC}"
docker pull $DOCKER_IMAGE || {
    echo -e "${RED}Failed to pull image. Make sure the image exists on Docker Hub.${NC}"
    exit 1
}

echo -e "${GREEN}✓ Image pulled successfully${NC}"
echo

# Create Docker network for containers to communicate
echo -e "${YELLOW}Creating Docker network...${NC}"
docker network create $NETWORK_NAME

# Create tmux session with two panes
echo -e "${YELLOW}Starting tmux session with two P2P chat peers...${NC}"
tmux new-session -d -s $SESSION_NAME -n chat

# Split window vertically
tmux split-window -h -t $SESSION_NAME:chat

# Configure panes
tmux select-pane -t $SESSION_NAME:chat.0 -T "Alice (Server)"
tmux select-pane -t $SESSION_NAME:chat.1 -T "Bob (Client)"

# Start first peer (Alice) in left pane
tmux send-keys -t $SESSION_NAME:chat.0 "echo -e '${GREEN}=== PEER 1 (Alice) - Server ===${NC}' && echo" C-m
tmux send-keys -t $SESSION_NAME:chat.0 "docker run -it --rm \\
  --name p2p-alice \\
  --network $NETWORK_NAME \\
  $DOCKER_IMAGE \\
  --port 8080 --nickname Alice" C-m

# Wait for Alice to start
sleep 3

# Start second peer (Bob) in right pane
tmux send-keys -t $SESSION_NAME:chat.1 "echo -e '${BLUE}=== PEER 2 (Bob) - Client ===${NC}' && echo" C-m
tmux send-keys -t $SESSION_NAME:chat.1 "docker run -it --rm \\
  --name p2p-bob \\
  --network $NETWORK_NAME \\
  $DOCKER_IMAGE \\
  --connect p2p-alice:8080 --nickname Bob" C-m

# Wait for connection
sleep 2

# Send test messages
echo -e "${YELLOW}Sending test messages...${NC}"
sleep 2
tmux send-keys -t $SESSION_NAME:chat.0 "Hello Bob! This is Alice testing the Docker Hub image." C-m
sleep 1
tmux send-keys -t $SESSION_NAME:chat.1 "Hi Alice! Bob here. The Docker deployment works great!" C-m
sleep 1
tmux send-keys -t $SESSION_NAME:chat.0 "/info" C-m

# Instructions
echo
echo -e "${GREEN}✓ Docker P2P Chat session started!${NC}"
echo
echo -e "${BLUE}tmux commands:${NC}"
echo "  Attach to session:  tmux attach -t $SESSION_NAME"
echo "  Switch panes:       Ctrl+B then arrow keys"
echo "  Detach:            Ctrl+B then D"
echo "  Kill session:       tmux kill-session -t $SESSION_NAME"
echo
echo -e "${BLUE}Docker commands:${NC}"
echo "  View containers:    docker ps"
echo "  View logs:         docker logs p2p-alice"
echo "  Stop containers:    docker stop p2p-alice p2p-bob"
echo
echo -e "${YELLOW}The peers are now connected and chatting!${NC}"
echo -e "${YELLOW}Attach to see the conversation and type your own messages.${NC}"

# Cleanup function
cat > /tmp/cleanup-docker-chat.sh << 'EOF'
#!/bin/bash
echo "Cleaning up Docker containers and network..."
docker stop p2p-alice p2p-bob 2>/dev/null
docker network rm p2p-chat-network 2>/dev/null
tmux kill-session -t docker-chat 2>/dev/null
echo "Cleanup complete!"
EOF
chmod +x /tmp/cleanup-docker-chat.sh

echo
echo -e "${BLUE}To clean up when done:${NC} /tmp/cleanup-docker-chat.sh"

# Ask if user wants to attach
read -p "Do you want to attach to the session now? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    tmux attach -t $SESSION_NAME
fi