@echo off
REM Batch script to demonstrate true P2P connectivity
REM Shows that both peers are equal - no fixed server/client roles

echo Rust P2P Chat - True Peer-to-Peer Demo
echo ======================================
echo.
echo This demonstrates that there's no fixed 'server' or 'client'
echo Both peers are equal and can initiate or accept connections
echo.

REM Check if executable exists
if not exist "target\release\rust-p2p-chat.exe" (
    echo Error: rust-p2p-chat.exe not found
    echo Please build the project first with: cargo build --release
    pause
    exit /b 1
)

echo Starting two peer instances...
echo.
echo Peer 1 (Alice) - Green window - Port 8080
echo Peer 2 (Bob)   - Blue window  - Port 8081
echo.
echo Both peers can:
echo - Send files by dragging onto their window
echo - Receive files from the other peer
echo - Act as both "server" and "client"
echo.

REM Start Peer 1 (Alice)
start "Peer 1 - Alice" cmd /k "color 2F && echo PEER 1 (Alice) - Drag files here to send && echo. && target\release\rust-p2p-chat.exe --gui --port 8080 --nickname Alice"

REM Wait a moment
timeout /t 1 /nobreak >nul

REM Start Peer 2 (Bob)
start "Peer 2 - Bob" cmd /k "color 1F && echo PEER 2 (Bob) - Drag files here to send && echo. && target\release\rust-p2p-chat.exe --gui --connect localhost:8080 --nickname Bob"

echo.
echo Both peers are now running!
echo.
echo Try dragging different files onto each window.
echo Close this window to keep peers running, or press any key to close all.
pause