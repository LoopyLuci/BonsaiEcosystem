# Public Models Directory

Storage location for publicly available machine learning models.

## Directory Structure

```
models/
├── README.md                   # This file
├── base-models/                # Pre-trained base models
│   └── [model directories]
├── checkpoints/                # Training checkpoints
│   └── [checkpoint directories]
└── quantized/                  # Quantized model versions
    └── [quantized models]
```

## Adding Public Models

To add a public model to this directory:

1. Create a subdirectory with the model name
2. Add model files (weights, config, tokenizer, etc.)
3. Include a model-specific README with:
   - Model description
   - Architecture details
   - Usage instructions
   - License information

## Model Requirements

Public models should be:
- ✅ Non-proprietary
- ✅ Safe to share publicly
- ✅ Reasonably sized (< 500MB preferred)
- ✅ Fully documented
- ✅ Licensed appropriately

## Using Models in Code

```python
import os

# Reference public model
public_models_dir = os.path.join(
    os.path.dirname(__file__), 
    "models"
)

model_path = os.path.join(public_models_dir, "base-models", "your-model")
```

## Integration with BUEB

For hardware-aware device allocation:

```python
from bonsai_backend import *

# Initialize hardware detection
initialize()

# Allocate devices
task = TaskRequirements(
    task_type=TaskType.Inference,
    estimated_memory_bytes=500_000_000,
    precision=Precision.Auto,
    allow_fallback=True
)

allocation = allocate(task)

# Load model on allocated device
model = load_model(model_path, allocation.devices[0])
```

## Best Practices

- ✅ Keep individual models < 500MB
- ✅ Include comprehensive documentation
- ✅ Specify license clearly
- ✅ Document training data source
- ✅ Include usage examples
- ✅ Use standard formats (PyTorch, ONNX, etc.)

## License

Models in this directory should have clear licensing. Include a LICENSE or license information in each model's README.

---

**Last Updated**: June 3, 2026
