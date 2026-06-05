# verify_F7.ps1 — Phase F7: Audio Engine
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @("titan/omnidesign/f7_audio.ti", "tests/test_f7.ti")
$regression = @("titan/omnidesign/f6_anim.ti", "titan/omnidesign/f5_physics.ti")
$failed = 0
Write-Host "`n=== Phase F7: Audio Engine ===" -ForegroundColor Cyan
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
if ($failed -eq 0) { Write-Host "RESULT: ALL $total VERIFIED — Phase F7 complete [score: 111]" -ForegroundColor Green; exit 0 }
else { Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red; exit 1 }
