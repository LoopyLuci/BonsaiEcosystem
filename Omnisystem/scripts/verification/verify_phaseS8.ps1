# verify_phaseS8.ps1 — Phase S8: OmniSplat: File I/O & Export
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/omnisplat/s8_export.ti",
    "tests/test_s8.ti"
)
$regression = @(
    "titan/omnisplat/s7_timeline.ti",
    "titan/omnisplat/s6_undo.ti"
)
$failed = 0
Write-Host "`n=== Phase S8: OmniSplat: File I/O & Export ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase S8 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
