use rust_p2p_chat::file_transfer::FileTransfer;
use std::fs;
use std::path::PathBuf;
use tempfile::{tempdir, NamedTempFile};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn test_file_transfer_basic() {
    let ft = FileTransfer::new(10); // 10MB limit
    let temp_dir = tempdir().unwrap();

    // Create a test file
    let test_content = b"Hello, file transfer!";
    let test_file_path = temp_dir.path().join("test_file.txt");
    fs::write(&test_file_path, test_content).unwrap();

    // Prepare file for transfer
    let file_info = ft.prepare_file(&test_file_path).await.unwrap();

    assert_eq!(file_info.size, test_content.len() as u64);
    assert_eq!(file_info.data, test_content);
    assert!(!file_info.hash.is_empty());
    assert!(!file_info.name.is_empty());
}

#[tokio::test]
async fn test_file_transfer_large_file() {
    let ft = FileTransfer::new(1); // 1MB limit
    let _temp_dir = tempdir().unwrap();

    // Create a file larger than the limit
    let large_content = vec![0u8; 2 * 1024 * 1024]; // 2MB
    let mut temp_file = NamedTempFile::new().unwrap();
    std::io::Write::write_all(&mut temp_file, &large_content).unwrap();
    std::io::Write::flush(&mut temp_file).unwrap();

    // Should fail due to size limit
    let result = ft.prepare_file(temp_file.path()).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("too large"));
}

#[tokio::test]
async fn test_file_transfer_save_and_verify() {
    let ft = FileTransfer::new(10);
    let temp_dir = tempdir().unwrap();

    // Create test file
    let test_content = b"Save and verify test content";
    let test_file_path = temp_dir.path().join("test_file.txt");
    fs::write(&test_file_path, test_content).unwrap();

    // Prepare and save file
    let file_info = ft.prepare_file(&test_file_path).await.unwrap();
    let save_dir = temp_dir.path().join("downloads");
    let saved_path = ft.save_file(&file_info, &save_dir).await.unwrap();

    // Verify saved file
    let saved_content = fs::read(&saved_path).unwrap();
    assert_eq!(saved_content, test_content);
}

#[tokio::test]
async fn test_file_transfer_hash_mismatch() {
    let ft = FileTransfer::new(10);
    let temp_dir = tempdir().unwrap();

    // Create test file
    let test_content = b"Hash mismatch test";
    let test_file_path = temp_dir.path().join("test_file.txt");
    fs::write(&test_file_path, test_content).unwrap();

    let mut file_info = ft.prepare_file(&test_file_path).await.unwrap();

    // Corrupt the hash
    file_info.hash = "invalid_hash".to_string();

    // Should fail due to hash mismatch
    let result = ft.save_file(&file_info, temp_dir.path()).await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("corrupted during transfer"));
}

#[test]
fn test_media_file_detection() {
    let media_extensions = vec![
        "jpg".to_string(),
        "png".to_string(),
        "mp4".to_string(),
        "pdf".to_string(),
        "txt".to_string(),
    ];

    assert!(FileTransfer::is_media_file("photo.jpg", &media_extensions));
    assert!(FileTransfer::is_media_file("image.PNG", &media_extensions)); // Case insensitive
    assert!(FileTransfer::is_media_file("video.mp4", &media_extensions));
    assert!(FileTransfer::is_media_file(
        "document.pdf",
        &media_extensions
    ));

    assert!(!FileTransfer::is_media_file("script.rs", &media_extensions));
    assert!(!FileTransfer::is_media_file("binary", &media_extensions)); // No extension
    assert!(!FileTransfer::is_media_file(
        "file.unknown",
        &media_extensions
    ));
}

#[tokio::test]
async fn test_file_transfer_nonexistent_file() {
    let ft = FileTransfer::new(10);
    let nonexistent_path = PathBuf::from("/nonexistent/file.txt");

    let result = ft.prepare_file(&nonexistent_path).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_file_transfer_directory_creation() {
    let ft = FileTransfer::new(10);
    let temp_dir = tempdir().unwrap();
    let nested_dir = temp_dir.path().join("nested").join("directory");

    // Create test file
    let test_content = b"Directory creation test";
    let test_file_path = temp_dir.path().join("test_file.txt");
    fs::write(&test_file_path, test_content).unwrap();

    let file_info = ft.prepare_file(&test_file_path).await.unwrap();

    // Save to nested directory (should create directories)
    let saved_path = ft.save_file(&file_info, &nested_dir).await.unwrap();

    assert!(saved_path.exists());
    assert!(nested_dir.exists());
}

#[tokio::test]
async fn test_file_transfer_empty_file() {
    let ft = FileTransfer::new(10);
    let temp_dir = tempdir().unwrap();

    // Create empty file
    let temp_file = NamedTempFile::new().unwrap();

    let file_info = ft.prepare_file(temp_file.path()).await.unwrap();
    assert_eq!(file_info.size, 0);
    assert!(file_info.data.is_empty());

    let saved_path = ft.save_file(&file_info, temp_dir.path()).await.unwrap();
    let saved_content = fs::read(&saved_path).unwrap();
    assert!(saved_content.is_empty());
}

#[tokio::test]
async fn test_file_transfer_unicode_filename() {
    let ft = FileTransfer::new(10);
    let temp_dir = tempdir().unwrap();

    // Create file with unicode content
    let test_content = "Hello, 世界! 🌍".as_bytes();
    let temp_file_path = temp_dir.path().join("测试文件.txt");

    let mut file = File::create(&temp_file_path).await.unwrap();
    file.write_all(test_content).await.unwrap();
    file.flush().await.unwrap();
    drop(file);

    let file_info = ft.prepare_file(&temp_file_path).await.unwrap();
    assert_eq!(file_info.name, "测试文件.txt");
    assert_eq!(file_info.data, test_content);

    let saved_path = ft.save_file(&file_info, temp_dir.path()).await.unwrap();
    let saved_content = fs::read(&saved_path).unwrap();
    assert_eq!(saved_content, test_content);
}
