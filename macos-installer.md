# macOS Installer for Rust P2P Chat

## Overview
This directory contains the necessary files to build a macOS installer for the Rust P2P Chat application.

## Files
- `build-macos.sh`: Main build script that creates the macOS app bundle and DMG installer
- `.cargo/config.toml`: Cargo configuration for cross-compilation to macOS targets

## Prerequisites
To build the macOS installer from a non-macOS system, you'll need:

1. **Rust with macOS targets:**
```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

2. **macOS cross-compilation toolchain:**
- Install `osxcross` for cross-compilation
- Or use a macOS system for native compilation

## Building the Installer

### On macOS (Native)
```bash
./build-macos.sh
```

### Cross-compilation (Linux/Windows)
You'll need to set up the macOS cross-compilation toolchain first:

1. Install osxcross following: https://github.com/tpoechtrager/osxcross
2. Set up the linkers in `.cargo/config.toml`
3. Run the build script

## Output
The build script creates:
- `RustP2PChat.app`: macOS application bundle
- `RustP2PChat-0.1.0.dmg`: Disk image installer

## Installation
Users can install by:
1. Downloading the DMG file
2. Opening the DMG
3. Dragging the app to the Applications folder

## App Bundle Structure
```
RustP2PChat.app/
├── Contents/
│  ├── Info.plist     # App metadata
│  ├── MacOS/
│  │  └── RustP2PChat   # Main executable
│  └── Resources/     # App resources (icons, etc.)
```

## Code Signing (Optional)
For distribution outside the App Store, you may want to sign the app:

```bash
# Sign the app bundle
codesign --force --deep --sign "Developer ID Application: Your Name" RustP2PChat.app

# Sign the DMG
codesign --force --sign "Developer ID Application: Your Name" RustP2PChat-0.1.0.dmg
```

## Notarization (Optional)
For Gatekeeper compatibility:
1. Upload to Apple for notarization
2. Staple the notarization to the DMG

## Troubleshooting
- Ensure Rust targets are installed: `rustup target list --installed`
- Check that the cross-compilation toolchain is properly configured
- Verify that `lipo` and `hdiutil` are available (macOS only)