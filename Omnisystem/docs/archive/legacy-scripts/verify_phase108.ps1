# verify_phase108.ps1 — Phase 108: Axiom Inductive Types & Tactic Engine

$root = Resolve-Path "$PSScriptRoot\..\.."
$exe  = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"

$score  = 0
$passed = 0
$failed = 0

function Run-Ti {
    param([string]$file, [string]$label)
    if (-not (Test-Path $file)) {
        Write-Host "  MISSING  $label"
        $script:failed++
        return
    }
    $out = & $exe $file --run 2>&1
    if ($out -match "Result:\s*111") {
        Write-Host "  PASS     $label"
        $script:passed++
        $script:score += 37
    } else {
        Write-Host "  FAIL     $label  ($out)"
        $script:failed++
    }
}

Write-Host "=== Phase 108: Axiom Inductive Types & Tactic Engine ==="
Write-Host ""

# Phase 108 files
Run-Ti "$root\titan\kernel\inductive.ti"               "titan/kernel/inductive.ti"
Run-Ti "$root\axiom\tactics\tactic_core.ti"            "axiom/tactics/tactic_core.ti"
Run-Ti "$root\tests\test_nat_commutativity.ti"         "tests/test_nat_commutativity.ti"

# Phase 107 regression check
Write-Host ""
Write-Host "--- Phase 107 regression ---"
Run-Ti "$root\titan\kernel\heap.ti"                    "titan/kernel/heap.ti"
Run-Ti "$root\titan\kernel\term.ti"                    "titan/kernel/term.ti"
Run-Ti "$root\titan\kernel\normalize.ti"               "titan/kernel/normalize.ti"
Run-Ti "$root\titan\kernel\equality.ti"                "titan/kernel/equality.ti"
Run-Ti "$root\titan\kernel\typechecker.ti"             "titan/kernel/typechecker.ti"
Run-Ti "$root\tests\test_kernel_real.ti"               "tests/test_kernel_real.ti"

Write-Host ""
Write-Host "Passed: $passed   Failed: $failed"

if ($failed -eq 0) {
    Write-Host ""
    Write-Host "RESULT: 111"
    exit 0
} else {
    Write-Host ""
    Write-Host "RESULT: FAIL"
    exit 1
}
