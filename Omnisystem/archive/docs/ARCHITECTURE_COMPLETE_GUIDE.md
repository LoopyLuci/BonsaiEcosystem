# Complete Architecture & Code Structure Guide

**Version**: 2.0  
**Date**: 2026-06-04  
**Audience**: Developers, System Architects, Contributors

---

## Table of Contents

1. [System Architecture Overview](#system-architecture-overview)
2. [Repository Structure](#repository-structure)
3. [Component Architecture](#component-architecture)
4. [Data Flow Architecture](#data-flow-architecture)
5. [Code Organization](#code-organization)
6. [Module Dependencies](#module-dependencies)
7. [Key Algorithms & Patterns](#key-algorithms--patterns)
8. [Build Architecture](#build-architecture)
9. [Testing Architecture](#testing-architecture)
10. [Deployment Architecture](#deployment-architecture)

---

## System Architecture Overview

### Five-Layer Stack

```
┌─────────────────────────────────────────────────────────┐
│ Layer 5: User Interface & Applications                 │
│  Desktop IDE (Tauri/Svelte) | Mobile Apps | Web UI    │
└─────────────────────────────┬───────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────┐
│ Layer 4: AI & Computing Services                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ BonsAI Core  │  │ Model Trainer│  │ Orchestrator │ │
│  │ (Inference)  │  │ (DPO/RLHF)   │  │ (Scheduler)  │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────┬───────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────┐
│ Layer 3: Memory & State Management                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ Knowledge DB │  │ Session Cache│  │ Event Store  │ │
│  │ (RAG/Vector) │  │ (Redis-like) │  │ (Event log)  │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────┬───────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────┐
│ Layer 2: Networking & Collaboration                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │ TransferDmn  │  │ Compute      │  │ Sync Engine  │ │
│  │ (P2P Proto)  │  │ Fabric       │  │ (CRDT)       │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
└─────────────────────────────┬───────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────┐
│ Layer 1: Operating System & Hardware Abstraction       │
│  Watchdog | Survival Engine | File System | Devices    │
│  ┌──────────────────────────────────────────────────┐  │
│  │ USOS (Unnamed Sovereign Operating System)        │  │
│  │ Replaces OS dependencies, Phase 0-6 replacement  │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Design Patterns Used

| Pattern | Usage | Example |
|---------|-------|---------|
| **Service Locator** | Finding components | Model discovery, tool registry |
| **Observer** | Event streaming | Activity log, state changes |
| **Strategy** | Pluggable behaviors | Multiple transport lanes (WebRTC, QUIC, Tor) |
| **Factory** | Object creation | Model instantiation, sidecar spawning |
| **Adapter** | Protocol bridges | Discord/Telegram/Email adapters |
| **Pipeline** | Sequential processing | Training pipeline, data preprocessing |
| **CRDT** | Collaborative editing | Real-time sync, conflict resolution |
| **Circuit Breaker** | Fault tolerance | Watchdog recovery attempts |

---

## Repository Structure

### Root Level

```
BonsaiEcosystem/
├── bonsai-workspace/              # Main Tauri application
│   ├── src/                       # Svelte frontend
│   ├── src-tauri/                # Rust backend (Tauri commands)
│   └── package.json
│
├── bonsai-bot/                    # Messaging bot server
│   ├── src/                       # Rust implementation
│   └── Cargo.toml
│
├── android-runtime/          # Mobile app (Android)
│   ├── app/src/                   # Kotlin/Java code
│   └── build.gradle
│
├── crates/                        # Shared Rust libraries
│   ├── ai-advisor/       # AI-optional fallback trait
│   ├── bonsai-transfer/          # P2P networking (TransferDaemon)
│   ├── bonsai-trainer/           # Model training engine
│   ├── bonsai-kdb/               # Knowledge database
│   ├── bonsai-fabric/            # Distributed compute
│   ├── bonsai-watchdog/          # Health monitoring
│   ├── bonsai-survival/          # Automatic recovery
│   ├── bonsai-crypto/            # Encryption (Argon2, AES)
│   ├── bonsai-sandbox/           # Process isolation
│   ├── bonsai-dpo/               # DPO training implementation
│   ├── bonsai-rlhf/              # RLHF training
│   ├── bonsai-distill/           # Model distillation
│   ├── bonsai-ui-core/           # Shared UI components
│   ├── bonsai-rpc/               # IPC & RPC system
│   ├── bonsai-error/             # Custom error types (Phase 1)
│   ├── bonsai-log/               # Logging framework (Phase 1)
│   ├── bonsai-rng/               # Random number generation (Phase 1)
│   └── bonsai-uuid/              # Unique ID generation (Phase 1)
│
├── docs/                          # Documentation
│   ├── 00-OVERVIEW.md
│   ├── 01-GETTING-STARTED.md
│   ├── 02-CORE-IDE.md
│   ├── 03-BONSAI-ASSISTANT.md
│   ├── 04-MODEL-TRAINER.md
│   ├── 05-SURVIVAL-SYSTEM.md
│   ├── 06-KNOWLEDGE-DATABASE.md
│   ├── 07-COLLABORATION.md
│   ├── 08-COMPUTE-FABRIC.md
│   ├── 09-MOBILE.md
│   ├── 10-SOVEREIGNTY.md
│   ├── 11-SECURITY.md
│   ├── 12-DEVELOPER.md
│   ├── 13-TROUBLESHOOTING.md
│   ├── 14-GLOSSARY.md
│   └── site/                     # VitePress docs site
│
├── .github/
│   └── workflows/                 # CI/CD pipelines
│       ├── rust.yml
│       ├── frontend.yml
│       ├── android.yml
│       └── release.yml
│
├── Cargo.toml                     # Workspace root (Rust)
├── Cargo.lock                     # Dependency lock file
├── package.json                   # Node dependencies
├── pnpm-lock.yaml
├── .env.example                   # Environment template
├── LICENSE.md                     # AGPL-3.0 license
├── README.md                      # Main project README
└── ECOSYSTEM_README.md            # Comprehensive ecosystem guide
```

### Key Directories Explained

#### `bonsai-workspace/src/`
**Frontend Svelte components** (Vue-like, reactive UI):

```
src/
├── App.svelte                    # Root component
├── Editor.svelte                 # Monaco editor wrapper
├── FileTree.svelte               # File navigation
├── Terminal.svelte               # PTY terminal
├── ActivityLog.svelte            # Event stream display
├── Assistant.svelte              # Chat interface
├── ToolCall.svelte               # Tool execution UI
├── Trainer.svelte                # Training dashboard
├── ModelBuilder.svelte           # KDB module builder
├── Collaboration.svelte          # Share & sync UI
├── MobileViewer.svelte          # Screen mirroring
├── BonsaiBot.svelte             # Bot configuration
├── Settings.svelte               # Preferences panel
├── lib/                          # Utility modules
│   ├── stores.js                 # Global state (Svelte stores)
│   ├── api.js                    # Tauri command wrapper
│   ├── models.js                 # Model management
│   ├── utils.js                  # Helper functions
│   └── theme.js                  # Theming system
└── components/                   # Reusable UI components
    ├── Button.svelte
    ├── Modal.svelte
    ├── Input.svelte
    ├── Tree.svelte
    └── ...
```

**Key Stores** (Global State):
- `editorStore` - Current file, cursor position
- `filesStore` - File tree structure
- `assistantStore` - Chat history, settings
- `modelStore` - Active model, loaded models
- `settingsStore` - User preferences
- `activityStore` - Event log entries

#### `bonsai-workspace/src-tauri/src/`
**Rust backend** (Tauri commands, OS interaction):

```
src/
├── main.rs                       # Application entry point
├── state.rs                      # Global state management
├── commands/                     # Tauri command handlers
│   ├── editor.rs                 # File operations
│   ├── assistant.rs              # Tool execution
│   ├── model.rs                  # Model management
│   ├── training.rs               # Training control
│   ├── collaboration.rs          # Sharing & sync
│   ├── mobile.rs                 # Android integration
│   └── bot.rs                    # BonsaiBot API
├── services/                     # Business logic
│   ├── editor.rs                 # File editing (LSP)
│   ├── llm.rs                    # LLM sidecar control
│   ├── tools.rs                  # Tool registry & execution
│   ├── training.rs               # Training orchestration
│   ├── knowledge.rs              # KDB operations
│   ├── transfer.rs               # TransferDaemon interface
│   └── watchdog.rs               # Health monitoring
├── models/                       # Data structures
│   ├── file.rs                   # File metadata
│   ├── chat.rs                   # Message types
│   ├── tool.rs                   # Tool definitions
│   ├── model.rs                  # Model configuration
│   └── settings.rs               # User settings
├── utils/                        # Helper utilities
│   ├── logger.rs                 # Logging setup
│   ├── errors.rs                 # Error handling
│   ├── paths.rs                  # Data directory resolution
│   └── crypto.rs                 # Encryption utilities
└── Cargo.toml                    # Dependencies
```

#### `crates/` Shared Libraries

Each crate is a reusable Rust library:

**Core Libraries**:
- `ai-advisor` - AI-optional architecture trait
- `bonsai-transfer` - P2P networking protocol
- `bonsai-trainer` - Model training engine
- `bonsai-kdb` - Knowledge database implementation
- `bonsai-fabric` - Distributed computing

**Supporting Libraries**:
- `bonsai-crypto` - Encryption & hashing
- `bonsai-sandbox` - Process isolation
- `bonsai-watchdog` - Health monitoring
- `bonsai-survival` - Auto-recovery

**Phase 1 Sovereignty**:
- `bonsai-error` - Custom error handling
- `bonsai-log` - Logging framework
- `bonsai-rng` - Random number generation
- `bonsai-uuid` - Unique ID generation

#### `docs/` Documentation

Each numbered doc covers one major system:

- **00-OVERVIEW.md** - Philosophy, vision, architecture
- **01-GETTING-STARTED.md** - Installation walkthrough
- **02-CORE-IDE.md** - Editor features & shortcuts
- **03-BONSAI-ASSISTANT.md** - Chat, tools, TrustGuard
- **04-MODEL-TRAINER.md** - DPO, distillation, training
- **05-SURVIVAL-SYSTEM.md** - Watchdog, auto-recovery
- **06-KNOWLEDGE-DATABASE.md** - RAG, KDB format, indexing
- **07-COLLABORATION.md** - TransferDaemon, sharing
- **08-COMPUTE-FABRIC.md** - Distributed compute
- **09-MOBILE.md** - Android apps, USB Lab
- **10-SOVEREIGNTY.md** - USOS phases, roadmap
- **11-SECURITY.md** - Encryption, threats, audit
- **12-DEVELOPER.md** - Build, contribute, testing
- **13-TROUBLESHOOTING.md** - Common issues & fixes
- **14-GLOSSARY.md** - All terminology defined

---

## Component Architecture

### 1. Frontend (Svelte/Tauri)

**Responsibilities**:
- Render UI
- Capture user input
- Display real-time data
- Call Tauri commands

**Key Components**:

```svelte
// bonsai-workspace/src/Assistant.svelte
<script>
  import { assistantStore, settingsStore } from './lib/stores.js';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let message = '';
  
  async function sendMessage() {
    const response = await invoke('assistant_send_message', {
      message,
      tools_enabled: true,
    });
    assistantStore.addMessage(response);
    message = '';
  }
</script>

<div class="chat">
  {#each $assistantStore.messages as msg}
    <div class="message">
      {msg.content}
      {#if msg.tool_calls}
        <ToolCall calls={msg.tool_calls} />
      {/if}
    </div>
  {/each}
  
  <input bind:value={message} on:keydown={(e) => e.key === 'Enter' && sendMessage()} />
  <button on:click={sendMessage}>Send</button>
</div>
```

**Architecture**:
```
User Input (Keyboard/Click)
  ↓
Svelte Component (Event Handler)
  ↓
Store Update ($assistantStore.addMessage())
  ↓
Tauri Command (invoke('assistant_send_message'))
  ↓
Rust Backend (Tauri Command Handler)
  ↓
Service Logic (Tool Execution)
  ↓
Response Back to Frontend
  ↓
Store Update
  ↓
UI Re-render (Svelte Reactivity)
```

### 2. Tauri Backend (Rust)

**Responsibilities**:
- Handle file system operations
- Execute tools (shell, read/write)
- Manage LLM sidecars
- Interface with external services

**Example Command Handler**:

```rust
// src-tauri/src/commands/assistant.rs
use tauri::State;
use crate::services::LlmService;

#[tauri::command]
pub async fn assistant_send_message(
    message: String,
    tools_enabled: bool,
    llm_state: State<'_, LlmService>,
) -> Result<AssistantResponse, String> {
    // 1. Add message to history
    let chat_history = llm_state.get_history();
    
    // 2. Call LLM (with streaming)
    let response = llm_state
        .call_llm(&message, &chat_history, tools_enabled)
        .await?;
    
    // 3. Execute tool calls if present
    if let Some(tool_calls) = &response.tool_calls {
        for call in tool_calls {
            llm_state.execute_tool(call).await?;
        }
    }
    
    // 4. Return response
    Ok(response)
}
```

### 3. LLM Sidecar Integration

**Sidecar Architecture**:
```
Tauri App (src-tauri/src/llm.rs)
  ↓
Spawn Process: llama-server
  ↓
HTTP JSON-RPC on localhost:8080
  ↓
POST /completion with {"prompt": "...", "stream": true}
  ↓
Streaming responses (SSE format)
  ↓
Parse token stream
  ↓
Send to UI via WebSocket
  ↓
Display in real-time
```

**Code**:
```rust
// src-tauri/src/services/llm.rs
pub struct LlmService {
    sidecar: Child,
    url: String,
}

impl LlmService {
    pub async fn call_llm(
        &self,
        prompt: &str,
        stream: bool,
    ) -> Result<LlmResponse> {
        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/completion", self.url))
            .json(&serde_json::json!({
                "prompt": prompt,
                "stream": stream,
            }))
            .send()
            .await?;
        
        // Parse streaming response
        if stream {
            self.handle_stream(response).await
        } else {
            response.json().await.map_err(|e| e.into())
        }
    }
}
```

### 4. TransferDaemon (P2P Networking)

**Protocol Flow**:
```
Peer A (Workspace)          Peer B (Workspace)
    ↓                           ↓
  Create offer              Listen for connection
    ↓                           ↓
  WebRTC/QUIC ←─────────→  Accept connection
    ↓                           ↓
  Noise_XX handshake        Noise_XX handshake
    ↓                           ↓
  Encrypted messages ←────→ Encrypted messages
    ↓                           ↓
  Sync files                Send deltas (CRDT)
  Collaborate               Receive updates
  Share compute
```

**Implementation**:
```rust
// crates/bonsai-transfer/src/lib.rs
pub struct TransferDaemon {
    transport: Transport,  // WebRTC, QUIC, Tor, Bluetooth
    encryption: NoiseXX,
    peers: HashMap<PeerId, Connection>,
}

impl TransferDaemon {
    pub async fn send_message(
        &mut self,
        peer: PeerId,
        message: Vec<u8>,
    ) -> Result<()> {
        let encrypted = self.encryption.encrypt(&message)?;
        self.transport.send(peer, encrypted).await
    }
}
```

### 5. Model Training Pipeline

**Training Flow**:
```
User Dataset (JSONL)
  ↓
Data Preprocessing (split, tokenize)
  ↓
DPO Training Loop (Direct Preference Optimization)
  ├─ Forward pass
  ├─ Compute preference loss
  ├─ Backward pass
  └─ Update weights
  ↓
Evaluation on test set
  ↓
Merge with base model (LoRA → full weights)
  ↓
Save trained model
  ↓
Load in Model Selector
```

**Code**:
```rust
// crates/bonsai-trainer/src/lib.rs
pub struct Trainer {
    model: LlmModel,
    config: TrainerConfig,
}

impl Trainer {
    pub async fn train_dpo(
        &mut self,
        dataset: &[PreferenceExample],
    ) -> Result<TrainingMetrics> {
        for epoch in 0..self.config.num_epochs {
            for batch in dataset.chunks(self.config.batch_size) {
                // Compute loss (preference pairs)
                let loss = self.dpo_loss(batch)?;
                
                // Backward pass
                loss.backward()?;
                
                // Update weights
                self.optimizer.step()?;
                self.optimizer.zero_grad()?;
                
                // Log metrics
                self.metrics.record_batch(loss)?;
            }
        }
        Ok(self.metrics.clone())
    }
}
```

### 6. Knowledge Database (RAG)

**KDB Architecture**:
```
User Input: "How do I use async/await?"
  ↓
Vector Embedding (sentence-transformers)
  ↓
Search in .kmod index (cosine similarity)
  ↓
Retrieve top-k passages (e.g., from Rust docs)
  ↓
Inject into context window:
  "Question: How do I use async/await?
   
   Context from knowledge modules:
   - async/await basics (Rust doc excerpt 1)
   - async/await patterns (Rust doc excerpt 2)
   
   Answer:"
  ↓
LLM generates answer using context
  ↓
Return enhanced response
```

**Implementation**:
```rust
// crates/bonsai-kdb/src/lib.rs
pub struct KnowledgeDb {
    index: VectorIndex,
    storage: FileStorage,
}

impl KnowledgeDb {
    pub async fn search(
        &self,
        query: &str,
        k: usize,
    ) -> Result<Vec<Passage>> {
        // Embed query
        let embedding = self.embed(query).await?;
        
        // Search index
        let results = self.index.search(embedding, k)?;
        
        // Load passages from storage
        let passages = results
            .iter()
            .map(|r| self.storage.get(r.passage_id))
            .collect::<Result<Vec<_>>>()?;
        
        Ok(passages)
    }
}
```

---

## Data Flow Architecture

### Chat Message Flow

```
User types message in Assistant.svelte
  │
  └─> Store.addMessage()
       │
       ├─> Update UI immediately (optimistic)
       │
       └─> Invoke Tauri command
            │
            └─> assistant_send_message() in Rust
                 │
                 ├─> Add to history
                 │
                 ├─> Call LLM sidecar (llama-server)
                 │   └─> Stream tokens over HTTP
                 │
                 ├─> Stream tokens back to frontend (WebSocket)
                 │   └─> Display in real-time
                 │
                 ├─> Check for tool calls in response
                 │
                 ├─> Execute tools (if approved)
                 │   └─> Tool results sent to LLM
                 │       └─> Generate follow-up response
                 │
                 └─> Save to assistant_store
                      │
                      └─> Update UI with full response
```

### File Edit Flow

```
User types in Editor.svelte
  │
  └─> Throttle (100ms debounce)
       │
       └─> Store.setFileContent()
            │
            └─> Invoke Tauri command
                 │
                 └─> editor_write_file() in Rust
                      │
                      ├─> Write to disk
                      │
                      ├─> Emit file_changed event
                      │
                      └─> If collaborating:
                           │
                           └─> Send CRDT delta to peers
                                │
                                └─> Via TransferDaemon
                                     │
                                     └─> Peers apply delta (conflict-free)
```

### Model Loading Flow

```
User clicks "Load Model" in Model Selector
  │
  └─> model_load_model() command
       │
       └─> Check if already loaded
            │
            ├─> Yes: Return current (no-op)
            │
            └─> No: Start sidecar
                 │
                 ├─> Spawn llama-server process
                 │
                 ├─> Wait for HTTP port (polling)
                 │
                 ├─> Load model weights from ~/.bonsai/models/
                 │   (first-time load: 30-60s for 7B model)
                 │
                 ├─> Test inference (ping)
                 │
                 └─> Update modelStore
                      │
                      └─> UI shows "Ready"
```

---

## Code Organization

### Naming Conventions

**Files**:
- Svelte: `PascalCase.svelte` (components)
- Rust: `snake_case.rs` (modules)
- JS/TS: `camelCase.js` (utilities)

**Variables**:
- `let message: String` - immutable by default
- `let mut count: i32 = 0` - mutable explicitly
- `const MAX_RETRIES: u32 = 3` - constants all caps

**Functions**:
- Rust command: `async fn assistant_send_message()`
- Service method: `pub async fn call_llm()`
- Svelte handler: `const handleSendMessage = ()`

### Code Organization by Responsibility

```
Commands (Tauri entry points)
  └─ Handle parameters, validation
  └─ Delegate to Services

Services (Business logic)
  └─ Orchestrate operations
  └─ Call Models for data access
  └─ Coordinate tools

Models (Data structures)
  └─ Define types
  └─ Serialize/deserialize
  └─ Validation rules

Utils (Reusable helpers)
  └─ Formatting
  └─ Cryptography
  └─ Path resolution
  └─ Error conversion
```

### Example: Adding a New Feature

**Requirement**: "Save chat history to file"

**1. Add Model** (`src-tauri/src/models/chat.rs`):
```rust
#[derive(Serialize, Deserialize)]
pub struct SavedChat {
    pub id: String,
    pub title: String,
    pub messages: Vec<Message>,
    pub created: DateTime<Utc>,
}
```

**2. Add Command** (`src-tauri/src/commands/assistant.rs`):
```rust
#[tauri::command]
pub async fn assistant_save_chat(
    chat_id: String,
    title: String,
) -> Result<(), String> {
    let service = AssistantService::new()?;
    service.save_chat(&chat_id, &title).await?;
    Ok(())
}
```

**3. Add Service** (`src-tauri/src/services/assistant.rs`):
```rust
impl AssistantService {
    pub async fn save_chat(
        &self,
        chat_id: &str,
        title: &str,
    ) -> Result<()> {
        let chat = self.get_chat(chat_id)?;
        let saved = SavedChat {
            id: chat_id.to_string(),
            title: title.to_string(),
            messages: chat.messages,
            created: Utc::now(),
        };
        
        let path = self.get_chat_path(chat_id);
        let json = serde_json::to_string(&saved)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }
}
```

**4. Add Svelte UI** (`bonsai-workspace/src/Assistant.svelte`):
```svelte
<script>
  async function saveChat() {
    await invoke('assistant_save_chat', {
      chat_id: $assistantStore.current_id,
      title: promptForTitle(),
    });
  }
</script>

<button on:click={saveChat}>Save Chat</button>
```

---

## Module Dependencies

### Dependency Graph

```
bonsai-workspace (Tauri app)
  ├─ depends on: tauri, serde, tokio
  ├─ calls: llama-server (subprocess)
  └─ links: all crates/ libraries

bonsai-bot (Messaging bot)
  ├─ depends on: tokio, reqwest, serde
  └─ links: bonsai-transfer, bonsai-trainer

android-runtime (Mobile app)
  ├─ Android SDK
  └─ Kotlin/Java dependencies

crates/ai-advisor
  └─ No external dependencies (by design)

crates/bonsai-transfer
  ├─ depends on: tokio, serde, libp2p
  ├─ optional: webrtc-rs, quinn (QUIC)
  └─ optional: arti (Tor)

crates/bonsai-trainer
  ├─ depends on: tch-rs (PyTorch bindings)
  ├─ depends on: tokenizers
  └─ depends on: ndarray

crates/bonsai-kdb
  ├─ depends on: serde, tokio
  └─ depends on: ort (ONNX Runtime for embeddings)

crates/bonsai-crypto
  ├─ depends on: argon2, aes-gcm
  ├─ depends on: curve25519-dalek
  └─ no external random (uses bonsai-rng)

USOS Phase 1 Crates:
  ├─ bonsai-error (custom error types)
  ├─ bonsai-log (logging framework)
  ├─ bonsai-rng (random numbers)
  └─ bonsai-uuid (IDs)
```

### Circular Dependency Prevention

**Rule**: Services depend on Models, never vice versa.

```
❌ BAD:
models/chat.rs imports from services/assistant.rs
(models shouldn't know about services)

✅ GOOD:
services/assistant.rs imports from models/chat.rs
(services use data models)
```

---

## Key Algorithms & Patterns

### 1. Graceful Degradation

**Problem**: What if LLM is unavailable?

**Solution**: Fallback chain:
```rust
let response = match call_llm().await {
    Ok(resp) => resp,
    Err(_) => {
        // Fallback 1: Use cached response
        if let Some(cached) = cache.get(&message) {
            return cached;
        }
        
        // Fallback 2: Use template response
        use_template_response(&message)
    }
};
```

### 2. Real-Time Sync (CRDT)

**Algorithm**: Conflict-free Replicated Data Type
```
Peer A: [1,2,3]  insert "4"  → [1,2,3,4]
Peer B: [1,2,3]  insert "5"  → [1,2,3,5]

Network:
  A sends: "insert 4 at position 3"
  B sends: "insert 5 at position 3"

CRDT resolution:
  Use operation IDs (A.1, B.1) to order
  Result on both: [1,2,3,4,5] (deterministic)
```

### 3. Work Distribution (Compute Fabric)

**Algorithm**: Load-balanced task distribution
```rust
let workers = get_available_workers();  // [W1, W2, W3]
let tasks = split_job(&job, workers.len());

for (worker, task) in workers.zip(tasks) {
    send_job_to_worker(worker, task).await?;
}

// Collect results as they arrive (not strictly in order)
let mut results = Vec::new();
for result in collect_results(workers).await {
    results.push(result);
}
```

### 4. Knowledge Database Search

**Algorithm**: Vector similarity search
```
Query: "How to use async/await?"
  ↓
Embed: [-0.5, 0.2, 0.8, ...]  (768 dimensions)
  ↓
Search index: cosine_similarity(query_embedding, doc_embeddings)
  ↓
Top 3 matches:
  - "async/await basics" (0.92 similarity)
  - "async patterns" (0.88 similarity)
  - "futures library" (0.85 similarity)
  ↓
Inject into context, LLM uses to answer
```

### 5. Tool Approval & Sandboxing

**Flow**:
```
User:   "ls /tmp"
  ↓
LLM generates: ToolCall { name: "shell", args: ["ls /tmp"] }
  ↓
TrustGuard:
  ├─ User has "shell" permission? Yes
  ├─ Is path allowed (/tmp)? Yes
  └─ Require approval? (depends on settings)
  ↓
Sandbox:
  ├─ Create isolated process
  ├─ Set resource limits (CPU, RAM, time)
  ├─ Execute: sh -c "ls /tmp"
  ├─ Capture stdout/stderr
  └─ Kill process (timeout: 30s)
  ↓
Result: "file1\nfile2\nfile3\n"
  ↓
LLM uses result in next response
```

---

## Build Architecture

### Tauri Build Pipeline

```
Source Code (Svelte + Rust)
  ↓
[Frontend Build]
  ├─ npm run build (Svelte → JS/CSS)
  ├─ Bundle with Vite
  └─ Output: dist/

  [Rust Build]
  ├─ cargo build --release (Tauri backend)
  ├─ Link binaries
  └─ Output: src-tauri/target/release/

  [Sidecar Binaries]
  ├─ Download llama-server (platform-specific)
  ├─ Copy to: src-tauri/binaries/
  └─ Embedded in final executable

[Tauri Bundle]
  ├─ Embed frontend dist/
  ├─ Link Rust backend
  ├─ Embed sidecars
  └─ Code sign (Windows: cert-signed, macOS: Apple cert)

[Output Artifact]
  ├─ Windows: MSI installer + portable EXE
  ├─ macOS: DMG + ZIP
  └─ Linux: AppImage + deb + rpm
```

**Build commands**:
```bash
# Dev mode (hot reload)
cd bonsai-workspace
npx tauri dev

# Production build
npx tauri build --release

# Custom target
npx tauri build --target x86_64-pc-windows-msvc
```

### Cargo Workspace Build

```
Workspace Root (Cargo.toml)
  ├─ members: [
  │   "bonsai-workspace/src-tauri",
  │   "bonsai-bot",
  │   "crates/bonsai-*",
  │ ]
  │
  └─ cargo build --release
       ├─ bonsai-error (Phase 1)
       ├─ bonsai-log (Phase 1)
       ├─ bonsai-rng (Phase 1)
       ├─ bonsai-uuid (Phase 1)
       ├─ bonsai-crypto (depends on above)
       ├─ bonsai-transfer
       ├─ bonsai-trainer
       ├─ bonsai-kdb
       ├─ ... (all crates)
       └─ src-tauri (depends on all crates)
```

---

## Testing Architecture

### Test Pyramid

```
        /\
       /  \
      / E2E \          [UI tests in Playwright/Cypress]
     /______\
      /    \
     / Intg \         [Component tests in Svelte Testing Library]
    /________\
     /      \
    / Unit   \        [Rust unit tests in each crate]
   /_________ \
```

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_llm_response_parsing() {
        let response = r#"{"content": "Hello", "stop_reason": "stop"}"#;
        let parsed = parse_response(response).unwrap();
        assert_eq!(parsed.content, "Hello");
    }
    
    #[tokio::test]
    async fn test_tool_execution() {
        let result = execute_tool("shell", &["echo hello"]).await.unwrap();
        assert!(result.contains("hello"));
    }
}
```

**Run tests**:
```bash
cargo test --all                    # All tests
cargo test -p bonsai-trainer        # Specific crate
cargo test test_llm_response        # Specific test
cargo test -- --nocapture          # Show println output
```

### Integration Tests

**Example**: Test assistant message flow
```rust
#[tokio::test]
async fn test_assistant_message_with_tool_call() {
    // Setup
    let app = create_test_app().await;
    let initial_messages = app.get_messages().await;
    
    // Action
    let response = app
        .send_message("What files are in /tmp?")
        .await;
    
    // Assert
    assert!(response.contains("files"));
    assert!(app.tool_was_called("shell"));
}
```

### End-to-End Tests (UI)

```javascript
// tests/e2e/chat.test.ts
test('User can chat with assistant', async ({ page }) => {
    // Navigate
    await page.goto('http://localhost:8080');
    
    // Type message
    await page.fill('[data-testid="chat-input"]', 'Hello');
    await page.click('[data-testid="send-button"]');
    
    // Wait for response
    await page.waitForSelector('[data-testid="message"]', {
        hasText: /Hello|Hi|Greetings/
    });
    
    // Verify
    const message = await page.textContent('[data-testid="message"]:last-of-type');
    expect(message).toBeTruthy();
});
```

---

## Deployment Architecture

### Desktop Deployment

```
GitHub Release
  ├─ Windows:
  │  ├─ MSI installer (auto-update via Tauri updater)
  │  └─ Portable EXE (no installer required)
  │
  ├─ macOS:
  │  ├─ DMG (signed & notarized by Apple)
  │  └─ ZIP (for CI/CD)
  │
  └─ Linux:
     ├─ AppImage (single-file, no dependencies)
     ├─ deb (for Ubuntu/Debian)
     └─ rpm (for Fedora/RHEL)
```

**Update mechanism**:
```
User has v1.0
  ↓
Check GitHub for v1.1
  ↓
Download delta (only changed files)
  ↓
Verify signature (prevent tampering)
  ↓
Install update
  ↓
Restart app
  ↓
User has v1.1
```

### Docker Deployment

```
Dockerfile
  ├─ Base: rust:latest (for building)
  ├─ Install: Node, Tauri deps, llama-server
  ├─ Copy: source code
  ├─ Run: cargo build --release
  ├─ Run: npm install && npm run build
  └─ Expose: port 8080 (web server mode)

docker run -d \
  -v ~/.bonsai:/home/user/.bonsai \
  -p 8080:8080 \
  bonsai-workspace:latest
```

### Mobile Deployment (Android)

```
android-runtime/
  ├─ Build APK (Android Studio)
  ├─ Sign with keystore
  └─ Upload to Google Play

Release Process:
  1. Increment version
  2. Build signed APK
  3. Upload to Play Store
  4. Users auto-update (or manual check)
```

---

## Summary: Code Flow Example

**User asks**: "List files in current directory"

**Complete execution path**:

```
1. UI Layer (Assistant.svelte)
   - User types message
   - handleSendMessage() invoked
   - Call invoke('assistant_send_message', {message})

2. Tauri Command Handler (src-tauri/src/commands/assistant.rs)
   - #[tauri::command] fn assistant_send_message()
   - Add message to history
   - Delegate to AssistantService

3. Service Layer (src-tauri/src/services/assistant.rs)
   - Format prompt (include conversation history)
   - Call LLM via llama-server HTTP API
   - Stream tokens to UI via WebSocket

4. LLM Sidecar (llama-server subprocess)
   - Process prompt tokens
   - Generate response tokens
   - Return streamed response

5. Response Processing (Service)
   - Parse token stream
   - Detect tool calls in response
   - Identify: ToolCall {name: "shell", args: ["ls"]}

6. Tool Execution (TrustGuard)
   - Check if user granted "shell" permission
   - Create sandbox process
   - Execute: sh -c "ls"
   - Capture output
   - Kill process (timeout: 30s)

7. Result Handling (Service)
   - Add tool result to context
   - Call LLM again: "Given the file listing, explain..."
   - Stream follow-up response

8. Frontend Update
   - WebSocket receives tokens
   - Svelte store updates
   - UI re-renders in real-time
   - User sees: "Current directory contains: file1.txt, file2.txt, ..."

Total latency: 0.5-2 seconds (depends on LLM speed)
```

---

**This guide provides complete understanding of every architectural layer and code component in the Bonsai Ecosystem.**

For specific implementation details, refer to the numbered documentation files (docs/NN-*.md) and inline code comments.

**Version**: 2.0 | **Last Updated**: 2026-06-04 | **Audience**: All developers & architects
