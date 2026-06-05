# verify_omnisplat.ps1 — OmniSplat Master Verification (all 35 modules)
$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$all = @(
    "titan/omnisplat/gsplat_data.ti",
    "titan/omnisplat/gsplat_io.ti",
    "titan/omnisplat/gsplat_gpu.ti",
    "titan/omnisplat/splat_shader.ti",
    "titan/omnisplat/overlay_shader.ti",
    "titan/omnisplat/outline_shader.ti",
    "titan/omnisplat/underlay_shader.ti",
    "titan/omnisplat/pick_shader.ti",
    "titan/omnisplat/compositor.ti",
    "titan/omnisplat/camera.ti",
    "titan/omnisplat/scene.ti",
    "titan/omnisplat/navigation.ti",
    "titan/omnisplat/s4_sel.ti",
    "titan/omnisplat/s5_xform.ti",
    "titan/omnisplat/s6_undo.ti",
    "titan/omnisplat/s7_timeline.ti",
    "titan/omnisplat/s8_export.ti",
    "titan/omnisplat/s9_ui.ti",
    "titan/omnisplat/s10_ide.ti",
    "titan/omnisplat/s11_pub.ti",
    "titan/omnisplat/s12_verify.ti",
    "titan/omnisplat/s13_app.ti",
    "tests/test_omnisplat_s1.ti",
    "tests/test_omnisplat_s2.ti",
    "tests/test_omnisplat_s3.ti",
    "tests/test_s4.ti",
    "tests/test_s5.ti",
    "tests/test_s6.ti",
    "tests/test_s7.ti",
    "tests/test_s8.ti",
    "tests/test_s9.ti",
    "tests/test_s10.ti",
    "tests/test_s11.ti",
    "tests/test_s12.ti",
    "tests/test_s13.ti"
)

$failed = 0
$passed = 0
Write-Host "`n=== OmniSplat Master Verification — $($all.Count) modules ===" -ForegroundColor Cyan
foreach ($mod in $all) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") {
        Write-Host "  PASS  $mod" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "  Passed: $passed / $($all.Count)" -ForegroundColor Cyan
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $($all.Count) VERIFIED — OmniSplat COMPLETE [score: 111]" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed / $($all.Count) FAILED" -ForegroundColor Red
    exit 1
}
