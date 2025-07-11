@echo off
REM Batch script to build Windows executable with drag and drop support
REM Run this script from the project root directory

echo Building Rust P2P Chat for Windows with drag and drop support...
echo.

REM Clean previous builds
echo Cleaning previous builds...
cargo clean

REM Build release version
echo Building release version...
cargo build --release

REM Check if build succeeded
if %ERRORLEVEL% EQU 0 (
    echo Build successful!
    echo.
    
    REM Create output directory
    if not exist "windows-release" mkdir "windows-release"
    
    REM Copy executable
    copy /Y "target\release\rust-p2p-chat.exe" "windows-release\rust-p2p-chat.exe"
    
    echo Executable copied to: windows-release\rust-p2p-chat.exe
    echo.
    echo Features:
    echo - Drag and drop files directly onto the chat window
    echo - Click the paperclip button to browse for files  
    echo - Supports images, music, documents, and any file type
    echo - Files up to 100MB supported
    echo - End-to-end encryption for all transfers
    echo.
    echo To run: windows-release\rust-p2p-chat.exe --gui
) else (
    echo Build failed!
    exit /b 1
)

pause