# Rust P2P Chat - API Documentation

## Overview

This document provides technical API documentation for developers who want to integrate with or extend the Rust P2P Chat application. The project includes comprehensive inline documentation for all modules and APIs.

## Documentation Status

✅ **Complete API Documentation**: All public modules, functions, and types have detailed rustdoc comments with examples  
✅ **Security Documentation**: Security considerations and best practices included throughout  
✅ **Thread Safety Documentation**: Concurrency guarantees and usage patterns documented  
✅ **Error Handling Documentation**: Detailed error types and patterns with examples  
✅ **Working Code Examples**: All examples are tested and functional  

**Generate API Documentation**:
```bash
cargo doc --open
# View at: target/doc/rust_p2p_chat/index.html
```

## Core Types

### Message Protocol

The application uses a strongly-typed message protocol defined in `src/protocol.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub timestamp: SystemTime,
    pub msg_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text(String),
    EncryptedText(String),
    File(FileInfo),
    Command(Command),
    Status(StatusUpdate),
    Heartbeat,
    Acknowledgment(u64),
    Encryption(EncryptionMessage),
}
```

### File Transfer

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub hash: String,    // SHA256 hash
    pub data: Vec<u8>,   // File contents
}
```

### Commands

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Quit,
    Help,
    Info,
    ListPeers,
    SendFile(String),
    SetNickname(String),
    ToggleAutoOpen,
}
```

### Configuration

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub nickname: Option<String>,
    pub default_port: u16,
    pub buffer_size: usize,
    pub heartbeat_interval_secs: u64,
    pub reconnect_attempts: u32,
    pub reconnect_delay_secs: u64,
    pub enable_encryption: bool,
    pub log_level: String,
    pub save_history: bool,
    pub history_file: Option<PathBuf>,
    pub max_file_size_mb: u64,
    pub download_dir: Option<PathBuf>,
    pub auto_open_media: bool,
    pub media_extensions: Vec<String>,
}
```

## Core APIs

### P2PChat Main Interface

```rust
impl P2PChat {
    pub fn new(config: Config) -> Result<Self>;
    pub async fn start(&mut self, port: u16, connect_addr: Option<String>) -> Result<()>;
}
```

### File Transfer API

```rust
impl FileTransfer {
    pub fn new(max_file_size_mb: u64) -> Self;
    
    pub async fn prepare_file(&self, path: &Path) -> Result<FileInfo>;
    pub async fn save_file(&self, file_info: &FileInfo, download_dir: &Path) -> Result<PathBuf>;
    
    pub fn open_file(path: &Path) -> Result<()>;
    pub fn is_media_file(filename: &str, media_extensions: &[String]) -> bool;
}
```

### Configuration API

```rust
impl Config {
    pub fn load() -> Result<Self>;
    pub fn save(&self) -> Result<()>;
    pub fn download_path(&self) -> PathBuf;
    pub fn history_path(&self) -> Option<PathBuf>;
}
```

### Encryption API

```rust
impl E2EEncryption {
    pub fn new() -> Result<Self>;
    pub fn generate_keypair(&mut self) -> Result<()>;
    pub fn get_public_key_base64(&self) -> Result<String>;
    pub fn set_peer_public_key(&mut self, key: &str) -> Result<()>;
    pub fn generate_shared_key(&self) -> Result<String>;
    pub fn encrypt_message(&self, plaintext: &str) -> Result<String>;
    pub fn decrypt_message(&self, encrypted: &str) -> Result<String>;
    pub fn is_ready(&self) -> bool;
}
```

### Command Handler API

```rust
impl CommandHandler {
    pub fn new(config: Config) -> Self;
    pub fn parse_command(input: &str) -> Option<Command>;
    pub async fn handle_command(&mut self, command: Command, peer_manager: &PeerManager) -> Result<String>;
}
```

## Network Protocol Specification

### Connection Flow

1. **TCP Connection**: Standard TCP handshake
2. **Encryption Handshake** (if enabled):
   - Peer A sends RSA public key
   - Peer B sends RSA public key
   - Both peers generate shared AES key
   - Encryption ready signal
3. **Message Exchange**: Binary or text protocol

### Message Serialization

Messages are serialized using `bincode` with the following format:

```
[8 bytes: Message ID][12 bytes: Timestamp][Variable: MessageType]
```

For backward compatibility, plain text messages use UTF-8 encoding with newline terminators.

### File Transfer Protocol

1. **Initiate**: `/send <filename>` command
2. **Prepare**: Read file, calculate SHA256 hash
3. **Send**: FileInfo message with complete file data
4. **Receive**: Verify hash, save file, optionally auto-open
5. **Confirm**: Status update with success/failure

## Error Handling

### Custom Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("Failed to bind to port {port}: {source}")]
    BindFailed { port: u16, source: std::io::Error },
    
    #[error("Failed to connect to peer: {0}")]
    ConnectFailed(String),
    
    #[error("Peer disconnected")]
    PeerDisconnected,
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("File transfer error: {0}")]
    FileTransfer(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Extension Points

### Adding New Commands

1. Add variant to `Command` enum in `protocol.rs`
2. Add parsing logic in `CommandHandler::parse_command`
3. Add handling logic in `CommandHandler::handle_command`
4. Update help text

### Adding New Message Types

1. Add variant to `MessageType` enum in `protocol.rs`
2. Add handling in `handle_message` function in `lib.rs`
3. Add serialization/deserialization support

### Custom File Handlers

The `FileTransfer::open_file` function can be extended to support custom file handlers based on file type or content.

### Encryption Plugins

The `E2EEncryption` trait can be implemented for different encryption backends:

```rust
trait Encryption {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn key_exchange(&mut self, peer_key: &[u8]) -> Result<()>;
}
```

## Testing APIs

### Test Utilities

```rust
// Create test configuration
fn test_config() -> Config;

// Create test peer
async fn create_test_peer(port: u16) -> Result<P2PChat>;

// Send test message
async fn send_test_message(peer: &mut P2PChat, message: &str) -> Result<()>;
```

### Integration Test Framework

The project includes comprehensive integration tests that demonstrate API usage:

- `tests/integration_tests.rs`: Full communication scenarios
- `tests/simple_integration_test.rs`: Basic connection tests

## Performance Considerations

### Buffer Management

- Default buffer size: 8KB (configurable)
- Zero-copy operations where possible
- Stream splitting for concurrent I/O

### Async Architecture

- Built on Tokio runtime
- Non-blocking operations
- Concurrent read/write streams

### Memory Usage

- Minimal allocations for message passing
- File data temporarily held in memory during transfer
- Configurable limits for file sizes

## Security Considerations

### Encryption

- RSA-1024 for key exchange (consider upgrading to 2048+ for production)
- AES-256-GCM for message content
- Perfect forward secrecy with session keys

### File Transfer

- SHA256 integrity verification
- Configurable file size limits
- Path traversal protection

### Network Security

- Direct TCP connections (no intermediary servers)
- Optional encryption (can be disabled for testing)
- No built-in authentication (rely on network security)

## Examples

### Basic Usage

```rust
use rust_p2p_chat::{P2PChat, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let mut chat = P2PChat::new(config)?;
    chat.start(8080, None).await?;
    Ok(())
}
```

### Custom Configuration

```rust
let mut config = Config::default();
config.nickname = Some("Alice".to_string());
config.max_file_size_mb = 200;
config.auto_open_media = false;

let mut chat = P2PChat::new(config)?;
```

### File Transfer

```rust
use rust_p2p_chat::FileTransfer;
use std::path::Path;

let transfer = FileTransfer::new(100);
let file_info = transfer.prepare_file(Path::new("document.pdf")).await?;
// Send file_info via protocol
```

## Installation & Distribution

### macOS Installer API

The application includes macOS installer functionality accessible through the build system:

```bash
# Build macOS installer
./build-macos.sh

# Creates:
# - RustP2PChat.app (Universal app bundle)
# - RustP2PChat-0.1.0.dmg (Installer disk image)
```

### Cross-Platform Build Configuration

```toml
# .cargo/config.toml
[target.x86_64-apple-darwin]
linker = "x86_64-apple-darwin14-clang"

[target.aarch64-apple-darwin]
linker = "aarch64-apple-darwin14-clang"
```

### App Bundle Structure

```
RustP2PChat.app/
├── Contents/
│   ├── Info.plist          # App metadata
│   ├── MacOS/
│   │   └── RustP2PChat     # Universal binary
│   └── Resources/          # App resources
```

## CLI Integration

The application provides a complete CLI interface in `src/main.rs` that demonstrates integration with all APIs.

### CLI Arguments

```bash
rust-p2p-chat [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -p, --port <PORT>              Port to listen on [default: 8080]
    -c, --connect <ADDRESS>        Peer address to connect to
    -n, --nickname <NAME>          Set your nickname
    -d, --debug                    Enable debug logging
        --no-encryption            Disable encryption

SUBCOMMANDS:
    config    Generate and save default configuration
```

## Module Documentation

All source modules include comprehensive inline documentation. Key modules:

### Core Modules
- **`src/lib.rs`**: Core P2P chat implementation with connection handling and encryption integration
- **`src/main.rs`**: CLI interface and application entry point with argument parsing
- **`src/config.rs`**: Configuration management with TOML serialization and validation
- **`src/error.rs`**: Custom error types with user-friendly messages and error chains

### Communication Modules  
- **`src/protocol.rs`**: Message types and protocol definitions with serialization
- **`src/peer.rs`**: Peer management and connection tracking with thread-safe operations
- **`src/reliability.rs`**: Message reliability with acknowledgments and retry mechanisms
- **`src/commands.rs`**: Command parsing and execution with alias support

### Feature Modules
- **`src/file_transfer.rs`**: File transfer with SHA-256 verification and cross-platform file opening
- **`src/encryption.rs`**: End-to-end encryption using RSA + AES-256-GCM
- **`src/colors.rs`**: ANSI color support for enhanced terminal output

### Documentation Access
```bash
# Generate complete API documentation
cargo doc --open

# Documentation includes:
# - Security considerations for all modules
# - Working code examples 
# - Thread safety guarantees
# - Error handling patterns
# - Cross-platform compatibility notes
```