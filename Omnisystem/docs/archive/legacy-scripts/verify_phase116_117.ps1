# Verify Phase 116 Data Structures & Phase 117 Algorithms

$modules = @(
    "titan/std/queue.ti",
    "titan/std/stack.ti",
    "titan/std/set.ti",
    "titan/std/btree.ti",
    "titan/std/graph.ti",
    "titan/std/priority_queue.ti",
    "tests/test_omni_lib_algorithms.ti"
)

# Phase 115 regression tests
$regression = @(
    "titan/std/vec.ti",
    "titan/std/map.ti"
)

$all = $modules + $regression
$passed = 0
$failed = 0

Write-Host "`n╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  PHASE 116/117 VERIFICATION SUITE              ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

foreach ($mod in $all) {
    if (-not (Test-Path $mod)) {
        Write-Host "⊘ $mod : NOT FOUND" -ForegroundColor Yellow
        $failed++
        continue
    }
    
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    $score = $result -replace ".*Result: (\d+).*", '$1'
    
    if ($score -eq "111") {
        Write-Host "✓ $mod : 111 [OK]" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "✗ $mod : $score [FAIL]" -ForegroundColor Red
        $failed++
    }
}

Write-Host "`n╔════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  RESULTS                                       ║" -ForegroundColor Cyan
Write-Host "╚════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

if ($failed -eq 0) {
    Write-Host "✓ All $($all.Count) modules verified, ZERO REGRESSIONS" -ForegroundColor Green
    Write-Host "✓ Phase 116 Data Structures: All 6 modules passing" -ForegroundColor Green
    Write-Host "✓ Phase 117 Algorithms: All 1 module passing" -ForegroundColor Green
    exit 0
} else {
    Write-Host "✗ $failed module(s) failed, $passed passed" -ForegroundColor Red
    exit 1
}
