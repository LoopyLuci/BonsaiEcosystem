# scripts/verification/verify_omnievent.ps1
# Phase 26: OmniEvent — Complete Verification Script

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  OMNIEVENT --- COMPLETE VERIFICATION" -ForegroundColor Cyan
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

Write-Host "`n--- OmniEvent Core Modules ---" -ForegroundColor Yellow
Test-Module "event_config.ti" "$root/titan/omnievent/event_config.ti" "111"
Test-Module "schema_registry.ti" "$root/titan/omnievent/schema_registry.ti" "111"

Write-Host "`n--- OmniEvent Orchestration ---" -ForegroundColor Yellow
Test-Module "event_broker.ae" "$root/aether/omnievent/event_broker.ae" "111"
Test-Module "dead_letter_handler.ae" "$root/aether/omnievent/dead_letter_handler.ae" "111"

Write-Host "`n--- OmniEvent UI & Monitoring ---" -ForegroundColor Yellow
Test-Module "event_explorer.sy" "$root/sylva/omnievent/event_explorer.sy" "111"
Test-Module "event_monitor.sy" "$root/sylva/omnievent/event_monitor.sy" "111"

Write-Host "`n--- OmniEvent Verification ---" -ForegroundColor Yellow
Test-Module "event_proofs.ax" "$root/axiom/omnievent/event_proofs.ax" "111"

Write-Host "`n--- Integration Test ---" -ForegroundColor Yellow
Test-Module "test_omnievent_complete.ti" "$root/tests/test_omnievent_complete.ti" "111"

Write-Host "`n--- Regression Tests ---" -ForegroundColor Yellow
Test-Module "auth_config.ti" "$root/titan/omniauth/auth_config.ti" "111"
Test-Module "test_omniauth_complete.ti" "$root/tests/test_omniauth_complete.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  OMNIEVENT: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -gt 0) { exit 1 }
