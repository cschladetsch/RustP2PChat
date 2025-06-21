use rust_p2p_chat::protocol::{Message, MessageType};
use rust_p2p_chat::reliability::{ReliabilityConfig, ReliabilityManager};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn test_reliability_config_default() {
    let config = ReliabilityConfig::default();
    
    assert_eq!(config.retry_attempts, 3);
    assert_eq!(config.retry_delay, Duration::from_secs(2));
    assert_eq!(config.ack_timeout, Duration::from_secs(10));
    assert_eq!(config.cleanup_interval, Duration::from_secs(30));
}

#[tokio::test]
async fn test_reliability_config_custom() {
    let config = ReliabilityConfig {
        retry_attempts: 5,
        retry_delay: Duration::from_millis(500),
        ack_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(60),
    };
    
    assert_eq!(config.retry_attempts, 5);
    assert_eq!(config.retry_delay, Duration::from_millis(500));
    assert_eq!(config.ack_timeout, Duration::from_secs(5));
    assert_eq!(config.cleanup_interval, Duration::from_secs(60));
}

#[tokio::test]
async fn test_reliability_manager_creation() {
    let (tx, _rx) = mpsc::channel(10);
    let config = ReliabilityConfig::default();
    let manager = ReliabilityManager::new(config, tx);
    
    let stats = manager.get_stats();
    assert_eq!(stats.total_pending, 0);
    assert!(stats.retry_distribution.is_empty());
}

#[tokio::test]
async fn test_send_reliable_message() {
    let (tx, mut rx) = mpsc::channel(10);
    let config = ReliabilityConfig::default();
    let mut manager = ReliabilityManager::new(config, tx);
    
    let message = Message::new_text("Test message".to_string());
    let message_id = message.id;
    
    // Send reliable message
    let result = manager.send_reliable(message).await;
    assert!(result.is_ok());
    
    // Verify message was sent
    let sent_message = rx.recv().await.unwrap();
    assert_eq!(sent_message.id, message_id);
    
    // Verify message is tracked
    let stats = manager.get_stats();
    assert_eq!(stats.total_pending, 1);
}

#[tokio::test]
async fn test_handle_acknowledgment() {
    let (tx, mut rx) = mpsc::channel(10);
    let config = ReliabilityConfig::default();
    let mut manager = ReliabilityManager::new(config, tx);
    
    let message = Message::new_text("Test message".to_string());
    let message_id = message.id;
    
    // Send reliable message
    manager.send_reliable(message).await.unwrap();
    let _sent_message = rx.recv().await.unwrap();
    
    // Verify message is pending
    let stats_before = manager.get_stats();
    assert_eq!(stats_before.total_pending, 1);
    
    // Handle acknowledgment
    manager.handle_acknowledgment(message_id);
    
    // Verify message is no longer pending
    let stats_after = manager.get_stats();
    assert_eq!(stats_after.total_pending, 0);
}

#[tokio::test]
async fn test_handle_unknown_acknowledgment() {
    let (tx, _rx) = mpsc::channel(10);
    let config = ReliabilityConfig::default();
    let mut manager = ReliabilityManager::new(config, tx);
    
    // Handle acknowledgment for unknown message
    // This should not panic or cause issues
    manager.handle_acknowledgment(12345);
    
    let stats = manager.get_stats();
    assert_eq!(stats.total_pending, 0);
}

#[tokio::test]
async fn test_message_retry() {
    let (tx, mut rx) = mpsc::channel(20);
    let config = ReliabilityConfig {
        retry_attempts: 2,
        retry_delay: Duration::from_millis(100),
        ..Default::default()
    };
    let mut manager = ReliabilityManager::new(config, tx);
    
    let message = Message::new_text("Retry test".to_string());
    let message_id = message.id;
    
    // Send reliable message
    manager.send_reliable(message).await.unwrap();
    let _initial_message = rx.recv().await.unwrap();
    
    // Wait for retry delay and process retries
    sleep(Duration::from_millis(150)).await;
    manager.process_retries().await;
    
    // Should receive retry
    let retry_message = timeout(Duration::from_millis(100), rx.recv()).await;
    assert!(retry_message.is_ok());
    assert_eq!(retry_message.unwrap().unwrap().id, message_id);
    
    // Check stats show retry
    let stats = manager.get_stats();
    assert_eq!(stats.total_pending, 1);
    assert_eq!(stats.retry_distribution.get(&1), Some(&1)); // 1 message with 1 retry
}

#[tokio::test]
async fn test_message_timeout() {
    let (tx, mut rx) = mpsc::channel(20);
    let config = ReliabilityConfig {
        retry_attempts: 1,
        retry_delay: Duration::from_millis(50),
        ..Default::default()
    };
    let mut manager = ReliabilityManager::new(config, tx);
    
    let message = Message::new_text("Timeout test".to_string());
    
    // Send reliable message
    manager.send_reliable(message).await.unwrap();
    let _initial_message = rx.recv().await.unwrap();
    
    // Wait and process first retry
    sleep(Duration::from_millis(60)).await;
    manager.process_retries().await;
    let _retry_message = rx.recv().await.unwrap();
    
    // Wait and process second retry (should timeout)
    sleep(Duration::from_millis(60)).await;
    manager.process_retries().await;
    
    // Message should be removed (timed out)
    let stats = manager.get_stats();
    assert_eq!(stats.total_pending, 0);
}

#[tokio::test]
async fn test_cleanup_old_messages() {
    let (tx, mut rx) = mpsc::channel(10);
    let config = ReliabilityConfig {
        ack_timeout: Duration::from_millis(100),
        ..Default::default()
    };
    let mut manager = ReliabilityManager::new(config, tx);
    
    let message = Message::new_text("Cleanup test".to_string());
    
    // Send reliable message
    manager.send_reliable(message).await.unwrap();
    let _sent_message = rx.recv().await.unwrap();
    
    // Verify message is pending
    let stats_before = manager.get_stats();
    assert_eq!(stats_before.total_pending, 1);
    
    // Wait beyond ack timeout
    sleep(Duration::from_millis(150)).await;
    
    // Run cleanup
    manager.cleanup_old_messages();
    
    // Message should be cleaned up
    let stats_after = manager.get_stats();
    assert_eq!(stats_after.total_pending, 0);
}

#[tokio::test]
async fn test_multiple_messages() {
    let (tx, mut rx) = mpsc::channel(50);
    let config = ReliabilityConfig::default();
    let mut manager = ReliabilityManager::new(config, tx);
    
    let mut message_ids = Vec::new();
    
    // Send multiple messages
    for i in 0..5 {
        let message = Message::new_text(format!("Message {}", i));
        message_ids.push(message.id);
        manager.send_reliable(message).await.unwrap();
    }
    
    // Receive all messages
    for _ in 0..5 {
        let _message = rx.recv().await.unwrap();
    }
    
    // Verify all messages are tracked
    let stats = manager.get_stats();
    assert_eq!(stats.total_pending, 5);
    
    // Acknowledge some messages
    for id in &message_ids[0..3] {
        manager.handle_acknowledgment(*id);
    }
    
    // Verify partial acknowledgment
    let stats_partial = manager.get_stats();
    assert_eq!(stats_partial.total_pending, 2);
}

#[tokio::test]
async fn test_stats_display() {
    let (tx, _rx) = mpsc::channel(10);
    let config = ReliabilityConfig::default();
    let manager = ReliabilityManager::new(config, tx);
    
    let stats = manager.get_stats();
    let display_text = format!("{}", stats);
    
    assert!(display_text.contains("Pending messages: 0"));
}

#[tokio::test]
async fn test_stats_with_retries() {
    let (tx, mut rx) = mpsc::channel(20);
    let config = ReliabilityConfig {
        retry_attempts: 3,
        retry_delay: Duration::from_millis(50),
        ..Default::default()
    };
    let mut manager = ReliabilityManager::new(config, tx);
    
    // Send messages and let them retry different amounts
    for i in 0..3 {
        let message = Message::new_text(format!("Message {}", i));
        manager.send_reliable(message).await.unwrap();
        let _initial = rx.recv().await.unwrap();
    }
    
    // Process one retry cycle
    sleep(Duration::from_millis(60)).await;
    manager.process_retries().await;
    
    // Consume retry messages
    for _ in 0..3 {
        let _retry = rx.recv().await.unwrap();
    }
    
    let stats = manager.get_stats();
    let display_text = format!("{}", stats);
    
    assert!(display_text.contains("Pending messages: 3"));
    assert!(display_text.contains("retries:"));
    assert!(display_text.contains("1:3")); // 3 messages with 1 retry each
}

#[tokio::test]
async fn test_reliability_manager_clone_config() {
    let config = ReliabilityConfig {
        retry_attempts: 5,
        retry_delay: Duration::from_millis(100),
        ack_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(30),
    };
    
    let cloned_config = config.clone();
    
    assert_eq!(config.retry_attempts, cloned_config.retry_attempts);
    assert_eq!(config.retry_delay, cloned_config.retry_delay);
    assert_eq!(config.ack_timeout, cloned_config.ack_timeout);
    assert_eq!(config.cleanup_interval, cloned_config.cleanup_interval);
}

#[tokio::test]
async fn test_send_reliable_channel_closed() {
    let (tx, rx) = mpsc::channel(10);
    let config = ReliabilityConfig::default();
    let mut manager = ReliabilityManager::new(config, tx);
    
    // Close the receiver
    drop(rx);
    
    let message = Message::new_text("Test message".to_string());
    
    // Should return error when channel is closed
    let result = manager.send_reliable(message).await;
    assert!(result.is_err());
}

#[test]
fn test_reliability_config_debug() {
    let config = ReliabilityConfig::default();
    let debug_str = format!("{:?}", config);
    
    assert!(debug_str.contains("ReliabilityConfig"));
    assert!(debug_str.contains("retry_attempts"));
    assert!(debug_str.contains("retry_delay"));
}