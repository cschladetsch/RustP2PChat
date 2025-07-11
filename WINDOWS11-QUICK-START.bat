@echo off
REM One-click Windows 11 Demo
REM Builds (if needed) and launches two auto-connecting chat windows

cd /d "%~dp0"

echo Rust P2P Chat - Windows 11 Quick Start
echo =====================================
echo.

REM Try to run the PowerShell version for better experience
powershell -ExecutionPolicy Bypass -Command "& { if (Test-Path '.\scripts\windows-demo-auto.ps1') { .\scripts\windows-demo-auto.ps1 } else { Write-Host 'PowerShell script not found, using batch fallback' -ForegroundColor Yellow } }" 2>nul

if %errorlevel% neq 0 (
    echo PowerShell execution failed, using batch version...
    echo.
    if exist "scripts\windows-demo-auto.bat" (
        call scripts\windows-demo-auto.bat
    ) else (
        echo Error: Demo scripts not found!
        echo Please ensure you're in the rust-p2p-chat directory.
        pause
    )
)