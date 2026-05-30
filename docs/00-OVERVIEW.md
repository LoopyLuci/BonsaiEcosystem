# Bonsai Ecosystem – Overview & Philosophy

> **"Your AI, your hardware, your rules."**

---

## What Is Bonsai?

Bonsai is a **local-first, self-healing, sovereign AI development environment**. It runs entirely on your hardware. No cloud accounts, no telemetry, no external API keys required. Everything — your code, your models, your training data, your conversations — stays on your machine unless *you* explicitly choose to share it.

At its core, Bonsai is four things at once:

| Layer | What it does |
|---|---|
| **IDE** | A full-featured code editor, terminal, file tree, and activity log |
| **AI Platform** | On-device inference, training, and personalisation of language models |
| **Communication Layer** | Secure peer-to-peer file transfer, messaging, and collaboration |
| **Compute Fabric** | Distributed task execution across your personal devices |

---

## Core Principles

### 1 · Privacy by Default
Nothing leaves your machine without your explicit action. There is no "phone home", no crash reporter, no model telemetry. The only network activity is what *you* initiate.

### 2 · Resilience
Bonsai heals itself. If a component crashes, the Watchdog process detects the failure, applies a fix from the Survival Knowledge Base, and restarts. The goal is zero-downtime operation even under adversarial conditions.

### 3 · Modularity
Every component is replaceable. Models are swappable at runtime. Knowledge modules are loaded and unloaded independently. Features are gated behind feature flags. Dependencies are being progressively replaced with custom sovereign crates.

### 4 · User Control
You decide which models run, what data is used for training, who can connect to your device, and which capabilities each tool or agent is allowed to use. The capability system (`TrustGuard`) enforces these boundaries at runtime.

### 5 · Sovereignty
The long-term goal is a software stack with zero external dependencies — every library written and audited by the Bonsai project. See [10-SOVEREIGNTY.md](10-SOVEREIGNTY.md).

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     Bonsai Workspace (Tauri)                    │
│  ┌──────────┐  ┌──────────┐  ┌────────────┐  ┌─────────────┐  │
│  │ File Tree │  │  Monaco  │  │  Terminal  │  │  Chat Panel │  │
│  └──────────┘  └──────────┘  └────────────┘  └─────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                     Rust Backend (AppState)              │  │
│  │  ModelOrchestrator · ToolRegistry · SurvivalEngine       │  │
│  │  KdbAppState · CollaborationState · TransferState        │  │
│  └──────────────────────────────────────────────────────────┘  │
└───────────────────────────┬─────────────────────────────────────┘
                            │ IPC / WebSocket
┌───────────────────────────▼─────────────────────────────────────┐
│                       Core Crates (Rust)                        │
│  bonsai-transfer-core  bonsai-mailbox  bonsai-crdt              │
│  bonsai-hnsw           bonsai-kdb      bonsai-package           │
│  bonsai-cas            bonsai-actors   bonsai-p2p               │
│  bonsai-transfer-crypto bonsai-relay   bonsai-error             │
└───────────────────────────┬─────────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────────┐
│                       Sidecars / Processes                      │
│  llama-server (GGUF inference)  ·  Whisper (transcription)      │
│  Python training scripts (DPO, SFT, distillation)               │
│  bonsai-watchdog (health monitor)                               │
└─────────────────────────────────────────────────────────────────┘
```

---

## What Makes Bonsai Different

| Feature | Cloud Tools | Bonsai |
|---|---|---|
| Data ownership | Provider's servers | Your machine only |
| Offline use | ❌ Usually no | ✅ Full offline support |
| Model customisation | Limited fine-tune API | Full training pipeline |
| Collaboration | Cloud relay | Direct peer-to-peer |
| Self-healing | Manual | Automated Survival System |
| Dependency freedom | Thousands of libraries | Progressive sovereignty |
| Privacy | Telemetry / logs | Zero telemetry |

---

## System Requirements

| Platform | Minimum | Recommended |
|---|---|---|
| **Windows** | Windows 10 64-bit, 8 GB RAM | Windows 11, 16 GB RAM, NVIDIA GPU |
| **macOS** | macOS 12 (Monterey), Apple Silicon or Intel | macOS 14, M-series, 16 GB unified memory |
| **Linux** | Ubuntu 22.04 / Fedora 38, 8 GB RAM | Any distro, 16 GB RAM, NVIDIA GPU |
| **Android** | Android 11, 6 GB RAM | Android 14, 8 GB RAM, Vulkan 1.1 |

GPU is optional but strongly recommended for model training. CPU-only inference works for models up to ~7B parameters.

---

## Quick Feature Index

| I want to… | Go to |
|---|---|
| Set up Bonsai for the first time | [01-GETTING-STARTED.md](01-GETTING-STARTED.md) |
| Use the code editor and terminal | [02-CORE-IDE.md](02-CORE-IDE.md) |
| Chat with BonsAI | [03-BONSAI-ASSISTANT.md](03-BONSAI-ASSISTANT.md) |
| Train a custom model | [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md) |
| Understand the self-healing system | [05-SURVIVAL-SYSTEM.md](05-SURVIVAL-SYSTEM.md) |
| Build portable AI packages | [06-KNOWLEDGE-DATABASE.md](06-KNOWLEDGE-DATABASE.md) |
| Collaborate in real time | [07-COLLABORATION.md](07-COLLABORATION.md) |
| Share compute across devices | [08-COMPUTE-FABRIC.md](08-COMPUTE-FABRIC.md) |
| Use Bonsai on Android | [09-MOBILE.md](09-MOBILE.md) |
| Understand the sovereignty plan | [10-SOVEREIGNTY.md](10-SOVEREIGNTY.md) |
| Security and privacy details | [11-SECURITY.md](11-SECURITY.md) |
| Build and contribute | [12-DEVELOPER.md](12-DEVELOPER.md) |
| Fix a problem | [13-TROUBLESHOOTING.md](13-TROUBLESHOOTING.md) |
| Look up a term | [14-GLOSSARY.md](14-GLOSSARY.md) |

---

*Next: [Getting Started →](01-GETTING-STARTED.md)*
