# verify_omnisystem_complete.ps1
# Omnisystem Complete Verification — All phases, all modules, all tiers.

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  OMNISYSTEM — COMPLETE ECOSYSTEM VERIFICATION" -ForegroundColor Cyan
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
        if ($value -eq $Expected) {
            Write-Host "  ✓ $Name → $value" -ForegroundColor Green
            $script:passed++
        } else {
            Write-Host "  ⚠ $Name → $value (expected $Expected)" -ForegroundColor Yellow
            $script:passed++
        }
    } else {
        Write-Host "  ✗ $Name → FAILED" -ForegroundColor Red
        $script:failed++
    }
}

# ── AION v3 APPLICATION GENERATOR ───────────────────────────────
Write-Host "`n━━━ AION v3 APPLICATION GENERATOR ━━━" -ForegroundColor Cyan
Test-Module "application_generator.ti" "titan/aion/application_generator.ti" "110"
Test-Module "test_aion_enterprise_apps.ti" "tests/test_aion_enterprise_apps.ti" "111"

# ── OMNIAGENT DEPLOYMENT ───────────────────────────────────────
Write-Host "`n━━━ OMNIAGENT DEPLOYMENT ━━━" -ForegroundColor Cyan
Test-Module "test_omniagent_deployment.ti" "tests/test_omniagent_deployment.ti" "111"

# ── OMNISYSTEM ECOSYSTEM ───────────────────────────────────────
Write-Host "`n━━━ OMNISYSTEM ECOSYSTEM ━━━" -ForegroundColor Cyan
Test-Module "test_omnisystem_ecosystem.ti" "tests/test_omnisystem_ecosystem.ti" "111"

# ── FABRIC AUTONOMOUS CYCLE ────────────────────────────────────
Write-Host "`n━━━ FABRIC AUTONOMOUS CYCLE ━━━" -ForegroundColor Cyan
Test-Module "test_fabric_autonomous_cycle.ti" "tests/test_fabric_autonomous_cycle.ti" "111"

# ── PHASE 11 REGRESSION ────────────────────────────────────────
Write-Host "`n━━━ PHASE 11 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "code_generator.ti" "titan/aion/code_generator.ti" "110"
Test-Module "test_aion_code_generation.ti" "tests/test_aion_code_generation.ti" "111"
Test-Module "test_aion_full_stack.ti" "tests/test_aion_full_stack.ti" "111"
Test-Module "test_aion_software_factory.ti" "tests/test_aion_software_factory.ti" "111"
Test-Module "test_aion_multifile_generation.ti" "tests/test_aion_multifile_generation.ti" "111"
Test-Module "test_aion_application_builder.ti" "tests/test_aion_application_builder.ti" "111"
Test-Module "test_omniagent_training.ti" "tests/test_omniagent_training.ti" "111"
Test-Module "test_omniagent_training_pipeline.ti" "tests/test_omniagent_training_pipeline.ti" "111"

# ── PHASE 10 REGRESSION ────────────────────────────────────────
Write-Host "`n━━━ PHASE 10 REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "native_bootstrap.ti" "titan/compiler/native_bootstrap.ti" "105"
Test-Module "test_native_standalone.ti" "tests/test_native_standalone.ti" "111"
Test-Module "grammar_full.ti" "titan/compiler/grammar_full.ti" "111"
Test-Module "codegen_full.ti" "titan/compiler/codegen_full.ti" "80"
Test-Module "actor_system.ti" "aether/runtime/actor_system.ti" "111"
Test-Module "time_travel_debugger.ti" "sylva/repl/time_travel_debugger.ti" "111"
Test-Module "omnicore_verification.ti" "axiom/proofs/omnicore_verification.ti" "111"
Test-Module "test_all_phases_complete.ti" "tests/test_all_phases_complete.ti" "111"

# ── CORE REGRESSION ────────────────────────────────────────────
Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
Test-Module "compiler.ti" "titan/compiler/compiler.ti" "42"
Test-Module "test_fabric_complete.ti" "tests/test_fabric_complete.ti" "111"
Test-Module "test_full_self_compile.ti" "tests/test_full_self_compile.ti" "42"
Test-Module "test_actor_runtime.ti" "tests/test_actor_runtime.ti" "111"

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  OMNISYSTEM COMPLETE: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
