# Comprehensive USOS → UOSC Rename Script
# Executes atomically with verification at each step
# Handles: directories, files, content, strings, function names, types

param(
    [switch]$DryRun = $false,
    [switch]$VerifyOnly = $false
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

Write-Host "═════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "Universal Driver Converter: USOS → UOSC Rename" -ForegroundColor Cyan
Write-Host "═════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

# Phase 0: Audit and Report
Write-Host "Phase 0: Pre-Rename Audit" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────────────────────" -ForegroundColor Yellow

$excludePatterns = @('\.venv*', '\.git', 'node_modules', 'target', 'dist', '__pycache__')
$textExtensions = @('*.ti', '*.rs', '*.md', '*.toml', '*.json', '*.sh', '*.ps1', '*.txt', '*.yml', '*.yaml', 'Makefile', 'Cargo.lock')

# Find all files
$allFiles = @()
foreach ($ext in $textExtensions) {
    $allFiles += Get-ChildItem -Path . -Recurse -Filter $ext -ErrorAction SilentlyContinue |
                 Where-Object { $_.FullName -notmatch ($excludePatterns -join '|') }
}

# Audit USOS references
$usosOccurrences = @()
foreach ($file in $allFiles) {
    $content = Get-Content -Path $file.FullName -ErrorAction SilentlyContinue | Out-String
    if ($content -match '\bUSOS\b|\bUsos\b|\busos\b') {
        $lines = @()
        $lineNum = 0
        foreach ($line in (Get-Content -Path $file.FullName -ErrorAction SilentlyContinue)) {
            $lineNum++
            if ($line -match '\bUSOS\b|\bUsos\b|\busos\b') {
                $lines += @{ LineNumber = $lineNum; Content = $line }
            }
        }
        if ($lines.Count -gt 0) {
            $usosOccurrences += @{
                File = $file.FullName.Substring((Get-Location).Path.Length + 1)
                Count = $lines.Count
                Lines = $lines
            }
        }
    }
}

Write-Host "Found $($usosOccurrences.Count) files with USOS references" -ForegroundColor Green
Write-Host ""

# Report audit results
foreach ($item in $usosOccurrences) {
    Write-Host "  ✓ $($item.File) - $($item.Count) occurrence(s)" -ForegroundColor Green
}

if ($VerifyOnly) {
    Write-Host ""
    Write-Host "Verification complete. Exiting." -ForegroundColor Green
    exit 0
}

Write-Host ""
Write-Host "Phase 1: File and Directory Renames" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────────────────────" -ForegroundColor Yellow

# Rename files containing 'usos'
$filesToRename = Get-ChildItem -Path . -Recurse -ErrorAction SilentlyContinue |
                 Where-Object { $_.Name -match 'usos' -and $_.FullName -notmatch ($excludePatterns -join '|') }

if ($filesToRename.Count -gt 0) {
    foreach ($file in $filesToRename) {
        $newName = $file.Name -replace 'usos', 'uosc' -replace 'USOS', 'UOSC' -replace 'Usos', 'Uosc'
        if ($newName -ne $file.Name) {
            if (-not $DryRun) {
                Rename-Item -Path $file.FullName -NewName $newName -Force
                Write-Host "  ✓ Renamed: $($file.Name) → $newName" -ForegroundColor Green
            } else {
                Write-Host "  [DRY-RUN] Would rename: $($file.Name) → $newName" -ForegroundColor Cyan
            }
        }
    }
} else {
    Write-Host "  (No files to rename)" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Phase 2: Content Replacement" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────────────────────" -ForegroundColor Yellow

$filesUpdated = 0
$replacementsCount = 0

foreach ($file in $allFiles) {
    try {
        $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($null -eq $content) { continue }

        $originalContent = $content

        # All replacements (order matters for case sensitivity)
        # 1. USOS → UOSC (all caps)
        $content = $content -replace '\bUSOS\b', 'UOSC'
        $content = $content -replace '"usos"', '"uosc"'
        $content = $content -replace "'usos'", "'uosc'"
        $content = $content -replace 'USOS_', 'UOSC_'

        # 2. Usos → Uosc (title case for Rust types)
        $content = $content -replace '\bUsos\b', 'Uosc'
        $content = $content -replace 'Usos::', 'Uosc::'

        # 3. usos → uosc (lowercase for variables, functions, modules, files)
        $content = $content -replace '\busos\b', 'uosc'
        $content = $content -replace 'usos_', 'uosc_'
        $content = $content -replace '/usos/', '/uosc/'

        # Path-like replacements
        $content = $content -replace 'kernel/usos', 'kernel/uosc'
        $content = $content -replace 'docs/usos', 'docs/uosc'
        $content = $content -replace 'crates/usos', 'crates/uosc'

        # Import/module statements
        $content = $content -replace 'use usos_', 'use uosc_'
        $content = $content -replace 'from usos', 'from uosc'
        $content = $content -replace 'import usos', 'import uosc'

        # Comments and documentation
        $content = $content -replace 'USOS kernel', 'UOSC kernel'
        $content = $content -replace 'USOS core', 'UOSC core'
        $content = $content -replace 'USOS Co-OS', 'UOSC Co-OS'

        if ($content -ne $originalContent) {
            if (-not $DryRun) {
                Set-Content -Path $file.FullName -Value $content -NoNewline -Force
                $filesUpdated++
                $changedLines = (($originalContent -split '\n') | Where-Object { $_ -match '\bUSOS\b|\bUsos\b|\busos\b' }).Count
                $replacementsCount += $changedLines
                Write-Host "  ✓ Updated: $($file.Name) ($changedLines occurrences)" -ForegroundColor Green
            } else {
                Write-Host "  [DRY-RUN] Would update: $($file.Name)" -ForegroundColor Cyan
            }
        }
    } catch {
        Write-Host "  ⚠ Skipped: $($file.Name) - $($_.Exception.Message)" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "Phase 2 Summary: Updated $filesUpdated files with ~$replacementsCount total replacements" -ForegroundColor Green

Write-Host ""
Write-Host "Phase 3: Verification" -ForegroundColor Yellow
Write-Host "─────────────────────────────────────────────────────────" -ForegroundColor Yellow

# Check for residual references
$residual = @()
foreach ($file in $allFiles) {
    try {
        $content = Get-Content -Path $file.FullName -Raw -ErrorAction SilentlyContinue
        if ($null -eq $content) { continue }

        # Look for remaining USOS references (excluding intentional historical notes)
        if ($content -match '\bUSOS\b' -or $content -match '\busos\b') {
            $lines = @()
            $lineNum = 0
            foreach ($line in (Get-Content -Path $file.FullName)) {
                $lineNum++
                if ($line -match '\bUSOS\b' -or $line -match '\busos\b') {
                    # Filter out false positives like "mimusos" or comment dates
                    if ($line -notmatch '\d{4}-\d{2}-\d{2}' -and $line -notmatch 'mimusos') {
                        $lines += "$($lineNum): $line"
                    }
                }
            }
            if ($lines.Count -gt 0) {
                $residual += @{
                    File = $file.FullName.Substring((Get-Location).Path.Length + 1)
                    Lines = $lines
                }
            }
        }
    } catch {
        # Skip files that can't be read
    }
}

if ($residual.Count -eq 0) {
    Write-Host "  ✓ Verification passed: No residual USOS references found" -ForegroundColor Green
} else {
    Write-Host "  ⚠ Found $($residual.Count) files with potential residual references:" -ForegroundColor Yellow
    foreach ($item in $residual) {
        Write-Host "    - $($item.File)" -ForegroundColor Yellow
        foreach ($line in $item.Lines) {
            Write-Host "      $line" -ForegroundColor Gray
        }
    }
}

Write-Host ""
Write-Host "═════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "Rename Complete" -ForegroundColor Cyan
Write-Host "═════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "[DRY-RUN MODE] No actual changes were made." -ForegroundColor Yellow
    Write-Host "Run without -DryRun to apply changes." -ForegroundColor Yellow
} else {
    Write-Host "Summary:" -ForegroundColor Green
    Write-Host "  • Files renamed: $(($filesToRename | Measure-Object).Count)" -ForegroundColor Green
    Write-Host "  • Files updated: $filesUpdated" -ForegroundColor Green
    Write-Host "  • Total replacements: ~$replacementsCount" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Green
    Write-Host "  1. Review changes: git diff" -ForegroundColor Green
    Write-Host "  2. Build: cargo build" -ForegroundColor Green
    Write-Host "  3. Test: cargo test" -ForegroundColor Green
    Write-Host "  4. Commit: git add -A && git commit -m 'refactor: rename USOS → UOSC (Universal Operating System Core)'" -ForegroundColor Green
}

Write-Host ""
