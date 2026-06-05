#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Omnisystem Alpha 0.1 Installation Script (Windows)

.DESCRIPTION
    Downloads Omnisystem, sets up Python virtual environment,
    installs dependencies, and configures PATH for the 'omni' command.

.PARAMETER OutPath
    Installation directory (default: $env:USERPROFILE\omnisystem)

.PARAMETER SkipPython
    Skip Python version check (use existing Python)

.EXAMPLE
    .\install.ps1
    .\install.ps1 -OutPath "C:\tools\omnisystem"
#>

param(
    [string]$OutPath = "$env:USERPROFILE\omnisystem",
    [switch]$SkipPython = $false
)

# Colors for output
$Colors = @{
    Success = "Green"
    Error   = "Red"
    Info    = "Cyan"
    Warn    = "Yellow"
}

function Write-Status {
    param([string]$Message, [string]$Level = "Info")
    Write-Host $Message -ForegroundColor $Colors[$Level]
}

function Write-Error {
    param([string]$Message)
    Write-Host "ERROR: $Message" -ForegroundColor $Colors["Error"]
}

# Header
Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║      OMNISYSTEM ALPHA 0.1 — INSTALLATION (Windows)          ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
Write-Status "[1/5] Checking prerequisites..." "Info"

# Check Python
$PythonCmd = $null
$PythonVersions = @("python3", "python")

foreach ($cmd in $PythonVersions) {
    try {
        $version = & $cmd --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            $PythonCmd = $cmd
            Write-Status "  Python found: $version" "Success"
            break
        }
    } catch {
        # Continue to next option
    }
}

if (!$PythonCmd) {
    Write-Error "Python not found on PATH. Please install Python 3.10+ from https://www.python.org"
    exit 1
}

# Check Git
try {
    $gitVersion = git --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Status "  Git found: $gitVersion" "Success"
    }
} catch {
    Write-Error "Git not found. Please install Git from https://git-scm.com"
    exit 1
}

# Create installation directory
Write-Status "[2/5] Setting up installation directory..." "Info"

if (!(Test-Path $OutPath)) {
    New-Item -ItemType Directory -Path $OutPath -Force | Out-Null
    Write-Status "  Created: $OutPath" "Success"
} else {
    Write-Status "  Directory exists: $OutPath" "Success"
}

# Clone or update repository
Write-Status "[3/5] Downloading Omnisystem source..." "Info"

if (Test-Path "$OutPath\.git") {
    Write-Status "  Repository exists, updating..." "Info"
    cd $OutPath
    git pull origin main 2>&1 | Select-Object -First 3
} else {
    Write-Status "  Cloning repository..." "Info"
    git clone https://github.com/omnilang/omnisystem.git "$OutPath" 2>&1 | Select-Object -First 3
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to clone repository"
        exit 1
    }
    cd $OutPath
}

Write-Status "  Source ready at: $OutPath" "Success"

# Create virtual environment
Write-Status "[4/5] Setting up Python environment..." "Info"

$VenvPath = "$OutPath\.venv"

if (Test-Path $VenvPath) {
    Write-Status "  Virtual environment exists" "Success"
} else {
    Write-Status "  Creating virtual environment..." "Info"
    & $PythonCmd -m venv $VenvPath
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Failed to create virtual environment"
        exit 1
    }
}

# Activate venv and install dependencies
$ActivateScript = "$VenvPath\Scripts\Activate.ps1"
& $ActivateScript

Write-Status "  Installing Python dependencies..." "Info"
pip install llvmlite==0.47.0 pytest 2>&1 | Select-Object -Last 2

Write-Status "  Python environment ready" "Success"

# Configure PATH
Write-Status "[5/5] Configuring PATH..." "Info"

$OmniPath = "$OutPath\tools\omni"
$OmniCmd = "$OmniPath\main.py"

# Create wrapper script
$WrapperScript = @"
@echo off
python "$OmniCmd" %*
"@

$WrapperPath = "$VenvPath\Scripts\omni.cmd"
Set-Content -Path $WrapperPath -Value $WrapperScript
Write-Status "  Created wrapper: $WrapperPath" "Success"

# Add to PATH if not already there
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$VenvBinPath = "$VenvPath\Scripts"

if ($UserPath -notlike "*$VenvBinPath*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$UserPath;$VenvBinPath",
        "User"
    )
    Write-Status "  Added to PATH: $VenvBinPath" "Success"
} else {
    Write-Status "  Already in PATH: $VenvBinPath" "Success"
}

# Success message
Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║              ✓ INSTALLATION COMPLETE                        ║" -ForegroundColor Green
Write-Host "╚══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""

Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Restart your terminal or run:"
Write-Host "   `$env:PATH = [System.Environment]::GetEnvironmentVariable('PATH','User')" -ForegroundColor Yellow
Write-Host ""
Write-Host "2. Verify installation:"
Write-Host "   omni --version" -ForegroundColor Yellow
Write-Host ""
Write-Host "3. Get started:"
Write-Host "   omni new myapp" -ForegroundColor Yellow
Write-Host "   cd myapp" -ForegroundColor Yellow
Write-Host "   omni run examples/hello_world.omni" -ForegroundColor Yellow
Write-Host ""
Write-Host "4. Try the REPL:"
Write-Host "   omni repl" -ForegroundColor Yellow
Write-Host ""
Write-Host "Documentation: https://omnilang.org/getting-started" -ForegroundColor Cyan
Write-Host "Community: https://github.com/omnilang/omnisystem" -ForegroundColor Cyan
Write-Host ""
