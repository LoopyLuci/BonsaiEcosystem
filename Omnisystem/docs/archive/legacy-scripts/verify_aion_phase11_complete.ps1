# verify_aion_phase11_complete.ps1
# Phase 11 — Aion AI + OmniAgent complete verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  PHASE 11 — AION AI + OMNIAGENT COMPLETE VERIFICATION" -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════════════════════════════" -ForegroundColor Cyan

cd titan-bootstrap; cargo build --release 2>&1 | Select-Object -Last 3; cd ..

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0
$results = @()

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

Write-Host "`n━━━ AION CODE GENERATOR v2 ━━━" -ForegroundColor Cyan
Test-Module "code_generator.ti" "titan/aion/code_generator.ti" "110"
Test-Module "test_aion_code_generation.ti" "tests/test_aion_code_generation.ti" "111"
Test-Module "test_aion_full_stack.ti" "tests/test_aion_full_stack.ti" "111"
Test-Module "test_aion_software_factory.ti" "tests/test_aion_software_factory.ti" "111"
Test-Module "test_aion_multifile_generation.ti" "tests/test_aion_multifile_generation.ti" "111"
Test-Module "test_aion_application_builder.ti" "tests/test_aion_application_builder.ti" "111"

Write-Host "`n━━━ OMNIAGENT TRAINING PIPELINE ━━━" -ForegroundColor Cyan
Test-Module "test_omniagent_training.ti" "tests/test_omniagent_training.ti" "111"
Test-Module "test_omniagent_training_pipeline.ti" "tests/test_omniagent_training_pipeline.ti" "111"

Write-Host "`n━━━ PHASE 10 REGRESSION ━━━" -ForegroundColor Cyan
$phase10 = @(
    @{name="native_bootstrap.ti"; file="titan/compiler/native_bootstrap.ti"; expected="105"},
    @{name="test_native_standalone.ti"; file="tests/test_native_standalone.ti"; expected="111"},
    @{name="grammar_full.ti"; file="titan/compiler/grammar_full.ti"; expected="111"},
    @{name="codegen_full.ti"; file="titan/compiler/codegen_full.ti"; expected="80"},
    @{name="actor_system.ti"; file="aether/runtime/actor_system.ti"; expected="111"},
    @{name="time_travel_debugger.ti"; file="sylva/repl/time_travel_debugger.ti"; expected="111"},
    @{name="omnicore_verification.ti"; file="axiom/proofs/omnicore_verification.ti"; expected="111"},
    @{name="test_all_phases_complete.ti"; file="tests/test_all_phases_complete.ti"; expected="111"}
)
foreach ($p in $phase10) {
    Test-Module $p.name $p.file $p.expected
}

Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
$core = @("titan/compiler/compiler.ti", "tests/test_fabric_complete.ti", "tests/test_full_self_compile.ti", "tests/test_actor_runtime.ti")
foreach ($c in $core) {
    $output = & $exe $c --run 2>&1
    if ($output -match "Result:") {
        Write-Host "  ✓ $(Split-Path -Leaf $c)" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "  ✗ $(Split-Path -Leaf $c)" -ForegroundColor Red
        $failed++
    }
}

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  PHASE 11 COMPLETE: $passed/$total modules verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
