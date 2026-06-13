# Knowledge Database & Model Packaging

> **"Instead of one monolithic model file, Bonsai lets you build AI from swappable knowledge modules — like LEGO bricks for intelligence."**

---

## The Concept

Traditional AI tools force you to choose one big model and live with its fixed knowledge. Bonsai separates two concerns:

- **Base Model** – the language understanding and reasoning core (a GGUF file)
- **Knowledge Modules** – independently versioned, searchable sets of facts, rules, or code patterns

At inference time, the **Retriever** queries active modules and injects the most relevant passages before each response. The result is a model that knows exactly what you need it to know, with no wasted context.

```
User message
    │
    ▼
┌─────────────────────────────┐
│  KDB Retriever               │
│  Query: "user message"       │
│  → top-k from each module   │
└────────────┬────────────────┘
             │ relevant context
             ▼
┌─────────────────────────────┐
│  System Prompt               │
│  [BONSAI.md rules]           │
│  [retrieved knowledge]       │
│  [user message]              │
└────────────┬────────────────┘
             │
             ▼
        Base Model (GGUF)
             │
             ▼
        Response
```

---

## Knowledge Module Format (`.kmod`)

A knowledge module is a directory:

```
my-module/
├── manifest.json       # metadata: name, version, domain, dim, entry_count
├── index.hnsw          # HNSW vector index (bonsai-hnsw format)
├── keys_f16.bin        # embedding vectors stored as f16 (half precision)
└── values.zst          # text values, newline-delimited, Zstd compressed
```

### `manifest.json` Fields

```json
{
  "id": "uuid-v4",
  "name": "rust-patterns",
  "version": "1.0.0",
  "domain": "code",
  "description": "Common Rust patterns and idioms",
  "dim": 384,
  "entry_count": 2847,
  "distance": "Cosine",
  "created_at": "2026-05-30T12:00:00Z",
  "blake3_index": "a3f2...",
  "blake3_values": "8e1c..."
}
```

The BLAKE3 hashes guarantee integrity — any corruption is detected on load.

---

## The Training Data Library (TDL)

The TDL is the source of truth for all training examples and dataset content used to build knowledge modules.

### Where it lives
`~/.bonsai/training_data/` — a structured directory of JSONL files, organised by domain.

### Supported formats

**Preference pairs (DPO)**
```jsonl
{"prompt": "How do I handle errors in Rust?", "chosen": "Use `anyhow::Result`...", "rejected": "Use unwrap everywhere..."}
```

**Completions (SFT)**
```jsonl
{"messages": [{"role": "user", "content": "..."}, {"role": "assistant", "content": "..."}]}
```

**Plain text (for knowledge modules)**
```jsonl
{"text": "The Rust borrow checker ensures memory safety without a garbage collector..."}
```

### Importing data
- **Drag-and-drop** a JSONL file into the Training panel
- **URL import** – paste a HuggingFace dataset URL
- **Tauri command**: `tdl_import_example` with path or raw JSON
- **From chat**: click 👍/👎 on any BonsAI response to automatically create a preference pair

---

## Building a Knowledge Module

1. Open **Model Builder** (🧠 toolbar icon or `Ctrl+M`)
2. Click **Build New Module**
3. Fill in the form:
   - **Name** – human-readable identifier
   - **Domain** – `code`, `science`, `business`, `personal`, etc.
   - **Dataset** – select from TDL or drop a JSONL file
   - **Embedding model** – defaults to `all-MiniLM-L6-v2` (384-dim, fast)
   - **Index parameters** – M (connectivity, default 16), ef_construction (quality, default 200)
4. Click **Build**. Progress bar shows:
   - `Tokenising…`
   - `Generating embeddings…` (batch GPU/CPU)
   - `Building HNSW index…`
   - `Compressing values…`
   - `Registering in KDB…`
5. The module appears in your library immediately.

Building 10,000 entries takes ~2 minutes on CPU, ~20 seconds on GPU.

---

## The Model Builder UI

The Model Builder has three columns:

```
┌─────────────┬────────────────────────┬──────────────┐
│ 1. Base     │ 2. Knowledge Modules   │  3. Actions  │
│ Model       │                        │              │
│ ○ Qwen-1.5B │ ● rust-patterns (On)   │ 🧪 Test      │
│ ● Qwen-7B ✓ │ ○ python-stdlib (Off)  │ 📦 Export    │
│ ○ DeepSeek  │ ● bonsai-docs (On)     │ 📥 Import    │
│             │                        │ 🔄 Refresh   │
└─────────────┴────────────────────────┴──────────────┘
```

### Column 1 — Base Model
Click any model to select it as the reasoning core. The active model is highlighted with a blue border and checkmark.

### Column 2 — Knowledge Modules
Each module shows:
- Name and domain
- Entry count and version
- **On/Off toggle** — loads/unloads the module at runtime (no restart needed)
- 🗑 Delete button — removes from registry (prompts for confirmation)

### Column 3 — Actions
- **Test Configuration** — opens a chat with the current base model + active modules. Useful for immediate feedback.
- **Export as .bkp** — packages everything into a portable file (see below).
- **Import .bkp** — loads a package from disk.
- **Refresh** — re-scans the registry for any externally added modules.

### Active modules summary
When any modules are active, a summary at the bottom of column 3 shows which modules are contributing to responses.

---

## Knowledge Packages (`.bkp`)

A `.bkp` file is a ZIP archive (store mode — no compression, enabling random access) containing:

```
package.bkp (ZIP)
├── manifest.json           # package metadata
├── provenance.json         # BLAKE3 hashes of all contents
├── base_model/             # GGUF model file(s)
│   └── model.gguf
└── kdb/
    └── modules/
        └── <module-name>/  # one directory per knowledge module
            ├── manifest.json
            ├── index.hnsw
            ├── keys_f16.bin
            └── values.zst
```

### Exporting a Package
1. Select a base model in Model Builder.
2. Enable the modules you want to include.
3. Click **Export as .bkp Package**.
4. Choose a filename and location in the save dialog.
5. Bonsai writes the package and shows a confirmation toast with file size.

### Importing a Package

**Inspect before importing:**
When you open a `.bkp` file (via Import button or file association), the **Package Import Dialog** appears:

```
┌────────────────────────────────────────────────┐
│  📦 Import Bonsai Package                      │
│                                                │
│  Name: rust-expert v1.0.0                     │
│  Base Model: Qwen2.5-7B (GGUF)                │
│  Knowledge Modules: 3                          │
│  Created: May 30, 2026                         │
│                                                │
│  ▶ Show 14 files in package                   │
│                                                │
│  [Cancel]          [📥 Add to My Library]     │
└────────────────────────────────────────────────┘
```

Click **Add to My Library** to extract the package to `~/.bonsai/` and register all modules in the KDB store.

**Temporary use (Try It):**
In Settings → Models you can load a `.bkp` temporarily — it runs directly from the ZIP without extracting. Useful for evaluating a package before committing to disk space.

---

## Retrieval in Action

Every time you send a message, the retriever runs silently:

1. The user message is embedded using the same model used to build the module index.
2. The HNSW index performs an approximate nearest-neighbour search (`top_k=5` by default).
3. The top-k text values from each active module are formatted into a context block:

```
[Knowledge: rust-patterns]
- Use `?` operator to propagate errors: `fn foo() -> Result<T, E> { let x = may_fail()?; ... }`
- Prefer `impl Trait` in function signatures over `Box<dyn Trait>` when the concrete type is known at compile time.
...

[Knowledge: bonsai-docs]
- The Survival Engine monitors IPC heartbeat every 5 seconds.
...
```

4. This context is prepended to the system prompt, giving BonsAI domain-specific knowledge without fine-tuning.

### Configuring retrieval
In Settings → Models → Knowledge Retrieval:
- **top_k per module** – how many entries to inject (default 5)
- **Score threshold** – minimum cosine similarity to include (default 0.3)
- **Max total tokens** – cap on total injected context length (default 1024 tokens)

---

*← [Survival System](05-SURVIVAL-SYSTEM.md) · [Collaboration →](07-COLLABORATION.md)*
