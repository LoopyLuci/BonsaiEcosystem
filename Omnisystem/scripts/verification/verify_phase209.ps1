# verify_phase209.ps1 — Phase 209: OmniStudio: Unified IDE Shell
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/studio/studio_config.ti",
    "titan/studio/service_manager.ti",
    "titan/studio/command_palette.ti",
    "aether/studio/event_bus.ae",
    "titan/studio/ide_main.ti",
    "tests/test_ide.ti"
)
$regression = @(
    "titan/omnishell/window.ti",
    "titan/omnishell/ipc.ti",
    "titan/omniview/renderer.ti",
    "titan/lsp/lsp_server.ti",
    "titan/agent_core/orchestrator.ti",
    "titan/aion/pipeline_runner.ti"
)
$failed = 0
Write-Host "`n=== Phase 209: OmniStudio: Unified IDE Shell ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase 209 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
