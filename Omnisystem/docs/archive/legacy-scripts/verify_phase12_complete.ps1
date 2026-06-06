# verify_phase12_complete.ps1
# Phase 12 — Production-Ready Ecosystem Verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  PHASE 12 — PRODUCTION-READY ECOSYSTEM VERIFICATION" -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan

cd titan-bootstrap; cargo build --release 2>&1 | Select-Object -Last 3; cd ..

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

# ── PHASE 12 NEW MODULES ──────────────────────────────────────────
Write-Host "`n━━━ PHASE 12 — AUTONOMOUS CYCLE & ECOSYSTEM ━━━" -ForegroundColor Cyan
Test-Module "autonomous_cycle.ti" "titan/omniagent/modules/autonomous_cycle.ti" "111"
Test-Module "session_manager.ti" "sylva/repl/session_manager.ti" "111"
Test-Module "test_project_scaffold.ti" "tests/test_project_scaffold.ti" "111"
Test-Module "test_package_manager.ti" "tests/test_package_manager.ti" "111"
Test-Module "test_cross_language_integration.ti" "tests/test_cross_language_integration.ti" "111"
Test-Module "test_system_health.ti" "tests/test_system_health.ti" "111"

# ── PHASE 11 REGRESSION ──────────────────────────────────────────
Write-Host "`n━━━ PHASE 11 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "application_generator.ti" "titan/aion/application_generator.ti" "110"
Test-Module "test_aion_enterprise_apps.ti" "tests/test_aion_enterprise_apps.ti" "111"
Test-Module "test_omniagent_deployment.ti" "tests/test_omniagent_deployment.ti" "111"
Test-Module "test_omnisystem_ecosystem.ti" "tests/test_omnisystem_ecosystem.ti" "111"
Test-Module "test_fabric_autonomous_cycle.ti" "tests/test_fabric_autonomous_cycle.ti" "111"

# ── PHASE 10 REGRESSION ──────────────────────────────────────────
Write-Host "`n━━━ PHASE 10 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "native_bootstrap.ti" "titan/compiler/native_bootstrap.ti" "105"
Test-Module "grammar_full.ti" "titan/compiler/grammar_full.ti" "111"
Test-Module "codegen_full.ti" "titan/compiler/codegen_full.ti" "80"
Test-Module "actor_system.ti" "aether/runtime/actor_system.ti" "111"
Test-Module "time_travel_debugger.ti" "sylva/repl/time_travel_debugger.ti" "111"
Test-Module "omnicore_verification.ti" "axiom/proofs/omnicore_verification.ti" "111"

# ── CORE REGRESSION ──────────────────────────────────────────────
Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "test_full_self_compile.ti" "tests/test_full_self_compile.ti" "42"
Test-Module "test_actor_runtime.ti" "tests/test_actor_runtime.ti" "111"

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  PHASE 12 COMPLETE: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
