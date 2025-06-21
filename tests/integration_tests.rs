use rust_p2p_chat::*;
use std::sync::{Arc, Mutex as StdMutex};
use std::collections::VecDeque;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_server_client_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        assert!(stream.peer_addr().is_ok());
        stream
    });
    
    let client_task = tokio::spawn(async move {
        let stream = TcpStream::connect(addr).await.unwrap();
        assert!(stream.peer_addr().is_ok());
        stream
    });
    
    let result = timeout(
        Duration::from_secs(5),
        futures::future::try_join(server_task, client_task)
    ).await;
    
    assert!(result.is_ok(), "Connection should be established within 5 seconds");
}

#[tokio::test]
async fn test_message_exchange() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let received_messages = Arc::new(StdMutex::new(Vec::new()));
    let messages_to_send = Arc::new(StdMutex::new(VecDeque::from(vec![
        "Hello from client\n".to_string(),
        "How are you?\n".to_string(),
    ])));
    
    let server_received = Arc::clone(&received_messages);
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        
        let read_handler = move |msg: String| {
            server_received.lock().unwrap().push(msg);
        };
        
        let write_handler = || None;
        
        let _ = timeout(
            Duration::from_secs(3),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let client_messages = Arc::clone(&messages_to_send);
    let client_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let stream = TcpStream::connect(addr).await.unwrap();
        
        let read_handler = |_: String| {};
        
        let write_handler = move || {
            client_messages.lock().unwrap().pop_front()
        };
        
        let _ = timeout(
            Duration::from_secs(3),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let _ = futures::future::join(server_task, client_task).await;
    
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    let received = received_messages.lock().unwrap();
    assert!(received.len() >= 2, "Should receive at least 2 messages, but got {}", received.len());
    assert!(received[0].contains("Hello from client"));
    assert!(received[1].contains("How are you?"));
}

#[tokio::test]
async fn test_bidirectional_communication() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_received = Arc::new(StdMutex::new(Vec::new()));
    let client_received = Arc::new(StdMutex::new(Vec::new()));
    
    let server_messages = Arc::new(StdMutex::new(VecDeque::from(vec![
        "Server message 1\n".to_string(),
        "Server message 2\n".to_string(),
    ])));
    
    let client_messages = Arc::new(StdMutex::new(VecDeque::from(vec![
        "Client message 1\n".to_string(),
        "Client message 2\n".to_string(),
    ])));
    
    let server_recv = Arc::clone(&server_received);
    let server_send = Arc::clone(&server_messages);
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        
        let read_handler = move |msg: String| {
            server_recv.lock().unwrap().push(msg);
        };
        
        let write_handler = move || {
            server_send.lock().unwrap().pop_front()
        };
        
        let _ = timeout(
            Duration::from_secs(3),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let client_recv = Arc::clone(&client_received);
    let client_send = Arc::clone(&client_messages);
    let client_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let stream = TcpStream::connect(addr).await.unwrap();
        
        let read_handler = move |msg: String| {
            client_recv.lock().unwrap().push(msg);
        };
        
        let write_handler = move || {
            client_send.lock().unwrap().pop_front()
        };
        
        let _ = timeout(
            Duration::from_secs(3),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let _ = futures::future::join(server_task, client_task).await;
    
    let server_msgs = server_received.lock().unwrap();
    let client_msgs = client_received.lock().unwrap();
    
    assert!(server_msgs.len() >= 2, "Server should receive at least 2 messages");
    assert!(client_msgs.len() >= 2, "Client should receive at least 2 messages");
    
    assert!(server_msgs[0].contains("Client message 1"));
    assert!(server_msgs[1].contains("Client message 2"));
    assert!(client_msgs[0].contains("Server message 1"));
    assert!(client_msgs[1].contains("Server message 2"));
}

#[tokio::test]
async fn test_connection_refused() {
    let result = timeout(
        Duration::from_secs(1),
        TcpStream::connect("127.0.0.1:1")
    ).await;
    
    assert!(result.is_ok(), "Should timeout or fail within 1 second");
    assert!(result.unwrap().is_err(), "Connection should be refused");
}

#[tokio::test]
async fn test_multiple_sequential_messages() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let received_messages = Arc::new(StdMutex::new(Vec::new()));
    let num_messages = 10;
    
    let messages: VecDeque<String> = (0..num_messages)
        .map(|i| format!("Message {}\n", i))
        .collect();
    
    let messages_to_send = Arc::new(StdMutex::new(messages.clone()));
    
    let server_received = Arc::clone(&received_messages);
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        
        let read_handler = move |msg: String| {
            server_received.lock().unwrap().push(msg);
        };
        
        let write_handler = || None;
        
        let _ = timeout(
            Duration::from_secs(5),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let client_messages = Arc::clone(&messages_to_send);
    let client_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let stream = TcpStream::connect(addr).await.unwrap();
        
        let read_handler = |_: String| {};
        
        let write_handler = move || {
            let mut msgs = client_messages.lock().unwrap();
            msgs.pop_front()
        };
        
        let _ = timeout(
            Duration::from_secs(5),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let _ = futures::future::join(server_task, client_task).await;
    
    let received = received_messages.lock().unwrap();
    assert_eq!(received.len(), num_messages, "Should receive all messages");
    
    for (i, msg) in received.iter().enumerate() {
        assert!(msg.contains(&format!("Message {}", i)), "Message order should be preserved");
    }
}

#[tokio::test]
async fn test_large_message() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let received_messages = Arc::new(StdMutex::new(Vec::new()));
    let large_message = "A".repeat(2000) + "\n";
    let messages_to_send = Arc::new(StdMutex::new(VecDeque::from(vec![large_message.clone()])));
    
    let server_received = Arc::clone(&received_messages);
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        
        let read_handler = move |msg: String| {
            server_received.lock().unwrap().push(msg);
        };
        
        let write_handler = || None;
        
        let _ = timeout(
            Duration::from_secs(5),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let client_messages = Arc::clone(&messages_to_send);
    let client_task = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let stream = TcpStream::connect(addr).await.unwrap();
        
        let read_handler = |_: String| {};
        
        let write_handler = move || {
            client_messages.lock().unwrap().pop_front()
        };
        
        let _ = timeout(
            Duration::from_secs(5),
            handle_connection_with_handlers(stream, read_handler, write_handler)
        ).await;
    });
    
    let _ = futures::future::join(server_task, client_task).await;
    
    let received = received_messages.lock().unwrap();
    assert!(!received.is_empty(), "Should receive the large message");
    
    let total_received: String = received.join("");
    assert!(total_received.contains(&"A".repeat(2000)), "Large message should be received correctly");
}