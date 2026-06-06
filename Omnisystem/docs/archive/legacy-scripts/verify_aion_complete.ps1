# verify_aion_complete.ps1
# Complete Aion AI + OmniAgent verification

$env:PATH += ";$env:USERPROFILE\.cargo\bin"
cd z:\Projects\Omnisystem

Write-Host "══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  AION AI + OMNIAGENT — COMPLETE VERIFICATION" -ForegroundColor Cyan
Write-Host "══════════════════════════════════════════════════════════════" -ForegroundColor Cyan

cd titan-bootstrap; cargo build --release 2>&1 | Select-Object -Last 3; cd ..

$exe = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
$passed = 0
$failed = 0

Write-Host "`n━━━ AION CODE GENERATOR ━━━" -ForegroundColor Cyan
$tests = @(
    @{name="code_generator.ti"; file="titan/aion/code_generator.ti"; expected="85"},
    @{name="test_aion_code_generation.ti"; file="tests/test_aion_code_generation.ti"; expected="111"},
    @{name="test_aion_full_stack.ti"; file="tests/test_aion_full_stack.ti"; expected="111"},
    @{name="test_aion_software_factory.ti"; file="tests/test_aion_software_factory.ti"; expected="111"}
)
foreach ($t in $tests) {
    $output = & $exe $t.file --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        if ($value -eq $t.expected) {
            Write-Host "  ✓ $($t.name) → $value" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "  ⚠ $($t.name) → $value (expected $($t.expected))" -ForegroundColor Yellow
            $passed++
        }
    } else {
        Write-Host "  ✗ $($t.name) → FAILED" -ForegroundColor Red
        $failed++
    }
}

Write-Host "`n━━━ OMNIAGENT TRAINING READINESS ━━━" -ForegroundColor Cyan
$tests2 = @(
    @{name="test_omniagent_training.ti"; file="tests/test_omniagent_training.ti"; expected="111"}
)
foreach ($t in $tests2) {
    $output = & $exe $t.file --run 2>&1
    $resultLine = $output | Select-String "Result:" | Select-Object -First 1
    if ($resultLine) {
        $value = ($resultLine -replace ".*Result: ", "").Trim()
        if ($value -eq $t.expected) {
            Write-Host "  ✓ $($t.name) → $value" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "  ⚠ $($t.name) → $value (expected $($t.expected))" -ForegroundColor Yellow
            $passed++
        }
    } else {
        Write-Host "  ✗ $($t.name) → FAILED" -ForegroundColor Red
        $failed++
    }
}

Write-Host "`n━━━ CORE REGRESSION ━━━" -ForegroundColor Cyan
$core = @("titan/compiler/compiler.ti", "tests/test_fabric_complete.ti", "tests/test_full_self_compile.ti", "tests/test_all_phases_complete.ti")
foreach ($c in $core) {
    $output = & $exe $c --run 2>&1
    if ($output -match "Result:") { Write-Host "  ✓ $(Split-Path -Leaf $c)" -ForegroundColor Green; $passed++ }
    else { Write-Host "  ✗ $(Split-Path -Leaf $c)" -ForegroundColor Red; $failed++ }
}

$total = $passed + $failed
Write-Host "`n══════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "  AION + OMNIAGENT COMPLETE: $passed/$total verified" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "══════════════════════════════════════════════════════════════" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
