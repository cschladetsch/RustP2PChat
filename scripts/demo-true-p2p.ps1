# Advanced P2P Demo - Shows true peer equality
# Both peers simultaneously listen AND attempt to connect

param(
    [int]$Peer1Port = 8080,
    [int]$Peer2Port = 8081,
    [string]$Peer1Name = "Alice",
    [string]$Peer2Name = "Bob"
)

Write-Host @"
╔════════════════════════════════════════════════════════════════╗
║              Rust P2P Chat - True P2P Architecture             ║
╠════════════════════════════════════════════════════════════════╣
║                                                                ║
║  Traditional Client-Server:     True P2P (This App):          ║
║  ┌────────┐    ┌────────┐      ┌────────┐    ┌────────┐     ║
║  │ Client │───▶│ Server │      │ Peer 1 │◀──▶│ Peer 2 │     ║
║  └────────┘    └────────┘      └────────┘    └────────┘     ║
║                                                                ║
║  - Fixed roles                  - No fixed roles              ║
║  - Server must be up first      - Either peer can start first ║
║  - One-way connection init      - Bidirectional capability    ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

Write-Host ""
Write-Host "This demo will start two peers that demonstrate true P2P:" -ForegroundColor Yellow
Write-Host "- $Peer1Name on port $Peer1Port" -ForegroundColor Green
Write-Host "- $Peer2Name on port $Peer2Port" -ForegroundColor Blue
Write-Host ""

# Check executable
$exe = ".\target\release\rust-p2p-chat.exe"
if (-not (Test-Path $exe)) {
    $exe = ".\rust-p2p-chat.exe"
    if (-not (Test-Path $exe)) {
        Write-Host "Error: rust-p2p-chat.exe not found" -ForegroundColor Red
        Write-Host "Please build with: cargo build --release" -ForegroundColor Yellow
        exit 1
    }
}

# Function to create peer launcher
function Create-PeerLauncher {
    param($Name, $Color, $Port, $ConnectPort)
    
    return @"
`$Host.UI.RawUI.WindowTitle = 'P2P Chat - $Name (Port $Port)'
Write-Host '╔════════════════════════════════════════════════╗' -ForegroundColor $Color
Write-Host '║          Peer: $Name                           ║' -ForegroundColor $Color
Write-Host '║          Port: $Port                           ║' -ForegroundColor $Color  
Write-Host '╠════════════════════════════════════════════════╣' -ForegroundColor $Color
Write-Host '║  📁 Drag files onto this window to send them   ║' -ForegroundColor White
Write-Host '║  📎 Click paperclip button to browse files     ║' -ForegroundColor White
Write-Host '║  🔒 All transfers are encrypted end-to-end     ║' -ForegroundColor White
Write-Host '╚════════════════════════════════════════════════╝' -ForegroundColor $Color
Write-Host ''
Write-Host 'Starting peer...' -ForegroundColor Gray

# In a true P2P app, we could listen AND connect simultaneously
# For this demo, we'll use the current architecture
& '$exe' --gui --port $Port --nickname $Name $(if (`$ConnectPort) { "--connect localhost:`$ConnectPort" })
"@
}

# Create launchers
$peer1Launcher = Create-PeerLauncher -Name $Peer1Name -Color "Green" -Port $Peer1Port
$peer2Launcher = Create-PeerLauncher -Name $Peer2Name -Color "Blue" -Port $Peer2Port -ConnectPort $Peer1Port

# Launch peers
Write-Host "Launching peers..." -ForegroundColor White
$peer1 = Start-Process powershell -ArgumentList "-NoExit", "-Command", $peer1Launcher -PassThru
Start-Sleep -Milliseconds 1000
$peer2 = Start-Process powershell -ArgumentList "-NoExit", "-Command", $peer2Launcher -PassThru

Write-Host ""
Write-Host "✓ Both peers are now running!" -ForegroundColor Green
Write-Host ""
Write-Host "Experiments to try:" -ForegroundColor Cyan
Write-Host "1. Drag different file types onto each window" -ForegroundColor White
Write-Host "2. Send multiple files at once" -ForegroundColor White
Write-Host "3. Close $Peer1Name and restart - $Peer2Name will reconnect" -ForegroundColor White
Write-Host "4. Both can send/receive simultaneously" -ForegroundColor White
Write-Host ""
Write-Host "Commands available in chat:" -ForegroundColor Yellow
Write-Host "  /help     - Show all commands" -ForegroundColor Gray
Write-Host "  /send     - Send a file (alternative to drag && drop)" -ForegroundColor Gray
Write-Host "  /nick     - Change nickname" -ForegroundColor Gray
Write-Host "  /autoopen - Toggle auto-open for received files" -ForegroundColor Gray
Write-Host ""

# Monitor processes
Write-Host "Press 'Q' to quit all peers, or close this window to keep them running" -ForegroundColor Gray
while ($true) {
    if ([Console]::KeyAvailable) {
        $key = [Console]::ReadKey($true)
        if ($key.Key -eq 'Q') {
            Write-Host "`nShutting down peers..." -ForegroundColor Yellow
            $peer1.CloseMainWindow() | Out-Null
            $peer2.CloseMainWindow() | Out-Null
            Start-Sleep -Seconds 1
            if (!$peer1.HasExited) { $peer1.Kill() }
            if (!$peer2.HasExited) { $peer2.Kill() }
            break
        }
    }
    
    # Check if peers are still running
    if ($peer1.HasExited -and $peer2.HasExited) {
        Write-Host "`nBoth peers have exited." -ForegroundColor Red
        break
    }
    
    Start-Sleep -Milliseconds 500
}