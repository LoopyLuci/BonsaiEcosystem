$ErrorActionPreference = "Continue"
Set-Location "$PSScriptRoot\.."

$results = New-Object System.Collections.Generic.List[object]
$timestamp = Get-Date
$reportDir = Join-Path (Get-Location) "artifacts\validation"
New-Item -ItemType Directory -Path $reportDir -Force | Out-Null

function Add-Result {
    param(
        [string]$Subsystem,
        [string]$Test,
        [string]$Status,
        [string]$Expected,
        [string]$Actual,
        [string]$FailureMode
    )
    $results.Add([pscustomobject]@{
        Subsystem   = $Subsystem
        Test        = $Test
        Status      = $Status
        Expected    = $Expected
        Actual      = $Actual
        FailureMode = $FailureMode
    }) | Out-Null
}

function Run-CommandCapture {
    param(
        [string]$FilePath,
        [string[]]$Arguments,
        [int]$TimeoutMs = 8000
    )

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $FilePath
    $psi.WorkingDirectory = (Get-Location).Path
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.CreateNoWindow = $true
    # Use Arguments string (compatible with .NET Framework / PowerShell 5)
    if ($Arguments.Count -gt 0) {
        $escaped = $Arguments | ForEach-Object {
            if ($_ -match '\s') { "`"$_`"" } else { $_ }
        }
        $psi.Arguments = $escaped -join " "
    }

    $proc = New-Object System.Diagnostics.Process
    $proc.StartInfo = $psi
    [void]$proc.Start()

    $finished = $proc.WaitForExit($TimeoutMs)
    if (-not $finished) {
        try { $proc.Kill() } catch {}
        return [pscustomobject]@{
            Output = "Timed out after ${TimeoutMs}ms"
            ExitCode = -999
            TimedOut = $true
        }
    }

    $stdout = $proc.StandardOutput.ReadToEnd()
    $stderr = $proc.StandardError.ReadToEnd()
    $output = ($stdout + "`n" + $stderr).Trim()
    return [pscustomobject]@{
        Output = $output
        ExitCode = $proc.ExitCode
        TimedOut = $false
    }
}

function Get-CompilerPath {
    $candidates = @(
        (Join-Path (Get-Location) "titan-bootstrap\output\titan-compiler.exe"),
        (Join-Path (Get-Location) "titan-compiler.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { return $c }
    }
    return $null
}

function Test-StubHeuristic {
    param([string]$Path)
    $text = Get-Content $Path -Raw

    $hasMain111 = $text -match 'pub\s+fn\s+main\s*\([^\)]*\)\s*->\s*i64\s*\{[\s\S]*?return\s+111\s*;'
    $hasControl = $text -match '\bif\b|\bwhile\b|\bfor\b|\bmatch\b'
    $hasCalls = $text -match '\w+\s*\('
    $shortFile = ((Get-Content $Path | Measure-Object -Line).Lines -lt 25)
    $returnOnly = $text -match 'pub\s+fn\s+main\s*\([^\)]*\)\s*->\s*i64\s*\{\s*return\s+111\s*;\s*\}'
    $lineCount = (Get-Content $Path | Measure-Object -Line).Lines
    $trivialFnCount = ([regex]::Matches($text, 'pub\s+fn\s+\w+\s*\([^\)]*\)\s*(->\s*[^\s\{]+)?\s*\{\s*return\s+[-]?\d+\s*;\s*\}')).Count
    $containsStubMarkers = $text -match '\bstub\b|TODO|placeholder'

    if ($returnOnly) { return "STUB" }
    if ($containsStubMarkers) { return "STUB_SUSPECT" }
    if ($lineCount -lt 80 -and $trivialFnCount -ge 2) { return "STUB_SUSPECT" }
    if ($hasMain111 -and -not $hasControl -and $shortFile) { return "STUB_SUSPECT" }
    if (-not $hasCalls) { return "STUB_SUSPECT" }
    return "OK"
}

$compiler = Get-CompilerPath
if (-not $compiler) {
    Add-Result "Core" "Compiler Presence" "FAIL" "Native compiler exists" "No compiler found at expected paths" "Missing compiler binary"
} else {
    Add-Result "Core" "Compiler Presence" "PASS" "Native compiler exists" "Found $compiler" ""
}

# 1) CI invariants gate
$invariants = Join-Path (Get-Location) "scripts\ci\check_bootstrap_invariants.ps1"
if (Test-Path $invariants) {
    $res = Run-CommandCapture -FilePath "powershell" -Arguments @("-NoProfile", "-ExecutionPolicy", "Bypass", "-File", $invariants) -TimeoutMs 300000
    if ($res.ExitCode -eq 0) {
        Add-Result "Bootstrap" "CI Invariants" "PASS" "Script exits 0" $res.Output ""
    } else {
        Add-Result "Bootstrap" "CI Invariants" "FAIL" "Script exits 0" $res.Output "Invariant breach"
    }
} else {
    Add-Result "Bootstrap" "CI Invariants" "FAIL" "Script exists" "scripts/ci/check_bootstrap_invariants.ps1 missing" "Missing gate script"
}

# 2) Titan compiler behavior tests
if ($compiler) {
    $tmpDir = Join-Path $reportDir "tmp"
    New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null

    $tests = @(
        @{ Name = "Basic Expression"; File = "test_basic.ti"; Source = "pub pub fn main() -> i64 { return 42; }"; Expected = "42" },
        @{ Name = "Arithmetic Control"; File = "test_arith_if.ti"; Source = "pub pub fn main() -> i64 { let x: i64 = 10; let y: i64 = 20; if x < y { return x + y; } else { return 0; } }"; Expected = "30" },
        @{ Name = "Recursion"; File = "test_recursion.ti"; Source = "pub pub fn main() -> i64 { return factorial(5); } pub fn factorial(n: i64) -> i64 { if n <= 1 { return 1; } return n * factorial(n - 1); }"; Expected = "120" },
        @{ Name = "Array Loop"; File = "test_array_loop.ti"; Source = "pub pub fn main() -> i64 { let arr: [i64; 5] = [1, 2, 3, 4, 5]; let mut sum: i64 = 0; let mut i: i64 = 0; while i < 5 { sum = sum + arr[i]; i = i + 1; } return sum; }"; Expected = "15" }
    )

    foreach ($t in $tests) {
        $p = Join-Path $tmpDir $t.File
        Set-Content -Path $p -Value $t.Source -Encoding ascii

        $compileOut = Run-CommandCapture -FilePath $compiler -Arguments @($p)

        # Bootstrap compiler transpiles+compiles+runs in one step; check for Result: NNN
        $exePath = [System.IO.Path]::ChangeExtension($p, ".exe")
        if ($compileOut.Output -match "Result:\s*$([regex]::Escape($t.Expected))") {
            Add-Result "Titan Compiler" $t.Name "PASS" "Result: $($t.Expected)" $compileOut.Output ""
        } elseif (Test-Path $exePath) {
            $runOut = Run-CommandCapture -FilePath $exePath -Arguments @()
            if ($runOut.Output -match [regex]::Escape($t.Expected)) {
                Add-Result "Titan Compiler" $t.Name "PASS" "Output contains $($t.Expected)" $runOut.Output ""
            } else {
                Add-Result "Titan Compiler" $t.Name "FAIL" "Output contains $($t.Expected)" "Compile: $($compileOut.Output)`nRun: $($runOut.Output)" "Incorrect semantics"
            }
        } else {
            Add-Result "Titan Compiler" $t.Name "FAIL" "Result: $($t.Expected)" "Compiler output: $($compileOut.Output)" "Unexpected result"
        }
    }

    # self-compilation fixpoint smoke
    $c1 = Join-Path $reportDir "titan-compiler-new.exe"
    $c2 = Join-Path $reportDir "titan-compiler-new2.exe"
    $s1 = Run-CommandCapture -FilePath $compiler -Arguments @("titan/compiler/", "-o", $c1)
    if (Test-Path $c1) {
        $s2 = Run-CommandCapture -FilePath $c1 -Arguments @("titan/compiler/", "-o", $c2)
        if (Test-Path $c2) {
            $b1 = [System.IO.File]::ReadAllBytes($c1)
            $b2 = [System.IO.File]::ReadAllBytes($c2)
            $same = ($b1.Length -eq $b2.Length)
            if ($same) {
                for ($i = 0; $i -lt $b1.Length; $i++) { if ($b1[$i] -ne $b2[$i]) { $same = $false; break } }
            }
            if ($same) {
                Add-Result "Titan Compiler" "Self-compilation fixpoint" "PASS" "Binaries identical" "Byte-identical outputs" ""
            } else {
                Add-Result "Titan Compiler" "Self-compilation fixpoint" "FAIL" "Binaries identical" "Generated binaries differ" "Fixpoint broken"
            }
        } else {
            Add-Result "Titan Compiler" "Self-compilation fixpoint" "FAIL" "Second compile creates binary" $s2.Output "Second-stage compile failed"
        }
    } else {
        Add-Result "Titan Compiler" "Self-compilation fixpoint" "FAIL" "First compile creates binary" $s1.Output "First-stage compile failed"
    }
}

# 3) OmniLib module sweep + stub heuristic
$stdFiles = Get-ChildItem "titan/std" -Filter "*.ti" -File -ErrorAction SilentlyContinue
foreach ($f in $stdFiles) {
    if ($compiler) {
        $r = Run-CommandCapture -FilePath $compiler -Arguments @($f.FullName, "--run")
        if ($r.Output -match "Result:\s*111") {
            $stub = Test-StubHeuristic -Path $f.FullName
            if ($stub -eq "OK") {
                Add-Result "OmniLib" $f.Name "PASS" "Result 111 with non-stub structure" $r.Output ""
            } else {
                Add-Result "OmniLib" $f.Name "FAIL" "Non-stub implementation" "$($r.Output) | Heuristic=$stub" "Potential stub/placeholder"
            }
        } else {
            Add-Result "OmniLib" $f.Name "FAIL" "Result: 111" $r.Output "Runtime/test failure"
        }
    } else {
        Add-Result "OmniLib" $f.Name "SKIP" "Compiler available" "Skipped (no compiler)" "Blocked"
    }
}

# 4) Axiom theory sweep
$theoryFiles = Get-ChildItem "axiom/theories" -Filter "*.ti" -File -ErrorAction SilentlyContinue
foreach ($f in $theoryFiles) {
    if ($compiler) {
        $r = Run-CommandCapture -FilePath $compiler -Arguments @($f.FullName, "--run")
        if ($r.Output -match "Result:\s*111") {
            Add-Result "Axiom" $f.Name "PASS" "Result: 111" $r.Output ""
        } else {
            Add-Result "Axiom" $f.Name "FAIL" "Result: 111" $r.Output "Proof/kernel regression"
        }
    } else {
        Add-Result "Axiom" $f.Name "SKIP" "Compiler available" "Skipped (no compiler)" "Blocked"
    }
}

# 5) Aion self-test if present
$aionTest = "aion/test_aion_core.ti"
if ((Test-Path $aionTest) -and $compiler) {
    $r = Run-CommandCapture -FilePath $compiler -Arguments @($aionTest, "--run")
    if ($r.Output -match "Result:\s*111") {
        Add-Result "Aion" "test_aion_core.ti" "PASS" "Result: 111" $r.Output ""
    } else {
        Add-Result "Aion" "test_aion_core.ti" "FAIL" "Result: 111" $r.Output "Aion self-test failed"
    }
} elseif (Test-Path $aionTest) {
    Add-Result "Aion" "test_aion_core.ti" "SKIP" "Compiler available" "Skipped (no compiler)" "Blocked"
} else {
    Add-Result "Aion" "test_aion_core.ti" "SKIP" "Test file exists" "Missing file" "Not available in repo"
}

# 6) Sylva smoke via known test module
$sylvaTest = "tests/test_sylva_repl.ti"
if ((Test-Path $sylvaTest) -and $compiler) {
    $r = Run-CommandCapture -FilePath $compiler -Arguments @($sylvaTest, "--run")
    if ($r.Output -match "Result:\s*111") {
        Add-Result "Sylva" "REPL smoke" "PASS" "Result: 111" $r.Output ""
    } else {
        Add-Result "Sylva" "REPL smoke" "FAIL" "Result: 111" $r.Output "REPL smoke failed"
    }
} elseif (Test-Path $sylvaTest) {
    Add-Result "Sylva" "REPL smoke" "SKIP" "Compiler available" "Skipped (no compiler)" "Blocked"
} else {
    Add-Result "Sylva" "REPL smoke" "SKIP" "Test file exists" "Missing file" "Not available in repo"
}

# 7) IDE/GUI and deploy checks (environment-dependent)
Add-Result "Studio" "VS Code extension activation" "SKIP" "Headless automation or manual run" "Not run in this script" "Requires IDE runtime"
Add-Result "OmniWeb" "Build + preview + deploy" "SKIP" "Runtime network/services available" "Not run in this script" "Requires service environment"

# Aggregate summary
$pass = ($results | Where-Object { $_.Status -eq "PASS" }).Count
$fail = ($results | Where-Object { $_.Status -eq "FAIL" }).Count
$skip = ($results | Where-Object { $_.Status -eq "SKIP" }).Count

$jsonPath = Join-Path $reportDir "validation-results.json"
$htmlPath = Join-Path $reportDir "validation-report.html"

$results | ConvertTo-Json -Depth 5 | Set-Content -Path $jsonPath -Encoding utf8

$summaryHtml = @"
<h1>Omnisystem Comprehensive Validation Report</h1>
<p><b>Generated:</b> $timestamp</p>
<p><b>PASS:</b> $pass &nbsp; <b>FAIL:</b> $fail &nbsp; <b>SKIP:</b> $skip</p>
"@

$tableHtml = $results |
    Select-Object Subsystem, Test, Status, Expected, Actual, FailureMode |
    ConvertTo-Html -Fragment -PreContent "<h2>Detailed Results</h2>"

$fullHtml = @"
<html>
<head>
<title>Omnisystem Validation Report</title>
<style>
body { font-family: Segoe UI, Arial, sans-serif; margin: 24px; }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid #ddd; padding: 8px; vertical-align: top; }
th { background: #f5f5f5; }
</style>
</head>
<body>
$summaryHtml
$tableHtml
</body>
</html>
"@

Set-Content -Path $htmlPath -Value $fullHtml -Encoding utf8

Write-Host "Validation complete." -ForegroundColor Cyan
Write-Host "PASS: $pass  FAIL: $fail  SKIP: $skip" -ForegroundColor Cyan
Write-Host "JSON: $jsonPath"
Write-Host "HTML: $htmlPath"

if ($fail -gt 0) { exit 1 }
exit 0
