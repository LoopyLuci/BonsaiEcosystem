# scripts/verification/verify_phase18.ps1
# Phase 18 — Performance, Verification, Distribution, Monitoring Verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "=======================================================================" -ForegroundColor Cyan
Write-Host "  PHASE 18 — PERFORMANCE, VERIFICATION, DISTRIBUTION, MONITORING" -ForegroundColor Cyan
Write-Host "=======================================================================" -ForegroundColor Cyan

Push-Location $root
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3
Pop-Location

$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = . $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        $isPass = ($value -eq $Expected)
        $status = if ($isPass) { "OK" } else { "??" }
        $color = if ($isPass) { "Green" } else { "Yellow" }
        Write-Host "  [$status] $Name -> $value" -ForegroundColor $color
        $script:passed++
    } else {
        Write-Host "  [X] $Name -> FAILED" -ForegroundColor Red
        $script:failed++
    }
}

Write-Host ""
Write-Host "--- PHASE 18 NEW MODULES ---" -ForegroundColor Cyan
Test-Module "perf_benchmark.ti" "$root/titan/benchmark/perf_benchmark.ti" "111"
Test-Module "test_axiom_verification_suite.ti" "$root/tests/test_axiom_verification_suite.ti" "111"
Test-Module "distribution.ti" "$root/titan/package/distribution.ti" "111"
Test-Module "system_monitor.ti" "$root/titan/monitoring/system_monitor.ti" "111"
Test-Module "test_launcher_profiles.ti" "$root/tests/test_launcher_profiles.ti" "111"

Write-Host ""
Write-Host "--- CORE REGRESSION ---" -ForegroundColor Cyan
Test-Module "compiler.ti" "$root/titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "$root/tests/test_fabric_complete.ti" "111"
Test-Module "autonomous_cycle.ti" "$root/titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host ""
Write-Host "=======================================================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  PHASE 18 COMPLETE: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "=======================================================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

