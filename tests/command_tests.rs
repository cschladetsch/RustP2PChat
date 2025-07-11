use rust_p2p_chat::commands::CommandHandler;
use rust_p2p_chat::config::Config;
use rust_p2p_chat::peer::PeerManager;
use rust_p2p_chat::protocol::Command;

#[test]
fn test_command_parsing_help() {
    assert!(matches!(
        CommandHandler::parse_command("/help"),
        Some(Command::Help)
    ));
    assert!(matches!(
        CommandHandler::parse_command("/?"),
        Some(Command::Help)
    ));
}

#[test]
fn test_command_parsing_quit() {
    assert!(matches!(
        CommandHandler::parse_command("/quit"),
        Some(Command::Quit)
    ));
    assert!(matches!(
        CommandHandler::parse_command("/exit"),
        Some(Command::Quit)
    ));
}

#[test]
fn test_command_parsing_info() {
    assert!(matches!(
        CommandHandler::parse_command("/info"),
        Some(Command::Info)
    ));
}

#[test]
fn test_command_parsing_peers() {
    assert!(matches!(
        CommandHandler::parse_command("/peers"),
        Some(Command::ListPeers)
    ));
    assert!(matches!(
        CommandHandler::parse_command("/list"),
        Some(Command::ListPeers)
    ));
}

#[test]
fn test_command_parsing_nickname() {
    if let Some(Command::SetNickname(nick)) = CommandHandler::parse_command("/nick Alice") {
        assert_eq!(nick, "Alice");
    } else {
        panic!("Expected SetNickname command");
    }

    if let Some(Command::SetNickname(nick)) = CommandHandler::parse_command("/nickname Bob Smith") {
        assert_eq!(nick, "Bob Smith");
    } else {
        panic!("Expected SetNickname command");
    }

    // Test nickname with spaces
    if let Some(Command::SetNickname(nick)) = CommandHandler::parse_command("/nick John Doe Jr.") {
        assert_eq!(nick, "John Doe Jr.");
    } else {
        panic!("Expected SetNickname command");
    }

    // Test empty nickname
    assert!(CommandHandler::parse_command("/nick").is_none());
}

#[test]
fn test_command_parsing_send_file() {
    if let Some(Command::SendFile(path)) = CommandHandler::parse_command("/send /path/to/file.txt")
    {
        assert_eq!(path, "/path/to/file.txt");
    } else {
        panic!("Expected SendFile command");
    }

    if let Some(Command::SendFile(path)) = CommandHandler::parse_command("/file document.pdf") {
        assert_eq!(path, "document.pdf");
    } else {
        panic!("Expected SendFile command");
    }

    // Test file path with spaces
    if let Some(Command::SendFile(path)) =
        CommandHandler::parse_command("/send /path/with spaces/file.txt")
    {
        assert_eq!(path, "/path/with spaces/file.txt");
    } else {
        panic!("Expected SendFile command");
    }

    // Test empty file path
    assert!(CommandHandler::parse_command("/send").is_none());
}

#[test]
fn test_command_parsing_auto_open() {
    assert!(matches!(
        CommandHandler::parse_command("/autoopen"),
        Some(Command::ToggleAutoOpen)
    ));
    assert!(matches!(
        CommandHandler::parse_command("/auto"),
        Some(Command::ToggleAutoOpen)
    ));
}

#[test]
fn test_command_parsing_stats() {
    assert!(matches!(
        CommandHandler::parse_command("/stats"),
        Some(Command::Stats)
    ));
    assert!(matches!(
        CommandHandler::parse_command("/statistics"),
        Some(Command::Stats)
    ));
}

#[test]
fn test_command_parsing_invalid() {
    assert!(CommandHandler::parse_command("hello").is_none());
    assert!(CommandHandler::parse_command("/invalid").is_none());
    assert!(CommandHandler::parse_command("").is_none());
    assert!(CommandHandler::parse_command("/").is_none());
    assert!(CommandHandler::parse_command("/ ").is_none());
}

#[test]
fn test_command_parsing_case_sensitivity() {
    // Commands should be case sensitive (lowercase only)
    assert!(CommandHandler::parse_command("/HELP").is_none());
    assert!(CommandHandler::parse_command("/Help").is_none());
    assert!(CommandHandler::parse_command("/QUIT").is_none());
    assert!(CommandHandler::parse_command("/Info").is_none());
}

#[test]
fn test_command_parsing_with_extra_spaces() {
    // Test commands with extra spaces (split_whitespace normalizes to single spaces)
    if let Some(Command::SetNickname(nick)) = CommandHandler::parse_command("/nick    Alice   Bob")
    {
        assert_eq!(nick, "Alice Bob");
    } else {
        panic!("Expected SetNickname command");
    }

    if let Some(Command::SendFile(path)) =
        CommandHandler::parse_command("/send    /path/to/file.txt")
    {
        assert_eq!(path, "/path/to/file.txt");
    } else {
        panic!("Expected SendFile command");
    }
}

#[tokio::test]
async fn test_command_handler_help() {
    let config = Config::default();
    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let result = handler.handle_command(Command::Help, &peer_manager).await;
    assert!(result.is_ok());

    let help_text = result.unwrap();
    assert!(help_text.contains("/help"));
    assert!(help_text.contains("/quit"));
    assert!(help_text.contains("/nick"));
    assert!(help_text.contains("/send"));
    assert!(help_text.contains("/stats"));
}

#[tokio::test]
async fn test_command_handler_info() {
    let config = Config {
        nickname: Some("TestUser".to_string()),
        enable_encryption: true,
        buffer_size: 4096,
        max_file_size_mb: 50,
        ..Default::default()
    };

    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let result = handler.handle_command(Command::Info, &peer_manager).await;
    assert!(result.is_ok());

    let info_text = result.unwrap();
    assert!(info_text.contains("TestUser"));
    assert!(info_text.contains("Enabled")); // Encryption enabled
    assert!(info_text.contains("4096")); // Buffer size
    assert!(info_text.contains("50")); // Max file size
}

#[tokio::test]
async fn test_command_handler_list_peers_empty() {
    let config = Config::default();
    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let result = handler
        .handle_command(Command::ListPeers, &peer_manager)
        .await;
    assert!(result.is_ok());

    let peers_text = result.unwrap();
    assert!(peers_text.contains("No peers connected"));
}

#[tokio::test]
async fn test_command_handler_set_nickname() {
    let config = Config::default();
    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let new_nickname = "NewTestUser".to_string();
    let result = handler
        .handle_command(Command::SetNickname(new_nickname.clone()), &peer_manager)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.contains("NewTestUser"));
    assert!(response.contains("Nickname set to"));
}

#[tokio::test]
async fn test_command_handler_quit() {
    let config = Config::default();
    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let result = handler.handle_command(Command::Quit, &peer_manager).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response, "Goodbye!");
}

#[tokio::test]
async fn test_command_handler_send_file() {
    let config = Config::default();
    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let file_path = "/test/path/file.txt".to_string();
    let result = handler
        .handle_command(Command::SendFile(file_path.clone()), &peer_manager)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.contains("Preparing to send file"));
    assert!(response.contains(&file_path));
}

#[tokio::test]
async fn test_command_handler_stats() {
    let config = Config::default();
    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let result = handler.handle_command(Command::Stats, &peer_manager).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.contains("Message reliability statistics"));
    assert!(response.contains("acknowledgments"));
}

#[tokio::test]
async fn test_command_handler_with_encryption_disabled() {
    let config = Config {
        enable_encryption: false,
        ..Default::default()
    };

    let mut handler = CommandHandler::new(config);
    let peer_manager = PeerManager::new().0;

    let result = handler.handle_command(Command::Info, &peer_manager).await;
    assert!(result.is_ok());

    let info_text = result.unwrap();
    assert!(info_text.contains("Disabled")); // Encryption disabled
}

#[test]
fn test_command_handler_new() {
    let config = Config {
        nickname: Some("HandlerTest".to_string()),
        ..Default::default()
    };

    let _handler = CommandHandler::new(config.clone());
    // We can't directly access the internal config, but we can test that it was created
    // This test mainly ensures the constructor doesn't panic

    // Test that we can create a handler with default config
    let _default_handler = CommandHandler::new(Config::default());
    // Similarly, just ensuring no panic
}
