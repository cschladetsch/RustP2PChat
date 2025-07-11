# Windows 11 Auto-Connect Demo Script
# Builds the project and launches two GUI windows that automatically connect

param(
    [string]$AlicePort = "8080",
    [string]$BobPort = "8081"
)

Write-Host "Rust P2P Chat - Windows 11 Auto-Connect Demo" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# Function to find the executable
function Find-Executable {
    $paths = @(
        ".\target\release\rust-p2p-chat.exe",
        ".\rust-p2p-chat.exe",
        ".\windows-release\rust-p2p-chat.exe"
    )
    
    foreach ($path in $paths) {
        if (Test-Path $path) {
            return (Resolve-Path $path).Path
        }
    }
    return $null
}

# Check if we need to build
$exe = Find-Executable
if (-not $exe) {
    Write-Host "Executable not found. Building project..." -ForegroundColor Yellow
    
    # Check if cargo is available
    try {
        $null = Get-Command cargo -ErrorAction Stop
    } catch {
        Write-Host "Error: Rust/Cargo not installed" -ForegroundColor Red
        Write-Host "Please install from: https://rustup.rs/" -ForegroundColor Yellow
        exit 1
    }
    
    # Build the project
    Write-Host "Running: cargo build --release" -ForegroundColor Gray
    cargo build --release
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Build failed!" -ForegroundColor Red
        exit 1
    }
    
    $exe = Find-Executable
    if (-not $exe) {
        Write-Host "Build succeeded but executable not found!" -ForegroundColor Red
        exit 1
    }
}

Write-Host "Using executable: $exe" -ForegroundColor Green
Write-Host ""

# Kill any existing instances
Write-Host "Cleaning up any existing instances..." -ForegroundColor Gray
Get-Process | Where-Object { $_.ProcessName -eq "rust-p2p-chat" } | Stop-Process -Force -ErrorAction SilentlyContinue

# Create window positions for side-by-side display
$screenWidth = (Get-WmiObject -Class Win32_VideoController).CurrentHorizontalResolution
if (-not $screenWidth) { $screenWidth = 1920 } # Default if can't detect

$windowWidth = [Math]::Floor($screenWidth / 2) - 20
$aliceX = 10
$bobX = $windowWidth + 30

# Launch Alice (left side)
Write-Host "Starting Alice on port $AlicePort (left window)..." -ForegroundColor Green
$aliceArgs = @(
    "--gui",
    "--port", $AlicePort,
    "--nickname", "Alice"
)

$aliceProcess = Start-Process -FilePath $exe -ArgumentList $aliceArgs -PassThru

# Give Alice time to start and bind to port
Write-Host "Waiting for Alice to initialize..." -ForegroundColor Gray
Start-Sleep -Seconds 2

# Launch Bob (right side) - connects to Alice
Write-Host "Starting Bob on port $BobPort (right window)..." -ForegroundColor Blue
$bobArgs = @(
    "--gui",
    "--port", $BobPort,
    "--connect", "localhost:$AlicePort",
    "--nickname", "Bob"
)

$bobProcess = Start-Process -FilePath $exe -ArgumentList $bobArgs -PassThru

# Wait for windows to appear
Start-Sleep -Seconds 2

# Try to position windows side by side (requires Windows API)
Add-Type @"
    using System;
    using System.Runtime.InteropServices;
    public class Win32 {
        [DllImport("user32.dll")]
        public static extern bool SetWindowPos(IntPtr hWnd, IntPtr hWndInsertAfter, 
            int X, int Y, int cx, int cy, uint uFlags);
        [DllImport("user32.dll")]
        public static extern bool ShowWindow(IntPtr hWnd, int nCmdShow);
    }
"@

try {
    # Position Alice window
    if ($aliceProcess.MainWindowHandle -ne [System.IntPtr]::Zero) {
        [Win32]::SetWindowPos($aliceProcess.MainWindowHandle, [System.IntPtr]::Zero, 
            $aliceX, 50, 800, 600, 0x0040) | Out-Null
    }
    
    # Position Bob window
    if ($bobProcess.MainWindowHandle -ne [System.IntPtr]::Zero) {
        [Win32]::SetWindowPos($bobProcess.MainWindowHandle, [System.IntPtr]::Zero, 
            $bobX, 50, 800, 600, 0x0040) | Out-Null
    }
} catch {
    Write-Host "Note: Could not automatically position windows" -ForegroundColor Gray
}

Write-Host ""
Write-Host "✓ Success! Two P2P chat windows are now running:" -ForegroundColor Green
Write-Host ""
Write-Host "  Alice (Green):" -ForegroundColor Green
Write-Host "    - Listening on port $AlicePort" -ForegroundColor Gray
Write-Host "    - Ready to accept connections" -ForegroundColor Gray
Write-Host ""
Write-Host "  Bob (Blue):" -ForegroundColor Blue
Write-Host "    - Listening on port $BobPort" -ForegroundColor Gray
Write-Host "    - Connected to Alice on port $AlicePort" -ForegroundColor Gray
Write-Host ""
Write-Host "Features to try:" -ForegroundColor Yellow
Write-Host "  • Drag and drop files onto either window" -ForegroundColor White
Write-Host "  • Type messages and press Enter to send" -ForegroundColor White
Write-Host "  • Use /help to see available commands" -ForegroundColor White
Write-Host "  • Close either window and restart - they'll reconnect!" -ForegroundColor White
Write-Host ""
Write-Host "Press Ctrl+C to close both windows, or close them individually" -ForegroundColor Gray

# Monitor processes
while ($true) {
    if ($aliceProcess.HasExited -and $bobProcess.HasExited) {
        Write-Host "`nBoth peers have exited." -ForegroundColor Yellow
        break
    }
    Start-Sleep -Milliseconds 500
    if ([Console]::KeyAvailable) {
        $key = [Console]::ReadKey($true)
        if ($key.Key -eq 'Q' -or $key.Key -eq 'Escape') {
            Write-Host "`nShutting down peers..." -ForegroundColor Yellow
            $aliceProcess.CloseMainWindow() | Out-Null
            $bobProcess.CloseMainWindow() | Out-Null
            Start-Sleep -Seconds 1
            if (!$aliceProcess.HasExited) { $aliceProcess.Kill() }
            if (!$bobProcess.HasExited) { $bobProcess.Kill() }
            break
        }
    }
}