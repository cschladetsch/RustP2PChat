//! Message reliability system with acknowledgments and retries.
//!
//! This module provides reliable message delivery for the P2P chat application
//! using acknowledgment-based confirmation and automatic retry mechanisms.
//! It ensures that important messages are delivered even in the presence of
//! network issues or temporary connection problems.
//!
//! # Features
//!
//! - Automatic message acknowledgment tracking
//! - Configurable retry attempts and delays
//! - Message timeout handling
//! - Background cleanup of expired messages
//! - Delivery confirmation guarantees
//!
//! # Reliability Protocol
//!
//! 1. **Message Sending**: Messages are stored as "pending" until acknowledged
//! 2. **Acknowledgment**: Recipients send back ACK messages with original message IDs
//! 3. **Retry Logic**: Unacknowledged messages are retried after configurable delays
//! 4. **Timeout Handling**: Messages that exceed retry limits are marked as failed
//! 5. **Cleanup**: Expired and acknowledged messages are removed automatically
//!
//! # Configuration
//!
//! The reliability system is highly configurable:
//! - **Retry Attempts**: How many times to retry failed messages (default: 3)
//! - **Retry Delay**: Time between retry attempts (default: 2 seconds)
//! - **ACK Timeout**: How long to wait for acknowledgments (default: 10 seconds)
//! - **Cleanup Interval**: How often to clean expired messages (default: 30 seconds)
//!
//! # Examples
//!
//! ```rust,no_run
//! use rust_p2p_chat::reliability::{ReliabilityManager, ReliabilityConfig};
//! use rust_p2p_chat::protocol::Message;
//! use tokio::sync::mpsc;
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create configuration
//!     let config = ReliabilityConfig {
//!         retry_attempts: 5,
//!         retry_delay: Duration::from_secs(3),
//!         ack_timeout: Duration::from_secs(15),
//!         cleanup_interval: Duration::from_secs(60),
//!     };
//!     
//!     // Create channel for outbound messages
//!     let (outbound_tx, mut outbound_rx) = mpsc::channel(100);
//!     
//!     // Create reliability manager
//!     let mut manager = ReliabilityManager::new(config, outbound_tx);
//!     
//!     // Send a message with reliability
//!     let msg = Message::new_text("Important message".to_string());
//!     manager.send_reliable(msg).await?;
//!     
//!     // Handle acknowledgments
//!     let message_id = 123;
//!     manager.handle_acknowledgment(message_id);
//!     
//!     Ok(())
//! }
//! ```

use crate::error::Result;
use crate::protocol::Message;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::{debug, error, trace, warn};

/// Configuration parameters for the message reliability system.
///
/// Controls all aspects of message delivery reliability including retry behavior,
/// timeout handling, and cleanup intervals. These settings can be tuned based
/// on network conditions and application requirements.
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::reliability::ReliabilityConfig;
/// use std::time::Duration;
///
/// // Default configuration
/// let config = ReliabilityConfig::default();
///
/// // Custom configuration for unreliable networks
/// let custom_config = ReliabilityConfig {
///     retry_attempts: 5,           // More retries
///     retry_delay: Duration::from_secs(5),  // Longer delays
///     ack_timeout: Duration::from_secs(20), // More time for ACKs
///     cleanup_interval: Duration::from_secs(60), // Less frequent cleanup
/// };
/// ```
#[derive(Clone, Debug)]
pub struct ReliabilityConfig {
    /// Maximum number of retry attempts for unacknowledged messages.
    pub retry_attempts: u8,
    /// Time to wait between retry attempts.
    pub retry_delay: Duration,
    /// Maximum time to wait for message acknowledgment before giving up.
    pub ack_timeout: Duration,
    /// Interval between cleanup operations for expired messages.
    pub cleanup_interval: Duration,
}

impl Default for ReliabilityConfig {
    fn default() -> Self {
        Self {
            retry_attempts: 3,
            retry_delay: Duration::from_secs(2),
            ack_timeout: Duration::from_secs(10),
            cleanup_interval: Duration::from_secs(30),
        }
    }
}

/// Tracks pending messages awaiting acknowledgment
#[derive(Debug)]
struct PendingMessage {
    message: Message,
    sent_at: Instant,
    retry_count: u8,
    next_retry: Instant,
}

/// Manages message reliability with acknowledgments and retries
pub struct ReliabilityManager {
    config: ReliabilityConfig,
    pending_messages: HashMap<u64, PendingMessage>,
    outbound_tx: mpsc::Sender<Message>,
}

impl ReliabilityManager {
    pub fn new(config: ReliabilityConfig, outbound_tx: mpsc::Sender<Message>) -> Self {
        Self {
            config,
            pending_messages: HashMap::new(),
            outbound_tx,
        }
    }

    /// Send a message with reliability guarantees
    pub async fn send_reliable(&mut self, message: Message) -> Result<()> {
        let message_id = message.id;
        debug!("Sending reliable message with ID: {}", message_id);

        // Send the message immediately
        self.outbound_tx
            .send(message.clone())
            .await
            .map_err(|_| crate::error::ChatError::PeerDisconnected)?;

        // Track it for acknowledgment
        let pending = PendingMessage {
            message,
            sent_at: Instant::now(),
            retry_count: 0,
            next_retry: Instant::now() + self.config.retry_delay,
        };

        self.pending_messages.insert(message_id, pending);
        trace!("Added message {} to pending list", message_id);
        Ok(())
    }

    /// Handle an incoming acknowledgment
    pub fn handle_acknowledgment(&mut self, message_id: u64) {
        if let Some(pending) = self.pending_messages.remove(&message_id) {
            let elapsed = pending.sent_at.elapsed();
            debug!(
                "Received ACK for message {} after {:?}",
                message_id, elapsed
            );
        } else {
            warn!("Received ACK for unknown message ID: {}", message_id);
        }
    }

    /// Process retries and timeouts
    pub async fn process_retries(&mut self) {
        let now = Instant::now();
        let mut to_retry = Vec::new();
        let mut to_timeout = Vec::new();

        // Check all pending messages
        for (id, pending) in &self.pending_messages {
            if now >= pending.next_retry {
                if pending.retry_count >= self.config.retry_attempts {
                    // Message has timed out
                    to_timeout.push(*id);
                } else {
                    // Message needs retry
                    to_retry.push(*id);
                }
            }
        }

        // Handle timeouts
        for id in to_timeout {
            if let Some(pending) = self.pending_messages.remove(&id) {
                warn!(
                    "Message {} timed out after {} retries",
                    id, pending.retry_count
                );
            }
        }

        // Handle retries
        for id in to_retry {
            if let Some(pending) = self.pending_messages.get_mut(&id) {
                pending.retry_count += 1;
                pending.next_retry = now + self.config.retry_delay;

                debug!(
                    "Retrying message {} (attempt {}/{})",
                    id, pending.retry_count, self.config.retry_attempts
                );

                // Resend the message
                if let Err(e) = self.outbound_tx.send(pending.message.clone()).await {
                    error!("Failed to resend message {}: {:?}", id, e);
                    self.pending_messages.remove(&id);
                }
            }
        }
    }

    /// Clean up old acknowledged messages
    pub fn cleanup_old_messages(&mut self) {
        let cutoff = Instant::now() - self.config.ack_timeout;
        let initial_count = self.pending_messages.len();

        self.pending_messages.retain(|id, pending| {
            if pending.sent_at < cutoff {
                warn!("Cleaning up very old pending message: {}", id);
                false
            } else {
                true
            }
        });

        let cleaned = initial_count - self.pending_messages.len();
        if cleaned > 0 {
            debug!("Cleaned up {} old pending messages", cleaned);
        }
    }

    /// Get statistics about pending messages
    pub fn get_stats(&self) -> ReliabilityStats {
        let mut by_retry_count = HashMap::new();
        let _now = Instant::now();

        for pending in self.pending_messages.values() {
            let count = by_retry_count.entry(pending.retry_count).or_insert(0);
            *count += 1;
        }

        ReliabilityStats {
            total_pending: self.pending_messages.len(),
            retry_distribution: by_retry_count,
        }
    }

    /// Start the reliability manager background task
    pub async fn run_background_task(mut self) {
        let mut retry_interval = interval(self.config.retry_delay);
        let mut cleanup_interval = interval(self.config.cleanup_interval);

        debug!("Starting reliability manager background task");

        loop {
            tokio::select! {
                _ = retry_interval.tick() => {
                    self.process_retries().await;
                }
                _ = cleanup_interval.tick() => {
                    self.cleanup_old_messages();
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ReliabilityStats {
    pub total_pending: usize,
    pub retry_distribution: HashMap<u8, usize>,
}

impl std::fmt::Display for ReliabilityStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pending messages: {}", self.total_pending)?;
        if !self.retry_distribution.is_empty() {
            write!(f, " (retries: ")?;
            for (retry_count, count) in &self.retry_distribution {
                write!(f, "{}:{} ", retry_count, count)?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}
