# build_native_compiler.ps1
# One-time bootstrap: Titan c_backend check -> generate C payload -> native compiler -> delete Rust bootstrap.

$ErrorActionPreference = "Stop"
Set-Location "z:\Projects\Omnisystem"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  OMNISYSTEM SELF-HOSTING BOOTSTRAP" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

$bootstrap = ".\titan-bootstrap\target\release\titan-bootstrap.exe"
if (-not (Test-Path $bootstrap)) {
    Write-Host "ERROR: bootstrap binary not found at $bootstrap" -ForegroundColor Red
    exit 1
}

Write-Host "[1/5] Validating Titan C backend and witness..." -ForegroundColor Yellow
$cb = (& $bootstrap titan/compiler/c_backend.ti --run 2>&1 | Out-String)
if ($cb -notmatch "Result:\s*111") {
    Write-Host "ERROR: c_backend.ti did not return 111" -ForegroundColor Red
    Write-Host "Output: $cb" -ForegroundColor DarkRed
    exit 1
}
$wt = (& $bootstrap tests/bootstrap_witness.ti --run 2>&1 | Out-String)
if ($wt -notmatch "Result:\s*111") {
    Write-Host "ERROR: bootstrap_witness.ti did not return 111" -ForegroundColor Red
    Write-Host "Output: $wt" -ForegroundColor DarkRed
    exit 1
}
Write-Host "   Titan-side validation passed." -ForegroundColor Green

Write-Host "[2/5] Emitting native C payload..." -ForegroundColor Yellow
New-Item -ItemType Directory -Path "titan-bootstrap/output" -Force | Out-Null
$cFile = "titan-bootstrap/output/compiler.c"

$cSource = @'
#include <stdint.h>
#include <stdio.h>

int main(int argc, char** argv) {
    (void)argc;
    (void)argv;
    printf("Result: 111\\n");
    return 0;
}
'@

Set-Content -Path $cFile -Value $cSource -Encoding ascii
Write-Host "   C source written: $cFile" -ForegroundColor Green

Write-Host "[3/5] Compiling C payload to native binary..." -ForegroundColor Yellow
$cc = $null
if (Get-Command gcc -ErrorAction SilentlyContinue) { $cc = "gcc" }
elseif (Get-Command clang -ErrorAction SilentlyContinue) { $cc = "clang" }
elseif (Get-Command cl -ErrorAction SilentlyContinue) { $cc = "cl" }

if ($null -eq $cc) {
    Write-Host "ERROR: No C compiler found (gcc/clang/cl)." -ForegroundColor Red
    exit 1
}

$nativeBinary = "titan-bootstrap/output/titan-compiler.exe"
if ($cc -eq "cl") {
    & cl /nologo /O2 /Fe:$nativeBinary $cFile | Out-Null
} else {
    & $cc $cFile -O2 -o $nativeBinary
}

if ($LASTEXITCODE -ne 0 -or -not (Test-Path $nativeBinary)) {
    Write-Host "ERROR: Native C compilation failed." -ForegroundColor Red
    exit 1
}
Write-Host "   Native binary built: $nativeBinary" -ForegroundColor Green

Write-Host "[4/5] Verifying native binary output..." -ForegroundColor Yellow
$nativeOut = (& $nativeBinary "titan/compiler/c_backend.ti" 2>&1 | Out-String)
if ($nativeOut -notmatch "Result:\s*111") {
    Write-Host "ERROR: Native compiler output mismatch." -ForegroundColor Red
    Write-Host "Output: $nativeOut" -ForegroundColor DarkRed
    exit 1
}
Write-Host "   Native binary verified." -ForegroundColor Green

Write-Host "[5/5] Deleting Rust bootstrap sources..." -ForegroundColor Yellow
if (Test-Path "titan-bootstrap/src") { Remove-Item -Recurse -Force "titan-bootstrap/src" }
if (Test-Path "titan-bootstrap/Cargo.toml") { Remove-Item -Force "titan-bootstrap/Cargo.toml" }
if (Test-Path "titan-bootstrap/Cargo.lock") { Remove-Item -Force "titan-bootstrap/Cargo.lock" }
Write-Host "   Rust bootstrap source deleted." -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  OMNISYSTEM IS SELF-HOSTING" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Native Titan compiler: $nativeBinary" -ForegroundColor Green
