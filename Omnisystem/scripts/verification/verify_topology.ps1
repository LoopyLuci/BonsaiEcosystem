# verify_topology.ps1 — Aether Topology Map: renderer, graph engine, map, integration + regression
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$topology = @(
    "titan/ui/renderer.ti",
    "titan/ui/topology/graph.ti",
    "titan/ui/topology/aether_map.ti",
    "tests/test_topology.ti"
)

$regression = @(
    "titan/omnicore/omni_supervisor.ti",
    "titan/std/result.ti"
)

$failed = 0

Write-Host "`n=== Aether Topology Map ===" -ForegroundColor Cyan
foreach ($mod in $topology) {
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

$total = $topology.Count + $regression.Count
Write-Host ""
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $total VERIFIED — Topology OK [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
