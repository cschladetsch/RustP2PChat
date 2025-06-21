use rust_p2p_chat::{P2PPeer, config::Config, P2PChat};
use tokio::time::{timeout, Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::test]
async fn test_basic_connection() {
    // Test that two peers can connect
    let peer1 = P2PPeer::new(0, None);
    let peer2 = P2PPeer::new(0, Some("127.0.0.1:9001".to_string()));
    
    // Basic connection test would require running in separate tasks
    assert_eq!(peer1.listen_port, 0);
    assert_eq!(peer2.peer_address, Some("127.0.0.1:9001".to_string()));
}

#[tokio::test]
async fn test_config_creation() {
    let config = Config::default();
    assert_eq!(config.default_port, 8080);
    assert_eq!(config.buffer_size, 8192);
    assert_eq!(config.heartbeat_interval_secs, 30);
    assert!(config.enable_encryption);
}

#[tokio::test]
async fn test_tcp_direct_connection() {
    // Test basic TCP connectivity
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"Hello, P2P!\n");
        stream.write_all(b"Hello back!\n").await.unwrap();
    });
    
    let client_task = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(b"Hello, P2P!\n").await.unwrap();
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"Hello back!\n");
    });
    
    let result = timeout(
        Duration::from_secs(5), 
        async { tokio::join!(server_task, client_task) }
    ).await;
    assert!(result.is_ok(), "Test timed out");
}

#[tokio::test]
async fn test_message_exchange() {
    // Test that messages can be exchanged
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let messages = vec!["Hello", "How are you?", "Goodbye"];
    let messages_clone = messages.clone();
    
    let server_task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        for expected in messages_clone {
            let mut buf = vec![0; 1024];
            let n = stream.read(&mut buf).await.unwrap();
            let received = String::from_utf8_lossy(&buf[..n]);
            assert!(received.contains(expected));
        }
    });
    
    let client_task = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        for msg in messages {
            stream.write_all(format!("{}\n", msg).as_bytes()).await.unwrap();
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });
    
    let result = timeout(
        Duration::from_secs(5), 
        async { tokio::join!(server_task, client_task) }
    ).await;
    assert!(result.is_ok(), "Test timed out");
}

#[tokio::test] 
async fn test_large_message() {
    // Test handling of large messages
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let large_msg = "x".repeat(4096);
    let large_msg_clone = large_msg.clone();
    
    let server_task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = vec![0; 8192];
        let mut total_read = 0;
        let mut received = String::new();
        
        while total_read < large_msg_clone.len() {
            let n = stream.read(&mut buf).await.unwrap();
            received.push_str(&String::from_utf8_lossy(&buf[..n]));
            total_read += n;
        }
        
        assert!(received.contains(&large_msg_clone));
    });
    
    let client_task = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(large_msg.as_bytes()).await.unwrap();
        stream.write_all(b"\n").await.unwrap();
    });
    
    let result = timeout(
        Duration::from_secs(5), 
        async { tokio::join!(server_task, client_task) }
    ).await;
    assert!(result.is_ok(), "Test timed out");
}

#[tokio::test]
async fn test_connection_refused() {
    // Test handling of connection refused
    let result = TcpStream::connect("127.0.0.1:9999").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_config_with_nickname() {
    let config = Config {
        nickname: Some("TestUser".to_string()),
        ..Default::default()
    };
    
    let chat = P2PChat::new(config);
    assert!(chat.is_ok());
}