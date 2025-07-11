# Windows Drag & Drop File Sharing

The Rust P2P Chat now includes a Windows executable with full drag and drop support for easy file sharing.

## Features

- **Drag & Drop**: Simply drag files from Windows Explorer onto the chat window
- **Multi-file Support**: Drop multiple files at once
- **All File Types**: Images, music, videos, documents - any file type is supported
- **Visual Feedback**: See a clear overlay when dragging files over the window
- **File Browser**: Click the 📎 button to browse and select files
- **Large Files**: Support for files up to 100MB
- **Encryption**: All file transfers are end-to-end encrypted

## Building the Windows Executable

### Prerequisites
- Rust toolchain (install from https://rustup.rs/)
- Windows 10 or later

### Build Instructions

1. **Using PowerShell:**
   ```powershell
   .\build-windows.ps1
   ```

2. **Using Command Prompt:**
   ```cmd
   build-windows.bat
   ```

The executable will be created in `windows-release\rust-p2p-chat.exe`

## Running the Application

### GUI Mode (Recommended for drag & drop):
```cmd
rust-p2p-chat.exe --gui
```

### Command Line Mode:
```cmd
rust-p2p-chat.exe --port 8080
```

## How to Use Drag & Drop

1. **Start the application** in GUI mode
2. **Connect to a peer** or wait for incoming connections
3. **Drag files** from Windows Explorer directly onto the chat window
4. You'll see **"Drop files here to send"** overlay when hovering
5. **Release the files** to send them to your peer
6. Files are automatically sent with encryption

## Supported File Types

The application supports all file types, but these are optimized for sharing:
- **Images**: JPG, PNG, GIF, BMP, WebP
- **Music**: MP3, WAV, FLAC, OGG, M4A
- **Videos**: MP4, AVI, MKV, MOV, WebM
- **Documents**: PDF, DOC, DOCX, TXT, RTF
- **Archives**: ZIP, RAR, 7Z, TAR

## Tips

1. **Multiple Files**: You can drag and drop multiple files at once
2. **Folders**: Currently, individual files only (folder support coming soon)
3. **Large Files**: Files over 100MB will be rejected with a friendly message
4. **Auto-open**: Enable auto-open in settings to automatically open received media files

## Troubleshooting

### Windows Defender / Antivirus
The first time you run the executable, Windows Defender might scan it. This is normal for new executables. If prompted, click "More info" and then "Run anyway".

### Drag & Drop Not Working
1. Make sure you're running in GUI mode (`--gui` flag)
2. Ensure the chat window is the active/focused window
3. Try running as Administrator if issues persist

### Build Errors
If you encounter build errors:
1. Update Rust: `rustup update`
2. Clean build: `cargo clean`
3. Ensure all dependencies are installed

## Security Note

All file transfers use the same end-to-end encryption as text messages:
- 1024-bit RSA key exchange
- AES-256-GCM encryption for file data
- Integrity verification for all transfers