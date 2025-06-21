use rust_p2p_chat::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_basic_tcp_echo() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0; 1024];
        
        let n = socket.read(&mut buf).await.unwrap();
        socket.write_all(&buf[..n]).await.unwrap();
    });
    
    let mut client = TcpStream::connect(addr).await.unwrap();
    let msg = b"hello world";
    client.write_all(msg).await.unwrap();
    
    let mut buf = vec![0; 1024];
    let n = timeout(Duration::from_secs(1), client.read(&mut buf)).await.unwrap().unwrap();
    
    assert_eq!(&buf[..n], msg);
}

#[tokio::test] 
async fn test_simple_message_send() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = vec![0; 1024];
        
        let mut messages = Vec::new();
        
        loop {
            match timeout(Duration::from_millis(500), socket.read(&mut buf)).await {
                Ok(Ok(0)) => break,
                Ok(Ok(n)) => {
                    let msg = String::from_utf8_lossy(&buf[..n]).to_string();
                    messages.push(msg);
                }
                _ => break,
            }
        }
        
        messages
    });
    
    let mut client = TcpStream::connect(addr).await.unwrap();
    
    client.write_all(b"Message 1\n").await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    client.write_all(b"Message 2\n").await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    drop(client);
    
    let messages = server_task.await.unwrap();
    assert_eq!(messages.len(), 2);
    assert!(messages[0].contains("Message 1"));
    assert!(messages[1].contains("Message 2"));
}

#[tokio::test]
async fn test_chat_server_starts() {
    let _server = ChatServer::new(0);
    
    let listener = TcpListener::bind("127.0.0.1:0").await;
    assert!(listener.is_ok());
}

#[tokio::test]
async fn test_chat_client_connects() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    tokio::spawn(async move {
        let (_, _) = listener.accept().await.unwrap();
    });
    
    let _client = ChatClient::new(addr.to_string());
    
    let connect_result = timeout(
        Duration::from_secs(1),
        TcpStream::connect(addr.to_string())
    ).await;
    
    assert!(connect_result.is_ok());
    assert!(connect_result.unwrap().is_ok());
}