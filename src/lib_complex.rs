pub mod colors;
pub mod error;
pub mod protocol;
pub mod config;
pub mod peer;
pub mod encryption;
pub mod file_transfer;
pub mod commands;

use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use futures::future::try_join;
use tokio::select;
use tracing::{info, warn, error, debug};

use crate::colors::Colors;
use crate::error::{ChatError, Result};
use crate::protocol::{Message, MessageType, Command, StatusUpdate};
use crate::config::Config;
use crate::peer::{PeerManager, Peer, PeerInfo};
use crate::file_transfer::FileTransfer;
use crate::commands::CommandHandler;

pub struct P2PChat {
    config: Config,
    peer_manager: PeerManager,
    command_handler: CommandHandler,
    file_transfer: FileTransfer,
}

impl P2PChat {
    pub fn new(config: Config) -> Result<Self> {
        let (peer_manager, _) = PeerManager::new();
        let command_handler = CommandHandler::new(config.clone());
        let file_transfer = FileTransfer::new(config.max_file_size_mb);

        Ok(Self {
            config,
            peer_manager,
            command_handler,
            file_transfer,
        })
    }

    pub async fn start(&mut self, listen_port: u16, peer_address: Option<String>) -> Result<()> {
        // Start listening
        let addr = format!("0.0.0.0:{}", listen_port);
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| ChatError::BindFailed(addr.clone(), e))?;
        
        println!("{}Listening on: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);

        if let Some(peer_addr) = peer_address {
            // Try to connect while also listening
            self.connect_or_accept(listener, &peer_addr).await?;
        } else {
            // Just listen
            println!("{}Waiting for peer to connect...{}", Colors::YELLOW, Colors::RESET);
            let (stream, peer_addr) = listener.accept().await?;
            println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, peer_addr, Colors::RESET);
            self.handle_peer_connection(stream, peer_addr).await?;
        }

        Ok(())
    }

    async fn connect_or_accept(&mut self, listener: TcpListener, peer_addr: &str) -> Result<()> {
        println!("{}Attempting to connect to peer at: {}{}", Colors::YELLOW, peer_addr, Colors::RESET);
        
        select! {
            // Accept incoming connections
            result = listener.accept() => {
                let (stream, addr) = result?;
                println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                self.handle_peer_connection(stream, addr).await?;
            }
            // Try to connect to peer
            result = TcpStream::connect(peer_addr) => {
                match result {
                    Ok(stream) => {
                        let addr = stream.peer_addr()?;
                        println!("{}✓ Connected to peer at: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                        self.handle_peer_connection(stream, addr).await?;
                    }
                    Err(e) => {
                        warn!("Failed to connect to peer: {}. Waiting for incoming connection...", e);
                        let (stream, addr) = listener.accept().await?;
                        println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                        self.handle_peer_connection(stream, addr).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_peer_connection(&mut self, stream: TcpStream, addr: SocketAddr) -> Result<()> {
        let peer_id = format!("{}", addr);
        let (tx, rx) = mpsc::channel(100);
        
        // For now, skip full peer manager integration

        // Start heartbeat
        let heartbeat_tx = tx.clone();
        let heartbeat_interval = self.config.heartbeat_interval_secs;
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(heartbeat_interval));
            loop {
                interval.tick().await;
                if heartbeat_tx.send(Message::new_heartbeat()).await.is_err() {
                    break;
                }
            }
        });

        // Split the stream
        let (reader, writer) = stream.into_split();
        
        // Start tasks
        let peer_id_clone = peer_id.clone();
        let config = self.config.clone();
        let read_handle = tokio::spawn(async move {
            read_messages(reader, peer_id_clone, config.buffer_size).await
        });
        
        let write_handle = tokio::spawn(async move {
            write_messages(writer, rx).await
        });
        
        let input_handle = tokio::spawn(self.handle_user_input(tx));

        // Wait for any task to complete
        tokio::select! {
            _ = read_handle => {},
            _ = write_handle => {},
            _ = input_handle => {},
        }

        // Remove peer when disconnected
        self.peer_manager.remove_peer(&peer_id).await;
        println!("\n{}Peer {} disconnected{}", Colors::RED, peer_id, Colors::RESET);

        Ok(())
    }

    async fn handle_user_input(&mut self, tx: mpsc::Sender<Message>) -> Result<()> {
        let reader = BufReader::new(tokio::io::stdin());
        let mut lines = reader.lines();
        
        println!("{}Type messages and press Enter to send (Ctrl+C to exit){}", Colors::DIM, Colors::RESET);
        println!("{}Type /help for available commands{}", Colors::DIM, Colors::RESET);
        print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
        io::Write::flush(&mut io::stdout())?;
        
        while let Ok(Some(line)) = lines.next_line().await {
            if line.is_empty() {
                print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
                io::Write::flush(&mut io::stdout())?;
                continue;
            }

            // Check if it's a command
            if let Some(command) = CommandHandler::parse_command(&line) {
                match command {
                    Command::Quit => break,
                    Command::SendFile(path) => {
                        // Handle file sending
                        match self.file_transfer.prepare_file(&PathBuf::from(&path)).await {
                            Ok(file_info) => {
                                let message = Message {
                                    id: rand::random(),
                                    timestamp: std::time::SystemTime::now(),
                                    msg_type: MessageType::File(file_info),
                                };
                                tx.send(message).await.map_err(|_| ChatError::PeerDisconnected)?;
                                println!("{}✓ File sent successfully{}", Colors::GREEN, Colors::RESET);
                            }
                            Err(e) => {
                                println!("{}✗ Failed to send file: {}{}", Colors::RED, e, Colors::RESET);
                            }
                        }
                    }
                    _ => {
                        match self.command_handler.handle_command(command, &self.peer_manager).await {
                            Ok(response) => println!("{}", response),
                            Err(e) => println!("{}Error: {}{}", Colors::RED, e, Colors::RESET),
                        }
                    }
                }
            } else {
                // Regular message - send as plain text for backward compatibility
                // Regular message - send as new protocol
                let _ = tx.send(Message::new_text(line)).await;
            }
            
            print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
            io::Write::flush(&mut io::stdout())?;
        }
        
        Ok(())
    }
}

async fn read_messages(mut reader: OwnedReadHalf, peer_id: String, buffer_size: usize) -> Result<()> {
    let mut buffer = vec![0; buffer_size];
    
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => {
                info!("Peer {} disconnected", peer_id);
                return Ok(());
            }
            Ok(n) => {
                // Try to deserialize as new protocol
                match Message::deserialize(&buffer[..n]) {
                    Ok(message) => {
                        handle_incoming_message(message, &peer_id).await?;
                    }
                    Err(_) => {
                        // Fallback to plain text for backward compatibility
                        let text = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
                        print!("\r\x1b[2K");
                        println!("{}{}Peer:{} {}", Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, text);
                        print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
                        io::Write::flush(&mut io::stdout())?;
                    }
                }
            }
            Err(e) => {
                error!("Error reading from peer {}: {}", peer_id, e);
                return Err(e.into());
            }
        }
    }
}

async fn handle_incoming_message(message: Message, peer_id: &str) -> Result<()> {
    match message.msg_type {
        MessageType::Text(text) => {
            print!("\r\x1b[2K");
            println!("{}{}Peer:{} {}", Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, text);
            print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
            io::Write::flush(&mut io::stdout())?;
        }
        MessageType::File(file_info) => {
            println!("\n{}Receiving file: {} ({} bytes){}", Colors::YELLOW, file_info.name, file_info.size, Colors::RESET);
            // File saving would be handled here
        }
        MessageType::Command(cmd) => {
            debug!("Received command from {}: {:?}", peer_id, cmd);
        }
        MessageType::Status(status) => {
            match status {
                StatusUpdate::TransferProgress(name, current, total) => {
                    let percent = (current as f64 / total as f64) * 100.0;
                    print!("\r{}Transferring {}: {:.1}%{}", Colors::YELLOW, name, percent, Colors::RESET);
                    io::Write::flush(&mut io::stdout())?;
                }
                _ => {}
            }
        }
        MessageType::Heartbeat => {
            debug!("Heartbeat from {}", peer_id);
        }
        MessageType::Acknowledgment(id) => {
            debug!("Message {} acknowledged by {}", id, peer_id);
        }
    }
    Ok(())
}

async fn write_messages(mut writer: OwnedWriteHalf, mut rx: mpsc::Receiver<Message>) -> Result<()> {
    while let Some(message) = rx.recv().await {
        // For backward compatibility, send text messages as plain strings
        match &message.msg_type {
            MessageType::Text(text) => {
                // Send as plain text for backward compatibility
                let plain = format!("{}\n", text);
                writer.write_all(plain.as_bytes()).await?;
                writer.flush().await?;
            }
            _ => {
                // Send other types using the new protocol
                if let Ok(data) = message.serialize() {
                    writer.write_all(&data).await?;
                    writer.flush().await?;
                }
            }
        }
    }
    Ok(())
}

// Keep backward compatibility
pub struct P2PPeer {
    chat: P2PChat,
    listen_port: u16,
    peer_address: Option<String>,
}

impl P2PPeer {
    pub fn new(listen_port: u16, peer_address: Option<String>) -> Self {
        let config = Config::default();
        let chat = P2PChat::new(config).expect("Failed to create chat");
        Self {
            chat,
            listen_port,
            peer_address,
        }
    }

    pub async fn start(&mut self) -> io::Result<()> {
        self.chat.start(self.listen_port, self.peer_address.clone()).await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

// Re-export the original handle_connection for backward compatibility
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