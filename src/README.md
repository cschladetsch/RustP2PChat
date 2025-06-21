# Source Code Documentation

This directory contains the core implementation of the Rust P2P Chat application. The codebase is organized into modular components that handle different aspects of peer-to-peer communication.

## 📁 Module Overview

### Core Modules

| Module | Purpose | Key Features |
|--------|---------|--------------|
| **`main.rs`** | Application entry point | CLI argument parsing, user interaction, main event loop |
| **`lib.rs`** | Core P2P implementation | Connection management, async message handling, peer coordination |
| **`protocol.rs`** | Message protocol | Message types, serialization, protocol definitions |
| **`peer.rs`** | Peer management | Peer connections, peer info, multi-peer support |
| **`encryption.rs`** | End-to-end encryption | RSA key exchange, AES-256-GCM encryption, digital signatures |
| **`config.rs`** | Configuration management | TOML config files, default settings, path resolution |
| **`error.rs`** | Error handling | Custom error types, user-friendly error messages |
| **`commands.rs`** | Command system | Chat commands, command parsing, handler dispatch |
| **`file_transfer.rs`** | File operations | File sending/receiving, progress tracking, hash verification |
| **`reliability.rs`** | Message reliability | Acknowledgments, retries, timeout handling |
| **`colors.rs`** | Terminal colors | ANSI color codes, styled output |

### Binary Modules

| Binary | Purpose | Description |
|--------|---------|-------------|
| **`bin/test_chat.rs`** | Encryption testing | Standalone encryption verification program |

## 🏗️ Architecture

### Core Design Principles

1. **Asynchronous First**: Built on Tokio runtime for high-performance async I/O
2. **Modular Design**: Each module has a single responsibility with clear interfaces
3. **Error Resilience**: Comprehensive error handling with graceful degradation
4. **Security Focus**: End-to-end encryption with proper key management
5. **Extensible**: Designed for future features like multi-peer support

### Message Flow

```
User Input → Commands → Protocol → Encryption → Network → Peer
     ↑                                                        ↓
Terminal ← Colors ← Error ← File Transfer ← Reliability ← Decryption
```

### Connection Lifecycle

1. **Initialization**: Configuration loading, key generation
2. **Connection**: TCP handshake, encryption key exchange
3. **Communication**: Message sending/receiving, file transfers
4. **Cleanup**: Graceful disconnection, resource cleanup

## 📋 Module Details

### `main.rs` - Application Entry Point
- **Purpose**: CLI interface and application lifecycle management
- **Key Functions**:
  - Command-line argument parsing with `clap`
  - User interaction for connection setup
  - Main application event loop
  - Signal handling for graceful shutdown
- **Dependencies**: `lib.rs`, `config.rs`, `colors.rs`

### `lib.rs` - Core P2P Implementation
- **Purpose**: Core peer-to-peer communication logic
- **Key Functions**:
  - `P2PChat` struct - main application state
  - Connection establishment (both server and client modes)
  - Async message handling loops
  - Peer connection management
- **Architecture**: Event-driven async design with separate read/write tasks

### `protocol.rs` - Message Protocol
- **Purpose**: Define message types and serialization format
- **Message Types**:
  - `Text` - Plain text messages
  - `EncryptedText` - Encrypted text messages
  - `File` - File transfer messages
  - `Command` - Chat commands
  - `Status` - Status updates
  - `Heartbeat` - Keep-alive messages
  - `Acknowledgment` - Message confirmations
  - `Encryption` - Key exchange messages
- **Serialization**: Uses `bincode` for efficient binary encoding

### `peer.rs` - Peer Management
- **Purpose**: Manage peer connections and information
- **Key Components**:
  - `PeerInfo` - Peer metadata (ID, nickname, address, connection time)
  - `Peer` - Active peer connection with stream and channels
  - `PeerManager` - Manages multiple peer connections
- **Features**: Concurrent access, IPv6 support, connection tracking

### `encryption.rs` - End-to-End Encryption
- **Purpose**: Provide secure communication between peers
- **Encryption Stack**:
  - **RSA-1024**: Public key cryptography for key exchange
  - **AES-256-GCM**: Symmetric encryption for messages
  - **Digital Signatures**: Message authentication with RSA-PKCS1v15
- **Key Features**:
  - Automatic key generation and exchange
  - Perfect forward secrecy (new keys per session)
  - Message integrity verification
  - TLS transport layer security

### `config.rs` - Configuration Management
- **Purpose**: Application configuration and settings
- **Configuration Options**:
  - User preferences (nickname, default port)
  - Network settings (buffer size, timeouts)
  - Security settings (encryption enabled)
  - File transfer settings (max size, auto-open)
  - Logging configuration
- **File Format**: TOML with platform-specific paths

### `error.rs` - Error Handling
- **Purpose**: Centralized error management with user-friendly messages
- **Error Types**:
  - `Io` - I/O and network errors
  - `Connection` - Connection-specific errors
  - `Protocol` - Protocol parsing errors
  - `InvalidMessage` - Message validation errors
  - `PeerDisconnected` - Peer disconnection
  - `BindFailed` - Port binding failures
  - `ConnectFailed` - Connection failures
  - `Encryption` - Encryption/decryption errors
  - `FileTransfer` - File operation errors
  - `Configuration` - Config file errors

### `commands.rs` - Command System
- **Purpose**: Handle chat commands and system operations
- **Commands Supported**:
  - `/help` - Show available commands
  - `/quit` - Exit the application
  - `/send <file>` - Send a file
  - `/nick <name>` - Set nickname
  - `/info` - Show connection info
  - `/autoopen` - Toggle media auto-open
  - `/peers` - List connected peers
  - `/stats` - Show reliability statistics
- **Architecture**: Command parsing with async handler dispatch

### `file_transfer.rs` - File Operations
- **Purpose**: Handle file sending and receiving with verification
- **Key Features**:
  - File preparation with metadata
  - SHA-256 hash verification
  - Progress tracking
  - Size limit enforcement
  - Unicode filename support
  - Automatic directory creation
- **Workflow**: Prepare → Send → Verify → Save

### `reliability.rs` - Message Reliability
- **Purpose**: Ensure message delivery with acknowledgments and retries
- **Features**:
  - Message acknowledgment system
  - Automatic retry with exponential backoff
  - Timeout handling
  - Delivery statistics
  - Old message cleanup
- **Configuration**: Retry attempts, delays, timeouts

### `colors.rs` - Terminal Colors
- **Purpose**: ANSI color codes for enhanced terminal output
- **Color Support**:
  - User messages (green)
  - Peer messages (cyan)
  - System messages (yellow)
  - Error messages (red)
  - Encryption indicators (lock icon)

## 🔧 Development Guidelines

### Code Style
- Follow Rust standard conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Document all public APIs

### Error Handling
- Use `Result<T, ChatError>` for fallible operations
- Provide user-friendly error messages
- Log technical details at appropriate levels
- Handle all error cases gracefully

### Async Programming
- Use `async/await` for I/O operations
- Avoid blocking operations in async context
- Use channels for communication between tasks
- Handle cancellation and timeouts properly

### Testing
- Write unit tests for individual functions
- Create integration tests for end-to-end flows
- Test error conditions and edge cases
- Use temporary files and directories for file operations

### Security
- Never log sensitive information (keys, passwords)
- Validate all user inputs
- Use secure random number generation
- Follow cryptographic best practices

## 🚀 Adding New Features

### Adding a New Message Type
1. Add variant to `MessageType` enum in `protocol.rs`
2. Update serialization/deserialization
3. Add handling in message processing loop
4. Update tests

### Adding a New Command
1. Add variant to `Command` enum in `protocol.rs`
2. Add parsing logic in `commands.rs`
3. Implement handler function
4. Add help text and documentation
5. Write tests

### Adding a New Configuration Option
1. Add field to `Config` struct in `config.rs`
2. Update `Default` implementation
3. Add validation if needed
4. Update documentation and examples

## 📈 Performance Considerations

### Memory Usage
- Use `Arc` for shared data structures
- Minimize cloning of large data structures
- Stream large file transfers
- Clean up resources promptly

### Network Performance
- Use buffered I/O for small messages
- Implement message batching for high throughput
- Use connection pooling for multiple peers
- Consider compression for large messages

### CPU Performance
- Use efficient serialization (bincode)
- Minimize encryption overhead
- Cache computed values
- Use parallel processing where appropriate

## 🔍 Debugging

### Logging
- Use `tracing` for structured logging
- Log at appropriate levels (trace, debug, info, warn, error)
- Include relevant context in log messages
- Use spans for request tracing

### Common Issues
- **Connection failures**: Check firewall and network settings
- **Port already in use**: Verify no other instance is running
- **Encryption errors**: Check key exchange and peer compatibility
- **File transfer failures**: Verify permissions and disk space

### Debugging Tools
- `cargo test` - Run test suite
- `cargo check` - Type checking
- `cargo clippy` - Linting
- `RUST_LOG=debug cargo run` - Enable debug logging

## 📚 Further Reading

- [Tokio Documentation](https://tokio.rs/)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)
- [Cryptography in Rust](https://github.com/RustCrypto)
- [Serde Serialization](https://serde.rs/)
- [Tracing Documentation](https://tracing.rs/)