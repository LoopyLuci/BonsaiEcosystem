# Team I: Advanced Enhancements Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team I: Advanced Enhancements" -ForegroundColor Cyan
cargo build --package bonsai-bedf-enhancements --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-enhancements --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-enhancements -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team I build completed" -ForegroundColor Green
