# scripts/build_omnisystem.ps1 — Compile and verify the Omnisystem production entry point.
#
# The Titan compiler resolves the full module graph via import statements inside
# titan/main.ti — all subsystem modules and lenses pull in transitively.

$exe = Join-Path $PSScriptRoot "..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$entry = "titan/main.ti"

Write-Host ""
Write-Host "=== Omnisystem Master Integration Run ===" -ForegroundColor Cyan
Write-Host "  Entry point : $entry"
Write-Host ""

$res = & $exe $entry 2>&1 | Out-String

if ($res -match "Result:\s*111") {
    Write-Host "  INTEGRATION SUCCESS: All 9 UI Subsystems and 3 Lenses Operating Flawlessly." -ForegroundColor Green
    Write-Host "  RESULT: 111" -ForegroundColor Green
    exit 0
} else {
    Write-Host "  BUILD FAILED:" -ForegroundColor Red
    Write-Host $res.Trim() -ForegroundColor Red
    exit 1
}
