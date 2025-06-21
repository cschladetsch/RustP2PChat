use std::fmt;
use std::io;
use std::error::Error;

#[derive(Debug)]
pub enum ChatError {
    Io(io::Error),
    Connection(String),
    Protocol(String),
    InvalidMessage(String),
    PeerDisconnected,
    BindFailed(String, io::Error),
    ConnectFailed(String, io::Error),
    Encryption(String),
    FileTransfer(String),
    Configuration(String),
}

impl fmt::Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatError::Io(e) => {
                match e.kind() {
                    io::ErrorKind::PermissionDenied => write!(f, "Permission denied - check file/directory permissions"),
                    io::ErrorKind::NotFound => write!(f, "File or directory not found"),
                    io::ErrorKind::ConnectionRefused => write!(f, "Connection refused - peer may not be listening"),
                    io::ErrorKind::ConnectionAborted => write!(f, "Connection lost unexpectedly"),
                    io::ErrorKind::TimedOut => write!(f, "Operation timed out - check network connection"),
                    _ => write!(f, "System error: {}", e),
                }
            },
            ChatError::Connection(msg) => write!(f, "Network issue: {}", msg),
            ChatError::Protocol(msg) => write!(f, "Communication error: {} (try reconnecting)", msg),
            ChatError::InvalidMessage(msg) => write!(f, "Received corrupted data: {}", msg),
            ChatError::PeerDisconnected => write!(f, "Your chat partner disconnected"),
            ChatError::BindFailed(addr, e) => {
                match e.kind() {
                    io::ErrorKind::AddrInUse => write!(f, "Port {} is already in use - try a different port", addr),
                    io::ErrorKind::PermissionDenied => write!(f, "Cannot use port {} - permission denied (try a port above 1024)", addr),
                    _ => write!(f, "Cannot start server on {}: {}", addr, e),
                }
            },
            ChatError::ConnectFailed(addr, e) => {
                match e.kind() {
                    io::ErrorKind::ConnectionRefused => write!(f, "Cannot reach {} - peer may not be running or firewall blocking", addr),
                    io::ErrorKind::TimedOut => write!(f, "Connection to {} timed out - check IP address and network", addr),
                    io::ErrorKind::InvalidInput => write!(f, "Invalid address '{}' - use format IP:PORT (e.g., 192.168.1.100:8080)", addr),
                    _ => write!(f, "Cannot connect to {}: {}", addr, e),
                }
            },
            ChatError::Encryption(msg) => write!(f, "Security error: {} (encryption may be compromised)", msg),
            ChatError::FileTransfer(msg) => {
                if msg.contains("too large") {
                    write!(f, "File too large: {}", msg)
                } else if msg.contains("hash mismatch") {
                    write!(f, "File corrupted during transfer - please try again")
                } else if msg.contains("Failed to create") {
                    write!(f, "Cannot save file: {} (check disk space and permissions)", msg)
                } else {
                    write!(f, "File transfer failed: {}", msg)
                }
            },
            ChatError::Configuration(msg) => {
                if msg.contains("Failed to read") {
                    write!(f, "Cannot read settings: {} (file may be corrupted)", msg)
                } else if msg.contains("Failed to write") {
                    write!(f, "Cannot save settings: {} (check permissions)", msg)
                } else {
                    write!(f, "Settings error: {}", msg)
                }
            },
        }
    }
}

impl Error for ChatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ChatError::Io(e) => Some(e),
            ChatError::BindFailed(_, e) | ChatError::ConnectFailed(_, e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for ChatError {
    fn from(error: io::Error) -> Self {
        ChatError::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, ChatError>;