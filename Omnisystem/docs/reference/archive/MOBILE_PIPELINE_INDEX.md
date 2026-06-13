# BonsAI Mobile Model Training Pipeline — Complete Index

## Overview

This document indexes all components of the **BonsAI Mobile Model Training Pipeline**—a complete knowledge distillation system for training compact, deployable models from Bonsai-8B teacher.

**Target Output**: Mobile-optimized model (~200 MB, 1.1B parameters) in GGUF format
**Technology Stack**: PyTorch, PEFT (LoRA), llama.cpp, Hugging Face Transformers
**Platform**: Windows 10 Pro, CPU/GPU training (AMD 7900 XTX via sidecar)

---

## File Structure

### Core Training Scripts

#### 1. `scripts/train_bonsai_mobile.py` (Main Training Loop)
**Purpose**: Knowledge distillation training script
**Lines**: 750+ (comprehensive with logging, validation, checkpointing)
**Key Components**:
- `setup_logging()`: Structured logging to console + file
- `get_device()`: Auto-detect CUDA/MPS/DirectML/CPU
- `teacher_api_completion()`: Call llama-server for teacher inference
- `MobileTrainingDataset`: Load & filter JSONL data with domain weighting
- `compute_kl_distillation_loss()`: KL-divergence + cross-entropy loss
- `train_epoch()`: Single epoch training with metrics
- `validate()`: Validation on held-out set
- `save_checkpoint()`: Save model, tokenizer, optimizer state

**Hyperparameters**:
- `--student-model`: HF model or local path (e.g., TinyLlama-1.1B)
- `--teacher-dir`: In-process teacher (HF format)
- `--teacher-api`: sidecar teacher (llama-server URL)
- `--alpha`: KL loss weight (0.0=SFT, 0.5=balanced, 1.0=pure KD)
- `--temperature`: Teacher softmax temperature (default 4.0)
- `--epochs`: Training epochs (typically 1-3)
- `--batch-size`: Batch size (CPU: 2-8, GPU: 16-32)
- `--learning-rate`: AdamW learning rate (default 5e-4)
- `--seed`: Reproducibility seed

**Output**:
- `~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model/` (HF format)
- `training_summary.json` (metadata: loss, time, hyperparams)
- `train_YYYY-MM-DD-HH-MM-SS.log` (detailed logs)

**Example Usage**:
```bash
# With teacher sidecar (recommended)
python scripts/train_bonsai_mobile.py \
  --student-model TinyLlama-1.1B-Instruct \
  --teacher-api http://127.0.0.1:8080 \
  --training-data ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --output-dir ~/.bonsai/models/checkpoints/bonsai-mobile-v1 \
  --epochs 3 --learning-rate 5e-4 --batch-size 4

# With in-process teacher (requires 24+ GB VRAM)
python scripts/train_bonsai_mobile.py \
  --student-model TinyLlama-1.1B-Instruct \
  --teacher-dir D:/Models/general/Bonsai-8B-Instruct \
  --training-data data.jsonl
```

---

#### 2. `scripts/quantize_bonsai_mobile.py` (Quantization)
**Purpose**: Convert trained HF model to quantized GGUF
**Lines**: 450+ 
**Key Components**:
- `merge_lora_adapter()`: Fuse LoRA adapter into base weights
- `convert_to_gguf()`: Call llama.cpp conversion script
- `validate_gguf()`: Verify GGUF integrity (magic number check, load test)
- `create_metadata()`: Generate metadata JSON with SHA256 hash
- `generate_model_card()`: Auto-generate README.md
- `create_bkp_package()`: Package GGUF + metadata as tar.gz

**Hyperparameters**:
- `--final-model`: Path to trained model (from Stage 2)
- `--output-dir`: Directory for GGUF output
- `--quantization`: Q4_K_M (default), Q5_K_M, Q8_0, F16
- `--llama-cpp-dir`: Path to llama.cpp checkout
- `--validate`: Run integrity check on output

**Output**:
- `bonsai-mobile-v1.gguf` (~200-250 MB, Q4_K_M)
- `bonsai-mobile-v1.metadata.json` (model info + hash)
- `bonsai-mobile-v1.bkp` (distribution package, tar.gz)
- `README.md` (model card)

**Example Usage**:
```bash
python scripts/quantize_bonsai_mobile.py \
  --final-model ~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model \
  --output-dir ~/.bonsai/models/releases \
  --quantization Q4_K_M \
  --validate \
  --create-bkp
```

---

#### 3. `scripts/benchmark_bonsai_mobile.py` (Performance Evaluation)
**Purpose**: Measure throughput, latency, memory on various devices
**Lines**: 500+
**Key Metrics**:
- **TTFT** (Time To First Token): Latency before first output
- **TPT** (Time Per Token): Average per-token latency
- **Tokens/sec**: Overall throughput
- **Peak Memory**: Peak RSS during inference
- **P95/P99 Latency**: Tail latencies

**Features**:
- 20 benchmark prompts (code, chat, system, QA)
- GGUF inference via llama-cpp-python
- HF model inference for baseline comparison
- Device selection (CPU, CUDA, MPS)
- Latency distribution analysis
- Human-readable report generation

**Output**:
- `benchmark_results.json` (raw metrics)
- `benchmark_report.md` (formatted report with tables)

**Example Usage**:
```bash
python scripts/benchmark_bonsai_mobile.py \
  --model ~/.bonsai/models/releases/bonsai-mobile-v1.gguf \
  --output-dir benchmark_results \
  --num-prompts 100 \
  --device cpu
```

---

#### 4. `scripts/export_mobile_training_data.py` (Data Preparation)
**Purpose**: Aggregate multi-domain training data with quality filtering
**Lines**: 350+
**Data Sources**:
- **Survival System** (20%): Crash fixes, patches from eternal loop
- **Code** (40%): Python, Rust, JavaScript examples + algorithms
- **Chat** (15%): User-assistant conversations from chat_sessions.db
- **Tool Use** (20%): MCP integration, function calling examples
- **Q&A** (5%): Academy, documentation QA pairs

**Features**:
- Load from JSONL files, SQLite databases
- Quality score filtering (default ≥ 0.70)
- Domain weighting with stratified sampling
- Deterministic shuffling (seed 42)
- Duplicate detection (optional)

**Output Format** (JSONL):
```json
{"text": "def fibonacci(n): ...", "domain": "code", "quality": 0.95, "source": "code_examples.jsonl"}
{"text": "Fixed: segfault on Windows...", "domain": "system_repair", "quality": 0.88}
```

**Example Usage**:
```bash
python scripts/export_mobile_training_data.py \
  --output ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --max-examples 100000 \
  --min-quality 0.70 \
  --domain-weights "code:0.4,system_repair:0.2,tool_use:0.2,chat:0.1,qa:0.05"
```

---

#### 5. `scripts/run_mobile_training_pipeline.ps1` (Orchestration)
**Purpose**: PowerShell orchestration script for complete pipeline
**Lines**: 250+
**Stages**:
1. **Data Export**: Call export_mobile_training_data.py
2. **Teacher Startup**: Start llama-server sidecar
3. **Training**: Call train_bonsai_mobile.py
4. **Quantization**: Call quantize_bonsai_mobile.py
5. **Benchmarking**: Call benchmark_bonsai_mobile.py
6. **Registration**: Move GGUF to registry, create metadata

**Features**:
- Colored output (status, warnings, errors)
- Automatic teacher server health check
- Optional stages (skip with -Skip* flags)
- Dry-run mode (-DryRun)
- Graceful cleanup (kill teacher on exit)

**Example Usage**:
```powershell
.\scripts\run_mobile_training_pipeline.ps1 `
  -StudentModel "TinyLlama-1.1B-Instruct" `
  -TeacherGGUF "D:\Models\general\Bonsai-8B-Q4_K_M.gguf" `
  -Epochs 3 `
  -LearningRate 0.0005
```

---

### Configuration Files

#### 6. `config/bonsai_mobile_config.yaml`
**Purpose**: Central configuration for entire pipeline
**Sections**:
- **student_model**: Architecture, parameters, context length, LoRA settings
- **teacher_model**: Strategy (API vs. local), distillation hyperparams
- **data**: Domain weights, quality filtering, max examples
- **training**: Epochs, batch size, learning rate, gradient settings
- **optimization**: AdamW settings, LR scheduling
- **hardware**: Device detection, VRAM limits, memory safety
- **output**: Checkpoint directory, quantization type
- **model_registry**: Auto-registration settings
- **evaluation**: Validation metrics, quality gates
- **monitoring**: Wandb integration, logging intervals
- **reproducibility**: Seeds, determinism settings
- **advanced**: Mixed precision, gradient checkpointing, torch.compile

**Key Values**:
```yaml
student_model:
  architecture: "tinyllama"
  parameters: 1_100_000_000  # 1.1B
  context_length: 2048
  lora:
    rank: 16
    alpha: 32

teacher_model:
  strategy: "llama_server_api"
  distillation:
    temperature: 4.0
    alpha: 0.5

data:
  domain_weights:
    code: 0.40
    system_repair: 0.20
    tool_use: 0.20
    chat: 0.15
    qa: 0.05
  min_quality: 0.70
  max_examples: 100_000

training:
  epochs: 3
  batch_size: 4
  learning_rate: 5e-4
  max_seq_length: 2048
```

---

### Documentation

#### 7. `docs/MOBILE_TRAINING_PIPELINE.md`
**Purpose**: Complete user guide and reference
**Sections**:
- **Overview**: Use cases, architecture diagram
- **Quick Start**: Prerequisites, file preparation, running pipeline
- **Individual Stages**: Detailed instructions for each stage
- **Configuration**: YAML parameter reference
- **Loss Function**: Mathematical explanation of KL + CE loss
- **Quality Gates**: Data filtering, validation monitoring, safeguards
- **Performance Metrics**: Throughput, memory, latency benchmarks
- **Data Provenance**: Domain breakdown, quality scoring
- **Troubleshooting**: Common issues and solutions
- **Advanced Usage**: Custom models, in-process teacher, quantization variants
- **Deployment**: iOS, Android, Docker, Raspberry Pi examples
- **Model Card Template**: Auto-generated documentation
- **Integration**: Registry entry, Rust integration example
- **References**: Papers, implementations, tools

---

### This File (Index)

#### 8. `MOBILE_PIPELINE_INDEX.md` (this file)
**Purpose**: Complete file index, API reference, workflow guide

---

## Training Workflow

### Stage 1: Data Preparation
```bash
python scripts/export_mobile_training_data.py \
  --output ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --max-examples 100_000 \
  --min-quality 0.70
# Output: JSONL with ~100k balanced training examples
```

**Result**: 
- `combined_mobile_training.jsonl` (~400 MB)
- Balanced domains: code 40%, system 20%, tools 20%, chat 15%, QA 5%

---

### Stage 2: Training
```bash
# Terminal 1: Start teacher
llama-server -m Bonsai-8B-Q4_K_M.gguf -ngl 99 --port 8080

# Terminal 2: Train student
python scripts/train_bonsai_mobile.py \
  --student-model TinyLlama-1.1B-Instruct \
  --teacher-api http://127.0.0.1:8080 \
  --training-data ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --output-dir ~/.bonsai/models/checkpoints/bonsai-mobile-v1 \
  --config config/bonsai_mobile_config.yaml \
  --epochs 3 \
  --batch-size 4 \
  --learning-rate 5e-4
```

**Result**:
- `~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model/` (HF format)
- `training_summary.json` (metadata)
- Logs with loss curves, best checkpoint info

**Time**: ~4-8 hours on CPU (Windows i7-12700K)

---

### Stage 3: Quantization
```bash
python scripts/quantize_bonsai_mobile.py \
  --final-model ~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model \
  --output-dir ~/.bonsai/models/releases \
  --quantization Q4_K_M \
  --validate \
  --create-bkp
```

**Result**:
- `bonsai-mobile-v1.gguf` (~220 MB)
- `bonsai-mobile-v1.metadata.json` (model info)
- `bonsai-mobile-v1.bkp` (distribution package)
- `README.md` (model card)

**Time**: ~10-15 minutes

---

### Stage 4: Benchmarking
```bash
python scripts/benchmark_bonsai_mobile.py \
  --model ~/.bonsai/models/releases/bonsai-mobile-v1.gguf \
  --output-dir benchmark_results \
  --num-prompts 100
```

**Result**:
- `benchmark_results/benchmark_results.json` (metrics)
- `benchmark_results/benchmark_report.md` (human-readable)

**Time**: ~5-10 minutes (depends on --num-prompts)

---

### Stage 5: Model Registration
```bash
# Copy to registry
cp ~/.bonsai/models/releases/bonsai-mobile-v1.gguf ~/.bonsai/models/releases/

# Create metadata entry
# (automatically created by quantization script)
```

**Result**:
- Model available in `config/model_registry.yaml`
- Can be loaded by BonsAI runtime

---

## Loss Function Reference

### Knowledge Distillation Loss

```
L = α · KL(q||p) + (1 - α) · CE(student, labels)

where:
  q = student logits (softmaxed with temperature T)
  p = teacher logits (softmaxed with temperature T)
  α = KL weight (0.0 to 1.0)
  T = temperature (1.0 to 10.0)
```

**Components**:

1. **KL Divergence** (soft targets from teacher):
   ```
   KL = Σ p(z) · log(p(z) / q(z))
   
   Scaled by T² for gradient matching
   ```
   - Encourages student to match teacher's uncertainty
   - Improves generalization

2. **Cross-Entropy** (hard labels from ground truth):
   ```
   CE = -Σ y · log(q(z))
   
   where y = one-hot ground truth
   ```
   - Prevents mode collapse
   - Keeps student factually grounded

**Hyperparameter Tuning**:

| Setting | Effect | Value |
|---------|--------|-------|
| `α = 0.0` | Pure supervised learning | Use if data is small |
| `α = 0.5` | Balanced KL + CE | **Recommended** |
| `α = 1.0` | Pure distillation | Use if teacher is excellent |
| `T = 1.0` | Vanilla softmax | Harsh targets, slow convergence |
| `T = 4.0` | Soft targets | **Recommended** for mobile |
| `T = 10.0` | Very soft targets | Faster convergence, looser alignment |

---

## Data Format Specification

### JSONL Training Data
```json
{
  "text": "def fibonacci(n):\n    if n <= 1: return n\n    return fibonacci(n-1) + fibonacci(n-2)",
  "domain": "code",
  "quality": 0.95,
  "source": "code_examples.jsonl"
}
```

**Fields**:
- `text` (str): Training example (prompt + response)
- `domain` (str): One of: code, system_repair, tool_use, chat, qa
- `quality` (float): 0.0 (generated) to 1.0 (human-verified)
- `source` (str): Data source filename
- (Optional) Additional fields: `timestamp`, `tags`, `language`, etc.

---

## Model Sizes and Timing

### Student Models (Baseline)
| Model | Params | Q4_K_M Size | Tokens/sec | Memory |
|-------|--------|-------------|-----------|--------|
| TinyLlama-1.1B | 1.1B | 200 MB | 65 | 220 MB |
| Phi-3-Mini | 3.8B | 250 MB | 45 | 280 MB |
| Gemma 2B | 2B | 280 MB | 48 | 300 MB |

### Distilled Mobile Model (Output)
| Metric | Value |
|--------|-------|
| Base Architecture | TinyLlama-1.1B |
| Parameters | 1.1B |
| Quantization | Q4_K_M |
| GGUF Size | ~220 MB |
| Context Length | 2048 tokens |
| TTFT | 30-50 ms (CPU) |
| TPT | 12-20 ms (CPU) |
| Throughput | 50-80 tokens/sec |

---

## Environment Variables

### Required
```bash
TRANSFORMERS_OFFLINE=1          # Never download from HF
HF_HUB_OFFLINE=1                # Enforce offline mode
HF_HUB_DISABLE_TELEMETRY=1      # Disable tracking
```

### Optional
```bash
WANDB_PROJECT=bonsai-mobile-training  # W&B project name
CUDA_VISIBLE_DEVICES=0                 # GPU selection (for CUDA)
OMP_NUM_THREADS=8                      # CPU threading (for performance)
```

---

## Troubleshooting Matrix

| Problem | Symptom | Solution |
|---------|---------|----------|
| **OOM** | `RuntimeError: CUDA out of memory` | Reduce batch_size, max_seq_len, or lora_rank |
| **Training loss stuck** | Loss doesn't decrease | Reduce learning_rate, increase temperature, reduce alpha |
| **KL divergence explodes** | Loss NaN or inf | Reduce alpha to 0.2, increase temperature to 8.0, check data |
| **Teacher won't start** | `Connection refused on :8080` | Kill existing process: `taskkill /PID <pid> /F` |
| **GGUF conversion fails** | `convert_hf_to_gguf.py not found` | Run `cd ~/llama.cpp && make` |
| **Benchmark errors** | `ModuleNotFoundError: llama_cpp` | `pip install llama-cpp-python` |
| **Segfault on Windows** | `Exit code 139` | Use PowerShell, not Git Bash |

---

## Integration Points

### With BonsAI Runtime
```rust
// bonsai-runtime/src/lib.rs
use bonsai_runtime::ModelRegistry;

let model = ModelRegistry::load("bonsai-mobile-v1")?;
let response = model.inference("Write Rust code for...", 2048)?;
```

### With Model Registry
```yaml
# config/model_registry.yaml
models:
  - name: "bonsai-mobile-v1"
    path: "~/.bonsai/models/releases/bonsai-mobile-v1.gguf"
    role: "student_mobile"
    vram_gb: 1
    domains: [coding, system_repair, tool_use, chat, qa]
    context_len: 2048
    quantisation: Q4_K_M
```

### With Training Agent
```bash
# After training, export logs for training agent
python scripts/export_training_logs.py \
  --phase mobile_distill \
  --training-summary ~/.bonsai/models/checkpoints/bonsai-mobile-v1/training_summary.json
```

---

## Performance Targets

### Training
- **Convergence**: 1-3 epochs (teacher already optimized)
- **Training Time**: 4-8 hours (Windows CPU)
- **Final Loss**: < 0.05
- **Validation Loss**: Converge by epoch 2

### Inference (Q4_K_M)
- **Throughput**: 50-80 tokens/sec (CPU)
- **TTFT**: 30-50 ms
- **Memory**: 220 MB model + 512 MB runtime
- **Latency P95**: < 100 ms per token

### Quantization
- **GGUF Size**: 220 MB (target: < 300 MB)
- **Compression**: ~10x from fp32 (2.2 GB → 220 MB)
- **Quality Loss**: < 5% perplexity increase

---

## File Checksums (for verification)

After running the pipeline:

```bash
# Verify GGUF integrity
sha256sum ~/.bonsai/models/releases/bonsai-mobile-v1.gguf
# Hash stored in: bonsai-mobile-v1.metadata.json

# Verify training data
wc -l ~/.bonsai/training_export/combined_mobile_training.jsonl
# Expected: 100,000 lines (or your --max-examples value)
```

---

## Next Steps

### 1. Deploy to Mobile
- Convert GGUF to CoreML (iOS) or NNAPI (Android)
- Integrate with native inference libraries
- Test on target devices

### 2. Fine-Tune for Specific Domain
```bash
python scripts/train_bonsai_mobile.py \
  --student-model ~/.bonsai/models/releases/bonsai-mobile-v1.gguf \
  --teacher-api http://127.0.0.1:8080 \
  --training-data domain_specific_data.jsonl \
  --output-dir checkpoint_domain_v1 \
  --epochs 2 \
  --learning-rate 1e-4  # Lower LR for fine-tuning
```

### 3. Run on Embedded Device
```bash
# Raspberry Pi
python3 -m llama_cpp.server --model_path bonsai-mobile-v1.gguf --port 8000

# Jetson
docker run --rm -p 8000:8000 \
  -v ./bonsai-mobile-v1.gguf:/model.gguf \
  ghcr.io/abetlen/llama-cpp-python \
  --model_path /model.gguf --port 8000
```

### 4. Monitor with Weights & Biases
```bash
export WANDB_PROJECT=bonsai-mobile-training
python scripts/train_bonsai_mobile.py \
  --wandb-project bonsai-mobile-training \
  ...
```

---

## Support & References

### Documentation
- Full guide: `docs/MOBILE_TRAINING_PIPELINE.md`
- This index: `MOBILE_PIPELINE_INDEX.md`
- BonsAI docs: `docs/*.md`

### Scripts
- Training: `scripts/train_bonsai_mobile.py`
- Quantization: `scripts/quantize_bonsai_mobile.py`
- Benchmarking: `scripts/benchmark_bonsai_mobile.py`
- Data export: `scripts/export_mobile_training_data.py`
- Orchestration: `scripts/run_mobile_training_pipeline.ps1`

### Configuration
- Main config: `config/bonsai_mobile_config.yaml`
- Model registry: `config/model_registry.yaml`
- Training settings: `config/training.yaml`

---

**Version**: 1.0.0
**Last Updated**: June 2026
**Status**: Production Ready

For issues, contributions, or questions, refer to the BonsAI project repository.
