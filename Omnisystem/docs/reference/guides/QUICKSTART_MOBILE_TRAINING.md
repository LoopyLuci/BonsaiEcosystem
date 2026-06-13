# BonsAI Mobile Training Pipeline — Quick Start

**TL;DR**: Train a mobile-optimized model from Bonsai-8B in 5 steps (~8 hours)

## Requirements

- **Models**: Bonsai-8B GGUF (~4-6 GB) + TinyLlama-1.1B base model
- **Tools**: Python 3.10+, llama.cpp, PyTorch
- **Hardware**: 64 GB RAM (Windows AMD 7900 XTX, CPU training)
- **Time**: ~8 hours (CPU), ~2 hours (GPU with VRAM)

## 5-Step Quick Start

### Step 1: Prepare Environment (5 min)

```bash
# Check Python
python --version  # 3.10+

# Install dependencies
pip install torch transformers peft pyyaml numpy

# Optional: GGUF inference
pip install llama-cpp-python

# Clone llama.cpp if not present
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp && make
```

### Step 2: Export Training Data (10 min)

```bash
python scripts/export_mobile_training_data.py \
  --output ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --max-examples 100000 \
  --min-quality 0.70
```

✓ Output: `combined_mobile_training.jsonl` (~400 MB)

### Step 3: Start Teacher Server (1 min)

Open a **new terminal**:

```bash
# Windows PowerShell
llama-server.exe `
  -m "D:\Models\general\Bonsai-8B-Q4_K_M.gguf" `
  -ngl 99 --port 8080 --no-mmap

# Or on Mac/Linux
llama-server -m ~/Models/Bonsai-8B-Q4_K_M.gguf -ngl 99 --port 8080
```

Wait for: `Server listening on http://127.0.0.1:8080`

### Step 4: Run Training (4-8 hours)

In **second terminal**:

```bash
python scripts/train_bonsai_mobile.py \
  --student-model TinyLlama-1.1B-Instruct \
  --teacher-api http://127.0.0.1:8080 \
  --training-data ~/.bonsai/training_export/combined_mobile_training.jsonl \
  --output-dir ~/.bonsai/models/checkpoints/bonsai-mobile-v1 \
  --config config/bonsai_mobile_config.yaml \
  --epochs 3 \
  --batch-size 4 \
  --learning-rate 5e-4 \
  --device cpu
```

Monitor progress in logs:
```bash
tail -f ~/.bonsai/models/checkpoints/bonsai-mobile-v1/train_*.log
```

✓ Output: `~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model/` (HF format)

### Step 5: Quantize to GGUF (10 min)

```bash
python scripts/quantize_bonsai_mobile.py \
  --final-model ~/.bonsai/models/checkpoints/bonsai-mobile-v1/final_model \
  --output-dir ~/.bonsai/models/releases \
  --quantization Q4_K_M \
  --validate \
  --create-bkp
```

✓ Output: `~/.bonsai/models/releases/bonsai-mobile-v1.gguf` (~220 MB)

---

## Optional: Benchmark & Register (15 min)

### Benchmark Performance

```bash
python scripts/benchmark_bonsai_mobile.py \
  --model ~/.bonsai/models/releases/bonsai-mobile-v1.gguf \
  --output-dir benchmark_results \
  --num-prompts 50
```

**Example results**:
```
TTFT: 32 ms
TPT: 14 ms
Throughput: 65 tokens/sec
Memory: 220 MB
```

### Register in BonsAI

```bash
# Copy to registry
cp ~/.bonsai/models/releases/bonsai-mobile-v1.gguf ~/.bonsai/models/releases/

# Update config/model_registry.yaml with:
# - name: "bonsai-mobile-v1"
#   path: "~/.bonsai/models/releases/bonsai-mobile-v1.gguf"
#   role: "student_mobile"
```

---

## Or Use Orchestration Script (Automated)

All 5 steps in one command:

```powershell
.\scripts\run_mobile_training_pipeline.ps1 `
  -StudentModel "TinyLlama-1.1B-Instruct" `
  -TeacherGGUF "D:\Models\general\Bonsai-8B-Q4_K_M.gguf" `
  -Epochs 3 `
  -LearningRate 0.0005
```

---

## Using the Trained Model

### Inference with llama-cpp-python

```python
from llama_cpp import Llama

model = Llama(model_path="bonsai-mobile-v1.gguf", n_ctx=2048)

response = model.create_completion(
    prompt="Write a Python function to find the longest palindrome:",
    max_tokens=256,
    temperature=0.7,
)

print(response["choices"][0]["text"])
```

### Inference with llama-server

```bash
# Start server
llama-server -m bonsai-mobile-v1.gguf -cb

# Query via API
curl http://localhost:8000/v1/completions \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Write a Python function to...",
    "max_tokens": 256,
    "temperature": 0.7
  }'
```

### In BonsAI App

```rust
// bonsai-runtime/src/lib.rs
let model = ModelRegistry::load("bonsai-mobile-v1")?;
let response = model.inference("Write Rust code for...", 2048)?;
```

---

## Troubleshooting

| Problem | Fix |
|---------|-----|
| **Teacher won't connect** | Kill existing: `taskkill /PID <pid> /F` then restart |
| **Training OOM** | Reduce `--batch-size 2` or `--max-seq-len 512` |
| **Loss not decreasing** | Try `--learning-rate 1e-4` or `--temperature 8.0` |
| **GGUF validation fails** | Ensure llama.cpp is built: `cd llama.cpp && make` |
| **Segfault on Windows** | Use PowerShell, not Git Bash |

See `docs/MOBILE_TRAINING_PIPELINE.md` for detailed troubleshooting.

---

## Configuration Tuning

Edit `config/bonsai_mobile_config.yaml`:

```yaml
# For faster training (less data)
data:
  max_examples: 50000

# For better quality (longer training)
training:
  epochs: 5
  learning_rate: 1e-4

# For smaller model (faster inference)
student_model:
  lora:
    rank: 8

# For larger model (better quality)
student_model:
  lora:
    rank: 32
```

---

## What You're Training

**Student Model**: TinyLlama-1.1B (mobile-optimized)
**Teacher Model**: Bonsai-8B (knowledge source)
**Method**: Knowledge Distillation (KL + CE loss)
**Output**: bonsai-mobile-v1.gguf (220 MB, Q4_K_M)

**Domains Trained**:
- Code (40%): Python, Rust, JavaScript
- System Repair (20%): Crash fixes, patches
- Tool Use (20%): Function calling, MCP
- Chat (15%): Conversations
- Q&A (5%): FAQs, documentation

---

## Next Steps

1. **Deploy to Mobile**: Convert GGUF to CoreML (iOS) or NNAPI (Android)
2. **Fine-tune for Domain**: Train on your specific data
3. **Monitor Inference**: Track latency and throughput
4. **Optimize Further**: Try Q2_K quantization for smaller size

---

## Performance Targets

| Metric | Value |
|--------|-------|
| Model Size | 220 MB (Q4_K_M) |
| Throughput | 50-80 tokens/sec |
| TTFT | 30-50 ms |
| Memory | ~512 MB runtime |
| Quality | ~95% of teacher |

---

## Files Created

After running the full pipeline:

```
~/.bonsai/models/
├── checkpoints/bonsai-mobile-v1/
│   ├── final_model/              # HF format (intermediate)
│   ├── training_summary.json
│   └── train_*.log
├── releases/
│   ├── bonsai-mobile-v1.gguf     # Final quantized model ✓
│   ├── bonsai-mobile-v1.metadata.json
│   ├── bonsai-mobile-v1.bkp      # Distribution package
│   └── README.md                 # Model card
└── benchmark_results/
    ├── benchmark_results.json
    └── benchmark_report.md
```

---

## Documentation

- **Full guide**: `docs/MOBILE_TRAINING_PIPELINE.md`
- **File index**: `MOBILE_PIPELINE_INDEX.md`
- **Config reference**: `config/bonsai_mobile_config.yaml`

---

**Ready to train?** Start with Step 1 above. You'll have a production-ready mobile model in ~8 hours.

Questions? See `docs/MOBILE_TRAINING_PIPELINE.md` or check logs in training output directory.

Good luck! 🚀
