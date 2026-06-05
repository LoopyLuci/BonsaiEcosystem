# verify_phase206.ps1 — Phase 206: Aion Integration: Autonomous Software Factory
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/aion/aion_config.ti",
    "titan/aion/specification_parser.ti",
    "titan/aion/code_generator.ti",
    "titan/aion/pipeline_runner.ti",
    "tests/test_aion.ti"
)
$regression = @(
    "titan/ai/model_registry.ti",
    "titan/ai/model_router.ti",
    "titan/agent_core/orchestrator.ti",
    "titan/lsp/lsp_server.ti",
    "titan/std/vec.ti"
)
$failed = 0
Write-Host "`n=== Phase 206: Aion Integration: Autonomous Software Factory ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}
Write-Host "`n=== Regression ===" -ForegroundColor Cyan
foreach ($mod in $regression) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}
$total = $modules.Count + $regression.Count
Write-Host ""
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $total VERIFIED — Phase 206 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
