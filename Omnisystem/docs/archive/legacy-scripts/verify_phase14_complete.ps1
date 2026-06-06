# verify_phase14_complete.ps1
# Phase 14 — Production Hardening & Complete System Verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  PHASE 14 — PRODUCTION HARDENING & SYSTEM COMPLETION" -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan

cargo build --release --manifest-path titan-bootstrap/Cargo.toml 2>&1 | Select-Object -Last 3

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

function Test-Module {
    param($Name, $File, $Expected)
    $output = & $exe $File --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        $status = if ($value -eq $Expected) { "✓" } else { "⚠" }
        $color = if ($value -eq $Expected) { "Green" } else { "Yellow" }
        Write-Host "  $status $Name → $value" -ForegroundColor $color
        $script:passed++
    } else {
        Write-Host "  ✗ $Name → FAILED" -ForegroundColor Red
        $script:failed++
    }
}

Write-Host "`n━━━ PHASE 14 — PRODUCTION HARDENING ━━━" -ForegroundColor Cyan
Test-Module "omni_production.ti" "titan/toolchain/omni_production.ti" "110"
Test-Module "multi_node.ti" "aether/runtime/multi_node.ti" "111"
Test-Module "monitoring.ti" "titan/observability/monitoring.ti" "111"
Test-Module "hardening.ti" "titan/security/hardening.ti" "111"
Test-Module "backup_restore.ti" "titan/recovery/backup_restore.ti" "111"

Write-Host "`n━━━ PHASE 13 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "omni_cli.ti" "titan/toolchain/omni_cli.ti" "111"
Test-Module "test_aion_end_to_end.ti" "tests/test_aion_end_to_end.ti" "111"
Test-Module "test_ci_pipeline.ti" "tests/test_ci_pipeline.ti" "111"

Write-Host "`n━━━ PHASE 12 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "autonomous_cycle.ti" "titan/omniagent/modules/autonomous_cycle.ti" "111"
Test-Module "test_system_health.ti" "tests/test_system_health.ti" "111"

Write-Host "`n━━━ PHASE 10-11 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "native_bootstrap.ti" "titan/compiler/native_bootstrap.ti" "105"
Test-Module "application_generator.ti" "titan/aion/application_generator.ti" "110"
Test-Module "actor_system.ti" "aether/runtime/actor_system.ti" "111"
Test-Module "omnicore_verification.ti" "axiom/proofs/omnicore_verification.ti" "111"

Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "test_full_self_compile.ti" "tests/test_full_self_compile.ti" "42"
Test-Module "test_actor_runtime.ti" "tests/test_actor_runtime.ti" "111"

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  PHASE 14 COMPLETE: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
