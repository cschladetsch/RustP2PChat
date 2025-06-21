use rust_p2p_chat::config::Config;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_config_default_values() {
    let config = Config::default();
    
    assert_eq!(config.default_port, 8080);
    assert_eq!(config.buffer_size, 8192);
    assert_eq!(config.heartbeat_interval_secs, 30);
    assert_eq!(config.reconnect_attempts, 3);
    assert_eq!(config.reconnect_delay_secs, 5);
    assert_eq!(config.enable_encryption, true);
    assert_eq!(config.log_level, "info");
    assert_eq!(config.save_history, true);
    assert_eq!(config.max_file_size_mb, 100);
    assert_eq!(config.auto_open_media, true);
    assert!(config.nickname.is_none());
    assert!(config.history_file.is_none());
    assert!(config.download_dir.is_none());
    
    // Check default media extensions
    let expected_extensions = vec![
        "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg",
        "mp4", "avi", "mov", "wmv",
        "mp3", "wav", "flac", "aac",
        "pdf", "doc", "docx", "txt"
    ];
    assert_eq!(config.media_extensions.len(), expected_extensions.len());
    for ext in expected_extensions {
        assert!(config.media_extensions.contains(&ext.to_string()));
    }
}

#[test]
fn test_config_with_custom_values() {
    let config = Config {
        nickname: Some("TestUser".to_string()),
        default_port: 9090,
        buffer_size: 4096,
        heartbeat_interval_secs: 60,
        reconnect_attempts: 5,
        reconnect_delay_secs: 10,
        enable_encryption: false,
        log_level: "debug".to_string(),
        save_history: false,
        history_file: Some(PathBuf::from("/tmp/test_history.json")),
        max_file_size_mb: 50,
        download_dir: Some(PathBuf::from("/tmp/downloads")),
        auto_open_media: false,
        media_extensions: vec!["txt".to_string(), "pdf".to_string()],
    };
    
    assert_eq!(config.nickname, Some("TestUser".to_string()));
    assert_eq!(config.default_port, 9090);
    assert_eq!(config.buffer_size, 4096);
    assert_eq!(config.heartbeat_interval_secs, 60);
    assert_eq!(config.reconnect_attempts, 5);
    assert_eq!(config.reconnect_delay_secs, 10);
    assert_eq!(config.enable_encryption, false);
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.save_history, false);
    assert_eq!(config.max_file_size_mb, 50);
    assert_eq!(config.auto_open_media, false);
    assert_eq!(config.media_extensions, vec!["txt", "pdf"]);
}

#[test]
fn test_config_download_path_default() {
    let config = Config::default();
    let download_path = config.download_path();
    
    // Should return a valid path (either Downloads folder or current directory)
    assert!(download_path.exists() || download_path == PathBuf::from("."));
}

#[test]
fn test_config_download_path_custom() {
    let temp_dir = tempdir().unwrap();
    let custom_download_dir = temp_dir.path().to_path_buf();
    
    let config = Config {
        download_dir: Some(custom_download_dir.clone()),
        ..Default::default()
    };
    
    assert_eq!(config.download_path(), custom_download_dir);
}

#[test]
fn test_config_history_path_default() {
    let config = Config::default();
    let history_path = config.history_path();
    
    // Should return Some path for default case
    assert!(history_path.is_some());
    
    let path = history_path.unwrap();
    assert!(path.to_string_lossy().contains("chat_history.json"));
}

#[test]
fn test_config_history_path_custom() {
    let custom_history = PathBuf::from("/tmp/custom_history.json");
    let config = Config {
        history_file: Some(custom_history.clone()),
        ..Default::default()
    };
    
    assert_eq!(config.history_path(), Some(custom_history));
}

#[tokio::test]
async fn test_config_save_and_load() {
    let temp_dir = tempdir().unwrap();
    
    // Create a custom config
    let original_config = Config {
        nickname: Some("SaveLoadTest".to_string()),
        default_port: 7777,
        enable_encryption: false,
        log_level: "trace".to_string(),
        media_extensions: vec!["test".to_string()],
        ..Default::default()
    };
    
    // Mock the config path to use our temp directory
    // Note: This test may not work perfectly due to config_path() being private
    // In a real scenario, you'd want to make config_path configurable
    
    // For now, test the serialization/deserialization logic
    let serialized = toml::to_string_pretty(&original_config).unwrap();
    let deserialized: Config = toml::from_str(&serialized).unwrap();
    
    assert_eq!(deserialized.nickname, original_config.nickname);
    assert_eq!(deserialized.default_port, original_config.default_port);
    assert_eq!(deserialized.enable_encryption, original_config.enable_encryption);
    assert_eq!(deserialized.log_level, original_config.log_level);
    assert_eq!(deserialized.media_extensions, original_config.media_extensions);
}

#[test]
fn test_config_clone() {
    let config = Config {
        nickname: Some("CloneTest".to_string()),
        default_port: 8888,
        media_extensions: vec!["clone".to_string(), "test".to_string()],
        ..Default::default()
    };
    
    let cloned_config = config.clone();
    
    assert_eq!(config.nickname, cloned_config.nickname);
    assert_eq!(config.default_port, cloned_config.default_port);
    assert_eq!(config.media_extensions, cloned_config.media_extensions);
    
    // Ensure it's a deep clone
    assert_ne!(config.media_extensions.as_ptr(), cloned_config.media_extensions.as_ptr());
}

#[test]
fn test_config_debug_format() {
    let config = Config {
        nickname: Some("DebugTest".to_string()),
        ..Default::default()
    };
    
    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("DebugTest"));
    assert!(debug_str.contains("default_port"));
}

#[test]
fn test_config_edge_cases() {
    // Test with extreme values
    let config = Config {
        default_port: 1,
        buffer_size: 1,
        heartbeat_interval_secs: 0,
        reconnect_attempts: 0,
        reconnect_delay_secs: 0,
        max_file_size_mb: 0,
        media_extensions: vec![],
        log_level: "".to_string(),
        nickname: Some("".to_string()),
        ..Default::default()
    };
    
    // Should handle edge cases gracefully
    assert_eq!(config.default_port, 1);
    assert_eq!(config.buffer_size, 1);
    assert_eq!(config.media_extensions.len(), 0);
    assert_eq!(config.log_level, "");
    assert_eq!(config.nickname, Some("".to_string()));
}