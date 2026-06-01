# BonsAI Mobile Model Training Pipeline

Complete knowledge distillation pipeline for training a compact, edge-deployable AI model (500M-1.7B parameters) from a large teacher model (Bonsai-8B).

## Overview

The BonsAI Mobile Pipeline trains small models suitable for:
- iOS/Android app inference
- Edge devices (Raspberry Pi, Jetson Nano, TPU)
- CPU-only environments
- Latency-sensitive applications
- Low-bandwidth scenarios

**Teacher Model**: Bonsai-8B or similar (35-70B for full knowledge transfer)
**Student Model**: TinyLlama-1.1B, Phi-3-Mini, or similar
**Output**: Quantized GGUF (~200-300 MB)

### Key Features

- **Knowledge Distillation**: KL-divergence loss + supervised fine-tuning
- **Domain-Balanced Data**: Code, system repair, tool use, chat, Q&A
- **Quality Filtering**: Only high-confidence training examples
- **Checkpoint Management**: Saves best model by validation loss
- **Quantization**: Automatic GGUF conversion (Q4_K_M)
- **Benchmarking**: Throughput, latency, memory profiling
- **Model Registry**: Auto-registration and distribution packaging

## Architecture

```
                    Training Pipeline
                          |
         ___________________+____________________
         |                  |                   |
    Data Export         Training              Quantization
         |                  |                   |
    - Survival fixes    - Distill loss      - Merge LoRA
    - Code examples     - Validation        - GGUF convert
    - Chat history      - Checkpoints       - Q4_K_M
    - Tool use          - Metrics           - Verify
    - Q&A pairs         - Wandb logging     - Package (.bkp)
         |                  |                   |
         +------ Shared Config File (YAML) ------+
```

## Quick Start

### Prerequisites

```bash
# Python 3.10+
python --version

# PyTorch (CPU or GPU)
pip install torch transformers peft datasets

# Quantization tools
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp && make

# llama-server binary
# (already in llama.cpp/build/bin/llama-server after make)

# Optional: benchmarking
pip install llama-cpp-python

# Optional: monitoring
pip install wandb
```

### Files to Prepare

1. **Teacher GGUF**: Download or build Bonsai-8B GGUF (~4-6 GB)
   ```bash
   # Example path on Windows
   D:\Models\general\Bonsai-8B-Q4_K_M.gguf
   ```

2. **Student Base Model**: HuggingFace model (auto-downloaded or local)
   ```bash
   # Auto-download from HF
   TinyLlama-1.1B-Instruct
   
   # Or local path
   ~/.cache/huggingface/hub/.../TinyLlama-1.1B-Instruct
   ```

3. **Training Data**: Export combined JSONL
   ```bash
   python scripts/export_mobile_training_data.py \
     --output ~/.bonsai/training_export/combined_mobile_training.jsonl \
     --max-examples 100000 \
     --min-quality 0.70
   ```

### Running the Full Pipeline (Windows PowerShell)

```powershell
# Terminal 1: Start teacher server
llama-server.exe `
  -m "D:\Models\general\Bonsai-8B-Q4_K_M.gguf" `
  -ngl 99 --port 8080 --no-mmap

# Terminal 2: Run pipeline
.\scripts\run_mobile_training_pipeline.ps1 `
  -StudentModel "TinyLlama-1.1B-Instruct" `
  -TeacherGGUF "D:\Models\general\Bonsai-8B-Q4_K_M.gguf" `
  -Epochs 3 `
  -LearningRate 0.0005
```

### Running Individual Stages

#### Stage 1: Export Training Data

```bash
python scripts/export_mobile_training_data.py \
  --output ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --max-examples 100000 \
  --min-quality 0.70 \
  --domain-weights "code:0.4,system_repair:0.2,tool_use:0.2,chat:0.1,qa:0.1"
```

**Output Format** (JSONL):
```json
{"text": "def fibonacci(n): ...", "domain": "code", "quality": 0.95, "source": "code_examples.jsonl"}
{"text": "Fixed: segfault in ...", "domain": "system_repair", "quality": 0.88}
```

#### Stage 2: Training

```bash
# Terminal 1: Teacher sidecar
llama-server -m Bonsai-8B.gguf -ngl 99 --port 8080

# Terminal 2: Student training
python scripts/train_bonsai_mobile.py \
  --student-model TinyLlama-1.1B-Instruct \
  --teacher-api http://127.0.0.1:8080 \
  --training-data ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --output-dir ~/.bonsai/models/checkpoints/bonsai-mobile-v1 \
  --config config/bonsai_mobile_config.yaml \
  --epochs 3 \
  --learning-rate 5e-4 \
  --batch-size 4 \
  --device cpu
```

**Key Hyperparameters**:
- `--alpha 0.5`: Balance KL divergence (0=SFT, 1=pure distillation)
- `--temperature 4.0`: Softness of teacher targets (higher = softer)
- `--seed 42`: Reproducibility

#### Stage 3: Quantization

```bash
python scripts/quantize_bonsai_mobile.py \
  --final-model ~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model \
  --output-dir ~/.bonsai/models/releases \
  --model-name bonsai-mobile-v1 \
  --quantization Q4_K_M \
  --validate \
  --create-bkp
```

**Output**:
- `bonsai-mobile-v1.gguf` (~200-250 MB)
- `bonsai-mobile-v1.metadata.json` (model info)
- `bonsai-mobile-v1.bkp` (distribution package)
- `README.md` (model card)

#### Stage 4: Benchmarking

```bash
python scripts/benchmark_bonsai_mobile.py \
  --model ~/.bonsai/models/releases/bonsai-mobile-v1.gguf \
  --output-dir benchmark_results \
  --num-prompts 100 \
  --device cpu
```

**Outputs**:
- `benchmark_results.json` (raw metrics)
- `benchmark_report.md` (human-readable)

## Configuration

Edit `config/bonsai_mobile_config.yaml` to customize:

```yaml
student_model:
  architecture: "tinyllama"
  parameters: 1_100_000_000
  context_length: 2048
  lora:
    rank: 16
    alpha: 32

teacher_model:
  strategy: "llama_server_api"  # or "local_weights"
  api_url: "http://127.0.0.1:8080"
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

## Loss Function

### Knowledge Distillation Loss

The training combines two objectives:

```
Loss = α · KL(q(z|x)||p(z|x)) + (1 - α) · CE(student, hard_labels)

where:
  q(z|x) = student softmax(logits / T)
  p(z|x) = teacher softmax(logits / T)
  T = temperature (default 4.0)
  α = KL weight (default 0.5)
```

- **KL-divergence component**: Soft targets from teacher (helps generalization)
- **Cross-entropy component**: Hard labels from ground truth (prevents mode collapse)
- **Temperature**: Higher T makes teacher targets softer (helps small models)

### Why This Works

1. **Soft targets** (KL) encode teacher's full uncertainty → student generalizes better
2. **Hard targets** (CE) ground student to factual correctness
3. **Temperature scaling** (T > 1) smooths teacher logits → mobile model can match
4. **LoRA adapter** keeps base model frozen → faster convergence, smaller checkpoints

## Quality Gates

The pipeline implements several safeguards:

### Validation Monitoring
- Holds out 10% of data for validation
- Tracks loss on held-out set
- Saves checkpoint with best validation loss (not final)

### KL Divergence Check
- If KL loss diverges, automatically falls back to supervised fine-tuning
- Prevents mode collapse on small models

### Data Quality
- Filters examples by quality score (default ≥ 0.70)
- Domain balancing (code 40%, repair 20%, etc.)
- Duplicate detection (optional)

### Gradient Safety
- Gradient clipping (max norm = 1.0)
- Weight decay (0.01)
- Mixed precision (optional, not on CPU)

## Performance Metrics

### Throughput
- **TTFT** (Time To First Token): Latency before first output
- **TPT** (Time Per Token): Average latency per subsequent token
- **Tokens/sec**: Overall throughput

Example (Intel i7-12700K, Q4_K_M):
```
TinyLlama-1.1B:  ~50-80 tokens/sec (CPU)
Gemma-2B:        ~30-50 tokens/sec (CPU)
Phi-3-Mini:      ~40-70 tokens/sec (CPU)
```

### Memory
- **Peak VRAM**: During inference
- **Model Size**: GGUF file size (Q4_K_M ~200 MB for 1.1B)

Example (Q4_K_M):
```
1.1B model:    ~200 MB
3B model:      ~600 MB
7B model:      ~2.5 GB
```

## Data Provenance

Training data comes from multiple domains:

### Code (40%)
- Rust/Python/JavaScript examples
- Algorithm implementations
- Bug fixes and patches

### System Repair (20%)
- Survival system fixes (crashes, segfaults)
- Error-fix pairs
- Patches from eternal training loop

### Tool Use (20%)
- MCP (Model Context Protocol) examples
- Function calling examples
- Tool integration patterns

### Chat (15%)
- User-assistant conversations
- Quality-scored exchanges
- Domain-specific QA

### Q&A (5%)
- Academy documentation
- FAQ pairs
- Explanation questions

### Quality Scoring
- Generated vs. human-written
- Factual correctness
- Domain relevance
- Completeness

## Troubleshooting

### Training OOM (Out of Memory)

**On Windows CPU**:
```bash
# Reduce batch size
python scripts/train_bonsai_mobile.py --batch-size 2 --max-seq-len 512

# Or reduce LoRA rank
python scripts/train_bonsai_mobile.py --lora-rank 8

# Or use gradient accumulation
# (in config: gradient_accumulation_steps: 4)
```

### Teacher Server Won't Start

```bash
# Check port is free
netstat -ano | findstr :8080

# Kill existing process
taskkill /PID <PID> /F

# Start with more GPU layers
llama-server -m teacher.gguf -ngl 99 --port 8080 --log-all

# Check logs
type llama-server.log
```

### Training Loss Doesn't Decrease

```bash
# 1. Check data loading
python scripts/export_mobile_training_data.py --output test.jsonl

# 2. Reduce learning rate
python scripts/train_bonsai_mobile.py --learning-rate 1e-4

# 3. Increase temperature (softer targets)
python scripts/train_bonsai_mobile.py --temperature 8.0

# 4. Reduce alpha (more supervised learning, less distillation)
python scripts/train_bonsai_mobile.py --alpha 0.2
```

### GGUF Conversion Fails

```bash
# Ensure llama.cpp is built
cd ~/llama.cpp && make

# Check Python environment
python -c "from transformers import *; from peft import *"

# Run with verbose output
python scripts/quantize_bonsai_mobile.py --final-model ... --validate
```

### Benchmark Errors

```bash
# Install llama-cpp-python
pip install --upgrade llama-cpp-python

# Test GGUF loading
python -c "from llama_cpp import Llama; m = Llama('model.gguf'); m.create_completion('test')"
```

## Advanced Usage

### Custom Student Model

```bash
python scripts/train_bonsai_mobile.py \
  --student-model "meta-llama/Llama-2-7b-hf" \
  --teacher-api http://127.0.0.1:8080 \
  --training-data data.jsonl \
  --lora-rank 32 \
  --epochs 5
```

### In-Process Teacher (Needs 24+ GB VRAM)

```bash
# Load teacher weights directly (faster inference, uses more VRAM)
python scripts/train_bonsai_mobile.py \
  --student-model TinyLlama-1.1B-Instruct \
  --teacher-dir "D:/Models/general/Bonsai-8B-Instruct" \
  --training-data data.jsonl
```

### Custom Quantization

```bash
# Q5_K_M (better quality, larger)
python scripts/quantize_bonsai_mobile.py \
  --final-model final_model \
  --quantization Q5_K_M

# Q2_K (smaller, faster, lower quality)
python scripts/quantize_bonsai_mobile.py \
  --final-model final_model \
  --quantization Q2_K
```

### Weights & Biases Logging

```bash
export WANDB_PROJECT=bonsai-mobile-training

python scripts/train_bonsai_mobile.py \
  --wandb-project bonsai-mobile-training \
  --wandb-run "distill-v1" \
  ...
```

Then view at: https://wandb.ai/your-entity/bonsai-mobile-training

## Model Card Template

After training, a model card is auto-generated:

```markdown
# bonsai-mobile-v1

Quantized mobile model trained via knowledge distillation from Bonsai-8B.

## Model Details

- **Base Model**: TinyLlama-1.1B-Instruct
- **Quantization**: Q4_K_M
- **Size**: 220 MB
- **Context Length**: 2048

## Training

- **Teacher**: Bonsai-8B-Instruct
- **Method**: Knowledge Distillation
- **Epochs**: 3
- **Best Validation Loss**: 0.0234

## Usage

### llama-cpp-python

```python
from llama_cpp import Llama

model = Llama(model_path="bonsai-mobile-v1.gguf", n_ctx=2048)
response = model.create_completion("Write Python code to...")
print(response["choices"][0]["text"])
```

### llama-server

```bash
llama-server -m bonsai-mobile-v1.gguf -cb -ngl 35
curl http://localhost:8000/v1/completions -H "Content-Type: application/json" ...
```

## Performance

- **Tokens/sec**: 50-80 (CPU, Q4_K_M)
- **Memory**: 200 MB model + 512 MB runtime
- **Latency**: 20-40 ms TTFT, 12-25 ms per token

## License

See TinyLlama and Bonsai-8B licenses.
```

## Deployment

### iOS / Android

1. Convert GGUF to CoreML (iOS) or NNAPI (Android)
   ```bash
   # Use llama.cpp's conversion tools
   python convert.py model.gguf model.mlmodel
   ```

2. Integrate with inference library
   ```swift
   // iOS example
   let model = MLModel(contentsOf: modelURL)
   let predictor = try model.makePredictor()
   ```

### Docker / Server

```dockerfile
FROM python:3.10-slim

WORKDIR /app
COPY bonsai-mobile-v1.gguf .
COPY requirements.txt .
RUN pip install -r requirements.txt

CMD ["python", "-m", "llama_cpp.server", \
     "--model_path", "bonsai-mobile-v1.gguf", \
     "--host", "0.0.0.0", "--port", "8000"]
```

### Raspberry Pi

```bash
# Install
sudo apt install python3-dev
pip3 install llama-cpp-python

# Run
python3 -m llama_cpp.server --model_path bonsai-mobile-v1.gguf --port 8000

# Test
curl http://localhost:8000/v1/completions \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello", "max_tokens": 50}'
```

## Benchmarking Results

Example results from training pipeline:

```
Model                  TTFT    TPT     Tokens/sec  Memory (MB)
────────────────────────────────────────────────────────────
bonsai-mobile-v1       32ms    14ms    65.2        220
TinyLlama-1.1B         28ms    12ms    72.5        200
Phi-3-Mini             40ms    18ms    52.1        250
Gemma-2B               45ms    20ms    48.3        280
```

(CPU: Intel i7-12700K, Q4_K_M quantization)

## Integration with BonsAI

After training and quantization, register the model:

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
    hot_reload_target: false
```

Then use in the app:

```rust
// bonsai-runtime/src/lib.rs
let model = ModelRegistry::load("bonsai-mobile-v1")?;
let response = model.inference("What is a closure?", 2048)?;
```

## References

### Papers

- **Hinton et al. (2015)**: "Distilling the Knowledge in a Neural Network"
  - Foundational work on knowledge distillation
- **Sanh et al. (2019)**: "DistilBERT, a distilled version of BERT"
  - Practical application to Transformers

### Implementations

- **llama.cpp**: https://github.com/ggerganov/llama.cpp
- **PEFT**: https://github.com/huggingface/peft
- **Transformers**: https://huggingface.co/docs/transformers/

### Tools

- **Weights & Biases**: https://wandb.ai (monitoring)
- **HuggingFace Hub**: https://huggingface.co (model hosting)
- **Ollama**: https://ollama.com (local inference)

## Contributing

Found a bug? Have improvement suggestions?

1. File an issue with logs from `train_*.log`
2. Include your configuration (`config/bonsai_mobile_config.yaml`)
3. Attach `training_summary.json`

## License

BonsAI Mobile Training Pipeline is part of BonsAI and follows the same license.

---

**Last Updated**: June 2026
**Version**: 1.0.0
**Maintainer**: BonsAI Team
