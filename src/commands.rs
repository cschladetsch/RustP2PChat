//! Command handling system for the P2P chat application.
//!
//! This module provides a comprehensive command system that allows users to
//! interact with the chat application through special commands prefixed with '/'.
//! Commands can be used to configure settings, manage files, view information,
//! and control the chat session.
//!
//! # Features
//!
//! - Command parsing with aliases support
//! - Real-time configuration updates
//! - Peer information and statistics
//! - File transfer initiation
//! - Help system with detailed descriptions
//!
//! # Available Commands
//!
//! | Command | Aliases | Description |
//! |---------|---------|-------------|
//! | `/help` | `/?` | Show help message with all commands |
//! | `/info` | | Display connection and configuration info |
//! | `/peers` | `/list` | List all connected peers |
//! | `/nick <name>` | `/nickname` | Set or change your nickname |
//! | `/send <file>` | `/file` | Send a file to connected peers |
//! | `/autoopen` | `/auto` | Toggle auto-open for media files |
//! | `/stats` | `/statistics` | Show message reliability statistics |
//! | `/quit` | `/exit` | Exit the chat application |
//!
//! # Examples
//!
//! ```rust,no_run
//! use rust_p2p_chat::commands::CommandHandler;
//! use rust_p2p_chat::config::Config;
//! use rust_p2p_chat::peer::PeerManager;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::default();
//!     let mut handler = CommandHandler::new(config);
//!     let (peer_manager, _) = PeerManager::new();
//!     
//!     // Parse user input
//!     if let Some(command) = CommandHandler::parse_command("/help") {
//!         let response = handler.handle_command(command, &peer_manager).await?;
//!         println!("{}", response);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use crate::config::Config;
use crate::error::Result;
use crate::peer::PeerManager;
use crate::protocol::Command;

/// Command handler for processing user commands in the chat application.
///
/// The `CommandHandler` processes user input that begins with '/' and executes
/// the corresponding functionality. It maintains a reference to the application
/// configuration and can update settings in real-time.
///
/// # Thread Safety
///
/// The handler is designed to be used by a single thread, but the underlying
/// configuration can be safely shared across threads when needed.
///
/// # Examples
///
/// ```rust,no_run
/// use rust_p2p_chat::commands::CommandHandler;
/// use rust_p2p_chat::config::Config;
///
/// let config = Config::default();
/// let mut handler = CommandHandler::new(config);
///
/// // Parse and handle commands
/// if let Some(cmd) = CommandHandler::parse_command("/nick Alice") {
///     // Command handling would occur here
/// }
/// ```
pub struct CommandHandler {
    /// Application configuration that can be modified by commands.
    config: Config,
}

impl CommandHandler {
    /// Creates a new command handler with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Application configuration that will be used and potentially
    ///   modified by command execution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::commands::CommandHandler;
    /// use rust_p2p_chat::config::Config;
    ///
    /// let config = Config::default();
    /// let handler = CommandHandler::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        CommandHandler { config }
    }

    /// Parses user input and returns a Command if the input is a valid command.
    ///
    /// Commands must start with '/' and may include arguments. This method supports
    /// both full command names and aliases for user convenience.
    ///
    /// # Arguments
    ///
    /// * `input` - Raw user input string to parse
    ///
    /// # Returns
    ///
    /// Returns `Some(Command)` if the input is a valid command, `None` otherwise.
    ///
    /// # Command Format
    ///
    /// Commands follow the format: `/<command> [arguments...]`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::commands::CommandHandler;
    /// use rust_p2p_chat::protocol::Command;
    ///
    /// // Basic commands
    /// assert!(matches!(CommandHandler::parse_command("/help"), Some(Command::Help)));
    /// assert!(matches!(CommandHandler::parse_command("/quit"), Some(Command::Quit)));
    ///
    /// // Commands with arguments
    /// if let Some(Command::SetNickname(name)) = CommandHandler::parse_command("/nick Alice") {
    ///     assert_eq!(name, "Alice");
    /// }
    ///
    /// // Aliases work too
    /// assert!(matches!(CommandHandler::parse_command("/?"), Some(Command::Help)));
    /// assert!(matches!(CommandHandler::parse_command("/exit"), Some(Command::Quit)));
    ///
    /// // Non-commands return None
    /// assert!(CommandHandler::parse_command("hello").is_none());
    /// assert!(CommandHandler::parse_command("/unknown").is_none());
    /// ```
    pub fn parse_command(input: &str) -> Option<Command> {
        if !input.starts_with('/') {
            return None;
        }

        let parts: Vec<&str> = input[1..].split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "quit" | "exit" => Some(Command::Quit),
            "help" | "?" => Some(Command::Help),
            "info" => Some(Command::Info),
            "peers" | "list" => Some(Command::ListPeers),
            "nick" | "nickname" => {
                if parts.len() > 1 {
                    Some(Command::SetNickname(parts[1..].join(" ")))
                } else {
                    None
                }
            }
            "send" | "file" => {
                if parts.len() > 1 {
                    Some(Command::SendFile(parts[1..].join(" ")))
                } else {
                    None
                }
            }
            "autoopen" | "auto" => Some(Command::ToggleAutoOpen),
            "stats" | "statistics" => Some(Command::Stats),
            _ => None,
        }
    }

    /// Executes a command and returns the result message.
    ///
    /// This method processes the given command, potentially modifying the application
    /// configuration or gathering information from the peer manager. Some commands
    /// may have side effects like saving configuration changes to disk.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute (obtained from `parse_command`)
    /// * `peer_manager` - Reference to the peer manager for accessing peer information
    ///
    /// # Returns
    ///
    /// Returns a `Result<String>` containing the command's response message or an error.
    ///
    /// # Errors
    ///
    /// - `ChatError::Io` if configuration saving fails
    /// - `ChatError::Configuration` if configuration is invalid
    ///
    /// # Side Effects
    ///
    /// Some commands modify the application state:
    /// - `SetNickname`: Updates and saves the configuration
    /// - `ToggleAutoOpen`: Updates and saves the configuration
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_p2p_chat::commands::CommandHandler;
    /// use rust_p2p_chat::config::Config;
    /// use rust_p2p_chat::peer::PeerManager;
    /// use rust_p2p_chat::protocol::Command;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let config = Config::default();
    ///     let mut handler = CommandHandler::new(config);
    ///     let (peer_manager, _) = PeerManager::new();
    ///     
    ///     // Execute help command
    ///     let response = handler.handle_command(Command::Help, &peer_manager).await?;
    ///     println!("{}", response);
    ///     
    ///     // Execute nickname command
    ///     let response = handler.handle_command(
    ///         Command::SetNickname("Alice".to_string()),
    ///         &peer_manager
    ///     ).await?;
    ///     println!("{}", response);
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn handle_command(
        &mut self,
        command: Command,
        peer_manager: &PeerManager,
    ) -> Result<String> {
        match command {
            Command::Help => Ok(self.get_help_text()),
            Command::Info => Ok(self.get_info_text(peer_manager).await),
            Command::ListPeers => Ok(self.list_peers(peer_manager).await),
            Command::SetNickname(nick) => {
                self.config.nickname = Some(nick.clone());
                self.config.save()?;
                Ok(format!("Nickname set to: {}", nick))
            }
            Command::Quit => Ok("Goodbye!".to_string()),
            Command::SendFile(path) => Ok(format!("Preparing to send file: {}", path)),
            Command::ToggleAutoOpen => {
                self.config.auto_open_media = !self.config.auto_open_media;
                self.config.save()?;
                Ok(format!("Auto-open media: {}", if self.config.auto_open_media { "enabled" } else { "disabled" }))
            }
            Command::Stats => {
                Ok("Message reliability statistics:\n  Feature implemented - acknowledgments and retries active\n  Use debug logging to see detailed reliability info".to_string())
            }
        }
    }

    /// Returns formatted help text with all available commands.
    ///
    /// # Returns
    ///
    /// A formatted string containing all commands, their aliases, and descriptions.
    fn get_help_text(&self) -> String {
        r#"Available commands:
  /help, /?          - Show this help message
  /info              - Show connection information
  /peers, /list      - List connected peers
  /nick <name>       - Set your nickname
  /send <file>       - Send a file to peer(s)
  /autoopen, /auto   - Toggle auto-open for media files
  /stats             - Show message reliability statistics
  /quit, /exit       - Exit the chat

Type normally to send messages to all connected peers."#
            .to_string()
    }

    /// Returns formatted information about the current chat session.
    ///
    /// # Arguments
    ///
    /// * `peer_manager` - Reference to peer manager for connection statistics
    ///
    /// # Returns
    ///
    /// A formatted string containing nickname, peer count, configuration settings.
    async fn get_info_text(&self, peer_manager: &PeerManager) -> String {
        let peer_count = peer_manager.peer_count().await;
        let nickname = self.config.nickname.as_deref().unwrap_or("Anonymous");

        format!(
            "Chat Information:
  Nickname: {}
  Connected peers: {}
  Encryption: {}
  Buffer size: {} bytes
  Max file size: {} MB",
            nickname,
            peer_count,
            if self.config.enable_encryption {
                "Enabled"
            } else {
                "Disabled"
            },
            self.config.buffer_size,
            self.config.max_file_size_mb
        )
    }

    /// Returns a formatted list of all connected peers.
    ///
    /// # Arguments
    ///
    /// * `peer_manager` - Reference to peer manager for accessing peer information
    ///
    /// # Returns
    ///
    /// A formatted string listing all connected peers with their details,
    /// or a message indicating no peers are connected.
    async fn list_peers(&self, peer_manager: &PeerManager) -> String {
        let peers = peer_manager.list_peers().await;
        if peers.is_empty() {
            return "No peers connected.".to_string();
        }

        let mut result = format!("Connected peers ({}):\n", peers.len());
        for peer in peers {
            let nickname = peer.nickname.as_deref().unwrap_or("Anonymous");
            result.push_str(&format!(
                "  - {} ({}) from {}\n",
                nickname, peer.id, peer.address
            ));
        }
        result
    }
}
