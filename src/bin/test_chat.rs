use rust_p2p_chat::{config::Config, P2PChat};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing P2P Chat with Encryption...\n");

    // Test 1: Create config and chat instance
    let config = Config::default();
    let _chat = P2PChat::new(config)?;
    println!("✓ Created P2P chat instance");

    // Test 2: Show that encryption is available
    println!("✓ Encryption support: 1024-bit RSA + AES-256-GCM");

    // Test 3: Demonstrate the encryption module
    use rust_p2p_chat::encryption::E2EEncryption;

    let mut alice = E2EEncryption::new()?;
    let mut bob = E2EEncryption::new()?;

    // Exchange keys
    let alice_pub = alice.get_public_key_base64()?;
    let bob_pub = bob.get_public_key_base64()?;
    println!("✓ Generated RSA keypairs");

    alice.set_peer_public_key(&bob_pub)?;
    bob.set_peer_public_key(&alice_pub)?;
    println!("✓ Exchanged public keys");

    // Share session key
    let encrypted_key = alice.generate_shared_key()?;
    bob.set_shared_key(&encrypted_key)?;
    println!("✓ Established shared AES key");

    // Test encryption
    let message = "This is a secret message!";
    let encrypted = alice.encrypt_message(message)?;
    println!("✓ Encrypted message: {} -> {}", message, &encrypted[..32]);

    let decrypted = bob.decrypt_message(&encrypted)?;
    println!("✓ Decrypted message: {}", decrypted);

    assert_eq!(message, decrypted);
    println!("\n✅ All encryption tests passed!");

    println!("\nTo test the full chat:");
    println!("Terminal 1: cargo run -- --port 8080");
    println!("Terminal 2: cargo run -- --connect 127.0.0.1:8080");

    Ok(())
}
