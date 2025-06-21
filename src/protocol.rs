//! Protocol definitions for P2P chat communication.
//!
//! This module defines the core messaging protocol used for communication between
//! peers in the P2P chat network. It includes message types, serialization,
//! encryption support, and file transfer protocols.
//!
//! # Protocol Overview
//!
//! The protocol supports several types of messages:
//! - **Text Messages**: Plain text chat messages
//! - **Encrypted Messages**: End-to-end encrypted text using AES-256-GCM
//! - **File Transfer**: Binary file data with integrity verification
//! - **Commands**: Special control messages for chat functionality
//! - **Status Updates**: Connection and system status notifications
//! - **Heartbeats**: Keep-alive messages for connection monitoring
//! - **Acknowledgments**: Message delivery confirmations
//! - **Encryption**: Key exchange and encryption setup messages
//!
//! # Serialization
//!
//! All messages are serialized using bincode for efficient binary encoding.
//! This provides fast serialization/deserialization and compact message sizes.
//!
//! # Security
//!
//! - Encrypted messages use Base64 encoding for text representation
//! - File transfers include SHA-256 hashes for integrity verification
//! - Encryption keys are exchanged using RSA public key cryptography
//! - All sensitive data is properly encrypted before transmission
//!
//! # Examples
//!
//! ```rust
//! use rust_p2p_chat::protocol::{Message, MessageType, Command};
//!
//! // Create a text message
//! let msg = Message::new_text("Hello, world!".to_string());
//!
//! // Serialize for transmission
//! let serialized = msg.serialize().unwrap();
//!
//! // Deserialize received data
//! let received = Message::deserialize(&serialized).unwrap();
//!
//! // Handle different message types
//! match received.msg_type {
//!     MessageType::Text(content) => println!("Received: {}", content),
//!     MessageType::EncryptedText(encrypted) => println!("Encrypted message received"),
//!     MessageType::Command(cmd) => println!("Command: {:?}", cmd),
//!     _ => println!("Other message type"),
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Enumeration of all supported message types in the P2P chat protocol.
///
/// Each variant represents a different type of communication that can occur
/// between peers. Messages are designed to be serializable and support both
/// encrypted and unencrypted communication.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::protocol::{MessageType, Command};
///
/// // Text message
/// let text_msg = MessageType::Text("Hello!".to_string());
///
/// // Command message
/// let cmd_msg = MessageType::Command(Command::Help);
///
/// // Heartbeat for keep-alive
/// let heartbeat = MessageType::Heartbeat;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    /// Plain text chat message.
    Text(String),
    /// Base64 encoded encrypted text message using AES-256-GCM.
    EncryptedText(String),
    /// File transfer data with metadata and integrity verification.
    File(FileInfo),
    /// Command execution request (e.g., /help, /quit).
    Command(Command),
    /// Status update notification (connection events, progress, etc.).
    Status(StatusUpdate),
    /// Keep-alive message for connection monitoring.
    Heartbeat,
    /// Message delivery acknowledgment containing the original message ID.
    Acknowledgment(u64),
    /// Encryption-related messages for key exchange and setup.
    Encryption(EncryptionMessage),
}

/// Core message structure for P2P chat communication.
///
/// Every message in the chat protocol is wrapped in this structure, which provides
/// unique identification, timing information, and the actual message content.
///
/// # Fields
///
/// - `id`: Unique identifier for message deduplication and acknowledgments
/// - `timestamp`: When the message was created (used for ordering and debugging)
/// - `msg_type`: The actual message content and type
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::protocol::{Message, MessageType};
///
/// // Create a text message
/// let msg = Message::new_text("Hello, world!".to_string());
/// println!("Message ID: {}", msg.id);
///
/// // Serialize for network transmission
/// let bytes = msg.serialize().unwrap();
///
/// // Deserialize received data
/// let received = Message::deserialize(&bytes).unwrap();
/// assert_eq!(msg.id, received.id);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier for deduplication and acknowledgments.
    pub id: u64,
    /// Timestamp when the message was created.
    pub timestamp: SystemTime,
    /// The message content and type.
    pub msg_type: MessageType,
}

/// File transfer information with integrity verification.
///
/// Contains all necessary data for transferring files between peers, including
/// the file content, metadata, and a SHA-256 hash for integrity verification.
///
/// # Security
///
/// The SHA-256 hash ensures that files are not corrupted during transfer.
/// Recipients verify the hash before saving the file to disk.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::protocol::FileInfo;
///
/// let file_info = FileInfo {
///     name: "document.pdf".to_string(),
///     size: 1024,
///     hash: "abc123...".to_string(),  // SHA-256 hash
///     data: vec![0; 1024],
/// };
///
/// println!("File: {} ({} bytes)", file_info.name, file_info.size);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    /// Original filename including extension.
    pub name: String,
    /// File size in bytes.
    pub size: u64,
    /// SHA-256 hash of the file data for integrity verification.
    pub hash: String,
    /// Complete file contents as bytes.
    pub data: Vec<u8>,
}

/// Available chat commands that can be executed by users.
///
/// Commands allow users to control the chat application, get information,
/// and modify settings. Commands are typically triggered by typing text
/// that starts with '/' in the chat interface.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::protocol::Command;
///
/// // Different types of commands
/// let help_cmd = Command::Help;
/// let quit_cmd = Command::Quit;
/// let nick_cmd = Command::SetNickname("Alice".to_string());
/// let file_cmd = Command::SendFile("/path/to/file.txt".to_string());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Command {
    /// Exit the chat application.
    Quit,
    /// Display help information with available commands.
    Help,
    /// Show connection and configuration information.
    Info,
    /// List all currently connected peers.
    ListPeers,
    /// Send a file to connected peers (contains file path).
    SendFile(String),
    /// Set or change the user's nickname.
    SetNickname(String),
    /// Toggle automatic opening of received media files.
    ToggleAutoOpen,
    /// Display message reliability and connection statistics.
    Stats,
}

/// Status update messages for system events and notifications.
///
/// These messages inform users about important system events such as peer
/// connections, file transfer progress, and encryption status changes.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::protocol::StatusUpdate;
///
/// // Peer events
/// let connected = StatusUpdate::PeerConnected("Alice".to_string());
/// let disconnected = StatusUpdate::PeerDisconnected("Bob".to_string());
///
/// // File transfer progress (filename, current_bytes, total_bytes)
/// let progress = StatusUpdate::TransferProgress("image.jpg".to_string(), 1024, 2048);
///
/// // Security events
/// let encryption_on = StatusUpdate::EncryptionEnabled;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StatusUpdate {
    /// A new peer has connected (contains peer identifier).
    PeerConnected(String),
    /// A peer has disconnected (contains peer identifier).
    PeerDisconnected(String),
    /// File transfer progress update (filename, bytes_transferred, total_bytes).
    TransferProgress(String, u64, u64),
    /// Error message for display to the user.
    Error(String),
    /// End-to-end encryption has been successfully enabled.
    EncryptionEnabled,
    /// Encryption has been disabled or failed to establish.
    EncryptionDisabled,
}

/// Encryption-related messages for secure key exchange.
///
/// These messages handle the establishment of end-to-end encryption between peers
/// using a hybrid cryptographic approach (RSA + AES-256-GCM).
///
/// # Security Protocol
///
/// 1. Peers exchange RSA public keys using `PublicKeyExchange`
/// 2. One peer generates an AES-256 key and sends it encrypted with the other's public key
/// 3. Both peers confirm successful setup with `HandshakeComplete`
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::protocol::EncryptionMessage;
///
/// // Step 1: Exchange public keys
/// let pub_key_msg = EncryptionMessage::PublicKeyExchange("base64_public_key".to_string());
///
/// // Step 2: Share encrypted AES key
/// let shared_key_msg = EncryptionMessage::SharedKeyExchange("encrypted_aes_key".to_string());
///
/// // Step 3: Confirm handshake
/// let complete_msg = EncryptionMessage::HandshakeComplete;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EncryptionMessage {
    /// RSA public key exchange (Base64 encoded public key).
    PublicKeyExchange(String),
    /// AES-256 key encrypted with peer's RSA public key (Base64 encoded).
    SharedKeyExchange(String),
    /// Confirmation that encryption handshake is complete.
    HandshakeComplete,
}

impl Message {
    /// Creates a new text message with a unique ID and current timestamp.
    ///
    /// # Arguments
    ///
    /// * `text` - The message content to send
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::protocol::Message;
    ///
    /// let msg = Message::new_text("Hello, world!".to_string());
    /// println!("Message ID: {}", msg.id);
    /// ```
    pub fn new_text(text: String) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Text(text),
        }
    }

    /// Creates a new acknowledgment message for the given message ID.
    ///
    /// # Arguments
    ///
    /// * `message_id` - The ID of the message being acknowledged
    pub fn new_acknowledgment(message_id: u64) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Acknowledgment(message_id),
        }
    }

    /// Creates a new command message.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The command to execute
    pub fn new_command(cmd: Command) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Command(cmd),
        }
    }

    /// Creates a new heartbeat message for connection keep-alive.
    pub fn new_heartbeat() -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Heartbeat,
        }
    }

    /// Serializes the message to bytes using bincode.
    ///
    /// # Returns
    ///
    /// Returns the serialized message as bytes, or a bincode error.
    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    /// Deserializes bytes into a Message using bincode.
    ///
    /// # Arguments
    ///
    /// * `data` - The serialized message bytes
    ///
    /// # Returns
    ///
    /// Returns the deserialized Message, or a bincode error.
    pub fn deserialize(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }

    /// Creates a new encryption-related message.
    ///
    /// # Arguments
    ///
    /// * `msg` - The encryption message (key exchange, handshake, etc.)
    pub fn new_encryption(msg: EncryptionMessage) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Encryption(msg),
        }
    }

    /// Creates a new encrypted text message.
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Base64 encoded encrypted text
    pub fn new_encrypted_text(encrypted: String) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::EncryptedText(encrypted),
        }
    }
}
