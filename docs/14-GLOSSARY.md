# Glossary

Definitions for every term used in the Bonsai Ecosystem documentation.

---

**Activity Log**  
A panel in the Bonsai Workspace that records every significant event: tool calls, model inference, training progress, file operations, network events, and errors. Events are stored in `~/.bonsai/activity.log` with daily rotation. See [02-CORE-IDE.md §Activity Log](02-CORE-IDE.md).

**Adapter** (also: LoRA adapter)  
A small set of extra weight matrices that modify a base model's behaviour without replacing the entire model. Created by fine-tuning (DPO or SFT). Stored as `.safetensors` in `~/.bonsai/adapters/`. Can be hot-swapped without restarting the app. See [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md).

**Aether Actors**  
The actor system used by Bonsai's backend. Each long-running component (model orchestrator, transfer daemon, survival engine, etc.) is an actor with its own message queue. If an actor crashes, its supervisor restarts it. State is checkpointed to CAS. Implemented in `crates/bonsai-actors/`.

**Argon2id**  
A memory-hard password hashing algorithm used to derive encryption keys from your passphrase. Configured with 64 MB memory, 3 iterations, and 4 parallelism lanes. Resistant to GPU-based brute force attacks.

**Audit Log (WAL)**  
A tamper-evident Write-Ahead Log recording all security-relevant events (tool approvals, file writes, peer connections). Each entry is BLAKE3-chained to the previous entry. Stored at `~/.bonsai/audit.wal`. See [11-SECURITY.md §Audit Logging](11-SECURITY.md).

**Base Model**  
A GGUF file providing the core language understanding and reasoning capability. Combined with knowledge modules and optionally a LoRA adapter in the Model Builder. See [06-KNOWLEDGE-DATABASE.md](06-KNOWLEDGE-DATABASE.md).

**BLAKE3**  
A cryptographic hash function used throughout Bonsai for content addressing (CAS keys), integrity verification, and audit log chaining. Faster than SHA-256 while being equally secure.

**`.bkp` (Bonsai Knowledge Package)**  
A portable ZIP archive (store mode) containing a base model, knowledge modules, and their manifests. Allows a complete AI configuration to be shared as a single file. See [06-KNOWLEDGE-DATABASE.md §Knowledge Packages](06-KNOWLEDGE-DATABASE.md).

**BonsAI**  
The AI assistant at the heart of the Bonsai Ecosystem. Runs locally on your hardware. Supports tool calls, code generation, reasoning, and personalisation through training. See [03-BONSAI-ASSISTANT.md](03-BONSAI-ASSISTANT.md).

**Bonsai Buddy**  
A lightweight, detachable, always-on-top AI chat window. Can be launched independently (`--mode buddy`) without the full IDE. Features voice synthesis (TTS) and voice input. See [03-BONSAI-ASSISTANT.md §BonsAI Buddy](03-BONSAI-ASSISTANT.md) and [09-MOBILE.md](09-MOBILE.md).

**BONSAI.md**  
A Markdown file in your workspace root that customises BonsAI's behaviour, style, and rules for the current project. Read at the start of every conversation. Updated nightly by DreamAgent. See [03-BONSAI-ASSISTANT.md §BONSAI.md](03-BONSAI-ASSISTANT.md).

**Brain Age**  
A gamified score (0–100%) measuring how well-trained your local BonsAI model is. Increases as training phases complete. Displayed as a badge in the status bar. Levels: Newborn → Curious → Learning → Smart → Genius → Flawless. See [04-MODEL-TRAINER.md §Brain Age](04-MODEL-TRAINER.md).

**Brain Update**  
A completed training phase that produces a new LoRA adapter. After a brain update, the model is hot-reloaded and Brain Age increases.

**CAS (Content-Addressed Storage)**  
A storage system where every piece of data is identified by its BLAKE3 hash. Enables deduplication, tamper detection, and reproducibility. Used for files, models, training data, and audit logs. Implemented in `crates/bonsai-cas/`.

**Capability / Effect**  
A declared permission required to perform an operation (e.g., `FileWrite(path)`, `Execute(cmd)`, `NetworkConnect(url)`). The TrustGuard enforces that agents can only use capabilities they have been granted. See [11-SECURITY.md §Capability-Based Security](11-SECURITY.md).

**Collaboration Session**  
A real-time workspace shared between multiple Bonsai users. Provides shared file editing (CRDT), group chat, voice/video calls, and terminal sharing over TransferDaemon. Created with an invitation code. See [07-COLLABORATION.md](07-COLLABORATION.md).

**CRDT (Conflict-Free Replicated Data Type)**  
A data structure that can be updated independently by multiple parties and merged automatically without conflicts. Used in Bonsai for shared file trees (`Map`), collaborative editing (`Sequence`/RGA), and module sets (`OrSet`). Implemented in `crates/bonsai-crdt/`. See [07-COLLABORATION.md](07-COLLABORATION.md).

**Daemon (bonsai-daemon)**  
The headless background service that exposes all Bonsai functionality over a local WebSocket JSON-RPC API. Used by the VSCode extension and other external clients. Starts automatically with the desktop app or can be run standalone.

**Distillation**  
A training method where a smaller student model learns to match the outputs of a larger, more capable teacher model. Used in Phases 4, 5, and 6 of the training pipeline to improve BonsAI's code, chat, and reasoning. See [04-MODEL-TRAINER.md §Distillation](04-MODEL-TRAINER.md).

**DPO (Direct Preference Optimisation)**  
A training algorithm that improves model behaviour by learning from pairs of (preferred response, rejected response) for the same prompt. Used for safety, survival, and tool-use training. See [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md).

**DreamAgent**  
A custom-trained 1.5B model that runs nightly to consolidate the day's memory nodes, update `BONSAI.md` with learned preferences, prune low-quality training data, and generate a training summary report. See [04-MODEL-TRAINER.md §EternalTrainingLoop](04-MODEL-TRAINER.md).

**ECF-RG Scheduler**  
The **Earliest Completion First with Reorder Guard** scheduler inside TransferDaemon's core engine. It dynamically distributes data chunks across available transport lanes to minimise total transfer time while guaranteeing in-order delivery.

**EternalTrainingLoop**  
An optional background process that continuously trains BonsAI on new data as it accumulates. Runs at low priority, pauses on low battery or high temperature, and triggers a Quick Train when enough new examples have been collected. See [04-MODEL-TRAINER.md §EternalTrainingLoop](04-MODEL-TRAINER.md).

**Feature Flag**  
A boolean switch in `config/features.yaml` and the Settings UI that enables or disables experimental capabilities. Allows incomplete features to exist in the codebase without affecting users. See [02-CORE-IDE.md §Feature Flags](02-CORE-IDE.md).

**GGUF**  
A binary format for quantised language model weights, used by llama.cpp and llama-server. Typically named like `model-Q4_K_M.gguf`. The `Q4_K_M` suffix describes the quantisation level (4-bit keys and 4-bit matrices, "medium" variant).

**Hot Reload**  
Swapping in a new model or adapter without restarting the application. After a training phase completes, the new adapter is hot-reloaded automatically. The next message uses the improved model. See [04-MODEL-TRAINER.md §Hot Reload](04-MODEL-TRAINER.md).

**HNSW (Hierarchical Navigable Small World)**  
A graph-based data structure for approximate nearest-neighbour search in high-dimensional vector spaces. Used in knowledge modules to find the most relevant text passages for a query. Implemented in `crates/bonsai-hnsw/`. See [06-KNOWLEDGE-DATABASE.md](06-KNOWLEDGE-DATABASE.md).

**KDB (Knowledge Database)**  
The system for managing knowledge modules. Includes a SQLite registry (`KdbStore`), an in-memory retriever (`KdbRetriever`), and the HNSW index inside each module. See [06-KNOWLEDGE-DATABASE.md](06-KNOWLEDGE-DATABASE.md).

**Knowledge Module (`.kmod`)**  
A directory containing an HNSW vector index, compressed text values, and a manifest. Provides domain-specific knowledge that is retrieved and injected into BonsAI's context at inference time. Swappable at runtime. See [06-KNOWLEDGE-DATABASE.md §Knowledge Module Format](06-KNOWLEDGE-DATABASE.md).

**LoRA (Low-Rank Adaptation)**  
A fine-tuning technique that adds small rank-decomposed matrices to specific layers of a model. Produces an adapter much smaller than the full model. Bonsai trains LoRA adapters for each training phase. See [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md).

**Memory Node**  
A fact, event, or preference recorded by BonsAI from a conversation. DreamAgent consolidates memory nodes nightly and updates `BONSAI.md`.

**Model Builder**  
A three-column UI panel (base model + knowledge modules + actions) for composing a custom AI configuration. Supports runtime module toggling and package export/import. See [06-KNOWLEDGE-DATABASE.md §Model Builder UI](06-KNOWLEDGE-DATABASE.md).

**Model Trainer**  
The UI panel and background pipeline for fine-tuning BonsAI on your local data. Runs seven training phases in sequence. See [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md).

**Noise Protocol (Noise_XX)**  
A cryptographic handshake framework used by TransferDaemon. The `XX` pattern means both parties authenticate each other using their static Ed25519 keys, then establish a forward-secret session key via X25519 Diffie-Hellman.

**Plan Review Gate**  
A confirmation dialog that appears before BonsAI executes high-risk actions (file write, command execution, deletion). Shows a numbered plan; you can approve, reject, or edit before proceeding. See [03-BONSAI-ASSISTANT.md §Plan Review Gate](03-BONSAI-ASSISTANT.md).

**QLoRA (Quantised LoRA)**  
A memory-efficient variant of LoRA training where the base model is loaded in 4-bit quantisation while the LoRA matrices are trained in full precision. Allows fine-tuning 7B models on consumer GPUs with 8–12 GB VRAM.

**Relay (bonsai-relay)**  
A blind relay server that helps devices behind NAT firewalls connect. It forwards encrypted packets between peers without being able to read them (end-to-end encryption is preserved). Can be self-hosted. See [11-SECURITY.md §Relay Trust](11-SECURITY.md).

**Resource Guard**  
A Tauri middleware that rate-limits and validates all IPC commands to prevent resource exhaustion attacks from the webview. Logs all commands for the audit trail.

**Retrieval**  
The process of finding the most relevant passages from active knowledge modules for a given query (user message). Uses HNSW approximate nearest-neighbour search on the query's embedding vector. See [06-KNOWLEDGE-DATABASE.md §Retrieval in Action](06-KNOWLEDGE-DATABASE.md).

**Sandbox**  
An isolated execution environment for code run by BonsAI tools. Three tiers: WASM (default), gVisor container, or native with TEE attestation. See [11-SECURITY.md §Sandboxing](11-SECURITY.md).

**SFT (Supervised Fine-Tuning)**  
A training method where the model learns to produce specific outputs for specific inputs from a labelled dataset. Used in the final merge phase of BonsAI's training. See [04-MODEL-TRAINER.md](04-MODEL-TRAINER.md).

**Sidecar**  
An external process managed by Bonsai. Examples: `llama-server` (inference), `whisper` (speech-to-text), Python training scripts. Sidecars are started and stopped by Bonsai's `SidecarManager` and appear as dots in the System Health Panel.

**Survival KB (Survival Knowledge Base)**  
A SQLite database at `~/.bonsai/survival_kb.db` containing `(symptom_pattern → fix_script)` rules. Pre-seeded with 35 rules; grows as Bonsai learns from successful repairs. See [05-SURVIVAL-SYSTEM.md §Survival Knowledge Base](05-SURVIVAL-SYSTEM.md).

**Survival System**  
The self-healing subsystem comprising the Watchdog process, Survival Engine, and Survival Knowledge Base. Detects crashes and errors, applies known fixes, and escalates to AI-generated repairs when no rule matches. See [05-SURVIVAL-SYSTEM.md](05-SURVIVAL-SYSTEM.md).

**Tauri**  
The framework used to build the Bonsai desktop app. Tauri wraps a Rust backend with a web-based frontend (Svelte), using the OS's native WebView. Provides the IPC bridge between Svelte and Rust. [tauri.app](https://tauri.app)

**TEE (Trusted Execution Environment)**  
A secure area inside a processor (Intel SGX, ARM TrustZone, Apple Secure Enclave) that can run code and handle data in isolation from the main OS. Used for the highest-security tier of Bonsai's compute sandbox.

**Tool Call**  
A structured JSON command generated by BonsAI to invoke a registered tool. Format: `{"tool": "read_file", "args": {"path": "src/main.rs"}}`. The Tauri backend executes the tool and returns the result. See [03-BONSAI-ASSISTANT.md §Tool Calls](03-BONSAI-ASSISTANT.md).

**Training Data Library (TDL)**  
A structured directory of training examples at `~/.bonsai/training_data/`, organised by domain. Feeds the Model Trainer. Examples are created from user feedback (👍/👎), survival events, and manual imports. See [04-MODEL-TRAINER.md §Dataset Management](04-MODEL-TRAINER.md).

**TransferDaemon**  
The universal peer-to-peer communication protocol powering all inter-device features in Bonsai. Provides encrypted multi-path file transfer, messaging, CRDT sync, media streaming, knowledge fetch, and task distribution. Implemented across `bonsai-transfer-core`, `bonsai-transfer-crypto`, `bonsai-transfer-store`, `bonsai-mailbox`, `bonsai-p2p`, and `bonsai-relay`. See [07-COLLABORATION.md §TransferDaemon](07-COLLABORATION.md).

**TrustGuard**  
The capability enforcement middleware. Checks every tool call and agent action against the granted capability set. Denies operations that exceed the granted scope. See [11-SECURITY.md §Capability-Based Security](11-SECURITY.md).

**Undercover Mode**  
A privacy feature that strips all references to "Bonsai", "BonsAI", model names, and AI attribution from system prompts, git commits, and UI labels. Useful for public demos or client work. See [03-BONSAI-ASSISTANT.md §Undercover Mode](03-BONSAI-ASSISTANT.md).

**WAL (Write-Ahead Log)**  
A log of pending operations written to disk before they are applied. Used in two ways: (1) crash recovery — uncommitted operations are replayed on the next launch; (2) audit trail — all security events are appended to the audit WAL with BLAKE3 chaining. See [11-SECURITY.md](11-SECURITY.md).

**Watchdog (`bonsai-watchdog`)**  
A small, independent Rust binary that launches the main Bonsai daemon and monitors its health via 15-second pings. On crash or timeout, it applies a fix from the Survival KB and restarts with exponential backoff. See [05-SURVIVAL-SYSTEM.md §Watchdog Process](05-SURVIVAL-SYSTEM.md).

---

*← [Troubleshooting](13-TROUBLESHOOTING.md) · [Overview →](00-OVERVIEW.md)*
