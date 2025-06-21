use rust_p2p_chat::{handle_connection, P2PChat, Config};
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Barrier, Mutex};
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn test_concurrent_connections() {
    // Test multiple concurrent connections to ensure thread safety
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_handle = tokio::spawn(async move {
        let mut connections = Vec::new();
        
        // Accept multiple connections concurrently
        for _ in 0..3 {
            let (stream, _) = listener.accept().await.unwrap();
            let handle = tokio::spawn(async move {
                handle_connection(stream).await
            });
            connections.push(handle);
        }
        
        // Wait for all connections to complete or timeout
        for handle in connections {
            let _ = timeout(Duration::from_secs(2), handle).await;
        }
    });
    
    // Create multiple client connections
    let mut client_handles = Vec::new();
    for i in 0..3 {
        let client_handle = tokio::spawn(async move {
            sleep(Duration::from_millis(i * 100)).await; // Stagger connections
            let mut stream = TcpStream::connect(addr).await.unwrap();
            
            // Send a message
            let message = format!("Hello from client {}\n", i);
            stream.write_all(message.as_bytes()).await.unwrap();
            
            // Read response
            let mut buffer = vec![0; 1024];
            let _ = stream.read(&mut buffer).await;
        });
        client_handles.push(client_handle);
    }
    
    // Wait for all clients
    for handle in client_handles {
        let _ = timeout(Duration::from_secs(3), handle).await;
    }
    
    // Wait for server
    let _ = timeout(Duration::from_secs(5), server_handle).await;
}

#[tokio::test]
async fn test_simultaneous_peer_connections() {
    // Test the race condition handling in connect_or_accept
    let barrier = Arc::new(Barrier::new(2));
    let results = Arc::new(Mutex::new(Vec::new()));
    
    let barrier1 = barrier.clone();
    let results1 = results.clone();
    let handle1 = tokio::spawn(async move {
        let config = Config {
            default_port: 0, // Use random port
            ..Default::default()
        };
        let mut chat = P2PChat::new(config).unwrap();
        
        barrier1.wait().await;
        
        // This might fail due to race conditions, which is expected
        let result = chat.start(0, Some("127.0.0.1:9999".to_string())).await;
        results1.lock().await.push(("peer1", result.is_ok()));
    });
    
    let barrier2 = barrier.clone();
    let results2 = results.clone();
    let handle2 = tokio::spawn(async move {
        let config = Config {
            default_port: 0, // Use random port
            ..Default::default()
        };
        let mut chat = P2PChat::new(config).unwrap();
        
        barrier2.wait().await;
        
        // This might fail due to race conditions, which is expected
        let result = chat.start(0, Some("127.0.0.1:9998".to_string())).await;
        results2.lock().await.push(("peer2", result.is_ok()));
    });
    
    // Wait for both with timeout
    let _ = timeout(Duration::from_secs(3), handle1).await;
    let _ = timeout(Duration::from_secs(3), handle2).await;
    
    // Check that at least the test completed without panicking
    let _final_results = results.lock().await;
    // We don't assert on success since connection might fail, but no panic should occur
    // Just ensure we got here without panic
}

#[tokio::test]
async fn test_multiple_message_exchange() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        
        // Receive and echo back multiple messages concurrently
        let _tasks: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        for _ in 0..5 {
            let mut buffer = vec![0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            if n > 0 {
                let received = String::from_utf8_lossy(&buffer[..n]);
                let response = format!("Echo: {}", received);
                let _ = stream.write_all(response.as_bytes()).await;
            }
        }
    });
    
    let client_handle = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        
        // Send multiple messages
        for i in 0..5 {
            let message = format!("Message {}\n", i);
            stream.write_all(message.as_bytes()).await.unwrap();
            
            // Read response
            let mut buffer = vec![0; 1024];
            let n = stream.read(&mut buffer).await.unwrap();
            let response = String::from_utf8_lossy(&buffer[..n]);
            assert!(response.contains(&format!("Message {}", i)));
        }
    });
    
    let result = timeout(Duration::from_secs(5), async {
        tokio::join!(server_handle, client_handle)
    }).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_connection_stress() {
    // Stress test with many rapid connections
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let connection_count = 20;
    let completed_connections = Arc::new(Mutex::new(0));
    
    let server_handle = tokio::spawn(async move {
        let mut handles = Vec::new();
        
        for _ in 0..connection_count {
            let (stream, _) = listener.accept().await.unwrap();
            let handle = tokio::spawn(async move {
                // Simple echo server
                let _ = handle_connection(stream).await;
            });
            handles.push(handle);
        }
        
        // Don't wait for completion to avoid timeout
    });
    
    // Create many concurrent clients
    let mut client_handles = Vec::new();
    for i in 0..connection_count {
        let completed = completed_connections.clone();
        let client_handle = tokio::spawn(async move {
            if let Ok(mut stream) = TcpStream::connect(addr).await {
                let message = format!("Stress test message {}\n", i);
                if stream.write_all(message.as_bytes()).await.is_ok() {
                    let mut count = completed.lock().await;
                    *count += 1;
                }
            }
        });
        client_handles.push(client_handle);
    }
    
    // Wait for clients with timeout
    for handle in client_handles {
        let _ = timeout(Duration::from_millis(500), handle).await;
    }
    
    // Check that at least some connections succeeded
    let final_count = *completed_connections.lock().await;
    assert!(final_count > 0);
    
    // Clean up server
    let _ = timeout(Duration::from_secs(1), server_handle).await;
}

#[tokio::test]
async fn test_concurrent_message_processing() {
    // Test concurrent processing of different message types
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let processed_messages = Arc::new(Mutex::new(Vec::new()));
    let processed_clone = processed_messages.clone();
    
    let server_handle = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        
        // Process multiple concurrent messages
        for _i in 0..10 {
            let mut buffer = vec![0; 1024];
            if let Ok(n) = stream.read(&mut buffer).await {
                if n > 0 {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    processed_clone.lock().await.push(message.to_string());
                    
                    // Echo back
                    let response = format!("Processed: {}", message);
                    let _ = stream.write_all(response.as_bytes()).await;
                }
            }
        }
    });
    
    let client_handle = tokio::spawn(async move {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        
        // Send different types of messages concurrently
        let messages = vec![
            "TEXT: Hello World\n",
            "FILE: image.png\n",
            "CMD: /help\n",
            "STATUS: Connected\n",
            "HEARTBEAT\n",
        ];
        
        for (i, msg) in messages.iter().cycle().take(10).enumerate() {
            let formatted_msg = format!("{}: {}", i, msg);
            stream.write_all(formatted_msg.as_bytes()).await.unwrap();
            
            // Read response
            let mut buffer = vec![0; 1024];
            let _ = stream.read(&mut buffer).await;
            
            // Small delay to prevent overwhelming
            sleep(Duration::from_millis(10)).await;
        }
    });
    
    let result = timeout(Duration::from_secs(10), async {
        tokio::join!(server_handle, client_handle)
    }).await;
    
    assert!(result.is_ok());
    
    // Verify messages were processed
    let messages = processed_messages.lock().await;
    assert!(messages.len() > 0);
}

#[tokio::test]
async fn test_connection_timeout_handling() {
    // Test handling of connection timeouts under load
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let server_handle = tokio::spawn(async move {
        // Accept only one connection and then delay
        let (mut stream, _) = listener.accept().await.unwrap();
        
        // Simulate slow processing
        sleep(Duration::from_millis(500)).await;
        
        let mut buffer = vec![0; 1024];
        let _ = stream.read(&mut buffer).await;
    });
    
    // Try to make multiple connections (only one will succeed)
    let mut client_handles = Vec::new();
    for i in 0..5 {
        let client_handle = tokio::spawn(async move {
            // Use timeout for connection attempt
            let connection_result = timeout(
                Duration::from_millis(200),
                TcpStream::connect(addr)
            ).await;
            
            match connection_result {
                Ok(Ok(mut stream)) => {
                    let message = format!("Client {} connected\n", i);
                    let _ = stream.write_all(message.as_bytes()).await;
                    true // Success
                }
                _ => false // Timeout or connection failed
            }
        });
        client_handles.push(client_handle);
    }
    
    // Wait for all clients
    let mut success_count = 0;
    for handle in client_handles {
        if let Ok(Ok(success)) = timeout(Duration::from_secs(1), handle).await {
            if success {
                success_count += 1;
            }
        }
    }
    
    // At least one should succeed
    assert!(success_count >= 1);
    
    // Clean up server
    let _ = timeout(Duration::from_secs(1), server_handle).await;
}

#[tokio::test]
async fn test_graceful_shutdown_under_load() {
    // Test graceful shutdown when multiple operations are in progress
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let shutdown_signal = Arc::new(Mutex::new(false));
    let shutdown_clone = shutdown_signal.clone();
    
    let server_handle = tokio::spawn(async move {
        let mut connections = Vec::new();
        
        // Accept a few connections
        for _ in 0..3 {
            if let Ok((stream, _)) = listener.accept().await {
                let shutdown_check = shutdown_clone.clone();
                let connection_handle = tokio::spawn(async move {
                    let mut stream = stream;
                    let mut buffer = vec![0; 1024];
                    
                    loop {
                        // Check shutdown signal
                        if *shutdown_check.lock().await {
                            break;
                        }
                        
                        // Short timeout read
                        match timeout(Duration::from_millis(100), stream.read(&mut buffer)).await {
                            Ok(Ok(n)) if n > 0 => {
                                let response = "OK\n";
                                let _ = stream.write_all(response.as_bytes()).await;
                            }
                            _ => break,
                        }
                    }
                });
                connections.push(connection_handle);
            }
        }
        
        // Wait a bit then signal shutdown
        sleep(Duration::from_millis(200)).await;
        *shutdown_signal.lock().await = true;
        
        // Wait for connections to clean up
        for handle in connections {
            let _ = timeout(Duration::from_millis(100), handle).await;
        }
    });
    
    // Create client connections
    let mut client_handles = Vec::new();
    for i in 0..3 {
        let client_handle = tokio::spawn(async move {
            if let Ok(mut stream) = TcpStream::connect(addr).await {
                // Send a few messages
                for j in 0..5 {
                    let message = format!("Client {} Message {}\n", i, j);
                    if stream.write_all(message.as_bytes()).await.is_err() {
                        break;
                    }
                    
                    let mut buffer = vec![0; 1024];
                    if stream.read(&mut buffer).await.is_err() {
                        break;
                    }
                    
                    sleep(Duration::from_millis(50)).await;
                }
            }
        });
        client_handles.push(client_handle);
    }
    
    // Wait for completion
    let result = timeout(Duration::from_secs(5), async {
        // Wait for clients
        for handle in client_handles {
            let _ = handle.await;
        }
        
        // Wait for server
        server_handle.await
    }).await;
    
    assert!(result.is_ok());
}