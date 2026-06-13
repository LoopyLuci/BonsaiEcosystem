# BonsAI Mobile Training — Quick Start

## 5-Minute Setup

### Prerequisites

```bash
# Python 3.10+
python --version

# Install dependencies
pip install torch transformers peft pyyaml llama-cpp-python[server]

# Verify llama-server
llama-server --version
```

## One-Command Training

```powershell
# Windows PowerShell
cd z:\Projects\BonsaiWorkspace
.\scripts\run_mobile_training_pipeline.ps1 `
  -Epochs 3 `
  -BatchSize 4 `
  -Cleanup
```

**Total time**: ~4 hours (CPU) / ~1 hour (GPU)

---

## What Happens

| Phase | Time | Output |
|-------|------|--------|
| Download teacher | 20 min | Bonsai-8B GGUF (3.5 GB) |
| Generate prompts | 1 min | 8,000 training examples |
| Start teacher server | 2 min | HTTP API on port 8080 |
| Train model | 2 hours | Best checkpoint + logs |
| Quantize to GGUF | 30 min | 220 MB model |
| **Total** | **~4 hours** | **Ready for mobile** |

---

## Test the Model

```python
from llama_cpp import Llama

model = Llama(
    model_path="training_output/bonsai-mobile-v1/gguf/bonsai-mobile-q4_k_m.gguf",
    n_threads=8,
)

response = model.create_completion(
    prompt="Write a Python function to sort a list",
    max_tokens=256,
)

print(response['choices'][0]['text'])
```

---

## Customization

```powershell
# Different student model
.\scripts\run_mobile_training_pipeline.ps1 `
  -StudentModel "microsoft/phi-2" `
  -Epochs 5

# GPU acceleration
.\scripts\run_mobile_training_pipeline.ps1 `
  -Device cuda `
  -BatchSize 16 `
  -Epochs 3

# Larger dataset
python scripts/generate_mobile_prompts.py --num-prompts 20000
.\scripts\run_mobile_training_pipeline.ps1 -Epochs 5
```

---

## Output Files

```
training_output/bonsai-mobile-v1/
├── final_model/           # Trained PyTorch model
├── gguf/
│   ├── bonsai-mobile-q4_k_m.gguf        # Ready for deployment
│   ├── bonsai-mobile-q4_k_m.metadata.json
│   ├── README.md                        # Model card
│   └── bonsai-mobile-q4_k_m.bkp         # Distribution package
├── logs/                  # Training logs
└── training_summary.json  # Metrics
```

---

## Troubleshooting

**"llama-server not found"**
```bash
pip install llama-cpp-python[server]
```

**"Teacher server won't start"**
```bash
# Kill existing process on port 8080
netstat -ano | findstr :8080
taskkill /PID <PID> /F
```

**"Out of memory during training"**
```powershell
.\scripts\run_mobile_training_pipeline.ps1 `
  -BatchSize 2 `
  -Device cpu
```

**"Training is too slow"**
```powershell
.\scripts\run_mobile_training_pipeline.ps1 `
  -Device cuda `  # Use GPU
  -BatchSize 16
```

---

## Deploy to Mobile

### iOS
1. Use llama.cpp iOS SDK: https://github.com/ggerganov/llama.cpp/tree/master/examples/llama.swiftui
2. Copy GGUF file to app bundle
3. Load with `LlamaContext(modelPath:)`

### Android
1. Use llama.cpp Android SDK: https://github.com/ggerganov/llama.cpp/tree/master/examples/llama.android
2. Copy GGUF to app assets
3. Initialize inference engine

---

## Performance Expectations

| Device | Speed | Memory | Latency |
|--------|-------|--------|---------|
| iPhone 14 Pro | 15 tok/s | 800 MB | 300ms (first) |
| MacBook M2 | 40 tok/s | 600 MB | 100ms |
| RTX 4090 | 800 tok/s | 2 GB | 10ms |
| Raspberry Pi 4 | 2 tok/s | 2.0 GB | 5s |

---

## Next Steps

1. **Review model**: `cat training_output/bonsai-mobile-v1/gguf/README.md`
2. **Test inference**: Run Python example above
3. **Finetune**: `python scripts/finetune_bonsai_mobile.py --training-data custom.jsonl`
4. **Deploy**: Use llama.cpp mobile SDKs
5. **Monitor**: Track inference latency in production

---

## Advanced Options

See `docs/11-MOBILE-TRAINING-PIPELINE.md` for:
- Custom student models (Phi-2, Falcon, etc.)
- In-process distillation (requires more VRAM)
- Domain-specific finetuning
- Different quantization types
- Model registry integration
- Federated learning setup

---

**Support**: See `docs/11-MOBILE-TRAINING-PIPELINE.md` for detailed docs
