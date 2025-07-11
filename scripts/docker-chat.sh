#!/bin/bash

# Docker helper script for Rust P2P Chat

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Print usage
usage() {
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  build           Build the Docker image"
    echo "  run             Run a single peer interactively"
    echo "  demo            Run two peers in demo mode"
    echo "  multi           Run three peers in demo mode"
    echo "  dev             Run in development mode with auto-reload"
    echo "  clean           Remove containers and images"
    echo "  logs            Show logs from all containers"
    echo "  shell           Open a shell in the container"
    echo ""
    echo "Options for 'run':"
    echo "  --port PORT     Listen on PORT (default: 8080)"
    echo "  --connect ADDR  Connect to peer at ADDR"
    echo "  --nickname NAME Set nickname"
    echo ""
    echo "Examples:"
    echo "  $0 build"
    echo "  $0 run --port 8080 --nickname Alice"
    echo "  $0 run --connect localhost:8080 --nickname Bob"
    echo "  $0 demo"
    exit 1
}

# Build Docker image
build_image() {
    echo -e "${GREEN}Building Docker image...${NC}"
    docker build -t rust-p2p-chat:latest .
    echo -e "${GREEN}Build complete!${NC}"
}

# Run single peer
run_peer() {
    shift # Remove 'run' from arguments
    
    echo -e "${GREEN}Starting Rust P2P Chat in Docker...${NC}"
    docker run -it --rm \
        --name p2p-chat-$(date +%s) \
        -e RUST_LOG=info \
        rust-p2p-chat:latest "$@"
}

# Run demo with docker-compose
run_demo() {
    echo -e "${GREEN}Starting P2P Chat demo with 2 peers...${NC}"
    echo -e "${YELLOW}Peer 1 (Alice) will listen on port 8080${NC}"
    echo -e "${YELLOW}Peer 2 (Bob) will connect to Peer 1${NC}"
    echo ""
    docker-compose up --build
}

# Run multi-peer demo
run_multi() {
    echo -e "${GREEN}Starting P2P Chat demo with 3 peers...${NC}"
    docker-compose --profile multi up --build
}

# Run in development mode
run_dev() {
    echo -e "${GREEN}Starting P2P Chat in development mode...${NC}"
    docker-compose -f docker-compose.dev.yml up --build
}

# Clean up containers and images
clean_up() {
    echo -e "${YELLOW}Cleaning up Docker resources...${NC}"
    docker-compose down -v || true
    docker-compose -f docker-compose.dev.yml down -v || true
    docker rm -f $(docker ps -a -q -f name=p2p-chat) 2>/dev/null || true
    docker rmi rust-p2p-chat:latest 2>/dev/null || true
    echo -e "${GREEN}Cleanup complete!${NC}"
}

# Show logs
show_logs() {
    docker-compose logs -f
}

# Open shell in container
open_shell() {
    echo -e "${GREEN}Opening shell in Rust P2P Chat container...${NC}"
    docker run -it --rm \
        --entrypoint /bin/bash \
        rust-p2p-chat:latest
}

# Main script logic
case "$1" in
    build)
        build_image
        ;;
    run)
        build_image
        run_peer "$@"
        ;;
    demo)
        run_demo
        ;;
    multi)
        run_multi
        ;;
    dev)
        run_dev
        ;;
    clean)
        clean_up
        ;;
    logs)
        show_logs
        ;;
    shell)
        build_image
        open_shell
        ;;
    *)
        usage
        ;;
esac