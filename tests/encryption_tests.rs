use rust_p2p_chat::encryption::{E2EEncryption, TlsConfig};
use rust_p2p_chat::protocol::{EncryptionMessage, Message, MessageType};
use base64::{engine::general_purpose, Engine as _};
use std::sync::Arc;
use tokio::sync::Mutex;

#[test]
fn test_tls_config_creation() {
    let tls_config = TlsConfig::new_self_signed();
    assert!(tls_config.is_ok());
    
    let _config = tls_config.unwrap();
    // TLS config should be created successfully
    // We can't directly test the internal certificate, but creation should not fail
}

#[test]
fn test_tls_config_multiple_instances() {
    // Test that multiple TLS configs can be created
    let config1 = TlsConfig::new_self_signed();
    let config2 = TlsConfig::new_self_signed();
    
    assert!(config1.is_ok());
    assert!(config2.is_ok());
    
    // Each should have its own certificate
    // (We can't directly compare certificates, but both should be valid)
}

#[tokio::test]
async fn test_e2e_encryption_creation() {
    let encryption = E2EEncryption::new();
    assert!(encryption.is_ok());
    
    let _enc = encryption.unwrap();
    // Should have RSA keypair generated
    // Should not have peer public key initially
    // Should not have AES key initially
}

#[tokio::test]
async fn test_e2e_encryption_get_public_key() {
    let encryption = E2EEncryption::new().unwrap();
    let public_key = encryption.get_public_key_base64();
    
    assert!(public_key.is_ok());
    let key_data = public_key.unwrap();
    
    // Should be base64 encoded
    assert!(general_purpose::STANDARD.decode(&key_data).is_ok());
    
    // Should be reasonably long (RSA 1024-bit key)
    assert!(key_data.len() > 100);
}

#[tokio::test]
async fn test_e2e_encryption_set_peer_public_key_valid() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let encryption2 = E2EEncryption::new().unwrap();
    
    // Get public key from encryption2
    let public_key = encryption2.get_public_key_base64().unwrap();
    
    // Set it in encryption1
    let result = encryption1.set_peer_public_key(&public_key);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_e2e_encryption_set_peer_public_key_invalid() {
    let mut encryption = E2EEncryption::new().unwrap();
    
    // Try to set invalid base64
    let result = encryption.set_peer_public_key(&"invalid_base64!@#".to_string());
    assert!(result.is_err());
    
    // Try to set valid base64 but invalid RSA key
    let invalid_key = general_purpose::STANDARD.encode("not_an_rsa_key");
    let result = encryption.set_peer_public_key(&invalid_key);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_set_peer_public_key_empty() {
    let mut encryption = E2EEncryption::new().unwrap();
    
    // Try to set empty key
    let result = encryption.set_peer_public_key(&"".to_string());
    assert!(result.is_err());
    
    // Try to set whitespace only
    let result = encryption.set_peer_public_key(&"   ".to_string());
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_generate_shared_key() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let encryption2 = E2EEncryption::new().unwrap();
    
    // Set up peer relationship
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    
    // Generate shared key
    let result = encryption1.generate_shared_key();
    assert!(result.is_ok());
    
    let encrypted_key = result.unwrap();
    // Should be base64 encoded
    assert!(general_purpose::STANDARD.decode(&encrypted_key).is_ok());
    
    // Should be reasonably long (encrypted AES key)
    assert!(encrypted_key.len() > 50);
}

#[tokio::test]
async fn test_e2e_encryption_generate_shared_key_no_peer() {
    let mut encryption = E2EEncryption::new().unwrap();
    
    // Try to generate shared key without setting peer public key
    let result = encryption.generate_shared_key();
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_set_shared_key_valid() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up peer relationship
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    
    // Generate shared key on encryption1
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    
    // Set it on encryption2
    let result = encryption2.set_shared_key(&encrypted_key);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_e2e_encryption_set_shared_key_invalid() {
    let mut encryption = E2EEncryption::new().unwrap();
    
    // Try to set invalid base64
    let result = encryption.set_shared_key(&"invalid_base64!@#".to_string());
    assert!(result.is_err());
    
    // Try to set valid base64 but can't decrypt (no private key match)
    let invalid_key = general_purpose::STANDARD.encode("not_encrypted_for_us");
    let result = encryption.set_shared_key(&invalid_key);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_set_shared_key_empty() {
    let mut encryption = E2EEncryption::new().unwrap();
    
    // Try to set empty key
    let result = encryption.set_shared_key(&"".to_string());
    assert!(result.is_err());
    
    // Try to set whitespace only
    let result = encryption.set_shared_key(&"   ".to_string());
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_full_key_exchange() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Full key exchange process
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    
    // Set each other's public keys
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    
    // Generate and exchange shared key
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Both should now be ready for encryption
    let message = "Test message for encryption";
    let encrypted = encryption1.encrypt_message(message);
    assert!(encrypted.is_ok());
    
    let decrypted = encryption2.decrypt_message(&encrypted.unwrap());
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), message);
}

#[tokio::test]
async fn test_e2e_encryption_encrypt_message_no_key() {
    let encryption = E2EEncryption::new().unwrap();
    
    // Try to encrypt without shared key
    let result = encryption.encrypt_message("test");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_encrypt_empty_message() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Test empty message
    let result = encryption1.encrypt_message("");
    assert!(result.is_ok());
    
    let decrypted = encryption2.decrypt_message(&result.unwrap());
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), "");
}

#[tokio::test]
async fn test_e2e_encryption_encrypt_large_message() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Test large message (1MB)
    let large_message = "A".repeat(1024 * 1024);
    let result = encryption1.encrypt_message(&large_message);
    assert!(result.is_ok());
    
    let decrypted = encryption2.decrypt_message(&result.unwrap());
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), large_message);
}

#[tokio::test]
async fn test_e2e_encryption_encrypt_unicode_message() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Test unicode message
    let unicode_message = "Hello 世界! 🌍 Café résumé naïve 🚀";
    let result = encryption1.encrypt_message(unicode_message);
    assert!(result.is_ok());
    
    let decrypted = encryption2.decrypt_message(&result.unwrap());
    assert!(decrypted.is_ok());
    assert_eq!(decrypted.unwrap(), unicode_message);
}

#[tokio::test]
async fn test_e2e_encryption_decrypt_invalid_ciphertext() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Try to decrypt invalid base64
    let result = encryption2.decrypt_message("invalid_base64!@#");
    assert!(result.is_err());
    
    // Try to decrypt valid base64 but invalid ciphertext
    let invalid_cipher = general_purpose::STANDARD.encode("not_valid_ciphertext");
    let result = encryption2.decrypt_message(&invalid_cipher);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_decrypt_corrupted_ciphertext() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Encrypt a message
    let original_message = "Test message for corruption";
    let encrypted = encryption1.encrypt_message(original_message).unwrap();
    
    // Corrupt the ciphertext
    let mut corrupted = encrypted.clone();
    corrupted.push('X'); // Corrupt by adding character
    let result = encryption2.decrypt_message(&corrupted);
    assert!(result.is_err());
    
    // Try with truncated ciphertext
    let truncated = &encrypted[..encrypted.len()-10];
    let result = encryption2.decrypt_message(truncated);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_decrypt_no_shared_key() {
    let encryption = E2EEncryption::new().unwrap();
    
    // Try to decrypt without shared key
    let result = encryption.decrypt_message("dGVzdA=="); // base64 "test"
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_sign_message() {
    let encryption = E2EEncryption::new().unwrap();
    
    let message = "Test message for signing";
    let signature = encryption.sign_message(message);
    assert!(signature.is_ok());
    
    let sig = signature.unwrap();
    // Should be base64 encoded
    assert!(general_purpose::STANDARD.decode(&sig).is_ok());
    
    // Should be reasonably long (RSA signature)
    assert!(sig.len() > 50);
}

#[tokio::test]
async fn test_e2e_encryption_sign_empty_message() {
    let encryption = E2EEncryption::new().unwrap();
    
    let signature = encryption.sign_message("");
    assert!(signature.is_ok());
    
    // Should be able to sign empty message
    let sig = signature.unwrap();
    assert!(general_purpose::STANDARD.decode(&sig).is_ok());
}

#[tokio::test]
async fn test_e2e_encryption_verify_signature_valid() {
    let encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up peer relationship for verification
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    
    // Sign message with encryption1
    let message = "Test message for verification";
    let signature = encryption1.sign_message(message).unwrap();
    
    // Verify with encryption2
    let result = encryption2.verify_signature(message, &signature);
    assert!(result.is_ok());
    assert!(result.unwrap()); // Should return true for valid signature
}

#[tokio::test]
async fn test_e2e_encryption_verify_signature_invalid() {
    let encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up peer relationship
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    
    // Sign message with encryption1
    let message = "Test message for verification";
    let signature = encryption1.sign_message(message).unwrap();
    
    // Try to verify with different message
    let result = encryption2.verify_signature("Different message", &signature);
    assert!(result.is_ok());
    assert!(!result.unwrap()); // Should return false for invalid signature
}

#[tokio::test]
async fn test_e2e_encryption_verify_signature_no_peer_key() {
    let encryption = E2EEncryption::new().unwrap();
    
    // Try to verify without peer public key
    let result = encryption.verify_signature("test", "dGVzdA=="); // base64 "test"
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_verify_signature_invalid_format() {
    let encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up peer relationship
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    
    // Try to verify invalid base64 signature
    let result = encryption2.verify_signature("test", "invalid_base64!@#");
    assert!(result.is_err());
    
    // Try to verify valid base64 but invalid signature
    let invalid_sig = general_purpose::STANDARD.encode("not_a_signature");
    let result = encryption2.verify_signature("test", &invalid_sig);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_e2e_encryption_concurrent_operations() {
    let encryption1 = Arc::new(Mutex::new(E2EEncryption::new().unwrap()));
    let encryption2 = Arc::new(Mutex::new(E2EEncryption::new().unwrap()));
    
    // Set up encryption in parallel
    let enc1_clone = encryption1.clone();
    let enc2_clone = encryption2.clone();
    
    let setup_task = tokio::spawn(async move {
        let public_key1 = enc1_clone.lock().await.get_public_key_base64().unwrap();
        let public_key2 = enc2_clone.lock().await.get_public_key_base64().unwrap();
        
        enc1_clone.lock().await.set_peer_public_key(&public_key2).unwrap();
        enc2_clone.lock().await.set_peer_public_key(&public_key1).unwrap();
        
        let encrypted_key = enc1_clone.lock().await.generate_shared_key().unwrap();
        enc2_clone.lock().await.set_shared_key(&encrypted_key).unwrap();
    });
    
    setup_task.unwrap();
    
    // Test concurrent encryption/decryption
    let mut handles = Vec::new();
    
    for i in 0..10 {
        let enc1 = encryption1.clone();
        let enc2 = encryption2.clone();
        let handle = tokio::spawn(async move {
            let message = format!("Concurrent message {}", i);
            let encrypted = enc1.lock().await.encrypt_message(&message).unwrap();
            let decrypted = enc2.lock().await.decrypt_message(&encrypted).unwrap();
            assert_eq!(decrypted, message);
        });
        handles.push(handle);
    }
    
    // Wait for all concurrent operations
    for handle in handles {
        handle.unwrap();
    }
}

#[tokio::test]
async fn test_e2e_encryption_key_reuse() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Test multiple messages with same key
    let messages = vec![
        "First message",
        "Second message",
        "Third message with special chars: !@#$%^&*()",
        "Fourth message with unicode: 你好世界",
    ];
    
    for message in messages {
        let encrypted = encryption1.encrypt_message(message).unwrap();
        let decrypted = encryption2.decrypt_message(&encrypted).unwrap();
        assert_eq!(decrypted, message);
    }
}

#[tokio::test]
async fn test_e2e_encryption_nonce_uniqueness() {
    let mut encryption1 = E2EEncryption::new().unwrap();
    let mut encryption2 = E2EEncryption::new().unwrap();
    
    // Set up encryption
    let public_key1 = encryption1.get_public_key_base64().unwrap();
    let public_key2 = encryption2.get_public_key_base64().unwrap();
    encryption1.set_peer_public_key(&public_key2).unwrap();
    encryption2.set_peer_public_key(&public_key1).unwrap();
    let encrypted_key = encryption1.generate_shared_key().unwrap();
    encryption2.set_shared_key(&encrypted_key).unwrap();
    
    // Encrypt same message multiple times
    let message = "Same message";
    let mut ciphertexts = Vec::new();
    
    for _ in 0..10 {
        let encrypted = encryption1.encrypt_message(message).unwrap();
        ciphertexts.push(encrypted);
    }
    
    // All ciphertexts should be different (due to random nonces)
    for i in 0..ciphertexts.len() {
        for j in i+1..ciphertexts.len() {
            assert_ne!(ciphertexts[i], ciphertexts[j]);
        }
    }
    
    // But all should decrypt to the same message
    for ciphertext in ciphertexts {
        let decrypted = encryption2.decrypt_message(&ciphertext).unwrap();
        assert_eq!(decrypted, message);
    }
}

#[test]
fn test_encryption_message_enum() {
    // Test EncryptionMessage enum variants
    let public_key_msg = EncryptionMessage::PublicKeyExchange("test_key".to_string());
    let shared_key_msg = EncryptionMessage::SharedKeyExchange("test_shared".to_string());
    
    match public_key_msg {
        EncryptionMessage::PublicKeyExchange(key) => assert_eq!(key, "test_key"),
        _ => panic!("Expected PublicKey variant"),
    }
    
    match shared_key_msg {
        EncryptionMessage::SharedKeyExchange(key) => assert_eq!(key, "test_shared"),
        _ => panic!("Expected SharedKey variant"),
    }
}

#[test]
fn test_message_with_encryption() {
    // Test Message with encrypted content
    let encrypted_msg = Message {
        id: 1,
        timestamp: std::time::SystemTime::now(),
        msg_type: MessageType::EncryptedText("encrypted_content".to_string()),
    };
    
    match encrypted_msg.msg_type {
        MessageType::EncryptedText(content) => assert_eq!(content, "encrypted_content"),
        _ => panic!("Expected EncryptedText variant"),
    }
}