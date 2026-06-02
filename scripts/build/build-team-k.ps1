# Team K: Knowledge Database Integration Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team K: Knowledge Database" -ForegroundColor Cyan
cargo build --package bonsai-kdb-ext --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-kdb-ext --release 2>&1 | tail -20
cargo clippy --package bonsai-kdb-ext -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team K build completed" -ForegroundColor Green
