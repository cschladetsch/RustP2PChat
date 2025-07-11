@echo off
REM Windows 11 Auto-Connect Demo - Batch Version
REM Builds and launches two chat windows that automatically connect

echo Rust P2P Chat - Windows 11 Auto-Connect Demo
echo ============================================
echo.

REM Check if executable exists
set EXE_PATH=
if exist "target\release\rust-p2p-chat.exe" (
    set EXE_PATH=target\release\rust-p2p-chat.exe
) else if exist "rust-p2p-chat.exe" (
    set EXE_PATH=rust-p2p-chat.exe
) else if exist "windows-release\rust-p2p-chat.exe" (
    set EXE_PATH=windows-release\rust-p2p-chat.exe
)

if "%EXE_PATH%"=="" (
    echo Executable not found. Building project...
    echo.
    
    REM Check if cargo exists
    where cargo >nul 2>nul
    if errorlevel 1 (
        echo Error: Rust/Cargo not installed
        echo Please install from: https://rustup.rs/
        pause
        exit /b 1
    )
    
    echo Running: cargo build --release
    cargo build --release
    
    if errorlevel 1 (
        echo Build failed!
        pause
        exit /b 1
    )
    
    set EXE_PATH=target\release\rust-p2p-chat.exe
)

echo Using executable: %EXE_PATH%
echo.

REM Kill any existing instances
echo Cleaning up any existing instances...
taskkill /F /IM rust-p2p-chat.exe >nul 2>&1

REM Start Alice (Listener)
echo Starting Alice on port 8080 (left window)...
start "P2P Chat - Alice (Port 8080)" /MIN cmd /c "color 2F && %EXE_PATH% --gui --port 8080 --nickname Alice"

REM Wait for Alice to start
timeout /t 2 /nobreak >nul

REM Start Bob (Connector)
echo Starting Bob on port 8081 (right window)...
start "P2P Chat - Bob (Port 8081)" /MIN cmd /c "color 1F && %EXE_PATH% --gui --port 8081 --connect localhost:8080 --nickname Bob"

echo.
echo Success! Two P2P chat windows are now running:
echo.
echo   Alice (Green Window):
echo     - Listening on port 8080
echo     - Waiting for connections
echo.
echo   Bob (Blue Window):
echo     - Listening on port 8081
echo     - Connected to Alice on port 8080
echo.
echo Features to try:
echo   - Drag and drop files onto either window
echo   - Type messages and press Enter to send
echo   - Use /help to see available commands
echo   - Close either window and restart - they'll reconnect!
echo.
echo Press any key to close both windows...
pause >nul

REM Clean up
taskkill /F /IM rust-p2p-chat.exe >nul 2>&1