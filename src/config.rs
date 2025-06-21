use crate::error::{ChatError, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub nickname: Option<String>,
    pub default_port: u16,
    pub buffer_size: usize,
    pub heartbeat_interval_secs: u64,
    pub reconnect_attempts: u32,
    pub reconnect_delay_secs: u64,
    pub enable_encryption: bool,
    pub log_level: String,
    pub save_history: bool,
    pub history_file: Option<PathBuf>,
    pub max_file_size_mb: u64,
    pub download_dir: Option<PathBuf>,
    pub auto_open_media: bool,
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

    fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "rustchat", "p2p-chat")
            .map(|dirs| dirs.config_dir().join("config.toml"))
    }

    pub fn history_path(&self) -> Option<PathBuf> {
        if let Some(ref path) = self.history_file {
            return Some(path.clone());
        }
        ProjectDirs::from("com", "rustchat", "p2p-chat")
            .map(|dirs| dirs.data_dir().join("chat_history.json"))
    }

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
