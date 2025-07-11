# Windows 11 Complete Guide

## Quick Start (One Click)

Just double-click: **`WINDOWS11-QUICK-START.bat`**

This will:
1. Check if the app is built (builds if needed)
2. Launch two chat windows (Alice & Bob)
3. Automatically connect them
4. Position windows side-by-side

## Building for Windows 11

### Option 1: Automated Build (Recommended)
```powershell
.\scripts\build-win11.ps1
```

Or with packaging:
```powershell
.\scripts\build-win11.ps1 -Package
```

### Option 2: Manual Build
```powershell
# Install Rust if needed
winget install Rustlang.Rustup

# Build optimized for Windows 11
cargo build --release
```

## Running the Demo

### Automatic Demo (Two Connected Windows)

**PowerShell:**
```powershell
.\scripts\windows-demo-auto.ps1
```

**Command Prompt:**
```cmd
.\scripts\windows-demo-auto.bat
```

**Features of the demo:**
- Launches two GUI windows automatically
- Alice (left/green) on port 8080
- Bob (right/blue) on port 8081
- Auto-connects Bob to Alice
- Positions windows side-by-side
- Both windows support drag & drop

### Manual Launch

**Single instance:**
```powershell
.\target\release\rust-p2p-chat.exe --gui
```

**Two instances manually:**
```powershell
# Terminal 1
.\target\release\rust-p2p-chat.exe --gui --port 8080 --nickname Alice

# Terminal 2
.\target\release\rust-p2p-chat.exe --gui --port 8081 --connect localhost:8080 --nickname Bob
```

## Windows 11 Specific Features

1. **Native Drag & Drop**
   - Drag files from File Explorer directly onto chat window
   - Multiple file selection supported
   - Visual feedback during drag operation

2. **Window Management**
   - Automatic side-by-side positioning
   - Remembers window positions (coming soon)
   - Taskbar integration

3. **Performance Optimizations**
   - Native Windows 11 APIs
   - Hardware acceleration for UI
   - Optimized for modern CPUs

## Troubleshooting

### "Script cannot be loaded" Error
```powershell
# Run this once to allow scripts
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Build Errors
1. Ensure Rust is installed: `rustc --version`
2. Update Rust: `rustup update`
3. Clean build: `cargo clean && cargo build --release`

### Connection Issues
- Windows Firewall may block first run - click "Allow"
- Ensure ports 8080/8081 are not in use
- Try different ports: `.\scripts\windows-demo-auto.ps1 -AlicePort 9000 -BobPort 9001`

### GUI Not Showing
- The GUI requires Windows 10 version 1903 or later
- Update graphics drivers
- Try running without GPU acceleration: `set WGPU_BACKEND=dx12`

## Distribution

### Creating a Package
```powershell
.\scripts\build-win11.ps1 -Package
```

This creates a `windows11-release` folder with:
- Optimized executable
- Launcher scripts
- Demo scripts
- README
- Desktop shortcut creator

### Sharing with Others
1. Build the package (above)
2. Zip the `windows11-release` folder
3. Share the zip file
4. Recipients just extract and run `START.bat`

## Advanced Usage

### Custom Ports
```powershell
.\scripts\windows-demo-auto.ps1 -AlicePort 9090 -BobPort 9091
```

### Debug Mode
```powershell
$env:RUST_LOG = "debug"
.\target\release\rust-p2p-chat.exe --gui --debug
```

### Multiple Peers
You can run more than 2 instances! Each needs a unique port:
```powershell
# Peer 1
.\target\release\rust-p2p-chat.exe --gui --port 8080 --nickname Alice

# Peer 2 
.\target\release\rust-p2p-chat.exe --gui --port 8081 --connect localhost:8080 --nickname Bob

# Peer 3
.\target\release\rust-p2p-chat.exe --gui --port 8082 --connect localhost:8080 --nickname Charlie
```

## Tips & Tricks

1. **Drag & Drop Multiple Files**: Select multiple files in Explorer and drag them all at once

2. **Quick File Sharing**: Keep both windows open on desktop for instant file sharing

3. **Network Play**: Replace `localhost` with actual IP for network connections:
   ```
   --connect 192.168.1.100:8080
   ```

4. **Auto-start**: Add to Windows startup:
   - Win+R → `shell:startup`
   - Copy shortcut there

5. **Keyboard Shortcuts** (in chat):
   - Ctrl+C: Copy selected text
   - Ctrl+V: Paste
   - Ctrl+A: Select all
   - Esc: Clear input

## Security Notes

- All file transfers are encrypted (AES-256)
- Windows Defender may scan the exe on first run
- No data is sent to any servers
- Completely peer-to-peer