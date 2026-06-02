# Team F: Sandbox Orchestration Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team F: Sandbox Orchestration" -ForegroundColor Cyan
cargo build --package bonsai-bedf-sandbox --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-sandbox --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-sandbox -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team F build completed" -ForegroundColor Green
