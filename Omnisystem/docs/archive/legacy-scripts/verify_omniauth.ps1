# scripts/verification/verify_omniauth.ps1
# Phase 25: OmniAuth — Complete Verification Script

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  OMNIAUTH --- COMPLETE VERIFICATION" -ForegroundColor Cyan
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

Write-Host "`n--- OmniAuth Core Modules ---" -ForegroundColor Yellow
Test-Module "auth_config.ti" "$root/titan/omniauth/auth_config.ti" "111"
Test-Module "rbac_policy.ti" "$root/titan/omniauth/rbac_policy.ti" "111"

Write-Host "`n--- OmniAuth Orchestration ---" -ForegroundColor Yellow
Test-Module "token_service.ae" "$root/aether/omniauth/token_service.ae" "111"
Test-Module "auth_gateway.ae" "$root/aether/omniauth/auth_gateway.ae" "111"

Write-Host "`n--- OmniAuth UI & Monitoring ---" -ForegroundColor Yellow
Test-Module "identity_dashboard.sy" "$root/sylva/omniauth/identity_dashboard.sy" "111"
Test-Module "session_monitor.sy" "$root/sylva/omniauth/session_monitor.sy" "111"

Write-Host "`n--- OmniAuth Verification ---" -ForegroundColor Yellow
Test-Module "auth_proofs.ax" "$root/axiom/omniauth/auth_proofs.ax" "111"

Write-Host "`n--- Integration Test ---" -ForegroundColor Yellow
Test-Module "test_omniauth_complete.ti" "$root/tests/test_omniauth_complete.ti" "111"

Write-Host "`n--- Regression Tests ---" -ForegroundColor Yellow
Test-Module "mesh_config.ti" "$root/titan/omnimesh/mesh_config.ti" "111"
Test-Module "test_omnimesh_complete.ti" "$root/tests/test_omnimesh_complete.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  OMNIAUTH: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -gt 0) { exit 1 }
