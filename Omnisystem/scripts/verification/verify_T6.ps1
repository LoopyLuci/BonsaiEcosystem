# verify_T6.ps1 — Phase T6: Integration Pipeline & Dashboards
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/omnitoken/t6_pipeline.ti",
    "tests/test_t6.ti"
)
$regression = @(
    "titan/omnitoken/t5_recycle.ti",
    "titan/omnitoken/t4_kvcache.ti",
    "titan/omnitoken/t3_abstract.ti",
    "titan/omnitoken/t2_compress.ti",
    "titan/omnitoken/t1_scorer.ti"
)
$failed = 0
Write-Host "`n=== Phase T6: Integration Pipeline & Dashboards ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase T6 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
