# verify_phase208.ps1 — Phase 208: OmniModel Runtime: 100% Native AI Inference
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/model/model_loader.ti",
    "titan/model/inference_engine.ti",
    "titan/model/local_policy.ti",
    "aether/gui/native_gui_agent.ae",
    "titan/aion/aion_native_gui_stage.ti",
    "tests/test_native_gui.ti"
)
$regression = @(
    "titan/gui/screen_capture.ti",
    "titan/gui/input_inject.ti",
    "titan/gui/action_parser.ti",
    "titan/ai/model_registry.ti",
    "titan/aion/aion_config.ti",
    "titan/std/vec.ti"
)
$failed = 0
Write-Host "`n=== Phase 208: OmniModel Runtime: 100% Native AI Inference ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase 208 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
