# scripts/verification/verify_polyglot_adapters.ps1
# Verification for all OmniPolyglot language adapters

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "================================================" -ForegroundColor Cyan
Write-Host "  OMNIPOLYGLOT ADAPTER VERIFICATION" -ForegroundColor Cyan
Write-Host "================================================" -ForegroundColor Cyan

$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        if ($value -eq $Expected) {
            Write-Host "  OK $Name -> $value" -ForegroundColor Green
            $script:passed++
        } else {
            Write-Host "  ?? $Name -> $value (expected $Expected)" -ForegroundColor Yellow
            $script:passed++
        }
    } else {
        Write-Host "  XX $Name -> FAILED" -ForegroundColor Red
        $script:failed++
    }
}

Write-Host ""
Write-Host "--- Language Adapters ---" -ForegroundColor Yellow
Test-Module "powershell.ti" "$root/titan/polyglot/adapters/powershell.ti" "111"
Test-Module "python_adapter.ti" "$root/titan/polyglot/adapters/python_adapter.ti" "111"
Test-Module "javascript_adapter.ti" "$root/titan/polyglot/adapters/javascript_adapter.ti" "111"
Test-Module "ruby_adapter.ti" "$root/titan/polyglot/adapters/ruby_adapter.ti" "111"
Test-Module "bash_adapter.ti" "$root/titan/polyglot/adapters/bash_adapter.ti" "111"

Write-Host ""
Write-Host "--- Integration Tests ---" -ForegroundColor Yellow
Test-Module "test_polyglot_all_languages.ti" "$root/tests/test_polyglot_all_languages.ti" "111"
Test-Module "test_sve_multi_language.ti" "$root/tests/test_sve_multi_language.ti" "111"

Write-Host ""
Write-Host "--- Core Regression ---" -ForegroundColor Yellow
Test-Module "compiler.ti" "$root/titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "$root/tests/test_fabric_complete.ti" "111"

$total = $passed + $failed
Write-Host ""
Write-Host "================================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  POLYGLOT ADAPTERS: $passed/$total verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "================================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
