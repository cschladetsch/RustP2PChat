use crate::error::{ChatError, Result};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand::rngs::OsRng;
use rsa::pkcs8::{DecodePublicKey, EncodePublicKey};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;
use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::{TlsAcceptor, TlsConnector};

pub struct TlsConfig {
    pub acceptor: TlsAcceptor,
    pub connector: TlsConnector,
}

impl TlsConfig {
    pub fn new_self_signed() -> Result<Self> {
        // Generate self-signed certificate
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])
            .map_err(|e| ChatError::Encryption(format!("Failed to generate certificate: {}", e)))?;

        let cert_der = cert.serialize_der().map_err(|e| {
            ChatError::Encryption(format!("Failed to serialize certificate: {}", e))
        })?;
        let key_der = cert.serialize_private_key_der();

        let certs = vec![Certificate(cert_der.clone())];
        let key = PrivateKey(key_der);

        // Create server config
        let server_config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs.clone(), key.clone())
            .map_err(|e| ChatError::Encryption(format!("Failed to create server config: {}", e)))?;

        // Create client config - for now, use a simple config
        // In production, you would properly verify certificates
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add(&Certificate(cert_der)).map_err(|_| {
            ChatError::Encryption("Failed to add certificate to root store".to_string())
        })?;

        let client_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(TlsConfig {
            acceptor: TlsAcceptor::from(Arc::new(server_config)),
            connector: TlsConnector::from(Arc::new(client_config)),
        })
    }
}

/// End-to-end encryption handler using RSA for key exchange and AES-256-GCM for messages
pub struct E2EEncryption {
    // Our RSA keypair
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,

    // Peer's public key (set after handshake)
    peer_public_key: Option<RsaPublicKey>,

    // Shared AES key derived from key exchange
    aes_key: Option<Key<Aes256Gcm>>,

    // Cipher for encryption/decryption
    cipher: Option<Aes256Gcm>,
}

impl E2EEncryption {
    /// Create new encryption handler with 1024-bit RSA keys
    pub fn new() -> Result<Self> {
        let mut rng = OsRng;

        // Generate 1024-bit RSA keypair
        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .map_err(|e| ChatError::Encryption(format!("Failed to generate RSA key: {}", e)))?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            private_key,
            public_key,
            peer_public_key: None,
            aes_key: None,
            cipher: None,
        })
    }

    /// Get our public key as base64-encoded string for exchange
    pub fn get_public_key_base64(&self) -> Result<String> {
        let public_key_der = self
            .public_key
            .to_public_key_der()
            .map_err(|e| ChatError::Encryption(format!("Failed to encode public key: {}", e)))?;
        Ok(general_purpose::STANDARD.encode(public_key_der.as_bytes()))
    }

    /// Set peer's public key from base64-encoded string
    pub fn set_peer_public_key(&mut self, key_base64: &str) -> Result<()> {
        let key_bytes = general_purpose::STANDARD
            .decode(key_base64)
            .map_err(|e| ChatError::Encryption(format!("Failed to decode public key: {}", e)))?;

        self.peer_public_key =
            Some(RsaPublicKey::from_public_key_der(&key_bytes).map_err(|e| {
                ChatError::Encryption(format!("Failed to parse public key: {}", e))
            })?);

        Ok(())
    }

    /// Generate and set shared AES key
    pub fn generate_shared_key(&mut self) -> Result<String> {
        // Generate random 256-bit AES key
        let mut key_bytes = [0u8; 32];
        use rand::RngCore;
        OsRng.fill_bytes(&mut key_bytes);

        // Store the key
        self.aes_key = Some(*Key::<Aes256Gcm>::from_slice(&key_bytes));
        self.cipher = Some(Aes256Gcm::new(self.aes_key.as_ref().unwrap()));

        // Encrypt the key with peer's public key
        let peer_key = self
            .peer_public_key
            .as_ref()
            .ok_or_else(|| ChatError::Encryption("Peer public key not set".to_string()))?;

        let encrypted_key = peer_key
            .encrypt(&mut OsRng, Pkcs1v15Encrypt, &key_bytes)
            .map_err(|e| ChatError::Encryption(format!("Failed to encrypt AES key: {}", e)))?;

        Ok(general_purpose::STANDARD.encode(&encrypted_key))
    }

    /// Decrypt and set shared AES key from peer
    pub fn set_shared_key(&mut self, encrypted_key_base64: &str) -> Result<()> {
        let encrypted_key = general_purpose::STANDARD
            .decode(encrypted_key_base64)
            .map_err(|e| ChatError::Encryption(format!("Failed to decode encrypted key: {}", e)))?;

        let key_bytes = self
            .private_key
            .decrypt(Pkcs1v15Encrypt, &encrypted_key)
            .map_err(|e| ChatError::Encryption(format!("Failed to decrypt AES key: {}", e)))?;

        if key_bytes.len() != 32 {
            return Err(ChatError::Encryption("Invalid AES key size".to_string()));
        }

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_bytes);

        self.aes_key = Some(*Key::<Aes256Gcm>::from_slice(&key_array));
        self.cipher = Some(Aes256Gcm::new(self.aes_key.as_ref().unwrap()));

        Ok(())
    }

    /// Encrypt a message
    pub fn encrypt_message(&self, plaintext: &str) -> Result<String> {
        let cipher = self
            .cipher
            .as_ref()
            .ok_or_else(|| ChatError::Encryption("Encryption not initialized".to_string()))?;

        // Generate random nonce (96 bits for GCM)
        let mut nonce_bytes = [0u8; 12];
        use rand::RngCore;
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| ChatError::Encryption(format!("Failed to encrypt message: {}", e)))?;

        // Combine nonce and ciphertext
        let mut combined = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        Ok(general_purpose::STANDARD.encode(&combined))
    }

    /// Decrypt a message
    pub fn decrypt_message(&self, ciphertext_base64: &str) -> Result<String> {
        let cipher = self
            .cipher
            .as_ref()
            .ok_or_else(|| ChatError::Encryption("Encryption not initialized".to_string()))?;

        let combined = general_purpose::STANDARD
            .decode(ciphertext_base64)
            .map_err(|e| ChatError::Encryption(format!("Failed to decode ciphertext: {}", e)))?;

        if combined.len() < 12 {
            return Err(ChatError::Encryption("Invalid ciphertext".to_string()));
        }

        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| ChatError::Encryption(format!("Failed to decrypt message: {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| ChatError::Encryption(format!("Failed to decode plaintext: {}", e)))
    }

    /// Check if encryption is ready
    pub fn is_ready(&self) -> bool {
        self.cipher.is_some()
    }

    /// Generate signature for a message
    pub fn sign_message(&self, message: &str) -> Result<String> {
        use rsa::sha2::Sha256;
        use rsa::signature::{SignatureEncoding, Signer};

        let signing_key = rsa::pkcs1v15::SigningKey::<Sha256>::new(self.private_key.clone());
        let signature = signing_key.sign(message.as_bytes());

        Ok(general_purpose::STANDARD.encode(signature.to_vec()))
    }

    /// Verify signature of a message
    pub fn verify_signature(&self, message: &str, signature_base64: &str) -> Result<bool> {
        use rsa::sha2::Sha256;
        use rsa::signature::Verifier;

        let peer_key = self
            .peer_public_key
            .as_ref()
            .ok_or_else(|| ChatError::Encryption("Peer public key not set".to_string()))?;

        let signature_bytes = general_purpose::STANDARD
            .decode(signature_base64)
            .map_err(|e| ChatError::Encryption(format!("Failed to decode signature: {}", e)))?;

        let verifying_key = rsa::pkcs1v15::VerifyingKey::<Sha256>::new(peer_key.clone());
        let signature = rsa::pkcs1v15::Signature::try_from(signature_bytes.as_slice())
            .map_err(|e| ChatError::Encryption(format!("Invalid signature format: {}", e)))?;

        Ok(verifying_key.verify(message.as_bytes(), &signature).is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e_encryption() {
        // Create two encryption instances
        let mut alice = E2EEncryption::new().unwrap();
        let mut bob = E2EEncryption::new().unwrap();

        // Exchange public keys
        let alice_pub = alice.get_public_key_base64().unwrap();
        let bob_pub = bob.get_public_key_base64().unwrap();

        alice.set_peer_public_key(&bob_pub).unwrap();
        bob.set_peer_public_key(&alice_pub).unwrap();

        // Alice generates and shares AES key
        let encrypted_key = alice.generate_shared_key().unwrap();
        bob.set_shared_key(&encrypted_key).unwrap();

        // Test encryption/decryption
        let message = "Hello, secure world!";
        let encrypted = alice.encrypt_message(message).unwrap();
        let decrypted = bob.decrypt_message(&encrypted).unwrap();

        assert_eq!(message, decrypted);

        // Test signature
        let signature = alice.sign_message(message).unwrap();
        assert!(bob.verify_signature(message, &signature).unwrap());
    }
}
