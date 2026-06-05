# scripts/test-all.ps1 — Compile and run all Omnisystem tests
# Usage: .\scripts\test-all.ps1
# Exit 0 = all pass, 1 = one or more failures

param([string]$Compiler = ".\titan-bootstrap\output\titan-compiler.exe")

Set-Location (Split-Path $PSScriptRoot -Parent)

if (-not (Test-Path $Compiler)) {
    Write-Error "Compiler not found: $Compiler"
    exit 1
}

$sources = [ordered]@{
    "Kernel" = @(
        "kernel\capability.ti",
        "kernel\memory.ti",
        "kernel\scheduler.ti",
        "kernel\boot_integration.ti"
    )
    "Services" = @(
        "services\p2p\p2p.ti",
        "services\compress\compress.ti",
        "services\container\container.ti",
        "services\observability\observability.ti",
        "services\storage\storage.ti",
        "services\cache\cache.ti",
        "services\queue\queue.ti",
        "services\rpc\rpc.ti",
        "services\auth\auth.ti",
        "services\crypto\crypto.ti"
    )
    "Axiom Proofs" = @(
        "titan\axlib\ax6_kernel.ti",
        "titan\axlib\ax7_services.ti"
    )
    "Aether" = @(
        "aether\crdt.ti",
        "aether\actor.ti",
        "aether\mesh.ti",
        "aether\crdt_map.ti",
        "aether\transport_socket_bridge.ti"
    )
    "Sylva" = @(
        "sylva\interpreter.ti",
        "sylva\compiler.ti",
        "sylva\jit.ti"
    )
    "Effect System" = @(
        "effect\perform.ti",
        "effect\socket_io.ti",
        "titan\std\effect_handlers.ti"
    )
    "Build Infrastructure" = @(
        "build\ir\ir.ti"
    )
    "Compiler Verification" = @(
        "titan\compiler\self_host_verify.ti",
        "titan\compiler\gpu_codegen.ti",
        "titan\compiler\dispatch_target.ti"
    )
    "Gap Closures (Phase 5)" = @(
        "aether\transport_p2p.ti",
        "kernel\boot_x86_64.ti",
        "sylva\compiler_strict.ti",
        "titan\compiler\gpu_backend.ti",
        "axiom\smt_solver.ti",
        "build\stage3.ti",
        "vm\frontend_registry.ti"
    )
    "Universal Validation Mesh" = @(
        "uvm\scheduler.ti",
        "uvm\agent.ti",
        "uvm\chaos.ti",
        "uvm\fuzz.ti",
        "aether\simulation.ti",
        "build\build.ti",
        "titan\axlib\ax8_services2.ti"
    )
}

$passed = 0; $failed = 0; $errors = @()

Write-Host ""
Write-Host "Omnisystem Test Suite" -ForegroundColor Cyan
Write-Host "Compiler: $Compiler" -ForegroundColor DarkGray

foreach ($category in $sources.Keys) {
    Write-Host ""
    Write-Host "  $category" -ForegroundColor Yellow
    foreach ($src in $sources[$category]) {
        $label = $src -replace ".*\\", ""

        $out = & $Compiler $src 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Host "    COMPILE FAIL  $label" -ForegroundColor Red
            $errors += "Compile: $src"
            $failed++; continue
        }

        $exe = $src -replace "\.ti$", ".exe"
        if (-not (Test-Path $exe)) {
            Write-Host "    NO OUTPUT     $label" -ForegroundColor Yellow
            $failed++; continue
        }

        & $exe 2>&1 | Out-Null
        if ($LASTEXITCODE -eq 0) {
            Write-Host "    PASS          $label" -ForegroundColor Green
            $passed++
        } else {
            Write-Host "    FAIL          $label" -ForegroundColor Red
            $errors += "Test failed: $src"
            $failed++
        }
    }
}

Write-Host ""
Write-Host ("─" * 46) -ForegroundColor DarkGray
$total = $passed + $failed
$color = if ($failed -eq 0) { "Green" } else { "Red" }
Write-Host "$passed / $total passing" -ForegroundColor $color

if ($errors.Count -gt 0) {
    Write-Host ""
    Write-Host "Errors:" -ForegroundColor Red
    $errors | ForEach-Object { Write-Host "  $_" -ForegroundColor DarkRed }
}

exit $(if ($failed -eq 0) { 0 } else { 1 })
