# Getting Started with Bonsai

Welcome! This guide takes you from zero to a running Bonsai workspace in under 10 minutes.

---

## Prerequisites

Before you start, make sure you have:

- **Git** – to clone the repository
- **Rust toolchain** (`rustup`) – 1.79 or later, with the `stable` channel
- **Node.js** – 20 LTS or later (for the Svelte frontend)
- **pnpm** – `npm install -g pnpm`
- **Tauri CLI** – `cargo install tauri-cli --version "^2"`
- **(Windows only)** – Microsoft C++ Build Tools (Visual Studio 2022)
- **(Optional)** – NVIDIA CUDA 12+ or ROCm 6+ for GPU acceleration

You do **not** need a cloud account, API key, or any external service.

---

## Installation

### Option A: Build from Source (Recommended)

```powershell
# 1. Clone the repository
git clone https://github.com/LoopyLuci/BonsaiWorkspace
cd BonsaiWorkspace

# 2. Install frontend dependencies
cd bonsai-workspace
pnpm install
cd ..

# 3. Build everything (uses `just` task runner)
cargo install just
just build

# 4. Launch
just run
```

The `just build` command compiles all Rust crates with `sccache` for fast incremental builds, then builds the Svelte frontend. On a first build expect 5–15 minutes depending on your machine.

### Option B: Pre-built Binary

Download the latest release from the [GitHub Releases page](https://github.com/LoopyLuci/BonsaiWorkspace/releases).

- **Windows**: run the `.msi` installer
- **macOS**: mount the `.dmg`, drag to Applications
- **Linux**: use the `.AppImage` (no install needed) or the `.deb`/`.rpm` package

---

## First Launch

When you first launch Bonsai you will see the **Setup Wizard**:

### Step 1 — Choose a Data Directory
By default Bonsai stores everything in `~/.bonsai/`. You can change this to any path with write access. The directory will contain:

```
~/.bonsai/
├── models/          # Downloaded GGUF model files
├── kdb/             # Knowledge modules
├── adapters/        # LoRA adapter weights
├── training_data/   # Datasets for fine-tuning
├── keys/            # Encrypted identity keypair
└── survival_kb.db   # Self-healing knowledge base
```

### Step 2 — Create Your Identity
Bonsai generates a BIP-39 mnemonic (12 words) that serves as your identity for peer-to-peer communication. Write it down and keep it safe — it is the only way to recover your identity if you lose your keys.

### Step 3 — Download a Base Model
The wizard shows recommended models by size:

| Model | Size on disk | Best for |
|---|---|---|
| Qwen2.5-1.5B-Instruct Q4 | ~1 GB | Low-memory machines, quick experiments |
| Qwen2.5-7B-Instruct Q4 | ~4.5 GB | Everyday use |
| DeepSeek-R1-8B Q4 | ~5 GB | Reasoning-heavy tasks |

Click **Download** next to your chosen model. Progress is shown in the transfers panel. You can skip this step and use a model you already have on disk.

### Step 4 — Done!
The wizard closes and the main workspace opens.

---

## Launch Modes

Bonsai supports three launch modes:

| Mode | Command | What you get |
|---|---|---|
| `workspace` | `just run` | Full IDE window only |
| `buddy` | `just run -- --mode buddy` | Detached chat window only (always on top) |
| `ecosystem` | `just run -- --mode ecosystem` | Both windows together |

You can also set the environment variable: `BONSAI_LAUNCH_MODE=buddy just run`.

---

## Quick Tour of the Interface

```
┌──────────────────────────────────────────────────────┐
│  [🌿 Bonsai] [File] [View] [Tools]     [🧠 Builder]  │  ← Toolbar
├─────────┬────────────────────────┬───────────────────┤
│         │                        │                   │
│  File   │    Monaco Editor       │   Chat Panel      │
│  Tree   │    (main editing area) │   (BonsAI chat)   │
│         │                        │                   │
├─────────┤                        ├───────────────────┤
│ Activity│    Terminal            │   Model Trainer   │
│ Log     │    (bottom panel)      │   (right panel)   │
└─────────┴────────────────────────┴───────────────────┘
                                   Status Bar (bottom)
```

**Sidebar icons** (left edge):
- 📁 File Tree
- 💬 Chat / BonsAI
- 🧠 Model Builder
- 📦 Transfers
- ⚙️ Settings

Click any icon to show/hide its panel. Panels are resizable by dragging their borders.

---

## Your First Five Actions

### 1 · Open a Folder
Click **File → Open Folder** or drag a folder onto the file tree. The tree populates with your project files.

### 2 · Start a Chat
Click the 💬 icon. Type a question or paste some code and press Enter. BonsAI replies in the chat panel.

### 3 · Run a Terminal Command
Click the terminal icon at the bottom, or press `` Ctrl+` ``. A shell session opens. Type any command — it runs locally.

### 4 · Load a Different Model
Click **Settings → Models**. Select a model from the list and click **Load**. The model swaps in without restarting the app.

### 5 · Enable a Feature
Click **Settings → Features**. Toggle switches to enable experimental features like `web_router`, `model_trainer`, or `undercover_mode`.

---

## Next Steps

- **Chat deeply with BonsAI** → [03-BONSAI-ASSISTANT.md](03-BONSAI-ASSISTANT.md)
- **Train your own model** → [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md)
- **Collaborate with others** → [07-COLLABORATION.md](07-COLLABORATION.md)
- **Troubleshoot an issue** → [13-TROUBLESHOOTING.md](13-TROUBLESHOOTING.md)

---

*← [Overview](00-OVERVIEW.md) · [Core IDE Features →](02-CORE-IDE.md)*
