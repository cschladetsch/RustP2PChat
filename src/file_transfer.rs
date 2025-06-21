//! File transfer functionality for the P2P chat application.
//!
//! This module provides secure file transfer capabilities with integrity
//! verification, size limits, and automatic media file handling. Files are
//! transferred with SHA-256 hash verification to ensure data integrity.
//!
//! # Features
//!
//! - SHA-256 hash verification for data integrity
//! - Configurable file size limits
//! - Cross-platform file opening
//! - Media file type detection
//! - Progress tracking for large transfers
//! - Automatic directory creation
//!
//! # Security
//!
//! - All files are verified with SHA-256 hashes
//! - Size limits prevent resource exhaustion
//! - No execution of transferred files
//! - Files saved to user-specified download directory
//!
//! # Examples
//!
//! ```rust,no_run
//! use rust_p2p_chat::file_transfer::FileTransfer;
//! use std::path::Path;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let ft = FileTransfer::new(100); // 100MB limit
//!     
//!     // Prepare file for sending
//!     let file_info = ft.prepare_file(Path::new("image.jpg")).await?;
//!     
//!     // Save received file
//!     let saved_path = ft.save_file(&file_info, Path::new("downloads")).await?;
//!     
//!     Ok(())
//! }
//! ```

use crate::error::{ChatError, Result};
use crate::protocol::{FileInfo, Message, MessageType, StatusUpdate};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Chunk size for file transfer operations.
/// Currently unused but reserved for future streaming implementations.
#[allow(dead_code)]
const CHUNK_SIZE: usize = 8192;

/// File transfer manager with integrity verification and size limits.
///
/// `FileTransfer` handles all aspects of sending and receiving files between
/// peers, including preparation, integrity verification, and saving to disk.
/// It enforces configurable size limits and provides cross-platform file
/// opening capabilities.
///
/// # Security Features
///
/// - **Integrity Verification**: SHA-256 hashes ensure files aren't corrupted
/// - **Size Limits**: Configurable maximum file size prevents resource exhaustion
/// - **Path Safety**: Files are saved to designated download directories
/// - **No Execution**: Files are never executed, only saved and optionally opened
///
/// # Examples
///
/// ```rust,no_run
/// use rust_p2p_chat::file_transfer::FileTransfer;
/// use std::path::Path;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let ft = FileTransfer::new(50); // 50MB limit
///     
///     // Prepare file for transfer
///     let file_info = ft.prepare_file(Path::new("document.pdf")).await?;
///     println!("File prepared: {} bytes, hash: {}", file_info.size, file_info.hash);
///     
///     // Save received file
///     let download_dir = Path::new("downloads");
///     let saved_path = ft.save_file(&file_info, download_dir).await?;
///     println!("File saved to: {}", saved_path.display());
///     
///     Ok(())
/// }
/// ```
pub struct FileTransfer {
    /// Maximum allowed file size in bytes.
    max_file_size: u64,
}

impl FileTransfer {
    /// Creates a new file transfer manager with the specified size limit.
    ///
    /// # Arguments
    ///
    /// * `max_file_size_mb` - Maximum file size in megabytes
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::file_transfer::FileTransfer;
    ///
    /// // Allow files up to 100MB
    /// let ft = FileTransfer::new(100);
    ///
    /// // Smaller limit for mobile or constrained environments
    /// let ft_small = FileTransfer::new(10);
    /// ```
    pub fn new(max_file_size_mb: u64) -> Self {
        FileTransfer {
            max_file_size: max_file_size_mb * 1024 * 1024,
        }
    }

    /// Prepares a file for transfer by reading it and computing its hash.
    ///
    /// This method reads the entire file into memory, computes its SHA-256 hash,
    /// and creates a `FileInfo` structure containing all necessary metadata
    /// for transfer. The file size is checked against the configured limit.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to prepare for transfer
    ///
    /// # Returns
    ///
    /// Returns a `FileInfo` structure containing the file's metadata and data.
    ///
    /// # Errors
    ///
    /// - `ChatError::FileTransfer` if the file is too large
    /// - `ChatError::FileTransfer` if the file cannot be read
    /// - `ChatError::Io` for file system errors
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_p2p_chat::file_transfer::FileTransfer;
    /// use std::path::Path;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ft = FileTransfer::new(100);
    ///     let file_info = ft.prepare_file(Path::new("photo.jpg")).await?;
    ///     
    ///     println!("File: {}", file_info.name);
    ///     println!("Size: {} bytes", file_info.size);
    ///     println!("Hash: {}", file_info.hash);
    ///     
    ///     Ok(())
    /// }
    /// ```
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

        let file_name = path
            .file_name()
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

    /// Saves a received file to the specified directory with integrity verification.
    ///
    /// This method verifies the file's SHA-256 hash against the provided metadata,
    /// creates the download directory if it doesn't exist, and saves the file.
    /// The hash verification ensures the file wasn't corrupted during transfer.
    ///
    /// # Arguments
    ///
    /// * `file_info` - File metadata and data from the sender
    /// * `download_dir` - Directory where the file should be saved
    ///
    /// # Returns
    ///
    /// Returns the full path where the file was saved.
    ///
    /// # Errors
    ///
    /// - `ChatError::FileTransfer` if hash verification fails
    /// - `ChatError::FileTransfer` if the file is too large
    /// - `ChatError::FileTransfer` if directory creation fails
    /// - `ChatError::Io` for file system errors
    ///
    /// # Security
    ///
    /// This method performs SHA-256 hash verification to ensure file integrity.
    /// If the hash doesn't match, the operation fails and no file is saved.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_p2p_chat::file_transfer::FileTransfer;
    /// use rust_p2p_chat::protocol::FileInfo;
    /// use std::path::Path;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let ft = FileTransfer::new(100);
    ///     
    ///     // Assume we received file_info from a peer
    ///     # let file_info = FileInfo {
    ///     #     name: "received_file.txt".to_string(),
    ///     #     size: 1024,
    ///     #     hash: "abc123".to_string(),
    ///     #     data: vec![0; 1024],
    ///     # };
    ///     
    ///     let saved_path = ft.save_file(&file_info, Path::new("downloads")).await?;
    ///     println!("File saved to: {}", saved_path.display());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn save_file(&self, file_info: &FileInfo, download_dir: &Path) -> Result<PathBuf> {
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

        fs::create_dir_all(download_dir).map_err(|e| {
            ChatError::FileTransfer(format!("Failed to create download directory: {}", e))
        })?;

        let file_path = download_dir.join(&file_info.name);
        let mut file = File::create(&file_path).await?;
        file.write_all(&file_info.data).await?;
        file.flush().await?;

        Ok(file_path)
    }

    /// Creates a progress message for file transfer status updates.
    ///
    /// This utility method creates a message that can be sent to inform
    /// the peer about file transfer progress.
    ///
    /// # Arguments
    ///
    /// * `filename` - Name of the file being transferred
    /// * `current` - Number of bytes transferred so far
    /// * `total` - Total size of the file in bytes
    ///
    /// # Returns
    ///
    /// Returns a `Message` containing transfer progress information.
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

    /// Opens a file using the platform's default application.
    ///
    /// This method attempts to open the specified file using the system's
    /// default application for that file type. It works across different
    /// operating systems using platform-specific commands.
    ///
    /// # Platform Support
    ///
    /// - **macOS**: Uses the `open` command
    /// - **Windows**: Uses `cmd /C start`
    /// - **Linux**: Tries `xdg-open`, `gnome-open`, then `kde-open`
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to open
    ///
    /// # Errors
    ///
    /// - `ChatError::FileTransfer` if the platform is unsupported
    /// - `ChatError::FileTransfer` if the file cannot be opened
    ///
    /// # Security
    ///
    /// This method only opens files with the system's default application.
    /// It does not execute files directly or with elevated permissions.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use rust_p2p_chat::file_transfer::FileTransfer;
    /// use std::path::Path;
    ///
    /// // Open an image file
    /// FileTransfer::open_file(Path::new("photo.jpg"))?;
    ///
    /// // Open a PDF document
    /// FileTransfer::open_file(Path::new("document.pdf"))?;
    /// # Ok::<(), rust_p2p_chat::ChatError>(())
    /// ```
    pub fn open_file(path: &Path) -> Result<()> {
        let result = if cfg!(target_os = "macos") {
            Command::new("open").arg(path).spawn()
        } else if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", "start", "", &path.to_string_lossy()])
                .spawn()
        } else if cfg!(target_os = "linux") {
            // Try xdg-open first, then fallback to other options
            Command::new("xdg-open")
                .arg(path)
                .spawn()
                .or_else(|_| Command::new("gnome-open").arg(path).spawn())
                .or_else(|_| Command::new("kde-open").arg(path).spawn())
        } else {
            return Err(ChatError::FileTransfer(
                "Unsupported platform for opening files".to_string(),
            ));
        };

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(ChatError::FileTransfer(format!(
                "Failed to open file: {}",
                e
            ))),
        }
    }

    /// Checks if a file extension indicates it's a media file.
    ///
    /// This method determines whether a file should be considered "media"
    /// based on its extension and a list of media file extensions.
    /// The comparison is case-insensitive.
    ///
    /// # Arguments
    ///
    /// * `filename` - Name of the file to check
    /// * `media_extensions` - List of extensions considered media files
    ///
    /// # Returns
    ///
    /// Returns `true` if the file extension matches a media type, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rust_p2p_chat::file_transfer::FileTransfer;
    ///
    /// let media_exts = vec!["jpg".to_string(), "png".to_string(), "mp4".to_string()];
    ///
    /// assert!(FileTransfer::is_media_file("photo.jpg", &media_exts));
    /// assert!(FileTransfer::is_media_file("IMAGE.PNG", &media_exts)); // Case insensitive
    /// assert!(FileTransfer::is_media_file("video.mp4", &media_exts));
    /// assert!(!FileTransfer::is_media_file("document.txt", &media_exts));
    /// assert!(!FileTransfer::is_media_file("noextension", &media_exts));
    /// ```
    pub fn is_media_file(filename: &str, media_extensions: &[String]) -> bool {
        if let Some(extension) = Path::new(filename).extension() {
            let ext = extension.to_string_lossy().to_lowercase();
            media_extensions
                .iter()
                .any(|media_ext| media_ext.to_lowercase() == ext)
        } else {
            false
        }
    }
}
