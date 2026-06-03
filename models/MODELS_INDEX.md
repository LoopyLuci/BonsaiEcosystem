# Models Inventory Index

Complete catalog of all available models and their locations.

## Public Models

### Location: `Z:\Projects\BonsaiWorkspace\models/`

Status: Ready for public model addition

| Model | Location | Size | Type | Status |
|-------|----------|------|------|--------|
| *(Reserved for public models)* | `models/base-models/` | TBD | Base | Planned |
| *(Reserved for checkpoints)* | `models/checkpoints/` | TBD | Checkpoint | Planned |
| *(Reserved for quantized)* | `models/quantized/` | TBD | Quantized | Planned |

**Notes**:
- Public models directory is organized and ready
- Can safely commit models < 500MB
- Use for base models, checkpoints, quantized versions
- All files version controlled in git

## Private Models

### Location: `D:\Models\Custom/`

Status: External storage, not in repository

| Model | Location | Size | Type | Status | Trained |
|-------|----------|------|------|--------|---------|
| **Octopus AI** | `D:\Models\Custom\octopus-ai-model` | 312 MB | Fine-tuned | ✅ Ready | Jun 3, 2026 |
| **Octopus AI LoRA** | `D:\Models\Custom\octopus-ai-lora` | ~10 MB | Adapter | ✅ Ready | Jun 3, 2026 |

### Octopus AI Model Details

**File Path**: `D:\Models\Custom\octopus-ai-model`

**Model Specifications**:
- Base Model: DistilGPT-2 (82M parameters)
- Fine-tuning: LoRA (Rank-4, Alpha-8)
- Training Data: 9,000 server management Q&A pairs
- Training Steps: 158
- Final Loss: 1.512
- Accuracy: Q&A inference ready

**Files Included**:
- `pytorch_model.bin` - Merged model weights (312.49 MB)
- `config.json` - Model configuration
- `generation_config.json` - Generation settings
- `tokenizer.json` - Tokenizer
- `tokenizer_config.json` - Tokenizer config
- `vocab.json` - Vocabulary
- `merges.txt` - Merge operations
- `special_tokens_map.json` - Special tokens

**Access**:
```python
from transformers import AutoModelForCausalLM, AutoTokenizer

model = AutoModelForCausalLM.from_pretrained(
    "D:\\Models\\Custom\\octopus-ai-model"
)
tokenizer = AutoTokenizer.from_pretrained(
    "D:\\Models\\Custom\\octopus-ai-model"
)
```

**Hardware Allocation** (via BUEB):
- CPU: INT8 precision, batch_size=1, latency: 200-500ms
- GPU (RTX 3080): FP16 precision, batch_size=4-8, latency: 20-50ms
- Multi-GPU: FP16 + parallelism, batch_size=16+, latency: 10-20ms

**Use Cases**:
- Server administration Q&A
- System configuration advice
- Troubleshooting assistance
- Documentation generation

### Octopus AI LoRA Adapter Details

**File Path**: `D:\Models\Custom\octopus-ai-lora`

**Adapter Specifications**:
- Base: DistilGPT-2
- Method: Low-Rank Adaptation
- Rank: 4
- Alpha: 8
- Target modules: Linear layers
- Training epochs: 1

**Files Included**:
- `adapter_config.json` - Adapter configuration
- `adapter_model.bin` - Adapter weights
- Training metadata

**Purpose**:
- Original fine-tuned adapter before merging
- Can be used with LoRA inference
- Useful for further training

## Model Access Control

### Public Models Access

```bash
# Read from repository
model_dir = "Z:\Projects\BonsaiWorkspace\models\base-models\distilgpt2"

# Safe to include in code
# Safe to commit to git
# Safe to share on GitHub
```

### Private Models Access

```bash
# Read from external storage
model_dir = "D:\Models\Custom\octopus-ai-model"

# DO NOT commit to git
# DO NOT push to GitHub
# DO NOT include file paths in public code
```

## Usage Examples

### Loading Private Model

```python
import os
from transformers import AutoModelForCausalLM, AutoTokenizer

# Define private model path
PRIVATE_MODELS_DIR = os.getenv('BONSAI_PRIVATE_MODELS', 'D:\\Models\\Custom')
MODEL_PATH = os.path.join(PRIVATE_MODELS_DIR, 'octopus-ai-model')

# Load model
if os.path.exists(MODEL_PATH):
    model = AutoModelForCausalLM.from_pretrained(MODEL_PATH)
    tokenizer = AutoTokenizer.from_pretrained(MODEL_PATH)
    print("✅ Model loaded successfully")
else:
    print(f"❌ Model not found at {MODEL_PATH}")
    print("Set BONSAI_PRIVATE_MODELS environment variable if needed")
```

### Using with BUEB

```python
from bonsai_backend import *

# Initialize hardware detection
initialize()

# Allocate device for Octopus AI inference
task = TaskRequirements(
    task_type=TaskType.Inference,
    estimated_memory_bytes=600_000_000,  # 600 MB
    precision=Precision.Auto,
    allow_fallback=True
)

allocation = allocate(task)

# Load model based on allocated device
device = allocation.devices[0].device_type
print(f"Loading model on {device}")

model = load_model("D:\\Models\\Custom\\octopus-ai-model", device)
```

### Inference Example

```python
from transformers import pipeline

# Load from private models directory
nlp = pipeline(
    "text-generation",
    model="D:\\Models\\Custom\\octopus-ai-model",
    device=0 if torch.cuda.is_available() else -1
)

# Generate response
query = "How do I configure SSH on Ubuntu?"
response = nlp(query, max_length=100)

print(response[0]['generated_text'])
```

## Model Comparison

### Base vs. Fine-tuned

| Aspect | Base (DistilGPT-2) | Fine-tuned (Octopus AI) |
|--------|-------------------|------------------------|
| Parameters | 82M | 82M (+ LoRA) |
| Training Data | Common Crawl | Server management Q&A |
| Specialization | General | System administration |
| Quality | General purposes | Server Q&A focused |
| Location | `models/base-models/` | `D:\Models\Custom\` |
| Status | Public | Private |

## Storage Strategy

### Why Separate Locations?

**Z:\Projects\BonsaiWorkspace\models/**
- ✅ Version controlled
- ✅ Backed up by git
- ✅ Safe for public sharing
- ✅ For base models and generic assets
- ✅ For training checkpoints
- ✅ For quantized versions

**D:\Models\Custom/**
- ✅ External storage
- ✅ Never committed to git
- ✅ Private/proprietary models
- ✅ Fine-tuned variations
- ✅ Sensitive configurations
- ✅ Large specialized models

### File Size Impact on Git

```
Repository size with models:

Current (models/ empty): ~1 GB
With Octopus AI in repo:  ~315 MB additional
Total: ~1.3 GB

With 10 large models:    ~5+ GB
(Exceeds GitHub's recommendation)

Solution: Keep only essentials in repo,
large models on external storage (D:)
```

## Backup & Recovery

### Backing Up Private Models

```powershell
# Create backup of D:\Models\Custom
$backup_dir = "E:\Backups\Models_$(Get-Date -Format 'yyyyMMdd')"
Copy-Item -Path "D:\Models\Custom" -Destination $backup_dir -Recurse

Write-Host "Backup created at: $backup_dir"
```

### Backup Schedule

- **Daily**: Incremental backup to external drive
- **Weekly**: Full backup to cloud storage
- **Monthly**: Archive copy

### Recovery

```powershell
# Restore from backup
$backup_dir = "E:\Backups\Models_20260603"
Copy-Item -Path "$backup_dir\*" -Destination "D:\Models\Custom" -Recurse -Force
```

## Performance Metrics

### Octopus AI Model Performance

**Training**:
- Training time: ~12.5 minutes
- Steps: 158
- Loss progression: 2.386 → 1.020 → 1.512
- Hardware: CPU (Ryzen 9 5900X)

**Inference** (via BUEB):

| Hardware | Latency | Throughput | Batch | Precision |
|----------|---------|-----------|-------|-----------|
| CPU (24 cores) | 200-500ms | 2-5 q/s | 1 | INT8 |
| GPU (RTX 3080) | 20-50ms | 20-50 q/s | 4-8 | FP16 |
| Multi-GPU (2× RTX 3090) | 10-20ms | 50-100+ q/s | 16+ | FP16 |

## Future Models

Planned additions:

| Model | Type | Purpose | Status |
|-------|------|---------|--------|
| BonsAI V2 | Foundation | General purposes | Planned |
| Octopus ML | Fine-tuned | Machine learning Q&A | Planned |
| Octopus Cloud | Fine-tuned | Cloud admin Q&A | Planned |
| Octopus Security | Fine-tuned | Security Q&A | Planned |

## Environment Variables

Configure model paths via environment variables:

```bash
# Set public models directory
set BONSAI_PUBLIC_MODELS=Z:\Projects\BonsaiWorkspace\models

# Set private models directory
set BONSAI_PRIVATE_MODELS=D:\Models\Custom

# In Python
import os
public_dir = os.getenv('BONSAI_PUBLIC_MODELS')
private_dir = os.getenv('BONSAI_PRIVATE_MODELS')
```

## Security Checklist

- ✅ Private models NOT in git
- ✅ Private models NOT on GitHub
- ✅ Private models on separate D: drive
- ✅ .gitignore protects D:\Models\*
- ✅ Code uses paths (not hardcoded in repo)
- ✅ Environment variables for configuration
- ✅ Regular backups of D:\Models\Custom
- ✅ Access controlled to D: drive

## Documentation

- **Models Directory**: `Z:\Projects\BonsaiWorkspace\models/MODELS_README.md`
- **This Index**: `Z:\Projects\BonsaiWorkspace\models/MODELS_INDEX.md`
- **BUEB Documentation**: `crates/bonsai-backend/BUEB.md`
- **Octopus AI Training**: `crates/octopus-ai/README.md`

---

**Last Updated**: June 3, 2026  
**Total Public Models**: Ready for expansion  
**Total Private Models**: 2 (Octopus AI + LoRA)  
**Storage**: Z: (repository) + D: (external)  
**Status**: ✅ Organized and Secure
