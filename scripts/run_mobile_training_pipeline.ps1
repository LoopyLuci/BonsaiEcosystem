# BonsAI Mobile Model Training Pipeline Orchestration
# PowerShell script to run the complete distillation pipeline.
#
# This script:
#   1. Exports training data (multiple domains with quality filtering)
#   2. Starts teacher sidecar (llama-server with large teacher model)
#   3. Runs distillation training
#   4. Quantizes to GGUF (Q4_K_M)
#   5. Runs benchmarks
#   6. Registers model with registry
#   7. Creates distribution package
#
# Prerequisites:
#   - Python 3.10+ with torch, transformers, peft
#   - llama.cpp with llama-server binary
#   - GGUF files for teacher and student base models
#
# Usage:
#   .\scripts\run_mobile_training_pipeline.ps1 -StudentModel "TinyLlama-1.1B-Instruct" `
#     -TeacherGGUF "D:/Models/general/Bonsai-8B-Q4_K_M.gguf"
#
# On Windows, always use PowerShell (not Git Bash) to avoid segfaults.

param(
    [string]$StudentModel = "TinyLlama-1.1B-Instruct",
    [string]$TeacherGGUF = "D:/Models/general/Bonsai-8B-Q4_K_M.gguf",
    [string]$ConfigPath = "config/bonsai_mobile_config.yaml",
    [string]$OutputDir = "$env:USERPROFILE\.bonsai\models\checkpoints\bonsai-mobile-v1",
    [string]$DataExportPath = "$env:USERPROFILE\.bonsai\training_export\combined_mobile_training.jsonl",
    [string]$LlamaServerPort = "8080",
    [string]$LlamaServerBin = "$env:USERPROFILE\llama.cpp\llama-server.exe",
    [int]$Epochs = 3,
    [float]$LearningRate = 0.0005,
    [string]$Device = "cpu",  # cpu, cuda, mps, directml
    [switch]$SkipDataExport,
    [switch]$SkipTeacher,
    [switch]$SkipTraining,
    [switch]$SkipQuantization,
    [switch]$SkipBenchmark,
    [switch]$SkipRegister,
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"

# ── Colors ────────────────────────────────────────────────────────────────

function Write-Header { param([string]$Message)
    Write-Host "`n" -NoNewline
    Write-Host ("=" * 80) -ForegroundColor Cyan
    Write-Host $Message -ForegroundColor Cyan
    Write-Host ("=" * 80) -ForegroundColor Cyan
}

function Write-Step { param([string]$Message)
    Write-Host "`n>>> " -NoNewline -ForegroundColor Green
    Write-Host $Message -ForegroundColor White
}

function Write-Error { param([string]$Message)
    Write-Host "ERROR: " -NoNewline -ForegroundColor Red
    Write-Host $Message -ForegroundColor White
}

function Write-Warning { param([string]$Message)
    Write-Host "WARNING: " -NoNewline -ForegroundColor Yellow
    Write-Host $Message -ForegroundColor White
}

# ── Validation ────────────────────────────────────────────────────────────

Write-Header "BonsAI Mobile Training Pipeline"

Write-Step "Validating prerequisites..."

if (-not (Get-Command python -ErrorAction SilentlyContinue)) {
    Write-Error "Python not found. Install Python 3.10+ and ensure it's in PATH."
    exit 1
}

$PythonVersion = python --version 2>&1
Write-Host "  Python: $PythonVersion"

# Check GGUF files
if (-not (Test-Path $TeacherGGUF)) {
    Write-Error "Teacher GGUF not found: $TeacherGGUF"
    exit 1
}
Write-Host "  Teacher GGUF: $TeacherGGUF"

# Check llama-server (optional for API mode)
if (-not (Test-Path $LlamaServerBin)) {
    Write-Warning "llama-server not found at $LlamaServerBin"
    Write-Warning "Will attempt to use system llama-server. Make sure it's in PATH."
}

Write-Host "  Student model: $StudentModel"
Write-Host "  Config: $ConfigPath"
Write-Host "  Output: $OutputDir"

# ── Phase 1: Export Training Data ─────────────────────────────────────────

if (-not $SkipDataExport) {
    Write-Header "Phase 1: Export Training Data"

    Write-Step "Exporting combined training data..."

    $ExportCmd = @(
        "python scripts/export_mobile_training_data.py",
        "--output $DataExportPath",
        "--max-examples 100000",
        "--min-quality 0.70",
        "--domain-weights `"code:0.4,system_repair:0.2,tool_use:0.2,chat:0.1,qa:0.05`""
    ) -join " "

    Write-Host "  Command: $ExportCmd"

    if (-not $DryRun) {
        & python scripts/export_mobile_training_data.py `
            --output "$DataExportPath" `
            --max-examples 100000 `
            --min-quality 0.70 `
            --domain-weights "code:0.4,system_repair:0.2,tool_use:0.2,chat:0.1,qa:0.05"

        if ($LASTEXITCODE -ne 0) {
            Write-Error "Data export failed"
            exit 1
        }

        if (Test-Path $DataExportPath) {
            $FileSize = (Get-Item $DataExportPath).Length / 1MB
            Write-Host "  Output: $DataExportPath ($FileSize MB)"
        }
    }
}

# ── Phase 2: Start Teacher Sidecar (llama-server) ──────────────────────────

$TeacherPID = $null

if (-not $SkipTeacher) {
    Write-Header "Phase 2: Start Teacher Sidecar (llama-server)"

    # Check if server already running
    $Existing = Get-Process llama-server -ErrorAction SilentlyContinue
    if ($Existing) {
        Write-Warning "llama-server already running on port $LlamaServerPort"
        Write-Step "Using existing teacher sidecar"
    } else {
        Write-Step "Starting llama-server with teacher model..."

        # Determine binary
        $ServerBin = if (Test-Path $LlamaServerBin) { $LlamaServerBin } else { "llama-server" }

        # Build llama-server command
        $TeacherCmd = @(
            $ServerBin,
            "-m", "`"$TeacherGGUF`"",
            "-ngl 99",
            "--port $LlamaServerPort",
            "--no-mmap",
            "-c 2048"
        ) -join " "

        Write-Host "  Command: $TeacherCmd"

        if (-not $DryRun) {
            # Start in background
            $Process = Start-Process -FilePath $ServerBin -ArgumentList `
                "-m `"$TeacherGGUF`" -ngl 99 --port $LlamaServerPort --no-mmap -c 2048" `
                -PassThru -RedirectStandardOutput "llama-server.log" -RedirectStandardError "llama-server.err"

            $TeacherPID = $Process.Id
            Write-Host "  Started (PID: $TeacherPID)"
            Write-Host "  Logs: llama-server.log"

            # Wait for server to be ready
            Write-Step "Waiting for teacher server to be ready..."
            $MaxRetries = 60
            $Retries = 0
            while ($Retries -lt $MaxRetries) {
                try {
                    $Response = curl -s "http://127.0.0.1:$LlamaServerPort/health" -ErrorAction SilentlyContinue
                    if ($Response) {
                        Write-Host "  Teacher ready on http://127.0.0.1:$LlamaServerPort"
                        break
                    }
                } catch { }
                $Retries++
                Start-Sleep -Seconds 1
            }

            if ($Retries -ge $MaxRetries) {
                Write-Warning "Teacher server may not be ready (timeout after 60s)"
            }
        }
    }
}

# ── Phase 3: Training ─────────────────────────────────────────────────────

if (-not $SkipTraining) {
    Write-Header "Phase 3: Knowledge Distillation Training"

    Write-Step "Running training loop..."

    $TrainCmd = @(
        "python scripts/train_bonsai_mobile.py",
        "--student-model $StudentModel",
        "--teacher-api http://127.0.0.1:$LlamaServerPort",
        "--teacher-gguf `"$TeacherGGUF`"",
        "--config `"$ConfigPath`"",
        "--training-data `"$DataExportPath`"",
        "--output-dir `"$OutputDir`"",
        "--epochs $Epochs",
        "--learning-rate $LearningRate",
        "--device $Device",
        "--seed 42"
    ) -join " "

    Write-Host "  Command: $TrainCmd"

    if (-not $DryRun) {
        & python scripts/train_bonsai_mobile.py `
            --student-model $StudentModel `
            --teacher-api "http://127.0.0.1:$LlamaServerPort" `
            --teacher-gguf "$TeacherGGUF" `
            --config "$ConfigPath" `
            --training-data "$DataExportPath" `
            --output-dir "$OutputDir" `
            --epochs $Epochs `
            --learning-rate $LearningRate `
            --device $Device `
            --seed 42

        if ($LASTEXITCODE -ne 0) {
            Write-Error "Training failed"
            exit 1
        }

        Write-Host "  Training logs: $OutputDir"
    }
}

# ── Phase 4: Quantization ─────────────────────────────────────────────────

if (-not $SkipQuantization) {
    Write-Header "Phase 4: Quantization to GGUF"

    $FinalModel = Join-Path $OutputDir "final_model"
    if (-not (Test-Path $FinalModel)) {
        Write-Error "Final model not found: $FinalModel"
        exit 1
    }

    Write-Step "Quantizing to GGUF (Q4_K_M)..."

    $QuantCmd = @(
        "python scripts/quantize_bonsai_mobile.py",
        "--final-model `"$FinalModel`"",
        "--output-dir `"$OutputDir/gguf`"",
        "--model-name bonsai-mobile-v1",
        "--quantization Q4_K_M",
        "--validate",
        "--create-bkp"
    ) -join " "

    Write-Host "  Command: $QuantCmd"

    if (-not $DryRun) {
        & python scripts/quantize_bonsai_mobile.py `
            --final-model "$FinalModel" `
            --output-dir "$OutputDir/gguf" `
            --model-name "bonsai-mobile-v1" `
            --quantization Q4_K_M `
            --validate `
            --create-bkp

        if ($LASTEXITCODE -ne 0) {
            Write-Error "Quantization failed"
            exit 1
        }

        $GGUFPath = Join-Path "$OutputDir/gguf" "bonsai-mobile-v1.gguf"
        if (Test-Path $GGUFPath) {
            $GGUFSize = (Get-Item $GGUFPath).Length / 1MB
            Write-Host "  GGUF: $GGUFPath ($([math]::Round($GGUFSize, 1)) MB)"
        }
    }
}

# ── Phase 5: Benchmarking ─────────────────────────────────────────────────

if (-not $SkipBenchmark) {
    Write-Header "Phase 5: Performance Benchmarking"

    $GGUFPath = Join-Path "$OutputDir/gguf" "bonsai-mobile-v1.gguf"
    if (-not (Test-Path $GGUFPath)) {
        Write-Warning "GGUF not found, skipping benchmark"
    } else {
        Write-Step "Benchmarking model..."

        $BenchCmd = @(
            "python scripts/benchmark_bonsai_mobile.py",
            "--model `"$GGUFPath`"",
            "--output-dir `"$OutputDir/benchmark`"",
            "--num-prompts 50",
            "--device $Device"
        ) -join " "

        Write-Host "  Command: $BenchCmd"

        if (-not $DryRun) {
            & python scripts/benchmark_bonsai_mobile.py `
                --model "$GGUFPath" `
                --output-dir "$OutputDir/benchmark" `
                --num-prompts 50 `
                --device $Device

            Write-Host "  Results: $OutputDir/benchmark"
        }
    }
}

# ── Phase 6: Model Registration ───────────────────────────────────────────

if (-not $SkipRegister) {
    Write-Header "Phase 6: Model Registry"

    $GGUFPath = Join-Path "$OutputDir/gguf" "bonsai-mobile-v1.gguf"
    if (-not (Test-Path $GGUFPath)) {
        Write-Warning "GGUF not found, skipping registration"
    } else {
        Write-Step "Registering model..."

        $ReleaseDir = Join-Path $env:USERPROFILE ".bonsai\models\releases"
        New-Item -ItemType Directory -Path $ReleaseDir -Force | Out-Null

        # Copy GGUF to releases
        Copy-Item $GGUFPath -Destination $ReleaseDir -Force
        Write-Host "  Registered: $ReleaseDir\bonsai-mobile-v1.gguf"

        # Create registry entry
        $RegistryEntry = @{
            name = "bonsai-mobile-v1"
            role = "student_mobile"
            gguf = "$ReleaseDir\bonsai-mobile-v1.gguf"
            vram_gb = 1
            domains = @("coding", "system_repair", "tool_use", "chat", "qa")
            context_len = 2048
            quantisation = "Q4_K_M"
            created = Get-Date -Format "o"
        }

        $RegistryPath = "$ReleaseDir\bonsai-mobile-v1.metadata.json"
        $RegistryEntry | ConvertTo-Json | Out-File $RegistryPath -Encoding UTF8
        Write-Host "  Metadata: $RegistryPath"
    }
}

# ── Cleanup ──────────────────────────────────────────────────────────────

Write-Header "Pipeline Complete"

if ($TeacherPID) {
    Write-Step "Stopping teacher server (PID: $TeacherPID)..."
    Stop-Process -Id $TeacherPID -ErrorAction SilentlyContinue
    Write-Host "  Stopped"
}

Write-Step "Summary:"
Write-Host "  Output directory: $OutputDir"
Write-Host "  Final model: $OutputDir/final_model"
Write-Host "  GGUF: $OutputDir/gguf/bonsai-mobile-v1.gguf"
Write-Host "  Benchmark results: $OutputDir/benchmark"
Write-Host "  Training logs: $OutputDir/train_*.log"

Write-Host "`n" -NoNewline
Write-Host ("=" * 80) -ForegroundColor Cyan
Write-Host "BonsAI Mobile Training Pipeline Complete!" -ForegroundColor Green
Write-Host ("=" * 80) -ForegroundColor Cyan
Write-Host ""

exit 0
