# Training Agent — Complete Specification
## `bonsai-training-agent` — The AI That Trains AIs

**Base model:** Qwen2.5-14B-Instruct (pilot: 7B, long-term: DeepSeek-R1-32B)  
**Output format:** GGUF Q4_K_M  
**Deployment:** llama-server sidecar + ToolRegistry tool `train_model`  
**Self-improvement:** Weekly fine-tune on accumulated training logs

---

## 1. Capability Specification

The Training Agent must answer all of the following correctly at ≥85% accuracy on the evaluation set:

### 1.1 Diagnosis Tasks
- Given a loss curve (as text/CSV), identify: divergence, underfitting, overfitting, gradient explosion, mode collapse, plateau
- Given an error traceback from `dpo_train.py` / `finetune_sft.py` / `distill.py`, provide the exact fix
- Given training config + hardware description, identify potential OOM / ACCESS_VIOLATION risks before the run starts

### 1.2 Prescription Tasks
- Given task type (safety alignment / code generation / instruction following / distillation) + model size + hardware, output a complete training recipe:
  - Which script to use (`dpo_train.py` vs `finetune_sft.py` vs `distill.py`)
  - All hyperparameter values with reasoning
  - Expected training duration
  - Expected final loss range
  - Data format requirements
- Given a failed run, output the exact CLI invocation that would succeed

### 1.3 Code Generation Tasks
- Write a new training script variant given a description of changes needed
- Add a feature to an existing script (e.g., "add learning rate warmup to dpo_train.py")
- Convert a training config from one format to another (e.g., axolotl YAML → our script's CLI args)

### 1.4 Bonsai-Specific Knowledge (Critical)
The agent must know these constraints by heart — they are not in any public dataset:

| Constraint | Rule |
|-----------|------|
| Windows CPU training | Always `--max-length ≤ 256`; default 128 for models ≥ 1B with two copies in RAM |
| DirectML backward pass | Never use DirectML for training. RX 7900 XTX = inference only via llama-server |
| Offline enforcement | Always set `TRANSFORMERS_OFFLINE=1`, `HF_HUB_OFFLINE=1`, `HF_DATASETS_OFFLINE=1`, `HF_HUB_DISABLE_TELEMETRY=1` before any HF import |
| Python interpreter | Use `C:\Users\limpi\AppData\Local\Programs\Python\Python312\python.exe` — the Windows Store stub does not have `peft` |
| Model sizes for CPU RAM | 0.5B: safe at max_len=256; 1.5B: safe at max_len=128; 7B: need max_len≤64 or reduce to 1 copy (reference only) |
| Offline build | `TRANSFORMERS_OFFLINE=1` must be set before `import transformers` — not after |
| Git Bash segfault | Do not run training scripts from Git Bash on Windows. Use PowerShell with `System.Diagnostics.Process` or Python subprocess |

---

## 2. Dataset Files

All data is stored under `~/.bonsai/training_agent/`:

```
~/.bonsai/training_agent/
├── tier1_theory.jsonl         ML textbooks + papers Q&A (~60,000 pairs)
├── tier2_failures.jsonl       Error→fix pairs from SO, GitHub, KB (~25,000 pairs)
├── tier3_dpo_pairs.jsonl      Preference pairs chosen/rejected (~10,000 pairs)
├── tier3_code_gen.jsonl       Script generation pairs (~5,000 pairs)
├── bonsai_logs.jsonl          Project-specific training run logs (growing)
├── combined_sft.jsonl         All instruction pairs merged + deduplicated
└── eval_set.jsonl             100 held-out examples for quality gate
```

---

## 3. Training Recipe (Final Configuration)

### Stage 1: SFT
```bash
python bonsai-workspace/runtimes/bonsai-trainer/finetune_sft.py \
  --base-model "C:/Users/limpi/.cache/huggingface/hub/models--Qwen--Qwen2.5-7B-Instruct/snapshots/<hash>/" \
  --data "C:/Users/limpi/.bonsai/training_agent/combined_sft.jsonl" \
  --output "C:/Users/limpi/.bonsai/adapters/training-agent-sft-v1" \
  --device cpu \
  --epochs 3 \
  --batch-size 2 \
  --grad-accum 16 \
  --lr 2e-4 \
  --max-length 256 \
  --lora-r 64 \
  --lora-alpha 16
```

### Stage 2: DPO
```bash
python bonsai-workspace/runtimes/bonsai-trainer/dpo_train.py \
  --base-model "C:/Users/limpi/.bonsai/adapters/training-agent-sft-v1" \
  --data "C:/Users/limpi/.bonsai/training_agent/tier3_dpo_pairs.jsonl" \
  --output "C:/Users/limpi/.bonsai/adapters/training-agent-dpo-v1" \
  --device cpu \
  --epochs 2 \
  --lr 5e-5 \
  --beta 0.1 \
  --max-length 256 \
  --max-pairs 10000
```

### Stage 3: GGUF Conversion
```bash
python C:/tools/llama.cpp/convert_hf_to_gguf.py \
  "C:/Users/limpi/.bonsai/adapters/training-agent-dpo-v1" \
  --outfile "D:/Models/training-agent/bonsai-training-agent-7b-q4.gguf" \
  --outtype q4_k_m
```

### Stage 4: Deploy
```bash
# Start as sidecar on port 8090 (port 8080 reserved for teacher)
llama-server \
  -m D:/Models/training-agent/bonsai-training-agent-7b-q4.gguf \
  --port 8090 \
  --ctx-size 32768 \
  --n-predict 4096
```

Record phase after each stage:
```bash
python scripts/record_training_phase.py --phase training_agent
```

---

## 4. Evaluation Protocol

Run `scripts/evaluate_training_agent.py` after each stage:

```python
# 100 held-out scenarios from eval_set.jsonl
# Metrics:
# 1. Diagnosis accuracy: % of failure logs correctly diagnosed
# 2. Prescription accuracy: % of prescriptions that result in successful training runs (requires execution)
# 3. Bonsai constraint recall: % of Bonsai-specific rules correctly applied
# 4. Code quality: % of generated scripts that compile and run
```

**Minimum passing scores to proceed to next stage:**
- Stage 1 → Stage 2: diagnosis accuracy ≥ 70%
- Stage 2 → Stage 3: diagnosis accuracy ≥ 80%, Bonsai constraints ≥ 90%
- Deploy to production: all metrics ≥ 85%
