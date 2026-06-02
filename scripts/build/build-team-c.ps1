# Team C: Memory Sanitizers Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team C: Memory Sanitizers" -ForegroundColor Cyan
$startTime = Get-Date
cargo build --package bonsai-bedf-sanitizers --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-sanitizers --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-sanitizers -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team C build completed" -ForegroundColor Green
