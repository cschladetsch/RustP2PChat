# Simple P2P Demo for Windows
Write-Host "Rust P2P Chat - Simple Demo" -ForegroundColor Cyan
Write-Host ""

$exe = ".\target\release\rust-p2p-chat.exe"
if (-not (Test-Path $exe)) {
    Write-Host "Error: Build the project first with: cargo build --release" -ForegroundColor Red
    exit
}

Write-Host "Starting two peers..." -ForegroundColor Green
Write-Host "- Peer 1 (Alice) on port 8080" -ForegroundColor Green
Write-Host "- Peer 2 (Bob) on port 8081" -ForegroundColor Blue
Write-Host ""

# Start peers
Start-Process $exe -ArgumentList "--gui --port 8080 --nickname Alice"
Start-Sleep -Seconds 2
Start-Process $exe -ArgumentList "--gui --connect localhost:8080 --nickname Bob"

Write-Host "Both peers are running!" -ForegroundColor Green
Write-Host "Drag files onto either window to share them." -ForegroundColor Yellow