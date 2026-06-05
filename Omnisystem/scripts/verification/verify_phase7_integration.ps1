# verify_phase7_integration.ps1 — Phase 7: Cross-subsystem integration tests
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "tests/test_integration.ti",
    "tests/test_omniwindow.ti",
    "tests/test_omniNet.ti",
    "tests/test_omniThread.ti",
    "tests/test_omniAudio.ti"
)
$regression = @(
    "titan/axlib/ax2_list.ti",
    "titan/omnilib/l1_net.ti",
    "titan/omniview/omniwindow.ti"
)

$failed = 0
Write-Host "`n=== Phase 7: Cross-Subsystem Integration Tests ===" -ForegroundColor Cyan
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
if ($failed -eq 0) { Write-Host "RESULT: ALL $total VERIFIED - Phase 7 complete [score: 111]" -ForegroundColor Green; exit 0 }
else { Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red; exit 1 }
