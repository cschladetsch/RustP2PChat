use rust_p2p_chat::protocol::{
    Command, EncryptionMessage, FileInfo, Message, MessageType, StatusUpdate,
};
use std::time::SystemTime;

#[test]
fn test_message_text_serialization() {
    let original = Message::new_text("Hello, world!".to_string());

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    assert_eq!(original.id, deserialized.id);
    if let (MessageType::Text(orig_text), MessageType::Text(deser_text)) =
        (&original.msg_type, &deserialized.msg_type)
    {
        assert_eq!(orig_text, deser_text);
    } else {
        panic!("Message types don't match");
    }
}

#[test]
fn test_message_encrypted_text_serialization() {
    let encrypted_content = "base64encodedencryptedtext";
    let original = Message::new_encrypted_text(encrypted_content.to_string());

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    if let (MessageType::EncryptedText(orig), MessageType::EncryptedText(deser)) =
        (&original.msg_type, &deserialized.msg_type)
    {
        assert_eq!(orig, deser);
    } else {
        panic!("Message types don't match");
    }
}

#[test]
fn test_message_file_serialization() {
    let file_info = FileInfo {
        name: "test_file.txt".to_string(),
        size: 1024,
        hash: "abc123hash".to_string(),
        data: vec![1, 2, 3, 4, 5],
    };

    let original = Message {
        id: rand::random(),
        timestamp: SystemTime::now(),
        msg_type: MessageType::File(file_info.clone()),
    };

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    if let MessageType::File(deserialized_file) = &deserialized.msg_type {
        assert_eq!(file_info.name, deserialized_file.name);
        assert_eq!(file_info.size, deserialized_file.size);
        assert_eq!(file_info.hash, deserialized_file.hash);
        assert_eq!(file_info.data, deserialized_file.data);
    } else {
        panic!("Message type is not File");
    }
}

#[test]
fn test_message_command_serialization() {
    let commands = vec![
        Command::Quit,
        Command::Help,
        Command::Info,
        Command::ListPeers,
        Command::SendFile("/path/to/file.txt".to_string()),
        Command::SetNickname("TestUser".to_string()),
        Command::ToggleAutoOpen,
        Command::Stats,
    ];

    for command in commands {
        let original = Message::new_command(command.clone());

        let serialized = original.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();

        if let MessageType::Command(deserialized_cmd) = &deserialized.msg_type {
            // Compare command variants
            assert_eq!(
                std::mem::discriminant(&command),
                std::mem::discriminant(deserialized_cmd)
            );
        } else {
            panic!("Message type is not Command");
        }
    }
}

#[test]
fn test_message_status_serialization() {
    let status_updates = vec![
        StatusUpdate::PeerConnected("192.168.1.100:8080".to_string()),
        StatusUpdate::PeerDisconnected("192.168.1.100:8080".to_string()),
        StatusUpdate::TransferProgress("file.txt".to_string(), 512, 1024),
        StatusUpdate::Error("Test error".to_string()),
        StatusUpdate::EncryptionEnabled,
        StatusUpdate::EncryptionDisabled,
    ];

    for status in status_updates {
        let original = Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Status(status.clone()),
        };

        let serialized = original.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();

        if let MessageType::Status(deserialized_status) = &deserialized.msg_type {
            assert_eq!(
                std::mem::discriminant(&status),
                std::mem::discriminant(deserialized_status)
            );
        } else {
            panic!("Message type is not Status");
        }
    }
}

#[test]
fn test_message_heartbeat_serialization() {
    let original = Message::new_heartbeat();

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    assert!(matches!(deserialized.msg_type, MessageType::Heartbeat));
}

#[test]
fn test_message_acknowledgment_serialization() {
    let msg_id = 12345u64;
    let original = Message::new_acknowledgment(msg_id);

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    if let MessageType::Acknowledgment(ack_id) = deserialized.msg_type {
        assert_eq!(msg_id, ack_id);
    } else {
        panic!("Message type is not Acknowledgment");
    }
}

#[test]
fn test_message_encryption_serialization() {
    let encryption_messages = vec![
        EncryptionMessage::PublicKeyExchange("base64publickey".to_string()),
        EncryptionMessage::SharedKeyExchange("base64sharedkey".to_string()),
        EncryptionMessage::HandshakeComplete,
    ];

    for enc_msg in encryption_messages {
        let original = Message::new_encryption(enc_msg.clone());

        let serialized = original.serialize().unwrap();
        let deserialized = Message::deserialize(&serialized).unwrap();

        if let MessageType::Encryption(deserialized_enc) = &deserialized.msg_type {
            assert_eq!(
                std::mem::discriminant(&enc_msg),
                std::mem::discriminant(deserialized_enc)
            );
        } else {
            panic!("Message type is not Encryption");
        }
    }
}

#[test]
fn test_invalid_message_deserialization() {
    let invalid_data = vec![0xFF, 0xFE, 0xFD, 0xFC]; // Random invalid bytes
    let result = Message::deserialize(&invalid_data);
    assert!(result.is_err());
}

#[test]
fn test_empty_message_deserialization() {
    let empty_data = vec![];
    let result = Message::deserialize(&empty_data);
    assert!(result.is_err());
}

#[test]
fn test_file_info_with_large_data() {
    let large_data = vec![42u8; 1024 * 1024]; // 1MB of data
    let file_info = FileInfo {
        name: "large_file.bin".to_string(),
        size: large_data.len() as u64,
        hash: "largefilehash".to_string(),
        data: large_data.clone(),
    };

    let original = Message {
        id: rand::random(),
        timestamp: SystemTime::now(),
        msg_type: MessageType::File(file_info),
    };

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    if let MessageType::File(deserialized_file) = &deserialized.msg_type {
        assert_eq!(large_data, deserialized_file.data);
        assert_eq!(large_data.len() as u64, deserialized_file.size);
    } else {
        panic!("Message type is not File");
    }
}

#[test]
fn test_file_info_with_unicode() {
    let unicode_name = "文件测试.txt";
    let unicode_content = "Hello, 世界! 🌍".as_bytes().to_vec();

    let file_info = FileInfo {
        name: unicode_name.to_string(),
        size: unicode_content.len() as u64,
        hash: "unicodehash".to_string(),
        data: unicode_content.clone(),
    };

    let original = Message {
        id: rand::random(),
        timestamp: SystemTime::now(),
        msg_type: MessageType::File(file_info),
    };

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    if let MessageType::File(deserialized_file) = &deserialized.msg_type {
        assert_eq!(unicode_name, deserialized_file.name);
        assert_eq!(unicode_content, deserialized_file.data);
    } else {
        panic!("Message type is not File");
    }
}

#[test]
fn test_message_timestamp_preservation() {
    let specific_time = SystemTime::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(1609459200)) // 2021-01-01
        .unwrap();

    let original = Message {
        id: 999,
        timestamp: specific_time,
        msg_type: MessageType::Text("Timestamp test".to_string()),
    };

    let serialized = original.serialize().unwrap();
    let deserialized = Message::deserialize(&serialized).unwrap();

    assert_eq!(original.timestamp, deserialized.timestamp);
    assert_eq!(original.id, deserialized.id);
}

#[test]
fn test_message_id_uniqueness() {
    let mut ids = std::collections::HashSet::new();

    // Generate 1000 messages and check ID uniqueness
    for _ in 0..1000 {
        let msg = Message::new_text("test".to_string());
        assert!(ids.insert(msg.id), "Duplicate ID found: {}", msg.id);
    }
}
