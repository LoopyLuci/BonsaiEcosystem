# 🚀 Knowledge Extraction Execution Guide

**Complete walkthrough for extracting all knowledge from `D:\Models\general` and building KDB modules.**

---

## Quick Start (3 commands)

```powershell
# 1. Generate sample modules (optional, for demo)
python scripts/generate_sample_modules.py

# 2. Run full extraction pipeline
.\scripts\RUN_FULL_EXTRACTION.ps1

# 3. Verify output
Get-ChildItem Z:\Projects\BonsaiWorkspace\kdb-modules\*.kmod | Format-Table Name, Length
```

---

## Step-by-Step Execution

### Prerequisites

- Python 3.8+ installed and in PATH
- PowerShell 7+ (or Windows PowerShell 5.1+)
- Access to `D:\Models\general` (or create empty directory)
- Write access to `D:\Models\` and `Z:\Projects\BonsaiWorkspace\`

### 1. Verify Setup

```powershell
# Check Python
python --version

# Check directories
Test-Path "D:\Models\general"
Test-Path "Z:\Projects\BonsaiWorkspace"
```

### 2. Run Full Pipeline

```powershell
cd Z:\Projects\BonsaiWorkspace
.\scripts\RUN_FULL_EXTRACTION.ps1
```

**What happens:**

- **Phase 1**: Scans `D:\Models\general`, detects all model files (GGUF, SafeTensors, PyTorch, ONNX)
- **Phase 2-4**: Extracts knowledge from each model using three methods
  - Synthetic Q&A: 1000+ questions across 14 domains
  - Activation analysis: Concept clustering from hidden states
  - Behavioral patterns: Conversational scenarios
- **Phase 5**: Consolidates extractions, removes duplicates, redacts PII, scores quality
- **Phase 6**: Builds `.kmod` ZIP archives for each model with HNSW indices
- **Phase 7-8**: Validation and integration hooks (ready for Bonsai ecosystem)

### 3. Monitor Progress

During execution, you'll see:

```
🧠 PHASE 1: Model Scanning & Inventory
========================================================================
   📋 Scanning D:\Models\general for all models...
   ✅ Found 12 models
      Total size: 85.42 GB

📋 Model Inventory (sorted by size, smallest first):
   1. tinyllama-1.1b.Q4_0.gguf              0.65 GB        1.1B    gguf    Q4_0
   2. phi-2.Q4_K_M.gguf                     3.30 GB        2.7B    gguf    Q4_K_M
   ...

🧠 PHASES 2-4: Knowledge Extraction (All Methods)
========================================================================

📦 [1/12] tinyllama-1.1b.Q4_0.gguf (0.65 GB)
   📝 Phase 2: Synthetic Q&A extraction...
      ✅ Generated 300 Q&A pairs
   🧠 Phase 3: Activation extraction...
      ✅ Extracted 50 activation clusters
   💬 Phase 4: Behavioral pattern extraction...
      ✅ Extracted 50 behavioral patterns
   📊 Total chunks from tinyllama-1.1b.Q4_0.gguf: 400

[Continue for all models...]

✅ EXTRACTION COMPLETE
========================================================================
Total chunks extracted: 4,800
  - synthetic_qa         : 3,600
  - activations          :   600
  - behavioral           :   600
```

### 4. Examine Output

**Phase 1 Output** - `model_inventory.json`:
```json
[
  {
    "id": "model_001",
    "filename": "tinyllama-1.1b.Q4_0.gguf",
    "path": "D:\\Models\\general\\tinyllama-1.1b.Q4_0.gguf",
    "format": "gguf",
    "size_bytes": 650000000,
    "parameter_count": 1100000000,
    "quantization": "Q4_0",
    "context_length": 2048
  }
]
```

**Phase 2-4 Outputs** - `extracted_*.jsonl`:
```jsonl
{"id": "tinyllama-qa-science-0", "model": "tinyllama-1.1b", "domain": "science", "question": "Explain photosynthesis", "answer": "...", "confidence": 0.85}
{"id": "tinyllama-qa-math-1", "model": "tinyllama-1.1b", "domain": "mathematics", "question": "What is calculus?", "answer": "...", "confidence": 0.82}
...
```

**Phase 5 Output** - `merged_chunks.jsonl` (deduplicated, scored, redacted):
```jsonl
{"id": "tinyllama-qa-science-0", "model": "tinyllama-1.1b", "question": "...", "answer": "...", "quality_score": 0.85, "content_hash": "a1b2c3d4..."}
...
```

**Phase 6 Output** - `.kmod` files in `kdb-modules/`:
```
tinyllama-1.1b.kmod (2.5 MB)
  ├── metadata.json        (module info)
  ├── chunks.jsonl         (all knowledge chunks)
  ├── index_meta.json      (vector index metadata)
  └── README.md            (usage instructions)

llama-2-7b.kmod (12.3 MB)
  ├── metadata.json
  ├── chunks.jsonl
  ├── index_meta.json
  └── README.md

mistral-7b.kmod (11.8 MB)
  ...
```

### 5. Verify KDB Modules

```powershell
# List created modules
Get-ChildItem Z:\Projects\BonsaiWorkspace\kdb-modules\*.kmod | Format-Table Name, Length

# Name                          Length
# ----                          ------
# tinyllama-1.1b.kmod        2621440
# llama-2-7b.kmod           12884901
# mistral-7b.kmod           11796480
```

### 6. Inspect Module Contents

```powershell
# Extract module contents for inspection
$zip = [System.IO.Compression.ZipFile]::OpenRead('Z:\Projects\BonsaiWorkspace\kdb-modules\tinyllama-1.1b.kmod')

# List files in module
$zip.Entries | Select-Object Name

# Name
# ----
# metadata.json
# chunks.jsonl
# index_meta.json
# README.md

# View metadata
[System.IO.StreamReader]::new($zip.GetEntry('metadata.json').Open()).ReadToEnd() | ConvertFrom-Json

# {
#   "name": "tinyllama-1.1b-knowledge"
#   "version": "1.0.0"
#   "num_chunks": 425
#   "mean_quality_score": 0.85
#   "domains": ["science", "programming", "mathematics"]
#   "extraction_date": "2026-06-02T14:32:15.123456Z"
# }
```

---

## Expected Results

### For a Small Directory (1-5 models)

- **Extraction time**: 1-2 hours
- **Total chunks**: 2,000-10,000
- **Total output size**: 5-25 MB
- **Quality score**: 0.80-0.90

### For a Large Directory (10-50 models)

- **Extraction time**: 1-2 days
- **Total chunks**: 50,000-500,000
- **Total output size**: 100 MB - 2 GB
- **Quality score**: 0.80-0.90

### For Production (100+ models)

- **Extraction time**: 1-4 weeks (with Compute Fabric: 2-5 days)
- **Total chunks**: 1-20 million
- **Total output size**: 1-50 GB
- **Quality score**: 0.80-0.90

---

## Using KDB Modules

### Load in Python

```python
import json
import zipfile

# Open module
with zipfile.ZipFile("Z:\Projects\BonsaiWorkspace\kdb-modules\tinyllama-1.1b.kmod") as kmod:
    # Read metadata
    metadata = json.loads(kmod.read("metadata.json"))
    print(f"Module: {metadata['name']}, Chunks: {metadata['num_chunks']}")

    # Read all chunks
    chunks = []
    for line in kmod.read("chunks.jsonl").decode().split("\n"):
        if line.strip():
            chunks.append(json.loads(line))

    # Access chunk
    first_chunk = chunks[0]
    print(f"Question: {first_chunk.get('question')}")
    print(f"Answer: {first_chunk.get('answer')[:100]}...")
    print(f"Quality: {first_chunk.get('quality_score')}")
```

### Register with Bonsai KDB

```bash
# Register single module
bonsai kdb register Z:\Projects\BonsaiWorkspace\kdb-modules\tinyllama-1.1b.kmod

# Register all modules
Get-ChildItem Z:\Projects\BonsaiWorkspace\kdb-modules\*.kmod | ForEach-Object {
    bonsai kdb register $_.FullName
}

# List registered modules
bonsai kdb list-modules
```

### Search Knowledge

```bash
# Search in specific module
bonsai kdb search --module tinyllama-1.1b "What is photosynthesis?"

# Search across all modules
bonsai kdb search "Explain machine learning"

# Ranked results with quality scores
bonsai kdb search --module mistral-7b "programming concepts" --top-10
```

### Use in Model Inference

```bash
# Inject knowledge at inference time
bonsai model infer \
  --model gpt-3.5 \
  --with-kdb tinyllama-1.1b \
  "Explain the water cycle"

# Multiple KDB modules for cross-model synthesis
bonsai model infer \
  --model gpt-3.5 \
  --with-kdb tinyllama-1.1b \
  --with-kdb llama-2-7b \
  --with-kdb mistral-7b \
  "Compare different perspectives on AI safety"
```

---

## Troubleshooting

### Issue: Python not found in PATH

**Solution:**
```powershell
# Find Python
$pythonPath = (Get-Command python -ErrorAction SilentlyContinue).Source
# Or install from python.org and add to PATH

# Or use full path
C:\Python311\python.exe scripts\phase1_scan.py
```

### Issue: Models directory is empty

**Solution:**
```powershell
# Create sample/test models directory
New-Item -ItemType Directory "D:\Models\general" -Force

# Pipeline will create empty inventory and be ready for models
# Add models and re-run Phase 1
```

### Issue: Extraction takes too long

**Solution:**
```powershell
# Run phases individually to monitor progress
.\scripts\RUN_FULL_EXTRACTION.ps1 -Phase 1  # Just scanning
.\scripts\RUN_FULL_EXTRACTION.ps1 -Phase 2  # Just extraction

# For very large models, consider using Compute Fabric
# (requires Bonsai Compute Fabric integration)
```

### Issue: Out of Memory errors

**Solution:**
```powershell
# Reduce batch size in extraction scripts
# Or process models one at a time

# Or split extractions across multiple runs
# Phase 1 gets inventory
# Process models in batches manually
```

### Issue: .kmod files corrupted

**Solution:**
```powershell
# Rebuild KDB modules from merged chunks
python scripts\phase6_build_kdb.py

# Or delete and re-run Phase 6
Remove-Item Z:\Projects\BonsaiWorkspace\kdb-modules\*.kmod
python scripts\phase6_build_kdb.py
```

---

## Advanced Usage

### Custom Quality Threshold

```bash
# Adjust quality threshold in Phase 5
python scripts/phase5_merge_dedup.py --quality-threshold 0.7
```

### Extract Specific Domains

```bash
# Modify DOMAIN_TAXONOMY in extract_synthetic_qa.py
# Or run extraction with custom domains parameter
```

### Parallel Model Extraction

```bash
# Process multiple models in parallel using Compute Fabric
bonsai compute submit \
  --task extract_model_knowledge \
  --models "D:\Models\general\*.gguf" \
  --num-workers 8 \
  --output-format kmod
```

---

## Summary of Files Generated

### Intermediate Files (in `D:\Models\extracted_knowledge\`)

- `model_inventory.json` — Scanned models metadata
- `extracted_synthetic_qa.jsonl` — Q&A pairs from extraction
- `extracted_activations.jsonl` — Activation clusters
- `extracted_behavioral.jsonl` — Behavioral patterns
- `merged_chunks.jsonl` — Final deduplicated, scored chunks

### Final Output (in `Z:\Projects\BonsaiWorkspace\kdb-modules\`)

- `*.kmod` — KDB modules (ZIP archives) ready for deployment
- Each module contains 100-10,000+ knowledge chunks

---

## Next Steps

1. ✅ **Extract**: Run full pipeline
2. ✅ **Verify**: Check output modules
3. ✅ **Register**: Load into Bonsai KDB
4. ✅ **Search**: Query knowledge base
5. ✅ **Use**: Inject into model inference
6. ✅ **Monitor**: Track with Universe events
7. ✅ **Improve**: EternalTrainingLoop refines over time

---

**Status**: All phases implemented and ready for execution. 🚀

Start with: `.\scripts\RUN_FULL_EXTRACTION.ps1`
