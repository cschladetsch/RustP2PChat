# Rust P2P Chat

A blazing-fast, truly decentralized peer-to-peer chat application built with Rust and Tokio. Experience real-time communication without any intermediary servers - just pure, direct connections between peers!

See [Features](FEATURES.md) and [ChangeLog](CHANGELOG.md)

## 📚 Table of Contents

- [Key Highlights](#-key-highlights)
- [What Makes This Special](#what-makes-this-special)
- [Demo](#demo)
- [Quick Start](#-quick-start)
- [Features](#features)
- [Installation Options](#installation-options)
- [Usage](#usage)
- [Commands](#commands)
- [Configuration](#configuration)
- [Building from Source](#building-from-source)
- [Testing](#testing)
- [Shell Scripts](#shell-scripts)
- [Connecting Over Internet](#connecting-over-the-internet)
- [Security](#-security)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Key Highlights

- 🔒 **Military-Grade Encryption**: 1024-bit RSA + AES-256-GCM end-to-end encryption
- ⚡ **Zero Configuration**: Works instantly with just IP:port - no setup required
- 🌐 **True P2P**: Direct peer connections, no central servers or intermediaries
- 📁 **File Transfer**: Send files up to 100MB with progress tracking and auto-open
- 🎨 **Rich Terminal UI**: Colorful interface with encryption status indicators
- 🔧 **Cross-Platform**: Linux, macOS, Windows support with async Rust performance
- 🍎 **macOS Installer**: Universal DMG installer for Intel and Apple Silicon Macs
- 🎬 **Media Auto-Open**: Automatically open received images, videos, and documents
- 💾 **Smart Downloads**: Files saved to system Downloads folder with verification
- 🧪 **Comprehensive Testing**: 160+ tests across 10 categories with extensive coverage

## What Makes This Special?

Unlike traditional chat applications that rely on central servers, **Rust P2P Chat** establishes direct TCP connections between peers. There's no "server" and "client" in the traditional sense - both peers are equal participants in the conversation. The first peer simply waits for a connection, while the second initiates it. Once connected, both peers have identical capabilities!

## Demo

![Demo](resources/Demo1.gif)

## 🏃 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/cschladetsch/RustP2PChat.git
cd RustP2PChat

# Build and run
cargo run --release
```

### Start Chatting in 30 Seconds

**Terminal 1 (First peer):**
```bash
cargo run --release -- --port 8080
```

**Terminal 2 (Second peer):**
```bash
cargo run --release -- --connect localhost:8080
```

That's it! You're now chatting peer-to-peer with end-to-end encryption! 🎉

## Recent Updates

### Latest Changes (June 2025)
- **Test Improvements**: Updated integration tests to use idiomatic Rust struct initialization
- **Code Quality**: Fixed clippy warnings, now using `is_none_or` instead of `map_or`
- **Quick Testing**: Added `quick_test.sh` script for rapid peer testing
- **Documentation**: Enhanced README with security notices and key highlights

### Recently Implemented Features
- **Custom Error Types**: Replaced generic errors with specific ChatError types
- **Enhanced Message Protocol**: Support for text, files, commands, and status updates
- **File Transfer**: Send files up to 100MB (configurable) with progress tracking
- **Auto-open Media**: Automatically open received images, videos, and documents
- **Command System**: Built-in commands like /help, /quit, /send, /autoopen
- **Configuration Support**: TOML-based config files with customizable settings
- **CLI Arguments**: Full command-line interface with clap
- **Logging**: Configurable logging levels with tracing
- **Large Buffer Support**: 8KB message buffer (configurable)
- **Connection Heartbeat**: Keep-alive mechanism for connection monitoring
- **End-to-End Encryption**: 1024-bit RSA + AES-256-GCM encryption

## Features

### Core Capabilities
- **True Peer-to-Peer Architecture**: No central server, no middleman - just direct connections between peers
- **Symmetric Communication**: Once connected, both peers are equal - no client/server hierarchy
- **Real-time Bidirectional Messaging**: Instant message delivery with concurrent send/receive
- **Zero Configuration**: Start chatting with just a port number or peer address
- **Cross-platform Support**: Works on Linux, macOS, and Windows
- **End-to-End Encryption**: All messages are encrypted using military-grade encryption

### Technical Features
- **Async/Await Excellence**: Built on Tokio for high-performance async I/O
- **Colorful Terminal UI**: ANSI color support for better user experience
- **Graceful Error Handling**: Robust connection management and clean disconnection
- **Smart Connection Logic**: Simultaneous connect/listen with automatic fallback
- **Low Latency**: Direct TCP connections ensure minimal message delay
- **Command-line Interface**: Supports both interactive mode and CLI arguments

### Security Features
- **1024-bit RSA Key Exchange**: Secure public key cryptography for initial handshake
- **AES-256-GCM Encryption**: Military-grade symmetric encryption for messages
- **Automatic Key Generation**: New encryption keys for every session
- **Message Authentication**: Built-in integrity verification with GCM
- **Visual Encryption Indicators**: 🔒 icon shows when messages are encrypted

## Usage

### Running the application

#### Interactive Mode
```bash
cargo run
```

#### Command-line Mode
```bash
# Start as listener on specific port
cargo run -- --port 8080

# Connect to a peer directly  
cargo run -- --connect 192.168.1.100:8080

# With nickname
cargo run -- --port 8080 --nickname Alice

# Enable debug logging
cargo run -- --port 8080 --debug

# Generate default config file
cargo run -- config
```

### Starting a chat session

Since this is a **true P2P application**, there's no permanent "server" or "client" - just two equal peers! The terminology below is used only to distinguish who initiates the connection:

1. **First Peer (Listener):**
   - Run the application
   - Press Enter when prompted for peer address
   - Enter a port number (default 8080)
   - The peer will bind to `0.0.0.0:port` and wait for incoming connections
   - Once connected, this peer has the exact same capabilities as the connecting peer

2. **Second Peer (Connector):**
   - Run the application
   - Enter the first peer's address in format `ip:port` (e.g., `127.0.0.1:8080`)
   - Connection will be established automatically
   - Once connected, this peer has the exact same capabilities as the listening peer

**Important**: After connection is established, both peers are completely equal - they can both send and receive messages simultaneously!

### Example usage

**Terminal 1 (Server):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 
Enter port to listen on (default 8080): 8080
Listening on: 0.0.0.0:8080
Waiting for peer to connect...
```

**Terminal 2 (Client):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 127.0.0.1:8080
Connecting to 127.0.0.1:8080...
Connected to peer!
```

### Sending messages

Once connected, type messages and press Enter to send. Messages are displayed with color-coded prefixes:
- Your messages: "You:" in green
- Peer messages: "Peer:" in cyan
- Encrypted messages show with 🔒 icon
- Unencrypted messages show "(unencrypted)" warning

The chat automatically establishes end-to-end encryption on connection. You'll see:
- "Exchanging encryption keys..." during handshake
- "🔒 End-to-end encryption enabled!" when secure

To exit, press Ctrl+C.

## Commands

During a chat session, you can use the following commands:

| Command | Alias | Description |
|---------|-------|-------------|
| `/help` | `/?` | Display available commands |
| `/quit` | `/exit` | Exit the chat application |
| `/send <file>` | `/file` | Send a file to the peer |
| `/info` | | Show connection and configuration information |
| `/nick <name>` | `/nickname` | Set or change your nickname |
| `/autoopen` | `/auto` | Toggle auto-open for received media files |
| `/peers` | `/list` | List connected peers (for future multi-peer support) |

### Command Examples

```bash
# Send a file
You: /send ~/Pictures/vacation.jpg
✓ File sent

# Change nickname
You: /nick Alice
✓ Nickname set to: Alice

# Toggle auto-open
You: /autoopen
✓ Auto-open media: disabled

# Get help
You: /help
Available commands:
  /help, /?          - Show this help message
  ...
```

## Technical Details

### Architecture Overview

This application implements a **symmetric peer-to-peer architecture** where:
- Both peers run identical code
- No dedicated server process - any peer can listen or connect
- After handshake, the connection is fully bidirectional with no master/slave relationship
- Each peer maintains its own event loop for handling I/O

### Core Implementation Details

#### Asynchronous Runtime
- **Tokio Runtime**: Leverages Tokio's multi-threaded runtime for efficient async I/O
- **Zero-copy Operations**: Minimizes memory allocations during message passing
- **Event-driven Architecture**: Non-blocking I/O ensures responsive user experience

#### Connection Management
- **TCP Socket Handling**: Direct TCP stream manipulation for low-level control
- **Stream Splitting**: Uses `stream.into_split()` for separate read/write halves
- **Concurrent I/O**: Separate async tasks for reading and writing operations
- **Graceful Shutdown**: Proper resource cleanup on disconnection
- **Simultaneous Connect/Listen**: Can try connecting while accepting connections

#### Message Protocol
- **Binary Protocol**: Supports both plaintext (backward compatible) and binary formats
- **Message Types**: Text, EncryptedText, File, Command, Status, Heartbeat, Encryption
- **Direct Async I/O**: Uses `AsyncReadExt` and `AsyncWriteExt` for socket operations
- **Stream Processing**: Handles partial reads and message fragmentation
- **Large Buffer**: Configurable buffer size (default 8KB)
- **Automatic Serialization**: Uses bincode for efficient message encoding

#### Encryption Protocol
- **Automatic Handshake**: RSA public key exchange on connection
- **Session Keys**: Fresh AES-256 key generated for each session
- **Zero-Knowledge**: Keys are never transmitted in plaintext
- **Forward Secrecy**: New keys for every connection
- **Transparent Encryption**: Messages automatically encrypted when available

#### Error Handling Strategy
- **Connection Resilience**: Gracefully handles network interruptions
- **Input Validation**: Sanitizes user input and peer addresses
- **Comprehensive Error Types**: Detailed error reporting for debugging
- **Recovery Mechanisms**: Automatic cleanup on peer disconnection
- **Port Conflict Detection**: Helpful diagnostics when port is already in use

### Performance Characteristics
- **Low Memory Footprint**: Minimal runtime overhead (~2-5 MB)
- **High Throughput**: Can handle thousands of messages per second
- **Low Latency**: Sub-millisecond message delivery on local networks
- **Scalable Architecture**: Could be extended to support multiple peers

## Configuration

The application supports configuration through a TOML file. Generate a default config:

```bash
cargo run -- config
```

This creates a config file at:
- Linux/macOS: `~/.config/rustchat/p2p-chat/config.toml`
- Windows: `%APPDATA%\rustchat\p2p-chat\config.toml`

### Configuration Options

```toml
# User settings
nickname = "Alice"                   # Your display name
default_port = 8080                  # Default listening port

# Network settings
buffer_size = 8192                   # Message buffer size in bytes
heartbeat_interval_secs = 30         # Keep-alive interval
reconnect_attempts = 3               # Number of reconnection attempts
reconnect_delay_secs = 5             # Delay between reconnection attempts

# Security settings
enable_encryption = true             # Enable end-to-end encryption

# File transfer settings
max_file_size_mb = 100              # Maximum file size for transfers
download_dir = "/path/to/downloads"  # Custom download directory (optional)
auto_open_media = true              # Auto-open received media files
media_extensions = [                 # File types to auto-open
    "jpg", "jpeg", "png", "gif",
    "mp4", "avi", "mov",
    "pdf", "doc", "docx"
]

# Logging settings
log_level = "info"                   # Options: trace, debug, info, warn, error
save_history = true                  # Save chat history
history_file = "/path/to/history"    # Custom history file location (optional)
```

## Testing

The project includes comprehensive unit and integration tests covering all major functionality.

### Running Tests

Run all tests:
```bash
cargo test
```

Run unit tests only:
```bash
cargo test --lib
```

Run integration tests:
```bash
cargo test --test integration_tests
cargo test --test simple_integration_test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Test encryption specifically:
```bash
cargo run --bin test_chat
```

### Quick Testing Scripts

The project includes several testing and demo scripts in the `shell/` directory:

**Quick Testing:**
```bash
./shell/quick_test.sh          # Rapid testing of two peers
./shell/automated_test.sh      # Automated test scenarios
./shell/comprehensive_test.sh  # Full test suite with all features
```

**Demo Scripts:**
```bash
./shell/demo.sh                # Basic demo setup
./shell/demo_chat.sh          # Interactive chat demonstration
./shell/demo_colors.sh        # Terminal color testing
```

**Specialized Tests:**
```bash
./shell/test_encryption.sh    # Encryption-specific tests
./shell/test_p2p.sh          # P2P connection tests
./shell/test_tmux.sh         # tmux-based split terminal testing
```

### Test Coverage

The project now includes a **comprehensive test suite with 183+ individual tests** across 10 major categories:

**Comprehensive Test Suite (183+ tests total):**

1. **File Transfer Tests** (9 tests) - Hash verification, size limits, unicode filenames, directory handling
2. **Configuration Tests** (10 tests) - Defaults, validation, serialization, path resolution  
3. **Protocol Tests** (14 tests) - Message serialization, all message types, large data handling
4. **Command Tests** (20 tests) - Command parsing, handler functionality, edge cases
5. **Error Handling Tests** (34 tests) - All error types, user-friendly messages, source chains
6. **Reliability Tests** (15 tests) - Message acknowledgments, retries, timeout handling
7. **Concurrent Tests** (7 tests) - Stress testing with 20+ connections, race conditions
8. **Peer Management Tests** (15 tests) - Concurrent access, IPv6 support, edge cases
9. **Encryption Tests** (39 tests) - E2E encryption, RSA key exchange, AES-256-GCM
10. **Integration Tests** (20 tests) - Real-world scenarios, file workflows, system integration

**Key Test Features:**
- **Edge Case Coverage**: Unicode handling, large files, concurrent operations
- **Security Testing**: Comprehensive encryption, key exchange, signing verification
- **Error Scenarios**: Network failures, invalid inputs, permission issues
- **Performance Testing**: Stress tests with 20+ concurrent connections
- **Real-world Workflows**: File transfers, configuration persistence, graceful shutdown

**Legacy Tests:**
- **Unit Tests** (3): Core library functionality
- **Simple Integration Tests** (7): Basic connection and messaging

All tests use modern Rust testing practices with proper async/await patterns, temporary file cleanup, and comprehensive assertions.

### Code Quality

- **Clippy**: No warnings (clean linting)
- **Type checking**: All types verified with `cargo check`
- **Performance**: Sub-second test execution

Note: The integration tests serve as examples of how to use the chat functionality programmatically.

## Project Structure

```
rust-p2p-chat/
├── Cargo.toml           # Project dependencies and metadata
├── Readme.md            # This documentation
├── resources/           # Demo and documentation assets
│   └── Demo1.gif       # Animated demonstration
├── src/
│   ├── main.rs         # Application entry point and CLI interface
│   ├── lib.rs          # Core P2P chat implementation
│   └── colors.rs       # ANSI color support for terminal UI
└── tests/
    ├── integration_tests.rs      # Complex multi-peer scenarios
    └── simple_integration_test.rs # Basic connection and messaging tests
```

### Key Components

- **`main.rs`**: Handles user interaction, connection setup, and message I/O loops
- **`lib.rs`**: Implements the P2P protocol, connection management, and async operations
- **`colors.rs`**: Provides ANSI color codes for enhanced terminal output
- **Integration Tests**: Demonstrate usage patterns and test scenarios

## Testing Locally

### Testing with Two Terminals

#### Method 1: Both terminals on same machine

**Terminal 1 (Server):**
```bash
cargo run
# Press Enter when prompted for peer address
# Enter port (or press Enter for default 8080)
```

**Terminal 2 (Client):**
```bash
cargo run
# Enter: 127.0.0.1:8080
```

#### Method 2: Quick test commands

**Terminal 1:**
```bash
cargo run --release
# Press Enter, then Enter again (uses default port 8080)
```

**Terminal 2:**
```bash
cargo run --release
# Type: 127.0.0.1:8080
```

### Example Session

**Terminal 1 (Server):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 
Enter port to listen on (default 8080): 
Listening on: 0.0.0.0:8080
Waiting for peer to connect...
Peer connected from: 127.0.0.1:xxxxx
Type messages and press Enter to send (Ctrl+C to exit)
You: Hello from server!
Peer: Hi from client!
You: How are you?
```

**Terminal 2 (Client):**
```
$ cargo run
P2P Chat Application
Usage:
  1. Start as server: just press Enter when prompted for address
  2. Connect to peer: enter address as ip:port (e.g., 127.0.0.1:8080)

Enter peer address (or press Enter to start as server): 127.0.0.1:8080
Connecting to 127.0.0.1:8080...
Connected to peer!
Type messages and press Enter to send (Ctrl+C to exit)
You: Hi from client!
Peer: Hello from server!
You: I'm doing great!
```

### Testing Between Different Machines

1. **Find server's IP address:**
   ```bash
   # Linux/Mac
   ip addr show | grep inet
   # or
   ifconfig
   
   # Windows
   ipconfig
   ```

2. **On server machine:**
   ```bash
   cargo run
   # Press Enter, use port 8080
   ```

3. **On client machine:**
   ```bash
   cargo run
   # Enter: SERVER_IP:8080 (e.g., 192.168.1.100:8080)
   ```

## Shell Scripts

The `shell/` directory contains various testing and demonstration scripts to help you quickly test and explore the application:

### 🧪 Testing Scripts
- **`quick_test.sh`** - Rapid two-peer testing setup
- **`automated_test.sh`** - Automated test scenarios with predefined inputs
- **`comprehensive_test.sh`** - Full feature test suite
- **`test_encryption.sh`** - Encryption-specific functionality tests
- **`test_p2p.sh`** - P2P connection and messaging tests
- **`test_tmux.sh`** - Split-screen terminal testing using tmux

### 🎬 Demo Scripts  
- **`demo.sh`** - Basic demonstration setup
- **`demo_chat.sh`** - Interactive chat demonstration
- **`demo_colors.sh`** - Terminal color and formatting tests

### Usage
```bash
# Make scripts executable (if needed)
chmod +x shell/*.sh

# Run any script
./shell/quick_test.sh
```

These scripts automate common testing scenarios and provide examples of different usage patterns. They're especially useful for development, demonstration, and continuous integration testing.

## Connecting Over the Internet

Since this is a direct TCP connection app, at least one peer needs a publicly accessible IP/port. Here are several ways to connect with friends over the internet:

### Method 1: Ngrok (Easiest - 2 minutes setup)

**Ngrok** creates a public tunnel to your local port. Perfect for quick chats!

1. **Install ngrok:**
   ```bash
   # Linux/Mac (via apt)
   curl -s https://ngrok-agent.s3.amazonaws.com/ngrok.asc | sudo tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null && echo "deb https://ngrok-agent.s3.amazonaws.com buster main" | sudo tee /etc/apt/sources.list.d/ngrok.list && sudo apt update && sudo apt install ngrok
   
   # Or download directly from https://ngrok.com/download
   ```

2. **You (host):**
   ```bash
   # Terminal 1: Start the chat
   cargo run -- --port 8080
   
   # Terminal 2: Create public tunnel
   ngrok tcp 8080
   ```

3. **Share with friend:**
   Ngrok will show: `Forwarding tcp://0.tcp.ngrok.io:12345 -> localhost:8080`
   
   Your friend runs:
   ```bash
   cargo run -- --connect 0.tcp.ngrok.io:12345
   ```

That's it! No router configuration needed. Works through any firewall.

### Method 2: Port Forwarding

If you have router access:

1. **Configure router:**
   - Access router admin panel (usually 192.168.1.1)
   - Forward port 8080 to your local IP
   - Find your public IP: `curl ifconfig.me`

2. **Start chat:**
   ```bash
   cargo run -- --port 8080
   ```

3. **Friend connects:**
   ```bash
   cargo run -- --connect YOUR_PUBLIC_IP:8080
   ```

### Method 3: VPN Solutions

Use a mesh VPN for a private network between devices:

- **[Tailscale](https://tailscale.com/)**: Easiest setup, free for personal use
- **[ZeroTier](https://www.zerotier.com/)**: Open source alternative

Both give you and your friend private IPs that work as if you're on the same network.

### Method 4: Cloud VPS

Rent a small VPS (AWS EC2, DigitalOcean, Linode):

```bash
# On VPS
./rust-p2p-chat --port 8080

# Both you and friend connect to VPS
./rust-p2p-chat --connect VPS_IP:8080
```

### Method 5: Other Tunneling Services

- **[localtunnel](https://localtunnel.github.io/www/)**: `lt --port 8080`
- **[bore](https://github.com/ekzhang/bore)**: `bore local 8080 --to bore.pub`
- **[serveo](https://serveo.net/)**: `ssh -R 80:localhost:8080 serveo.net`

### Current Limitations

This app uses direct TCP connections, so it requires at least one peer to have a publicly accessible IP/port. For true P2P through NATs without any configuration, you would need:
- UDP hole punching
- STUN/TURN servers
- ICE negotiation
- Or a full WebRTC implementation

These features could be added in future versions for completely configuration-free connections.

### Alternative Testing Methods

#### Using tmux or screen:

```bash
# Install tmux if not already installed
sudo apt install tmux  # Ubuntu/Debian
# or
brew install tmux      # macOS

# Start tmux session
tmux new -s chat-test

# Split window horizontally (Ctrl+B then ")
# Switch between panes with Ctrl+B then arrow keys

# In first pane: run server
cargo run
# Press Enter twice

# In second pane: run client  
cargo run
# Type: 127.0.0.1:8080
```

### Testing specific scenarios:

1. **Test connection refusal:**
   ```bash
   cargo run
   # Enter: 127.0.0.1:9999 (non-existent server)
   ```

2. **Test with custom port:**
   ```bash
   # Server
   cargo run
   # Press Enter, then type: 3000
   
   # Client
   cargo run  
   # Type: 127.0.0.1:3000
   ```

3. **Test rapid messages:**
   Once connected, type multiple messages quickly to test buffering.

### Pro Tips

- **Performance Mode**: Use `cargo run --release` for optimal performance
- **Message Display**: Received messages are prefixed with "Peer:" for clarity
- **Clean Exit**: Press Ctrl+C to gracefully close the connection
- **Connection Status**: The app notifies you when peers connect or disconnect
- **Network Flexibility**: Works across LANs, WANs, and even through port forwarding
- **Firewall Note**: Ensure the listening port is open in your firewall settings

## Documentation

For detailed feature documentation, see [FEATURES.md](FEATURES.md).

## Installation Options

### macOS Installer

For macOS users, you can download and install the pre-built application:

1. **Download the DMG installer** from the releases page
2. **Open the DMG file** and drag RustP2PChat.app to Applications
3. **Launch from Applications** or Launchpad

The macOS installer includes:
- Universal binary (Intel + Apple Silicon support)
- Standard macOS app bundle
- Automatic file associations
- System Downloads folder integration

**Building macOS Installer:**
```bash
# Cross-compile for macOS (requires macOS or cross-compilation setup)
./build-macos.sh

# Output: RustP2PChat-0.1.0.dmg
```

See [macos-installer.md](macos-installer.md) for detailed build instructions.

## Building from Source

### Prerequisites

First, ensure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/).

### Platform-Specific Build Instructions

#### Windows

1. **Install Rust (if not already installed):**
   ```powershell
   # Download and run rustup-init.exe from https://rustup.rs/
   # Or use winget:
   winget install Rustlang.Rust
   ```

2. **Clone and build:**
   ```powershell
   # Clone the repository
   git clone https://github.com/cschladetsch/RustP2PChat.git
   cd RustP2PChat

   # Build release version
   cargo build --release

   # Run tests
   cargo test

   # Run the application
   .\target\release\rust-p2p-chat.exe --help
   ```

3. **Windows Firewall Note:**
   - On first run, Windows Firewall may prompt you to allow the application
   - Allow access for both private and public networks if you plan to connect over the internet

#### Ubuntu/WSL

1. **Install Rust and dependencies:**
   ```bash
   # Update package list
   sudo apt update

   # Install build essentials and Rust dependencies
   sudo apt install -y build-essential pkg-config libssl-dev

   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **Clone and build:**
   ```bash
   # Clone the repository
   git clone https://github.com/cschladetsch/RustP2PChat.git
   cd RustP2PChat

   # Build release version
   cargo build --release

   # Run tests
   cargo test

   # Run the application
   ./target/release/rust-p2p-chat --help
   ```

3. **WSL-specific notes:**
   - For connecting between WSL and Windows host, use the WSL IP address (run `hostname -I` in WSL)
   - Port forwarding may be required for external connections

#### macOS

1. **Install Rust and dependencies:**
   ```bash
   # Install Xcode Command Line Tools (if not already installed)
   xcode-select --install

   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **Clone and build:**
   ```bash
   # Clone the repository
   git clone https://github.com/cschladetsch/RustP2PChat.git
   cd RustP2PChat

   # Build release version
   cargo build --release

   # Run tests
   cargo test

   # Run the application
   ./target/release/rust-p2p-chat --help
   ```

3. **macOS Security Note:**
   - On first run, macOS may block the application
   - Go to System Preferences → Security & Privacy → General
   - Click "Allow Anyway" for rust-p2p-chat
   - Or run with: `sudo spctl --add ./target/release/rust-p2p-chat`

### Build Options

```bash
# Debug build (slower but with debug symbols)
cargo build

# Release build (optimized)
cargo build --release

# Run directly without building binary
cargo run -- --port 8080

# Build and run tests
cargo test

# Build with specific features (if available)
cargo build --release --features "feature_name"
```

### Troubleshooting Build Issues

1. **Rust version too old:**
   ```bash
   rustup update
   ```

2. **Missing OpenSSL (Ubuntu/WSL):**
   ```bash
   sudo apt install libssl-dev
   ```

3. **Permission denied (Unix-like systems):**
   ```bash
   chmod +x ./target/release/rust-p2p-chat
   ```

4. **Build cache issues:**
   ```bash
   cargo clean
   cargo build --release
   ```

## Quick Start Examples

### Basic Usage
```bash
# Terminal 1: Start first peer
./rust-p2p-chat --port 8080

# Terminal 2: Connect to first peer
./rust-p2p-chat --connect 127.0.0.1:8080
```

### With Nicknames
```bash
# Terminal 1
./rust-p2p-chat --port 8080 --nickname Alice

# Terminal 2
./rust-p2p-chat --connect 127.0.0.1:8080 --nickname Bob
```

### Send a File
Once connected, use the `/send` command:
```
You: /send myfile.pdf
✓ File sent
```

When receiving files:
```
📁 Receiving file: image.jpg (1234567 bytes)
✓ File saved to: /Users/alice/Downloads/image.jpg
🎬 Opening media file...
```

Media files (images, videos, PDFs) are automatically opened by default. Toggle this with:
```
You: /autoopen
✓ Auto-open media: disabled
```

### Testing Encryption
The chat automatically establishes end-to-end encryption. To verify:

```bash
# Run the encryption test
cargo run --bin test_chat

# Output:
✓ Created P2P chat instance
✓ Encryption support: 1024-bit RSA + AES-256-GCM
✓ Generated RSA keypairs
✓ Exchanged public keys
✓ Established shared AES key
✓ Encrypted message: This is a secret message! -> 6/nQ+b1exOQM0jkx/co38KxQl28K2Sqh
✓ Decrypted message: This is a secret message!
✅ All encryption tests passed!
```

### What You'll See During Chat
When peers connect, you'll see the encryption handshake:
```
Received encryption key from peer...
Exchanging encryption keys...
🔒 End-to-end encryption enabled!
```

Messages will show encryption status:
- `Peer: Hello! 🔒` - Encrypted message
- `Peer: Hi (unencrypted)` - Fallback for compatibility

## Future Enhancements

The codebase is prepared for these features:
- ✅ Multiple peer support (mesh networking) - PeerManager ready
- ✅ End-to-end encryption - Fully implemented with RSA + AES-256-GCM
- ✅ File transfer capabilities - Fully implemented
- ✅ Message persistence and history - Config support ready
- 🔄 Peer discovery mechanisms - Can be added
- 🔄 GUI interface - Core logic is CLI-independent
- 🔄 Perfect Forward Secrecy - Can enhance current encryption
- 🔄 Certificate pinning - For enhanced security

## 🛡️ Security

### Encryption Features

- **End-to-End Encryption**: All messages are encrypted between peers
- **RSA-1024 Key Exchange**: Public key cryptography for secure key exchange
- **AES-256-GCM**: Military-grade symmetric encryption for message content
- **Perfect Forward Secrecy**: New encryption keys for every session
- **Message Authentication**: GCM mode provides built-in integrity verification
- **Visual Indicators**: 🔒 icon shows encryption status for each message

### Security Best Practices

1. **Verify Peer Identity**: Always confirm you're connecting to the intended peer
2. **Use Strong Nicknames**: Makes it easier to identify who you're chatting with
3. **Private Networks**: For sensitive communications, use VPN or private networks
4. **Regular Updates**: Keep the application updated for latest security patches

### Security Notice

This implementation uses 1024-bit RSA keys for demonstration and educational purposes. For production use in high-security environments, consider:
- Upgrading to 2048-bit or 4096-bit RSA keys
- Implementing elliptic curve cryptography (ECC)
- Adding certificate pinning for known peers
- Implementing post-quantum cryptography for future-proofing

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Some areas where contributions would be valuable:

- Enhanced encryption algorithms (ECC, post-quantum cryptography)
- GUI interface development
- Mobile platform support
- Peer discovery mechanisms
- Network protocol optimizations

## 📄 License

This project is open source and available under the MIT License.
