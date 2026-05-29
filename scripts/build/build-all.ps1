<#
.SYNOPSIS
    Cross-platform Bonsai Workspace build script (Windows PowerShell).

.PARAMETER Watchdog
    Also build the bonsai-watchdog survival binary.

.PARAMETER Clean
    Remove target/ dirs before building.

.EXAMPLE
    .\build-all.ps1
    .\build-all.ps1 -Watchdog
#>
param(
    [switch]$Watchdog,
    [switch]$Clean
)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$Root       = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
$TauriSrc   = Join-Path $Root 'bonsai-workspace\src'
$TauriDir   = Join-Path $Root 'bonsai-workspace\src-tauri'
$WatchdogManifest = Join-Path $Root 'crates\bonsai-watchdog\Cargo.toml'

function Require-Command([string]$Name) {
    if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
        Write-Host "ERROR: '$Name' not found. Install it and re-run." -ForegroundColor Red
        exit 1
    }
}

function Run([string]$Desc, [scriptblock]$Block) {
    Write-Host "==> $Desc" -ForegroundColor Cyan
    & $Block
    if ($LASTEXITCODE -ne 0) {
        Write-Host "FAILED: $Desc (exit $LASTEXITCODE)" -ForegroundColor Red
        exit $LASTEXITCODE
    }
}

Require-Command cargo
Require-Command node
Require-Command npm

if (Get-Command sccache -ErrorAction SilentlyContinue) {
    $env:RUSTC_WRAPPER = 'sccache'
    Write-Host "    sccache enabled" -ForegroundColor DarkGray
}

if ($Clean) {
    Write-Host "==> Cleaning target directories" -ForegroundColor Yellow
    Remove-Item -Recurse -Force (Join-Path $TauriDir 'target') -ErrorAction SilentlyContinue
}

# ── Frontend ──────────────────────────────────────────────────────────────────
Run "Installing frontend dependencies" {
    npm --prefix $TauriSrc install --prefer-offline --no-audit --no-fund --loglevel=error
}

# ── Tauri build ───────────────────────────────────────────────────────────────
$tauriBuildOk = $false

# Try cargo tauri
if (Get-Command 'cargo-tauri' -ErrorAction SilentlyContinue) {
    Write-Host "==> Building with cargo tauri build" -ForegroundColor Cyan
    Push-Location $TauriDir
    & cargo tauri build
    if ($LASTEXITCODE -eq 0) { $tauriBuildOk = $true }
    Pop-Location
}

# Try npx tauri
if (-not $tauriBuildOk) {
    Write-Host "==> Building with npx tauri build" -ForegroundColor Cyan
    Push-Location $TauriSrc
    & npx --no-install tauri build 2>$null
    if ($LASTEXITCODE -ne 0) { & npx tauri build }
    if ($LASTEXITCODE -eq 0) { $tauriBuildOk = $true }
    Pop-Location
}

# Fallback: plain cargo build
if (-not $tauriBuildOk) {
    Write-Host "==> Tauri CLI unavailable — building frontend + cargo" -ForegroundColor Yellow
    Run "Frontend build (npm run build)" {
        npm --prefix $TauriSrc run build
    }
    Run "Cargo build --release" {
        Push-Location $TauriDir
        cargo build --release
        Pop-Location
    }
    $tauriBuildOk = $true
}

# ── Watchdog ──────────────────────────────────────────────────────────────────
if ($Watchdog -and (Test-Path $WatchdogManifest)) {
    Run "Building bonsai-watchdog" {
        cargo build --release --manifest-path $WatchdogManifest
    }
    $watchdogSrc = Join-Path $Root 'crates\bonsai-watchdog\target\release\bonsai-watchdog.exe'
    if (Test-Path $watchdogSrc) {
        $null = New-Item -ItemType Directory -Force -Path (Join-Path $Root 'target\release')
        Copy-Item $watchdogSrc (Join-Path $Root 'target\release\bonsai-watchdog.exe') -Force
        Write-Host "==> Watchdog: $(Join-Path $Root 'target\release\bonsai-watchdog.exe')" -ForegroundColor Green
    }
}

# ── Summary ───────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "Build complete." -ForegroundColor Green
$bundleDir = Join-Path $TauriDir 'target\release\bundle'
if (Test-Path $bundleDir) {
    $installers = Get-ChildItem $bundleDir -Recurse -Include '*.msi','*.exe','*.nsis' -File -ErrorAction SilentlyContinue
    foreach ($f in $installers) {
        $mb = [Math]::Round($f.Length/1MB, 1)
        Write-Host "  Installer: $($f.FullName) ($mb MB)" -ForegroundColor Green
    }
}
