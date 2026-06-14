@echo off
REM OMNISYSTEM COMPILATION SCRIPT FOR WINDOWS
REM Builds Omnisystem executable with Titan compiler
REM Creates omnisystem.exe that launches to App Menu

setlocal enabledelayedexpansion

echo.
echo ╔════════════════════════════════════════════════════════════════╗
echo ║         OMNISYSTEM BUILD SCRIPT - WINDOWS COMPILATION          ║
echo ╚════════════════════════════════════════════════════════════════╝
echo.

set BUILD_DIR=build
set BIN_DIR=%BUILD_DIR%\bin
set COMPILER=%BIN_DIR%\titan.exe

REM Create directories
echo Creating build directories...
if not exist %BIN_DIR% mkdir %BIN_DIR%
echo   ✅ Directories created
echo.

REM Check if Titan compiler exists
if not exist %COMPILER% (
    echo ❌ ERROR: Titan compiler not found at %COMPILER%
    echo.
    echo Building Titan bootstrap compiler from C...
    call :build_bootstrap_compiler
)

REM Compile Omnisystem modules
echo.
echo ═════════════════════════════════════════════════════════════════
echo PHASE 1: COMPILING OMNISYSTEM MODULES
echo ═════════════════════════════════════════════════════════════════
echo.

echo Compiling hardware_detection.ti...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\hardware_detection.ti ^
    --output=%BIN_DIR%\hardware_detection.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling gpu_abstraction.ti...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\gpu_abstraction.ti ^
    --output=%BIN_DIR%\gpu_abstraction.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling memory management modules...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\memory\*.ti ^
    --output=%BIN_DIR%\memory.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling enterprise modules...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\enterprise\*.ti ^
    --output=%BIN_DIR%\enterprise.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling UI and App Menu...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\ui\app_menu.ti ^
    --output=%BIN_DIR%\app_menu.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling persistence modules...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\persistence\*.ti ^
    --output=%BIN_DIR%\persistence.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling cache layer...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\cache\*.ti ^
    --output=%BIN_DIR%\cache.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling message queue...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\messaging\*.ti ^
    --output=%BIN_DIR%\messaging.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling observability modules...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\observability\*.ti ^
    --output=%BIN_DIR%\observability.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling API gateway...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\api\*.ti ^
    --output=%BIN_DIR%\api.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

echo Compiling operations modules...
%COMPILER% compile ^
    --input=Omnisystem\omnisystem_modules\operations\*.ti ^
    --output=%BIN_DIR%\operations.obj ^
    --target=x86_64-w64-mingw32
echo   ✅ Complete
echo.

REM Link all modules
echo.
echo ═════════════════════════════════════════════════════════════════
echo PHASE 2: LINKING OMNISYSTEM
echo ═════════════════════════════════════════════════════════════════
echo.

echo Linking object files...
%COMPILER% link ^
    --input=%BIN_DIR%\*.obj ^
    --output=%BIN_DIR%\omnisystem.exe ^
    --target=x86_64-w64-mingw32 ^
    --subsystem=console

if exist %BIN_DIR%\omnisystem.exe (
    echo   ✅ Linking successful
) else (
    echo   ❌ Linking failed
    goto :error
)
echo.

REM Verify executable
echo ═════════════════════════════════════════════════════════════════
echo PHASE 3: VERIFICATION
echo ═════════════════════════════════════════════════════════════════
echo.

if exist %BIN_DIR%\omnisystem.exe (
    for /F "tokens=*" %%A in ('dir /B %BIN_DIR%\omnisystem.exe') do set "EXE_NAME=%%A"
    for /F "tokens=5" %%A in ('dir %BIN_DIR%\omnisystem.exe') do set "EXE_SIZE=%%A"

    echo Executable: %EXE_NAME%
    echo Size: %EXE_SIZE% bytes
    echo Status: ✅ BUILD SUCCESSFUL
) else (
    echo Status: ❌ BUILD FAILED
    goto :error
)
echo.

REM Print final summary
echo ═════════════════════════════════════════════════════════════════
echo OMNISYSTEM BUILD COMPLETE ✅
echo ═════════════════════════════════════════════════════════════════
echo.
echo Executable: %BIN_DIR%\omnisystem.exe
echo.
echo To run Omnisystem:
echo   %BIN_DIR%\omnisystem.exe
echo.
echo This will launch the Omnisystem App Menu with:
echo   • System dashboard and status
echo   • API endpoint information
echo   • Configuration options
echo   • Test runner
echo   • System logs
echo   • Settings
echo.

pause
exit /b 0

:build_bootstrap_compiler
echo.
echo Building Titan bootstrap compiler from C sources...
echo This may take a few minutes...
echo.

REM Compile C bootstrap compiler
if exist compiler\bootstrap\*.c (
    gcc -O2 ^
        compiler\bootstrap\*.c ^
        -o %COMPILER% ^
        -lm -lpthread

    if exist %COMPILER% (
        echo   ✅ Bootstrap compiler ready
    ) else (
        echo   ❌ Failed to build bootstrap compiler
        goto :error
    )
) else (
    echo   ❌ Bootstrap C source files not found
    goto :error
)
exit /b 0

:error
echo.
echo ❌ BUILD FAILED
echo.
pause
exit /b 1
