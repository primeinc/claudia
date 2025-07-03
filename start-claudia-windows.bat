@echo off
SETLOCAL

echo =================================================================
echo ==                                                             ==
echo ==              CLAUDIA FOR WINDOWS - LOCAL LAUNCHER          ==
echo ==                                                             ==
echo =================================================================
echo.

REM Change to the directory where this script is located
cd /d "%~dp0"
echo Working Directory: %CD%
echo.

REM --- Check and Setup WSL Bridge if needed ---
SET "CLAUDIA_BIN_PATH=%USERPROFILE%\.claudia\bin"
IF NOT EXIST "%CLAUDIA_BIN_PATH%" (
    echo [INFO] Creating directory: %CLAUDIA_BIN_PATH%
    mkdir "%CLAUDIA_BIN_PATH%"
)

REM Check if claude.bat bridge exists
IF NOT EXIST "%CLAUDIA_BIN_PATH%\claude.bat" (
    echo [INFO] WSL bridge not found. Creating claude.bat bridge script...
    
    (
        echo @echo off
        echo REM This is a bridge script for Claudia on Windows.
        echo REM It calls the 'claude' CLI within WSL and handles argument filtering.
        echo:
        echo SETLOCAL
        echo:
        echo REM --- Configuration ---
        echo REM Enter the name of your WSL distribution here ^(e.g., "Ubuntu"^).
        echo REM Leave empty to use the default distribution.
        echo SET "WSL_DISTRO="
        echo:
        echo REM --- Argument Handling ---
        echo:
        echo REM ** IMPORTANT: Version check for the patched Claudia GUI **
        echo REM Gets the real version from WSL and appends ^(WSL Bridge^) suffix
        echo if /I "%%1" == "--version" ^(
        echo     for /f "tokens=*" %%%%i in ^('wsl -- bash -lc "claude --version" 2^^^>^^^&1'^) do ^(
        echo         echo %%%%i ^(WSL Bridge^^^)
        echo         exit /b 0
        echo     ^)
        echo ^)
        echo:
        echo REM Build filtered arguments
        echo set "args="
        echo for %%%%i in ^(%%*^) do ^(
        echo     echo %%%%i ^| findstr /B "--no-color" ^>nul
        echo     if errorlevel 1 ^(
        echo         if defined args ^(
        echo             set "args=%%args%% %%%%i"
        echo         ^) else ^(
        echo             set "args=%%%%i"
        echo         ^)
        echo     ^)
        echo ^)
        echo:
        echo REM --- WSL Call ---
        echo if defined WSL_DISTRO ^(
        echo     wsl -d "%%WSL_DISTRO%%" -- bash -lc "claude %%args%%"
        echo ^) else ^(
        echo     wsl -- bash -lc "claude %%args%%"
        echo ^)
    ) > "%CLAUDIA_BIN_PATH%\claude.bat"
    
    echo [OK] WSL bridge created successfully.
    echo.
) ELSE (
    echo [OK] WSL bridge already exists.
)

REM Check if Claude is installed in WSL
echo [INFO] Checking Claude CLI in WSL...
wsl -- bash -lc "claude --version" >nul 2>&1
if %ERRORLEVEL% == 0 (
    for /f "delims=" %%i in ('wsl -- bash -lc "claude --version" 2^>^&1') do echo Claude version: %%i
) else (
    echo [WARNING] Claude CLI not found in WSL!
    echo Please install it using: npm install -g @anthropic-ai/claude-cli
    echo.
    pause
)

REM Ensure .claude directory exists in WSL
wsl mkdir -p ~/.claude 2>nul

REM Add bun and claudia bin to PATH
SET "PATH=%USERPROFILE%\.bun\bin;%CLAUDIA_BIN_PATH%;%PATH%"

REM Disable Rust incremental compilation for Windows
SET CARGO_INCREMENTAL=0

REM Kill any process using port 5173
echo [INFO] Clearing port 5173...
for /f "tokens=5" %%a in ('netstat -aon ^| findstr :5173 2^>nul') do (
    taskkill /F /PID %%a >nul 2>&1
)
wsl -- pkill -f "vite.*5173" 2>nul


REM Install dependencies
echo [INFO] Installing dependencies...
bun install

echo.
echo Frontend will be available at http://localhost:5173/
echo Press Ctrl+C to exit.
echo.

REM Run Tauri dev
bun run tauri dev

pause