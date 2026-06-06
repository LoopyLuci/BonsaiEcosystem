# verify_phase121.ps1 — Phase 121: Bootstrap Hardening + Crypto Suite

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

Write-Host "=== Phase 121: Bootstrap Hardening + Crypto Suite ==="
Write-Host ""
Write-Host "--- Phase 121: Crypto modules (5) ---"
Run-Ti "$root\axiom\crypto\fnv_hash.ti"    "axiom/crypto/fnv_hash.ti"
Run-Ti "$root\axiom\crypto\xor_cipher.ti"  "axiom/crypto/xor_cipher.ti"
Run-Ti "$root\axiom\crypto\checksum.ti"    "axiom/crypto/checksum.ti"
Run-Ti "$root\axiom\crypto\bit_permute.ti" "axiom/crypto/bit_permute.ti"
Run-Ti "$root\axiom\crypto\merkle_hash.ti" "axiom/crypto/merkle_hash.ti"

Write-Host ""
Write-Host "--- Phase 111 regression (2) ---"
Run-Ti "$root\axiom\theories\nat_induction_proofs.ti"  "axiom/theories/nat_induction_proofs.ti"
Run-Ti "$root\tests\test_induction_tactic.ti"          "tests/test_induction_tactic.ti"

Write-Host ""
Write-Host "--- Phase 110 regression (2) ---"
Run-Ti "$root\axiom\theories\nat_tactics_proofs.ti"    "axiom/theories/nat_tactics_proofs.ti"
Run-Ti "$root\tests\test_tactic_proofs.ti"             "tests/test_tactic_proofs.ti"

Write-Host ""
Write-Host "--- Phase 109 regression (2) ---"
Run-Ti "$root\axiom\theories\nat_stdlib.ti"            "axiom/theories/nat_stdlib.ti"
Run-Ti "$root\tests\test_nat_proofs.ti"                "tests/test_nat_proofs.ti"

Write-Host ""
Write-Host "--- Phase 108 regression (1) ---"
Run-Ti "$root\titan\kernel\inductive.ti"               "titan/kernel/inductive.ti"

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
