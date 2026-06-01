 # Bonsai Workspace

 Local-first AI development workspace built with Tauri 2, Svelte, Rust, and on-device model sidecars.

🏛️ The Guiding Philosophy: Sovereignty, Privacy, and Resilience


Sovereignty by Design: The ultimate goal is a software stack with zero external dependencies, meaning every single library is written and audited by the Bonsai project itself. This is a direct counter to concerns about supply chain attacks and abandoned or changing APIs, giving the user full control and auditability over every line of code they run.

Privacy by Default: Nothing leaves your machine without your explicit action. There is no "phone home", no telemetry, and no cloud-based model API calls. On a typical day with no peer connections, the app makes zero outbound network requests, a claim you could verify with a network monitor. This is fundamentally different from cloud-based IDEs like GitHub Codespaces or AI assistants like GitHub Copilot.

Resilience & Self-Healing: The system is designed to "run forever without human intervention". Through a dedicated "Watchdog" process, it automatically detects crashes, applies fixes from a "Survival Knowledge Base," and restarts components, aiming for zero downtime.

Modularity & User Control: Every component is replaceable, models are swappable at runtime, and features are gated behind flags. The TrustGuard capability system ensures you have granular control over which capabilities—like reading/writing files or running shell commands—are granted to BonsAI.

🧩 Core Functionality and Architecture
Bonsai's functionality is organized into logical layers, all built on a modern Rust and Svelte foundation.

The BonsAI Assistant: The heart of the ecosystem is BonsAI, a local LLM that can interact with your development environment through tool calls. These tool calls are capabilities like read_file or Execute(cmd), and their use is governed by the TrustGuard system. BonsAI also features "chain-of-thought" for reasoning models and can be accessed via a detached "Buddy" window that even supports voice interaction.

The Model Trainer & Personalization Pipeline: Bonsai enables you to fine-tune models locally on your own data, a key part of its "your AI, your rules" philosophy. The training pipeline is a multi-stage process that includes:

Safety & Survival DPO: Teaching the model to refuse harmful requests and to diagnose software errors.

Tool Use DPO: Improving the model's ability to correctly formulate and select tool calls.

Distillation & Merging: Learning from a more powerful "teacher" model to compress its knowledge into the local model.
This is all orchestrated through a UI that tracks "Brain Age" and provides both full training runs and quick incremental updates.

The Knowledge Database (KDB): This is a powerful concept. Instead of retraining a monolithic model for new information, Bonsai uses retrieval-augmented generation (RAG). It separates the base model from "knowledge modules" (*.kmod files), which are searchable databases of facts, rules, or code patterns. At inference time, relevant passages from these modules are retrieved and injected into the model's context, making the AI knowledgeable without wasted context or constant retraining.

The TransferDaemon: Bonsai's networking layer is a peer-to-peer protocol that powers all collaboration, file transfer, and sync features. It can dynamically select the best transport lane (WebRTC, libp2p/QUIC, Tor, or even Bluetooth) and handles NAT traversal automatically, all with end-to-end encryption.

The Compute Fabric: This feature allows you to pool computational resources from all your devices (including phones and tablets) for a "distributed supercomputer". A "Coordinator" on your main device splits large jobs and distributes them to "Worker" devices on your network.

Security & Hardening: Security is implemented in multiple layers. Encryption at rest is handled by Argon2id and AES-256-GCM for your keys and training data. Encryption in transit uses the Noise_XX protocol for session establishment, with each message signed and encrypted. The Watchdog and Survival Engine automatically monitor for and attempt to fix faults, from process crashes to runtime errors, often using BonsAI itself to generate novel fixes.

💡 The Road to Sovereignty: The "USOS" Concept
The provided documentation doesn't mention "USOS" explicitly, but the concept is clearly embedded in docs/10-SOVEREIGNTY.md. The "Sovereignty" roadmap outlines a 6-phase plan to replace all external dependencies with custom bonsai-* crates.

Phase 0: Vendor all dependencies and ensure CI runs offline (Completed).

Phase 1: Replace core functionality like error handling, logging, RNG, and IDs (In Progress).

Phase 2-6: Gradually replace everything from crypto and databases to ML inference, git functionality, and finally, the entire async runtime and the Tauri/Svelte app framework (Planned).

The Unnamed Sovereign Operating System (USOS) is almost certainly the final stage of this roadmap. It represents the long-term vision where Bonsai is not just an application but a foundational layer that replaces the operating system's own dependencies, creating a fully self-contained, auditable, and sovereign computing environment.

🤔 How Bonsai Workspace Stands Out
Bonsai's philosophy leads to a very different feature set compared to mainstream tools:

True Local-First vs. Cloud-Dependent: Unlike Cursor or Copilot, Bonsai is designed to work completely offline, a core selling point for users concerned about privacy and vendor lock-in.

Radical Modularity vs. Monolithic Models: The separation of base models from knowledge modules is more flexible than standard fine-tuning or RAG implementations seen in projects like PrivateGPT.

Peer-to-Peer Collaboration vs. Client-Server: Instead of relying on a central server for collaboration features, Bonsai's TransferDaemon creates direct, secure connections between users, a concept not found in mainstream IDEs.

Resilience as a Core Feature vs. an Afterthought: The integrated Watchdog and self-healing mechanisms are features typically reserved for enterprise server software, not a desktop development environment.

 This repository bundles a full desktop IDE, a native Rust backend for local automation and device tooling, and optional on-device model sidecars so you can iterate without cloud credentials.

 This README summarizes the product features, developer quick-starts, and where to find detailed docs in the repository.

 ## Documentation

 | Doc | Description |
 |-----|-------------|
 | [00 · Overview & Philosophy](docs/00-OVERVIEW.md) | Vision, principles, architecture, system requirements |
 | [01 · Getting Started](docs/01-GETTING-STARTED.md) | Installation, first launch, quick tour |
 | [02 · Core IDE Features](docs/02-CORE-IDE.md) | Editor, terminal, file tree, settings, feature flags |
 | [03 · BonsAI Assistant](docs/03-BONSAI-ASSISTANT.md) | Chat, tool calls, Buddy window, Undercover Mode |
 | [04 · Model Trainer](docs/04-MODEL-TRAINER.md) | Training pipeline, Brain Age, EternalTrainingLoop |
 | [05 · Survival System](docs/05-SURVIVAL-SYSTEM.md) | Watchdog, self-healing, crash recovery |
 | [06 · Knowledge Database](docs/06-KNOWLEDGE-DATABASE.md) | Knowledge modules, Model Builder, .bkp packages |
 | [07 · Collaboration](docs/07-COLLABORATION.md) | TransferDaemon, shared editing, calls, permissions |
 | [08 · Compute Fabric](docs/08-COMPUTE-FABRIC.md) | Distributed computing across personal devices |
 | [09 · Mobile Apps](docs/09-MOBILE.md) | Bonsai Workspace Android, Bonsai Buddy Android |
 | [10 · Sovereignty](docs/10-SOVEREIGNTY.md) | Replacing all external dependencies with custom crates |
 | [11 · Security & Privacy](docs/11-SECURITY.md) | Encryption, sandboxing, threat model, audit logging |
 | [12 · Developer Guide](docs/12-DEVELOPER.md) | Build, contribute, add features, architecture |
 | [13 · Troubleshooting](docs/13-TROUBLESHOOTING.md) | Common issues and fixes |
 | [14 · Glossary](docs/14-GLOSSARY.md) | Definitions for every Bonsai term |

 ## Demo

 **MLP walkthrough video:** [`docs/bonsai-mlp-demo.mp4`](docs/bonsai-mlp-demo.mp4) _(record with OBS following [`docs/VIDEO_TUTORIAL_SCRIPT.md`](docs/VIDEO_TUTORIAL_SCRIPT.md))_

 **Docs site:** built with VitePress — run `npm run docs:dev` in `docs/site/` to browse locally.

 ## What's New

 - Model Data system with rich metadata and AI-assisted model profile generation for local and catalog models.
 - Quick Options dropdown in chat for Weather, Time, Files, Sys Stats, and Web actions.
 - Task queue with priority-aware scheduling and resource gating so chat/tool requests can be processed safely under load.
 - BonsaiBot multi-platform support across Discord, Telegram, Email, and Matrix.
 - BonsaiExeLauncherBuilder scripts for repeatable local build and packaging workflows.

 ## Quick Start

 1. Launch the desktop stack:

 ```powershell
 node bonsai-workspace/src/launch-all.mjs --mode desktop
 ```

 2. Open Model Selector and choose a model to load.
 3. Start chatting with Bonsai Buddy in the chat panel.

 ## Building from Source

 Use the launcher builder scripts from the repository root to produce desktop artifacts:

 ```powershell
 .\BonsaiExeLauncherBuilder.ps1
 ```

> **Note:** `npm audit fix --force` will upgrade Svelte 4 to 5 and Vite 5 to 6,
> which breaks the frontend build. Only use `npm audit fix` (without `--force`).

 The builder script runs frontend and Tauri build steps, then resolves and stages the built executable. See launcher options in the script help and `bonsai-workspace/launcher_manual.md`.

 ## Highlights

 - Multi-pane IDE with file tree, Monaco editor, integrated terminal, command palette, status bar, and activity-first logging.
 - Assistant & Bonsai Buddy: an integrated assistant system (chat, assistant profiles, TTS, saved sessions) with a detachable always-on-top Buddy window.
 - BonsaiBot: a lightweight messaging bot server for Discord/Telegram/Matrix/Email with an admin API used by the workspace.
 - Multi-agent swarm orchestration for orchestrating many small agents with leader/worker semantics, retries, and resource gating.
 - Mobile tooling: Android USB Lab, QR mobile pairing, Mobile Viewer (scrcpy integration), remote surface streaming & input.
 - Rich tooling: editor tool profiles (lint/format/test), per-language commands, Agent Vision, Agent Connect and plugin tooling.

 ## Quick Start (Windows)

 From the repository root (recommended):

 ```powershell
 cd Z:\Projects\BonsaiWorkspace\bonsai-workspace
 # Start the local Rust bot (admin API)
 .\bonsai-bot\target\release\bonsai-bot.exe

 # In another shell: run the desktop app (dev)
 npx tauri dev
 ```

 Or use the provided launchers from the workspace root:

 ```powershell
 .\Launch-BonsaiWorkspace.cmd            # one-click start
 .\Launch-BonsaiWorkspace.ps1           # PowerShell variant
 ```

 Common modes:

 ```powershell
 .\Launch-BonsaiWorkspace.cmd -Mode desktop+usb
 .\Launch-BonsaiWorkspace.cmd -Mode desktop+usb -RemoteSurfaceSmoke
 ```

 Developer flow (frontend + Tauri):

 ```bash
 cd bonsai-workspace/src
 npm install
 # from the workspace root Tauri finds src-tauri/tauri.conf.json
 cd ..
 npx tauri dev
 ```

 If you prefer building a production bundle:

 ```bash
 cd bonsai-workspace/src
 npm run build
 cd ../src-tauri
 cargo tauri build
 ```

 ## Key Components & Features

 ### Editor & Explorer

 - File tree with quick create, filter, and context actions.
 - Monaco editor with language autodetection, autosave, inline completions, and diff hunk apply/reject.
 - Per-language tooling profiles (format, lint, test, run) with persisted templates and placeholders.

 ### Assistant, Bonsai Buddy & Session Tools

 - Full featured assistant with:
   - persistent profiles and avatars,
   - saved chat sessions and session history,
   - approval-gated tool calls and replayable tool traces,
   - TTS playback and voice synthesis management.
 - `Bonsai Buddy` — detachable assistant window controlled via Tools → Bonsai Buddy or `toggle_assistant_window` Tauri command.

 ### BonsaiBot (Messaging Bot)

 - A small server (`bonsai-bot`) that provides an admin API for messaging integrations and automation.
 - Implements platform adapters (Discord, Telegram, Matrix, Email) and exposes tests and configuration via the app Settings.
 - Port discovery and token storage use OS keyring and a persisted port probe file (`bonsai-bot-port.json`).
 - See `bonsai-bot/MAIL_SERVER_PROD_PLAN.md` for the mail server rollout plan and integration notes.

 ### Multi-Agent Swarm, Agent Vision & Agent Connect

 - Persona and agent config CRUD for multi-agent workflows.
 - Leader/worker orchestration with runtime controls, token streaming, and debug event emission.
 - `Agent Vision` for image/video analysis workflows and `Agent Connect` for remote session orchestration.

 ### Mobile Tooling & Android USB Lab

 - `Android USB Lab` provides a guided readiness flow: detect device, check authorization, configure reverse port, install APKs, bootstrap connection, and run regression suites.
 - `Mobile Viewer` uses `scrcpy` (when available) to mirror and control a connected Android device; when scrcpy is missing the UI lists candidate executable paths resolved by the backend to aid troubleshooting.
 - Remote Surface: a web-accessible frame + input endpoints for device streaming and input routing.

- Mobile automation server: supports optional token-based authentication. The Testing Toolkit UI exposes a "Server auth token" field to supply the token when required; see `SECURITY.md` for recommended token storage and rotation practices.

Requirements for Android workflows: `adb` (Android platform tools) and, for screen mirroring, `scrcpy` installed on the host.

 ### Model Orchestration & Sidecars

 - Local model orchestration for `llama-server` style backends and optional whisper/tts sidecars.
 - Sidecar binaries live under `src-tauri/binaries` when present (platform-suffixed); models are stored in the platform data directory.
 - The app can operate in degraded mode without any sidecars for editing and orchestration tasks.

 ### Terminal, PTY & Activity Log

 - Multi-tab PTY terminal sessions with an Activity Log tab that streams app events, tool-call traces, and diagnostics.

- Activity Log instrumentation: all interactive UI controls are annotated with `data-bonsai-action` (format "Area:Action") so user interactions are captured with structured labels. Clickable elements in draggable titlebar regions are set to `-webkit-app-region: no-drag` to preserve interactivity.

 ### Settings, Secrets & Security

 - Settings panel supports API host/port configuration, bot platform secrets, and keyring-backed credential storage.
 - `assistant_commands` expose Tauri calls for SMTP secrets (`set_smtp_credentials`, `has_smtp_credentials`, `clear_smtp_credentials`).

 ## Running & Testing

 Run the backend bot for local integration tests:

 ```powershell
 cd bonsai-bot
 cargo run --release   # or run the built binary in target/release
 # admin API listens on 127.0.0.1:11424 by default
 ```

 Start the desktop app (recommended from the `bonsai-workspace` root so Tauri finds `src-tauri/tauri.conf.json`):

 ```powershell
 cd Z:\Projects\BonsaiWorkspace\bonsai-workspace
 npx tauri dev
 ```

 Mobile/USB smoke tests:

 ```bash
 # in separate shell
 cd bonsai-workspace/src
 npm run test:android-usb-regression
 ```

 ## Docs & Where to Look

 - User-facing guides: `bonsai-workspace/user_manual.md` and `bonsai-workspace/launcher_manual.md`.
 - Developer notes: `Runner-Streaming_System.md`, `Cluster-Orchestrator-Design.md`, `Multi-Agent_Swarm.md`.
 - Mail server plan: `bonsai-bot/MAIL_SERVER_PROD_PLAN.md`.

 ## Contributing

 Please open PRs against `main` and follow the repository's CI checks. See `.github/workflows` for CI details.

 ---

 If you'd like, I can also add a short quickstart README specifically for contributors (dev-only steps and checks). Would you like that next?

## Multimodal Tools & Models

This workspace includes multiple local multimodal tools. Models must be downloaded
manually and placed into `~/.bonsai/models/`.

Common tools (tool names shown for `tools/call` / Tauri `run_tool`):
- `numarkdown_image_to_markdown` — NuMarkdown-8B OCR → Markdown (model: `NuMarkdown-8B-Thinking-Q4_K_M.gguf`)
- `numarkdown_extract_structure` — Document structure extraction
- `demo_streaming` — Demo progress-streaming tool (for testing `$\/progress`)
- `system_info` — System info (built-in)

Install models (example):

1. Download model(s) from your provider (e.g. Hugging Face) and place under:

```
%USERPROFILE%/.bonsai/models/  # Windows
~/.bonsai/models/              # Linux/macOS
```

2. Start the BonsAI app; missing-model errors will be shown in the UI when tools are invoked.

