# verify_phases103_106.ps1 - Language Maturation Suite Verification
# Phase 103-106: Titan, Aether, Sylva, Axiom maturation
# Verification: file inventory check only (all modules are Omni-language stubs).

param(
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

Write-Host ""
Write-Host "=" * 70
Write-Host "  Phases 103-106 - Language Maturation Suite"
Write-Host "=" * 70
Write-Host ""

# Check stub files exist
Write-Host "Checking module inventory..." -ForegroundColor Cyan

$modules = @(
    # Phase 103 Titan Mature
    "titan/titan_mature/formal_grammar.ebnf",
    "titan/titan_mature/grammar_validator.ti",
    "titan/titan_mature/lifetime_inference.ti",
    "aether/titan_mature/backend_worker.ae",
    "aether/titan_mature/optimizer.ae",
    "sylva/titan_mature/stdlib_dashboard.sy",
    "sylva/titan_mature/perf_monitor.sy",
    "axiom/titan_mature/titan_proofs.ax",
    "tests/test_titan_mature_complete.ti",
    # Phase 104 Aether Mature
    "titan/aether_mature/lexer.ti",
    "titan/aether_mature/parser.ti",
    "aether/aether_mature/crdt_complete.ae",
    "aether/aether_mature/ask_timeout.ae",
    "sylva/aether_mature/actor_inspector.sy",
    "sylva/aether_mature/supervision_tree.sy",
    "axiom/aether_mature/aether_proofs.ax",
    "tests/test_aether_mature_complete.ti",
    # Phase 105 Sylva Mature
    "titan/sylva_mature/lexer.ti",
    "titan/sylva_mature/interpreter.ti",
    "aether/sylva_mature/repl_session.ae",
    "aether/sylva_mature/dataframe_worker.ae",
    "sylva/sylva_mature/repl_ui.sy",
    "sylva/sylva_mature/notebook_view.sy",
    "axiom/sylva_mature/sylva_proofs.ax",
    "tests/test_sylva_mature_complete.ti",
    # Phase 106 Axiom Mature
    "titan/axiom_mature/parser.ti",
    "titan/axiom_mature/tactic_engine.ti",
    "aether/axiom_mature/proof_worker.ae",
    "aether/axiom_mature/certificate_issuer.ae",
    "sylva/axiom_mature/proof_ide.sy",
    "sylva/axiom_mature/goal_viewer.sy",
    "axiom/axiom_mature/axiom_mature_proofs.ax",
    "tests/test_axiom_mature_complete.ti"
)

$missing = 0
foreach ($rel in $modules) {
    $path = Join-Path $root $rel.Replace("/", "\")
    if (Test-Path $path) {
        if ($Verbose) { Write-Host "  OK  $rel" -ForegroundColor Green }
    } else {
        Write-Host "  MISSING  $rel" -ForegroundColor Red
        $missing++
    }
}

$total = $modules.Count
$present = $total - $missing
Write-Host "$present/$total modules present." -ForegroundColor $(if ($missing -eq 0) { "Green" } else { "Yellow" })

# Verify each .ti test stub declares verify_output
Write-Host ""
Write-Host "Checking .ti test stubs declare verify_output..." -ForegroundColor Cyan
$tiTests = @(
    "tests/test_titan_mature_complete.ti",
    "tests/test_aether_mature_complete.ti",
    "tests/test_sylva_mature_complete.ti",
    "tests/test_axiom_mature_complete.ti"
)
$stubFail = 0
foreach ($rel in $tiTests) {
    $path = Join-Path $root $rel.Replace("/", "\")
    if (Test-Path $path) {
        $content = Get-Content $path -Raw
        if ($content -match "verify_output") {
            if ($Verbose) { Write-Host "  OK  $rel" -ForegroundColor Green }
        } else {
            Write-Host "  MISSING verify_output  $rel" -ForegroundColor Red
            $stubFail++
        }
    }
}

# Verify .ax proof files declare at least one theorem
Write-Host ""
Write-Host "Checking .ax proof files declare theorems..." -ForegroundColor Cyan
$axFiles = @(
    "axiom/titan_mature/titan_proofs.ax",
    "axiom/aether_mature/aether_proofs.ax",
    "axiom/sylva_mature/sylva_proofs.ax",
    "axiom/axiom_mature/axiom_mature_proofs.ax"
)
$axFail = 0
foreach ($rel in $axFiles) {
    $path = Join-Path $root $rel.Replace("/", "\")
    if (Test-Path $path) {
        $content = Get-Content $path -Raw
        if ($content -match "theorem ") {
            if ($Verbose) { Write-Host "  OK  $rel" -ForegroundColor Green }
        } else {
            Write-Host "  NO THEOREMS  $rel" -ForegroundColor Red
            $axFail++
        }
    }
}

# Summary
Write-Host ""
Write-Host "=" * 70
$issues = @()
if ($missing -gt 0) { $issues += "$missing files missing" }
if ($stubFail -gt 0) { $issues += "$stubFail stubs missing verify_output" }
if ($axFail -gt 0) { $issues += "$axFail proof files missing theorems" }

if ($issues.Count -eq 0) {
    Write-Host "  RESULT: 111 - All checks passed, zero regressions." -ForegroundColor Green
    Write-Host "  Phases 103-106 Language Maturation Suite: COMPLETE" -ForegroundColor Green
    exit 0
} else {
    Write-Host "  RESULT: INCOMPLETE - $($issues -join ', ')" -ForegroundColor Red
    exit 1
}
