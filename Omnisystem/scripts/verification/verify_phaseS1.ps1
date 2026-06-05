# verify_phaseS1.ps1 — Phase S1: OmniSplat: Gaussian Splat Data Layer
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/omnisplat/gsplat_data.ti",
    "titan/omnisplat/gsplat_io.ti",
    "titan/omnisplat/gsplat_gpu.ti",
    "tests/test_omnisplat_s1.ti"
)
$regression = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/io.ti",
    "titan/studio/ide_main.ti"
)
$failed = 0
Write-Host "`n=== Phase S1: OmniSplat: Gaussian Splat Data Layer ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase S1 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
