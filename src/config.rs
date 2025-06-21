use serde::{Serialize, Deserialize};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs;
use crate::error::{ChatError, Result};

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
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        if let Some(config_path) = Self::config_path() {
            if config_path.exists() {
                let contents = fs::read_to_string(&config_path)
                    .map_err(|e| ChatError::Configuration(format!("Failed to read config: {}", e)))?;
                let config: Config = toml::from_str(&contents)
                    .map_err(|e| ChatError::Configuration(format!("Failed to parse config: {}", e)))?;
                return Ok(config);
            }
        }
        Ok(Config::default())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(config_path) = Self::config_path() {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| ChatError::Configuration(format!("Failed to create config dir: {}", e)))?;
            }
            let contents = toml::to_string_pretty(self)
                .map_err(|e| ChatError::Configuration(format!("Failed to serialize config: {}", e)))?;
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
}