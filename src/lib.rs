//! # Rust P2P Chat
//! 
//! A blazing-fast, truly decentralized peer-to-peer chat application built with Rust and Tokio.
//! 
//! ## Features
//! 
//! - **True P2P Architecture**: Direct TCP connections, no central servers
//! - **End-to-End Encryption**: RSA + AES-256-GCM encryption
//! - **File Transfer**: Send files up to 100MB with auto-open support
//! - **Cross-Platform**: Windows, macOS, Linux support
//! - **Zero Configuration**: Works instantly with just IP:port
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use rust_p2p_chat::{P2PChat, Config};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let mut chat = P2PChat::new(config)?;
//!     chat.start(8080, None).await?;
//!     Ok(())
//! }
//! ```
//! 
//! ## Architecture
//! 
//! The application is built around several core modules:
//! 
//! - [`P2PChat`]: Main application orchestrator
//! - [`config::Config`]: Configuration management
//! - [`file_transfer::FileTransfer`]: File operations
//! - [`encryption::E2EEncryption`]: End-to-end encryption
//! - [`protocol`]: Message types and serialization
//! - [`commands`]: Command system

pub mod colors;
pub mod error;
pub mod protocol;  
pub mod config;
pub mod peer;
pub mod encryption;
pub mod file_transfer;
pub mod commands;

use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use futures::future::try_join;
use tokio::select;

use crate::colors::Colors;
use crate::protocol::{Message, MessageType, Command, StatusUpdate, EncryptionMessage};
use crate::commands::CommandHandler;
use crate::encryption::E2EEncryption;
use crate::peer::PeerManager;

// Re-export important types for library users
pub use crate::config::Config;
pub use crate::error::{ChatError, Result};


/// A decentralized peer-to-peer chat application.
/// 
/// `P2PChat` provides a complete implementation of a peer-to-peer chat system
/// with end-to-end encryption, file transfers, and a command system.
/// 
/// # Features
/// 
/// - Direct TCP connections between peers
/// - End-to-end encryption with RSA + AES-256-GCM
/// - File transfer with automatic integrity verification
/// - Auto-open for received media files
/// - Configurable settings via TOML files
/// - Cross-platform support (Windows, macOS, Linux)
/// 
/// # Examples
/// 
/// ```rust,no_run
/// use rust_p2p_chat::{P2PChat, Config};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::default();
///     let mut chat = P2PChat::new(config)?;
///     
///     // Start listening on port 8080
///     chat.start(8080, None).await?;
///     Ok(())
/// }
/// ```
/// 
/// # Protocol
/// 
/// The application uses a binary protocol with support for:
/// - Text messages (encrypted and unencrypted)
/// - File transfers with metadata
/// - Commands and status updates
/// - Heartbeat for connection monitoring
/// 
/// # Security
/// 
/// - RSA-1024 key exchange (configurable)
/// - AES-256-GCM message encryption
/// - SHA256 file integrity verification
/// - Perfect forward secrecy with session keys
pub struct P2PChat {
    /// Application configuration
    config: Config,
}

impl P2PChat {
    /// Creates a new P2P chat instance with the given configuration.
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration settings for the chat application
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rust_p2p_chat::{P2PChat, Config};
    /// 
    /// let config = Config::default();
    /// let chat = P2PChat::new(config).unwrap();
    /// ```
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    /// Starts the P2P chat application.
    /// 
    /// This method either connects to a peer at the specified address or starts
    /// listening for incoming connections on the given port.
    /// 
    /// # Arguments
    /// 
    /// * `listen_port` - Port number to listen on (if not connecting to a peer)
    /// * `peer_address` - Optional peer address to connect to (format: "ip:port")
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` when the chat session ends, or an error if connection fails.
    /// 
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use rust_p2p_chat::{P2PChat, Config};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut chat = P2PChat::new(Config::default())?;
    ///     
    ///     // Listen on port 8080
    ///     chat.start(8080, None).await?;
    ///     
    ///     // Or connect to a peer
    ///     // chat.start(0, Some("192.168.1.100:8080".to_string())).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    /// 
    /// # Errors
    /// 
    /// - `ChatError::BindFailed` if the port is already in use
    /// - `ChatError::ConnectFailed` if connection to peer fails
    /// - `ChatError::Io` for other network-related errors
    pub async fn start(&mut self, listen_port: u16, peer_address: Option<String>) -> Result<()> {
        let addr = format!("0.0.0.0:{}", listen_port);
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| ChatError::BindFailed(addr.clone(), e))?;
        
        println!("{}Listening on: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
        println!("{}Type /help for available commands{}", Colors::DIM, Colors::RESET);

        if let Some(peer_addr) = peer_address {
            self.connect_or_accept(listener, &peer_addr).await?;
        } else {
            println!("{}Waiting for peer to connect...{}", Colors::YELLOW, Colors::RESET);
            let (stream, peer_addr) = listener.accept().await?;
            println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, peer_addr, Colors::RESET);
            handle_enhanced_connection(stream, self.config.clone()).await?;
        }

        Ok(())
    }

    async fn connect_or_accept(&mut self, listener: TcpListener, peer_addr: &str) -> Result<()> {
        println!("{}Attempting to connect to peer at: {}{}", Colors::YELLOW, peer_addr, Colors::RESET);
        
        select! {
            result = listener.accept() => {
                let (stream, addr) = result?;
                println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                handle_enhanced_connection(stream, self.config.clone()).await?;
            }
            result = TcpStream::connect(peer_addr) => {
                match result {
                    Ok(stream) => {
                        let addr = stream.peer_addr()?;
                        println!("{}✓ Connected to peer at: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                        handle_enhanced_connection(stream, self.config.clone()).await?;
                    }
                    Err(e) => {
                        println!("{}Failed to connect: {}. Waiting for incoming connection...{}", 
                                Colors::YELLOW, e, Colors::RESET);
                        let (stream, addr) = listener.accept().await?;
                        println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                        handle_enhanced_connection(stream, self.config.clone()).await?;
                    }
                }
            }
        }
        Ok(())
    }
}

// Enhanced connection handler with new features
pub async fn handle_enhanced_connection(stream: TcpStream, config: Config) -> Result<()> {
    let (reader, writer) = stream.into_split();
    let (tx, rx) = mpsc::channel(100);
    
    // Initialize encryption
    let encryption = Arc::new(tokio::sync::Mutex::new(E2EEncryption::new()?));
    let enc_clone1 = encryption.clone();
    let enc_clone2 = encryption.clone();
    let tx_clone = tx.clone();
    
    // Initialize file transfer
    let file_transfer = Arc::new(file_transfer::FileTransfer::new(config.max_file_size_mb));
    
    // Start encryption handshake
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        let enc = enc_clone1.lock().await;
        if let Ok(pub_key) = enc.get_public_key_base64() {
            let msg = Message::new_encryption(EncryptionMessage::PublicKeyExchange(pub_key));
            let _ = tx_clone.send(msg).await;
        }
    });
    
    // Spawn tasks with encryption
    let read_handle = tokio::spawn(read_enhanced_messages(reader, config.clone(), enc_clone2.clone(), tx.clone(), file_transfer.clone()));
    let write_handle = tokio::spawn(write_enhanced_messages(writer, rx));
    let input_handle = tokio::spawn(handle_enhanced_input(tx, config, encryption, file_transfer));
    
    // Wait for any task to complete
    tokio::select! {
        _ = read_handle => {},
        _ = write_handle => {},
        _ = input_handle => {},
    }
    
    Ok(())
}

async fn read_enhanced_messages(
    mut reader: OwnedReadHalf, 
    config: Config,
    encryption: Arc<tokio::sync::Mutex<E2EEncryption>>,
    tx: mpsc::Sender<Message>,
    file_transfer: Arc<file_transfer::FileTransfer>
) -> Result<()> {
    let mut buffer = vec![0; config.buffer_size];
    
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("\n{}Peer disconnected{}", Colors::RED, Colors::RESET);
                return Ok(());
            }
            Ok(n) => {
                // Try new protocol first
                match Message::deserialize(&buffer[..n]) {
                    Ok(message) => handle_message(message, encryption.clone(), tx.clone(), &config, &file_transfer).await?,
                    Err(_) => {
                        // Fallback to plain text
                        let text = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
                        print!("\r\x1b[2K");
                        println!("{}{}Peer:{} {}", Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, text);
                        print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
                        io::Write::flush(&mut io::stdout())?;
                    }
                }
            }
            Err(e) => {
                eprintln!("\nError reading: {}", e);
                return Err(e.into());
            }
        }
    }
}

async fn handle_message(
    message: Message,
    encryption: Arc<tokio::sync::Mutex<E2EEncryption>>,
    tx: mpsc::Sender<Message>,
    config: &Config,
    file_transfer: &Arc<file_transfer::FileTransfer>
) -> Result<()> {
    match message.msg_type {
        MessageType::Text(text) => {
            print!("\r\x1b[2K");
            println!("{}{}Peer:{} {} {}(unencrypted){}", 
                Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, text, Colors::DIM, Colors::RESET);
            print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
            io::Write::flush(&mut io::stdout())?;
        }
        MessageType::EncryptedText(encrypted) => {
            let enc = encryption.lock().await;
            match enc.decrypt_message(&encrypted) {
                Ok(text) => {
                    print!("\r\x1b[2K");
                    println!("{}{}Peer:{} {} {}🔒{}", 
                        Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, text, Colors::GREEN, Colors::RESET);
                    print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
                    io::Write::flush(&mut io::stdout())?;
                }
                Err(_) => {
                    println!("\n{}Failed to decrypt message{}", Colors::RED, Colors::RESET);
                }
            }
        }
        MessageType::File(file_info) => {
            println!("\n{}📁 Receiving file: {} ({} bytes){}", 
                    Colors::YELLOW, file_info.name, file_info.size, Colors::RESET);
            
            // Save the file
            let download_dir = config.download_path();
            match file_transfer.save_file(&file_info, &download_dir).await {
                Ok(file_path) => {
                    println!("{}✓ File saved to: {}{}", 
                            Colors::GREEN, file_path.display(), Colors::RESET);
                    
                    // Check if auto-open is enabled and if it's a media file
                    if config.auto_open_media && 
                       file_transfer::FileTransfer::is_media_file(&file_info.name, &config.media_extensions) {
                        println!("{}🎬 Opening media file...{}", Colors::CYAN, Colors::RESET);
                        if let Err(e) = file_transfer::FileTransfer::open_file(&file_path) {
                            eprintln!("{}Failed to open file: {}{}", Colors::RED, e, Colors::RESET);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}Failed to save file: {}{}", Colors::RED, e, Colors::RESET);
                }
            }
        }
        MessageType::Status(status) => {
            match status {
                StatusUpdate::TransferProgress(name, current, total) => {
                    let percent = (current as f64 / total as f64) * 100.0;
                    print!("\r{}Progress {}: {:.1}%{}", Colors::YELLOW, name, percent, Colors::RESET);
                    io::Write::flush(&mut io::stdout())?;
                }
                StatusUpdate::EncryptionEnabled => {
                    println!("\n{}🔒 End-to-end encryption enabled!{}", Colors::GREEN, Colors::RESET);
                }
                _ => {}
            }
        }
        MessageType::Encryption(enc_msg) => {
            match enc_msg {
                EncryptionMessage::PublicKeyExchange(key) => {
                    println!("\n{}Received encryption key from peer...{}", Colors::YELLOW, Colors::RESET);
                    // Set peer's public key
                    let mut enc = encryption.lock().await;
                    if let Err(e) = enc.set_peer_public_key(&key) {
                        eprintln!("Failed to set peer public key: {}", e);
                        return Ok(());
                    }
                    
                    // If we haven't sent our key yet, send it
                    if !enc.is_ready() {
                        if let Ok(encrypted_key) = enc.generate_shared_key() {
                            drop(enc); // Release lock before sending
                            let msg = Message::new_encryption(EncryptionMessage::SharedKeyExchange(encrypted_key));
                            tx.send(msg).await.map_err(|_| ChatError::PeerDisconnected)?;
                            println!("{}Sending encrypted session key...{}", Colors::YELLOW, Colors::RESET);
                        }
                    }
                }
                EncryptionMessage::SharedKeyExchange(encrypted_key) => {
                    println!("\n{}Received encrypted session key...{}", Colors::YELLOW, Colors::RESET);
                    let mut enc = encryption.lock().await;
                    if let Err(e) = enc.set_shared_key(&encrypted_key) {
                        eprintln!("Failed to set shared key: {}", e);
                        return Ok(());
                    }
                    drop(enc);
                    
                    // Send confirmation
                    let msg = Message::new_encryption(EncryptionMessage::HandshakeComplete);
                    tx.send(msg).await.map_err(|_| ChatError::PeerDisconnected)?;
                    
                    // Send status update
                    let status_msg = Message {
                        id: rand::random(),
                        timestamp: std::time::SystemTime::now(),
                        msg_type: MessageType::Status(StatusUpdate::EncryptionEnabled),
                    };
                    tx.send(status_msg).await.map_err(|_| ChatError::PeerDisconnected)?;
                }
                EncryptionMessage::HandshakeComplete => {
                    println!("\n{}🔒 Encryption handshake complete!{}", Colors::GREEN, Colors::RESET);
                    // Send status update
                    let status_msg = Message {
                        id: rand::random(),
                        timestamp: std::time::SystemTime::now(),
                        msg_type: MessageType::Status(StatusUpdate::EncryptionEnabled),
                    };
                    tx.send(status_msg).await.map_err(|_| ChatError::PeerDisconnected)?;
                }
            }
        }
        _ => {}
    }
    Ok(())
}

async fn write_enhanced_messages(mut writer: OwnedWriteHalf, mut rx: mpsc::Receiver<Message>) -> Result<()> {
    while let Some(message) = rx.recv().await {
        match &message.msg_type {
            MessageType::Text(text) => {
                // Send as plain text for compatibility
                let plain = format!("{}\n", text);
                writer.write_all(plain.as_bytes()).await?;
                writer.flush().await?;
            }
            _ => {
                // Send using new protocol
                if let Ok(data) = message.serialize() {
                    writer.write_all(&data).await?;
                    writer.flush().await?;
                }
            }
        }
    }
    Ok(())
}

async fn handle_enhanced_input(
    tx: mpsc::Sender<Message>,
    mut config: Config,
    encryption: Arc<tokio::sync::Mutex<E2EEncryption>>,
    file_transfer: Arc<file_transfer::FileTransfer>
) -> Result<()> {
    let reader = BufReader::new(tokio::io::stdin());
    let mut lines = reader.lines();
    let mut command_handler = CommandHandler::new(config.clone());
    let peer_manager = PeerManager::new().0;
    
    println!("{}Type messages and press Enter to send (Ctrl+C to exit){}", Colors::DIM, Colors::RESET);
    print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
    io::Write::flush(&mut io::stdout())?;
    
    while let Ok(Some(line)) = lines.next_line().await {
        if line.is_empty() {
            print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
            io::Write::flush(&mut io::stdout())?;
            continue;
        }

        // Check for commands
        if let Some(command) = CommandHandler::parse_command(&line) {
            match &command {
                Command::Quit => break,
                Command::SendFile(path) => {
                    match file_transfer.prepare_file(&PathBuf::from(&path)).await {
                        Ok(file_info) => {
                            let msg = Message {
                                id: rand::random(),
                                timestamp: std::time::SystemTime::now(),
                                msg_type: MessageType::File(file_info),
                            };
                            tx.send(msg).await.map_err(|_| ChatError::PeerDisconnected)?;
                            println!("{}✓ File sent{}", Colors::GREEN, Colors::RESET);
                        }
                        Err(e) => println!("{}✗ Error: {}{}", Colors::RED, e, Colors::RESET),
                    }
                }
                Command::ToggleAutoOpen => {
                    config.auto_open_media = !config.auto_open_media;
                    config.save()?;
                    command_handler = CommandHandler::new(config.clone());
                    println!("{}✓ Auto-open media: {}{}", 
                            Colors::GREEN, 
                            if config.auto_open_media { "enabled" } else { "disabled" }, 
                            Colors::RESET);
                }
                _ => {
                    match command_handler.handle_command(command, &peer_manager).await {
                        Ok(response) => println!("{}", response),
                        Err(e) => println!("{}✗ Error: {}{}", Colors::RED, e, Colors::RESET),
                    }
                }
            }
        } else {
            // Try to encrypt message if encryption is ready
            let enc = encryption.lock().await;
            let msg = if enc.is_ready() {
                match enc.encrypt_message(&line) {
                    Ok(encrypted) => Message::new_encrypted_text(encrypted),
                    Err(_) => Message::new_text(line),
                }
            } else {
                Message::new_text(line)
            };
            drop(enc);
            
            tx.send(msg).await.map_err(|_| ChatError::PeerDisconnected)?;
        }
        
        print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
        io::Write::flush(&mut io::stdout())?;
    }
    
    Ok(())
}


// Keep original simple implementation for backward compatibility
pub struct P2PPeer {
    pub listen_port: u16,
    pub peer_address: Option<String>,
}

impl P2PPeer {
    pub fn new(listen_port: u16, peer_address: Option<String>) -> Self {
        Self { listen_port, peer_address }
    }

    pub async fn start(&self) -> io::Result<()> {
        let config = Config::default();
        let mut chat = P2PChat::new(config)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        chat.start(self.listen_port, self.peer_address.clone()).await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

// Keep the original handle_connection for tests
pub async fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let (reader, writer) = stream.into_split();
    
    let read_handle = tokio::spawn(async move {
        read_messages_simple(reader).await
    });
    
    let write_handle = tokio::spawn(async move {
        write_messages_simple(writer).await
    });
    
    let _ = try_join(
        async { read_handle.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e)) },
        async { write_handle.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e)) }
    ).await?;
    
    Ok(())
}

async fn read_messages_simple(mut reader: OwnedReadHalf) -> io::Result<()> {
    loop {
        let mut buffer = vec![0; 1024];
        match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("\n{}Peer disconnected{}", Colors::RED, Colors::RESET);
                return Ok(());
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
                print!("\r\x1b[2K");
                println!("{}{}Peer:{} {}", Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, message);
                print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
                io::Write::flush(&mut io::stdout())?;
            }
            Err(e) => {
                eprintln!("\nError reading from peer: {}", e);
                return Err(e);
            }
        }
    }
}

async fn write_messages_simple(mut writer: OwnedWriteHalf) -> io::Result<()> {
    let reader = BufReader::new(tokio::io::stdin());
    let mut lines = reader.lines();
    
    println!("{}Type messages and press Enter to send (Ctrl+C to exit){}", Colors::DIM, Colors::RESET);
    print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
    io::Write::flush(&mut io::stdout())?;
    
    while let Ok(Some(line)) = lines.next_line().await {
        if !line.is_empty() {
            let message = format!("{}\n", line);
            if let Err(e) = writer.write_all(message.as_bytes()).await {
                eprintln!("\nError sending message: {}", e);
                return Err(e);
            }
            writer.flush().await?;
        }
        
        print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
        io::Write::flush(&mut io::stdout())?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.default_port, 8080);
        assert_eq!(config.buffer_size, 8192);
    }

    #[test]
    fn test_command_parsing() {
        assert!(CommandHandler::parse_command("/help").is_some());
        assert!(CommandHandler::parse_command("/quit").is_some());
        assert!(CommandHandler::parse_command("not a command").is_none());
    }
}