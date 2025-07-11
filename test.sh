#!/bin/bash

# Docker Hub deployment script for rust-p2p-chat
# Usage: ./deploy.sh [username] [tag]

set -e  # Exit on any error

# Configuration
IMAGE_NAME="rust-p2p-chat"
DEFAULT_TAG="latest"

# Get username from command line or prompt
if [ -z "$1" ]; then
    read -p "Enter your Docker Hub username: " DOCKER_USERNAME
else
    DOCKER_USERNAME="$1"
fi

# Get tag from command line or use default
if [ -z "$2" ]; then
    TAG="$DEFAULT_TAG"
else
    TAG="$2"
fi

FULL_IMAGE_NAME="$DOCKER_USERNAME/$IMAGE_NAME:$TAG"

echo "?? Deploying $IMAGE_NAME to Docker Hub as $FULL_IMAGE_NAME"
echo ""

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "? Error: Docker is not running"
    exit 1
fi

# Build the image
echo "?? Building Docker image..."
docker build -t "$IMAGE_NAME" .

# Tag the image
echo "???  Tagging image as $FULL_IMAGE_NAME..."
docker tag "$IMAGE_NAME" "$FULL_IMAGE_NAME"

# Login to Docker Hub (if not already logged in)
echo "?? Logging in to Docker Hub..."
docker login

# Push to Docker Hub
echo "??  Pushing to Docker Hub..."
docker push "$FULL_IMAGE_NAME"

echo ""
echo "? Successfully deployed!"
echo ""
echo "Others can now connect to your P2P chat server with:"
echo "  docker run -it --rm $FULL_IMAGE_NAME"
echo ""
echo "To run YOUR server and expose it on port 8080:"
echo "  docker run -it --rm -p 8080:8080 $FULL_IMAGE_NAME --port 8080 --nickname YourName"
