$modules = @(
    "titan/std/queue.ti",
    "titan/std/stack.ti",
    "titan/std/set.ti",
    "titan/std/btree.ti",
    "titan/std/graph.ti",
    "titan/std/priority_queue.ti",
    "tests/test_omni_lib_advanced.ti"
)

$regression = @(
    "titan/std/vec.ti",
    "titan/std/map.ti",
    "titan/std/string.ti"
)

$all = $modules + $regression
$passed = 0
$failed = 0

Write-Host "`n=== PHASE 116 VERIFICATION - OmniLib Advanced ===" -ForegroundColor Cyan
Write-Host "7 New Modules + 3 Regression Tests`n" -ForegroundColor Cyan

foreach ($mod in $all) {
    $output = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
    if ($output -match "Result: 111") {
        Write-Host "OK $mod" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "FAIL $mod" -ForegroundColor Red
        $failed++
    }
}

Write-Host "`n--- SUMMARY ---" -ForegroundColor Cyan
Write-Host "New modules: $($modules.Count), Regression: $($regression.Count), Total: $($all.Count)" -ForegroundColor White
Write-Host "Passed: $passed, Failed: $failed" -ForegroundColor White

if ($failed -eq 0) {
    Write-Host "`nSUCCESS: All 10 modules verified" -ForegroundColor Green
    Write-Host "Phase 116 complete: OmniLib Advanced Data Structures ready" -ForegroundColor Green
} else {
    Write-Host "`nFAILED: $failed module(s) need attention" -ForegroundColor Red
}
