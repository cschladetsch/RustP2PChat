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
            ChatError::Io(e) => write!(f, "I/O error: {}", e),
            ChatError::Connection(msg) => write!(f, "Connection error: {}", msg),
            ChatError::Protocol(msg) => write!(f, "Protocol error: {}", msg),
            ChatError::InvalidMessage(msg) => write!(f, "Invalid message: {}", msg),
            ChatError::PeerDisconnected => write!(f, "Peer disconnected"),
            ChatError::BindFailed(addr, e) => write!(f, "Failed to bind to {}: {}", addr, e),
            ChatError::ConnectFailed(addr, e) => write!(f, "Failed to connect to {}: {}", addr, e),
            ChatError::Encryption(msg) => write!(f, "Encryption error: {}", msg),
            ChatError::FileTransfer(msg) => write!(f, "File transfer error: {}", msg),
            ChatError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
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