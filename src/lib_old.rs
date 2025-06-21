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
use tokio::sync::{mpsc, Mutex};
use tokio::time::{interval, Duration};
use tokio_rustls::server::TlsStream as ServerTlsStream;
use tokio_rustls::client::TlsStream as ClientTlsStream;
use futures::future::try_join;
use tokio::select;
use tracing::{info, warn, error, debug};

use crate::colors::Colors;
use crate::error::{ChatError, Result};
use crate::protocol::{Message, MessageType, Command, StatusUpdate};
use crate::config::Config;
use crate::peer::{PeerManager, Peer, PeerInfo};
use crate::encryption::TlsConfig;
use crate::file_transfer::FileTransfer;
use crate::commands::CommandHandler;

pub struct P2PChat {
    config: Config,
    peer_manager: PeerManager,
    command_handler: CommandHandler,
    file_transfer: FileTransfer,
    tls_config: Option<TlsConfig>,
}

impl P2PChat {
    pub fn new(config: Config) -> Result<Self> {
        let (peer_manager, _) = PeerManager::new();
        let command_handler = CommandHandler::new(config.clone());
        let file_transfer = FileTransfer::new(config.max_file_size_mb);
        
        let tls_config = if config.enable_encryption {
            Some(TlsConfig::new_self_signed()?)
        } else {
            None
        };

        Ok(Self {
            config,
            peer_manager,
            command_handler,
            file_transfer,
            tls_config,
        })
    }

    pub async fn start(&mut self, listen_port: u16, peer_address: Option<String>) -> Result<()> {
        // Initialize logging
        use tracing::Level;
        let level = match self.config.log_level.as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        };
        tracing_subscriber::fmt()
            .with_max_level(level)
            .init();

        info!("Starting P2P Chat with port {} and peer {:?}", listen_port, peer_address);

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
            self.handle_peer_connection(stream, peer_addr, false).await?;
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
                self.handle_peer_connection(stream, addr, false).await?;
            }
            // Try to connect to peer
            result = TcpStream::connect(peer_addr) => {
                match result {
                    Ok(stream) => {
                        let addr = stream.peer_addr()?;
                        println!("{}✓ Connected to peer at: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                        self.handle_peer_connection(stream, addr, true).await?;
                    }
                    Err(e) => {
                        warn!("Failed to connect to peer: {}. Waiting for incoming connection...", e);
                        let (stream, addr) = listener.accept().await?;
                        println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);
                        self.handle_peer_connection(stream, addr, false).await?;
                    }
                }
            }
        }
        Ok(())
    }

    async fn handle_peer_connection(&mut self, stream: TcpStream, addr: SocketAddr, is_client: bool) -> Result<()> {
        let peer_id = format!("{}", addr);
        let (tx, mut rx) = mpsc::channel(100);
        
        // Create peer info
        let peer_info = PeerInfo {
            id: peer_id.clone(),
            nickname: None,
            address: addr,
            connected_at: std::time::SystemTime::now(),
        };

        // For now, disable encryption until we fix the implementation
        let stream = Arc::new(stream);

        // Create peer
        let peer = Peer {
            info: peer_info,
            stream: stream.clone(),
            tx: tx.clone(),
        };

        // Add peer to manager
        self.peer_manager.add_peer(peer_id.clone(), peer).await?;

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

        // Handle the connection
        let read_stream = stream.clone();
        let write_stream = stream;
        
        let read_handle = tokio::spawn(self.read_messages(read_stream, peer_id.clone()));
        let write_handle = tokio::spawn(self.write_messages(write_stream, rx));
        let input_handle = tokio::spawn(self.handle_user_input(tx));

        // Wait for tasks to complete
        let _ = tokio::try_join!(read_handle, write_handle, input_handle);

        // Remove peer when disconnected
        self.peer_manager.remove_peer(&peer_id).await;
        println!("\n{}Peer {} disconnected{}", Colors::RED, peer_id, Colors::RESET);

        Ok(())
    }

    async fn read_messages(&self, mut stream: Arc<dyn AsyncReadWrite>, peer_id: String) -> Result<()> {
        let mut buffer = vec![0; self.config.buffer_size];
        
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    info!("Peer {} disconnected", peer_id);
                    return Ok(());
                }
                Ok(n) => {
                    match Message::deserialize(&buffer[..n]) {
                        Ok(message) => {
                            self.handle_incoming_message(message, &peer_id).await?;
                        }
                        Err(_) => {
                            // Fallback to text for backward compatibility
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

    async fn handle_incoming_message(&self, message: Message, peer_id: &str) -> Result<()> {
        match message.msg_type {
            MessageType::Text(text) => {
                print!("\r\x1b[2K");
                println!("{}{}Peer:{} {}", Colors::BOLD, Colors::BRIGHT_CYAN, Colors::RESET, text);
                print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
                io::Write::flush(&mut io::stdout())?;
            }
            MessageType::File(file_info) => {
                println!("\n{}Receiving file: {} ({} bytes){}", Colors::YELLOW, file_info.name, file_info.size, Colors::RESET);
                let download_dir = PathBuf::from("downloads");
                match self.file_transfer.save_file(&file_info, &download_dir).await {
                    Ok(_) => println!("{}✓ File saved to downloads/{}{}", Colors::GREEN, file_info.name, Colors::RESET),
                    Err(e) => println!("{}✗ Failed to save file: {}{}", Colors::RED, e, Colors::RESET),
                }
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

    async fn write_messages(&self, mut stream: Arc<dyn AsyncReadWrite>, mut rx: mpsc::Receiver<Message>) -> Result<()> {
        while let Some(message) = rx.recv().await {
            let data = message.serialize()
                .map_err(|e| ChatError::Protocol(format!("Failed to serialize message: {}", e)))?;
            stream.write_all(&data).await?;
            stream.flush().await?;
        }
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
                                tx.send(message).await?;
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
                // Regular message
                let message = Message::new_text(line);
                tx.send(message).await?;
            }
            
            print!("{}{}You:{} ", Colors::BOLD, Colors::BRIGHT_GREEN, Colors::RESET);
            io::Write::flush(&mut io::stdout())?;
        }
        
        Ok(())
    }
}

// Trait for async read/write to support both TLS and plain TCP
#[async_trait::async_trait]
trait AsyncReadWrite: Send + Sync {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    async fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
    async fn flush(&mut self) -> io::Result<()>;
}

#[async_trait::async_trait]
impl AsyncReadWrite for TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        AsyncReadExt::read(self, buf).await
    }
    
    async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        AsyncWriteExt::write_all(self, buf).await
    }
    
    async fn flush(&mut self) -> io::Result<()> {
        AsyncWriteExt::flush(self).await
    }
}

#[async_trait::async_trait]
impl AsyncReadWrite for ServerTlsStream<TcpStream> {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        AsyncReadExt::read(self, buf).await
    }
    
    async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        AsyncWriteExt::write_all(self, buf).await
    }
    
    async fn flush(&mut self) -> io::Result<()> {
        AsyncWriteExt::flush(self).await
    }
}

#[async_trait::async_trait]
impl AsyncReadWrite for ClientTlsStream<TcpStream> {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        AsyncReadExt::read(self, buf).await
    }
    
    async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        AsyncWriteExt::write_all(self, buf).await
    }
    
    async fn flush(&mut self) -> io::Result<()> {
        AsyncWriteExt::flush(self).await
    }
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