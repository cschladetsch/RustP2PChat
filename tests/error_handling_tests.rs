use rust_p2p_chat::error::{ChatError, Result};
use std::error::Error;
use std::io;

#[test]
fn test_chat_error_display_io_permission_denied() {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let chat_error = ChatError::Io(io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Permission denied - check file/directory permissions"));
}

#[test]
fn test_chat_error_display_io_not_found() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let chat_error = ChatError::Io(io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("File or directory not found"));
}

#[test]
fn test_chat_error_display_io_connection_refused() {
    let io_error = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");
    let chat_error = ChatError::Io(io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Connection refused - peer may not be listening"));
}

#[test]
fn test_chat_error_display_io_connection_aborted() {
    let io_error = io::Error::new(io::ErrorKind::ConnectionAborted, "Connection aborted");
    let chat_error = ChatError::Io(io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Connection lost unexpectedly"));
}

#[test]
fn test_chat_error_display_io_timed_out() {
    let io_error = io::Error::new(io::ErrorKind::TimedOut, "Timed out");
    let chat_error = ChatError::Io(io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Operation timed out - check network connection"));
}

#[test]
fn test_chat_error_display_io_other() {
    let io_error = io::Error::new(io::ErrorKind::Other, "Some other error");
    let chat_error = ChatError::Io(io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("System error"));
    assert!(display_text.contains("Some other error"));
}

#[test]
fn test_chat_error_display_connection() {
    let chat_error = ChatError::Connection("Network timeout".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Network issue: Network timeout"));
}

#[test]
fn test_chat_error_display_protocol() {
    let chat_error = ChatError::Protocol("Invalid message format".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Communication error: Invalid message format (try reconnecting)"));
}

#[test]
fn test_chat_error_display_invalid_message() {
    let chat_error = ChatError::InvalidMessage("Corrupted data received".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Received corrupted data: Corrupted data received"));
}

#[test]
fn test_chat_error_display_peer_disconnected() {
    let chat_error = ChatError::PeerDisconnected;
    let display_text = format!("{}", chat_error);
    assert_eq!(display_text, "Your chat partner disconnected");
}

#[test]
fn test_chat_error_display_bind_failed_addr_in_use() {
    let io_error = io::Error::new(io::ErrorKind::AddrInUse, "Address already in use");
    let chat_error = ChatError::BindFailed("127.0.0.1:8080".to_string(), io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Port 127.0.0.1:8080 is already in use - try a different port"));
}

#[test]
fn test_chat_error_display_bind_failed_permission_denied() {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let chat_error = ChatError::BindFailed("0.0.0.0:80".to_string(), io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Cannot use port 0.0.0.0:80 - permission denied (try a port above 1024)"));
}

#[test]
fn test_chat_error_display_bind_failed_other() {
    let io_error = io::Error::new(io::ErrorKind::Other, "Some bind error");
    let chat_error = ChatError::BindFailed("127.0.0.1:8080".to_string(), io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Cannot start server on 127.0.0.1:8080"));
}

#[test]
fn test_chat_error_display_connect_failed_connection_refused() {
    let io_error = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");
    let chat_error = ChatError::ConnectFailed("192.168.1.100:8080".to_string(), io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Cannot reach 192.168.1.100:8080 - peer may not be running or firewall blocking"));
}

#[test]
fn test_chat_error_display_connect_failed_timed_out() {
    let io_error = io::Error::new(io::ErrorKind::TimedOut, "Connection timed out");
    let chat_error = ChatError::ConnectFailed("10.0.0.1:9000".to_string(), io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Connection to 10.0.0.1:9000 timed out - check IP address and network"));
}

#[test]
fn test_chat_error_display_connect_failed_invalid_input() {
    let io_error = io::Error::new(io::ErrorKind::InvalidInput, "Invalid input");
    let chat_error = ChatError::ConnectFailed("invalid_address".to_string(), io_error);
    
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Invalid address 'invalid_address' - use format IP:PORT (e.g., 192.168.1.100:8080)"));
}

#[test]
fn test_chat_error_display_encryption() {
    let chat_error = ChatError::Encryption("Key exchange failed".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Security error: Key exchange failed (encryption may be compromised)"));
}

#[test]
fn test_chat_error_display_file_transfer_too_large() {
    let chat_error = ChatError::FileTransfer("File too large: 200MB (max: 100MB)".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("File too large: File too large: 200MB (max: 100MB)"));
}

#[test]
fn test_chat_error_display_file_transfer_hash_mismatch() {
    let chat_error = ChatError::FileTransfer("File hash mismatch detected".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("File corrupted during transfer - please try again"));
}

#[test]
fn test_chat_error_display_file_transfer_failed_to_create() {
    let chat_error = ChatError::FileTransfer("Failed to create download directory".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Cannot save file: Failed to create download directory (check disk space and permissions)"));
}

#[test]
fn test_chat_error_display_file_transfer_other() {
    let chat_error = ChatError::FileTransfer("Unknown file error".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("File transfer failed: Unknown file error"));
}

#[test]
fn test_chat_error_display_configuration_failed_to_read() {
    let chat_error = ChatError::Configuration("Failed to read config file".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Cannot read settings: Failed to read config file (file may be corrupted)"));
}

#[test]
fn test_chat_error_display_configuration_failed_to_write() {
    let chat_error = ChatError::Configuration("Failed to write config file".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Cannot save settings: Failed to write config file (check permissions)"));
}

#[test]
fn test_chat_error_display_configuration_other() {
    let chat_error = ChatError::Configuration("Invalid configuration".to_string());
    let display_text = format!("{}", chat_error);
    assert!(display_text.contains("Settings error: Invalid configuration"));
}

#[test]
fn test_chat_error_source_io() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "Not found");
    let chat_error = ChatError::Io(io_error);
    
    assert!(chat_error.source().is_some());
}

#[test]
fn test_chat_error_source_bind_failed() {
    let io_error = io::Error::new(io::ErrorKind::AddrInUse, "Address in use");
    let chat_error = ChatError::BindFailed("127.0.0.1:8080".to_string(), io_error);
    
    assert!(chat_error.source().is_some());
}

#[test]
fn test_chat_error_source_connect_failed() {
    let io_error = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection refused");
    let chat_error = ChatError::ConnectFailed("127.0.0.1:8080".to_string(), io_error);
    
    assert!(chat_error.source().is_some());
}

#[test]
fn test_chat_error_source_others() {
    let errors = vec![
        ChatError::Connection("test".to_string()),
        ChatError::Protocol("test".to_string()),
        ChatError::InvalidMessage("test".to_string()),
        ChatError::PeerDisconnected,
        ChatError::Encryption("test".to_string()),
        ChatError::FileTransfer("test".to_string()),
        ChatError::Configuration("test".to_string()),
    ];
    
    for error in errors {
        assert!(error.source().is_none());
    }
}

#[test]
fn test_chat_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let chat_error: ChatError = io_error.into();
    
    if let ChatError::Io(inner) = chat_error {
        assert_eq!(inner.kind(), io::ErrorKind::NotFound);
    } else {
        panic!("Expected ChatError::Io variant");
    }
}

#[test]
fn test_result_type_alias() {
    fn test_function() -> Result<String> {
        Ok("success".to_string())
    }
    
    let result = test_function();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[test]
fn test_result_error_propagation() {
    fn inner_function() -> Result<()> {
        Err(ChatError::PeerDisconnected)
    }
    
    fn outer_function() -> Result<String> {
        inner_function()?;
        Ok("never reached".to_string())
    }
    
    let result = outer_function();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ChatError::PeerDisconnected));
}

#[test]
fn test_chat_error_debug_format() {
    let chat_error = ChatError::Connection("test connection error".to_string());
    let debug_str = format!("{:?}", chat_error);
    assert!(debug_str.contains("Connection"));
    assert!(debug_str.contains("test connection error"));
}

#[test]
fn test_chat_error_equality_check() {
    // Test that we can match on error variants
    let error = ChatError::PeerDisconnected;
    
    match error {
        ChatError::PeerDisconnected => {
            // This should match
        }
        _ => panic!("Should match PeerDisconnected"),
    }
}

#[test]
fn test_complex_error_chain() {
    // Test error conversion chain
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Access denied");
    let chat_error: ChatError = io_error.into();
    
    let display_msg = format!("{}", chat_error);
    assert!(display_msg.contains("Permission denied"));
    
    // Test that source is preserved
    assert!(chat_error.source().is_some());
}