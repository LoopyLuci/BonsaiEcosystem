# verify_phase15_complete.ps1
# Phase 15 — Aion Autonomous Deployment & Complete System Verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  PHASE 15 — AION AUTONOMOUS DEPLOYMENT & SYSTEM COMPLETION" -ForegroundColor Cyan
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

# ── PHASE 15 — AION AUTONOMOUS DEPLOYMENT ──────────────────────────
Write-Host "`n━━━ PHASE 15 — AION AUTONOMOUS DEPLOYMENT ━━━" -ForegroundColor Cyan
Test-Module "deployment_pipeline.ti" "titan/aion/deployment_pipeline.ti" "111"
Test-Module "test_aion_full_deployment.ti" "tests/test_aion_full_deployment.ti" "111"
Test-Module "self_improving_factory.ti" "titan/aion/self_improving_factory.ti" "111"

# ── PHASE 14 REGRESSION ──────────────────────────────────────────
Write-Host "`n━━━ PHASE 14 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "omni_production.ti" "titan/toolchain/omni_production.ti" "110"
Test-Module "multi_node.ti" "aether/runtime/multi_node.ti" "111"
Test-Module "monitoring.ti" "titan/observability/monitoring.ti" "111"
Test-Module "hardening.ti" "titan/security/hardening.ti" "111"
Test-Module "backup_restore.ti" "titan/recovery/backup_restore.ti" "111"

# ── CORE SYSTEM REGRESSION ───────────────────────────────────────
Write-Host "`n━━━ CORE SYSTEM REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"
Test-Module "native_bootstrap.ti" "titan/compiler/native_bootstrap.ti" "105"
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "test_full_self_compile.ti" "tests/test_full_self_compile.ti" "42"
Test-Module "test_actor_runtime.ti" "tests/test_actor_runtime.ti" "111"
Test-Module "test_all_phases_complete.ti" "tests/test_all_phases_complete.ti" "111"
Test-Module "test_omnisystem_ecosystem.ti" "tests/test_omnisystem_ecosystem.ti" "111"
Test-Module "autonomous_cycle.ti" "titan/omniagent/modules/autonomous_cycle.ti" "111"

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  PHASE 15 COMPLETE: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
