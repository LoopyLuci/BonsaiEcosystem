# verify_phaseS2.ps1 — Phase S2: OmniSplat: Gaussian Splat Shader Pipeline
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/omnisplat/splat_shader.ti",
    "titan/omnisplat/overlay_shader.ti",
    "titan/omnisplat/outline_shader.ti",
    "titan/omnisplat/underlay_shader.ti",
    "titan/omnisplat/pick_shader.ti",
    "titan/omnisplat/compositor.ti",
    "tests/test_omnisplat_s2.ti"
)
$regression = @(
    "titan/omnisplat/gsplat_data.ti",
    "titan/omnisplat/gsplat_io.ti",
    "titan/omnisplat/gsplat_gpu.ti"
)
$failed = 0
Write-Host "`n=== Phase S2: OmniSplat: Gaussian Splat Shader Pipeline ===" -ForegroundColor Cyan
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
    Write-Host "RESULT: ALL $total VERIFIED — Phase S2 complete [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
