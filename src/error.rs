//! Error handling for the P2P chat application.
//!
//! This module provides a comprehensive error type system with user-friendly
//! error messages. All errors implement proper error chaining and provide
//! context-appropriate suggestions for resolution.
//!
//! # Features
//!
//! - User-friendly error messages
//! - Proper error source chaining
//! - Context-aware error descriptions
//! - Automatic conversion from standard library errors
//! - Categorized error types for different subsystems
//!
//! # Examples
//!
//! ```rust
//! use rust_p2p_chat::{ChatError, Result};
//!
//! fn example_function() -> Result<()> {
//!     // Operations that might fail
//!     Err(ChatError::PeerDisconnected)
//! }
//!
//! match example_function() {
//!     Ok(_) => println!("Success!"),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```

use std::error::Error;
use std::fmt;
use std::io;

/// Comprehensive error type for the P2P chat application.
///
/// `ChatError` provides detailed, user-friendly error messages for all
/// possible failure modes in the application. Each variant includes
/// context-specific information to help users understand and resolve issues.
///
/// # Error Categories
///
/// - **Network Errors**: Connection, binding, and communication failures
/// - **Protocol Errors**: Message parsing and protocol violations
/// - **Security Errors**: Encryption and authentication failures
/// - **File System Errors**: File transfer and configuration issues
/// - **System Errors**: Low-level I/O and resource errors
///
/// # User-Friendly Messages
///
/// All error messages are designed to be shown directly to users, with:
/// - Clear descriptions of what went wrong
/// - Suggestions for how to fix the issue
/// - Context about the specific operation that failed
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::ChatError;
/// use std::io;
///
/// // Network error with context
/// let error = ChatError::BindFailed(
///     "127.0.0.1:80".to_string(),
///     io::Error::from(io::ErrorKind::PermissionDenied)
/// );
/// println!("{}", error); // "Cannot use port 127.0.0.1:80 - permission denied (try a port above 1024)"
///
/// // Simple peer disconnection
/// let error = ChatError::PeerDisconnected;
/// println!("{}", error); // "Your chat partner disconnected"
/// ```
#[derive(Debug)]
pub enum ChatError {
    /// Generic I/O error from the standard library.
    ///
    /// This covers file system operations, network I/O, and other
    /// system-level errors. The inner `io::Error` provides specific
    /// details about what went wrong.
    Io(io::Error),
    
    /// Network connection error with descriptive message.
    ///
    /// Used for general connection issues that don't fit into
    /// more specific categories like `BindFailed` or `ConnectFailed`.
    Connection(String),
    
    /// Protocol-level error in message handling.
    ///
    /// Indicates problems with message parsing, serialization,
    /// or protocol violations between peers.
    Protocol(String),
    
    /// Invalid or corrupted message received.
    ///
    /// Used when a message is received but cannot be processed
    /// due to format issues or corruption.
    InvalidMessage(String),
    
    /// The peer has disconnected from the chat session.
    ///
    /// This is a normal termination condition and not necessarily
    /// an error, but requires handling in the application flow.
    PeerDisconnected,
    
    /// Failed to bind to the specified address and port.
    ///
    /// Contains the address that failed to bind and the underlying
    /// I/O error. Common causes include port already in use or
    /// insufficient permissions.
    BindFailed(String, io::Error),
    
    /// Failed to connect to the specified peer address.
    ///
    /// Contains the target address and the underlying I/O error.
    /// Common causes include peer not listening, network issues,
    /// or invalid addresses.
    ConnectFailed(String, io::Error),
    
    /// Encryption or decryption operation failed.
    ///
    /// Covers key generation, key exchange, message encryption,
    /// and decryption failures. May indicate compromised security.
    Encryption(String),
    
    /// File transfer operation failed.
    ///
    /// Covers file reading, writing, hash verification, and
    /// size limit violations during file transfer operations.
    FileTransfer(String),
    
    /// Configuration file operation failed.
    ///
    /// Covers loading, parsing, and saving configuration files.
    /// May indicate file corruption or permission issues.
    Configuration(String),
}

impl fmt::Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatError::Io(e) => match e.kind() {
                io::ErrorKind::PermissionDenied => {
                    write!(f, "Permission denied - check file/directory permissions")
                }
                io::ErrorKind::NotFound => write!(f, "File or directory not found"),
                io::ErrorKind::ConnectionRefused => {
                    write!(f, "Connection refused - peer may not be listening")
                }
                io::ErrorKind::ConnectionAborted => write!(f, "Connection lost unexpectedly"),
                io::ErrorKind::TimedOut => {
                    write!(f, "Operation timed out - check network connection")
                }
                _ => write!(f, "System error: {}", e),
            },
            ChatError::Connection(msg) => write!(f, "Network issue: {}", msg),
            ChatError::Protocol(msg) => {
                write!(f, "Communication error: {} (try reconnecting)", msg)
            }
            ChatError::InvalidMessage(msg) => write!(f, "Received corrupted data: {}", msg),
            ChatError::PeerDisconnected => write!(f, "Your chat partner disconnected"),
            ChatError::BindFailed(addr, e) => match e.kind() {
                io::ErrorKind::AddrInUse => {
                    write!(f, "Port {} is already in use - try a different port", addr)
                }
                io::ErrorKind::PermissionDenied => write!(
                    f,
                    "Cannot use port {} - permission denied (try a port above 1024)",
                    addr
                ),
                _ => write!(f, "Cannot start server on {}: {}", addr, e),
            },
            ChatError::ConnectFailed(addr, e) => match e.kind() {
                io::ErrorKind::ConnectionRefused => write!(
                    f,
                    "Cannot reach {} - peer may not be running or firewall blocking",
                    addr
                ),
                io::ErrorKind::TimedOut => write!(
                    f,
                    "Connection to {} timed out - check IP address and network",
                    addr
                ),
                io::ErrorKind::InvalidInput => write!(
                    f,
                    "Invalid address '{}' - use format IP:PORT (e.g., 192.168.1.100:8080)",
                    addr
                ),
                _ => write!(f, "Cannot connect to {}: {}", addr, e),
            },
            ChatError::Encryption(msg) => {
                write!(f, "Security error: {} (encryption may be compromised)", msg)
            }
            ChatError::FileTransfer(msg) => {
                if msg.contains("too large") {
                    write!(f, "File too large: {}", msg)
                } else if msg.contains("hash mismatch") {
                    write!(f, "File corrupted during transfer - please try again")
                } else if msg.contains("Failed to create") {
                    write!(
                        f,
                        "Cannot save file: {} (check disk space and permissions)",
                        msg
                    )
                } else {
                    write!(f, "File transfer failed: {}", msg)
                }
            }
            ChatError::Configuration(msg) => {
                if msg.contains("Failed to read") {
                    write!(f, "Cannot read settings: {} (file may be corrupted)", msg)
                } else if msg.contains("Failed to write") {
                    write!(f, "Cannot save settings: {} (check permissions)", msg)
                } else {
                    write!(f, "Settings error: {}", msg)
                }
            }
        }
    }
}

/// Implements the standard `Error` trait for `ChatError`.
///
/// This implementation provides proper error source chaining,
/// allowing users to access the underlying cause of errors
/// when available.
impl Error for ChatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChatError::Io(e) => Some(e),
            ChatError::BindFailed(_, e) | ChatError::ConnectFailed(_, e) => Some(e),
            _ => None,
        }
    }
}

/// Automatic conversion from `std::io::Error` to `ChatError`.
///
/// This allows using the `?` operator with I/O operations throughout
/// the codebase, automatically wrapping I/O errors in the `Io` variant.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::{ChatError, Result};
/// use std::fs;
///
/// fn read_file() -> Result<String> {
///     let content = fs::read_to_string("config.toml")?; // Auto-converts io::Error
///     Ok(content)
/// }
/// ```
impl From<io::Error> for ChatError {
    fn from(error: io::Error) -> Self {
        ChatError::Io(error)
    }
}

/// Type alias for `Result<T, ChatError>`.
///
/// This is a convenient shorthand used throughout the codebase
/// for functions that can fail with a `ChatError`. It follows
/// the same pattern as the standard library's `io::Result`.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::Result;
///
/// fn connect_to_peer(address: &str) -> Result<()> {
///     // Connection logic here
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, ChatError>;
