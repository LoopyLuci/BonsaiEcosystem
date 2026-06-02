# Team B: Concurrency Testing Build Script

$ErrorActionPreference = "Stop"

Write-Host "🚀 Building Team B: Concurrency Testing" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan

$startTime = Get-Date

Write-Host "`n📦 Building crate..." -ForegroundColor Yellow
cargo build --package bonsai-bedf-concurrency --release
if ($LASTEXITCODE -ne 0) { Write-Host "❌ Build failed" -ForegroundColor Red; exit 1 }

Write-Host "`n🧪 Running tests..." -ForegroundColor Yellow
cargo test --package bonsai-bedf-concurrency --release
if ($LASTEXITCODE -ne 0) { Write-Host "❌ Tests failed" -ForegroundColor Red; exit 1 }

Write-Host "`n🔍 Running clippy..." -ForegroundColor Yellow
cargo clippy --package bonsai-bedf-concurrency -- -D warnings

$duration = (Get-Date) - $startTime
Write-Host "`n✅ Team B build completed in $($duration.TotalSeconds)s" -ForegroundColor Green
