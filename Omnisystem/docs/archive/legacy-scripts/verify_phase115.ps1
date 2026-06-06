#!/usr/bin/env powershell
# Phase 115 Verification Script

Write-Host "PHASE 115 - OMNILIB FOUNDATION" -ForegroundColor Cyan
Write-Host ""

$modules = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/string.ti",
    "tests/test_omni_lib.ti"
)

$regression = @(
    "titan/sylva_interp/lexer.ti",
    "titan/sylva_interp/eval.ti"
)

$all = $modules + $regression
$passed = 0
$failed = 0

Write-Host "Phase 115 Modules:" -ForegroundColor White
foreach ($mod in $modules) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "  [PASS] $mod" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  [FAIL] $mod" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "Regression Tests:" -ForegroundColor White
foreach ($mod in $regression) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "  [PASS] $mod" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  [FAIL] $mod" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
if ($failed -eq 0) {
    Write-Host "SUCCESS: All modules verified" -ForegroundColor Green
    exit 0
} else {
    Write-Host "FAILED: Some modules did not pass" -ForegroundColor Red
    exit 1
}
