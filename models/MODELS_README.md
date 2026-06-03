# Models Directory

Storage location for public machine learning models and model assets.

## Location

`Z:\Projects\BonsaiWorkspace\models/` - **Public models** (repository-safe)  
`D:\Models\Custom/` - **Private models** (external, never committed)

## Directory Structure

```
Z:\Projects\BonsaiWorkspace/
└── models/                     # PUBLIC MODELS
    ├── MODELS_README.md        # This file
    ├── MODELS_INDEX.md         # Model inventory
    ├── base-models/            # Base pre-trained models
    │   ├── distilgpt2/         # DistilGPT-2 base
    │   └── [other public bases]
    ├── checkpoints/            # Training checkpoints
    │   └── [saved states]
    ├── quantized/              # Quantized versions
    │   └── [Q4_K_M, GGUF files]
    └── [other public model assets]

D:\Models\Custom/              # PRIVATE MODELS (External)
├── octopus-ai-model/          # Fine-tuned Octopus AI
│   ├── pytorch_model.bin       # 312 MB merged model
│   ├── config.json
│   ├── tokenizer.json
│   └── [model files]
│
├── octopus-ai-lora/            # LoRA adapter
│   ├── adapter_config.json
│   ├── adapter_model.bin
│   └── [adapter files]
│
└── [other custom/fine-tuned models]
```

## Model Categories

### Public Models (`Z:\Projects\BonsaiWorkspace\models/`)

**Purpose**: Models that are safe to include in the repository
- Base models (DistilGPT-2, etc.)
- Quantized versions (Q4_K_M format)
- Training checkpoints
- Model configurations
- Model metadata

**Characteristics**:
- ✅ Version controlled in git
- ✅ Small file sizes (< 500MB typically)
- ✅ Non-proprietary
- ✅ Can be published with code
- ✅ Safe to commit

### Private Models (`D:\Models\Custom/`)

**Purpose**: Models that should never be committed to the repository
- Fine-tuned models (Octopus AI)
- LoRA adapters
- Proprietary training outputs
- Custom implementations
- Sensitive configurations

**Characteristics**:
- ❌ NOT version controlled
- ❌ Stored externally on D: drive
- ❌ Never pushed to GitHub
- ❌ Personal/proprietary
- ❌ Large files (usually 300MB+)

## Current Models

### Public Models

Currently, the public models directory is ready for:
- [ ] DistilGPT-2 base model
- [ ] Quantized model versions
- [ ] Configuration files
- [ ] Training checkpoints

### Private Models (External)

**Location**: `D:\Models\Custom/`

**octopus-ai-model/** (312 MB)
- Fine-tuned Octopus AI server management model
- Based on DistilGPT-2
- Trained on 9,000 instruction-response pairs
- Ready for inference
- Storage: D:\Models\Custom\octopus-ai-model

**octopus-ai-lora/** 
- LoRA adapter for the model
- Rank-4, Alpha-8 configuration
- Used during training
- Storage: D:\Models\Custom\octopus-ai-lora

## Access Paths

### Public Models

```python
# In code
PUBLIC_MODEL_DIR = "Z:\\Projects\\BonsaiWorkspace\\models"
model_path = os.path.join(PUBLIC_MODEL_DIR, "base-models", "distilgpt2")
```

### Private Models

```python
# In code
PRIVATE_MODEL_DIR = "D:\\Models\\Custom"
octopus_model = os.path.join(PRIVATE_MODEL_DIR, "octopus-ai-model")
```

## Using Private Models

### Loading Octopus AI Model

```python
from transformers import AutoTokenizer, AutoModelForCausalLM
import os

# Private model location
PRIVATE_MODEL_PATH = "D:\\Models\\Custom\\octopus-ai-model"

# Load model
model = AutoModelForCausalLM.from_pretrained(PRIVATE_MODEL_PATH)
tokenizer = AutoTokenizer.from_pretrained(PRIVATE_MODEL_PATH)

# Inference
inputs = tokenizer("How do I configure SSH?", return_tensors="pt")
outputs = model.generate(**inputs, max_length=100)
response = tokenizer.decode(outputs[0])
```

### Using with BUEB Hardware Abstraction

```rust
use bonsai_backend::*;

// Initialize BUEB
initialize()?;

// Allocate device for inference
let task = TaskRequirements {
    task_type: TaskType::Inference,
    estimated_memory_bytes: 600_000_000,  // 600 MB for Octopus AI
    precision: Precision::Auto,
    allow_fallback: true,
};

let allocation = allocate(&task);
println!("Using device: {}", allocation.devices[0].device_type);
println!("Precision: {}", allocation.precision);

// Load model with allocated device
// ... load model based on allocation.devices[0]
```

## Adding New Models

### Adding Public Models

1. **Create subdirectory** under `models/`
   ```
   models/new-model-name/
   ```

2. **Add model files**
   ```
   models/new-model-name/
   ├── model.bin or config.json
   ├── tokenizer.json
   └── README.md (describe the model)
   ```

3. **Update MODELS_INDEX.md**
   ```markdown
   ## New Model Name
   - Location: models/new-model-name/
   - Size: [size]
   - License: [license]
   - Description: [brief description]
   ```

4. **Commit to git**
   ```bash
   git add models/new-model-name
   git commit -m "feat: Add new model - [name]"
   ```

### Adding Private Models

1. **Create directory** on D: drive
   ```
   D:\Models\Custom\model-name\
   ```

2. **Copy model files** to external location
   ```powershell
   Copy-Item -Path "local\model" -Destination "D:\Models\Custom\model-name" -Recurse
   ```

3. **Add to D:\Models\Custom\INDEX.md** (not repo)
   - Document location
   - Document access method
   - Document size and requirements

4. **Reference in code** but don't commit model files
   - Use path: `D:\Models\Custom\model-name`
   - Document in code where this path comes from

## Model Management

### Listing Models

**Public models**:
```bash
ls -lah Z:\Projects\BonsaiWorkspace\models\
```

**Private models**:
```bash
ls -lah D:\Models\Custom\
```

### Cleaning Up

**Remove unused public model**:
```bash
# Remove directory
Remove-Item -Path "Z:\Projects\BonsaiWorkspace\models\old-model" -Recurse

# Commit removal
git add models/
git commit -m "refactor: Remove obsolete model"
```

**Remove unused private model**:
```bash
# Remove from D: drive
Remove-Item -Path "D:\Models\Custom\old-model" -Recurse
# No git commit needed (external storage)
```

## Storage Limits

### Public Models (Repository)

- **Target total**: < 2 GB
- **Individual model**: < 500 MB preferred
- **Rationale**: GitHub push size limits

### Private Models (External)

- **No limit** on D: drive
- **Backup responsibility**: Your own backup
- **Disk space**: Monitor available space on D:

## Model Serving

### Local Inference

```python
from transformers import pipeline

# Using BUEB-allocated device
device = "cuda" if torch.cuda.is_available() else "cpu"

# Load private model
pipe = pipeline("text-generation", 
                model="D:\\Models\\Custom\\octopus-ai-model",
                device=device)

# Generate
response = pipe("How do I configure SSH?")
```

### API Server

```python
# FastAPI example
from fastapi import FastAPI
import torch
from transformers import AutoTokenizer, AutoModelForCausalLM

app = FastAPI()

# Load model once at startup
model = AutoModelForCausalLM.from_pretrained(
    "D:\\Models\\Custom\\octopus-ai-model"
)
tokenizer = AutoTokenizer.from_pretrained(
    "D:\\Models\\Custom\\octopus-ai-model"
)

@app.post("/generate")
async def generate(query: str):
    inputs = tokenizer(query, return_tensors="pt")
    outputs = model.generate(**inputs, max_length=100)
    return {"response": tokenizer.decode(outputs[0])}
```

## Integration with Bonsai Components

### With Octopus AI

```python
# octopus-ai inference
from bonsai_backend import *

initialize()
allocation = allocate(TaskRequirements(...))

# Load from D:\Models\Custom\octopus-ai-model
model = load_model("D:\\Models\\Custom\\octopus-ai-model")
```

### With BMF (Messaging Fabric)

```python
# Message encoding using models
from bonsai_bmf_core import encode_message

# Models help with spam detection, encryption, etc.
encoded = encode_message(
    "Your message here",
    model_dir="D:\\Models\\Custom\\octopus-ai-model"
)
```

### With KDB (Knowledge Database)

```python
# Vector embeddings for search
from bonsai_kdb import HNSW

# Use model for embeddings
embeddings = model.encode(["document 1", "document 2"])
index = HNSW.create(embeddings)
```

## Security

### Private Model Protection

✅ **D:\Models\Custom is NOT**:
- Version controlled in git
- Pushed to GitHub
- Included in .gitignore exceptions
- Accessible from repository code directly

✅ **Code references D:\Models\Custom**:
- Via environment variables
- Via configuration files
- Via hardcoded paths (documented)
- Via CLI arguments

✅ **Backups are YOUR responsibility**:
- Regular backup of D:\Models\Custom
- Cloud storage backup
- External hard drive backup

### Repository Security

✅ **models/ directory in repository**:
- Only contains public, safe models
- Fully version controlled
- Safe to share on GitHub
- Reviewed before commit

✅ **.gitignore protection**:
```
# Never track private models
D:\Models\*
*.private
**/private/**
```

## File Size Reference

| Model | Size | Location | Type |
|-------|------|----------|------|
| DistilGPT-2 (base) | ~350 MB | `models/base-models/` | Public |
| Octopus AI (fine-tuned) | ~312 MB | `D:\Models\Custom\` | Private |
| Octopus AI (LoRA) | ~10 MB | `D:\Models\Custom\` | Private |
| Quantized (GGUF, Q4) | ~100 MB | `models/quantized/` | Public |

## Troubleshooting

### Model Not Found

```python
# Check path
import os
model_path = "D:\\Models\\Custom\\octopus-ai-model"
if os.path.exists(model_path):
    print("Model found")
else:
    print("Model not found!")
    print("Available:", os.listdir("D:\\Models\\Custom\\"))
```

### Model Loading Error

```python
# Verify model integrity
from transformers import AutoConfig

try:
    config = AutoConfig.from_pretrained(
        "D:\\Models\\Custom\\octopus-ai-model"
    )
    print("Model config valid:", config)
except Exception as e:
    print("Error:", e)
```

### Disk Space Issues

```powershell
# Check D: drive space
Get-Volume D:

# List large files
Get-ChildItem -Path "D:\Models\Custom" -Recurse | 
    Sort-Object -Property Length -Descending | 
    Select-Object -First 10
```

## Best Practices

✅ **DO**:
- Store private models on D: drive
- Document private model locations
- Use BUEB for device allocation
- Keep public models small and curated
- Backup D:\Models\Custom regularly

❌ **DON'T**:
- Commit private models to git
- Push to GitHub without review
- Store credentials in model directories
- Use repository models for proprietary work
- Ignore D:\Models\ in .gitignore

## Future Plans

Potential model additions:
- [ ] BonsAI V2 base model
- [ ] Domain-specific fine-tuned models
- [ ] Multilingual Octopus AI variants
- [ ] Quantized GGUF versions
- [ ] Model conversion utilities

## References

- **BUEB Documentation**: `crates/bonsai-backend/BUEB.md`
- **Octopus AI Training**: `crates/octopus-ai/README.md`
- **BMF Specification**: `docs/specifications/BMF_MESSAGING_SPECIFICATION.md`
- **KDB Documentation**: `crates/bonsai-kdb/README.md`

---

**Last Updated**: June 3, 2026  
**Status**: ✅ Organized and Documented  
**Public Models**: Ready for expansion  
**Private Models**: D:\Models\Custom (secure, external)
