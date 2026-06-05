# verify_ui_master.ps1 — OmniUI Master Integration Shell + full stack regression
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/ui/omni_ui_master.ti"
)

$regression = @(
    "titan/ui/omni_shell.ti",
    "titan/ui/inspector.ti",
    "titan/ui/command_palette.ti",
    "titan/ui/proof_explorer.ti",
    "titan/ui/axiom_console.ti",
    "titan/ui/topology/aether_map.ti",
    "titan/ui/topology/graph.ti",
    "titan/ui/renderer.ti",
    "titan/omnicore/config_change.ti",
    "titan/omnicore/events.ti"
)

$failed = 0

Write-Host "`n=== OmniUI Master Integration Shell ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}

Write-Host "`n=== Full Stack Regression ===" -ForegroundColor Yellow
foreach ($mod in $regression) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}

$total = $modules.Count + $regression.Count
Write-Host ""
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $total VERIFIED — OmniUI Master OK [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
