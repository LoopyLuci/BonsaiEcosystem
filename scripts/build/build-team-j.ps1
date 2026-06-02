# Team J: Survival System Integration Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team J: Survival System" -ForegroundColor Cyan
cargo build --package bonsai-survival-system-ext --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-survival-system-ext --release 2>&1 | tail -20
cargo clippy --package bonsai-survival-system-ext -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team J build completed" -ForegroundColor Green
