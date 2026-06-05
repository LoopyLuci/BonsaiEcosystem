# verify_supervisor.ps1 — OmniCore Supervisor + stdlib + fixpoint + capabilities + Trinity
# All modules must return Result: 111. Trinity tests prove sandbox, fault recovery, fixpoint.

$exe = Join-Path $PSScriptRoot "..\..\titan-bootstrap\output\titan-compiler.exe"
if (-not (Test-Path $exe)) { Write-Error "Bootstrap not found: $exe"; exit 1 }

$modules = @(
    "titan/std/result.ti",
    "titan/std/error.ti",
    "titan/std/effects.ti",
    "titan/std/scheduler.ti",
    "titan/omnicore/capability.ti",
    "titan/omnicore/runtime_linker.ti",
    "titan/omnicore/omni_supervisor.ti",
    "titan/intrinsics/runtime.ti",
    "titan/concurrency/sync.ti",
    "titan/arch/gatekeeper.ti",
    "titan/compiler/axiom_semantic_pass.ti",
    "aether/protocol.ti",
    "aether/session_channel.ti",
    "tests/fixpoint_witness.ti"
)

$trinity = @(
    "tests/trinity_1_malicious_injection.ti",
    "tests/trinity_2_fault_recovery.ti",
    "tests/trinity_3_fixpoint_hash.ti"
)

$failed = 0

Write-Host "`n=== OmniCore Supervisor + Verified Stdlib ===" -ForegroundColor Cyan
foreach ($mod in $modules) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") { Write-Host "  PASS  $mod" -ForegroundColor Green }
    else { Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red; $failed++ }
}

Write-Host "`n=== Trinity Validation Suite ===" -ForegroundColor Magenta
foreach ($mod in $trinity) {
    $out = & $exe $mod 2>&1 | Out-String
    if ($out -match "Result:\s*111") {
        Write-Host "  PASS  $mod" -ForegroundColor Green
    } else {
        Write-Host "  FAIL  $mod  ($($out.Trim()))" -ForegroundColor Red
        $failed++
    }
}

$total = $modules.Count + $trinity.Count
Write-Host ""
if ($failed -eq 0) {
    Write-Host "RESULT: ALL $total VERIFIED — OmniCore + Trinity complete [score: 111]" -ForegroundColor Green
    Write-Host "Trinity 1: Intrinsic Sandbox holds — malicious HW_ACCESS rejected" -ForegroundColor Green
    Write-Host "Trinity 2: Fault recovery — gatekeeper catches fault, supervisor restarts" -ForegroundColor Green
    Write-Host "Trinity 3: Fixpoint identity — post-recovery binary matches golden hash" -ForegroundColor Green
    exit 0
} else {
    Write-Host "RESULT: $failed/$total FAILED" -ForegroundColor Red
    exit 1
}
