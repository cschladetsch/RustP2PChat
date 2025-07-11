# Simple Windows Setup Script - No special characters
Write-Host "Rust P2P Chat - Windows Setup" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan
Write-Host ""

# Check Rust
Write-Host "Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "Rust is installed: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Rust not found. Please install from https://rustup.rs/" -ForegroundColor Red
    Write-Host "After installing, run this script again." -ForegroundColor Yellow
    pause
    exit 1
}

# Setup directories
Write-Host ""
Write-Host "Setting up project..." -ForegroundColor Yellow

$source = "\\wsl.localhost\Ubuntu\home\xian\local\RustChat\rust-p2p-chat"
$dest = "C:\RustProjects\rust-p2p-chat"

# Create directory
if (!(Test-Path "C:\RustProjects")) {
    New-Item -ItemType Directory -Path "C:\RustProjects" -Force | Out-Null
}

# Copy project
if (!(Test-Path $dest)) {
    Write-Host "Copying project files..." -ForegroundColor Gray
    xcopy $source $dest /E /I /Q /Y
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Failed to copy files" -ForegroundColor Red
        pause
        exit 1
    }
} else {
    Write-Host "Project already exists at $dest" -ForegroundColor Gray
}

# Build
Write-Host ""
Write-Host "Building project..." -ForegroundColor Yellow
Set-Location $dest

if (Test-Path "target") {
    Write-Host "Cleaning old build..." -ForegroundColor Gray
    cargo clean
}

Write-Host "Running cargo build --release..." -ForegroundColor Gray
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    pause
    exit 1
}

Write-Host "Build successful!" -ForegroundColor Green

# Launch demo
Write-Host ""
Write-Host "Launching demo..." -ForegroundColor Yellow

$exe = ".\target\release\rust-p2p-chat.exe"

# Start Alice
Write-Host "Starting Alice on port 8080..." -ForegroundColor Green
$alice = Start-Process $exe -ArgumentList "--gui --port 8080 --nickname Alice" -PassThru

# Wait
Start-Sleep -Seconds 2

# Start Bob
Write-Host "Starting Bob on port 8081..." -ForegroundColor Blue
$bob = Start-Process $exe -ArgumentList "--gui --port 8081 --connect localhost:8080 --nickname Bob" -PassThru

Write-Host ""
Write-Host "SUCCESS! Two windows are running and connected!" -ForegroundColor Green
Write-Host ""
Write-Host "Try:" -ForegroundColor Yellow
Write-Host "- Drag and drop files onto either window" -ForegroundColor White
Write-Host "- Type messages and press Enter" -ForegroundColor White
Write-Host "- Use /help for commands" -ForegroundColor White
Write-Host ""
Write-Host "Project location: $dest" -ForegroundColor Gray
Write-Host ""
Write-Host "Press any key to close both windows..." -ForegroundColor Yellow

$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Cleanup
if (!$alice.HasExited) { $alice.Kill() }
if (!$bob.HasExited) { $bob.Kill() }

Write-Host "Done!" -ForegroundColor Green