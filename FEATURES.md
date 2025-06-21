# Rust P2P Chat - Feature Documentation

## Overview

This document provides comprehensive technical documentation for all features implemented in the Rust P2P Chat application. The project includes 183+ tests across 10 categories, complete documentation coverage, and a macOS installer system. For quick start guides and basic usage, see the [README](Readme.md).

## Table of Contents

1. [Core Features](#core-features)
2. [Security & Encryption](#security--encryption)
3. [File Transfer](#file-transfer)
4. [Command System](#command-system)
5. [Configuration](#configuration)
6. [Installation & Distribution](#installation--distribution)
7. [Network Protocol](#network-protocol)
8. [Architecture](#architecture)
9. [Performance](#performance)
10. [Troubleshooting](#troubleshooting)
11. [Development Guide](#development-guide)

## Core Features

### True Peer-to-Peer Architecture

- **No Central Server**: Direct TCP connections between peers
- **Symmetric Design**: Both peers run identical code
- **Equal Capabilities**: After connection, both peers have the same features
- **Simultaneous Connect/Listen**: Can attempt outbound connection while accepting inbound

### Network Communication

- **Direct TCP Connections**: Low-latency, reliable communication
- **Binary Protocol**: Efficient message serialization with bincode
- **Backward Compatibility**: Falls back to plain text for legacy support
- **Stream Splitting**: Separate read/write halves for concurrent I/O
- **Automatic Reconnection**: Configurable retry attempts and delays

### Command System

Available commands during chat:

| Command | Description |
|---------|-------------|
| `/help` or `/?` | Display available commands |
| `/quit` or `/exit` | Exit the chat application |
| `/send <filename>` | Send a file to the peer |
| `/info` | Show connection information |
| `/nick <name>` | Set your nickname |
| `/autoopen` or `/auto` | Toggle auto-open for media files |

### File Transfer

- **Size Limit**: Default 100MB (configurable)
- **Hash Verification**: SHA256 integrity checking
- **Progress Tracking**: Real-time transfer progress
- **Auto-save**: Files saved to system Downloads folder or current directory
- **Auto-open Media**: Automatically open received media files (images, videos, audio, PDFs)
  - Can be toggled with `/autoopen` command
  - Platform-specific: Uses `open` on macOS, `start` on Windows, `xdg-open` on Linux
  - Supported formats: jpg, jpeg, png, gif, bmp, webp, svg, mp4, avi, mov, wmv, mp3, wav, flac, aac, pdf, doc, docx, txt

### Configuration System

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
download_dir = "/path/to/downloads"  # Optional, defaults to system Downloads folder
auto_open_media = true               # Automatically open received media files
media_extensions = ["jpg", "png", "mp4", "pdf"]  # File types to auto-open
```

### Graphical User Interface (GUI)

The application includes an optional GUI mode for enhanced user experience:

- **Native Window**: Cross-platform GUI using modern Rust GUI frameworks
- **Real-time Updates**: Messages appear instantly in the chat window
- **User-friendly Controls**: Buttons and text fields for easy interaction
- **Visual Design**: Clean interface with proper spacing and layout
- **Encryption Indicators**: Visual feedback for encryption status
- **File Transfer Support**: Drag-and-drop or browse for files to send

Launch GUI mode:
```bash
# Start GUI as listener
./rust-p2p-chat --gui --port 8080

# Start GUI and connect to peer
./rust-p2p-chat --gui --connect 192.168.1.100:8080
```

### End-to-End Encryption

The application now features military-grade end-to-end encryption:

- **1024-bit RSA Key Exchange**: Secure public key cryptography for initial handshake
- **AES-256-GCM Encryption**: Military-grade symmetric encryption for messages
- **Automatic Key Generation**: New encryption keys for every session
- **Message Authentication**: Built-in integrity verification with GCM
- **Visual Indicators**: 🔒 icon shows when messages are encrypted
- **Transparent Operation**: Encryption is automatic and requires no user configuration

### Command-Line Interface

```bash
# Basic usage
./rust-p2p-chat --port 8080
./rust-p2p-chat --connect 192.168.1.100:8080

# With options
./rust-p2p-chat --port 8080 --nickname Alice --debug
./rust-p2p-chat --connect 192.168.1.100:8080 --no-encryption

# Launch with GUI
./rust-p2p-chat --gui
./rust-p2p-chat --gui --connect 192.168.1.100:8080

# Generate config
./rust-p2p-chat config
```

CLI Options:
- `-p, --port <PORT>`: Port to listen on (default: 8080)
- `-c, --connect <ADDRESS>`: Peer address to connect to
- `-n, --nickname <NAME>`: Set your nickname
- `-d, --debug`: Enable debug logging
- `-g, --gui`: Launch graphical user interface
- `--no-encryption`: Disable encryption (not recommended)

### Error Handling

Custom error types provide clear, actionable error messages:
- `BindFailed`: Port already in use with diagnostic help
- `ConnectFailed`: Connection refused with fallback behavior
- `PeerDisconnected`: Clean disconnection handling
- `Protocol`: Message parsing errors
- `FileTransfer`: File-specific errors with size limits
- `Configuration`: Config file issues

### Logging

Configurable logging levels:
- `trace`: Very detailed debugging
- `debug`: Debugging information
- `info`: General information (default)
- `warn`: Warning messages
- `error`: Error messages only

## Security & Encryption

### Encryption Implementation

The application uses a hybrid encryption approach combining asymmetric and symmetric cryptography:

1. **Key Exchange Phase**:
   - Each peer generates a 1024-bit RSA keypair on startup
   - Public keys are exchanged automatically on connection
   - Keys are encoded in base64 for transmission

2. **Message Encryption**:
   - AES-256-GCM is used for message content
   - Each session generates a unique AES key
   - GCM mode provides authenticated encryption

3. **Security Features**:
   - Perfect Forward Secrecy: New keys for each session
   - Message Authentication: GCM prevents tampering
   - Visual Indicators: 🔒 shows encryption status
   - Automatic Fallback: Works with non-encrypted peers

### Security Considerations

- RSA-1024 is used for demonstration (upgrade to 2048+ for production)
- No certificate validation (consider adding for known peers)
- Keys are stored in memory only (not persisted)
- No protection against MITM attacks (consider adding key fingerprints)

## Installation & Distribution

### macOS Installer

The application includes a complete macOS installer system:

#### Features
- **Universal Binary**: Supports both Intel and Apple Silicon Macs
- **Standard DMG Package**: Drag-and-drop installation to Applications folder
- **App Bundle Structure**: Follows macOS app conventions with proper Info.plist
- **System Integration**: Integrates with macOS Downloads folder and file associations

#### Build Process
```bash
# Build macOS installer
./build-macos.sh

# Creates:
# - RustP2PChat.app (Universal app bundle)
# - RustP2PChat-0.1.0.dmg (Installer disk image)
```

#### Cross-Compilation Setup
```bash
# Add macOS targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Configure linkers in .cargo/config.toml
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin14-clang"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin14-clang"
```

#### Installation Process
1. **Download** the DMG file from releases
2. **Mount** the disk image
3. **Drag** RustP2PChat.app to Applications folder
4. **Launch** from Applications or Launchpad

#### Code Signing (Optional)
```bash
# Sign the app bundle
codesign --force --deep --sign "Developer ID Application: Your Name" RustP2PChat.app

# Sign the DMG
codesign --force --sign "Developer ID Application: Your Name" RustP2PChat-0.1.0.dmg
```

### Source Distribution

Alternative installation methods:

#### From Source
```bash
git clone https://github.com/cschladetsch/RustP2PChat.git
cd RustP2PChat
cargo build --release
```

#### Binary Releases
- Pre-compiled binaries for Windows, macOS, and Linux
- Available on GitHub releases page
- Automatic CI/CD builds for multiple platforms

## Network Protocol

### Message Types

The application uses a binary protocol with the following message types:

```rust
enum MessageType {
    Text(String),              // Plain text message
    EncryptedText(String),     // Base64 encoded encrypted text
    File(FileInfo),            // File transfer with metadata
    Command(Command),          // User commands
    Status(StatusUpdate),      // Connection status updates
    Heartbeat,                 // Keep-alive ping
    Acknowledgment(u64),       // Message delivery confirmation
    Encryption(EncryptionMessage), // Key exchange messages
}
```

### Protocol Flow

1. **Connection Establishment**:
   ```
   Peer A                    Peer B
   |------ TCP Connect ----->|
   |<--- TCP Accept ---------|
   |<--- PublicKey --------->|
   |<--- Encryption Ready -->|
   ```

2. **Message Exchange**:
   ```
   Serialize → Encrypt (optional) → Send → Receive → Decrypt → Deserialize
   ```

3. **File Transfer**:
   ```
   Command → Read File → Create FileInfo → Send → Receive → Verify Hash → Save → Auto-open
   ```

### Wire Format

Messages are serialized using bincode with the following structure:
- 8 bytes: Message ID (u64)
- 12 bytes: Timestamp (SystemTime)
- Variable: MessageType (bincode serialized)

For backward compatibility, plain text messages are sent as UTF-8 strings with newline terminators.

### Color Support

The application uses ANSI colors for better readability:
- **Green**: Your messages
- **Cyan**: Peer messages
- **Yellow**: Status messages
- **Red**: Error messages

## Architecture

### Module Overview

The application is structured with clear separation of concerns:

```
src/
├── main.rs              # CLI interface and entry point
├── lib.rs               # Core P2P chat implementation
├── protocol.rs          # Message types and serialization
├── config.rs            # Configuration management
├── file_transfer.rs     # File operations and auto-open
├── commands.rs          # Command parsing and handling
├── encryption.rs        # End-to-end encryption
├── peer.rs              # Peer management
├── error.rs             # Custom error types
└── colors.rs            # ANSI color codes
```

### Core Components

1. **P2PChat**: Main orchestrator that handles connection setup and management
2. **Message Protocol**: Type-safe message handling with serialization
3. **FileTransfer**: Secure file transfer with integrity verification
4. **E2EEncryption**: Hybrid encryption implementation
5. **CommandHandler**: Extensible command system
6. **Config**: Persistent configuration management

### Async Architecture

The application uses Tokio's async runtime with the following task structure:

```
Main Thread
├── Connection Handler
│   ├── Read Task (incoming messages)
│   ├── Write Task (outgoing messages)
│   └── Input Task (user input)
└── Encryption Handler (key exchange)
```

### Data Flow

1. **User Input** → Command Parser → Message Creation
2. **Message Creation** → Encryption (optional) → Serialization
3. **Network** → Deserialization → Decryption → Display/Action
4. **File Commands** → File Reading → Transfer → Auto-open

## Performance

### Optimization Features

- **8KB Buffer**: Larger buffer for efficient message handling
- **Async I/O**: Non-blocking operations with Tokio
- **Zero-copy**: Minimal memory allocations where possible
- **Stream Splitting**: Separate read/write for concurrent I/O
- **Lazy Loading**: Configuration loaded only when needed

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

### Common Issues

#### Port Already in Use
```
Error: Port 8080 is already in use!
```
**Solutions:**
- Find the process: `lsof -i :8080` (Linux/macOS) or `netstat -an | findstr :8080` (Windows)
- Use a different port: `--port 8081`
- Kill the process using the port

#### Connection Refused
The application automatically falls back to listening mode if initial connection fails.
- Verify the target peer is listening
- Check firewall settings
- Ensure correct IP address and port

#### File Transfer Issues
- **Large File Failed**: Check `max_file_size_mb` in config
- **Hash Mismatch**: Network corruption, retry the transfer
- **Auto-open Failed**: Verify the system has a default application for the file type

#### Encryption Problems
- **Key Exchange Failed**: Restart both peers
- **Messages Not Encrypted**: Check that both peers support encryption
- **Performance Issues**: Consider disabling encryption for local testing

#### Configuration Issues
- **Config Not Found**: Run `cargo run -- config` to generate default
- **Invalid Config**: Check TOML syntax and data types
- **Permissions**: Ensure write access to config directory

### Debug Mode

Enable detailed logging:
```bash
cargo run -- --debug
```

Or set log level in config:
```toml
log_level = "debug"
```

### Network Diagnostics

Test connectivity:
```bash
# Test if port is reachable
telnet <peer-ip> <port>

# Check listening ports
netstat -tuln | grep <port>

# Firewall status
sudo ufw status  # Ubuntu
sudo firewall-cmd --list-all  # CentOS/RHEL
```

## Development Guide

### Building the Project

```bash
# Clone repository
git clone https://github.com/cschladetsch/RustP2PChat.git
cd RustP2PChat

# Debug build (with debug symbols)
cargo build

# Release build (optimized)
cargo build --release

# Check for errors without building
cargo check

# Check with Clippy for code quality
cargo clippy
```

### Testing

The project includes comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test --lib                           # Library tests only
cargo test --test integration_tests        # Integration tests
cargo test encryption                      # Encryption-related tests

# Run tests with output
cargo test -- --nocapture

# Run tests with debug logging
RUST_LOG=debug cargo test -- --nocapture
```

### Test Categories

The project includes a **comprehensive test suite with 183+ individual tests** across 10 major categories, providing extensive validation of all functionality:

1. **File Transfer Tests** (9 tests):
   - Hash verification and integrity checking
   - Size limits and large file handling
   - Unicode filename support
   - Directory creation and path handling
   - Empty file edge cases

2. **Configuration Tests** (10 tests):
   - Default value validation
   - TOML serialization/deserialization
   - Path resolution and directory handling
   - Custom configuration scenarios
   - Validation and error handling

3. **Protocol Tests** (14 tests):
   - Message serialization for all types
   - Large data handling (1MB+ messages)
   - Unicode content support
   - Binary protocol compatibility
   - Edge case message handling

4. **Command Tests** (20 tests):
   - Command parsing with aliases
   - Parameter handling and validation
   - Error scenarios and edge cases
   - Help text generation
   - Command execution workflows

5. **Error Handling Tests** (34 tests):
   - All ChatError variant coverage
   - User-friendly error messages
   - Error source chain validation
   - Error display formatting
   - Edge case error scenarios

6. **Reliability Tests** (15 tests):
   - Message acknowledgment systems
   - Retry mechanisms and timeouts
   - Network failure simulation
   - Message ordering guarantees
   - Delivery confirmation

7. **Concurrent Tests** (7 tests):
   - Stress testing with 20+ connections
   - Race condition prevention
   - Graceful shutdown handling
   - Resource cleanup verification
   - Performance under load

8. **Peer Management Tests** (15 tests):
   - Concurrent peer access
   - IPv6 address support
   - Special character handling
   - Connection lifecycle management
   - Peer metadata tracking

9. **Encryption Tests** (39 tests):
   - End-to-end encryption workflows
   - RSA key exchange protocols
   - AES-256-GCM encryption/decryption
   - Key validation and verification
   - Security edge cases

10. **Integration Tests** (20 tests):
    - Real-world usage scenarios
    - File transfer workflows
    - Configuration persistence
    - Network simulation
    - System integration testing

**Legacy Tests**:
- **Unit Tests** (3): Core library functionality
- **Simple Integration Tests** (7): Basic connection and messaging

### Code Quality

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Generate documentation
cargo doc --open

# Check for unused dependencies
cargo machete
```

### Documentation System

The project includes comprehensive documentation at multiple levels:

#### **Inline Code Documentation**
- **Complete API Documentation**: All public modules, functions, and types have detailed rustdoc comments
- **Security Considerations**: Important security notes and best practices included
- **Code Examples**: Working examples for all major APIs
- **Error Documentation**: Detailed error types and handling patterns
- **Thread Safety**: Concurrency guarantees and usage patterns documented

**Module Documentation Coverage**:
- `src/lib.rs`: Core P2P chat implementation with connection handling
- `src/config.rs`: Configuration management and validation
- `src/error.rs`: Error handling and user-friendly error types
- `src/file_transfer.rs`: File transfer with integrity verification and security notes
- `src/colors.rs`: ANSI color support for terminal UI
- `src/commands.rs`: Command parsing and execution system
- `src/protocol.rs`: Message types and protocol definitions
- `src/peer.rs`: Peer management and connection tracking
- `src/reliability.rs`: Message reliability with acknowledgments
- `src/encryption.rs`: End-to-end encryption implementation

#### **Directory Documentation**
- **`src/README.md`**: Complete source code overview with architecture details
- **`tests/README.md`**: Test suite documentation with running instructions
- **`shell/README.md`**: Shell scripts documentation and usage guide

#### **Project Documentation**
- **`README.md`**: Main project documentation with quick start and examples
- **`FEATURES.md`**: Comprehensive technical feature documentation
- **`API.md`**: Developer API reference and integration guide
- **`DOCUMENTATION.md`**: Documentation overview and structure guide
- **`CHANGELOG.md`**: Version history and release notes
- **`macos-installer.md`**: macOS installer creation and deployment guide

#### **Generated Documentation**
```bash
# Generate and view rustdoc documentation
cargo doc --open

# Available at: target/doc/rust_p2p_chat/index.html
# Includes: All public APIs, examples, security notes, and implementation details
```

#### **Documentation Standards**
- **Working Examples**: All code examples are tested and functional
- **Cross-Platform Coverage**: Platform-specific instructions for Windows, macOS, Linux
- **Security Focus**: Security considerations prominently documented
- **Progressive Complexity**: Documentation flows from basic to advanced usage
- **Visual Navigation**: Emojis and clear structure for easy navigation

### Adding Features

1. **New Commands**: Add to `protocol.rs` Command enum and `commands.rs` handler
2. **Message Types**: Extend MessageType enum in `protocol.rs`
3. **Configuration**: Add fields to Config struct in `config.rs`
4. **Error Types**: Extend ChatError enum in `error.rs`

### Contributing Guidelines

1. Run tests before submitting: `cargo test`
2. Follow Rust formatting: `cargo fmt`
3. Address Clippy warnings: `cargo clippy`
4. Update documentation for new features
5. Add tests for new functionality

### Performance Profiling

```bash
# Build with profiling
cargo build --release --features=profiling

# Run with perf (Linux)
perf record --call-graph=dwarf ./target/release/rust-p2p-chat
perf report

# Memory profiling with valgrind
valgrind --tool=massif ./target/release/rust-p2p-chat
```