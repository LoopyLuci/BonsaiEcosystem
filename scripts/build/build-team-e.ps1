# Team E: Penetration Testing Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team E: Penetration Testing" -ForegroundColor Cyan
cargo build --package bonsai-bedf-pentest --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-pentest --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-pentest -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team E build completed" -ForegroundColor Green
