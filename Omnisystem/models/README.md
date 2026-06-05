# Omnisystem AI Models

Complete reference guide to all **Omni-AI** models within the Omnisystem ecosystem.

> **Note:** Within the Omnisystem, all BonsAI models are renamed to **Omni-AI** to reflect their integration into the sovereign computing platform.

---

## AI Model Inventory

| Model | Type | Status | Location |
|-------|------|--------|----------|
| **Omni-AI Octopus** | Vision-Language Fine-tuned | ✅ Ready | `D:\Models\Custom\octopus-ai-model/` |
| **Omni-AI Poe** | Personality-based Agent | 🔄 Training | `Omnisystem/omni-ai/poe-ai/` |
| **Omni-AI Registry** | Model Management System | ✅ Ready | `Omnisystem/omni-ai/registry/` |

---

## 1. Omni-AI Octopus

### Overview
Fine-tuned vision-language model based on instruction-following architecture. Originally trained as part of the Bonsai ecosystem, now integrated as **Omni-AI Octopus** within the Omnisystem.

### Files & Location
```
Private Storage (NOT in repo):
  D:\Models\Custom\octopus-ai-model/
    ├── pytorch_model.bin         (312 MB)
    ├── config.json               (Model config)
    ├── tokenizer.json            (BPE tokenizer)
    ├── special_tokens_map.json   (Special tokens)
    └── training_args.bin         (Training metadata)

Public Reference (in repo):
  Omnisystem/models/octopus-ai/
    └── README.md                 (Instructions to access model)
```

### Training Specification
- **Base Model**: Llama-2 7B
- **Training Data**: 9,000 instruction-response pairs
- **Validation Data**: 1,000 examples
- **Fine-tuning Method**: LoRA (Low-Rank Adaptation)
- **LoRA Rank**: 16
- **LoRA Alpha**: 32
- **Merged**: Yes (adapter merged into base model)

### Capabilities
✅ Visual reasoning and understanding  
✅ Instruction following with high fidelity  
✅ Context-aware responses  
✅ Domain-specific knowledge (training corpus)

### Integration with Omnisystem
Within the Omnisystem, Omni-AI Octopus is used as:
- **Language model component** for natural language processing in `sylva/` runtime
- **Knowledge base** for fact retrieval in `axiom/` proof tactics
- **Decision engine** for `uvm/` validation mesh strategies

### Using Omni-AI Octopus

**In Titan code:**
```titan
extern "omni-ai" {
    fn octopus_infer(prompt: i64, max_tokens: i64) -> i64;
    fn octopus_load_context(context_hash: i64) -> i64;
}

pub fn perform_reasoning(query_hash: i64) -> i64 {
    // Dispatch to Omni-AI Octopus
    let response = octopus_infer(query_hash, 256);
    return response;
}
```

**In Python (for training):**
```python
from omnisystem.omni_ai.octopus import OctopusModel

model = OctopusModel.load("D:\\Models\\Custom\\octopus-ai-model")
output = model.infer("What is the Omnisystem?", max_tokens=256)
```

### Files
- `Omnisystem/models/octopus-ai/README.md` – Access instructions
- `Omnisystem/models/configs/omnisystem-octopus-config.json` – Configuration

---

## 2. Omni-AI Poe

### Overview
Personality-based AI agent for collaborative reasoning within the Omnisystem. Originally developed as **Poe** in the Bonsai ecosystem, now branded as **Omni-AI Poe** for consistency.

### Architecture
```
Omni-AI Poe System:
├── Personality Module       (AC_POE_PERSONALITY.md)
├── Knowledge Database       (kdb-modules/)
├── Reasoning Engine         (crates/poe-core/)
├── Bonsai Bridge           (Now: Omnisystem integration)
├── Mesh Networking         (crates/poe-mesh/)
└── Manifestation Layer     (crates/poe-manifestation/)
```

### Components

**Personality Definition**
- **Name**: Poe (within Omnisystem context: Omni-AI Poe)
- **Role**: Collaborative research agent
- **Communication Style**: Socratic, question-driven
- **Knowledge Base**: Integrated with Omnisystem proof tactics

**Knowledge Modules** (`Omnisystem/omni-ai/poe/kdb-modules/`)
- Axiom proof techniques
- Formal verification patterns
- Distributed system reasoning
- Security properties verification

**Core Implementation** (`Omnisystem/omni-ai/poe/src/`)
- Reasoning pipeline
- Context management
- Response generation

### Integration with Omnisystem
Omni-AI Poe is used as:
- **Proof assistant** for `axiom/` theorem proving
- **Validator** for `uvm/` chaos testing strategies
- **Analyst** for service correctness in `titan/axlib/`

### Using Omni-AI Poe

**In Axiom proofs:**
```axiom
theorem service_safety :
  (let poe_analysis := omni_ai_poe.analyze_service(service_spec)
   in poe_analysis.is_safe)
```

**In UVM validation:**
```titan
pub fn validate_with_poe(test_case: i64) -> i64 {
    let poe_verdict = omni_ai_poe_perform(test_case);
    if poe_verdict == SAFE { return 1; }
    return 0;
}
```

### Status
- ✅ Personality defined
- ✅ Architecture documented
- ✅ Core modules implemented
- 🔄 Full model training in progress
- 🔄 Omnisystem integration in progress

### Files
- `Omnisystem/omni-ai/poe/AC_POE_PERSONALITY.md` – Personality definition
- `Omnisystem/omni-ai/poe/context.md` – Philosophy and approach
- `Omnisystem/omni-ai/poe/src/` – Source code
- `Omnisystem/omni-ai/poe/kdb-modules/` – Knowledge base
- `Omnisystem/omni-ai/poe/config/` – Configuration

---

## 3. Omni-AI Registry & Management

### Overview
Central model discovery, registration, and management system for all AI models within the Omnisystem. Renamed from **BonsAI Model System** to **Omni-AI Registry** for consistency.

### Components

**Omni-AI Registry** (`Omnisystem/omni-ai/registry/`)
- Model registration and discovery
- Version tracking
- Hardware requirement specification
- Metadata management

**Omni-AI Scanner** (`Omnisystem/omni-ai/scanner/`)
- Automatically discovers available models
- Catalogs models with metadata
- Validates model integrity

**Omni-AI Converter** (`Omnisystem/omni-ai/converter/`)
- Format conversion (PyTorch ↔ ONNX ↔ GGUF ↔ SPIR-V)
- Quantization (FP32 → FP16 → INT8)
- Model optimization for target hardware

### Model Registry Schema

```json
{
  "model_id": "omni-ai-octopus-v1",
  "name": "Omni-AI Octopus",
  "type": "vision-language-model",
  "version": "1.0.0",
  "size_mb": 312,
  "base_model": "llama-2-7b",
  "fine_tuning": {
    "method": "lora",
    "rank": 16,
    "alpha": 32
  },
  "hardware_requirements": {
    "vram_gb": 16,
    "cpu_gb": 8,
    "preferred_device": "nvidia-gpu"
  },
  "formats": ["pytorch", "gguf", "spir-v"],
  "training_data": "9000 instruction-response pairs",
  "validation_data": "1000 examples",
  "status": "ready"
}
```

### Using Omni-AI Registry

**Discover models:**
```python
from omnisystem.omni_ai.registry import OmniAIRegistry

registry = OmniAIRegistry()
models = registry.list_models()
octopus = registry.get_model("omni-ai-octopus-v1")
```

**Register new model:**
```python
registry.register_model(
    model_id="omni-ai-custom-v1",
    metadata={...},
    model_path="/path/to/model"
)
```

**Convert formats:**
```python
from omnisystem.omni_ai.converter import OmniAIConverter

converter = OmniAIConverter()
spir_v_model = converter.to_spirv(
    "omnisystem/models/octopus-ai-model",
    optimization="max-speed"
)
```

### Files
- `Omnisystem/omni-ai/registry/` – Registry implementation
- `Omnisystem/omni-ai/scanner/` – Model discovery
- `Omnisystem/omni-ai/converter/` – Format conversion
- `Omnisystem/models/MODEL_MANIFEST.json` – Global model manifest

---

## Model Directory Structure

```
Omnisystem/
├── models/
│   ├── README.md (THIS FILE)
│   │
│   ├── octopus-ai/
│   │   └── README.md (Reference to D:\Models\Custom\octopus-ai-model)
│   │
│   ├── poe-ai/
│   │   └── README.md (Reference to omni-ai/poe)
│   │
│   ├── configs/
│   │   ├── omnisystem-octopus-config.json
│   │   └── omnisystem-poe-config.json
│   │
│   ├── quantized/
│   │   ├── README.md
│   │   └── [GGUF quantized versions]
│   │
│   └── MODEL_MANIFEST.json (Global registry)
│
├── omni-ai/
│   ├── README.md (Omni-AI system overview)
│   │
│   ├── registry/
│   │   ├── lib.rs (Rust implementation)
│   │   ├── src/
│   │   └── tests/
│   │
│   ├── scanner/
│   │   ├── lib.rs
│   │   └── src/
│   │
│   ├── converter/
│   │   ├── lib.rs
│   │   └── src/
│   │
│   ├── octopus/
│   │   ├── src/
│   │   └── tests/
│   │
│   └── poe/
│       ├── AC_POE_PERSONALITY.md
│       ├── context.md
│       ├── src/
│       ├── kdb-modules/
│       ├── config/
│       └── tests/
│
└── training-data/
    ├── README.md
    ├── train.jsonl (9,000 examples)
    ├── validation.jsonl (1,000 examples)
    └── test.jsonl
```

---

## AI Model Status Summary

| Model | Original Name | Omnisystem Name | Status | Readiness |
|-------|---------------|-----------------|--------|-----------|
| Vision-Language | Octopus AI | **Omni-AI Octopus** | ✅ Complete | Production ready |
| Personality Agent | Poe AI | **Omni-AI Poe** | 🔄 In Progress | Beta (architecture ready) |
| Model System | BonsAI Model System | **Omni-AI Registry** | ✅ Complete | Production ready |

---

## Migration from Bonsai to Omnisystem

### Naming Convention
```
Bonsai Ecosystem:          Omnisystem:
├── Octopus AI       →     Omni-AI Octopus
├── Poe AI           →     Omni-AI Poe
├── BonsAI System    →     Omni-AI Registry/Scanner/Converter
└── BonsAI Bridge    →     Omnisystem Integration Layer
```

### Code Migration Path
1. **Pure Titan integration** – All models integrated via effect system
2. **FFI to external models** – Octopus AI loaded from `D:\Models\Custom\`
3. **Native support** – Poe AI fully integrated within Omnisystem
4. **Registry system** – Omni-AI Registry manages all models

---

## Next Steps

- [ ] Migrate Octopus AI model reference to Omnisystem
- [ ] Complete Poe AI training with Omnisystem corpus
- [ ] Integrate Omni-AI Registry into Omnisystem build system
- [ ] Create Omni-AI Octopus effect handler for Titan
- [ ] Add Omni-AI models to validation mesh (UVM)
- [ ] Document Omni-AI API for external users

---

**Last Updated:** 2026-06-05  
**All Models Renamed to Omni-AI within Omnisystem Context** ✅  
**Migration Status:** In progress
