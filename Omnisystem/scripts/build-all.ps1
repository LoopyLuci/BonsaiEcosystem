# Master Build Script for Omnisystem
# Coordinates all build tasks and phases

param(
    [ValidateSet('Debug', 'Release')]
    [string]$Configuration = 'Release',

    [switch]$Clean,
    [switch]$Test,
    [switch]$Package,
    [switch]$Verify,
    [switch]$All
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir

Write-Host "`n╔══════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║   OMNISYSTEM BUILD MASTER                    ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════╝`n" -ForegroundColor Cyan

Write-Host "Configuration: $Configuration" -ForegroundColor Yellow
Write-Host "Root Directory: $RootDir`n" -ForegroundColor Gray

# Default: Run clean build
if (-not $Clean -and -not $Test -and -not $Package -and -not $Verify -and -not $All) {
    $Clean = $true
}

# If -All specified, do everything
if ($All) {
    $Clean = $true
    $Test = $true
    $Package = $true
    $Verify = $true
}

try {
    # Phase 1: Clean
    if ($Clean) {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
        Write-Host "Phase 1: CLEAN" -ForegroundColor Cyan
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Cyan

        Write-Host "Cleaning build artifacts..." -ForegroundColor Gray
        cd $RootDir
        cargo clean
        Write-Host "✓ Artifacts cleaned`n" -ForegroundColor Green
    }

    # Phase 2: Build
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
    Write-Host "Phase 2: BUILD ($Configuration)" -ForegroundColor Cyan
    Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Cyan

    Write-Host "Building Omnisystem..." -ForegroundColor Gray
    cd $RootDir

    if ($Configuration -eq 'Release') {
        cargo build --workspace --release
    } else {
        cargo build --workspace
    }

    Write-Host "✓ Build complete`n" -ForegroundColor Green

    # Phase 3: Test (optional)
    if ($Test) {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
        Write-Host "Phase 3: TEST" -ForegroundColor Cyan
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Cyan

        Write-Host "Running test suite..." -ForegroundColor Gray
        cd $RootDir
        cargo test --workspace
        Write-Host "✓ Tests passed`n" -ForegroundColor Green
    }

    # Phase 4: Verify (optional)
    if ($Verify) {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
        Write-Host "Phase 4: VERIFY" -ForegroundColor Cyan
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Cyan

        if (Test-Path "$ScriptDir\verification\master_verify.ps1") {
            & "$ScriptDir\verification\master_verify.ps1"
            Write-Host "✓ Verification complete`n" -ForegroundColor Green
        } else {
            Write-Host "⚠ Verification script not found`n" -ForegroundColor Yellow
        }
    }

    # Phase 5: Package (optional)
    if ($Package) {
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Cyan
        Write-Host "Phase 5: PACKAGE" -ForegroundColor Cyan
        Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━`n" -ForegroundColor Cyan

        if (Test-Path "$ScriptDir\build\package_release.ps1") {
            & "$ScriptDir\build\package_release.ps1"
            Write-Host "✓ Package complete`n" -ForegroundColor Green
        } else {
            Write-Host "⚠ Package script not found`n" -ForegroundColor Yellow
        }
    }

    # Summary
    Write-Host "╔══════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║   ✓ BUILD COMPLETE                          ║" -ForegroundColor Green
    Write-Host "╚══════════════════════════════════════════════╝`n" -ForegroundColor Green

    $tasks = @()
    if ($Clean) { $tasks += "Clean" }
    $tasks += "Build"
    if ($Test) { $tasks += "Test" }
    if ($Verify) { $tasks += "Verify" }
    if ($Package) { $tasks += "Package" }

    Write-Host "Completed: $($tasks -join ' → ')`n" -ForegroundColor Green
    Write-Host "Configuration: $Configuration" -ForegroundColor Gray
    Write-Host "Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')`n" -ForegroundColor Gray

} catch {
    Write-Host "╔══════════════════════════════════════════════╗" -ForegroundColor Red
    Write-Host "║   ✗ BUILD FAILED                            ║" -ForegroundColor Red
    Write-Host "╚══════════════════════════════════════════════╝`n" -ForegroundColor Red
    Write-Host "Error: $_`n" -ForegroundColor Red
    exit 1
}
