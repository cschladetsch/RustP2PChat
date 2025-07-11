#!/bin/bash
# Build Windows executable from WSL/Linux

echo "Building Rust P2P Chat for Windows from WSL..."

# Check if MinGW is installed
if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Error: MinGW cross-compiler not found!"
    echo ""
    echo "Please install it first:"
    echo "  sudo apt update"
    echo "  sudo apt install gcc-mingw-w64-x86-64"
    echo ""
    echo "After installation, run this script again."
    exit 1
fi

# Install Windows target if not already installed
rustup target add x86_64-pc-windows-gnu

# Clean previous builds
cargo clean

# Build for Windows
echo "Building Windows release version..."
cargo build --release --target x86_64-pc-windows-gnu

if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Create output directory
    mkdir -p windows-release
    
    # Copy executable
    cp target/x86_64-pc-windows-gnu/release/rust-p2p-chat.exe windows-release/
    
    echo ""
    echo "Windows executable created at: windows-release/rust-p2p-chat.exe"
    echo ""
    echo "Features:"
    echo "- Drag and drop files directly onto the chat window"
    echo "- Click the paperclip button to browse for files"
    echo "- Supports images, music, documents, and any file type"
    echo "- Files up to 100MB supported"
    echo "- End-to-end encryption for all transfers"
    echo ""
    echo "To run on Windows: rust-p2p-chat.exe --gui"
else
    echo "Build failed!"
    exit 1
fi