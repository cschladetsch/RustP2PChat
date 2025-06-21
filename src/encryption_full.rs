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
        
        let certs = vec![Certificate(cert_der)];
        let key = PrivateKey(key_der);
        
        // Create server config
        let server_config = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs.clone(), key.clone())
            .map_err(|e| ChatError::Encryption(format!("Failed to create server config: {}", e)))?;
        
        // Create client config that accepts self-signed certificates
        let client_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(DangerousAcceptAllVerifier))
            .with_no_client_auth();
        
        Ok(TlsConfig {
            acceptor: TlsAcceptor::from(Arc::new(server_config)),
            connector: TlsConnector::from(Arc::new(client_config)),
        })
    }
}

// WARNING: This accepts all certificates. Only use for development!
struct DangerousAcceptAllVerifier;

impl rustls::client::ServerCertVerifier for DangerousAcceptAllVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> std::result::Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}