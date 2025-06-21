use tokio_rustls::rustls::{self, Certificate, PrivateKey};
use tokio_rustls::{TlsAcceptor, TlsConnector};
use std::sync::Arc;
use crate::error::{ChatError, Result};

pub struct TlsConfig {
    pub acceptor: TlsAcceptor,
    pub connector: TlsConnector,
}

impl TlsConfig {
    pub fn new_self_signed() -> Result<Self> {
        // Generate self-signed certificate
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])
            .map_err(|e| ChatError::Encryption(format!("Failed to generate certificate: {}", e)))?;
        
        let cert_der = cert.serialize_der()
            .map_err(|e| ChatError::Encryption(format!("Failed to serialize certificate: {}", e)))?;
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
        root_store.add(&Certificate(cert_der))
            .map_err(|_| ChatError::Encryption("Failed to add certificate to root store".to_string()))?;
        
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