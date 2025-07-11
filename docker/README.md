# Docker Support for Rust P2P Chat

This directory contains Docker configurations for running the Rust P2P Chat application in containers.

## Prerequisites

- Docker Engine 20.10+ installed
- Docker Compose 2.0+ (optional, for multi-peer setups)
- User added to `docker` group (Linux) or Docker Desktop running (Windows/macOS)

## Files Overview

- **Dockerfile**: Multi-stage production build (optimized size ~50MB)
- **Dockerfile.dev**: Development build with cargo-watch for auto-reload
- **docker-compose.yml**: Run 2-3 peers automatically
- **docker-compose.dev.yml**: Development setup with volume mounts
- **docker-chat.sh**: Helper script for common Docker operations
- **.dockerignore**: Excludes unnecessary files from build context

## Quick Start

### Build the Image

```bash
docker build -t rust-p2p-chat .
```

### Run Single Peer

```bash
# As listener
docker run -it --rm -p 8080:8080 rust-p2p-chat --port 8080

# As connector (in another terminal)
docker run -it --rm rust-p2p-chat --connect host.docker.internal:8080
```

### Run Multiple Peers with Docker Compose

```bash
# Two peers (Alice and Bob)
docker-compose up

# Three peers (add Charlie)
docker-compose --profile multi up
```

### Development Mode

```bash
# With auto-reload on code changes
docker-compose -f docker-compose.dev.yml up
```

## Docker Chat Helper Script

The `docker-chat.sh` script simplifies common Docker operations:

```bash
./docker-chat.sh build    # Build the image
./docker-chat.sh demo     # Run 2-peer demo
./docker-chat.sh multi    # Run 3-peer demo
./docker-chat.sh dev      # Development mode
./docker-chat.sh clean    # Remove all containers/images
```

## Network Configuration

### Container-to-Container

When running multiple containers, they can connect using service names:
- `peer1:8080`
- `peer2:8080`

### Host-to-Container

- Linux: `localhost:8080`
- Windows/macOS: `host.docker.internal:8080`

### External Connections

Expose ports in docker-compose.yml or use `-p` flag:
```bash
docker run -p 8080:8080 rust-p2p-chat --port 8080
```

## Volume Mounts

For persistent configuration:
```bash
docker run -v ~/.config/rustchat:/home/rustchat/.config/rustchat rust-p2p-chat
```

## Security Notes

- Runs as non-root user (UID 1001)
- Minimal base image (debian-slim)
- No unnecessary packages installed
- Read-only root filesystem compatible

## Troubleshooting

### Permission Denied

If you get Docker permission errors:
```bash
# Add user to docker group (Linux)
sudo usermod -aG docker $USER
newgrp docker

# Or use sudo (not recommended)
sudo docker build -t rust-p2p-chat .
```

### Connection Issues

- Ensure ports are not already in use
- Check firewall settings
- Verify Docker network configuration

### Build Failures

- Check Docker daemon is running
- Ensure sufficient disk space
- Try cleaning Docker cache: `docker system prune`

## Image Size Optimization

The multi-stage build reduces the final image size:
- Build stage: ~1.5GB (includes Rust toolchain)
- Final stage: ~50MB (only runtime dependencies)

## CI/CD Integration

GitHub Actions workflow included for automatic builds:
- Builds on push to main/master
- Multi-platform support (amd64, arm64)
- Publishes to GitHub Container Registry