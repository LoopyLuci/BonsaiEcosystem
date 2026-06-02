# Team D: Property Testing Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team D: Property Testing" -ForegroundColor Cyan
cargo build --package bonsai-bedf-property --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-property --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-property -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team D build completed" -ForegroundColor Green
