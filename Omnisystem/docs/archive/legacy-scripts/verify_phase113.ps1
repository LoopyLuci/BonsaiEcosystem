# Phase 113 Verification Script

$modules = @(
    "titan/sylva_interp/lexer.ti",
    "titan/sylva_interp/parser.ti",
    "titan/sylva_interp/eval.ti",
    "tests/test_sylva_interp.ti"
)

$regression = @(
    "titan/kernel/inductive.ti",
    "axiom/theories/complete_induction.ti"
)

Write-Host "PHASE 113 - SYLVA INTERPRETER VERIFICATION" -ForegroundColor Cyan
Write-Host ""

$passed = 0
$failed = 0

Write-Host "Phase 113 Modules:" -ForegroundColor Yellow
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
    Write-Host "Phase 113 Complete" -ForegroundColor Green
} else {
    Write-Host "FAILED: $failed of $($passed + $failed) modules failed" -ForegroundColor Red
}
