# Phase 112 Verification Script
# Tests complete induction proofs implementation

$modules = @(
    "axiom/theories/complete_induction.ti",
    "tests/test_complete_induction.ti"
)

$regression = @(
    "titan/kernel/inductive.ti",
    "axiom/theories/nat_induction_proofs.ti"
)

$all = $modules + $regression
$passed = 0
$failed = 0

Write-Host "Phase 112 - Complete Induction Proofs" -ForegroundColor Cyan
Write-Host ""

foreach ($mod in $all) {
    if (Test-Path $mod) {
        $result = & .\titan-bootstrap\target\release\titan-bootstrap.exe $mod --run 2>&1
        
        if ($result -match "Result: 111") {
            Write-Host "PASS: $mod" -ForegroundColor Green
            $passed++
        }
        else {
            Write-Host "FAIL: $mod" -ForegroundColor Red
            $failed++
        }
    }
}

Write-Host ""
Write-Host "Total: $passed passed, $failed failed" -ForegroundColor Cyan

if ($failed -eq 0) {
    Write-Host "All modules verified" -ForegroundColor Green
    exit 0
}
else {
    Write-Host "Some modules failed" -ForegroundColor Red
    exit 1
}
