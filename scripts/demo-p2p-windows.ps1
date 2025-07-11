# PowerShell script to demonstrate true P2P connectivity
# Both peers will try to connect to each other - whoever succeeds first establishes the connection

Write-Host "Rust P2P Chat - True Peer-to-Peer Demo" -ForegroundColor Cyan
Write-Host "======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "This demonstrates that there's no fixed 'server' or 'client'" -ForegroundColor Yellow
Write-Host "Both peers are equal and can initiate or accept connections" -ForegroundColor Yellow
Write-Host ""

# Check if executable exists
$exePath = ".\target\release\rust-p2p-chat.exe"
if (-not (Test-Path $exePath)) {
    Write-Host "Error: rust-p2p-chat.exe not found at $exePath" -ForegroundColor Red
    Write-Host "Please build the project first with: cargo build --release" -ForegroundColor Yellow
    exit 1
}

# Port configuration
$port1 = 8080
$port2 = 8081

Write-Host "Starting Peer 1 (Alice) - Will listen on port $port1 and try to connect to port $port2" -ForegroundColor Green
Write-Host "Starting Peer 2 (Bob) - Will listen on port $port2 and try to connect to port $port1" -ForegroundColor Blue
Write-Host ""
Write-Host "Whichever peer starts first will be waiting, the other will connect" -ForegroundColor White
Write-Host "Once connected, both peers have identical capabilities!" -ForegroundColor White
Write-Host ""

# Create start commands
$peer1Cmd = @"
Write-Host 'PEER 1 (Alice) - Port $port1' -ForegroundColor Green
Write-Host 'Drag and drop files onto this window to send to Peer 2' -ForegroundColor Yellow
Write-Host ''
& '$exePath' --gui --port $port1 --nickname Alice
"@

$peer2Cmd = @"
Write-Host 'PEER 2 (Bob) - Port $port2' -ForegroundColor Blue  
Write-Host 'Drag and drop files onto this window to send to Peer 1' -ForegroundColor Yellow
Write-Host ''
Start-Sleep -Seconds 2  # Give Peer 1 time to start listening
& '$exePath' --gui --connect localhost:$port1 --nickname Bob
"@

# Start both peers in new windows
Start-Process powershell -ArgumentList "-NoExit", "-Command", $peer1Cmd
Start-Sleep -Milliseconds 500
Start-Process powershell -ArgumentList "-NoExit", "-Command", $peer2Cmd

Write-Host "Both peers are starting..." -ForegroundColor Green
Write-Host ""
Write-Host "Try these experiments to see true P2P in action:" -ForegroundColor Cyan
Write-Host "1. Close Peer 1 and restart it - Peer 2 will be waiting" -ForegroundColor White
Write-Host "2. Start them in reverse order - connection still works" -ForegroundColor White
Write-Host "3. Both peers can send files by dragging onto their windows" -ForegroundColor White
Write-Host "4. Both peers can receive files simultaneously" -ForegroundColor White
Write-Host ""
Write-Host "Press any key to exit this launcher (peers will continue running)..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")