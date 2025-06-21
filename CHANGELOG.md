# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2025-06-22

### Added
- **Graphical User Interface (GUI)**: Optional GUI mode with --gui flag
  - Cross-platform native window interface
  - Real-time message updates in chat window
  - User-friendly controls with buttons and text fields
  - Visual encryption status indicators
  - File transfer support with GUI controls
  - Launch with `--gui` flag for enhanced user experience

- **Shell Script Helpers**: Convenience scripts for building and running
  - `./b` - Quick build script
  - `./r` - Build and run script that passes all arguments to the application
  - Streamlined development workflow
- **macOS Installer System**: Complete macOS distribution support
  - Universal binary supporting both Intel and Apple Silicon Macs
  - Standard DMG installer with drag-to-Applications interface
  - Cross-compilation setup with `build-macos.sh` script
  - App bundle structure following macOS conventions
  - Support for code signing and notarization (optional)
  - Automatic system integration (Downloads folder, file associations)

- **Comprehensive Test Suite**: 183+ individual tests across 10 categories
  - File Transfer Tests (9): Hash verification, size limits, unicode handling
  - Configuration Tests (10): Defaults, validation, serialization
  - Protocol Tests (14): Message types, serialization, large data
  - Command Tests (20): Parsing, handling, edge cases
  - Error Handling Tests (34): All error types, user-friendly messages
  - Reliability Tests (15): Acknowledgments, retries, timeouts
  - Concurrent Tests (7): Stress testing, graceful shutdown
  - Peer Management Tests (15): Concurrent access, IPv6 support
  - Encryption Tests (39): E2E encryption, key exchange, signing
  - Integration Tests (20): Real-world scenarios, file workflows

- **Comprehensive Documentation System**: Complete documentation overhaul at all levels
  - **Inline Code Documentation**: All modules, functions, and types have detailed rustdoc comments
    - Security considerations and best practices included throughout
    - Working code examples for all major APIs  
    - Thread safety and concurrency documentation
    - Detailed error handling patterns and examples
  - **Module Documentation**: Complete coverage of all source modules
    - `src/lib.rs`, `src/config.rs`, `src/error.rs`, `src/file_transfer.rs`
    - `src/colors.rs`, `src/commands.rs`, `src/protocol.rs`, `src/peer.rs`
    - `src/reliability.rs`, `src/encryption.rs` with security focus
  - **Directory Documentation**: README files for major directories
    - `src/README.md`: Source code architecture and development guide
    - `tests/README.md`: Test suite documentation with running instructions
    - `shell/README.md`: Shell scripts documentation and usage guide
  - **Project Documentation**: Enhanced core documentation files
    - Enhanced README with macOS installer instructions and test coverage
    - Comprehensive FEATURES.md with installation & distribution section
    - Updated API.md with documentation status and rustdoc generation
    - Complete DOCUMENTATION.md overview and navigation guide
    - Platform-specific build instructions for Windows, Ubuntu/WSL, and macOS
    - Security best practices and considerations throughout
    - Performance optimization guidelines and profiling instructions

- **Auto-open Media Files**: Automatically open received media files after download
  - Platform-specific implementation (macOS: `open`, Windows: `start`, Linux: `xdg-open`)
  - Configurable via `/autoopen` command during chat
  - Support for images (jpg, png, gif, etc.), videos (mp4, avi, etc.), audio, and documents
  - Files are saved to system Downloads folder by default
  - New config options: `auto_open_media`, `download_dir`, `media_extensions`

- **End-to-End Encryption**: Full implementation of 1024-bit RSA + AES-256-GCM encryption
  - Automatic key exchange on connection
  - Visual encryption indicators (🔒 icon)
  - Transparent encryption for all messages
  - Session-based key generation for perfect forward secrecy

- **Enhanced Command System**: Extended command functionality
  - `/autoopen` or `/auto` - Toggle auto-open for media files
  - Improved `/info` command with comprehensive system information
  - Better error handling and user feedback

- **Configuration Enhancements**: Extended configuration options
  - `download_dir` - Custom download directory
  - `auto_open_media` - Toggle auto-open behavior
  - `media_extensions` - Configurable file types for auto-open
  - Better default values and validation

- **Test Improvements**: Enhanced test coverage and quality
  - Updated to use idiomatic Rust struct initialization
  - Fixed clippy warnings (using `is_none_or` instead of `map_or`)
  - Added `quick_test.sh` script for rapid testing
  - Comprehensive integration test coverage

### Changed
- Improved code quality with clippy recommendations
- Updated integration tests for better maintainability
- File transfer now saves to system Downloads folder instead of local `downloads/` directory
- Enhanced error messages and user feedback
- Better command handling and state management

### Security
- Added security best practices documentation
- Enhanced encryption documentation with implementation details
- Security considerations for production deployment
- Recommendations for key size upgrades

## [0.2.0] - 2024-12-22

### Added
- Custom error types (`ChatError`) for better error handling and debugging
- Binary message protocol supporting multiple message types:
  - Text messages (backward compatible)
  - File transfers with SHA256 verification
  - Commands (help, quit, send, info, nickname)
  - Status updates and progress tracking
  - Heartbeat for connection monitoring
- File transfer capability:
  - Send files up to 100MB (configurable)
  - Automatic integrity verification
  - Progress tracking
  - Auto-save to downloads directory
- Command system with built-in commands:
  - `/help` or `/?` - Show available commands
  - `/quit` or `/exit` - Exit gracefully
  - `/send <file>` - Transfer files
  - `/info` - Display connection information
  - `/nick <name>` - Set nickname
- Configuration file support (TOML):
  - Customizable settings (buffer size, timeouts, etc.)
  - CLI arguments override config values
  - `config` subcommand to generate default configuration
- Enhanced CLI with clap:
  - `--port` - Specify listening port
  - `--connect` - Connect to peer address
  - `--nickname` - Set display name
  - `--debug` - Enable debug logging
  - `--no-encryption` - Disable TLS (future use)
- Logging system with configurable levels (trace, debug, info, warn, error)
- Increased message buffer from 1KB to 8KB (configurable)
- Connection heartbeat mechanism (30s default interval)
- TLS encryption infrastructure (ready for activation)
- Comprehensive test suite updates

### Changed
- Improved error messages with actionable diagnostics
- Enhanced connection handling with simultaneous accept/connect
- Better resource cleanup on disconnection
- More robust message parsing with fallback to plain text
- Updated documentation with all new features

### Technical Improvements
- Replaced `Box<dyn Error>` with specific error types
- Implemented proper async patterns throughout
- Added module separation for better code organization
- Prepared foundation for multi-peer support
- Stream splitting for efficient concurrent I/O

## [0.1.0] - Previous Version

### Features
- Basic P2P chat functionality
- Direct TCP connections
- Real-time bidirectional messaging
- Simple console interface
- Cross-platform support