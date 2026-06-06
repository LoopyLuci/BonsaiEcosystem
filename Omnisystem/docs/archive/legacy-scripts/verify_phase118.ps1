#!/usr/bin/env pwsh
# Phase 118 Verification Suite - OmniLib I/O & Filesystem

Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  PHASE 118 VERIFICATION SUITE                 ║" -ForegroundColor Cyan
Write-Host "║  OmniLib I/O & Filesystem                     ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Phase 118 modules
$phase118_modules = @(
    "titan/std/io.ti",
    "titan/std/path.ti",
    "tests/test_omni_lib_io.ti"
)

# Regression tests (Phase 115, 116, 117)
$regression_modules = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/queue.ti",
    "titan/std/stack.ti",
    "titan/std/set.ti",
    "titan/std/btree.ti",
    "titan/std/graph.ti",
    "titan/std/priority_queue.ti",
    "tests/test_omni_lib_algorithms.ti"
)

$all_modules = $phase118_modules + $regression_modules
$passed = 0
$failed = 0

# Test Phase 118 modules
Write-Host "PHASE 118 MODULES:" -ForegroundColor Cyan
foreach ($mod in $phase118_modules) {
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
Write-Host "REGRESSION TESTS (Phase 115/116/117):" -ForegroundColor Cyan
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
    Write-Host "✓ Phase 118 I/O & Filesystem: All 3 modules passing" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $failed module(s) failed, $passed passed" -ForegroundColor Red
    exit 1
}
