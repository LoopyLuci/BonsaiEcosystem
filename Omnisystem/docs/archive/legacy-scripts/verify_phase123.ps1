# verify_phase123.ps1 — Phase 123: OmniLib Concurrency & Parallelism Primitives

$root = Resolve-Path "$PSScriptRoot\..\.."
$exe  = "$root\titan-bootstrap\target\release\titan-bootstrap.exe"

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
    } else {
        Write-Host "  FAIL     $label  ($out)"
        $script:failed++
    }
}

Write-Host "=== Phase 123: OmniLib Concurrency & Parallelism Primitives ==="
Write-Host ""
Write-Host "--- Phase 123: New modules (5) ---"
Run-Ti "$root\titan\std\channel.ti"                   "titan/std/channel.ti"
Run-Ti "$root\titan\std\mutex.ti"                     "titan/std/mutex.ti"
Run-Ti "$root\titan\std\atomic.ti"                    "titan/std/atomic.ti"
Run-Ti "$root\titan\std\thread.ti"                    "titan/std/thread.ti"
Run-Ti "$root\tests\test_omni_lib_concurrency.ti"     "tests/test_omni_lib_concurrency.ti"

Write-Host ""
Write-Host "--- Phase 122 regression (5) ---"
Run-Ti "$root\axiom\control\match_dispatch.ti" "axiom/control/match_dispatch.ti"
Run-Ti "$root\axiom\control\range_loops.ti"    "axiom/control/range_loops.ti"
Run-Ti "$root\axiom\control\break_continue.ti" "axiom/control/break_continue.ti"
Run-Ti "$root\axiom\data\fibonacci_iter.ti"    "axiom/data/fibonacci_iter.ti"
Run-Ti "$root\axiom\data\prime_count.ti"       "axiom/data/prime_count.ti"

Write-Host ""
Write-Host "--- Phase 121 regression (5) ---"
Run-Ti "$root\axiom\crypto\fnv_hash.ti"    "axiom/crypto/fnv_hash.ti"
Run-Ti "$root\axiom\crypto\xor_cipher.ti"  "axiom/crypto/xor_cipher.ti"
Run-Ti "$root\axiom\crypto\checksum.ti"    "axiom/crypto/checksum.ti"
Run-Ti "$root\axiom\crypto\bit_permute.ti" "axiom/crypto/bit_permute.ti"
Run-Ti "$root\axiom\crypto\merkle_hash.ti" "axiom/crypto/merkle_hash.ti"

Write-Host ""
Write-Host "--- Phase 111 regression (2) ---"
Run-Ti "$root\axiom\theories\nat_induction_proofs.ti" "axiom/theories/nat_induction_proofs.ti"
Run-Ti "$root\tests\test_induction_tactic.ti"         "tests/test_induction_tactic.ti"

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
