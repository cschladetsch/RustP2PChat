# Rust P2P Chat - Feature Documentation

## Overview

This document provides detailed information about all the features implemented in the Rust P2P Chat application.

## Core Features

### 1. True Peer-to-Peer Architecture

- **No Central Server**: Direct TCP connections between peers
- **Symmetric Design**: Both peers run identical code
- **Equal Capabilities**: After connection, both peers have the same features
- **Simultaneous Connect/Listen**: Can attempt outbound connection while accepting inbound

### 2. Enhanced Message Protocol

The application supports multiple message types through a binary protocol:

- **Text Messages**: Regular chat messages (backward compatible)
- **File Transfers**: Send files up to 100MB (configurable)
- **Commands**: Built-in command system
- **Status Updates**: Progress tracking and connection status
- **Heartbeat**: Keep-alive mechanism

### 3. Command System

Available commands during chat:

| Command | Description |
|---------|-------------|
| `/help` or `/?` | Display available commands |
| `/quit` or `/exit` | Exit the chat application |
| `/send <filename>` | Send a file to the peer |
| `/info` | Show connection information |
| `/nick <name>` | Set your nickname |

### 4. File Transfer

- **Size Limit**: Default 100MB (configurable)
- **Hash Verification**: SHA256 integrity checking
- **Progress Tracking**: Real-time transfer progress
- **Auto-save**: Files saved to `downloads/` directory

### 5. Configuration System

Create a default configuration file:
```bash
./rust-p2p-chat config
```

Configuration options (config.toml):
```toml
nickname = "Alice"
default_port = 8080
buffer_size = 8192
heartbeat_interval_secs = 30
reconnect_attempts = 3
reconnect_delay_secs = 5
enable_encryption = true
log_level = "info"
save_history = true
max_file_size_mb = 100
```

### 6. Command-Line Interface

```bash
# Basic usage
./rust-p2p-chat --port 8080
./rust-p2p-chat --connect 192.168.1.100:8080

# With options
./rust-p2p-chat --port 8080 --nickname Alice --debug
./rust-p2p-chat --connect 192.168.1.100:8080 --no-encryption

# Generate config
./rust-p2p-chat config
```

CLI Options:
- `-p, --port <PORT>`: Port to listen on (default: 8080)
- `-c, --connect <ADDRESS>`: Peer address to connect to
- `-n, --nickname <NAME>`: Set your nickname
- `-d, --debug`: Enable debug logging
- `--no-encryption`: Disable encryption (when implemented)

### 7. Error Handling

Custom error types provide clear, actionable error messages:
- `BindFailed`: Port already in use with diagnostic help
- `ConnectFailed`: Connection refused with fallback behavior
- `PeerDisconnected`: Clean disconnection handling
- `Protocol`: Message parsing errors
- `FileTransfer`: File-specific errors with size limits
- `Configuration`: Config file issues

### 8. Logging

Configurable logging levels:
- `trace`: Very detailed debugging
- `debug`: Debugging information
- `info`: General information (default)
- `warn`: Warning messages
- `error`: Error messages only

### 9. Color Support

The application uses ANSI colors for better readability:
- **Green**: Your messages
- **Cyan**: Peer messages
- **Yellow**: Status messages
- **Red**: Error messages

### 10. Performance Features

- **8KB Buffer**: Larger buffer for efficient message handling
- **Async I/O**: Non-blocking operations with Tokio
- **Zero-copy**: Minimal memory allocations
- **Stream Splitting**: Separate read/write for concurrent I/O

## Usage Examples

### Basic Chat Session

**Terminal 1 (First Peer):**
```bash
$ ./rust-p2p-chat --port 8080 --nickname Alice
Listening on: 0.0.0.0:8080
Waiting for peer to connect...
✓ Peer connected from: 127.0.0.1:51234
Type messages and press Enter to send (Ctrl+C to exit)
Type /help for available commands
You: Hello Bob!
Peer: Hi Alice! How are you?
You: /send document.pdf
✓ File sent
```

**Terminal 2 (Second Peer):**
```bash
$ ./rust-p2p-chat --connect 127.0.0.1:8080 --nickname Bob
Attempting to connect to peer at: 127.0.0.1:8080
✓ Connected to peer at: 127.0.0.1:8080
Type messages and press Enter to send (Ctrl+C to exit)
Type /help for available commands
Peer: Hello Bob!
You: Hi Alice! How are you?
📁 Receiving file: document.pdf (1048576 bytes)
✓ File saved to downloads/document.pdf
```

### Advanced Usage

#### Testing with Debug Logging
```bash
./rust-p2p-chat --port 8080 --debug
```

#### Custom Configuration
1. Generate config: `./rust-p2p-chat config`
2. Edit the generated config.toml
3. Run normally - config will be loaded automatically

#### Testing Between Machines
```bash
# Machine A (IP: 192.168.1.100)
./rust-p2p-chat --port 8080

# Machine B
./rust-p2p-chat --connect 192.168.1.100:8080
```

## Architecture Details

### Module Structure

- **`main.rs`**: CLI interface and application entry point
- **`lib.rs`**: Core chat implementation
- **`protocol.rs`**: Message types and serialization
- **`error.rs`**: Custom error types
- **`config.rs`**: Configuration management
- **`file_transfer.rs`**: File transfer functionality
- **`commands.rs`**: Command parsing and handling
- **`peer.rs`**: Peer management (for future multi-peer support)
- **`encryption.rs`**: TLS support (ready for activation)
- **`colors.rs`**: ANSI color codes

### Message Flow

1. User input → Command parser → Message creation
2. Message → Serialization → TCP stream
3. TCP stream → Deserialization → Message handling
4. Message handling → Display or action

### Future Enhancements

The codebase is prepared for:
- **Multiple Peers**: PeerManager can handle multiple connections
- **Full Encryption**: TLS infrastructure is ready
- **Message History**: Can be saved to disk
- **GUI Interface**: Core logic is separate from CLI
- **Peer Discovery**: Can be added with minimal changes

## Troubleshooting

### Port Already in Use
```
Error: Port 8080 is already in use!
To find the process: lsof -i :8080 or netstat -tuln | grep 8080
```

### Connection Refused
The application automatically falls back to listening mode if connection fails.

### Large File Transfer Failed
Check the `max_file_size_mb` setting in your configuration.

## Development

### Running Tests
```bash
# All tests
cargo test

# Lib tests only
cargo test --lib

# With output
cargo test -- --nocapture
```

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```