#!/bin/bash
# Script to create release binaries for all platforms

VERSION="1.0.0"
RELEASE_DIR="release-$VERSION"

echo "Creating release binaries for Rust P2P Chat v$VERSION"

# Create release directory
rm -rf $RELEASE_DIR
mkdir -p $RELEASE_DIR

# Build for Linux
echo "Building for Linux..."
cargo build --release
cp target/release/rust-p2p-chat $RELEASE_DIR/rust-p2p-chat-linux-x64
chmod +x $RELEASE_DIR/rust-p2p-chat-linux-x64

# Build for Windows (if in WSL with MinGW installed)
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Building for Windows..."
    rustup target add x86_64-pc-windows-gnu 2>/dev/null
    cargo build --release --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/release/rust-p2p-chat.exe $RELEASE_DIR/rust-p2p-chat-windows-x64.exe
else
    echo "Skipping Windows build (MinGW not installed)"
fi

# Create Windows package with scripts
if [ -f $RELEASE_DIR/rust-p2p-chat-windows-x64.exe ]; then
    echo "Creating Windows package..."
    WINDOWS_PKG="rust-p2p-chat-windows-$VERSION"
    mkdir -p $WINDOWS_PKG
    cp $RELEASE_DIR/rust-p2p-chat-windows-x64.exe $WINDOWS_PKG/rust-p2p-chat.exe
    cp WINDOWS_DRAGDROP.md $WINDOWS_PKG/README-DRAGDROP.txt
    cp Readme.md $WINDOWS_PKG/README.txt
    
    # Create launcher batch file
    cat > $WINDOWS_PKG/START-CHAT.bat << 'EOF'
@echo off
echo Starting Rust P2P Chat with GUI and drag & drop support...
start rust-p2p-chat.exe --gui
EOF
    
    # Create desktop shortcut creator
    cat > $WINDOWS_PKG/CREATE-DESKTOP-SHORTCUT.bat << 'EOF'
@echo off
echo Creating desktop shortcut...
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Rust P2P Chat.lnk'); $Shortcut.TargetPath = '%~dp0rust-p2p-chat.exe'; $Shortcut.Arguments = '--gui'; $Shortcut.IconLocation = '%~dp0rust-p2p-chat.exe'; $Shortcut.Save()"
echo Desktop shortcut created!
pause
EOF
    
    # Zip the Windows package
    cd $WINDOWS_PKG
    zip -r ../rust-p2p-chat-windows-$VERSION.zip *
    cd ..
    rm -rf $WINDOWS_PKG
    mv rust-p2p-chat-windows-$VERSION.zip $RELEASE_DIR/
fi

# Create source archive
echo "Creating source archive..."
git archive --format=tar.gz --prefix=rust-p2p-chat-$VERSION/ -o $RELEASE_DIR/rust-p2p-chat-$VERSION-source.tar.gz HEAD

# Create checksums
cd $RELEASE_DIR
echo "Creating checksums..."
sha256sum * > SHA256SUMS.txt

echo ""
echo "Release artifacts created in $RELEASE_DIR:"
ls -la $RELEASE_DIR/
echo ""
echo "To create a GitHub release:"
echo "1. Go to https://github.com/your-username/RustChat/releases/new"
echo "2. Create a new release with tag v$VERSION"
echo "3. Upload all files from $RELEASE_DIR/"
echo "4. Include release notes about drag & drop support"