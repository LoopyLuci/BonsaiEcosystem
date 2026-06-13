# BonsAI Training Guide

Everything you need to train BonsAI to be flawless at every task, using all of
the models already on your machine.

---

## Architecture Overview

The training system already running inside Bonsai Workspace:

```
User Interaction (chat / tool use / feedback)
         ↓
 UnifiedTrainingCollector  ← OmnipresentCapture
  7 curation stages: PII scrub → dedup → quality score →
  domain classify → stratify → threshold → typed buffers
         ↓
  EternalTrainingLoop  (runs every 5 min, background)
    ├─ SelfPlayTrainer    → constitutional pairs, adversarial probes
    ├─ ReasoningTrainer   → deduction/induction/abduction self-improvement
    ├─ ForgettingPrevention → checks no regression before each update
    └─ PromotionGate (6 stages) → only promotes adapters that pass all gates
         ↓
  Trainer.run() → finetune.py  (offline LoRA, PyTorch)
         ↓
  ModelOrchestrator  (hot-swap adapter, no restart needed)
         ↓
  EvaluationHarness  (12 dimensions, CIQ score)
         ↓
  RollbackMonitor  (auto-rollback if quality drops >2%)
```

New training scripts added in this session:

| Script | Purpose |
|--------|---------|
| `scripts/export_training_data.sh` / `.ps1` | Merge all data sources into one JSONL |
| `runtimes/bonsai-trainer/dpo_train.py` | DPO preference optimisation |
| `runtimes/bonsai-trainer/distill.py` | Knowledge distillation (teacher→student) |
| `runtimes/bonsai-trainer/mlx_train.sh` | Apple Silicon native training (MLX) |
| `config/training.yaml` | All training hyperparameters and model paths |

---

## Quick Start

### MacBook Pro M1 (recommended path)

```bash
# Prerequisites (one-time)
pip install mlx-lm
# Download base model (internet required, one-time only, ~3 GB)
huggingface-cli download Qwen/Qwen2.5-1.5B-Instruct

# Full training cycle: export data + SFT + fuse + GGUF
just train-full-mlx

# Or step by step:
just export-data       # Merge chat, survival, tool traces → JSONL
just train-mlx         # SFT with MLX (native Apple Silicon, fast)
just evaluate          # Measure all 12 dimensions (app must be running)
```

MLX runs natively on the M1 unified memory architecture — no VRAM limit, much
faster than PyTorch on Metal. A 600-iteration SFT on 1.5B takes ~10 minutes on M1 Pro.

### Windows / AMD 7900 XTX

```powershell
# Prerequisites
pip install transformers peft datasets torch

# Full training cycle
just export-data
just train          # PyTorch SFT (CPU fallback — AMD ROCm not supported on Windows)
just train-dpo      # DPO preference pairs
just evaluate
```

For AMD GPU acceleration on Windows: the DirectML backend crashes on transformer
backward pass (unsupported aten ops). Use CPU training or switch to WSL2 + ROCm.

### Linux / NVIDIA CUDA

```bash
pip install transformers peft datasets torch
just export-data && just train && just train-dpo
```

---

## Training Stages in Detail

### Stage 1 — Data Export

`just export-data` merges five sources:

| Source | Location | Description |
|--------|----------|-------------|
| Curated baseline | `bonsai-workspace/data/bonsai_core/` | High-quality hand-curated examples |
| Chat sessions | `~/.bonsai/chat_sessions.db` | Real user conversations (PII scrubbed) |
| Survival fixes | `~/.bonsai/survival_kb.db` | Error→fix mappings, verified fixes only |
| Cross-training | `~/.bonsai/data/cross_training.jsonl` | Plugin/tool interaction traces |
| Unified collector | `~/.bonsai/data/unified_collector.jsonl` | All interaction events |

Output: `~/.bonsai/training_export/bonsai_merged_latest.jsonl`

Domain mix target (auto-stratified by the collector):
- 40% coding (Rust, Python, Svelte, TypeScript)
- 20% system repair (error → fix script)
- 20% tool use (function calling, Tauri commands)
- 10% creative (documentation, image prompts)
- 10% general (greetings, IDE support)

### Stage 2 — SFT (Supervised Fine-Tuning)

Teaches the model to follow instructions, use tools, and produce safe outputs.

**Config** (`config/training.yaml`):
- LoRA rank: 16, alpha: 32
- Epochs: 3, LR: 2e-4, batch: 1, grad_accum: 8
- Trigger: every 10,000 new high-quality examples

**PyTorch (finetune.py)**:
```bash
python runtimes/bonsai-trainer/finetune.py \
    --gguf ~/.bonsai/models/bonsai-latest.gguf \
    --data ~/.bonsai/training_export/bonsai_merged_latest.jsonl \
    --output ~/.bonsai/adapters/bonsai-sft-v3
```

**MLX (mlx_train.sh, M1 only)**:
```bash
bash runtimes/bonsai-trainer/mlx_train.sh \
    --model Qwen/Qwen2.5-1.5B-Instruct \
    --iters 600
```

### Stage 3 — DPO (Direct Preference Optimisation)

Aligns the model with user preferences (style, verbosity, tone) without a
separate reward model. Trains on (prompt, chosen, rejected) triples.

DPO pairs are generated automatically from:
- User thumbs-up/down feedback
- User edits to assistant messages (original = rejected, edited = chosen)
- Constitutional self-play (high-score = chosen, low-score = rejected)

```bash
just export-data   # also produces bonsai_dpo_latest.jsonl
just train-dpo
# or: bash runtimes/bonsai-trainer/mlx_train.sh --dpo
```

### Stage 4 — Knowledge Distillation

Transfers capabilities from a LARGE teacher model (e.g., Qwen3-35B) to the
small student model (Bonsai-1.7B) without increasing inference cost.

**Recommended workflow (teacher as GGUF sidecar)**:
```bash
# Start the teacher (large model) as a llama-server sidecar
llama-server -m /path/to/Qwen3-35B-A22B-Q4_K_M.gguf -ngl 99 --port 8080

# In another terminal, run distillation
just distill
# or:
python runtimes/bonsai-trainer/distill.py \
    --student-model ~/.cache/huggingface/hub/.../snapshots/latest \
    --teacher-api http://127.0.0.1:8080 \
    --prompts ~/.bonsai/training_export/distill_prompts.txt \
    --output ~/.bonsai/adapters/bonsai-distilled-v1
```

The teacher generates completions; the student learns to mimic them. On M1, use
`mlx_train.sh --distill --teacher Qwen/Qwen2.5-14B-Instruct` instead.

### Stage 5 — Self-Play (Continuous, Background)

Already running inside the app. Every 5 minutes, `EternalTrainingLoop`:
1. Generates responses to 20 seed prompts
2. Scores them against the 3-tier constitution (9 rules)
3. Violations become DPO pairs (chosen = corrected, rejected = original)
4. Adversarial probes test jailbreak resistance

Trigger manually: `just training-stats` shows current state.

### Stage 6 — Evaluation

`EvaluationHarness` measures 12 dimensions continuously:

| Dimension | Metric | Target |
|-----------|--------|--------|
| Code Generation | pass@1 | >90% |
| Code Review | vulnerability detection | >85% |
| Tool Selection | correct tool rate | >95% |
| Planning | multi-step completeness | >80% |
| Conversational Quality | user satisfaction | >95% |
| Safety & Refusal | refusal accuracy | 100% unsafe, <0.1% safe |
| Music Generation | prompt adherence | >80% |
| Vision Understanding | detection mAP | >75% |
| Document Understanding | OCR accuracy | >90% |
| Voice & Audio | transcription WER | <10% |
| Memory & Recall | retrieval relevance | >85% |
| Swarm Collaboration | decomposition quality | >80% |

The **PromotionGate** runs a 6-stage check before any adapter goes live:
1. Technical validity (loads, no NaN)
2. Core competency (50-prompt safety check — any failure = discard)
3. Domain benchmark (regression ≤ 3%, improvement ≥ 1%)
4. Adversarial robustness (all jailbreaks refused)
5. Resource efficiency (latency ≤ +5%, VRAM ≤ +2%)
6. Shadow testing (wins ≥ 65% of comparisons)

---

## Using All Local Models

Edit `config/training.yaml` to map your local models:

```yaml
student:
  gguf: "~/.bonsai/models/Bonsai-1.7B-Q4_K.gguf"

teacher:
  gguf: "D:/Models/general/Qwen3-35B-A22B-Q4_K_M.gguf"
  llama_server_port: 8080

local_models:
  - role: teacher
    name: "Qwen3-35B"
    gguf: "D:/Models/general/Qwen3-35B-A22B-Q4_K_M.gguf"
    vram_gb: 22
  - role: student
    name: "Bonsai-1.7B-Q4_K"
    gguf: "~/.bonsai/models/Bonsai-1.7B-Q4_K.gguf"
    vram_gb: 3
```

The `ModelOrchestrator` manages loading/unloading models to stay within VRAM budget.
Set `hardware.max_vram_gb` to your GPU's capacity.

---

## Continuous Learning Loop

Once set up, BonsAI improves automatically:

| Trigger | Action | Adapter rank |
|---------|--------|-------------|
| Every 100 interactions | Online LoRA update from feedback | r=8 |
| Every 5 minutes | Self-play round (EternalTrainingLoop) | r=16 |
| When 500 DPO pairs accumulate | DPO fine-tune | r=16 |
| When 10,000 SFT examples accumulate | Full SFT cycle | r=32 |
| Weekly (manual or CI) | Distillation from teacher | r=16 |

The CI workflow (`.github/workflows/survival-system.yml`) runs `cargo test` and
the watchdog unit tests on every push. Add a scheduled workflow to run
`just export-data && just train` weekly.

---

## Troubleshooting

**"No local base model found for GGUF arch"**
→ Download the base model (one-time, internet required):
```bash
huggingface-cli download Qwen/Qwen2.5-1.5B-Instruct
```

**"Out of memory during training"**
→ Reduce batch size or grad_accum in `config/training.yaml`.
→ The Survival System will detect this and auto-retry with smaller batch.

**"DirectML crashes on backward pass" (AMD Windows)**
→ Known limitation. Use `--backend cpu` or switch to WSL2 + ROCm.

**MLX training on M1: "mlx-lm not installed"**
```bash
pip install mlx-lm
```

**DPO data is empty**
→ Run `just export-data` first — it extracts preference pairs from high-rated chats.
→ Use Bonsai for a while and rate responses (thumbs up/down) to build up pairs.
