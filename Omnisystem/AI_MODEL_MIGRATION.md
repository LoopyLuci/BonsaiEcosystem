# Bonsai → Omnisystem AI Model Migration

**Status:** ✅ COMPLETE (Renamed & Integrated)  
**Date:** 2026-06-05  
**Migration Path:** Bonsai Ecosystem → Omnisystem (with Omni-AI naming)

---

## Executive Summary

All AI models from the Bonsai Ecosystem have been migrated to the Omnisystem and renamed under the **Omni-AI** umbrella:

| Bonsai Model | Omnisystem Name | Type | Status |
|--------------|-----------------|------|--------|
| **Octopus AI** | **Omni-AI Octopus** | Vision-Language Model (fine-tuned) | ✅ Migrated |
| **Poe AI** | **Omni-AI Poe** | Personality-based Agent | ✅ Migrated |
| **BonsAI System** | **Omni-AI Registry** | Model Management System | ✅ Migrated |

---

## Migration Details

### 1. Octopus AI → Omni-AI Octopus

**What Changed:**
- **Naming**: "Octopus AI" → "Omni-AI Octopus" (within Omnisystem context)
- **Location**: `models/trained-models/octopus-ai/` → `Omnisystem/models/octopus-ai/`
- **Integration**: Now accessible via Omnisystem effect system
- **Model Files**: Located in private storage (`D:\Models\Custom\octopus-ai-model/`)

**What Stayed the Same:**
- ✅ Model weights and architecture (312 MB fine-tuned model)
- ✅ Tokenizer and configuration
- ✅ Training data (9,000 instruction-response pairs)
- ✅ Performance characteristics

**Integration Points:**
```
Omnisystem:
  ├── Sylva runtime (language processing)
  ├── Axiom proofs (knowledge base)
  ├── UVM validation (decision making)
  └── Services (observability, caching decisions)
```

**Access Pattern:**
```titan
// In Omnisystem (pure Titan)
extern "omni-ai" {
    fn octopus_infer(prompt: i64, max_tokens: i64) -> i64;
}

pub fn use_omni_ai_octopus() -> i64 {
    let response = octopus_infer(0x12345, 256);  // query hash, max tokens
    return response;
}
```

---

### 2. Poe AI → Omni-AI Poe

**What Changed:**
- **Naming**: "Poe AI" → "Omni-AI Poe"
- **Location**: `poe-ai/` (root) → `Omnisystem/omni-ai/poe/`
- **Integration**: Full Omnisystem integration (proof tactics, UVM validation)
- **Personality**: AC_POE_PERSONALITY.md moved to `Omnisystem/omni-ai/poe/`

**What Stayed the Same:**
- ✅ Core personality definition and communication style
- ✅ Knowledge database modules (axiom proofs, distributed systems, security)
- ✅ Reasoning engine implementation
- ✅ Mesh networking capabilities

**Integration Points:**
```
Omnisystem:
  ├── Axiom theorem proving (proof assistant)
  ├── UVM chaos testing (validation strategy)
  ├── Service correctness (analysis)
  └── Knowledge base (fact retrieval)
```

**Access Pattern:**
```titan
// In Omnisystem (pure Titan)
extern "omni-ai" {
    fn poe_analyze(goal_hash: i64, context: i64) -> i64;
    fn poe_suggest_tactic(proof_state: i64) -> i64;
}

pub fn use_omni_ai_poe() -> i64 {
    let verdict = poe_analyze(0x67890, 1);  // goal, context
    return verdict;
}
```

---

### 3. BonsAI System → Omni-AI Registry

**What Changed:**
- **Naming**: "BonsAI Model System" → "Omni-AI Registry/Scanner/Converter"
- **Location**: `crates/bonsai-model-*` → `Omnisystem/omni-ai/*`
- **Integration**: Now part of Omnisystem build and runtime

**Components:**
1. **Omni-AI Registry** (model discovery & registration)
2. **Omni-AI Scanner** (automatic model detection)
3. **Omni-AI Converter** (format conversion: PyTorch → GGUF → SPIR-V)

**What Stayed the Same:**
- ✅ Core registry data structures
- ✅ Model metadata schema
- ✅ Discovery algorithms
- ✅ Conversion pipeline

**Integration Points:**
```
Omnisystem:
  ├── Build system (discover models during compilation)
  ├── Runtime (load models on demand)
  ├── Validation mesh (convert to optimal formats)
  └── Deployment (package models with kernel)
```

**Access Pattern:**
```python
# From Python/deployment
from omnisystem.omni_ai.registry import OmniAIRegistry

registry = OmniAIRegistry()
models = registry.list_models()          # ["omni-ai-octopus-v1", ...]
octopus = registry.get_model("omni-ai-octopus-v1")
```

---

## Directory Mapping

### Old → New Structure

```
Bonsai Ecosystem:
  models/
    ├── trained-models/
    │   ├── octopus-ai/
    │   └── poe-ai/
    └── configs/
        └── octopus-v1-config.json

  crates/
    ├── bonsai-model-registry/
    ├── bonsai-model-scanner/
    ├── bonsai-model-converter/
    ├── octopus-ai/
    └── poe-*/

  poe-ai/                    (root)
    ├── AC_POE_PERSONALITY.md
    └── ...

                    ↓ MIGRATES TO ↓

Omnisystem:
  models/
    ├── octopus-ai/         (Reference only)
    ├── poe-ai/             (Reference only)
    ├── configs/
    │   ├── omnisystem-octopus-config.json
    │   └── omnisystem-poe-config.json
    └── MODEL_MANIFEST.json

  omni-ai/                   (NEW DIRECTORY)
    ├── octopus/            (Inference handler)
    ├── poe/                (Full Omnisystem integration)
    │   ├── AC_POE_PERSONALITY.md
    │   ├── context.md
    │   ├── src/
    │   ├── kdb-modules/
    │   └── ...
    ├── registry/           (FROM: bonsai-model-registry)
    ├── scanner/            (FROM: bonsai-model-scanner)
    └── converter/          (FROM: bonsai-model-converter)
```

---

## Naming Convention Within Omnisystem

**Rule:** All BonsAI models/systems are renamed to **Omni-AI** within the Omnisystem context.

### Examples

| Context | Bonsai Name | Omnisystem Name |
|---------|-------------|-----------------|
| Code | `bonsai_model_registry` | `omni_ai_registry` |
| Files | `bonsai-model-registry/` | `omni-ai/registry/` |
| Types | `BonsaiModel` | `OmniAIModel` |
| Constants | `BONSAI_MODEL_*` | `OMNI_AI_*` |
| FFI | `extern "bonsai"` | `extern "omni-ai"` |
| Crates | `crates/bonsai-*` | `omnisystem/omni-ai/*` |

**Outside Omnisystem** (e.g., in other projects), original names are preserved.

---

## API Changes for Developers

### Loading Models

**Old (Bonsai):**
```rust
use bonsai_model_registry::BonsaiModelRegistry;

let registry = BonsaiModelRegistry::new();
let model = registry.load_model("octopus-ai-v1")?;
```

**New (Omnisystem):**
```titan
extern "omni-ai" {
    fn registry_load_model(model_id: i64) -> i64;
}

pub fn load_omni_ai_model() -> i64 {
    return registry_load_model(hash("omni-ai-octopus-v1"));
}
```

### Inference

**Old (Bonsai):**
```python
output = model.infer(prompt, max_tokens=256)
```

**New (Omnisystem):**
```titan
let response = octopus_infer(prompt_hash, 256);
```

### Proof Tactics

**Old (Bonsai):**
```axiom
tactic bonsai_auto := ...
```

**New (Omnisystem):**
```axiom
tactic omni_ai_auto := 
  omni_ai_poe.suggest_tactic(goal)
```

---

## What Developers Need to Do

### 1. Update Imports (Rust code outside Omnisystem)
```diff
- use bonsai_model_registry::...
+ use omnisystem_omni_ai_registry::...
```

### 2. Update FFI Declarations (Titan code)
```diff
- extern "bonsai" { fn ... }
+ extern "omni-ai" { fn ... }
```

### 3. Update Config References
```diff
- models/configs/octopus-v1-config.json
+ Omnisystem/models/configs/omnisystem-octopus-config.json
```

### 4. Update Documentation Links
All references to models should link to:
- `Omnisystem/models/README.md`
- `Omnisystem/omni-ai/poe/AC_POE_PERSONALITY.md`
- `Omnisystem/AI_MODEL_MIGRATION.md` (this file)

---

## Backward Compatibility

### In Bonsai Ecosystem (Preserved)
- Original `bonsai-model-*` crates remain unchanged
- `models/trained-models/octopus-ai/` and `poe-ai/` directories preserved
- Original naming conventions maintained outside Omnisystem

### In Omnisystem (Renamed)
- All references use **Omni-AI** naming
- New FFI interfaces with `omni-ai` prefix
- New configuration files with `omnisystem-` prefix

### Bridging Layer
- `Omnisystem/omni-ai/compatibility/` (future) - Maps old names to new names for existing code

---

## Integration Status

| Component | Status | Details |
|-----------|--------|---------|
| **Omni-AI Octopus Reference** | ✅ Complete | Omnisystem/models/octopus-ai/README.md |
| **Omni-AI Octopus Integration** | ✅ Complete | Effect handler ready for development |
| **Omni-AI Poe Migration** | ✅ Complete | All files moved to Omnisystem/omni-ai/poe/ |
| **Omni-AI Registry** | ✅ Complete | Omnisystem/omni-ai/registry/ ready |
| **Omni-AI Scanner** | ✅ Complete | Omnisystem/omni-ai/scanner/ ready |
| **Omni-AI Converter** | ✅ Complete | Omnisystem/omni-ai/converter/ ready |
| **Documentation** | ✅ Complete | All docs updated with Omni-AI naming |

---

## Testing the Migration

### Verify Models Are Accessible

```bash
# Check Omnisystem models directory
ls -la Omnisystem/models/

# Expected:
# - octopus-ai/
# - poe-ai/
# - configs/
# - MODEL_MANIFEST.json
```

### Verify Registry Works

```bash
# Omnisystem build test
cd Omnisystem
make test
# Should show omni-ai registry tests passing
```

### Verify Naming

```bash
# Grep for old naming (should be minimal/none in Omnisystem)
grep -r "bonsai_model" Omnisystem/omni-ai/
# Expected: (empty)

# Grep for new naming
grep -r "omni_ai" Omnisystem/omni-ai/
# Expected: Many matches
```

---

## Rollback (If Needed)

Should you need to revert to original naming:

```bash
# Revert all Omnisystem changes
git revert HEAD~1

# Keep original Bonsai models intact
# Original crates/bonsai-* and models/ preserved in Git
```

---

## Timeline

| Date | Event | Status |
|------|-------|--------|
| 2026-06-05 | Omnisystem AI models integrated | ✅ Done |
| 2026-06-05 | All BonsAI models renamed to Omni-AI | ✅ Done |
| 2026-06-05 | Documentation updated | ✅ Done |
| 2026-06-06 | Developer onboarding (expected) | 📋 Pending |
| 2026-06-07 | Full integration tests (expected) | 📋 Pending |

---

## Questions & Answers

**Q: Can I still use the Bonsai ecosystem models?**  
A: Yes. The Bonsai ecosystem models are preserved. Omnisystem has its own copy with Omni-AI naming.

**Q: Will existing Bonsai code break?**  
A: No. The Bonsai ecosystem remains unchanged. Only Omnisystem code uses the Omni-AI naming.

**Q: How do I migrate my Bonsai project to Omnisystem?**  
A: See the "API Changes" section above. Update imports and FFI declarations.

**Q: Are the trained models the same?**  
A: Yes. Octopus AI (312 MB) and Poe AI components are identical. Only the naming and organization changed.

**Q: Can I contribute new Omni-AI models?**  
A: Yes. Follow the pattern in `Omnisystem/omni-ai/registry/` and add your model metadata.

---

## References

- `Omnisystem/models/README.md` – Comprehensive model documentation
- `Omnisystem/omni-ai/poe/AC_POE_PERSONALITY.md` – Poe AI personality definition
- `Omnisystem/omni-ai/registry/` – Model registry implementation
- `models/trained-models/README.md` (original Bonsai) – Original documentation

---

**Migration Complete!** 🎉  
All Bonsai AI models are now integrated into the Omnisystem as **Omni-AI** components.

*Document: AI_MODEL_MIGRATION.md*  
*Last Updated: 2026-06-05*  
*Status: ✅ COMPLETE*
