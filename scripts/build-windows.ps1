# PowerShell script to build Windows executable with drag and drop support
# Run this script from the project root directory

Write-Host "Building Rust P2P Chat for Windows with drag and drop support..." -ForegroundColor Green

# Clean previous builds
Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
cargo clean

# Build release version
Write-Host "Building release version..." -ForegroundColor Yellow
cargo build --release --target x86_64-pc-windows-gnu

# Check if build succeeded
if ($LASTEXITCODE -eq 0) {
    Write-Host "Build successful!" -ForegroundColor Green
    
    # Create output directory
    $outputDir = ".\windows-release"
    if (!(Test-Path $outputDir)) {
        New-Item -ItemType Directory -Path $outputDir | Out-Null
    }
    
    # Copy executable
    Copy-Item ".\target\x86_64-pc-windows-gnu\release\rust-p2p-chat.exe" -Destination "$outputDir\rust-p2p-chat.exe" -Force
    
    Write-Host "Executable copied to: $outputDir\rust-p2p-chat.exe" -ForegroundColor Green
    Write-Host ""
    Write-Host "Features:" -ForegroundColor Cyan
    Write-Host "- Drag and drop files directly onto the chat window" -ForegroundColor White
    Write-Host "- Click the paperclip button to browse for files" -ForegroundColor White
    Write-Host "- Supports images, music, documents, and any file type" -ForegroundColor White
    Write-Host "- Files up to 100MB supported" -ForegroundColor White
    Write-Host "- End-to-end encryption for all transfers" -ForegroundColor White
    Write-Host ""
    Write-Host "To run: .\windows-release\rust-p2p-chat.exe --gui" -ForegroundColor Yellow
} else {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}