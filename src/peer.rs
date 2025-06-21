use crate::error::Result;
use crate::protocol::Message;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

#[derive(Clone)]
pub struct PeerInfo {
    pub id: String,
    pub nickname: Option<String>,
    pub address: SocketAddr,
    pub connected_at: std::time::SystemTime,
}

pub struct PeerManager {
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    #[allow(dead_code)]
    message_tx: mpsc::Sender<(String, Message)>,
}

pub struct Peer {
    pub info: PeerInfo,
    pub stream: Arc<TcpStream>,
    pub tx: mpsc::Sender<Message>,
}

impl PeerManager {
    pub fn new() -> (Self, mpsc::Receiver<(String, Message)>) {
        let (tx, rx) = mpsc::channel(100);
        (
            PeerManager {
                peers: Arc::new(Mutex::new(HashMap::new())),
                message_tx: tx,
            },
            rx,
        )
    }

    pub async fn add_peer(&self, id: String, peer: Peer) -> Result<()> {
        let mut peers = self.peers.lock().await;
        peers.insert(id, peer);
        Ok(())
    }

    pub async fn remove_peer(&self, id: &str) -> Option<Peer> {
        let mut peers = self.peers.lock().await;
        peers.remove(id)
    }

    pub async fn get_peer(&self, id: &str) -> Option<Peer> {
        let peers = self.peers.lock().await;
        peers.get(id).cloned()
    }

    pub async fn broadcast(&self, message: Message, exclude: Option<&str>) -> Result<()> {
        let peers = self.peers.lock().await;
        for (id, peer) in peers.iter() {
            if exclude.is_none_or(|ex| ex != id) {
                let _ = peer.tx.send(message.clone()).await;
            }
        }
        Ok(())
    }

    pub async fn list_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.lock().await;
        peers.values().map(|p| p.info.clone()).collect()
    }

    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.lock().await;
        peers.len()
    }
}

impl Clone for Peer {
    fn clone(&self) -> Self {
        Peer {
            info: self.info.clone(),
            stream: self.stream.clone(),
            tx: self.tx.clone(),
        }
    }
}
