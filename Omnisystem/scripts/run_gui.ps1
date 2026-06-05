# scripts/run_gui.ps1
Remove-Item Omnisystem.exe -Force -ErrorAction SilentlyContinue
$log = & .\titan-bootstrap\output\titan-compiler.exe Omnisystem.ti 2>&1
Write-Host "Transpiler Output:`n$log"

$exe = Get-Item "Omnisystem.exe" -ErrorAction SilentlyContinue
if ($null -ne $exe -and $exe.Length -gt 63746) {
    Write-Host "Native GUI successfully linked ($($exe.Length) bytes). Launching..."
    & .\Omnisystem.exe
} else {
    Write-Host "FATAL: Compiler reverted to stub. Size: $($exe.Length) bytes."
}
