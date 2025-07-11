#!/bin/bash

# Deploy script for pushing Rust P2P Chat to Docker Hub
# Usage: ./deploy-docker.sh <your-dockerhub-username>

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if username is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Docker Hub username required${NC}"
    echo "Usage: $0 <your-dockerhub-username>"
    echo "Example: $0 myusername"
    exit 1
fi

DOCKER_USERNAME=$1
IMAGE_NAME="rust-p2p-chat"
FULL_IMAGE_NAME="${DOCKER_USERNAME}/${IMAGE_NAME}"

echo -e "${YELLOW}=== Docker Hub Deployment Script ===${NC}"
echo -e "Deploying to: ${GREEN}${FULL_IMAGE_NAME}${NC}"
echo

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Error: Docker daemon is not running${NC}"
    exit 1
fi

# Login to Docker Hub
echo -e "${YELLOW}Step 1: Logging in to Docker Hub${NC}"
echo "Please enter your Docker Hub credentials:"
docker login || {
    echo -e "${RED}Failed to login to Docker Hub${NC}"
    echo "Make sure you have a Docker Hub account at https://hub.docker.com"
    exit 1
}

# Build the image
echo -e "${YELLOW}Step 2: Building Docker image${NC}"
docker build -t ${IMAGE_NAME} . || {
    echo -e "${RED}Failed to build Docker image${NC}"
    exit 1
}

# Tag the image
echo -e "${YELLOW}Step 3: Tagging image${NC}"
docker tag ${IMAGE_NAME}:latest ${FULL_IMAGE_NAME}:latest

# Push to Docker Hub
echo -e "${YELLOW}Step 4: Pushing to Docker Hub${NC}"
docker push ${FULL_IMAGE_NAME}:latest || {
    echo -e "${RED}Failed to push to Docker Hub${NC}"
    echo "Make sure you have push access to ${FULL_IMAGE_NAME}"
    exit 1
}

echo
echo -e "${GREEN}✓ Successfully deployed to Docker Hub!${NC}"
echo
echo -e "${YELLOW}Your image is now available at:${NC}"
echo -e "  ${GREEN}docker pull ${FULL_IMAGE_NAME}:latest${NC}"
echo
echo -e "${YELLOW}To run your image:${NC}"
echo -e "  # As server:"
echo -e "  ${GREEN}docker run -it --rm -p 8080:8080 ${FULL_IMAGE_NAME}:latest --port 8080${NC}"
echo
echo -e "  # As client:"
echo -e "  ${GREEN}docker run -it --rm ${FULL_IMAGE_NAME}:latest --connect <server-ip>:8080${NC}"
echo
echo -e "${YELLOW}Share this command with others to let them use your chat:${NC}"
echo -e "  ${GREEN}docker run -it --rm ${FULL_IMAGE_NAME}:latest${NC}"