# Bonsai Workspace — common tasks
# Install: cargo install just
# Usage:   just <recipe>
#
# Works on Windows (PowerShell), macOS, and Linux.
# On Windows, recipes that use shell scripts fall back to PowerShell equivalents.

workspace_root := justfile_directory()
os := os()

# List available recipes
default:
    @just --list

# ── Build ─────────────────────────────────────────────────────────────────────

# Build for the current platform (recommended)
build:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/build/build-all.ps1"
    else
        bash "{{workspace_root}}/scripts/build/build-all.sh"
    fi

# Build macOS universal binary (arm64 + x86_64, macOS only)
build-universal:
    bash "{{workspace_root}}/scripts/build/build-all.sh" --universal

# Build including survival watchdog
build-with-watchdog:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/build/build-all.ps1" -Watchdog
    else
        bash "{{workspace_root}}/scripts/build/build-all.sh" --watchdog
    fi

# Build only the survival watchdog binary
build-watchdog:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/build/Build-Watchdog.ps1"
    else
        cargo build --release \
            --manifest-path "{{workspace_root}}/crates/bonsai-watchdog/Cargo.toml"
    fi

# ── Launch ────────────────────────────────────────────────────────────────────

# Launch full Bonsai Ecosystem (IDE + Buddy chat window)
launch:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/launch/Launch-Ecosystem.ps1"
    else
        bash "{{workspace_root}}/scripts/launch/launch-ecosystem.sh"
    fi

# Launch Bonsai Workspace IDE only
launch-workspace:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/launch/Launch-Workspace.ps1"
    else
        bash "{{workspace_root}}/scripts/launch/launch-workspace.sh"
    fi

# Launch Bonsai Buddy chat window only
launch-buddy:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/launch/Launch-Buddy.ps1"
    else
        bash "{{workspace_root}}/scripts/launch/launch-buddy.sh"
    fi

# Start the Tauri dev server with HMR (workspace mode)
dev:
    cd bonsai-workspace/src && npx tauri dev

# Start the Tauri dev server in buddy mode
dev-buddy:
    cd bonsai-workspace/src && BONSAI_LAUNCH_MODE=buddy npx tauri dev

# Start the Tauri dev server in ecosystem mode
dev-ecosystem:
    cd bonsai-workspace/src && BONSAI_LAUNCH_MODE=ecosystem npx tauri dev

# Start the headless daemon
daemon:
    cargo run -p bonsai-daemon

# ── Test ──────────────────────────────────────────────────────────────────────

# Run all Rust workspace tests
test:
    cargo test --workspace

# Run watchdog tests only
test-watchdog:
    cargo test --manifest-path crates/bonsai-watchdog/Cargo.toml -- --nocapture

# Run frontend tests
test-frontend:
    npm --prefix bonsai-workspace/src run test

# Run integration tests
test-integration:
    python tests/integration/test_daemon_local.py

# ── Lint / Check ──────────────────────────────────────────────────────────────

# Fast workspace check (no codegen)
check:
    cargo check --workspace

# Clippy + fmt check + frontend lint
lint:
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    npm --prefix bonsai-workspace/src run lint

# ── Training ──────────────────────────────────────────────────────────────────

# Step 1: Export all training data from every source into ~/.bonsai/training_export/
export-data:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/export_training_data.ps1"
    else
        bash "{{workspace_root}}/scripts/export_training_data.sh"
    fi

# Step 2a: SFT fine-tune the student model (PyTorch, all platforms)
# Requires: pip install transformers peft datasets torch
# On M1 use 'just train-mlx' instead (10x faster)
train:
    #!/usr/bin/env sh
    EXPORT="$HOME/.bonsai/training_export/bonsai_merged_latest.jsonl"
    BASELINE="{{workspace_root}}/bonsai-workspace/data/bonsai_core/bonsai_core_train_v2.jsonl"
    DATA="${EXPORT:-$BASELINE}"
    OUT="$HOME/.bonsai/adapters/bonsai-sft-$(date +%Y%m%d)"
    python3 "{{workspace_root}}/bonsai-workspace/runtimes/bonsai-trainer/finetune.py" \
        --data "$DATA" \
        --output "$OUT"

# Step 2b: SFT fine-tune using Apple MLX (M1/M2/M3 — native, fast, recommended on Mac)
# Requires: pip install mlx-lm
# Also runs DPO and can distill from a larger teacher model.
train-mlx:
    bash "{{workspace_root}}/bonsai-workspace/runtimes/bonsai-trainer/mlx_train.sh"

# Step 2c: DPO preference optimisation (requires preference pairs from export-data)
train-dpo:
    #!/usr/bin/env sh
    DPO="$HOME/.bonsai/training_export/bonsai_dpo_latest.jsonl"
    MODEL_DIR="$(python3 -c \"import os,glob; h=os.path.expanduser('~/.cache/huggingface/hub'); snaps=glob.glob(h+'/models--Qwen--**/snapshots/*/config.json',recursive=True); print(os.path.dirname(snaps[0])) if snaps else print('')\" 2>/dev/null)"
    OUT="$HOME/.bonsai/adapters/bonsai-dpo-$(date +%Y%m%d)"
    if [ -z "$MODEL_DIR" ]; then echo "No local HF model found. Run: huggingface-cli download Qwen/Qwen2.5-1.5B-Instruct"; exit 1; fi
    python3 "{{workspace_root}}/bonsai-workspace/runtimes/bonsai-trainer/dpo_train.py" \
        --base-model "$MODEL_DIR" --data "$DPO" --output "$OUT"

# Step 2d: Knowledge distillation (teacher → student, teacher runs via llama-server)
# Start teacher first: llama-server -m /path/to/large-model.gguf --port 8080
distill:
    #!/usr/bin/env sh
    MODEL_DIR="$(python3 -c \"import os,glob; h=os.path.expanduser('~/.cache/huggingface/hub'); snaps=glob.glob(h+'/models--Qwen--**/snapshots/*/config.json',recursive=True); print(os.path.dirname(snaps[0])) if snaps else print('')\" 2>/dev/null)"
    PROMPTS="$HOME/.bonsai/training_export/distill_prompts.txt"
    OUT="$HOME/.bonsai/adapters/bonsai-distilled-$(date +%Y%m%d)"
    python3 "{{workspace_root}}/bonsai-workspace/runtimes/bonsai-trainer/distill.py" \
        --student-model "$MODEL_DIR" \
        --teacher-api "http://127.0.0.1:8080" \
        --prompts "$PROMPTS" --output "$OUT"

# Full training cycle: export → SFT → DPO (use train-mlx on M1 instead)
train-full:
    just export-data
    just train
    just train-dpo

# Full training cycle for Apple Silicon (M1/M2/M3)
train-full-mlx:
    just export-data
    just train-mlx

# Step 3: Evaluate the latest adapter against all 12 dimensions
# (triggers EvaluationHarness via Tauri — app must be running)
evaluate:
    #!/usr/bin/env sh
    curl -s -X POST http://127.0.0.1:11369/api/training/evaluate \
        -H "Content-Type: application/json" \
        -d '{"run_full": true}' | python3 -m json.tool || \
    echo "App not running. Launch Bonsai Workspace first, then re-run 'just evaluate'."

# Step 4: Deploy the latest trained adapter (copies to ~/.bonsai/models/bonsai-latest.gguf)
# The app must be restarted after deploying.
deploy-model ADAPTER_PATH="":
    #!/usr/bin/env sh
    if [ -z "{{ADAPTER_PATH}}" ]; then
        ADAPTER_PATH="$(ls -dt $HOME/.bonsai/adapters/bonsai-* 2>/dev/null | head -1)"
    fi
    if [ -z "$ADAPTER_PATH" ]; then echo "No adapter found. Run 'just train' first."; exit 1; fi
    echo "Deploying adapter: $ADAPTER_PATH"
    curl -s -X POST http://127.0.0.1:11369/api/training/deploy \
        -H "Content-Type: application/json" \
        -d "{\"adapter_path\": \"$ADAPTER_PATH\"}" | python3 -m json.tool || \
    echo "App not running — copy adapter manually to ~/.bonsai/models/bonsai-latest.gguf"

# Show training statistics from the running app
training-stats:
    curl -s http://127.0.0.1:11369/api/training/stats | python3 -m json.tool 2>/dev/null || \
    echo "App not running."

# ── Release ───────────────────────────────────────────────────────────────────

# Tag and push a release: just release VERSION=v0.2.1
release VERSION="":
    #!/usr/bin/env sh
    if [ -z "{{VERSION}}" ]; then echo "Usage: just release VERSION=v0.x.y"; exit 1; fi
    git tag -a "{{VERSION}}" -m "{{VERSION}}"
    git push origin "{{VERSION}}"
    gh release create "{{VERSION}}" --title "{{VERSION}}" --notes-file CHANGELOG.md
