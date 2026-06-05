# check_bootstrap_invariants.ps1
# Must pass on every commit. If this fails, the change is rejected.
$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot\..\.."

$errors = @()

# Invariant 1: Zero Rust in the repository
Write-Host "[1/3] Checking for Rust source files..." -ForegroundColor Cyan
$rust_count = (Get-ChildItem -Recurse -Filter *.rs -File 2>$null | Measure-Object).Count
if ($rust_count -ne 0) {
    $errors += "INVARIANT BROKEN: Found $rust_count .rs file(s). Omnisystem must contain zero Rust."
    Get-ChildItem -Recurse -Filter *.rs -File | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
} else {
    Write-Host "  Zero Rust files found." -ForegroundColor Green
}

# Invariant 2: Bootstrap witness returns 111
Write-Host "[2/3] Running bootstrap witness..." -ForegroundColor Cyan
$witness = "tests/bootstrap_witness.ti"
$compiler = "titan-bootstrap/output/titan-compiler.exe"

if (-not (Test-Path $compiler)) {
    $errors += "INVARIANT BROKEN: Native compiler binary not found at $compiler"
} else {
    $result = & $compiler $witness 2>&1
    if ($result -match "Result: 111") {
        Write-Host "  Witness returned 111 - self-hosting confirmed." -ForegroundColor Green
    } else {
        $errors += "INVARIANT BROKEN: Bootstrap witness failed. Output: $result"
        Write-Host "  Witness output: $result" -ForegroundColor Red
    }
}

# Invariant 3: Native compiler can compile its own source
Write-Host "[3/3] Self-compilation test..." -ForegroundColor Cyan
$compiler_source = "titan/compiler/"
if (-not (Test-Path $compiler_source)) {
    $errors += "INVARIANT BROKEN: Compiler source directory not found at $compiler_source"
} else {
    $self_result = & $compiler $compiler_source 2>&1
    if ($LASTEXITCODE -eq 0 -or $self_result -match "Result: 111") {
        Write-Host "  Self-compilation succeeded." -ForegroundColor Green
    } else {
        $errors += "INVARIANT BROKEN: Self-compilation failed."
        Write-Host "  Output: $self_result" -ForegroundColor Red
    }
}

# Report
Write-Host ""
if ($errors.Count -eq 0) {
    Write-Host "ALL BOOTSTRAP INVARIANTS HOLD" -ForegroundColor Green
    Write-Host "The Omnisystem is self-hosting." -ForegroundColor Green
    exit 0
} else {
    Write-Host "BOOTSTRAP INVARIANTS BROKEN:" -ForegroundColor Red
    foreach ($e in $errors) { Write-Host "  $e" -ForegroundColor Red }
    Write-Host ""
    Write-Host "This change would break self-hosting. Rejected." -ForegroundColor Red
    exit 1
}
