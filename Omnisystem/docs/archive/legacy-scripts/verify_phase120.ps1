#!/usr/bin/env pwsh
# Phase 120 Verification Suite - Networking & HTTP

Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  PHASE 120 VERIFICATION SUITE                 ║" -ForegroundColor Cyan
Write-Host "║  OmniLib Networking & HTTP                    ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$phase120 = @(
    "titan/std/net.ti",
    "titan/std/url.ti",
    "titan/std/http.ti",
    "tests/test_omni_lib_networking.ti"
)

$regression = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/queue.ti",
    "titan/std/stack.ti",
    "titan/std/sort.ti",
    "titan/std/search.ti",
    "titan/std/json.ti"
)

$all = $phase120 + $regression
$passed = 0
$failed = 0

Write-Host "PHASE 120 MODULES:" -ForegroundColor Cyan
foreach ($mod in $phase120) {
    $output = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    $result = $output | Select-String -Pattern "Result: (\d+)" | ForEach-Object { $_.Matches[0].Groups[1].Value }
    
    if ($result -eq "111") {
        Write-Host "✓ $($mod) : 111 [OK]" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $($mod) : Result: $result [FAIL]" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "REGRESSION TESTS (Phase 115-119):" -ForegroundColor Cyan
foreach ($mod in $regression) {
    $output = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    $result = $output | Select-String -Pattern "Result: (\d+)" | ForEach-Object { $_.Matches[0].Groups[1].Value }
    
    if ($result -eq "111") {
        Write-Host "✓ $($mod) : 111 [OK]" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $($mod) : Result: $result [FAIL]" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
Write-Host "╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  RESULTS                                       ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝" -ForegroundColor Cyan

if ($failed -eq 0) {
    Write-Host "✓ All $($passed) module(s) verified, ZERO REGRESSIONS" -ForegroundColor Green
    Write-Host "✓ Phase 120 Networking & HTTP: All 4 modules passing" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $failed module(s) failed, $passed passed" -ForegroundColor Red
    exit 1
}
