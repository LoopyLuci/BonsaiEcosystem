# scripts/verification/verify_omnideploy.ps1
# Phase 23: OmniDeploy — Complete Verification Script

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  OMNIDEPLOY --- COMPLETE VERIFICATION" -ForegroundColor Cyan
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

Write-Host "`n--- OmniDeploy Core Modules ---" -ForegroundColor Yellow
Test-Module "deployment_config.ti" "$root/titan/omnideploy/deployment_config.ti" "111"
Test-Module "multi_cloud.ti" "$root/titan/omnideploy/multi_cloud.ti" "111"
Test-Module "canary_rollout.ti" "$root/titan/omnideploy/canary_rollout.ti" "111"

Write-Host "`n--- OmniDeploy Orchestration ---" -ForegroundColor Yellow
Test-Module "deploy_orchestrator.ae" "$root/aether/omnideploy/deploy_orchestrator.ae" "111"
Test-Module "rollback_manager.ae" "$root/aether/omnideploy/rollback_manager.ae" "111"
Test-Module "deploy_dashboard.sy" "$root/sylva/omnideploy/deploy_dashboard.sy" "111"
Test-Module "deploy_proofs.ax" "$root/axiom/omnideploy/deploy_proofs.ax" "111"

Write-Host "`n--- Integration Test ---" -ForegroundColor Yellow
Test-Module "test_omnideploy_complete.ti" "$root/tests/test_omnideploy_complete.ti" "111"

Write-Host "`n--- Regression Tests ---" -ForegroundColor Yellow
Test-Module "language_envs.ti" "$root/titan/omnienv/language_envs.ti" "111"
Test-Module "test_omnienv_complete.ti" "$root/tests/test_omnienv_complete.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  OMNIDEPLOY: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })

if ($failed -gt 0) { exit 1 }
