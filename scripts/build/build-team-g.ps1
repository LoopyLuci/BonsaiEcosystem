# Team G: Triage & AI Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team G: Triage & AI" -ForegroundColor Cyan
cargo build --package bonsai-bedf-triage --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-triage --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-triage -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team G build completed" -ForegroundColor Green
