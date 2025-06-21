use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, AsyncReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::time::Duration;
use futures::future::try_join;

pub struct ChatServer {
    port: u16,
}

impl ChatServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn start(&self) -> io::Result<()> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        println!("Listening on: {}", addr);
        println!("Waiting for peer to connect...");

        let (stream, peer_addr) = listener.accept().await?;
        println!("Peer connected from: {}", peer_addr);
        
        handle_connection(stream).await?;
        Ok(())
    }
}

pub struct ChatClient {
    address: String,
}

impl ChatClient {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub async fn connect(&self) -> io::Result<()> {
        println!("Connecting to {}...", self.address);
        let stream = TcpStream::connect(&self.address).await?;
        println!("Connected to peer!");
        
        handle_connection(stream).await?;
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
                println!("\nPeer disconnected");
                return Ok(());
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).trim_end().to_string();
                print!("\r\x1b[2K");  // Clear current line
                println!("Peer: {}", message);
                print!("You: ");
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
    
    println!("Type messages and press Enter to send (Ctrl+C to exit)");
    print!("You: ");
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
        
        print!("You: ");
        io::Write::flush(&mut io::stdout())?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{timeout, Duration};

    #[tokio::test]
    async fn test_server_creation() {
        let server = ChatServer::new(8080);
        assert_eq!(server.port, 8080);
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = ChatClient::new("127.0.0.1:8080".to_string());
        assert_eq!(client.address, "127.0.0.1:8080");
    }

    #[tokio::test]
    async fn test_server_bind() {
        let _server = ChatServer::new(0);
        let listener = TcpListener::bind("127.0.0.1:0").await;
        assert!(listener.is_ok());
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


pub async fn handle_connection_with_handlers<R, W>(
    stream: TcpStream,
    read_handler: R,
    write_handler: W,
) -> io::Result<()>
where
    R: Fn(String) + Send + Sync + 'static,
    W: Fn() -> Option<String> + Send + Sync + 'static,
{
    let (reader, writer) = stream.into_split();
    
    let read_handle = tokio::spawn(async move {
        read_messages_split(reader, read_handler).await
    });
    
    let write_handle = tokio::spawn(async move {
        write_messages_split(writer, write_handler).await
    });
    
    let _ = try_join(
        async { read_handle.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e)) },
        async { write_handle.await.map_err(|e| io::Error::new(io::ErrorKind::Other, e)) }
    ).await?;
    
    Ok(())
}


async fn read_messages_split<F>(
    mut reader: OwnedReadHalf,
    handler: F,
) -> io::Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    let mut buffer = vec![0; 1024];
    
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => return Ok(()),
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]).to_string();
                handler(message);
            }
            Err(e) => return Err(e),
        }
    }
}

async fn write_messages_split<F>(
    mut writer: OwnedWriteHalf,
    handler: F,
) -> io::Result<()>
where
    F: Fn() -> Option<String> + Send + Sync + 'static,
{
    loop {
        if let Some(message) = handler() {
            writer.write_all(message.as_bytes()).await?;
            writer.flush().await?;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

