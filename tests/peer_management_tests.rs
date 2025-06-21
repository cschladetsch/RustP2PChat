use rust_p2p_chat::peer::{PeerInfo, PeerManager};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_peer_manager_creation() {
    let (manager, mut _receiver) = PeerManager::new();
    
    // Test that manager is created successfully
    let peer_count = manager.peer_count().await;
    assert_eq!(peer_count, 0);
    
    let peers = manager.list_peers().await;
    assert!(peers.is_empty());
}

#[tokio::test]
async fn test_peer_info_creation() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let peer_info = PeerInfo {
        id: "test_peer_1".to_string(),
        nickname: Some("TestUser".to_string()),
        address: addr,
        connected_at: SystemTime::now(),
    };
    
    assert_eq!(peer_info.id, "test_peer_1");
    assert_eq!(peer_info.nickname, Some("TestUser".to_string()));
    assert_eq!(peer_info.address, addr);
    assert!(peer_info.connected_at <= SystemTime::now());
}

#[tokio::test]
async fn test_peer_info_without_nickname() {
    let addr: SocketAddr = "192.168.1.100:9000".parse().unwrap();
    let peer_info = PeerInfo {
        id: "anonymous_peer".to_string(),
        nickname: None,
        address: addr,
        connected_at: SystemTime::now(),
    };
    
    assert_eq!(peer_info.id, "anonymous_peer");
    assert_eq!(peer_info.nickname, None);
    assert_eq!(peer_info.address, addr);
}

#[tokio::test]
async fn test_peer_creation() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let peer_info = PeerInfo {
        id: "peer_test".to_string(),
        nickname: Some("TestPeer".to_string()),
        address: addr,
        connected_at: SystemTime::now(),
    };
    
    // Create a mock stream (we'll use a placeholder since we can't easily create a real TcpStream in tests)
    // In practice, this would come from accepting a connection
    // For testing purposes, we'll skip the actual Peer creation with stream
    
    let (_tx, _rx) = mpsc::channel::<rust_p2p_chat::protocol::Message>(10);
    
    // Note: In actual usage, Peer would be created with a real TcpStream
    // For this test, we just verify the PeerInfo can be created correctly
    assert_eq!(peer_info.id, "peer_test");
    assert_eq!(peer_info.nickname, Some("TestPeer".to_string()));
    assert_eq!(peer_info.address, addr);
}

#[tokio::test]
async fn test_peer_clone() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let peer_info = PeerInfo {
        id: "clone_test".to_string(),
        nickname: Some("CloneUser".to_string()),
        address: addr,
        connected_at: SystemTime::now(),
    };
    
    // Test PeerInfo cloning behavior
    let cloned_info = peer_info.clone();
    
    assert_eq!(peer_info.id, cloned_info.id);
    assert_eq!(peer_info.nickname, cloned_info.nickname);
    assert_eq!(peer_info.address, cloned_info.address);
    assert_eq!(peer_info.connected_at, cloned_info.connected_at);
}

#[tokio::test]
async fn test_peer_info_clone() {
    let addr: SocketAddr = "10.0.0.1:5000".parse().unwrap();
    let original_time = SystemTime::now();
    
    let peer_info = PeerInfo {
        id: "info_clone_test".to_string(),
        nickname: Some("InfoClone".to_string()),
        address: addr,
        connected_at: original_time,
    };
    
    let cloned_info = peer_info.clone();
    
    assert_eq!(peer_info.id, cloned_info.id);
    assert_eq!(peer_info.nickname, cloned_info.nickname);
    assert_eq!(peer_info.address, cloned_info.address);
    assert_eq!(peer_info.connected_at, cloned_info.connected_at);
    
    // Ensure deep clone (different memory locations for String)
    assert_ne!(peer_info.id.as_ptr(), cloned_info.id.as_ptr());
}

#[tokio::test]
async fn test_peer_manager_multiple_operations() {
    let (manager, mut _receiver) = PeerManager::new();
    
    // Test multiple peer_count calls
    for _ in 0..5 {
        let count = manager.peer_count().await;
        assert_eq!(count, 0);
    }
    
    // Test multiple list_peers calls
    for _ in 0..5 {
        let peers = manager.list_peers().await;
        assert!(peers.is_empty());
    }
}

#[tokio::test]
async fn test_concurrent_peer_access() {
    let (manager, mut _receiver) = PeerManager::new();
    let manager = Arc::new(manager);
    
    // Spawn multiple tasks accessing peer manager concurrently
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let mgr = manager.clone();
        let handle = tokio::spawn(async move {
            let count = mgr.peer_count().await;
            let peers = mgr.list_peers().await;
            (i, count, peers.len())
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        let (_task_id, count, peer_list_len) = handle.await.unwrap();
        assert_eq!(count, 0);
        assert_eq!(peer_list_len, 0);
    }
}

#[tokio::test]
async fn test_peer_info_with_different_addresses() {
    let addresses = vec![
        "127.0.0.1:8080".parse().unwrap(),
        "0.0.0.0:9000".parse().unwrap(),
        "192.168.1.100:3000".parse().unwrap(),
        "[::1]:8080".parse().unwrap(), // IPv6
    ];
    
    for (i, addr) in addresses.iter().enumerate() {
        let peer_info = PeerInfo {
            id: format!("peer_{}", i),
            nickname: Some(format!("User{}", i)),
            address: *addr,
            connected_at: SystemTime::now(),
        };
        
        assert_eq!(peer_info.address, *addr);
        assert_eq!(peer_info.id, format!("peer_{}", i));
    }
}

#[tokio::test]
async fn test_peer_info_time_ordering() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    let peer1 = PeerInfo {
        id: "peer1".to_string(),
        nickname: None,
        address: addr,
        connected_at: SystemTime::now(),
    };
    
    // Small delay to ensure different timestamps
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    
    let peer2 = PeerInfo {
        id: "peer2".to_string(),
        nickname: None,
        address: addr,
        connected_at: SystemTime::now(),
    };
    
    // peer1 should be connected before peer2
    assert!(peer1.connected_at < peer2.connected_at);
}

#[tokio::test]
async fn test_peer_with_special_characters() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    let special_names = vec![
        "用户", // Chinese characters
        "José María", // Accented characters
        "User🎉", // Emoji
        "test@user.com", // Email-like
        "user-with-dashes_and_underscores",
        "", // Empty string
    ];
    
    for (i, name) in special_names.iter().enumerate() {
        let peer_info = PeerInfo {
            id: format!("special_peer_{}", i),
            nickname: if name.is_empty() { None } else { Some(name.to_string()) },
            address: addr,
            connected_at: SystemTime::now(),
        };
        
        if name.is_empty() {
            assert_eq!(peer_info.nickname, None);
        } else {
            assert_eq!(peer_info.nickname, Some(name.to_string()));
        }
    }
}

#[tokio::test] 
async fn test_peer_manager_stress() {
    let (manager, mut _receiver) = PeerManager::new();
    let manager = Arc::new(manager);
    
    // Stress test with many concurrent operations
    let mut handles = Vec::new();
    
    for i in 0..100 {
        let mgr = manager.clone();
        let handle = tokio::spawn(async move {
            // Alternate between peer_count and list_peers
            if i % 2 == 0 {
                mgr.peer_count().await
            } else {
                mgr.list_peers().await.len()
            }
        });
        handles.push(handle);
    }
    
    // All operations should complete successfully
    for handle in handles {
        let result = handle.await.unwrap();
        assert_eq!(result, 0); // No peers should be present
    }
}

#[test]
fn test_peer_info_debug_format() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let peer_info = PeerInfo {
        id: "debug_test".to_string(),
        nickname: Some("DebugUser".to_string()),
        address: addr,
        connected_at: SystemTime::UNIX_EPOCH,
    };
    
    let debug_str = format!("{:?}", peer_info);
    assert!(debug_str.contains("debug_test"));
    assert!(debug_str.contains("DebugUser"));
    assert!(debug_str.contains("127.0.0.1:8080"));
}

#[tokio::test]
async fn test_peer_manager_receiver() {
    let (_manager, mut receiver) = PeerManager::new();
    
    // Test that receiver exists and can be used
    // Since PeerManager doesn't send messages in the current implementation,
    // we just verify the receiver was created properly
    
    let try_recv_result = receiver.try_recv();
    assert!(try_recv_result.is_err()); // Should be empty initially
    
    // Test with timeout to ensure non-blocking behavior
    let timeout_result = tokio::time::timeout(
        tokio::time::Duration::from_millis(10),
        receiver.recv()
    ).await;
    assert!(timeout_result.is_err()); // Should timeout since no messages
}

#[tokio::test]
async fn test_peer_edge_cases() {
    // Test with maximum port number
    let max_port_addr: SocketAddr = "127.0.0.1:65535".parse().unwrap();
    let peer_info = PeerInfo {
        id: "max_port_peer".to_string(),
        nickname: None,
        address: max_port_addr,
        connected_at: SystemTime::now(),
    };
    assert_eq!(peer_info.address.port(), 65535);
    
    // Test with minimum port number
    let min_port_addr: SocketAddr = "127.0.0.1:1".parse().unwrap();
    let peer_info = PeerInfo {
        id: "min_port_peer".to_string(),
        nickname: None,
        address: min_port_addr,
        connected_at: SystemTime::now(),
    };
    assert_eq!(peer_info.address.port(), 1);
    
    // Test with very long ID
    let long_id = "a".repeat(1000);
    let peer_info = PeerInfo {
        id: long_id.clone(),
        nickname: None,
        address: "127.0.0.1:8080".parse().unwrap(),
        connected_at: SystemTime::now(),
    };
    assert_eq!(peer_info.id.len(), 1000);
    assert_eq!(peer_info.id, long_id);
}