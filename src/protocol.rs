use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text(String),
    File(FileInfo),
    Command(Command),
    Status(StatusUpdate),
    Heartbeat,
    Acknowledgment(u64), // Message ID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub timestamp: SystemTime,
    pub msg_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub hash: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Quit,
    Help,
    Info,
    ListPeers,
    SendFile(String),
    SetNickname(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusUpdate {
    PeerConnected(String),
    PeerDisconnected(String),
    TransferProgress(String, u64, u64), // filename, current, total
    Error(String),
}

impl Message {
    pub fn new_text(text: String) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Text(text),
        }
    }

    pub fn new_command(cmd: Command) -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Command(cmd),
        }
    }

    pub fn new_heartbeat() -> Self {
        Message {
            id: rand::random(),
            timestamp: SystemTime::now(),
            msg_type: MessageType::Heartbeat,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}