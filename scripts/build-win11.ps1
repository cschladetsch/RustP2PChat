# Windows 11 Build Script with Optimizations
# Builds an optimized executable for Windows 11 with all features

param(
    [switch]$Clean,
    [switch]$Package,
    [string]$OutputDir = "windows11-release"
)

Write-Host "Rust P2P Chat - Windows 11 Build Script" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""

# Check for Rust installation
try {
    $rustVersion = rustc --version
    Write-Host "Found Rust: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: Rust not installed!" -ForegroundColor Red
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    
    # Download and run rustup
    Write-Host "Downloading rustup installer..." -ForegroundColor Gray
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
    
    Write-Host "Running rustup installer..." -ForegroundColor Gray
    Start-Process -FilePath "$env:TEMP\rustup-init.exe" -ArgumentList "-y" -Wait
    
    # Refresh PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    
    # Verify installation
    try {
        $rustVersion = rustc --version
        Write-Host "Rust installed successfully: $rustVersion" -ForegroundColor Green
    } catch {
        Write-Host "Failed to install Rust. Please install manually from https://rustup.rs/" -ForegroundColor Red
        exit 1
    }
}

# Clean if requested
if ($Clean) {
    Write-Host "Cleaning previous builds..." -ForegroundColor Yellow
    cargo clean
}

# Set Windows 11 specific environment variables for optimization
Write-Host "Setting Windows 11 optimization flags..." -ForegroundColor Gray
$env:RUSTFLAGS = "-C target-cpu=native -C opt-level=3 -C lto=fat -C embed-bitcode=yes"
$env:CARGO_PROFILE_RELEASE_LTO = "true"
$env:CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1"

# Build the project
Write-Host "Building optimized release for Windows 11..." -ForegroundColor Yellow
Write-Host "This may take a few minutes for first build..." -ForegroundColor Gray

$buildStart = Get-Date
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

$buildTime = (Get-Date) - $buildStart
Write-Host "Build completed in $($buildTime.TotalSeconds) seconds" -ForegroundColor Green

# Get file info
$exePath = ".\target\release\rust-p2p-chat.exe"
$exeInfo = Get-Item $exePath
$sizeMB = [Math]::Round($exeInfo.Length / 1MB, 2)

Write-Host ""
Write-Host "Build successful!" -ForegroundColor Green
Write-Host "  Executable: $exePath" -ForegroundColor Gray
Write-Host "  Size: $sizeMB MB" -ForegroundColor Gray
Write-Host ""

# Package if requested
if ($Package) {
    Write-Host "Creating Windows 11 package..." -ForegroundColor Yellow
    
    # Create output directory
    if (Test-Path $OutputDir) {
        Remove-Item $OutputDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
    
    # Copy executable
    Copy-Item $exePath -Destination "$OutputDir\rust-p2p-chat.exe"
    
    # Create launcher script
    @"
@echo off
title Rust P2P Chat
echo Starting Rust P2P Chat with GUI...
start rust-p2p-chat.exe --gui
"@ | Out-File -FilePath "$OutputDir\START.bat" -Encoding ASCII
    
    # Create auto-demo script
    Copy-Item ".\scripts\windows-demo-auto.bat" -Destination "$OutputDir\DEMO-TWO-PEERS.bat" -ErrorAction SilentlyContinue
    
    # Create README
    @"
Rust P2P Chat for Windows 11
============================

Quick Start:
1. Double-click START.bat to launch the chat with GUI
2. Or run DEMO-TWO-PEERS.bat to see two peers in action

Features:
- Drag & drop files directly onto the window
- End-to-end encryption (AES-256)
- True peer-to-peer (no server needed)
- Supports files up to 100MB

Command Line Usage:
  rust-p2p-chat.exe --gui                           # Launch with GUI
  rust-p2p-chat.exe --port 8080                     # Listen on port 8080
  rust-p2p-chat.exe --connect 192.168.1.100:8080   # Connect to peer
  rust-p2p-chat.exe --help                          # Show all options

Optimized for Windows 11 with native performance.
"@ | Out-File -FilePath "$OutputDir\README.txt" -Encoding UTF8
    
    # Create desktop shortcut creator
    @"
`$WshShell = New-Object -comObject WScript.Shell
`$Shortcut = `$WshShell.CreateShortcut("`$env:USERPROFILE\Desktop\Rust P2P Chat.lnk")
`$Shortcut.TargetPath = "`$PSScriptRoot\rust-p2p-chat.exe"
`$Shortcut.Arguments = "--gui"
`$Shortcut.WorkingDirectory = `$PSScriptRoot
`$Shortcut.IconLocation = "`$PSScriptRoot\rust-p2p-chat.exe"
`$Shortcut.Description = "Rust P2P Chat with Drag & Drop"
`$Shortcut.Save()

Write-Host "Desktop shortcut created successfully!" -ForegroundColor Green
"@ | Out-File -FilePath "$OutputDir\CREATE-SHORTCUT.ps1" -Encoding UTF8
    
    # Calculate package size
    $packageSize = (Get-ChildItem $OutputDir -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB
    
    Write-Host ""
    Write-Host "Package created successfully!" -ForegroundColor Green
    Write-Host "  Location: $OutputDir" -ForegroundColor Gray
    Write-Host "  Total size: $([Math]::Round($packageSize, 2)) MB" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Package contents:" -ForegroundColor Yellow
    Get-ChildItem $OutputDir | ForEach-Object {
        Write-Host "  - $($_.Name)" -ForegroundColor Gray
    }
}

Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Run the demo: .\scripts\windows-demo-auto.ps1" -ForegroundColor White
Write-Host "2. Or start single instance: .\target\release\rust-p2p-chat.exe --gui" -ForegroundColor White
if ($Package) {
    Write-Host "3. Distribute the package: $OutputDir\" -ForegroundColor White
}