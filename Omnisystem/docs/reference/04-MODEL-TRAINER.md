# Model Trainer & Training Pipeline

Bonsai includes a complete, on-device AI training pipeline. You can teach BonsAI new skills, improve its behaviour, and produce a custom model that is yours — without sending any data to the cloud.

---

## Overview

The training pipeline transforms raw interactions and curated datasets into a smarter, more personalised BonsAI. The full pipeline looks like this:

```
User interactions
  + Curated datasets        ─→  Training Data Library (TDL)
  + Survival KB events                    │
                                          ▼
                                   Data preparation
                                   (tokenisation, dedup)
                                          │
                                          ▼
                        ┌─────── Training Phases ────────┐
                        │  1. Safety DPO                 │
                        │  2. Survival DPO               │
                        │  3. Tool Use DPO               │
                        │  4. Code Distillation          │
                        │  5. General Distillation       │
                        │  6. Reasoning Distillation     │
                        │  7. Final SFT Merge            │
                        └──────────────┬─────────────────┘
                                       │
                                       ▼
                              LoRA adapter (.safetensors)
                                       │
                                       ▼
                              Merge → new GGUF
                                       │
                                       ▼
                              Hot-reload (no restart)
```

Each phase produces a LoRA adapter. The final step merges all adapters into the base model to produce a standalone GGUF.

---

## The Model Trainer UI

Open the **Model Trainer** panel (🧠 icon, or `Ctrl+T`).

The trainer is intentionally simple:

```
┌─────────────────────────────────────────────────┐
│  🧠 Model Trainer                               │
│                                                 │
│  Current Brain Age: ████████░░ 78% (Level 4)   │
│                                                 │
│  [Train Everything]  [Quick Train]              │
│                                                 │
│  Progress: Safety DPO ████████████ 100%        │
│            Survival DPO ████░░░░░░ 40%         │
│            ...                                  │
│                                                 │
│  📋 Live Logs                                   │
│  [2026-05-30 14:22] epoch 3/5 loss=0.412       │
│  [2026-05-30 14:23] epoch 4/5 loss=0.389       │
└─────────────────────────────────────────────────┘
```

### Train Everything
Runs the full seven-phase pipeline in order. Takes 30 minutes to several hours depending on hardware and dataset size. Best run overnight.

### Quick Train
Runs only phases with new data since the last training session. Much faster — typically 5–15 minutes. Run after adding new training examples.

---

## Training Phases Explained

### Phase 1 – Safety DPO
**Goal**: teach BonsAI to refuse harmful requests firmly and explain why.  
**Data**: pairs of (harmful prompt → refused response) vs (harmful prompt → compliant response).  
**Why first**: safety constraints must be established before other behaviour is learned.

### Phase 2 – Survival DPO
**Goal**: teach BonsAI to diagnose software errors and generate correct repair commands.  
**Data**: (error log → wrong fix) vs (error log → correct fix) pairs from the Survival Knowledge Base.  
**Result**: BonsAI becomes better at debugging your specific setup.

### Phase 3 – Tool Use DPO
**Goal**: teach correct JSON tool-call format and smart tool selection.  
**Data**: (user request → wrong tool call) vs (user request → correct tool call).  
**Result**: fewer malformed tool calls, better task decomposition.

### Phase 4 – Code Distillation
**Goal**: improve code generation quality by learning from a stronger teacher model.  
**Method**: the teacher model (e.g., DeepSeek-R1-32B) generates high-quality code solutions; BonsAI is trained to match them via SFT.  
**Data**: the `code` domain of the Training Data Library.

### Phase 5 – General Distillation
**Goal**: improve everyday conversation and instruction following.  
**Teacher**: Qwen3-35B or similar.  
**Data**: the `chat` domain.

### Phase 6 – Reasoning Distillation
**Goal**: improve multi-step reasoning and problem solving.  
**Teacher**: DeepSeek-R1-70B.  
**Data**: the `reasoning` domain.

### Phase 7 – Final SFT Merge
**Goal**: combine all adapters into a single, coherent fine-tuned model.  
**Method**: all LoRA adapters are merged into the base model weights; the result is quantised to Q4_K_M and saved as a new GGUF.

---

## Dataset Management

### Training Data Library (TDL)
The TDL is a structured store of all training examples. Each example has:
- A BLAKE3 content hash (for deduplication)
- A type: `preference_pair`, `completion`, `conversation`
- Domain tags: `safety`, `survival`, `code`, `chat`, `reasoning`, `tool_use`
- A quality score (0.0–1.0)
- Source provenance

### Adding Examples

**From the UI**: in the Chat Panel, click 👍 on a response you liked or 👎 on one you disliked. The exchange is automatically recorded as a preference pair.

**From the training dashboard**: click **Import Dataset** and select a JSONL file. Each line must be:
```json
{"prompt": "...", "chosen": "...", "rejected": "..."}
```
or for SFT:
```json
{"messages": [{"role": "user", "content": "..."}, {"role": "assistant", "content": "..."}]}
```

**From scripts**:
```python
# scripts/generate_safety_data.py produces safety DPO pairs automatically
python scripts/generate_safety_data.py --output training_data/safety/
```

### Exporting a Dataset
Click **Export Dataset** → choose domain and quality threshold → saves as JSONL. Use this to share datasets, back them up, or load them into other tools.

---

## Monitoring Progress

### Live Logs
The trainer streams training output directly into the log panel. Key metrics to watch:
- **loss** – should decrease each epoch. If it stops decreasing, training has converged.
- **grad_norm** – if this explodes (> 10), reduce learning rate in `config/training.yaml`.
- **lr** – learning rate schedule (warmup then cosine decay).

### Milestone Notifications
When a phase completes, a milestone card appears:
- Phase name and duration
- Final validation loss
- Whether a new adapter was saved

### 🎊 Confetti
When the final merge completes successfully, confetti falls. You earned it.

---

## Brain Age Indicator

The **Brain Age** is a fun score that measures how well-trained your model is. It lives in the status bar.

| Level | Brain Age | Meaning |
|---|---|---|
| 1 | 0–19% | Fresh base model |
| 2 | 20–39% | Basic safety + tool use learned |
| 3 | 40–59% | Code and survival knowledge added |
| 4 | 60–79% | Reasoning distillation complete |
| 5 | 80–99% | Full pipeline, all phases done |
| 6 | 100% | Merged, quantised, production-ready |

Brain Age increases after each completed phase. It resets (slightly) if you switch to a different base model.

---

## Deploying New Adapters

### Hot-Reload
After a training phase completes, Bonsai automatically hot-reloads the new adapter without restarting the app. You'll see "Model updated" in the status bar. The next message uses the improved model.

### Rollback
If the new model behaves poorly:
1. Open **Settings → Models → Adapter History**.
2. Select a previous adapter checkpoint.
3. Click **Load Previous**. The old adapter loads immediately.

Adapters are kept for 30 days by default (configurable).

---

## EternalTrainingLoop

The **EternalTrainingLoop** is an optional background process that continuously trains BonsAI on new data as it accumulates.

Enable it with the `eternal_training_loop` feature flag.

**How it works**:
1. Monitors the TDL for new examples (polls every 10 minutes).
2. When enough new examples accumulate (configurable threshold, default 50), starts a Quick Train cycle.
3. Runs at low priority so it doesn't interfere with foreground work.
4. Skips training if battery < 20% or GPU temperature > 80°C.

**DreamAgent** (runs nightly at 2:00 AM):
- Reviews the day's interactions and survival events.
- Updates `BONSAI.md` with any learned preferences.
- Prunes low-quality training examples from the TDL.
- Generates a training summary report.

---

*← [BonsAI Assistant](03-BONSAI-ASSISTANT.md) · [Survival System →](05-SURVIVAL-SYSTEM.md)*
