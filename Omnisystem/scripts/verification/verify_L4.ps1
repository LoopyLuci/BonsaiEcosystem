# verify_L4.ps1 — Phase L4: Machine Learning Primitives
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @("titan/omnilib/l4_ml.ti", "tests/test_l4.ti")
$regression = @("titan/omnilib/l3_media.ti", "titan/omnilib/l2_db.ti")
$failed = 0
Write-Host "`n=== Phase L4: Machine Learning Primitives ===" -ForegroundColor Cyan
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
if ($failed -eq 0) { Write-Host "RESULT: ALL $total VERIFIED — Phase L4 complete [score: 111]" -ForegroundColor Green; exit 0 }
else { Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red; exit 1 }
