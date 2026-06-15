#Requires -Version 5.0
<#
.SYNOPSIS
    Omnisystem Build Script - Creates Omnisystem.exe in root directory

.DESCRIPTION
    Builds the Omnisystem Tauri desktop application and creates a standalone
    executable in the project root directory.

.EXAMPLE
    .\Build-Omnisystem.ps1

.NOTES
    Author: Omnisystem
    Date: 2026-06-15
#>

param(
    [switch]$Release = $false,
    [switch]$Clean = $false,
    [switch]$Launch = $false
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Colors
$Green = [ConsoleColor]::Green
$Yellow = [ConsoleColor]::Yellow
$Cyan = [ConsoleColor]::Cyan
$Red = [ConsoleColor]::Red

function Write-Header {
    param([string]$Message)
    Write-Host ""
    Write-Host "╔════════════════════════════════════════════════════════════╗" -ForegroundColor $Cyan
    Write-Host "║ $($Message.PadRight(58)) ║" -ForegroundColor $Cyan
    Write-Host "╚════════════════════════════════════════════════════════════╝" -ForegroundColor $Cyan
    Write-Host ""
}

function Write-Success {
    param([string]$Message)
    Write-Host "✅ $Message" -ForegroundColor $Green
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ️  $Message" -ForegroundColor $Cyan
}

function Write-Warning {
    param([string]$Message)
    Write-Host "⚠️  $Message" -ForegroundColor $Yellow
}

function Write-Error {
    param([string]$Message)
    Write-Host "❌ $Message" -ForegroundColor $Red
}

# Main script
Write-Header "OMNISYSTEM BUILD - CREATE EXECUTABLE"

$ProjectRoot = Split-Path -Parent $PSCommandPath
$GuiDir = Join-Path $ProjectRoot "omnisystem-gui"
$OutputDir = Join-Path $ProjectRoot "dist"
$ExePath = Join-Path $ProjectRoot "Omnisystem.exe"

Write-Info "Project Root: $ProjectRoot"
Write-Info "GUI Directory: $GuiDir"
Write-Info "Output Directory: $OutputDir"
Write-Info "Executable: $ExePath"

# Check if GUI directory exists
if (-not (Test-Path $GuiDir)) {
    Write-Error "GUI directory not found: $GuiDir"
    exit 1
}

# Change to GUI directory
Write-Info "Changing to GUI directory..."
Push-Location $GuiDir

try {
    # Clean if requested
    if ($Clean) {
        Write-Header "CLEANING BUILD ARTIFACTS"
        Write-Info "Removing node_modules..."
        if (Test-Path "node_modules") {
            Remove-Item -Recurse -Force "node_modules" -ErrorAction SilentlyContinue
            Write-Success "node_modules removed"
        }

        Write-Info "Removing build directories..."
        if (Test-Path "dist") {
            Remove-Item -Recurse -Force "dist" -ErrorAction SilentlyContinue
        }
        if (Test-Path "src-tauri/target") {
            Remove-Item -Recurse -Force "src-tauri/target" -ErrorAction SilentlyContinue
        }
        Write-Success "Build artifacts cleaned"
    }

    # Install dependencies
    Write-Header "INSTALLING DEPENDENCIES"
    Write-Info "Running npm install..."
    npm install 2>&1 | Out-Null
    Write-Success "Dependencies installed"

    # Build the app
    Write-Header "BUILDING OMNISYSTEM"
    Write-Info "Building Tauri application..."

    if ($Release) {
        Write-Info "Building in RELEASE mode..."
        npm run tauri:build 2>&1 | Tee-Object -FilePath "build.log"
    } else {
        Write-Info "Building in DEV mode..."
        npm run tauri:dev 2>&1 | Tee-Object -FilePath "build.log"
    }

    Write-Success "Build completed"

    # Find the executable
    Write-Header "LOCATING EXECUTABLE"

    $PossiblePaths = @(
        (Join-Path $ProjectRoot "omnisystem-gui/src-tauri/target/release/omnisystem-gui.exe"),
        (Join-Path $ProjectRoot "omnisystem-gui/src-tauri/target/debug/omnisystem-gui.exe"),
        (Join-Path $ProjectRoot "omnisystem-gui/dist/omnisystem-gui.exe")
    )

    $FoundExe = $null
    foreach ($Path in $PossiblePaths) {
        if (Test-Path $Path) {
            $FoundExe = $Path
            Write-Success "Found executable: $Path"
            break
        }
    }

    if (-not $FoundExe) {
        Write-Error "Executable not found in expected locations"
        Write-Info "Searching for .exe files..."
        $ExeFiles = Get-ChildItem -Recurse -Filter "omnisystem*.exe" -ErrorAction SilentlyContinue
        if ($ExeFiles) {
            $FoundExe = $ExeFiles[0].FullName
            Write-Success "Found: $FoundExe"
        } else {
            Write-Error "No Omnisystem executable found"
            exit 1
        }
    }

    # Copy to root
    Write-Header "CREATING ROOT EXECUTABLE"
    Write-Info "Copying to: $ExePath"
    Copy-Item -Path $FoundExe -Destination $ExePath -Force
    Write-Success "Omnisystem.exe created in root directory"

    # Verify
    if (Test-Path $ExePath) {
        $FileSize = (Get-Item $ExePath).Length / 1MB
        Write-Success "Executable verified: $([math]::Round($FileSize, 2)) MB"
    } else {
        Write-Error "Executable copy failed"
        exit 1
    }

    Write-Header "BUILD COMPLETE"
    Write-Host ""
    Write-Host "🚀 LAUNCH OMNISYSTEM:" -ForegroundColor $Green
    Write-Host "   .\Omnisystem.exe" -ForegroundColor $Cyan
    Write-Host ""
    Write-Host "📍 Location: $ExePath" -ForegroundColor $Cyan
    Write-Host ""

    # Launch if requested
    if ($Launch) {
        Write-Info "Launching Omnisystem..."
        Start-Process $ExePath
        Write-Success "Application launched"
    }

} finally {
    Pop-Location
}

Write-Success "Build script completed successfully"
exit 0
