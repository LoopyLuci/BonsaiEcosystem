# Team H: MCP Tools Build Script
$ErrorActionPreference = "Stop"
Write-Host "🚀 Building Team H: MCP Tools" -ForegroundColor Cyan
cargo build --package bonsai-bedf-mcp --release 2>&1 | tail -20
if ($LASTEXITCODE -ne 0) { exit 1 }
cargo test --package bonsai-bedf-mcp --release 2>&1 | tail -20
cargo clippy --package bonsai-bedf-mcp -- -D warnings 2>&1 | tail -10
Write-Host "✅ Team H build completed" -ForegroundColor Green
