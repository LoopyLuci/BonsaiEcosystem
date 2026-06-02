# Team A: Fuzzing Engine Build Script
# This script builds only Team A's crate with all dependencies

$ErrorActionPreference = "Stop"
$WarningPreference = "Continue"

Write-Host "🚀 Building Team A: Fuzzing Engine" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan

$startTime = Get-Date

# Build
Write-Host "`n📦 Building crate..." -ForegroundColor Yellow
cargo build --package bonsai-bedf-fuzzing --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed" -ForegroundColor Red
    exit 1
}

# Test
Write-Host "`n🧪 Running tests..." -ForegroundColor Yellow
cargo test --package bonsai-bedf-fuzzing --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Tests failed" -ForegroundColor Red
    exit 1
}

# Lint
Write-Host "`n🔍 Running clippy..." -ForegroundColor Yellow
cargo clippy --package bonsai-bedf-fuzzing -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  Clippy warnings (non-fatal)" -ForegroundColor Yellow
}

# Format check
Write-Host "`n📝 Checking formatting..." -ForegroundColor Yellow
cargo fmt --package bonsai-bedf-fuzzing -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "⚠️  Format issues found. Run: cargo fmt --package bonsai-bedf-fuzzing" -ForegroundColor Yellow
}

$duration = (Get-Date) - $startTime
Write-Host "`n✅ Team A build completed in $($duration.TotalSeconds)s" -ForegroundColor Green
