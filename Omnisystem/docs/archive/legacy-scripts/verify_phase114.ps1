# Phase 114 Verification Script

$modules = @(
    "titan/sylva_runtime/repl.ti",
    "titan/sylva_runtime/stdlib.ti",
    "titan/sylva_runtime/error_handler.ti",
    "tests/test_sylva_runtime.ti"
)

$regression = @(
    "titan/sylva_interp/lexer.ti",
    "titan/sylva_interp/eval.ti"
)

Write-Host "PHASE 114 - SYLVA RUNTIME AND INTERACTIVE REPL" -ForegroundColor Cyan
Write-Host ""

$passed = 0
$failed = 0

Write-Host "Phase 114 Modules:" -ForegroundColor Yellow
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
Write-Host "Regression Tests:" -ForegroundColor Yellow
foreach ($reg in $regression) {
    $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $reg --run 2>&1
    if ($result -match "Result: 111") {
        Write-Host "  [PASS] $reg" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  [FAIL] $reg" -ForegroundColor Red
        $failed++
    }
}

Write-Host ""
if ($failed -eq 0) {
    Write-Host "SUCCESS: All $passed modules verified" -ForegroundColor Green
    Write-Host "Phase 114 Complete" -ForegroundColor Green
} else {
    Write-Host "FAILED: $failed of $($passed + $failed) modules failed" -ForegroundColor Red
}
