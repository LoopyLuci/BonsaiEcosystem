# BonsAI Training Data Library & Knowledge Database
## Next-Generation Modular AI Architecture

**Version:** 1.0  
**Status:** Active Design + Implementation  
**Scope:** TDL (Training Data Library), KDB (Knowledge Database), .bkp package format  
**Philosophy:** Every piece of knowledge is addressable, auditable, replaceable, and composable. Models are assembled, not monolithically trained.

---

## Executive Summary

Three interlocking systems transform the BonsAI Ecosystem from a training pipeline into a **model factory**:

1. **Training Data Library (TDL)** — every training example ever created, versioned, hashed, searchable, with per-item quality scores and provenance. The single source of truth for all model building.

2. **Knowledge Database (KDB)** — a modular inference architecture where a model's knowledge is stored as discrete, swappable `.kmod` units instead of opaque monolithic weights. Models become manifests that select from the library.

3. **Bonsai Knowledge Package (.bkp)** — a portable container format (ZIP/zstd) that bundles base model + KDB modules + manifest into one transportable file, loadable both packaged and unpackaged.

Together: **drag-and-drop model building**. Select a base brain, add knowledge packs, deploy in seconds. Remove bad data without retraining. Ship a working model as a single file.

---

## Part 1: Training Data Library (TDL)

### 1.1 Core Design Principles

| Principle | How Enforced |
|-----------|-------------|
| **Content-addressed** | Every example keyed by its BLAKE3 hash via the existing `bonsai-cas` CAS store |
| **Immutable** | Examples never modified in place; edits create new versions with `parent_hash` chain |
| **Provenance-complete** | Every example stores source, creator, session_id, generator script, and timestamp |
| **Per-item auditable** | Individual examples can be flagged, removed, or superseded without touching others |
| **Auto-ingested** | The `UnifiedTrainingCollector` already collects data; TDL wraps it with versioning and search |
| **Offline-first** | All storage is local SQLite + flat files; no external calls |

### 1.2 Database Schema

New database: `~/.bonsai/tdl.db` (separate from existing DBs to avoid contention).

```sql
-- Datasets: logical collections of related examples
CREATE TABLE datasets (
    id           TEXT PRIMARY KEY,          -- UUIDv4
    name         TEXT NOT NULL,
    description  TEXT,
    domain       TEXT NOT NULL,             -- code|safety|chat|tool_use|survival|distill|custom
    format       TEXT NOT NULL DEFAULT 'dpo', -- dpo|sft|distill|instruction|raw
    item_count   INTEGER NOT NULL DEFAULT 0,
    quality_score REAL NOT NULL DEFAULT 0.5, -- 0..1, aggregate
    size_bytes   INTEGER NOT NULL DEFAULT 0,
    provenance   TEXT NOT NULL,             -- JSON: {source, creator, generator, session_ids}
    tags         TEXT NOT NULL DEFAULT '[]', -- JSON array
    version      INTEGER NOT NULL DEFAULT 1,
    parent_id    TEXT REFERENCES datasets(id),
    cas_key      TEXT,                      -- BLAKE3 hex of the full JSONL file in CAS
    file_path    TEXT NOT NULL,             -- ~/.bonsai/tdl/datasets/<id>/data.jsonl
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL,
    archived     INTEGER NOT NULL DEFAULT 0  -- soft delete
);

CREATE INDEX idx_datasets_domain   ON datasets(domain);
CREATE INDEX idx_datasets_quality  ON datasets(quality_score);
CREATE INDEX idx_datasets_created  ON datasets(created_at);

-- Individual training examples within datasets
CREATE TABLE examples (
    item_id      TEXT PRIMARY KEY,          -- UUIDv4
    dataset_id   TEXT NOT NULL REFERENCES datasets(id),
    position     INTEGER NOT NULL,          -- index in the JSONL file
    cas_key      TEXT NOT NULL,             -- BLAKE3 of the serialized item JSON
    example_type TEXT NOT NULL,             -- prompt_response|preference_pair|instruction|tool_call|raw
    domain       TEXT,
    language     TEXT,
    quality      REAL NOT NULL DEFAULT 0.5,
    difficulty   REAL NOT NULL DEFAULT 0.5, -- 0=easy, 1=hard
    flagged      INTEGER NOT NULL DEFAULT 0,
    flag_reason  TEXT,
    parent_item  TEXT,                      -- if this item supersedes another
    metadata     TEXT NOT NULL DEFAULT '{}', -- JSON
    created_at   TEXT NOT NULL
);

CREATE INDEX idx_examples_dataset  ON examples(dataset_id);
CREATE INDEX idx_examples_quality  ON examples(quality);
CREATE INDEX idx_examples_flagged  ON examples(flagged);
CREATE INDEX idx_examples_type     ON examples(example_type);

-- Full-text search over prompt/instruction content
CREATE VIRTUAL TABLE examples_fts USING fts5(
    item_id UNINDEXED,
    dataset_id UNINDEXED,
    prompt_text,
    response_text,
    content='examples',
    tokenize='porter ascii'
);

-- Audit log: every mutation to examples or datasets
CREATE TABLE audit_log (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type TEXT NOT NULL,  -- dataset|example
    entity_id   TEXT NOT NULL,
    action      TEXT NOT NULL,  -- create|flag|unflag|archive|version|import
    actor       TEXT NOT NULL,  -- user|system|agent|eternal_loop
    reason      TEXT,
    old_value   TEXT,           -- JSON snapshot of previous state
    new_value   TEXT,
    occurred_at TEXT NOT NULL
);

-- Recipes: named collections of dataset filters used to build a training run
CREATE TABLE recipes (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT,
    components  TEXT NOT NULL,  -- JSON: [{dataset_id, weight, filters}]
    base_model  TEXT,
    training_method TEXT,       -- sft|dpo|distill|combined
    created_at  TEXT NOT NULL,
    last_used   TEXT
);
```

### 1.3 Example Wire Format

Every item stored in CAS under its BLAKE3 key:

```json
{
  "item_id": "01934f7a-...",
  "cas_key": "blake3:7f3a...",
  "type": "preference_pair",
  "data": {
    "system": "You are a BonsAI assistant.",
    "prompt": "How do I prevent SQL injection in Rust?",
    "chosen": "Use parameterized queries via sqlx: `sqlx::query(\"SELECT * FROM users WHERE id = $1\").bind(user_id)`",
    "rejected": "Escape the input manually with string replace."
  },
  "metadata": {
    "domain": "code",
    "language": "rust",
    "difficulty": 0.6,
    "quality": 0.93,
    "source": "eternal_training_loop",
    "session_id": "sess_abc123",
    "generator": "UnifiedTrainingCollector",
    "created_at": "2026-05-30T10:00:00Z",
    "tags": ["security", "sql", "backend"],
    "parent_hash": null
  },
  "signature": null
}
```

### 1.4 Tauri Commands

New module: `bonsai-workspace/src-tauri/src/training_data_library.rs`

```rust
// List datasets with optional filters
#[tauri::command]
pub async fn tdl_list_datasets(
    filter: Option<DatasetFilter>,
    state: State<'_, AppState>,
) -> Result<Vec<DatasetSummary>, String>;

// Get full dataset with first N items preview
#[tauri::command]
pub async fn tdl_get_dataset(
    id: String,
    preview_count: Option<usize>,
    state: State<'_, AppState>,
) -> Result<DatasetDetail, String>;

// Import external JSONL/CSV/Parquet file
#[tauri::command]
pub async fn tdl_import_dataset(
    file_path: String,
    name: String,
    domain: String,
    tags: Vec<String>,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns dataset_id

// Create dataset from items provided by UI
#[tauri::command]
pub async fn tdl_create_dataset(
    name: String,
    domain: String,
    format: String,
    items: Vec<serde_json::Value>,
    state: State<'_, AppState>,
) -> Result<String, String>;

// Export to JSONL for use in training
#[tauri::command]
pub async fn tdl_export_dataset(
    id: String,
    exclude_flagged: bool,
    min_quality: f32,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns temp file path

// Flag a specific example as bad data
#[tauri::command]
pub async fn tdl_flag_example(
    dataset_id: String,
    item_id: String,
    reason: String,
    state: State<'_, AppState>,
) -> Result<(), String>;

// Build a derived dataset (new version with filters applied)
#[tauri::command]
pub async fn tdl_derive_dataset(
    parent_id: String,
    name: String,
    filter: ExampleFilter,
    state: State<'_, AppState>,
) -> Result<String, String>;

// Full-text + faceted search
#[tauri::command]
pub async fn tdl_search(
    query: String,
    domain: Option<String>,
    min_quality: Option<f32>,
    max_difficulty: Option<f32>,
    tags: Vec<String>,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<Vec<ExampleSearchResult>, String>;

// Get training recipe suggestions from Training Agent
#[tauri::command]
pub async fn tdl_suggest_recipe(
    task_description: String,
    state: State<'_, AppState>,
) -> Result<RecipeSuggestion, String>;

// Build a training run from a recipe
#[tauri::command]
pub async fn tdl_launch_recipe(
    recipe_id: String,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns job_id
```

### 1.5 Auto-Ingestion Pipeline

Extend the existing `UnifiedTrainingCollector` and `EternalTrainingLoop` to register all output to the TDL automatically:

```rust
// In eternal_training_loop.rs, after each export:
pub async fn register_export_to_tdl(
    export_path: &Path,
    domain: &str,
    source: &str,
    tdl: &TrainingDataLibrary,
) -> Result<String> {
    let items = parse_jsonl(export_path)?;
    let dataset_id = tdl.create_dataset(CreateDatasetRequest {
        name: format!("{domain} export {}", chrono::Utc::now().format("%Y-%m-%d")),
        domain: domain.to_string(),
        format: infer_format(&items),
        provenance: json!({ "source": source, "auto_imported": true }),
        items,
    }).await?;
    Ok(dataset_id)
}
```

A filesystem watcher on `~/.bonsai/training_export/` triggers this whenever a new `.jsonl` file appears.

### 1.6 TDL Svelte UI

New panel: `src/lib/components/TrainingDataLibrary.svelte`

**Layout:**
```
┌─────────────────────────────────────────────────────────────────┐
│  Training Data Library                        [Import] [+ New]  │
├──────────────┬──────────────────────────────────────────────────┤
│ FILTERS      │  DATASETS                           SORT: Quality│
│              │  ┌────────────┐ ┌────────────┐ ┌────────────┐  │
│ Domain       │  │ Safety DPO │ │ Rust Code  │ │ Tool Use   │  │
│ ○ All        │  │ 50 pairs   │ │ 12,400 ex  │ │ 800 pairs  │  │
│ ● Code       │  │ ★★★★★ 0.97│ │ ★★★★☆ 0.82│ │ ★★★★☆ 0.79│  │
│ ○ Safety     │  │ DPO  code  │ │ SFT  rust  │ │ DPO  tools │  │
│ ○ Tool Use   │  └────────────┘ └────────────┘ └────────────┘  │
│ ○ Chat       │                                                   │
│ ○ Survival   │  [Details pane when a card is clicked]            │
│              │                                                   │
│ Quality ≥    │  ┌────────────────────────────────────────────┐  │
│ [====--] 0.7 │  │ Rust Code — 12,400 examples               │  │
│              │  │ Domain: code   Lang: rust   Quality: 0.82  │  │
│ Min Examples │  │ Source: eternal_loop   Created: 2026-05-28 │  │
│ [ 100      ] │  │                                            │  │
│              │  │ Preview (first 3 items):                   │  │
│ Tags         │  │  1. "How to use Arc<Mutex<T>>?" ...       │  │
│ [rust      ] │  │  2. "Explain the borrow checker..." ...   │  │
│              │  │  3. [flagged ⚑] "Use unsafe everywhere"   │  │
│              │  │                                            │  │
│              │  │ [Add to Recipe ➕] [Export ↓] [Flag Items] │  │
│              │  └────────────────────────────────────────────┘  │
└──────────────┴──────────────────────────────────────────────────┘
```

**Drag-and-drop to Recipe Builder:**
- Each dataset card has a drag handle (⠿)
- Recipe Builder (in Model Trainer) is a drop target
- Dropping adds the dataset tile with a weight slider

---

## Part 2: Knowledge Database (KDB) — Modular Model Architecture

### 2.1 Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                    BONSAI INFERENCE ENGINE                          │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    BASE MODEL (1.5B)                         │  │
│  │  Compact reasoning core. Knows language + reasoning.        │  │
│  │  Has cross-attention slots at layers 6, 12, 18, 24.        │  │
│  └───────────────────────────┬──────────────────────────────────┘  │
│                               │ query vector at each step           │
│                               ▼                                     │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                   KDB RETRIEVER                              │  │
│  │  Searches active modules for top-K relevant knowledge.      │  │
│  │  Returns value representations injected into cross-attn.    │  │
│  └──────┬───────────────┬───────────────────────────────────────┘  │
│         │               │                                           │
│  ┌──────▼────┐   ┌──────▼────┐   ┌──────────┐   ┌──────────┐     │
│  │ .kmod     │   │ .kmod     │   │ .kmod    │   │ .kmod    │     │
│  │ Rust Code │   │ Safety    │   │ Tool Use │   │ Survival │     │
│  │ 45k items │   │ 500 items │   │ 800 items│   │ 200 items│     │
│  └───────────┘   └───────────┘   └──────────┘   └──────────┘     │
│                                                                     │
│  [Active modules controlled by user via Model Studio UI]           │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 Knowledge Module Format (.kmod)

A `.kmod` is a directory (or a single file when packaged):

```
~/.bonsai/library/modules/<module_id>/
├── manifest.json          # module metadata
├── index.hnsw             # HNSW approximate nearest neighbor index
├── keys.f16               # key vectors (float16), shape: [N, DIM_KEY]
├── values.txt.zst         # zstd-compressed text values (one per line)
├── values.dense.f16       # optional: dense value vectors for cross-attention
├── adapters/              # optional: per-cluster LoRA-like adapters
│   ├── cluster_0.bin
│   ├── cluster_1.bin
│   └── ...
└── checksum.blake3        # hash of all above files for integrity
```

**manifest.json:**
```json
{
  "module_id": "mod_a1b2c3d4",
  "name": "Rust Programming",
  "version": "1.0",
  "type": "textual_memory",
  "source_dataset_id": "ds_rust_code_v3",
  "source_dataset_version": 2,
  "base_model_family": "qwen2",
  "embedding_model": "local:bonsai-embed-256",
  "dim_key": 256,
  "num_items": 45231,
  "index_type": "hnsw",
  "compression": "f16",
  "value_type": "text",
  "has_dense_values": false,
  "has_adapters": false,
  "domains": ["code", "rust", "systems"],
  "quality_score": 0.87,
  "created_at": "2026-05-30T12:00:00Z",
  "build_params": {
    "hnsw_m": 16,
    "hnsw_ef_construction": 200,
    "hnsw_ef_search": 50,
    "quantize_keys": "f16"
  }
}
```

**Type taxonomy:**

| Type | Key | Value | Use Case |
|------|-----|-------|----------|
| `textual_memory` | Semantic embedding | Compressed text string | RAG-style fact injection |
| `dense_value` | Semantic embedding | Fixed-size hidden-state vector | Cross-attention fusion into base model layers |
| `lora_adapter` | Semantic embedding | Per-cluster LoRA delta weights | Fine-grained behavior modulation |
| `rule_set` | Keyword hash | Structured rule JSON | Safety rules, formatting constraints |
| `graph_nodes` | Entity embedding | KG triple (subject, predicate, object) | Fact graph traversal |

### 2.3 HNSW Index Format

The HNSW index uses a custom pure-Rust implementation in `crates/bonsai-hnsw/` (no external C libraries):

```rust
// crates/bonsai-hnsw/src/lib.rs
pub struct HnswIndex {
    /// Serializable/deserializable graph structure
    graph: Vec<Node>,
    /// All key vectors (float16 for memory efficiency)
    keys: Vec<[f16; MAX_DIM]>,
    dim: usize,
    /// HNSW tuning parameters
    m: usize,               // max connections per layer
    ef_construction: usize, // beam width during build
    ef_search: usize,       // beam width during query
}

impl HnswIndex {
    pub fn build(keys: &[[f32]], m: usize, ef: usize) -> Self;
    pub fn search(&self, query: &[f32], k: usize) -> Vec<(usize, f32)>;
    pub fn save(&self, path: &Path) -> Result<()>;
    pub fn load(path: &Path) -> Result<Self>;
    // Memory-map the key vectors directly from disk
    pub fn load_mmap(path: &Path) -> Result<Self>;
}
```

### 2.4 Retriever Runtime

```rust
// crates/bonsai-kdb/src/retriever.rs
pub struct KdbRetriever {
    /// Active modules, indexed for fast parallel search
    modules: Vec<LoadedModule>,
    /// LRU cache of recently retrieved items (item_id → value bytes)
    cache: LruCache<u64, Vec<u8>>,
    /// Query encoder: converts text to embedding vector
    encoder: Arc<dyn QueryEncoder>,
}

pub struct LoadedModule {
    pub manifest: ModuleManifest,
    pub index: Arc<HnswIndex>,       // memory-mapped HNSW
    pub values: Arc<ValueStore>,     // memory-mapped text / dense values
}

impl KdbRetriever {
    pub fn new(encoder: Arc<dyn QueryEncoder>) -> Self;

    /// Add/remove modules without restarting inference
    pub fn set_active_modules(&mut self, module_paths: Vec<PathBuf>) -> Result<()>;

    /// Called by the inference engine at each forward pass step
    pub fn retrieve(&self, query_vec: &[f32], top_k: usize) -> Vec<RetrievedItem>;

    /// For textual_memory modules: returns items as text strings
    pub fn retrieve_text(&self, query: &str, top_k: usize) -> Vec<String>;
}

#[derive(Debug, Clone)]
pub struct RetrievedItem {
    pub module_id: String,
    pub item_index: usize,
    pub score: f32,         // cosine similarity
    pub value: ItemValue,
}

pub enum ItemValue {
    Text(String),
    DenseVector(Vec<f16>),
    AdapterDelta { cluster_id: u32, weights: Vec<u8> },
}
```

### 2.5 Model Manifest (the model is a manifest)

```json
{
  "manifest_version": "1.0",
  "model_id": "my-rust-expert",
  "display_name": "Rust Security Expert",
  "description": "BonsAI + Rust code + safety rules + survival KB",
  "base_model": {
    "family": "qwen2",
    "size_b": 1.5,
    "gguf_path": "~/.bonsai/models/Qwen2.5-1.5B-Q4_K_M.gguf",
    "tokenizer": "~/.bonsai/models/Qwen2.5-1.5B/tokenizer.json"
  },
  "lora_adapter": {
    "path": "~/.bonsai/adapters/bonsai-safety-v1",
    "active": true
  },
  "knowledge_modules": [
    {
      "module_id": "mod_rust_code",
      "path": "~/.bonsai/library/modules/mod_rust_code",
      "weight": 1.0,
      "active": true
    },
    {
      "module_id": "mod_safety_rules",
      "path": "~/.bonsai/library/modules/mod_safety_rules",
      "weight": 0.8,
      "active": true
    },
    {
      "module_id": "mod_survival_kb",
      "path": "~/.bonsai/library/modules/mod_survival_kb",
      "weight": 0.6,
      "active": true
    }
  ],
  "retrieval_config": {
    "top_k": 5,
    "min_score": 0.65,
    "inject_mode": "system_prompt_prefix"
  },
  "created_at": "2026-05-30T12:00:00Z",
  "source_datasets": ["ds_rust_code_v3", "ds_safety_v1", "ds_survival_v7"]
}
```

### 2.6 Module Builder Pipeline

```
scripts/build_knowledge_module.py
  Input:  dataset_id (from TDL)
          base_model_path (for embedding model)
          options: {type, hnsw_m, hnsw_ef, quantize}

  Steps:
  1. Export dataset from TDL → temp JSONL
  2. For each item, compute embedding via llama-server /embeddings endpoint
     (uses the same local llama-server already running for inference)
  3. Build HNSW index from embeddings
  4. Store keys (float16) and values (text, compressed with zstd)
  5. Write manifest.json
  6. Register module in KDB: INSERT INTO knowledge_modules ...
  7. Return module_id
```

```python
# scripts/build_knowledge_module.py
def build_module(dataset_path: str, output_dir: str, embed_url: str, options: dict) -> str:
    items = load_jsonl(dataset_path)
    keys, values = [], []
    for item in tqdm(items):
        text = extract_text(item)          # get prompt + response text
        emb  = get_embedding(text, embed_url)  # call llama-server /embeddings
        keys.append(np.array(emb, dtype=np.float16))
        values.append(text)
    
    # Build HNSW index (using hnswlib or our own bonsai-hnsw)
    index = build_hnsw(keys, m=options.get("m", 16), ef=options.get("ef", 200))
    
    # Write files
    os.makedirs(output_dir, exist_ok=True)
    save_keys(keys, f"{output_dir}/keys.f16")
    save_values(values, f"{output_dir}/values.txt.zst")
    save_hnsw(index, f"{output_dir}/index.hnsw")
    write_manifest(output_dir, items, keys, options)
    compute_checksum(output_dir)
    
    return output_dir
```

### 2.7 Inference Integration

The retriever integrates with the existing llama-server at the **system prompt level** (Phase 1) and later at the **hidden state level** (Phase 2):

**Phase 1 — System prompt injection (immediate, no model changes needed):**
```python
# In the chat inference path (bonsai-workspace/src-tauri/src/chat_handler)
async fn build_system_prompt(query: &str, retriever: &KdbRetriever) -> String {
    let relevant = retriever.retrieve_text(query, top_k=5);
    if relevant.is_empty() {
        return base_system_prompt();
    }
    let knowledge_block = relevant.join("\n\n---\n\n");
    format!(
        "{base}\n\n## Relevant Knowledge\n{knowledge_block}",
        base = base_system_prompt()
    )
}
```

**Phase 2 — Cross-attention fusion (requires custom inference engine):**
Deferred to `bonsai-tensor` implementation. The base model gets cross-attention adapters at layers 6, 12, 18, 24 that consume the dense value vectors from the KDB retriever.

### 2.8 Knowledge Database Registry

```sql
-- In tdl.db, additional tables for KDB:

CREATE TABLE knowledge_modules (
    module_id    TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
    version      TEXT NOT NULL DEFAULT '1.0',
    type         TEXT NOT NULL,   -- textual_memory|dense_value|lora_adapter|rule_set
    source_dataset_id TEXT REFERENCES datasets(id),
    base_model_family TEXT,
    dim_key      INTEGER NOT NULL,
    num_items    INTEGER NOT NULL DEFAULT 0,
    quality_score REAL NOT NULL DEFAULT 0.5,
    domains      TEXT NOT NULL DEFAULT '[]',
    module_path  TEXT NOT NULL,   -- path to module directory
    size_bytes   INTEGER NOT NULL DEFAULT 0,
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL,
    archived     INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE model_manifests (
    model_id     TEXT PRIMARY KEY,
    display_name TEXT NOT NULL,
    description  TEXT,
    manifest_json TEXT NOT NULL,   -- full manifest stored as JSON
    base_model   TEXT NOT NULL,    -- family+size string
    module_ids   TEXT NOT NULL,    -- JSON array of module_ids
    lora_path    TEXT,
    quality_estimate REAL,
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL
);
```

### 2.9 KDB Tauri Commands

```rust
// In training_data_library.rs (extended for KDB)

#[tauri::command]
pub async fn kdb_build_module(
    dataset_id: String,
    name: String,
    module_type: String,
    options: BuildModuleOptions,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns module_id

#[tauri::command]
pub async fn kdb_list_modules(
    domain: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ModuleSummary>, String>;

#[tauri::command]
pub async fn kdb_create_manifest(
    name: String,
    base_model: String,
    module_ids: Vec<String>,
    lora_path: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns model_id

#[tauri::command]
pub async fn kdb_set_active_modules(
    module_paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<(), String>;

#[tauri::command]
pub async fn kdb_test_model(
    model_id: String,
    test_prompts: Vec<String>,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String>; // returns responses
```

---

## Part 3: Bonsai Knowledge Package (.bkp) Format

### 3.1 Format Specification

A `.bkp` file is a **ZIP archive (store-only, no compression)** containing:

```
<model-name>.bkp  (ZIP, store mode — no compression for random access)
├── manifest.json              ← REQUIRED: top-level package descriptor
├── base_model/
│   ├── model.gguf             ← base model weights (Q4_K_M quantized)
│   ├── tokenizer.json
│   ├── tokenizer_config.json
│   └── config.json
├── modules/
│   ├── <module_id>/
│   │   ├── manifest.json
│   │   ├── index.hnsw
│   │   ├── keys.f16
│   │   ├── values.txt.zst
│   │   └── checksum.blake3
│   └── ...
├── adapters/
│   └── <adapter_name>/
│       ├── adapter_config.json
│       └── adapter_model.safetensors  ← LoRA weights
├── training_data/             ← OPTIONAL: subset of TDL examples
│   ├── examples.jsonl
│   └── dataset_manifest.json
├── provenance.json            ← full lineage chain
└── signature.sig              ← OPTIONAL: Ed25519 signature of manifest.json
```

**Why ZIP store-mode:**
- Random access to any file without full decompression
- Memory-mappable: the OS can page entries directly into virtual memory
- The GGUF and HNSW indices are already self-compressing; ZIP compression adds overhead with no benefit
- Appendable: new modules can be added by appending to the ZIP central directory

**Why not tar.zst:**
- tar.zst is sequential; cannot random-access without full decompression
- ZIP store-mode achieves the same portability with better random access

### 3.2 Package Manifest Schema

`manifest.json`:
```json
{
  "bkp_version": "1.0",
  "package_id": "pkg_a1b2c3d4e5f6",
  "name": "BonsAI Rust Expert v1",
  "description": "Rust + security focused assistant with Survival KB",
  "created_at": "2026-05-30T12:00:00Z",
  "created_by": "bonsai-workspace/1.0",
  "base_model": {
    "family": "qwen2",
    "original_id": "Qwen/Qwen2.5-1.5B-Instruct",
    "quantization": "Q4_K_M",
    "size_gb": 0.9,
    "gguf": "base_model/model.gguf",
    "tokenizer": "base_model/tokenizer.json",
    "config": "base_model/config.json",
    "hash": "blake3:7f3a..."
  },
  "modules": [
    {
      "module_id": "mod_rust_code",
      "name": "Rust Programming",
      "path": "modules/mod_rust_code",
      "type": "textual_memory",
      "item_count": 45231,
      "hash": "blake3:abc1..."
    }
  ],
  "adapters": [
    {
      "name": "safety-dpo-v1",
      "path": "adapters/safety-dpo-v1",
      "hash": "blake3:def2..."
    }
  ],
  "training_data_included": false,
  "runtime_requirements": {
    "min_ram_gb": 4,
    "min_vram_gb": 0,
    "bkp_loader_version": ">=1.0"
  },
  "default_retrieval": {
    "active_module_ids": ["mod_rust_code", "mod_safety_rules"],
    "top_k": 5,
    "inject_mode": "system_prompt_prefix"
  },
  "content_hash": "blake3:full_package_hash",
  "signature": null
}
```

### 3.3 New Rust Crate: `crates/bonsai-package`

```
crates/bonsai-package/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── writer.rs       # PackageWriter: build a .bkp from components
    ├── reader.rs       # PackageReader: open and random-access a .bkp
    ├── manifest.rs     # Manifest + Module + Adapter structs
    ├── verify.rs       # Hash verification, signature checking
    └── extract.rs      # Library import: extract .bkp into ~/.bonsai/
```

**Key types:**

```rust
// src/writer.rs
pub struct PackageWriter {
    zip: zip::ZipWriter<File>,
    manifest: PackageManifest,
}

impl PackageWriter {
    pub fn new(output_path: &Path, name: &str) -> Result<Self>;
    
    /// Add the base model GGUF + tokenizer + config
    pub fn add_base_model(&mut self, model_dir: &Path) -> Result<()>;
    
    /// Add a knowledge module directory
    pub fn add_module(&mut self, module_dir: &Path) -> Result<()>;
    
    /// Add a LoRA adapter directory
    pub fn add_adapter(&mut self, adapter_dir: &Path, name: &str) -> Result<()>;
    
    /// Add training data (JSONL subset from TDL)
    pub fn add_training_data(&mut self, jsonl_path: &Path) -> Result<()>;
    
    /// Finalize: write manifest, compute content_hash, close ZIP
    pub fn finish(self) -> Result<PathBuf>;
}

// src/reader.rs
pub struct PackageReader {
    zip: zip::ZipArchive<Mmap>,  // memory-mapped for zero-copy reads
    manifest: PackageManifest,
}

impl PackageReader {
    /// Open .bkp, read manifest, verify hashes
    pub fn open(path: &Path) -> Result<Self>;
    
    /// Get the GGUF model as a memory slice (zero-copy via mmap)
    pub fn model_bytes(&self) -> Result<&[u8]>;
    
    /// Get a module's index as a memory slice
    pub fn module_index_bytes(&self, module_id: &str) -> Result<&[u8]>;
    
    /// Get a module's key vectors
    pub fn module_keys(&self, module_id: &str) -> Result<&[u8]>;
    
    /// Get a module's values (decompresses zstd if needed)
    pub fn module_values_text(&self, module_id: &str) -> Result<Vec<String>>;
    
    /// Extract all contents to a directory (for library import)
    pub fn extract_all(&self, target_dir: &Path) -> Result<()>;
    
    /// Extract a specific module to target dir (partial import)
    pub fn extract_module(&self, module_id: &str, target_dir: &Path) -> Result<()>;
}
```

### 3.4 Tauri Commands for Package Management

```rust
// src-tauri/src/package_manager.rs

/// Export a model manifest as a .bkp package
#[tauri::command]
pub async fn bkp_export(
    model_id: String,
    output_path: String,
    include_training_data: bool,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns final file path

/// Load a .bkp without importing (runs directly from archive)
#[tauri::command]
pub async fn bkp_load(
    package_path: String,
    state: State<'_, AppState>,
) -> Result<LoadedPackageInfo, String>;

/// Import a .bkp into ~/.bonsai/ library permanently
#[tauri::command]
pub async fn bkp_import(
    package_path: String,
    state: State<'_, AppState>,
) -> Result<String, String>; // returns model_id

/// Eject a temporarily loaded package
#[tauri::command]
pub async fn bkp_eject(
    package_id: String,
    state: State<'_, AppState>,
) -> Result<(), String>;

/// List currently loaded packages (temporary + imported)
#[tauri::command]
pub async fn bkp_list_loaded(
    state: State<'_, AppState>,
) -> Result<Vec<PackageInfo>, String>;

/// Verify a package's integrity (hashes + optional signature)
#[tauri::command]
pub async fn bkp_verify(
    package_path: String,
    state: State<'_, AppState>,
) -> Result<VerificationResult, String>;
```

### 3.5 Drag-and-Drop Import

Tauri file association for `.bkp`:

```json
// tauri.conf.json
"bundle": {
  "fileAssociations": [
    { "ext": ["bkp"], "name": "Bonsai Knowledge Package", "mimeType": "application/x-bonsai-package" }
  ]
}
```

When a `.bkp` is double-clicked or dragged onto the Bonsai window:
```rust
// App.svelte / deep_link handler
app.listen("tauri://file-drop", |event| {
    if let Some(paths) = event.payload {
        for path in paths.filter(|p| p.ends_with(".bkp")) {
            show_import_dialog(path);
        }
    }
});
```

Import dialog options: **[Load temporarily]** | **[Import to Library]** | **[Cancel]**

### 3.6 Provenance Chain

`provenance.json` inside every `.bkp`:
```json
{
  "package_id": "pkg_a1b2c3d4",
  "base_model_source": "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct (downloaded 2026-01-15)",
  "training_runs": [
    {
      "phase": "safety",
      "method": "DPO",
      "dataset": "Safety DPO v1 (50 pairs)",
      "config": {"beta": 0.15, "epochs": 3, "lr": 5e-5},
      "completed_at": "2026-05-30T07:38:00Z"
    }
  ],
  "knowledge_modules": [
    {
      "module_id": "mod_rust_code",
      "source_dataset": "Rust Code SFT v3 (12,400 examples)",
      "built_at": "2026-05-30T10:00:00Z"
    }
  ],
  "lineage_hash": "blake3:..."
}
```

### 3.7 Python Loader Library (`scripts/bkp_tools.py`)

For use by external software and the Training Agent:

```python
# scripts/bkp_tools.py
"""Bonsai Knowledge Package tools — load, inspect, and extract .bkp files."""

import zipfile, json, struct, numpy as np
from pathlib import Path

class BkpReader:
    def __init__(self, path: str):
        self.zip = zipfile.ZipFile(path, 'r')
        self.manifest = json.loads(self.zip.read("manifest.json"))
    
    @property
    def name(self) -> str:
        return self.manifest["name"]
    
    @property
    def modules(self) -> list[dict]:
        return self.manifest["modules"]
    
    def read_model_gguf(self) -> bytes:
        return self.zip.read(self.manifest["base_model"]["gguf"])
    
    def read_module_keys(self, module_id: str) -> np.ndarray:
        path = f"modules/{module_id}/keys.f16"
        raw = self.zip.read(path)
        return np.frombuffer(raw, dtype=np.float16)
    
    def read_module_values(self, module_id: str) -> list[str]:
        import zstandard as zstd
        raw = self.zip.read(f"modules/{module_id}/values.txt.zst")
        text = zstd.ZstdDecompressor().decompress(raw).decode("utf-8")
        return text.split("\n")
    
    def extract_for_llama_cpp(self, output_dir: str) -> str:
        """Extract just the GGUF for use with llama.cpp or other tools."""
        out = Path(output_dir)
        out.mkdir(parents=True, exist_ok=True)
        gguf_path = out / "model.gguf"
        gguf_path.write_bytes(self.read_model_gguf())
        return str(gguf_path)
    
    def describe(self) -> dict:
        return {
            "name": self.name,
            "base_model": self.manifest["base_model"]["original_id"],
            "modules": [m["name"] for m in self.modules],
            "adapters": [a["name"] for a in self.manifest.get("adapters", [])],
            "size_info": f"Estimated: {self._estimate_size_mb():.0f} MB"
        }
    
    def _estimate_size_mb(self) -> float:
        return sum(self.zip.getinfo(f.filename).file_size 
                   for f in self.zip.infolist()) / (1024*1024)


def inspect(path: str) -> None:
    r = BkpReader(path)
    info = r.describe()
    print(f"Package: {info['name']}")
    print(f"Base model: {info['base_model']}")
    print(f"Modules: {', '.join(info['modules'])}")
    print(f"Adapters: {', '.join(info['adapters'])}")
    print(info["size_info"])


if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        inspect(sys.argv[1])
```

---

## Part 4: Model Studio UI

### 4.1 Model Studio Panel Layout

New Svelte panel: `TrainingDataLibrary.svelte` + `ModelStudio.svelte`

```
┌─────────────────────────────────────────────────────────────────────────┐
│  Model Studio                                                           │
├──────────────────────────┬──────────────────────────────────────────────┤
│  MY MODELS               │  KNOWLEDGE MODULES           [+ Build Module]│
│  ──────────────────────  │  ─────────────────────────────────────────── │
│  ┌────────────────────┐  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │ BonsAI Default     │  │  │ Rust Code│  │ Safety   │  │Survival  │  │
│  │ Qwen2.5-1.5B       │  │  │ 45k items│  │ 500 items│  │ 200 items│  │
│  │ No KDB modules     │  │  │ ★★★★☆   │  │ ★★★★★   │  │ ★★★★☆   │  │
│  │ [Edit] [Export]    │  │  └──────────┘  └──────────┘  └──────────┘  │
│  └────────────────────┘  │                                              │
│  ┌────────────────────┐  │  ─────────────────────────────────────────── │
│  │ Rust Expert v1     │  │  MODEL BUILDER CANVAS                        │
│  │ Qwen2.5-1.5B       │  │                                              │
│  │ + Rust Code        │  │  Base Model: [Qwen2.5-1.5B    ▼]            │
│  │ + Safety           │  │  Adapter:    [safety-dpo-v1   ▼] [none]     │
│  │ ★★★★☆ Quality est │  │                                              │
│  │ [Edit] [Export ↓]  │  │  Knowledge Modules (drag from above):        │
│  └────────────────────┘  │  ┌────────────────────────────────────────┐  │
│                          │  │ ⠿ Rust Code          weight: ████ 1.0  │  │
│  [+ New Model]           │  │ ⠿ Safety Rules       weight: ████ 0.8  │  │
│                          │  │ [drop more modules here]               │  │
│                          │  └────────────────────────────────────────┘  │
│                          │                                              │
│                          │  [Test Model ▶] [Save] [Export .bkp ↓]      │
└──────────────────────────┴──────────────────────────────────────────────┘
```

### 4.2 Knowledge Module Card

Each module card shows:
- Module name, type badge (📝 text / 🧮 dense / 🔌 adapter)
- Item count, domain tags, quality stars
- Source dataset link
- "Rebuild" button (rebuilds module from updated dataset version)
- Drag handle for dropping onto Model Builder canvas

### 4.3 Drag-and-Drop Dataset → Module Auto-Build

When a user drags a **dataset** onto the Knowledge Modules area:
1. System checks: does a module already exist for this dataset?
2. If yes: adds existing module
3. If no: shows "Build module from dataset?" dialog with options (type, HNSW params)
4. Launches `build_knowledge_module.py` as a background job
5. Progress bar in the module card; "✓ Ready" when complete

---

## Part 5: Implementation Roadmap

### Week 1: TDL Backend
- [ ] Create `~/.bonsai/tdl.db` with schema
- [ ] `training_data_library.rs` — all Tauri commands
- [ ] Auto-ingestion watcher on `~/.bonsai/training_export/`
- [ ] Register existing safety DPO dataset as first TDL entry
- [ ] Import existing `bonsai_logs.jsonl` (Training Agent data) as a dataset

### Week 2: TDL Frontend
- [ ] `TrainingDataLibrary.svelte` — card grid, search, filters
- [ ] Dataset detail pane with item preview and flag/unflag
- [ ] Import wizard (file drop + URL)
- [ ] Drag source on dataset cards

### Week 3: KDB Backend
- [ ] `crates/bonsai-hnsw/` — pure-Rust HNSW implementation
- [ ] `crates/bonsai-kdb/` — retriever, module loader, manifest management
- [ ] `scripts/build_knowledge_module.py` — pipeline to build .kmod from dataset
- [ ] KDB Tauri commands
- [ ] Wire KDB retriever into chat inference (Phase 1: system prompt injection)

### Week 4: Package Format
- [ ] `crates/bonsai-package/` — PackageWriter, PackageReader
- [ ] Tauri commands: `bkp_export`, `bkp_load`, `bkp_import`, `bkp_eject`
- [ ] `.bkp` file association (tauri.conf.json)
- [ ] `scripts/bkp_tools.py` — Python loader for external use

### Week 5: Model Studio UI
- [ ] `ModelStudio.svelte` — model list, KDB canvas, drag-and-drop
- [ ] Module cards with build/rebuild flow
- [ ] Test panel (run prompts against assembled model)
- [ ] Export .bkp button

### Week 6: Integration + Audit UI
- [ ] Wire everything into `AppState`
- [ ] Flag/unflag propagation: flag an example → rebuild module → hot-swap
- [ ] Provenance viewer (trace a module back to its source examples)
- [ ] Weekly auto-rebuild: if dataset updated, queue module rebuild

---

## Part 6: Data Audit & Removal Flow

This is the key differentiator from monolithic models:

```
1. User notices bad output from the model
2. Opens Knowledge Explorer, finds the relevant module
3. Searches for the bad example (by keyword or by seeing it in the top-K retrievals)
4. Clicks "Flag This Example" with reason
5. System: marks example flagged in tdl.db
6. System: schedules module rebuild (exclude flagged)
7. On next rebuild: new .kmod without the bad example
8. Hot-swap: KdbRetriever.set_active_modules([new_module_path])
9. Bad knowledge is gone — in seconds, without any retraining
```

Timeline from "I found bad data" to "model no longer uses it": **< 5 minutes**.

With traditional monolithic models, removing bad training data requires full retraining: **days to weeks**.

---

*This document is the canonical spec for the TDL, KDB, and .bkp systems. See [SPEEDRUN_PLAN.md](SPEEDRUN_PLAN.md) for how this integrates with the sovereignty plan.*
