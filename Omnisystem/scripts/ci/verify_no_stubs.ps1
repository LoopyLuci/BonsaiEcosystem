# verify_no_stubs.ps1
# Checks that every OmniLib module is a real implementation, not a stub.
$ErrorActionPreference = "Stop"
Set-Location "$PSScriptRoot\..\.."

$modules = Get-ChildItem titan/std/*.ti
$failures = @()

foreach ($mod in $modules) {
    $content = Get-Content $mod.FullName -Raw
    $lines = (Get-Content $mod.FullName).Count
    $lower = $content.ToLowerInvariant()

    # Heuristic 1: Explicit placeholder language is always a failure.
    if ($lower -match 'stub|placeholder|todo|simulated|no-op|in this version') {
        $failures += "$($mod.Name) : STUB MARKER language present"
        continue
    }

    # Heuristic 2: Must not be trivial return-111 main only.
    if ($content -match 'pub\s+fn\s+main\s*\([^\)]*\)\s*->\s*i64\s*\{\s*return\s+111\s*;\s*\}') {
        $failures += "$($mod.Name) : trivial main return"
        continue
    }

    # Heuristic 3: Reject files made mostly of constant-return functions.
    $fnCount = ([regex]::Matches($content, 'pub\s+fn\s+\w+\s*\(')).Count
    $constReturnCount = ([regex]::Matches($content, 'pub\s+fn\s+\w+\s*\([^\)]*\)\s*(->\s*[^\s\{]+)?\s*\{\s*return\s+[-]?\d+\s*;\s*\}')).Count
    if ($fnCount -ge 3 -and $constReturnCount -ge [Math]::Max(2, [Math]::Floor($fnCount * 0.6))) {
        $failures += "$($mod.Name) : mostly constant-return functions"
        continue
    }

    # Heuristic 4: Very small files must still show real behavior.
    if ($lines -lt 25 -and $fnCount -lt 3) {
        $failures += "$($mod.Name) : too small to be a full implementation ($lines lines)"
        continue
    }

    # Heuristic 5: Require either control flow or non-trivial call usage.
    $hasControl = $content -match '\bwhile\b|\bfor\b|\bif\b|\bmatch\b'
    $hasCalls = $content -match '\w+\s*\('
    if (-not $hasControl -and -not $hasCalls) {
        $failures += "$($mod.Name) : no behavioral structure detected"
        continue
    }
}

if ($failures.Count -eq 0) {
    Write-Host "ALL MODULES HAVE REAL IMPLEMENTATIONS" -ForegroundColor Green
    exit 0
} else {
    Write-Host "STUBS DETECTED:" -ForegroundColor Red
    foreach ($f in $failures) { Write-Host "  $f" -ForegroundColor Red }
    exit 1
}
