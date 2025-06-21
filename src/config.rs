//! Configuration management for the P2P chat application.
//!
//! This module provides configuration loading, saving, and default values
//! for the Rust P2P Chat application. Configuration is stored in TOML format
//! and automatically located in platform-appropriate directories.
//!
//! # Features
//!
//! - TOML-based configuration files
//! - Platform-specific config directories
//! - Automatic fallback to sensible defaults
//! - File path resolution for downloads and history
//! - Media file extension configuration
//!
//! # Examples
//!
//! ```rust
//! use rust_p2p_chat::Config;
//!
//! // Load configuration (or create default)
//! let config = Config::load().unwrap();
//!
//! // Modify and save
//! let mut config = Config::default();
//! config.nickname = Some("Alice".to_string());
//! config.save().unwrap();
//! ```

use crate::error::{ChatError, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration structure.
///
/// Contains all configurable settings for the P2P chat application,
/// including network settings, file transfer options, encryption preferences,
/// and user interface customization.
///
/// # Configuration File Location
///
/// - **Linux**: `~/.config/rustchat/p2p-chat/config.toml`
/// - **macOS**: `~/Library/Application Support/rustchat/p2p-chat/config.toml`
/// - **Windows**: `%APPDATA%\rustchat\p2p-chat\config.toml`
///
/// # Examples
///
/// ```rust
/// use rust_p2p_chat::Config;
///
/// // Create default configuration
/// let config = Config::default();
/// assert_eq!(config.default_port, 8080);
/// assert_eq!(config.buffer_size, 8192);
/// assert!(config.enable_encryption);
///
/// // Load from file (or create default if not found)
/// let config = Config::load().unwrap();
///
/// // Customize and save
/// let mut config = Config::default();
/// config.nickname = Some("Alice".to_string());
/// config.default_port = 9000;
/// config.save().unwrap();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// User's display name in chat sessions.
    /// If None, a default nickname will be generated.
    pub nickname: Option<String>,
    
    /// Default port to listen on when no port is specified.
    /// Valid range: 1024-65535 (privileged ports require elevated permissions).
    pub default_port: u16,
    
    /// Size of the message buffer in bytes.
    /// Larger buffers can handle bigger messages but use more memory.
    pub buffer_size: usize,
    
    /// Interval between heartbeat messages in seconds.
    /// Used to detect disconnected peers and maintain connections.
    pub heartbeat_interval_secs: u64,
    
    /// Number of times to retry failed connection attempts.
    /// Set to 0 to disable automatic reconnection.
    pub reconnect_attempts: u32,
    
    /// Delay in seconds between reconnection attempts.
    /// Uses exponential backoff starting from this value.
    pub reconnect_delay_secs: u64,
    
    /// Whether to enable end-to-end encryption by default.
    /// When true, all messages are encrypted using RSA + AES-256-GCM.
    pub enable_encryption: bool,
    
    /// Logging level for the application.
    /// Valid values: "trace", "debug", "info", "warn", "error"
    pub log_level: String,
    
    /// Whether to save chat history to a file.
    /// History includes messages, file transfers, and connection events.
    pub save_history: bool,
    
    /// Custom path for the chat history file.
    /// If None, uses platform-specific data directory.
    pub history_file: Option<PathBuf>,
    
    /// Maximum file size for transfers in megabytes.
    /// Files larger than this limit will be rejected.
    pub max_file_size_mb: u64,
    
    /// Custom directory for downloaded files.
    /// If None, uses the system's Downloads folder.
    pub download_dir: Option<PathBuf>,
    
    /// Whether to automatically open received media files.
    /// Uses the system's default application for each file type.
    pub auto_open_media: bool,
    
    /// File extensions that are considered "media" for auto-opening.
    /// Extensions are matched case-insensitively.
    pub media_extensions: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            nickname: None,
            default_port: 8080,
            buffer_size: 8192,
            heartbeat_interval_secs: 30,
            reconnect_attempts: 3,
            reconnect_delay_secs: 5,
            enable_encryption: true,
            log_level: "info".to_string(),
            save_history: true,
            history_file: None,
            max_file_size_mb: 100,
            download_dir: None,
            auto_open_media: true,
            media_extensions: vec![
                "jpg".to_string(),
                "jpeg".to_string(),
                "png".to_string(),
                "gif".to_string(),
                "bmp".to_string(),
                "webp".to_string(),
                "svg".to_string(),
                "mp4".to_string(),
                "avi".to_string(),
                "mov".to_string(),
                "wmv".to_string(),
                "mp3".to_string(),
                "wav".to_string(),
                "flac".to_string(),
                "aac".to_string(),
                "pdf".to_string(),
                "doc".to_string(),
                "docx".to_string(),
                "txt".to_string(),
            ],
        }
    }
}

impl Config {
    /// Loads configuration from the config file, or creates default if not found.
    ///
    /// This method attempts to load configuration from the platform-specific
    /// config directory. If no config file exists, it returns the default
    /// configuration without creating a file.
    ///
    /// # Returns
    ///
    /// Returns the loaded configuration or default values if no config file exists.
    ///
    /// # Errors
    ///
    /// - `ChatError::Configuration` if the config file exists but is invalid
    /// - File I/O errors when reading the config file
    /// - TOML parsing errors for malformed config files
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::Config;
    ///
    /// let config = Config::load().unwrap();
    /// println!("Using port: {}", config.default_port);
    /// ```
    pub fn load() -> Result<Self> {
        if let Some(config_path) = Self::config_path() {
            if config_path.exists() {
                let contents = fs::read_to_string(&config_path).map_err(|e| {
                    ChatError::Configuration(format!("Failed to read config: {}", e))
                })?;
                let config: Config = toml::from_str(&contents).map_err(|e| {
                    ChatError::Configuration(format!("Failed to parse config: {}", e))
                })?;
                return Ok(config);
            }
        }
        Ok(Config::default())
    }

    /// Saves the current configuration to the config file.
    ///
    /// This method serializes the configuration to TOML format and saves it
    /// to the platform-specific config directory. It automatically creates
    /// the necessary parent directories if they don't exist.
    ///
    /// # Errors
    ///
    /// - `ChatError::Configuration` if serialization fails
    /// - File I/O errors when creating directories or writing the file
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::Config;
    ///
    /// let mut config = Config::default();
    /// config.nickname = Some("Alice".to_string());
    /// config.save().unwrap();
    /// ```
    pub fn save(&self) -> Result<()> {
        if let Some(config_path) = Self::config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    ChatError::Configuration(format!("Failed to create config dir: {}", e))
                })?;
            }
            let contents = toml::to_string_pretty(self).map_err(|e| {
                ChatError::Configuration(format!("Failed to serialize config: {}", e))
            })?;
            fs::write(&config_path, contents)
                .map_err(|e| ChatError::Configuration(format!("Failed to write config: {}", e)))?;
        }
        Ok(())
    }

    /// Returns the platform-specific path for the configuration file.
    ///
    /// Uses the `directories` crate to find the appropriate config directory
    /// for the current platform:
    /// - Linux: `~/.config/rustchat/p2p-chat/config.toml`
    /// - macOS: `~/Library/Application Support/rustchat/p2p-chat/config.toml`
    /// - Windows: `%APPDATA%\rustchat\p2p-chat\config.toml`
    ///
    /// # Returns
    ///
    /// Returns `Some(PathBuf)` with the config file path, or `None` if the
    /// platform-specific directories cannot be determined.
    fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "rustchat", "p2p-chat")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }

    /// Returns the path where chat history should be stored.
    ///
    /// If a custom history file path is configured, returns that path.
    /// Otherwise, returns the platform-specific data directory path.
    ///
    /// # Returns
    ///
    /// Returns `Some(PathBuf)` with the history file path, or `None` if
    /// neither a custom path is set nor platform directories can be determined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::Config;
    ///
    /// let config = Config::default();
    /// if let Some(history_path) = config.history_path() {
    ///     println!("History will be saved to: {}", history_path.display());
    /// }
    /// ```
    pub fn history_path(&self) -> Option<PathBuf> {
        if let Some(ref path) = self.history_file {
            return Some(path.clone());
        }
        ProjectDirs::from("com", "rustchat", "p2p-chat")
            .map(|dirs| dirs.data_dir().join("chat_history.json"))
    }

    /// Returns the directory where downloaded files should be saved.
    ///
    /// If a custom download directory is configured, returns that path.
    /// Otherwise, attempts to use the system's Downloads folder.
    /// Falls back to the current working directory if all else fails.
    ///
    /// # Returns
    ///
    /// Always returns a valid `PathBuf`. Never fails, but may return
    /// the current directory as a last resort.
    ///
    /// # Platform Behavior
    ///
    /// - **Linux**: `~/Downloads`
    /// - **macOS**: `~/Downloads`
    /// - **Windows**: `%USERPROFILE%\Downloads`
    /// - **Fallback**: Current working directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::Config;
    ///
    /// let config = Config::default();
    /// let download_dir = config.download_path();
    /// println!("Files will be saved to: {}", download_dir.display());
    /// ```
    pub fn download_path(&self) -> PathBuf {
        if let Some(ref path) = self.download_dir {
            return path.clone();
        }
        // Default to Downloads folder or current directory
        if let Some(dirs) = directories::UserDirs::new() {
            if let Some(download_dir) = dirs.download_dir() {
                return download_dir.to_path_buf();
            }
        }
        // Fallback to current directory
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}
