use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use futures::future::try_join;
use tokio::select;

mod colors;
use colors::Colors;

pub struct P2PPeer {
    listen_port: u16,
    peer_address: Option<String>,
}

impl P2PPeer {
    pub fn new(listen_port: u16, peer_address: Option<String>) -> Self {
        Self {
            listen_port,
            peer_address,
        }
    }

    pub async fn start(&self) -> io::Result<()> {
        // Start listening on our port
        let addr = format!("0.0.0.0:{}", self.listen_port);
        let listener = match TcpListener::bind(&addr).await {
            Ok(listener) => listener,
            Err(e) => {
                if e.kind() == io::ErrorKind::AddrInUse {
                    eprintln!("Error: Port {} is already in use!", self.listen_port);
                    eprintln!("Please try a different port or kill the existing process.");
                    eprintln!("To find the process: lsof -i :{} or netstat -tuln | grep {}", self.listen_port, self.listen_port);
                }
                return Err(e);
            }
        };
        println!("{}Listening on: {}{}", Colors::BRIGHT_GREEN, addr, Colors::RESET);

        if let Some(peer_addr) = &self.peer_address {
            // If we have a peer address, try to connect while also listening
            println!("{}Attempting to connect to peer at: {}{}", Colors::YELLOW, peer_addr, Colors::RESET);
            
            // Try both listening and connecting simultaneously
            select! {
                // Accept incoming connections
                result = listener.accept() => {
                    match result {
                        Ok((stream, peer_addr)) => {
                            println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, peer_addr, Colors::RESET);
                            handle_connection(stream).await?;
                        }
                        Err(e) => return Err(e),
                    }
                }
                // Try to connect to peer
                result = TcpStream::connect(peer_addr) => {
                    match result {
                        Ok(stream) => {
                            println!("{}✓ Connected to peer at: {}{}", Colors::BRIGHT_GREEN, peer_addr, Colors::RESET);
                            handle_connection(stream).await?;
                        }
                        Err(e) => {
                            println!("{}Failed to connect to peer: {}. Waiting for incoming connection...{}", Colors::YELLOW, e, Colors::RESET);
                            // If connection fails, just wait for incoming
                            let (stream, peer_addr) = listener.accept().await?;
                            println!("{}✓ Peer connected from: {}{}", Colors::BRIGHT_GREEN, peer_addr, Colors::RESET);
                            handle_connection(stream).await?;
                        }
                    }
                }
            }
        } else {
            // No peer address provided, just listen
            println!("{}Waiting for peer to connect...{}", Colors::YELLOW, Colors::RESET);
            let (stream, peer_addr) = listener.accept().await?;
            println!("Peer connected from: {}", peer_addr);
            handle_connection(stream).await?;
        }

        Ok(())
    }
}

pub async fn handle_connection(stream: TcpStream) -> io::Result<()> {
    let (reader, writer) = stream.into_split();
    
    let read_handle = tokio::spawn(async move {
        read_messages(reader).await
    });
    
    let write_handle = tokio::spawn(async move {
        write_messages(writer).await
    });
    
    let _ = try_join(
        async { read_handle.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e)) },
        async { write_handle.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e)) }
    ).await?;
    
    Ok(())
}

async fn read_messages(mut reader: OwnedReadHalf) -> io::Result<()> {
    loop {
        let mut buffer = vec![0; 1024];
        
        match reader.read(&mut buffer).await {
            Ok(0) => {
                println!("\n{}Peer disconnected{}", Colors::RED, Colors::RESET);
                return Ok(());
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
                print!("\r\x1b[2K");  // Clear current line
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

async fn write_messages(mut writer: OwnedWriteHalf) -> io::Result<()> {
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
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_peer_creation() {
        let peer = P2PPeer::new(8080, Some("127.0.0.1:8081".to_string()));
        assert_eq!(peer.listen_port, 8080);
        assert_eq!(peer.peer_address, Some("127.0.0.1:8081".to_string()));
    }

    #[tokio::test]
    async fn test_peer_creation_no_target() {
        let peer = P2PPeer::new(8080, None);
        assert_eq!(peer.listen_port, 8080);
        assert_eq!(peer.peer_address, None);
    }

    #[tokio::test]
    async fn test_tcp_connection() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        
        let server_handle = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            stream
        });
        
        let client_handle = tokio::spawn(async move {
            TcpStream::connect(addr).await.unwrap()
        });
        
        let result = timeout(Duration::from_secs(5), 
            futures::future::try_join(server_handle, client_handle)).await;
        
        assert!(result.is_ok());
    }
}