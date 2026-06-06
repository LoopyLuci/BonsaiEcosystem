#!/usr/bin/env pwsh
# Phase 119 Verification Suite - OmniLib Serialization & Data Formats

Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  PHASE 119 VERIFICATION SUITE                 ║" -ForegroundColor Cyan
Write-Host "║  OmniLib Serialization & Data Formats         ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Phase 119 modules
$phase119_modules = @(
    "titan/std/json.ti",
    "titan/std/msgpack.ti",
    "tests/test_omni_lib_serialization.ti"
)

# Regression tests (Phase 115-118)
$regression_modules = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/queue.ti",
    "titan/std/stack.ti",
    "titan/std/sort.ti",
    "titan/std/search.ti",
    "titan/std/hash.ti",
    "tests/test_omni_lib_algorithms.ti",
    "titan/std/io.ti",
    "titan/std/path.ti"
)

$all_modules = $phase119_modules + $regression_modules
$passed = 0
$failed = 0

# Test Phase 119 modules
Write-Host "PHASE 119 MODULES:" -ForegroundColor Cyan
foreach ($mod in $phase119_modules) {
    if (-not (Test-Path $mod)) {
        Write-Host "✗ $mod : FILE NOT FOUND" -ForegroundColor Red
        $failed++
        continue
    }
    
    $output = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($output -match "Result: 111") {
        Write-Host "✓ $mod : 111 [OK]" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $mod : FAILED" -ForegroundColor Red
        Write-Host "  Output: $output" -ForegroundColor Yellow
        $failed++
    }
}

Write-Host ""
Write-Host "REGRESSION TESTS (Phase 115-118):" -ForegroundColor Cyan
foreach ($mod in $regression_modules) {
    if (-not (Test-Path $mod)) {
        Write-Host "? $mod : FILE NOT FOUND" -ForegroundColor Yellow
        continue
    }
    
    $output = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($output -match "Result: 111") {
        Write-Host "✓ $mod : 111 [OK]" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $mod : REGRESSION FAILURE" -ForegroundColor Red
        Write-Host "  Output: $output" -ForegroundColor Yellow
        $failed++
    }
}

Write-Host ""
Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  RESULTS                                       ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Cyan

if ($failed -eq 0) {
    Write-Host "✓ All $passed module(s) verified, ZERO REGRESSIONS" -ForegroundColor Green
    Write-Host "✓ Phase 119 Serialization & Formats: All 3 modules passing" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $failed module(s) failed, $passed passed" -ForegroundColor Red
    exit 1
}
