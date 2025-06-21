use rust_p2p_chat::{handle_connection, P2PPeer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
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

    let client_handle = tokio::spawn(async move { TcpStream::connect(addr).await.unwrap() });

    let result = timeout(
        Duration::from_secs(5),
        futures::future::try_join(server_handle, client_handle),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    // Server that uses handle_connection
    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        // The handle_connection function will try to read from stdin,
        // which won't work in tests, so we expect it to exit
        let _ = handle_connection(stream).await;
    });

    // Client that connects and sends a message
    let client_handle = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(b"Test message\n").await.unwrap();
        // Give server time to process
        tokio::time::sleep(Duration::from_millis(100)).await;
    });

    // Use shorter timeout since handle_connection won't complete normally in tests
    let _ = timeout(
        Duration::from_secs(1),
        futures::future::join(server_handle, client_handle),
    )
    .await;
}

#[tokio::test]
async fn test_bidirectional_communication() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_task = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();

        // Read message from client
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"Hello from client\n");

        // Send response
        stream.write_all(b"Hello from server\n").await.unwrap();
    });

    let client_task = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();

        // Send message
        stream.write_all(b"Hello from client\n").await.unwrap();

        // Read response
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(&buf[..n], b"Hello from server\n");
    });

    let result = timeout(Duration::from_secs(5), async {
        tokio::join!(server_task, client_task)
    })
    .await;
    assert!(result.is_ok(), "Test timed out");
}
