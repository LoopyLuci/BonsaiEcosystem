# verify_proof_lens.ps1 — Proof Explorer as fourth Inspector lens
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/ui/inspector.ti",
    "tests/test_proof_lens.ti",
    "tests/test_inspector.ti"
)

$regression = @(
    "titan/omnicore/events.ti",
    "titan/ui/proof_explorer.ti",
    "titan/ui/axiom_console.ti",
    "titan/ui/omni_shell.ti",
    "titan/ui/topology/aether_map.ti"
)

$failed = 0

Write-Host "`n=== Proof Explorer Lens (Truth Layer) ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}

Write-Host "`n=== Regression Check ===" -ForegroundColor Yellow
foreach ($mod in $regression) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}

$total = $modules.Count + $regression.Count
Write-Host ""
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $total VERIFIED — Proof Lens OK [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
