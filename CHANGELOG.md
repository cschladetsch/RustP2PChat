# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2025-06-21

### Added
- **End-to-End Encryption**: Full implementation of 1024-bit RSA + AES-256-GCM encryption
  - Automatic key exchange on connection
  - Visual encryption indicators (🔒 icon)
  - Transparent encryption for all messages
  - Session-based key generation for perfect forward secrecy
- **Test Improvements**: Enhanced test coverage and quality
  - Updated to use idiomatic Rust struct initialization
  - Fixed clippy warnings (using `is_none_or` instead of `map_or`)
  - Added `quick_test.sh` script for rapid testing
- **Documentation Updates**: Comprehensive documentation improvements
  - Added security notices about RSA key sizes
  - Enhanced README with key highlights section
  - Updated test coverage information
  - Added encryption test examples

### Changed
- Improved code quality with clippy recommendations
- Updated integration tests for better maintainability

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