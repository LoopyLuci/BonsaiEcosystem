# BonsAI Sovereignty Speedrun Plan
## AFAP (As Fast As Possible) — AI-Driven Parallel Execution

**Version:** 1.0  
**Target:** Complete the 50-crate, 214k-LOC sovereignty plan in **8–12 months** instead of 48  
**Method:** Parallel AI agent assembly line + a dedicated Training Agent model that continuously improves itself and the rest of the ecosystem

---

## Part 1: Parallel AI Agent Assembly Line

### 1.1 Execution Model

Every crate in the sovereignty plan is built by a *crate factory* — an AI agent that takes a specification document and iterates until the crate compiles, tests pass, and the replaced dependency is removed from every `Cargo.toml`. Multiple factories run concurrently, each on its own git worktree branch.

```
docs/specs/bonsai-X.md   ──►  [Agent: Generate]  ──►  crates/bonsai-X/src/
                               [Agent: Test]       ──►  crates/bonsai-X/tests/
                               [Agent: Integrate]  ──►  Cargo.toml patches
                               [Agent: Verify]     ──►  cargo check --workspace
                               [CI merge gate]     ──►  main
```

Five agent streams run in parallel, each responsible for one domain:

| Stream | Domain | Phase 1 Target Crates | Phase 2+ |
|--------|--------|----------------------|---------|
| **A — Foundation** | errors, logging, IDs, time, random, codec | `bonsai-error`, `bonsai-log`, `bonsai-rand`, `bonsai-time`, `bonsai-id`, `bonsai-codec` | `bonsai-cli`, `bonsai-text` |
| **B — Crypto & Security** | all cryptographic primitives | `bonsai-crypto` | `bonsai-tls`, `bonsai-auth` |
| **C — Network & Protocol** | HTTP, WebSocket, P2P, database | `bonsai-net`, `bonsai-http`, `bonsai-websocket`, `bonsai-db` | `bonsai-p2p` expansion, bot clients |
| **D — Content & Platform** | media, text processing, OS integration | `bonsai-image`, `bonsai-markup`, `bonsai-fs`, `bonsai-native` | `bonsai-audio`, `bonsai-dom`, `bonsai-regex` |
| **E — AI & Runtime** | tensor ops, WASM, async executor | `bonsai-tensor` | `bonsai-wasm`, `bonsai-executor` |

### 1.2 Crate Factory Script

[scripts/generate_crate.ps1](../scripts/generate_crate.ps1) is the entrypoint for each factory run. For each crate:

```
1. Read docs/specs/<crate>.md
2. Call teacher model (Qwen3-35B via llama-server) with the spec
3. Write generated code to crates/<crate>/src/
4. Run: cargo check -p <crate>
5. If errors → feed error output back to teacher with context → retry (max 20 iters)
6. Run: cargo test -p <crate>
7. If test failures → same feedback loop
8. Run: cargo clippy -p <crate> -- -D warnings
9. Patch every Cargo.toml that referenced the old dependency
10. Run: cargo check --workspace
11. Commit to branch feat/sovereignty-<crate>
```

This script is called by Claude Code agents via the `bonsai-coordinator` work queue. Each iteration calls the teacher inference server, which is already running on the RX 7900 XTX.

### 1.3 Spec Template

Every crate starts with a spec in `docs/specs/<crate>.md`:

```markdown
# Crate Spec: bonsai-X

## Replaces
`dep-a`, `dep-b`

## API Surface
```rust
// Key public types and functions
pub struct Foo { ... }
pub fn bar(x: &Foo) -> Result<Baz, BonsaiError>;
```

## Invariants
- Invariant 1: ...
- Invariant 2: ...

## Performance Target
- bar() must complete in < Xµs for typical inputs

## Test Vectors
- Input A → Output B (from reference implementation / RFC)

## Migration Notes
Replace all call sites: `old_crate::Foo` → `bonsai_x::Foo`
```

---

## Part 2: The Training Agent

The Training Agent is a fine-tuned language model that understands ML training theory, debugging, hyperparameter optimization, and data pipeline engineering at expert level. It is the keystone of the AFAP strategy: once built, it autonomously manages training runs for all other BonsAI models, freeing human attention for architecture decisions and code review.

### 2.1 Role and Capabilities

The Training Agent must be able to:

1. **Read training logs** and diagnose failure modes: loss spikes, gradient explosion/vanishing, NaN propagation, mode collapse, underfitting/overfitting, data imbalance, tokenizer mismatches
2. **Prescribe and apply fixes** autonomously: adjust learning rate, change warmup schedule, switch optimiser, reduce batch size, add gradient clipping, modify LoRA rank, swap data mix ratio
3. **Design training recipes** from scratch for any model architecture and dataset, following best practices for the specific combination (e.g., QLoRA vs. full fine-tune, DPO vs. RLHF vs. SFT for a given alignment task)
4. **Generate synthetic training data** by calling the teacher inference server with carefully crafted prompts, then filtering and deduplicating outputs
5. **Orchestrate multi-phase training pipelines**: sequence SFT → DPO → distillation → GGUF conversion → evaluation, monitor each phase, and gate the next phase on quality thresholds
6. **Evaluate model quality** on held-out test sets, report Tool F1, perplexity, BLEU, and domain-specific metrics, and compare adapter versions
7. **Write and modify training scripts** (`dpo_train.py`, `finetune_sft.py`, `distill.py`) to add new features or fix bugs found during runs
8. **Interface with the BonsAI tool registry** as a first-class tool, callable from the Model Trainer UI via `train_model(spec_path)`

### 2.2 Base Model Selection

The Training Agent is built on top of a strong reasoning base. Selection criteria: strong instruction following, code understanding, mathematical reasoning, long context (≥32k tokens for reading full training logs).

**Primary choice: Qwen2.5-14B-Instruct**
- 14B parameters — fits in CPU RAM (28 GB fp16, ~14 GB with Q4_K_M quantisation)
- Strong math and code reasoning (top performer on HumanEval, MATH benchmarks)
- Long context: 32k tokens native (enough for full training log + model card)
- Already on-disk at `D:/Models/` if the weekly_train pipeline has run
- Fine-tuning on CPU with QLoRA/LoRA: feasible in ~24–72h per phase

**Fallback: Qwen2.5-7B-Instruct**
- 7B parameters — ~4 GB Q4_K_M, much faster iteration
- Slightly weaker reasoning but adequate for hyperparameter tuning tasks
- Use this for the pilot run to validate the data pipeline

**Long-term target: DeepSeek-R1-Distill-Qwen-32B**
- 32B — chain-of-thought reasoning, best for complex multi-step training decisions
- Too large for CPU fine-tuning; use GPU distillation once Phase 2 infra is ready

### 2.3 Training Agent Dataset — Comprehensive Design

The dataset is the most important part. A mediocre dataset produces a mediocre agent. Every source below is included because it teaches a specific, non-overlapping capability.

#### Tier 1: Core ML Theory (What to Do and Why)

**Source 1 — ML Textbooks (formatted Q&A)**
Extract knowledge from the canonical texts as instruction-response pairs:

- *Deep Learning* (Goodfellow, Bengio, Courville) — architecture design, optimisation theory, regularisation
- *Pattern Recognition and Machine Learning* (Bishop) — probabilistic foundations, Bayesian inference
- *The Elements of Statistical Learning* (Hastie et al.) — generalisation, bias-variance
- *Speech and Language Processing* (Jurafsky & Martin) — language model fundamentals
- *Reinforcement Learning: An Introduction* (Sutton & Barto) — RL fundamentals for RLHF

Format: `{"instruction": "Explain the role of the warmup schedule in transformer training", "response": "..."}`

Extraction method: PDF → OCR → section chunking → teacher (Qwen3-35B) rewrites each chunk as a clean Q&A pair. Estimated yield: ~40,000 instruction pairs.

**Source 2 — Research Papers (key findings)**
Target papers from NeurIPS, ICML, ICLR, ACL (2019–2025) in these categories:
- Training stability: gradient clipping, layer normalisation, residual connections
- Fine-tuning: LoRA, QLoRA, prefix tuning, adapters, full fine-tune best practices
- Alignment: RLHF, DPO, SPIN, ORPO, SimPO, KTO
- Data: data quality filtering, deduplication, synthetic data generation, curriculum learning
- Distillation: KL-divergence distillation, sequence-level distillation, speculative decoding

Format: Each paper → abstract + methods section → teacher-rewritten as "Given [problem], what does the literature recommend? [answer citing key results]"

Estimated yield: ~20,000 instruction pairs from ~2,000 papers.

**Source 3 — Framework Documentation (usage patterns)**
Full documentation from:
- PyTorch official docs (all APIs used in training: `optim`, `autograd`, `cuda`, `amp`)
- Hugging Face `transformers` training guide + `peft` library docs
- `trl` library (SFTTrainer, DPOTrainer, PPOTrainer)
- `axolotl` configuration reference
- `unsloth` optimisation notes
- `llama.cpp` / `llama-server` API

Format: `{"instruction": "How do I use gradient checkpointing in HuggingFace Transformers?", "response": "..."}`

Estimated yield: ~8,000 instruction pairs.

#### Tier 2: Failure Pattern Library (What Not to Do, and Fixes)

**Source 4 — Error Databases**

Collected from:
- Stack Overflow: all questions tagged `pytorch`, `transformers`, `fine-tuning`, `qlora` with accepted answers and ≥5 upvotes — ~15,000 Q&A
- GitHub Issues: `huggingface/transformers`, `huggingface/peft`, `ggerganov/llama.cpp` — extract bug reports with resolution comments — ~5,000 pairs
- BonsAI Survival KB: existing 35+ entries plus every new entry added during the sovereignty build — small but highly project-specific
- WandB / MLflow public runs: extract (hyperparameter config, final metrics, loss curve shape) triples and ask teacher to annotate whether the run succeeded or failed and why

Format (failure → diagnosis → fix):
```json
{
  "instruction": "My DPO training loss spiked to NaN after epoch 1. Logs: [loss=0.45, loss=0.21, loss=NaN]. Config: beta=0.5, lr=1e-4, batch=4",
  "response": "This is a gradient explosion caused by too-high beta combined with a high learning rate. The DPO loss gradient scales with beta; at beta=0.5 with lr=1e-4, the update step can overflow float16. Fix: reduce beta to 0.1-0.15, reduce lr to 2e-5, add gradient clipping at 1.0, and optionally switch to float32 accumulation."
}
```

Estimated yield: ~25,000 failure-fix pairs.

**Source 5 — Bonsai's Own Training Logs (Project-Specific)**

Every training run on this machine produces structured logs (`[dpo]`, `[epoch]`, `[progress]`, `[train]`). Accumulated over time these are converted to training examples:

```json
{
  "context": "Phase: safety_dpo. Config: beta=0.15, epochs=3, lr=5e-5, max_length=128, model=Qwen2.5-0.5B. Hardware: CPU Ryzen 5900X.",
  "logs": "[epoch] epoch=1 avg_loss=0.2844\n[epoch] epoch=2 avg_loss=0.0201\n[epoch] epoch=3 avg_loss=0.0022\n[train] status=complete steps=150 elapsed=418s",
  "diagnosis": "Successful run. Loss converged cleanly. max_length=128 was essential to avoid ACCESS_VIOLATION on Windows CPU allocator (default 512 caused segfault).",
  "recommendation": "For this hardware (Windows CPU), always use max_length ≤ 128 with two model copies in RAM. At 0.5B parameters, 50 pairs, 3 epochs: expect ~420s total, final loss ~0.002."
}
```

This is the highest-value data in the dataset because it is specific to the exact hardware, OS, model sizes, and failure modes encountered in this project. Start collecting immediately; every future training run adds to it.

#### Tier 3: Synthetic Preferences (What Good Looks Like vs. Bad)

**Source 6 — DPO Preference Pairs for Training Decisions**

Generated by the teacher (Qwen3-35B) given a training scenario and asked to produce:
- `chosen`: the correct training decision with sound reasoning
- `rejected`: a plausible but wrong decision with subtle errors

Example scenario: "Training a 1.5B model for code generation. After 1 epoch the validation loss stopped improving. What should you do?"
- Chosen: "Implement early stopping with patience=2 and reduce LR by 0.5x via a ReduceLROnPlateau scheduler. Also check if the validation set is representative — if it was accidentally constructed from training distribution the plateau is misleading."
- Rejected: "Immediately increase learning rate 10× to escape the plateau."

Generate 10,000 such pairs covering:
- Hyperparameter tuning decisions (LR, batch size, gradient accumulation, warmup)
- Architecture choices (LoRA rank, target modules, dropout)
- Data decisions (data mix ratio, max sequence length, deduplication strategy)
- Hardware-specific decisions (CPU vs GPU, memory management, batch size selection)
- Debugging decisions (what to change when loss is NaN, stuck, oscillating, diverging)

**Source 7 — Code Generation for Training Scripts**

Pairs of (training task description → correct Python script):
```json
{
  "instruction": "Write a DPO training script for Qwen2.5-0.5B on CPU with max_length=128 to avoid Windows OOM.",
  "response": "#!/usr/bin/env python3\n...[correct script with all the fixes we've learned]..."
}
```

The "correct" scripts are exactly the fixed versions of `dpo_train.py`, `finetune_sft.py`, `distill.py` as they exist in this repo, plus variations. This teaches the agent to reproduce and extend our exact patterns.

Estimated yield: ~5,000 script generation pairs.

#### Dataset Summary

| Source | Type | Est. Pairs | Priority |
|--------|------|-----------|---------|
| ML Textbooks | Theory Q&A | 40,000 | High |
| Research Papers | Findings Q&A | 20,000 | High |
| Framework Docs | Usage Q&A | 8,000 | Medium |
| Error Databases | Failure→Fix | 25,000 | Critical |
| Bonsai Training Logs | Project-specific logs | Growing (start: ~50) | Critical |
| DPO Preference Pairs | Chosen/Rejected decisions | 10,000 | High |
| Script Generation | Code Q&A | 5,000 | High |
| **Total** | | **~108,000** | |

### 2.4 Data Collection Scripts

Three scripts handle data pipeline:

#### `scripts/collect_training_agent_data.py`
```
Purpose: Scrape and format Tier 1 sources
- Download ArXiv papers from list → PDF → text extraction → teacher rewrite to Q&A
- Parse PyTorch/HF docs (cached HTML) → teacher Q&A rewrite
- Parse Stack Overflow data dump (offline XML) → filter by tags and score
Output: ~/.bonsai/training_agent/tier1_raw.jsonl (40k-70k entries)
```

#### `scripts/generate_training_agent_dpo.py`
```
Purpose: Generate Tier 3 preference pairs using teacher
- Load scenario templates (200+ base scenarios)
- For each: call llama-server (Qwen3-35B teacher) to generate chosen + rejected
- Filter: both responses must differ; chosen must have correct reasoning per teacher judgment
Output: ~/.bonsai/training_agent/dpo_pairs.jsonl (10k entries)
```

#### `scripts/export_training_logs.py`
```
Purpose: Convert Bonsai training run logs to Source 5 format
- Reads ~/.bonsai/brain_metadata.json for completed phases
- Reads ~/.bonsai/training_export/*.log for raw logs
- Converts each run to the (context, logs, diagnosis, recommendation) format
- Appends to ~/.bonsai/training_agent/bonsai_logs.jsonl
Run: after every training session
```

### 2.5 Training Methodology — Detailed

#### Stage 0: Data Pipeline Validation (Week 1)
Before committing to a full fine-tune, validate the pipeline with the 7B model and 1,000 examples:
```
python finetune_sft.py \
  --base-model ~/.cache/huggingface/.../Qwen2.5-0.5B-Instruct/.../  \
  --data ~/.bonsai/training_agent/tier1_raw_sample.jsonl \
  --output ~/.bonsai/adapters/training-agent-pilot-v0 \
  --device cpu --epochs 1 --max-pairs 200 --max-length 128
```
Check: loss curve is smooth, no NaN, adapter saves correctly. Fix any pipeline issues at this stage with the cheap 0.5B model.

#### Stage 1: Supervised Fine-Tuning (SFT) — Weeks 2–4

**Model:** Qwen2.5-7B-Instruct (pilot) → scale to 14B once pilot validates  
**Data:** All 108,000 instruction pairs combined, shuffled, deduplicated  
**Hardware:** CPU (Ryzen 5900X) — 7B at Q4_K_M is ~3.5GB model; two copies ~7GB; well within 64GB RAM  

Hyperparameters (tuned for CPU/Windows stability based on what we know):
```python
# finetune_sft.py equivalent config
lora_r         = 64        # High rank for complex knowledge
lora_alpha     = 16        # Keep ratio r/alpha=4 for stable updates
lora_dropout   = 0.05
target_modules = ["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"]
lr             = 2e-4      # Standard QLoRA learning rate
warmup_steps   = 100       # ~1% of steps
weight_decay   = 0.01
epochs         = 3
batch_size     = 2         # CPU constraint
grad_accum     = 16        # effective batch = 32
max_length     = 256       # CPU-safe (128 is ultra-safe, 256 works for 7B)
scheduler      = "cosine"  # cosine decay to 10% of peak LR
```

Expected duration: ~108,000 pairs × 3 epochs ÷ (effective_batch=32) × (time_per_step_cpu) ≈ **48–72 hours** on the 5900X.  
Run via `scripts/weekly_train.ps1` with a new "training_agent" phase flag.

#### Stage 2: DPO Alignment — Week 5

**Data:** 10,000 preference pairs from Source 6  
**Config:**
```python
beta     = 0.1    # Conservative — agent should reason, not be overly bold
epochs   = 2
lr       = 5e-5   # Much lower than SFT
max_len  = 256
```

DPO teaches the agent to prefer sound reasoning over plausible-but-wrong answers. This is the stage that makes the difference between an agent that gives reasonable answers and one that gives correct answers.

Expected duration: ~16 hours on CPU.

#### Stage 3: Distillation from 35B Teacher — Week 6 (optional, accelerates capability)

Use `distill.py` with Qwen3-35B as teacher (already on disk at `D:/Models/general/`):
```bash
# Start teacher on GPU
llama-server -m D:/Models/general/Qwen3-35B-A22B-Q4_K_M.gguf -ngl 99 --port 8080

# Train student via soft-label distillation
python distill.py \
  --student-model ~/.cache/huggingface/.../Qwen2.5-7B-Instruct/.../ \
  --teacher-api http://127.0.0.1:8080 \
  --prompts ~/.bonsai/training_agent/distill_prompts.txt \
  --output ~/.bonsai/adapters/training-agent-distilled-v1 \
  --alpha 0.7   # 70% KL-divergence, 30% hard cross-entropy
```

The distillation prompts (1,000 complex training scenarios) force the student to learn the teacher's token-level distributions on hard reasoning problems.

#### Stage 4: GGUF Conversion and Deployment — Week 7

```powershell
# Convert to GGUF Q4_K_M (optimal quality/size for 7B)
python C:/tools/llama.cpp/convert_hf_to_gguf.py \
  ~/.bonsai/adapters/training-agent-distilled-v1 \
  --outfile D:/Models/training-agent/bonsai-training-agent-7b-q4.gguf \
  --outtype q4_k_m

# Register as a tool in ToolRegistry
# (see section 2.6)
```

#### Stage 5: Continuous Self-Improvement (Ongoing)

Every successful training run managed by the Training Agent produces a new log entry. The `export_training_logs.py` script converts it to a training example. Weekly, the Training Agent is fine-tuned on the accumulated new logs:

```
weekly_train.ps1 → run phase: training_agent_update
  - export new logs: export_training_logs.py
  - SFT fine-tune on new logs only: finetune_sft.py --data new_logs.jsonl --epochs 1
  - DPO on any new preference pairs collected this week
  - Convert to GGUF, hot-reload via Tauri deploy_adapter command
```

This creates a positive feedback loop: the more the Training Agent trains models, the more training logs accumulate, the smarter the Training Agent becomes.

### 2.6 Integration into BonsAI

#### Tool Registry Entry
Register the Training Agent as a callable tool:

```rust
// In bonsai-tool-registry
Tool {
    name: "train_model",
    description: "Autonomously execute a training pipeline. Accepts a JSON spec describing the model, dataset, hardware, and objectives. Returns job_id for monitoring.",
    parameters: schema!({
        "spec_path": { "type": "string", "description": "Path to training spec JSONL or markdown" },
        "phase": { "type": "string", "enum": ["safety", "tool_use", "code", "chat", "reason", "distill", "custom"] },
        "dry_run": { "type": "bool", "default": false }
    }),
}
```

#### Aether Actor: `TrainingAgentActor`

```rust
// In bonsai-actors / bonsai-creator
pub struct TrainingAgentActor {
    model_path: PathBuf,    // path to bonsai-training-agent-7b-q4.gguf
    inference_port: u16,    // local llama-server port for the agent itself
    job_registry: Arc<DashMap<JobId, TrainingJob>>,
}

impl TrainingAgentActor {
    // Called by ModelTrainer UI "Auto-Train" button
    pub async fn handle_train_request(&self, spec: TrainingSpec) -> JobId { ... }
    
    // Internal: watch a running job, intervene if loss diverges
    async fn supervise(&self, job: &TrainingJob) { ... }
    
    // Called by EternalTrainingLoop after each phase
    pub async fn collect_feedback(&self, job_id: JobId) -> TrainingFeedback { ... }
}
```

#### ModelTrainer UI Commands

New Tauri commands (in `bonsai-workspace/src-tauri/src/lib.rs`):

```rust
#[tauri::command]
async fn start_autonomous_training(
    state: State<'_, AppState>,
    phase: String,
    hyperparams: serde_json::Value,
) -> Result<String, String> { ... }  // returns job_id

#[tauri::command]
async fn get_training_agent_recommendation(
    state: State<'_, AppState>,
    log_snippet: String,
) -> Result<String, String> { ... }  // returns recommendation text
```

---

## Part 3: AFAP Timeline — 8-Month Speedrun

### Month 0 (This Week): Infrastructure

| Day | Task | Owner |
|-----|------|-------|
| 1 | `cargo vendor vendor/` — commit vendor directory | Claude |
| 1 | Write spec template in `docs/specs/TEMPLATE.md` | Claude |
| 2 | Write specs for all 8 Phase 1 crates | Claude + agents |
| 3 | `scripts/generate_crate.ps1` — factory loop | Claude |
| 4 | `scripts/collect_training_agent_data.py` — data collection | Claude |
| 5 | `scripts/generate_training_agent_dpo.py` — DPO generation | Claude |
| 6–7 | Pilot SFT on 0.5B to validate data pipeline | Claude |

### Month 1: Phase 1 Foundation + Training Agent Stage 1

**Week 1–2 (parallel, 5 streams):**
- Stream A: `bonsai-error` + `bonsai-log`
- Stream B: `bonsai-rand` + `bonsai-id`
- Stream C: `bonsai-time` + `bonsai-codec`
- Stream D: `bonsai-fs` + `bonsai-sanitise`
- Stream E: Training Agent Stage 1 SFT (running overnight continuously)

**Week 3–4:**
- Integrate all 8 Phase 1 crates: remove `anyhow`, `thiserror`, `tracing`, `log`, `rand`, `chrono`, `uuid`, `base64`, `hex`, `walkdir`, `globset`, `notify`, `tempfile` from every `Cargo.toml`
- `cargo check --workspace` clean
- Training Agent SFT completes

### Month 2: Phase 2 Crypto/Net/DB

**Parallel streams:**
- Stream B: `bonsai-crypto` — SHA-2, BLAKE3, AES-GCM, ChaCha20, Argon2, Ed25519/X25519, HMAC, BIP-39 (4 weeks)
- Stream C: `bonsai-net` + `bonsai-http` + `bonsai-websocket` (4 weeks, sequential within stream)
- Stream C-2: `bonsai-db` begins (8 weeks total, this is the longest single crate)
- Stream A: `bonsai-sync` + `bonsai-collection` (1 week each)
- Training Agent: Stage 2 DPO + Stage 3 Distillation (running nights/weekends)

**Month 2 exit:** `reqwest`, `hyper`, `axum`, `aes-gcm`, `argon2`, `blake3`, `ed25519-dalek`, `x25519-dalek`, `hmac`, `sha2`, `rand`, `arc-swap`, `dashmap` removed.

### Month 3: Phase 2 Completion + Training Agent Deployment

- `bonsai-db` completes — remove `rusqlite`, `libsqlite3-sys`, `sqlx`, `sqlparser`
- `bonsai-tls` — vendored `rustls` source + thin wrapper
- `bonsai-yaml`, `bonsai-toml`, `bonsai-cbor` — serial generation (1 week each)
- **Training Agent v1 deployed**: loaded as a llama-server sidecar, registered in ToolRegistry
- First autonomous training run managed by the Training Agent

### Month 4: Phase 3 Content & Platform

**Stream D (parallel):**
- `bonsai-image` (PNG/JPEG/WebP/GIF codecs) — 3 weeks
- `bonsai-markup` (CommonMark parser) — 2 weeks
- `bonsai-dom` (HTML5 tree builder + CSS selector) — 3 weeks

**Stream A:**
- `bonsai-regex` (NFA/DFA engine) — 3 weeks
- `bonsai-diff` + `bonsai-feed` + `bonsai-qr` + `bonsai-archive` — 1 week each

**Stream D-2:**
- `bonsai-cli` — 1 week
- `bonsai-text` (unicode normalisation) — 3 days
- `bonsai-rate` (token bucket) — 2 days

**Stream E:**
- `bonsai-editor` (lightweight code editor, replaces Monaco) — 4 weeks
- `bonsai-term` (VT100 terminal emulator, replaces xterm) — 3 weeks

### Month 5: Phase 3 Platform + Phase 4 Begin

**Stream D (platform):**
- `bonsai-native`: sysinfo, dirs, synthetic input, screen capture, PTY, keyring — 6 weeks
- `bonsai-audio`: WAV, MP3, Opus, playback — 4 weeks

**Stream E:**
- `bonsai-tensor` begins — the most complex single component (12–15 weeks total)
  - Week 1–2: GGUF parser (read `.gguf` model files)
  - Week 3–4: Basic tensor ops (matmul, softmax, layernorm)
  - Week 5–6: Transformer layer (attention + MLP)
  - Week 7–8: Qwen2/LLaMA inference pass

**Stream C:**
- `bonsai-discord` + `bonsai-telegram` — 2 weeks each
- `bonsai-matrix` — 3 weeks
- `bonsai-mail` (SMTP + IMAP) — 2 weeks

### Month 6: Phase 4 AI & WASM

**Stream E:**
- `bonsai-tensor` continues (KV cache, quantisation, full inference)
- `bonsai-wasm` — WASM interpreter + WASI snapshot preview1 — 8 weeks
- `bonsai-lua` — embed Lua 5.4 source — 2 weeks
- `bonsai-vision` — core image ops + Rust/WASM build — 6 weeks

**Stream B:**
- `bonsai-git` — 8 weeks

**Training Agent v2:** Retrained on accumulated 6-month log corpus (~500+ training runs). By this point it has supervised every phase of the sovereignty build and possesses highly specific knowledge of this codebase's patterns.

### Month 7: Phase 5 Async Runtime (Tokio Replacement)

This is the highest-risk phase. All 21 `tokio`-dependent crates must be migrated atomically.

**Approach:**
1. Build `bonsai-executor` with exact `tokio` API surface (2 weeks)
2. In each crate's `Cargo.toml`, add `bonsai-executor` as an alias: `extern crate bonsai_executor as tokio;` (1 week, automated script)
3. Fix compilation errors crate-by-crate (2 weeks, 5 agents in parallel)
4. Remove alias, clean up, remove `tokio` from all `Cargo.toml` (1 week)

**`bonsai-executor` architecture (2,000-word internal design):**
```
bonsai-executor/
├── reactor/
│   ├── iocp.rs          Windows IOCP event loop
│   ├── epoll.rs         Linux epoll event loop
│   └── kqueue.rs        macOS kqueue event loop
├── task/
│   ├── waker.rs         Waker implementation via AtomicPtr
│   ├── queue.rs         Work-stealing deque (Chase-Lev algorithm)
│   └── executor.rs      poll() loop, spawn(), block_on()
├── sync/
│   ├── mutex.rs         Async Mutex<T>
│   ├── channel.rs       mpsc and oneshot channels
│   ├── notify.rs        Notify (like tokio::sync::Notify)
│   └── semaphore.rs     Semaphore
├── io/
│   ├── tcp.rs           TcpStream, TcpListener (wraps reactor)
│   ├── udp.rs           UdpSocket
│   └── unix.rs          UnixStream (non-Windows)
├── time/
│   ├── sleep.rs         Sleep future
│   ├── interval.rs      Interval (like tokio::time::interval)
│   └── timeout.rs       timeout() wrapper
├── fs/
│   └── spawn_blocking.rs  Offloads blocking file I/O to thread pool
└── net/
    └── resolve.rs       Async DNS resolution
```

**Migration compatibility layer:** The `tokio` macro surface (`tokio::main`, `tokio::test`, `tokio::select!`, `tokio::spawn`, `tokio::join!`) is re-exported from `bonsai-executor` under the same names.

### Month 8: Phase 6 Shell & Final Integration

**`bonsai-shell` (Tauri replacement) — 8 weeks:**
- Week 1–2: Window management via Win32 `CreateWindowEx` / `winit` (vendored)
- Week 3–4: WebView integration (WebView2 COM bindings, same as Tauri's `wry`)
- Week 5–6: IPC bridge (PostMessage JSON protocol, same API as current Tauri IPC — zero frontend changes)
- Week 7: Plugin re-implementation (fs, dialog, notification, shell command, barcode scanner)
- Week 8: Build pipeline (resource embedding, NSIS packaging)

**Frontend remains Svelte for now** — `bonsai-ui` (Svelte replacement) is Phase 6b, deferred to Month 12–18. The frontend can run on Svelte indefinitely since it is compiled to vanilla JS at build time.

**Final integration:**
- `cargo check --workspace` with zero external crate dependencies
- `npm ci` using only vendored packages
- Full test suite run
- Performance benchmarks vs. original dependencies

---

## Part 4: Quality Gates and Metrics

Every crate and every training run is gated:

### Crate Quality Gate
```
☐ cargo check -p <crate>              — must pass
☐ cargo test -p <crate>               — ≥80% pass rate
☐ cargo clippy -p <crate> -- -D warnings — zero warnings
☐ cargo check --workspace             — must pass after integration
☐ No new `unsafe` blocks (exceptions: bonsai-crypto, bonsai-native, bonsai-executor)
☐ Benchmark vs. replaced crate: ≤2× slowdown (most should be on-par or faster)
☐ doc comment on every public item
```

### Training Agent Quality Gate (per version)
```
☐ Held-out diagnostic accuracy: ≥85%
   (given 100 training log snippets with known issues, identify issue correctly in ≥85)
☐ Hyperparameter prescription accuracy: ≥80%
   (given a training setup, recommend hyperparams that achieve loss convergence in a test run)
☐ Script generation quality: ≥90% of generated scripts compile and run without error
☐ No hallucinated library APIs (evaluated against the actual installed library versions)
☐ Bonsai-specific knowledge: ≥95% accuracy on known Windows/CPU training constraints
   (must always recommend max_length≤256 for CPU, must always set TRANSFORMERS_OFFLINE=1, etc.)
```

### Monthly Milestones
| Month | % External Deps Removed | Training Agent Version |
|-------|------------------------|------------------------|
| 1 | 25% (Foundation crates) | v0.1 (pilot SFT) |
| 2 | 45% (Crypto/Net) | v0.5 (SFT complete) |
| 3 | 55% (DB complete) | v1.0 (DPO + distillation) |
| 4 | 70% (Content/CLI) | v1.5 (first autonomous runs) |
| 5 | 80% (Platform/UI components) | v2.0 (full autonomy) |
| 6 | 88% (AI/WASM/Bots) | v2.5 (self-improving) |
| 7 | 96% (Async runtime) | v3.0 (tokio-replacement aware) |
| 8 | 100% | v3.5 (bonsai-shell aware) |

---

## Part 5: The Self-Improving Loop

Once the Training Agent v1.0 is deployed, the system becomes self-reinforcing:

```
[User clicks "Auto-Train" in ModelTrainer UI]
         │
         ▼
[TrainingAgentActor spawns inference server]
         │
         ▼
[Agent reads spec, examines available data, hardware profile]
         │
         ▼
[Agent generates training recipe: script params, data mix, schedule]
         │
         ▼
[Agent launches training job (dpo_train.py / finetune_sft.py / distill.py)]
         │
         ▼
[Agent monitors logs in real-time]
         │
    ┌────┴────┐
    │ anomaly │ ──► Agent intervenes: adjust LR / stop / restart
    └────┬────┘
         │ no anomaly
         ▼
[Training completes → agent evaluates on test set]
         │
         ▼
[Agent generates training report]
         │
         ▼
[export_training_logs.py converts run → training example for agent's own dataset]
         │
         ▼
[Weekly: agent re-trains itself on new examples → becomes smarter]
         │
         └──────────────────────────────────────────────────────────────►
                                                              (next training run is better)
```

This loop has no ceiling. Every model trained by the ecosystem makes the ecosystem smarter at training models.

---

## Part 6: Immediate Action Items (This Session)

In priority order, these are the first concrete steps to kick off the speedrun:

1. **`cargo vendor vendor/`** — run, commit, update `.cargo/config.toml`, verify offline build
2. **Write specs for all 8 Phase 1 crates** — `docs/specs/bonsai-error.md` through `docs/specs/bonsai-sanitise.md`
3. **`scripts/generate_crate.ps1`** — the factory loop that drives all crate generation
4. **`scripts/collect_training_agent_data.py`** — begin downloading and formatting Source 1–4 data
5. **`scripts/export_training_logs.py`** — convert today's training logs to agent training examples
6. **Pilot SFT run** for Training Agent validation on Qwen2.5-0.5B with 200 examples
7. **Start `bonsai-error`** — first crate, the simplest, unblocks everything

Each of these is a self-contained task that can be assigned to an AI agent or executed directly in this session.

---

*This document, combined with [SOVEREIGNTY_PLAN.md](SOVEREIGNTY_PLAN.md), forms the complete execution blueprint. Update the monthly milestone table as each phase closes.*
