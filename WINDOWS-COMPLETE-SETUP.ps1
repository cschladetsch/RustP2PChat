# Complete Windows Setup Script
# This script does EVERYTHING - installs Rust, copies project, builds, and runs demo

Write-Host @"

╔═══════════════════════════════════════════════════════════╗
║        Rust P2P Chat - Complete Windows Setup             ║
╚═══════════════════════════════════════════════════════════╝

"@ -ForegroundColor Cyan

# Function to test if running as admin
function Test-Admin {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Function to refresh PATH
function Refresh-Path {
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

# Step 1: Check and install Rust if needed
Write-Host "Step 1: Checking Rust installation..." -ForegroundColor Yellow
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "✓ Rust is already installed: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "Rust not found. Installing Rust..." -ForegroundColor Yellow
    
    # Check if winget is available
    try {
        winget --version | Out-Null
        Write-Host "Installing via winget..." -ForegroundColor Gray
        winget install Rustlang.Rustup --silent --accept-package-agreements --accept-source-agreements
        
        # Refresh PATH
        Refresh-Path
        
        # Run rustup-init if needed
        $rustupPath = "$env:USERPROFILE\.cargo\bin\rustup.exe"
        if (Test-Path $rustupPath) {
            & $rustupPath default stable
        }
    } catch {
        Write-Host "Winget not available. Downloading Rust installer..." -ForegroundColor Gray
        $rustupUrl = "https://win.rustup.rs/x86_64"
        $rustupExe = "$env:TEMP\rustup-init.exe"
        
        # Download rustup
        [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupExe -UseBasicParsing
        
        # Run installer
        Write-Host "Running Rust installer (please follow the prompts)..." -ForegroundColor Yellow
        Start-Process -FilePath $rustupExe -ArgumentList "-y", "--default-toolchain", "stable" -Wait
        
        # Clean up
        Remove-Item $rustupExe -Force
    }
    
    # Refresh PATH again
    Refresh-Path
    
    # Verify installation
    try {
        $rustVersion = & "$env:USERPROFILE\.cargo\bin\rustc.exe" --version
        Write-Host "✓ Rust installed successfully: $rustVersion" -ForegroundColor Green
    } catch {
        Write-Host "✗ Failed to install Rust. Please install manually from https://rustup.rs/" -ForegroundColor Red
        Write-Host "After installing, run this script again." -ForegroundColor Yellow
        pause
        exit 1
    }
}

# Step 2: Set up project directory
Write-Host "`nStep 2: Setting up project directory..." -ForegroundColor Yellow

$projectSource = "\\wsl.localhost\Ubuntu\home\xian\local\RustChat\rust-p2p-chat"
$projectDest = "C:\RustProjects\rust-p2p-chat"

# Check if source exists
if (-not (Test-Path $projectSource)) {
    Write-Host "✗ WSL source not found at: $projectSource" -ForegroundColor Red
    Write-Host "Please ensure WSL is running and the path is correct." -ForegroundColor Yellow
    pause
    exit 1
}

# Create destination directory
if (-not (Test-Path "C:\RustProjects")) {
    New-Item -ItemType Directory -Path "C:\RustProjects" -Force | Out-Null
}

# Copy or update project
if (Test-Path $projectDest) {
    Write-Host "Project already exists at $projectDest" -ForegroundColor Gray
    $response = Read-Host "Update existing project? (y/n)"
    if ($response -eq 'y') {
        Write-Host "Updating project files..." -ForegroundColor Gray
        # Copy only source files to preserve Windows build artifacts
        robocopy "$projectSource\src" "$projectDest\src" /E /XO /NJH /NJS /NC /NS /NP
        robocopy "$projectSource" "$projectDest" Cargo.toml Cargo.lock /XO /NJH /NJS /NC /NS /NP
        Copy-Item "$projectSource\scripts" "$projectDest\" -Recurse -Force
    }
} else {
    Write-Host "Copying project from WSL to Windows..." -ForegroundColor Gray
    Write-Host "This may take a minute..." -ForegroundColor Gray
    
    # Use robocopy for better performance
    robocopy $projectSource $projectDest /E /XD target .git /XF .gitignore /NJH /NJS
    
    if ($LASTEXITCODE -ge 8) {
        Write-Host "✗ Failed to copy project files" -ForegroundColor Red
        pause
        exit 1
    }
}

Write-Host "✓ Project ready at: $projectDest" -ForegroundColor Green

# Step 3: Build the project
Write-Host "`nStep 3: Building the project..." -ForegroundColor Yellow
Set-Location $projectDest

# Ensure we're using the correct Rust
$env:Path = "$env:USERPROFILE\.cargo\bin;" + $env:Path

# Clean any previous builds
if (Test-Path "target") {
    Write-Host "Cleaning previous build artifacts..." -ForegroundColor Gray
    cargo clean
}

Write-Host "Building release version (this may take a few minutes)..." -ForegroundColor Gray
Write-Host "Building with features: GUI, Drag && Drop, Windows optimizations" -ForegroundColor Gray

# Set optimization flags
$env:RUSTFLAGS = "-C target-cpu=native"

# Build
$buildStart = Get-Date
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ Build failed!" -ForegroundColor Red
    Write-Host "Please check the error messages above." -ForegroundColor Yellow
    pause
    exit 1
}

$buildTime = (Get-Date) - $buildStart
Write-Host "✓ Build completed in $([int]$buildTime.TotalSeconds) seconds" -ForegroundColor Green

# Step 4: Launch the demo
Write-Host "`nStep 4: Launching P2P Chat Demo..." -ForegroundColor Yellow

$exe = ".\target\release\rust-p2p-chat.exe"
if (-not (Test-Path $exe)) {
    Write-Host "✗ Executable not found!" -ForegroundColor Red
    pause
    exit 1
}

# Get file size
$exeSize = (Get-Item $exe).Length / 1MB
Write-Host "Executable size: $([Math]::Round($exeSize, 2)) MB" -ForegroundColor Gray

# Kill any existing instances
Get-Process | Where-Object { $_.ProcessName -eq "rust-p2p-chat" } | Stop-Process -Force -ErrorAction SilentlyContinue

Write-Host "`nStarting two P2P chat windows..." -ForegroundColor Green
Write-Host "  • Alice (Green) - Port 8080" -ForegroundColor Green
Write-Host "  • Bob (Blue) - Port 8081" -ForegroundColor Blue

# Launch Alice
$alice = Start-Process $exe -ArgumentList "--gui", "--port", "8080", "--nickname", "Alice" -PassThru
Write-Host "✓ Alice started (PID: $($alice.Id))" -ForegroundColor Green

# Wait for Alice to initialize
Start-Sleep -Seconds 2

# Launch Bob
$bob = Start-Process $exe -ArgumentList "--gui", "--port", "8081", "--connect", "localhost:8080", "--nickname", "Bob" -PassThru
Write-Host "✓ Bob started (PID: $($bob.Id))" -ForegroundColor Blue

Write-Host @"

╔═══════════════════════════════════════════════════════════╗
║                    SUCCESS! 🎉                            ║
╠═══════════════════════════════════════════════════════════╣
║  Two P2P chat windows are now running and connected!      ║
║                                                           ║
║  Try these features:                                      ║
║  • Drag & drop files onto either window                   ║
║  • Type messages and press Enter                          ║
║  • Use /help to see commands                              ║
║  • Close and restart either window - they reconnect!      ║
╚═══════════════════════════════════════════════════════════╝

"@ -ForegroundColor Cyan

Write-Host "Project location: $projectDest" -ForegroundColor Gray
Write-Host "To run again: .\target\release\rust-p2p-chat.exe --gui" -ForegroundColor Gray
Write-Host ""
Write-Host "Press any key to close both windows and exit..." -ForegroundColor Yellow

$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

# Cleanup
if (-not $alice.HasExited) { $alice.CloseMainWindow() | Out-Null }
if (-not $bob.HasExited) { $bob.CloseMainWindow() | Out-Null }
Start-Sleep -Seconds 1
if (-not $alice.HasExited) { $alice.Kill() }
if (-not $bob.HasExited) { $bob.Kill() }

Write-Host "`nDemo closed. Thanks for trying Rust P2P Chat!" -ForegroundColor Green