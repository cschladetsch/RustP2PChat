use std::path::Path;
use std::fs;
use sha2::{Sha256, Digest};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::protocol::{FileInfo, Message, MessageType, StatusUpdate};
use crate::error::{ChatError, Result};

#[allow(dead_code)]
const CHUNK_SIZE: usize = 8192;

pub struct FileTransfer {
    max_file_size: u64,
}

impl FileTransfer {
    pub fn new(max_file_size_mb: u64) -> Self {
        FileTransfer {
            max_file_size: max_file_size_mb * 1024 * 1024,
        }
    }

    pub async fn prepare_file(&self, path: &Path) -> Result<FileInfo> {
        let metadata = fs::metadata(path)
            .map_err(|e| ChatError::FileTransfer(format!("Failed to read file metadata: {}", e)))?;
        
        if metadata.len() > self.max_file_size {
            return Err(ChatError::FileTransfer(format!(
                "File too large: {} MB (max: {} MB)",
                metadata.len() / 1024 / 1024,
                self.max_file_size / 1024 / 1024
            )));
        }

        let file_name = path.file_name()
            .ok_or_else(|| ChatError::FileTransfer("Invalid file name".to_string()))?
            .to_string_lossy()
            .to_string();

        let mut file = File::open(path).await?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;

        let hash = format!("{:x}", Sha256::digest(&data));

        Ok(FileInfo {
            name: file_name,
            size: metadata.len(),
            hash,
            data,
        })
    }

    pub async fn save_file(&self, file_info: &FileInfo, download_dir: &Path) -> Result<()> {
        if file_info.size > self.max_file_size {
            return Err(ChatError::FileTransfer(format!(
                "File too large: {} MB (max: {} MB)",
                file_info.size / 1024 / 1024,
                self.max_file_size / 1024 / 1024
            )));
        }

        // Verify hash
        let hash = format!("{:x}", Sha256::digest(&file_info.data));
        if hash != file_info.hash {
            return Err(ChatError::FileTransfer("File hash mismatch".to_string()));
        }

        fs::create_dir_all(download_dir)
            .map_err(|e| ChatError::FileTransfer(format!("Failed to create download directory: {}", e)))?;

        let file_path = download_dir.join(&file_info.name);
        let mut file = File::create(&file_path).await?;
        file.write_all(&file_info.data).await?;

        Ok(())
    }

    pub fn create_progress_message(filename: &str, current: u64, total: u64) -> Message {
        Message {
            id: rand::random(),
            timestamp: std::time::SystemTime::now(),
            msg_type: MessageType::Status(StatusUpdate::TransferProgress(
                filename.to_string(),
                current,
                total,
            )),
        }
    }
}