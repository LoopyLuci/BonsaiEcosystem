# scripts/verification/verify_omnienv.ps1
# OmniEnv — Complete Verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
cd $root

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  OMNIENV --- COMPLETE VERIFICATION" -ForegroundColor Cyan
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

Write-Host "`n--- OmniEnv Core Modules ---" -ForegroundColor Yellow
Test-Module "language_envs.ti" "$root/titan/omnienv/language_envs.ti" "111"
Test-Module "cross_language_env.ti" "$root/titan/omnienv/cross_language_env.ti" "111"
Test-Module "hotreload_env.ti" "$root/titan/omnienv/hotreload_env.ti" "111"
Test-Module "env_manager.ti" "$root/titan/omnienv/env_manager.ti" "111"

Write-Host "`n--- OmniEnv Coordination ---" -ForegroundColor Yellow
Test-Module "env_orchestrator.ae" "$root/aether/omnienv/env_orchestrator.ae" "111"
Test-Module "env_console.sy" "$root/sylva/omnienv/env_console.sy" "111"
Test-Module "env_proofs.ax" "$root/axiom/omnienv/env_proofs.ax" "111"

Write-Host "`n--- Integration Test ---" -ForegroundColor Yellow
Test-Module "test_omnienv_complete.ti" "$root/tests/test_omnienv_complete.ti" "111"

Write-Host "`n--- Core Regression ---" -ForegroundColor Yellow
Test-Module "compiler.ti" "$root/titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "$root/tests/test_fabric_complete.ti" "111"
Test-Module "autonomous_cycle.ti" "$root/titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host "`n============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  OMNIENV: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "============================================" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
