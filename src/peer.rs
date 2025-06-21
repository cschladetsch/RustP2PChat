//! Peer management system for P2P chat connections.
//!
//! This module provides functionality for managing multiple peer connections
//! in the P2P chat application. It handles peer registration, message broadcasting,
//! connection tracking, and concurrent access to peer information.
//!
//! # Features
//!
//! - Concurrent peer management with thread-safe operations
//! - Message broadcasting to multiple peers
//! - Peer information tracking (nickname, address, connection time)
//! - Automatic peer cleanup on disconnection
//! - Message passing between peers using channels
//!
//! # Architecture
//!
//! The peer management system uses the following components:
//! - `PeerManager`: Central coordinator for all peer operations
//! - `Peer`: Individual peer connection wrapper
//! - `PeerInfo`: Metadata about each connected peer
//!
//! # Thread Safety
//!
//! All operations are thread-safe using Arc<Mutex<T>> for shared state
//! and mpsc channels for message passing between async tasks.
//!
//! # Examples
//!
//! ```rust,no_run
//! use rust_p2p_chat::peer::{PeerManager, Peer, PeerInfo};
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (peer_manager, mut message_rx) = PeerManager::new();
//!     
//!     // Get peer count
//!     let count = peer_manager.peer_count().await;
//!     println!("Connected peers: {}", count);
//!     
//!     // List all peers
//!     let peers = peer_manager.list_peers().await;
//!     for peer in peers {
//!         println!("Peer: {} at {}", peer.id, peer.address);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use crate::error::Result;
use crate::protocol::Message;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

/// Information about a connected peer in the chat network.
///
/// Contains metadata about each peer including identification, network address,
/// optional nickname, and connection timestamp. This information is used for
/// display purposes and connection management.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::peer::PeerInfo;
/// use std::net::SocketAddr;
///
/// let peer_info = PeerInfo {
///     id: "peer123".to_string(),
///     nickname: Some("Alice".to_string()),
///     address: "127.0.0.1:8080".parse().unwrap(),
///     connected_at: std::time::SystemTime::now(),
/// };
///
/// println!("Peer {} connected from {}", peer_info.id, peer_info.address);
/// ```
#[derive(Clone, Debug)]
pub struct PeerInfo {
    /// Unique identifier for the peer.
    pub id: String,
    /// Optional display name set by the peer.
    pub nickname: Option<String>,
    /// Network address of the peer connection.
    pub address: SocketAddr,
    /// Timestamp when the connection was established.
    pub connected_at: std::time::SystemTime,
}

/// Central manager for all peer connections in the chat network.
///
/// The `PeerManager` coordinates all peer-related operations including adding
/// and removing peers, broadcasting messages, and providing peer information.
/// It uses thread-safe data structures to handle concurrent access from
/// multiple async tasks.
///
/// # Thread Safety
///
/// All operations are thread-safe and can be called concurrently from
/// multiple async tasks. The internal state is protected by Arc<Mutex<T>>.
///
/// # Examples
///
/// ```rust,no_run
/// use rust_p2p_chat::peer::PeerManager;
/// use rust_p2p_chat::protocol::Message;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let (peer_manager, _rx) = PeerManager::new();
///     
///     // Check peer count
///     let count = peer_manager.peer_count().await;
///     println!("Active peers: {}", count);
///     
///     // Broadcast a message to all peers
///     let msg = Message::new_text("Hello everyone!".to_string());
///     peer_manager.broadcast(msg, None).await?;
///     
///     Ok(())
/// }
/// ```
pub struct PeerManager {
    /// Thread-safe storage for all connected peers.
    peers: Arc<Mutex<HashMap<String, Peer>>>,
    /// Channel for sending messages between peers (reserved for future use).
    #[allow(dead_code)]
    message_tx: mpsc::Sender<(String, Message)>,
}

/// Represents an individual peer connection in the chat network.
///
/// A `Peer` contains all the necessary information and communication channels
/// for interacting with a connected peer. It wraps the TCP stream and provides
/// a message channel for sending data to the peer.
///
/// # Examples
///
/// ```rust,no_run
/// use rust_p2p_chat::peer::{Peer, PeerInfo};
/// use tokio::sync::mpsc;
/// use std::sync::Arc;
///
/// // Note: This is a conceptual example - actual peer creation
/// // is handled by the PeerManager in practice
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
/// # let (tx, _rx) = mpsc::channel(100);
/// # let info = PeerInfo {
/// #     id: "peer1".to_string(),
/// #     nickname: None,
/// #     address: "127.0.0.1:8080".parse()?,
/// #     connected_at: std::time::SystemTime::now(),
/// # };
/// let peer = Peer {
///     info,
///     stream: Arc::new(stream),
///     tx,
/// };
/// # Ok(())
/// # }
/// ```
pub struct Peer {
    /// Metadata about this peer.
    pub info: PeerInfo,
    /// Shared TCP stream for network communication.
    pub stream: Arc<TcpStream>,
    /// Channel sender for queuing messages to this peer.
    pub tx: mpsc::Sender<Message>,
}

impl PeerManager {
    /// Creates a new peer manager with an empty peer list.
    ///
    /// # Returns
    ///
    /// A tuple containing the PeerManager instance and a message receiver
    /// channel for handling inter-peer messages.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::peer::PeerManager;
    ///
    /// let (peer_manager, message_rx) = PeerManager::new();
    /// // Use peer_manager for peer operations
    /// // Use message_rx to receive messages from peers
    /// ```
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

    /// Adds a new peer to the manager.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the peer
    /// * `peer` - The peer instance to add
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rust_p2p_chat::peer::{PeerManager, Peer, PeerInfo};
    /// # use tokio::sync::mpsc;
    /// # use std::sync::Arc;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (peer_manager, _) = PeerManager::new();
    /// // peer creation omitted for brevity
    /// # let stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;
    /// # let (tx, _rx) = mpsc::channel(100);
    /// # let info = PeerInfo { id: "peer1".to_string(), nickname: None, address: "127.0.0.1:8080".parse()?, connected_at: std::time::SystemTime::now() };
    /// # let peer = Peer { info, stream: Arc::new(stream), tx };
    /// peer_manager.add_peer("peer1".to_string(), peer).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_peer(&self, id: String, peer: Peer) -> Result<()> {
        let mut peers = self.peers.lock().await;
        peers.insert(id, peer);
        Ok(())
    }

    /// Removes a peer from the manager.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the peer to remove
    ///
    /// # Returns
    ///
    /// Returns the removed `Peer` if it existed, `None` otherwise.
    pub async fn remove_peer(&self, id: &str) -> Option<Peer> {
        let mut peers = self.peers.lock().await;
        peers.remove(id)
    }

    /// Retrieves a peer by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the peer to retrieve
    ///
    /// # Returns
    ///
    /// Returns a cloned `Peer` if found, `None` otherwise.
    pub async fn get_peer(&self, id: &str) -> Option<Peer> {
        let peers = self.peers.lock().await;
        peers.get(id).cloned()
    }

    /// Broadcasts a message to all connected peers.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to broadcast
    /// * `exclude` - Optional peer ID to exclude from the broadcast
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success. Individual send failures are ignored.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use rust_p2p_chat::peer::PeerManager;
    /// # use rust_p2p_chat::protocol::Message;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let (peer_manager, _) = PeerManager::new();
    /// let msg = Message::new_text("Hello everyone!".to_string());
    /// 
    /// // Broadcast to all peers
    /// peer_manager.broadcast(msg.clone(), None).await?;
    /// 
    /// // Broadcast to all except "peer1"
    /// peer_manager.broadcast(msg, Some("peer1")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn broadcast(&self, message: Message, exclude: Option<&str>) -> Result<()> {
        let peers = self.peers.lock().await;
        for (id, peer) in peers.iter() {
            if exclude.is_none_or(|ex| ex != id) {
                let _ = peer.tx.send(message.clone()).await;
            }
        }
        Ok(())
    }

    /// Returns a list of all connected peers' information.
    ///
    /// # Returns
    ///
    /// A vector containing `PeerInfo` for all connected peers.
    pub async fn list_peers(&self) -> Vec<PeerInfo> {
        let peers = self.peers.lock().await;
        peers.values().map(|p| p.info.clone()).collect()
    }

    /// Returns the number of currently connected peers.
    ///
    /// # Returns
    ///
    /// The count of active peer connections.
    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.lock().await;
        peers.len()
    }
}

impl Clone for Peer {
    /// Creates a clone of the peer.
    ///
    /// All components (info, stream, and channel) are cloned, allowing
    /// the peer to be shared across multiple async tasks safely.
    fn clone(&self) -> Self {
        Peer {
            info: self.info.clone(),
            stream: self.stream.clone(),
            tx: self.tx.clone(),
        }
    }
}
