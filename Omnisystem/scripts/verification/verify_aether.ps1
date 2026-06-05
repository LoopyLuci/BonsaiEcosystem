# verify_aether.ps1 — Aether language modules: CRDTs, session types, virtual actors
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/aether/omnicrodt.ti",
    "titan/aether/session.ti",
    "titan/aether/actor_virtual.ti"
)

$failed = 0
Write-Host "`n=== Aether: CRDTs / Session Types / Virtual Actors ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}
$total = $modules.Count
Write-Host ""
if ($failed -eq 0) { Write-Host "RESULT: ALL $total VERIFIED - Aether complete [score: 111]" -ForegroundColor Green; exit 0 }
else { Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red; exit 1 }
