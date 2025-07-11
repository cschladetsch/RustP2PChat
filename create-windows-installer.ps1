# PowerShell script to create a simple Windows installer/package
# This creates a portable ZIP file with the executable and documentation

param(
    [string]$Version = "1.0.0"
)

Write-Host "Creating Windows installer package for Rust P2P Chat v$Version..." -ForegroundColor Green

# Build the release version first
Write-Host "Building release executable..." -ForegroundColor Yellow
& .\build-windows.ps1

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed! Aborting installer creation." -ForegroundColor Red
    exit 1
}

# Create package directory
$packageDir = ".\rust-p2p-chat-windows-$Version"
if (Test-Path $packageDir) {
    Remove-Item $packageDir -Recurse -Force
}
New-Item -ItemType Directory -Path $packageDir | Out-Null

# Copy files
Write-Host "Copying files..." -ForegroundColor Yellow
Copy-Item ".\windows-release\rust-p2p-chat.exe" -Destination $packageDir
Copy-Item ".\WINDOWS_DRAGDROP.md" -Destination "$packageDir\README-DRAGDROP.txt"
Copy-Item ".\Readme.md" -Destination "$packageDir\README.txt"

# Create a simple batch launcher
$launcherContent = @"
@echo off
echo Starting Rust P2P Chat with GUI and drag & drop support...
start rust-p2p-chat.exe --gui
"@
Set-Content -Path "$packageDir\START-CHAT.bat" -Value $launcherContent

# Create desktop shortcut creator
$shortcutCreator = @"
@echo off
echo Creating desktop shortcut...
powershell -Command "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Rust P2P Chat.lnk'); $Shortcut.TargetPath = '%~dp0rust-p2p-chat.exe'; $Shortcut.Arguments = '--gui'; $Shortcut.IconLocation = '%~dp0rust-p2p-chat.exe'; $Shortcut.Save()"
echo Desktop shortcut created!
pause
"@
Set-Content -Path "$packageDir\CREATE-DESKTOP-SHORTCUT.bat" -Value $shortcutCreator

# Create quick start guide
$quickStart = @"
RUST P2P CHAT - QUICK START GUIDE
=================================

1. QUICK START:
   - Double-click START-CHAT.bat to launch with GUI
   - Or run CREATE-DESKTOP-SHORTCUT.bat to add a desktop icon

2. DRAG & DROP FILES:
   - Simply drag any file onto the chat window
   - Or click the paperclip button to browse
   - Supports all file types up to 100MB

3. CONNECTING:
   - To host: Leave peer address empty and click Connect
   - To join: Enter peer's address (e.g., 192.168.1.100:8080)

4. FEATURES:
   - End-to-end encryption (AES-256)
   - Multi-file drag & drop
   - Auto-open received media files
   - No central server required

For more details, see README-DRAGDROP.txt
"@
Set-Content -Path "$packageDir\QUICK-START.txt" -Value $quickStart

# Create ZIP package
Write-Host "Creating ZIP package..." -ForegroundColor Yellow
$zipFile = ".\rust-p2p-chat-windows-$Version.zip"
if (Test-Path $zipFile) {
    Remove-Item $zipFile -Force
}

Compress-Archive -Path $packageDir\* -DestinationPath $zipFile -CompressionLevel Optimal

# Clean up
Remove-Item $packageDir -Recurse -Force

Write-Host ""
Write-Host "Package created successfully!" -ForegroundColor Green
Write-Host "File: $zipFile" -ForegroundColor Cyan
Write-Host ""
Write-Host "Distribution instructions:" -ForegroundColor Yellow
Write-Host "1. Upload $zipFile to GitHub Releases" -ForegroundColor White
Write-Host "2. Users can extract and run START-CHAT.bat" -ForegroundColor White
Write-Host "3. No installation required - fully portable!" -ForegroundColor White