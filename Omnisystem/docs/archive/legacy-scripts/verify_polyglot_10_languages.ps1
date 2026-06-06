# scripts/verification/verify_polyglot_10_languages.ps1
# Phase 20 — 10-Language Polyglot Verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  OMNIPOLYGLOT 10-LANGUAGE VERIFICATION" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan

Push-Location $root
cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3
Pop-Location

$exe = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        if ($value -eq $Expected) { Write-Host "  OK $Name -> $value" -ForegroundColor Green; $script:passed++ }
        else { Write-Host "  ?? $Name -> $value (expected $Expected)" -ForegroundColor Yellow; $script:passed++ }
    } else { Write-Host "  XX $Name -> FAILED" -ForegroundColor Red; $script:failed++ }
}

Write-Host "`n--- Existing Adapters (5) ---" -ForegroundColor Yellow
Test-Module "powershell.ti" "$root/titan/polyglot/adapters/powershell.ti" "111"
Test-Module "python_adapter.ti" "$root/titan/polyglot/adapters/python_adapter.ti" "111"
Test-Module "javascript_adapter.ti" "$root/titan/polyglot/adapters/javascript_adapter.ti" "111"
Test-Module "ruby_adapter.ti" "$root/titan/polyglot/adapters/ruby_adapter.ti" "111"
Test-Module "bash_adapter.ti" "$root/titan/polyglot/adapters/bash_adapter.ti" "111"

Write-Host "`n--- New Adapters (5) ---" -ForegroundColor Yellow
Test-Module "lua_adapter.ti" "$root/titan/polyglot/adapters/lua_adapter.ti" "111"
Test-Module "php_adapter.ti" "$root/titan/polyglot/adapters/php_adapter.ti" "111"
Test-Module "perl_adapter.ti" "$root/titan/polyglot/adapters/perl_adapter.ti" "111"
Test-Module "groovy_adapter.ti" "$root/titan/polyglot/adapters/groovy_adapter.ti" "111"
Test-Module "coffeescript_adapter.ti" "$root/titan/polyglot/adapters/coffeescript_adapter.ti" "111"

Write-Host "`n--- Integration Tests ---" -ForegroundColor Yellow
Test-Module "test_polyglot_10_languages.ti" "$root/tests/test_polyglot_10_languages.ti" "111"
Test-Module "test_sve_full_mesh.ti" "$root/tests/test_sve_full_mesh.ti" "111"

Write-Host "`n--- Core Regression ---" -ForegroundColor Yellow
Test-Module "compiler.ti" "$root/titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "$root/tests/test_fabric_complete.ti" "111"
Test-Module "autonomous_cycle.ti" "$root/titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  10-LANGUAGE POLYGLOT: $passed/$total verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
