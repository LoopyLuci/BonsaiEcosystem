# 🌳 Bonsai Ecosystem: Comprehensive System Documentation

**Version**: 2.0  
**Last Updated**: 2026-06-04  
**Status**: 🟢 Production-Ready

---

## Table of Contents

1. [Executive Overview](#executive-overview)
2. [Core Philosophy](#core-philosophy)
3. [System Architecture](#system-architecture)
4. [Feature Catalog](#feature-catalog)
5. [Component Deep Dives](#component-deep-dives)
6. [Installation & Setup](#installation--setup)
7. [Quick Start Guides](#quick-start-guides)
8. [Advanced Topics](#advanced-topics)
9. [Troubleshooting](#troubleshooting)
10. [Contributing](#contributing)

---

## Executive Overview

### What is the Bonsai Ecosystem?

The **Bonsai Ecosystem** is a comprehensive, sovereignty-first computing platform designed to give users complete control over their development environment, AI assistants, and computing infrastructure. It consists of multiple interconnected systems working together:

1. **Bonsai Workspace** - Desktop IDE with integrated AI assistant
2. **BonsaiBot** - Multi-platform messaging bot (Discord, Telegram, Email, Matrix)
3. **Bonsai Buddy** - Mobile AI assistant for Android
4. **Bonsai Network** - Peer-to-peer collaboration and data sync
5. **USOS** (Unnamed Sovereign Operating System) - Long-term vision for full OS replacement

### Core Value Propositions

| Aspect | Bonsai | Cloud Alternatives |
|--------|--------|-------------------|
| **Privacy** | 100% local, no cloud calls | Cloud-dependent, data leaves device |
| **Sovereignty** | Full source audit trail | Proprietary, hidden APIs |
| **Offline Operation** | Works completely offline | Requires cloud connectivity |
| **Customization** | Swap models at runtime | Fixed model choices |
| **Cost** | One-time setup | Ongoing subscription |
| **Resilience** | Self-healing with Watchdog | Cloud outage = service down |

### Target Users

- **Developers** needing privacy-first AI assistance
- **Organizations** with data sovereignty requirements
- **Researchers** studying deterministic and verifiable AI
- **System builders** creating custom computing platforms

---

## Core Philosophy

### Three Pillars

#### 1. **Sovereignty by Design**

**Goal**: Zero external dependencies for core functionality.

A complete replacement strategy (USOS phases 0-6) is underway to replace every dependency with custom bonsai-* crates:

- ✅ **Phase 0** (Complete): Vendor all dependencies, offline CI
- 🔄 **Phase 1** (In Progress): Error handling, logging, RNG, IDs
- 📋 **Phase 2-6** (Planned): Crypto, databases, inference, git, async runtime

**Benefit**: Complete auditability, zero supply chain attack surface, maximum control.

#### 2. **Privacy by Default**

**Goal**: No data leaves your machine without explicit action.

**Guarantees**:
- Zero telemetry
- Zero cloud API calls (unless you enable them)
- No tracking
- All models run locally
- Network: peer-to-peer only

**Verification**: Use `tcpdump` or Wireshark on a typical usage day with no connections enabled; you'll see zero outbound network requests.

#### 3. **Resilience & Self-Healing**

**Goal**: System runs forever without human intervention.

**Components**:
- **Watchdog**: Monitors process health 24/7
- **Survival Engine**: Automatically diagnoses and fixes crashes
- **Knowledge Base**: Library of pre-tested fixes for common failures
- **BonsAI Self-Healing**: Uses AI to generate novel repairs when needed

**SLA**: Target 99.99% uptime in production mode.

---

## System Architecture

### High-Level System Diagram

```
┌──────────────────────────────────────────────────────────┐
│                   User Applications                      │
│  IDE | Mobile Apps | CLI Tools | Browser Extensions     │
└──────────────────┬───────────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────────┐
│              BonsAI Computing Layer                      │
│  ┌──────────────────┐  ┌──────────────┐                 │
│  │ Local LLM Models │  │ Model Trainer│  Orchestrator   │
│  │ (Llama, Mistral) │  │ (DPO, RLHF)  │  & Coordinator  │
│  └──────────────────┘  └──────────────┘                 │
└──────────────────┬───────────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────────┐
│           Knowledge & Memory Systems                     │
│  ┌──────────────────┐  ┌──────────────┐                 │
│  │ Knowledge DB     │  │ Context Cache│  RAG Pipeline   │
│  │ (*.kmod files)   │  │ (Vector store)│                 │
│  └──────────────────┘  └──────────────┘                 │
└──────────────────┬───────────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────────┐
│          Networking & Collaboration                      │
│  ┌──────────────────┐  ┌──────────────┐                 │
│  │ TransferDaemon   │  │ Compute      │  Sync Engine    │
│  │ (P2P Network)    │  │ Fabric       │  & Replication  │
│  └──────────────────┘  └──────────────┘                 │
└──────────────────┬───────────────────────────────────────┘
                   │
┌──────────────────▼───────────────────────────────────────┐
│        Operating System & Hardware Abstraction           │
│  Watchdog | Survival Engine | Hardware Discovery        │
│           USOS (Long-term target)                       │
└──────────────────────────────────────────────────────────┘
```

### Component Relationships

```
Bonsai Workspace (Desktop IDE)
├── Uses: BonsAI Assistant + Tool Calling
├── Uses: Knowledge Database for context
├── Uses: TransferDaemon for collaboration
├── Uses: Compute Fabric for distributed work
├── Runs: On-device model sidecars
└── Manages: Training pipeline & Model orchestration

BonsaiBot (Messaging Bot Server)
├── Admin API: Managed by Workspace
├── Platforms: Discord, Telegram, Email, Matrix
├── Uses: Shared BonsAI core
└── Can: Orchestrate workspace tasks

Bonsai Buddy (Mobile App)
├── Uses: Lightweight BonsAI inference
├── Syncs: Via TransferDaemon
├── Calls: BonsaiBot for extended functionality
└── Stores: Encrypted data locally

Compute Fabric
├── Coordinator: Runs on main device
├── Workers: Android, other devices
├── Transport: TransferDaemon (P2P)
└── Scheduling: Work distribution & result aggregation

Knowledge Database
├── Storage: Encrypted .kmod files
├── Indexing: Vector embeddings
├── Sync: Via TransferDaemon
└── Usage: RAG context injection
```

---

## Feature Catalog

### Workspace IDE Features

#### **Editor & File Management**
- **Monaco Editor**: Full IDE experience with language support
  - Syntax highlighting for 200+ languages
  - Inline completions (AI-powered)
  - Diff view with hunk apply/reject
  - Multi-cursor editing
  - Keyboard shortcuts (VSCode-compatible)

- **File Tree**:
  - Quick file create/delete
  - Search & filter
  - Context menus
  - Drag-and-drop
  - Quick open (Ctrl+P)

- **Per-Language Tooling**:
  - Format (prettier, rustfmt, black, gofmt, etc.)
  - Lint (eslint, clippy, pylint, golangci-lint, etc.)
  - Test (jest, pytest, cargo test, go test, etc.)
  - Run (custom scripts)
  - Compile (gcc, cargo, rustc, etc.)

#### **BonsAI Assistant**
- **Chat Interface**:
  - Real-time streaming responses
  - Conversation history
  - Session management
  - Export conversations

- **Tool Calling**:
  - Read/write files
  - Execute shell commands (with approval)
  - Call external APIs
  - Analyze code
  - Generate fixes
  - All calls logged & auditable

- **TrustGuard Capability System**:
  - Granular permissions (file access, shell, network, etc.)
  - Per-command approval
  - Session-based grants
  - Audit logging of all actions

- **Bonsai Buddy**:
  - Always-on-top assistant window
  - Detachable from main workspace
  - Voice chat support
  - Quick actions (summarize, explain, fix)

#### **Model Management**
- **Model Selector**: Choose from local library
- **Sidecar Management**: Auto-start/stop llama-server, whisper, TTS
- **Model Profiles**: Save configurations (temperature, top-p, etc.)
- **Fallback Behavior**: Graceful degradation if no model running

#### **Training Pipeline**
- **Safety Training**: DPO to refuse harmful requests
- **Tool Use Training**: Learn to call tools correctly
- **Distillation**: Learn from larger teacher models
- **Quick Updates**: Incremental training without full retraining
- **Brain Age Tracking**: Monitor model evolution

#### **Terminal & Shell**
- **Multi-Tab PTY**: Multiple shell sessions
- **Activity Log**: Real-time event streaming
  - Tool calls & traces
  - Errors & warnings
  - User actions
  - System diagnostics
- **Command Palette**: Quick command execution

### Network & Collaboration Features

#### **TransferDaemon v2**
The networking backbone of Bonsai:

**Capabilities**:
- **Multi-Transport**: WebRTC, libp2p/QUIC, Tor, Bluetooth
- **Smart Selection**: Automatically chooses best lane
- **NAT Traversal**: Works behind NAT/firewalls
- **End-to-End Encryption**: Noise_XX protocol
- **Peer Discovery**: mDNS, DHT, manual pairing
- **Message Streaming**: Progressive delivery of results

**Use Cases**:
- Share code in real-time (like VS Code Live Share)
- Sync files across devices
- Collaborate with teammates
- Distribute compute jobs
- Backup data securely

#### **Compute Fabric**
Distributed computing across personal devices:

**Components**:
- **Coordinator**: Runs on main device
  - Splits large jobs
  - Distributes to workers
  - Aggregates results
  - Monitors health

- **Worker**: Runs on any device
  - Receives jobs via TransferDaemon
  - Executes work
  - Streams partial results
  - Reports completion

**Example**: Training a model across desktop + laptop + Android phone.

### Security & Hardening

#### **Encryption**
- **At Rest**: Argon2id + AES-256-GCM
- **In Transit**: Noise_XX + per-message signing
- **Key Storage**: OS keyring or encrypted file
- **Model Weights**: Encrypted when not in use

#### **Sandboxing**
- **Process Isolation**: Separate process per untrusted code
- **Resource Limits**: CPU, memory, disk quotas
- **File Access Control**: Whitelist/blacklist paths
- **Network Sandbox**: Restrict external connections

#### **Audit & Logging**
- **Tool Call Audit**: Every action logged
- **Session Recording**: Inputs/outputs can be recorded
- **Event Streaming**: Real-time event export
- **Compliance**: HIPAA/SOC2-ready audit trails

### Bot & Automation

#### **BonsaiBot Server**
Multi-platform messaging bot:

**Platforms**:
- Discord (slash commands, reactions)
- Telegram (inline bots)
- Email (SMTP/IMAP)
- Matrix (direct messages)

**Features**:
- Admin API for orchestration
- Token-based security
- Message templating
- Auto-responders
- Custom workflows
- Rate limiting

#### **Mobile Automation**
- **Android USB Lab**: Device setup & testing
- **Mobile Viewer**: Screen mirroring (scrcpy)
- **Remote Surface**: Web-accessible device control
- **Mobile Automation Server**: Programmable device control

### Advanced Features

#### **Knowledge Database (RAG)**
Instead of retraining for new information:

**Structure**:
- **Knowledge Modules** (*.kmod files)
  - Searchable fact databases
  - Code pattern libraries
  - Documentation snippets
  - Structured data

**Processing**:
1. User asks question
2. Search KDB for relevant passages
3. Inject passages into LLM context
4. Generate answer with current knowledge

**Benefits**:
- Update knowledge without retraining
- Reduce hallucinations
- Lower inference latency
- Customizable per-domain

#### **Chain-of-Thought Reasoning**
For reasoning models:
- Step-by-step internal thinking
- Shows work for verification
- Better accuracy on complex tasks
- Audit trail for understanding

#### **Watchdog & Survival Engine**
Automatic fault recovery:

**Monitoring**:
- Process health checks (heartbeat)
- Memory leaks (RSS growth)
- Disk space (prevent full disk)
- Network connectivity
- Resource exhaustion

**Recovery**:
1. Detect fault
2. Look up in knowledge base
3. Apply known fix (or generate new one)
4. Restart process
5. Verify recovery

---

## Component Deep Dives

### Architecture Overview

Each major component is documented in `docs/NN-*.md`. Here's what's covered:

#### [00-OVERVIEW.md](docs/00-OVERVIEW.md)
- Philosophy & vision
- High-level architecture
- System requirements
- Getting started checklist

#### [01-GETTING-STARTED.md](docs/01-GETTING-STARTED.md)
- Installation steps
- First launch walkthrough
- Initial configuration
- Quick tour of UI

#### [02-CORE-IDE.md](docs/02-CORE-IDE.md)
- Editor controls & shortcuts
- File tree operations
- Terminal & PTY
- Settings & preferences
- Feature flags

**Key Code Files**:
- `bonsai-workspace/src/Editor.svelte` - Main editor component
- `bonsai-workspace/src/FileTree.svelte` - File navigation
- `src-tauri/src/editor.rs` - Tauri file operations

#### [03-BONSAI-ASSISTANT.md](docs/03-BONSAI-ASSISTANT.md)
- Chat interface
- Tool calling mechanism
- TrustGuard permissions
- Buddy window control
- Voice interaction

**Key Code Files**:
- `bonsai-workspace/src/Assistant.svelte` - Chat UI
- `bonsai-workspace/src/ToolCall.svelte` - Tool rendering
- `src-tauri/src/assistant.rs` - Tool execution

#### [04-MODEL-TRAINER.md](docs/04-MODEL-TRAINER.md)
- Training pipeline architecture
- DPO (Direct Preference Optimization)
- Distillation process
- Brain Age tracking
- EternalTrainingLoop

**Key Code Files**:
- `crates/bonsai-trainer/src/lib.rs` - Training engine
- `bonsai-workspace/src/Trainer.svelte` - UI
- `crates/bonsai-dpo/src/lib.rs` - DPO implementation

#### [05-SURVIVAL-SYSTEM.md](docs/05-SURVIVAL-SYSTEM.md)
- Watchdog architecture
- Fault detection
- Automatic recovery
- Knowledge base structure
- Self-healing with BonsAI

**Key Code Files**:
- `crates/bonsai-watchdog/src/lib.rs` - Monitoring
- `crates/bonsai-survival/src/lib.rs` - Recovery engine

#### [06-KNOWLEDGE-DATABASE.md](docs/06-KNOWLEDGE-DATABASE.md)
- Knowledge module format
- Indexing & search
- Vector embeddings
- Model Builder tool
- .bkp package format

**Key Code Files**:
- `crates/bonsai-kdb/src/lib.rs` - Database implementation
- `bonsai-workspace/src/ModelBuilder.svelte` - UI

#### [07-COLLABORATION.md](docs/07-COLLABORATION.md)
- TransferDaemon protocol
- Shared editing
- Voice/video calls
- Permission system
- Conflict resolution

**Key Code Files**:
- `crates/bonsai-transfer/src/lib.rs` - P2P protocol
- `bonsai-workspace/src/Collaboration.svelte` - UI

#### [08-COMPUTE-FABRIC.md](docs/08-COMPUTE-FABRIC.md)
- Coordinator architecture
- Worker management
- Job distribution
- Result aggregation
- Resource scheduling

**Key Code Files**:
- `crates/bonsai-fabric/src/coordinator.rs` - Coordinator
- `crates/bonsai-fabric/src/worker.rs` - Worker

#### [09-MOBILE.md](docs/09-MOBILE.md)
- Bonsai Buddy (Android)
- Bonsai Workspace Mobile
- USB Lab workflow
- Mobile automation
- Scrcpy integration

**Key Code Files**:
- `bonsai-buddy-android/app/src/main/` - Android app
- `bonsai-workspace/src/MobileViewer.svelte` - Screen mirror UI

#### [10-SOVEREIGNTY.md](docs/10-SOVEREIGNTY.md)
- USOS concept & roadmap
- Dependency replacement phases
- bonsai-* crate directory
- Progress tracking
- Contribution guide

**Active Phases**:
- `crates/bonsai-error/` - Error handling (Phase 1)
- `crates/bonsai-log/` - Logging (Phase 1)
- `crates/bonsai-rng/` - RNG (Phase 1)
- `crates/bonsai-uuid/` - ID generation (Phase 1)

#### [11-SECURITY.md](docs/11-SECURITY.md)
- Threat model
- Encryption details
- Sandboxing strategy
- Audit logging
- Compliance (HIPAA, SOC2)

**Key Code Files**:
- `crates/bonsai-crypto/src/lib.rs` - Encryption
- `crates/bonsai-sandbox/src/lib.rs` - Process isolation

#### [12-DEVELOPER.md](docs/12-DEVELOPER.md)
- Build instructions
- Development workflow
- Adding new features
- Testing strategy
- CI/CD pipeline

**Workflows**:
- `./BonsaiExeLauncherBuilder.ps1` - Desktop build
- `.github/workflows/` - CI checks
- `bonsai-workspace/src/orchestrate-bonsai-ecosystem.mjs` - Orchestrates full ecosystem startup (Tauri + bonsai-bot, preflight checks, health validation)

#### [13-TROUBLESHOOTING.md](docs/13-TROUBLESHOOTING.md)
- Common issues & fixes
- Log locations
- Diagnostic tools
- Performance tuning
- Support resources

#### [14-GLOSSARY.md](docs/14-GLOSSARY.md)
- All Bonsai terminology
- Cross-references
- Acronym definitions

---

## Installation & Setup

### System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | 4 cores, 2.4 GHz | 8+ cores, 3.5+ GHz |
| **RAM** | 8 GB | 32 GB |
| **Disk** | 20 GB SSD | 100+ GB SSD |
| **GPU** | Optional | NVIDIA (CUDA) or AMD (ROCm) |
| **OS** | Windows 10, macOS 11, Linux | Windows 11, macOS 12+, Ubuntu 22.04+ |

### Installation Steps

#### **Option 1: Pre-Built Executable (Windows)**
```powershell
# Download from releases page
# Run installer
.\BonsaiWorkspace-Setup-v2.0.exe

# Or quick-start:
.\Launch-BonsaiWorkspace.cmd
```

#### **Option 2: Build from Source**
```powershell
# Clone repo
git clone https://github.com/bonsai/bonsai-workspace.git
cd bonsai-workspace

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node (if needed)
# https://nodejs.org/ - install v18+

# Build
.\BonsaiExeLauncherBuilder.ps1

# Run
.\target\release\BonsaiWorkspace.exe
```

#### **Option 3: Docker (Linux)**
```bash
# Build container
docker build -t bonsai-workspace:latest .

# Run container
docker run -it -v ~/.bonsai:/home/bonsai/.bonsai \
  -p 8080:8080 \
  bonsai-workspace:latest

# Open browser to http://localhost:8080
```

### Post-Installation Setup

1. **Create Data Directory**
   ```
   Windows: %USERPROFILE%\.bonsai\
   Linux:   ~/.bonsai/
   macOS:   ~/Library/Application Support/bonsai/
   ```

2. **Download Models**
   - Visit https://huggingface.co/bonsai-models
   - Download preferred model (GGUF format)
   - Place in `~/.bonsai/models/`

3. **Configure Settings**
   - Open Workspace → Settings
   - Set model path
   - Configure LLM server (port, host)
   - Set up bot secrets (if using)

4. **First Launch**
   - Start Workspace
   - Click Model Selector
   - Choose downloaded model
   - Wait for llama-server to start (~30s)
   - Start chatting!

---

## Quick Start Guides

### Quick Start 1: Desktop IDE

**Objective**: Get IDE working with file editing

```powershell
# Launch workspace
cd bonsai-workspace
npx tauri dev

# Or use launcher
.\Launch-BonsaiWorkspace.cmd
```

**Usage**:
1. File → New File → `test.py`
2. Type code
3. Tools → Format (uses black)
4. Tools → Run (uses python)

### Quick Start 2: AI Chat Assistance

**Objective**: Chat with BonsAI assistant

**Steps**:
1. Model → Model Selector
2. Choose "Mistral-7B"
3. Wait for load (30-60s)
4. Chat panel → Type question
5. Watch response stream in real-time
6. Use tool calls to read/write files

### Quick Start 3: Multi-Agent Workflow

**Objective**: Run multiple AI agents on task

**Setup**:
1. Tools → Agent Studio
2. Create agents: "Coder", "Reviewer", "Tester"
3. Configure each with role/instructions
4. Create workflow: Coder → Reviewer → Tester

**Run**:
1. Click "Run Workflow"
2. Watch agents collaborate
3. Review final output

### Quick Start 4: Mobile AI Buddy

**Objective**: Chat with AI on phone

**Setup**:
1. Mobile → Buddy Setup
2. Install APK from output directory
3. Launch Bonsai Buddy on Android phone
4. Scan QR code from workspace
5. Workspace syncs to phone

**Usage**:
- Chat on phone, context syncs to desktop
- Access offline mode with smaller model
- Voice input/output support

### Quick Start 5: Distributed Compute

**Objective**: Train model across desktop + phone + laptop

**Setup**:
1. Compute → Fabric Setup
2. Add worker devices (scan QR codes)
3. Verify all devices show "Ready"

**Run**:
1. Model Trainer → Start DPO training
2. Fabric auto-distributes batches to workers
3. Monitor progress on dashboard
4. Results aggregated automatically

---

## Advanced Topics

### Fine-Tuning Models

**Complete Workflow**:

1. **Prepare Data**
   - Create `.jsonl` file with {prompt, completion} pairs
   - Tools → Data Prep → Import & validate
   - Workspace auto-splits into train/test sets

2. **Configure Training**
   - Method: DPO (recommended) or SFT (supervised)
   - Learning rate: 1e-5 (DPO) or 1e-4 (SFT)
   - Batch size: 8-16 (depends on RAM)
   - Epochs: 3-5

3. **Monitor Training**
   - Watch loss curve in real-time
   - Brain Age increments
   - Evaluate on test set

4. **Deploy**
   - Export merged model
   - Or save LoRA adapter
   - Use in Model Selector

### Building Knowledge Modules

**Process**:

1. **Create Module**
   ```
   Tools → Model Builder
   New → Type: "Knowledge Module"
   Name: "Python Best Practices"
   ```

2. **Add Content**
   - Import files (code, docs, specs)
   - Or paste text directly
   - Or auto-extract from web

3. **Build Index**
   - Workspace chunks & embeds content
   - Creates searchable vector DB
   - Saves as .kmod file

4. **Use in Chat**
   - Chat → Attach knowledge module
   - Relevant passages auto-injected
   - BonsAI uses module context

### Custom Tool Development

**Adding New Tool**:

1. **Define Interface**
   ```rust
   // src-tauri/src/tools/my_tool.rs
   pub struct MyTool;
   
   impl Tool for MyTool {
       fn name(&self) -> &str { "my_tool" }
       fn call(&self, args: &[&str]) -> Result<String> {
           // Implementation
       }
   }
   ```

2. **Register Tool**
   ```rust
   // src-tauri/src/tools/mod.rs
   mod my_tool;
   
   fn get_tools() -> Vec<Box<dyn Tool>> {
       vec![
           Box::new(my_tool::MyTool),
           // other tools...
       ]
   }
   ```

3. **Test Tool**
   ```bash
   cargo test --package bonsai-workspace
   ```

4. **Use in Chat**
   - Chat → Tool Call → select "my_tool"
   - Or ask assistant: "use my_tool to..."

### Setting Up Email Server

**Production Mail Server** (for BonsaiBot):

See [bonsai-bot/MAIL_SERVER_PROD_PLAN.md](bonsai-bot/MAIL_SERVER_PROD_PLAN.md) for:
- SMTP server setup
- IMAP configuration
- Email bot commands
- Rate limiting
- Bounce handling

### Peer-to-Peer Collaboration

**Share Workspace**:

1. **Start Sharing**
   - Collaboration → Share Workspace
   - Get shareable link (or QR code)
   - Send to colleague

2. **Join**
   - Colleague clicks link or scans QR
   - Workspace syncs in real-time
   - Both see same files & cursor positions

3. **Permissions**
   - Workspace → Settings → Share Permissions
   - Grant: View, Edit, Delete per person
   - Revoke at any time

### Compute Fabric Administration

**Adding Worker**:

1. Install Bonsai on worker device
2. Worker → Join Fabric
3. Scan coordinator QR code
4. Worker registers with coordinator
5. Shows up in Fabric dashboard

**Job Distribution**:

```
Coordinator assigns: { job_id, input, resources_needed }
Worker receives job
Worker: compile, execute, report progress
Coordinator: aggregate partial results
Return to user when done
```

---

## Troubleshooting

### Issue: Model fails to load
**Symptom**: "Model not found" or "incompatible GGUF"

**Solutions**:
1. Check file exists in `~/.bonsai/models/`
2. Verify GGUF format (file command)
3. Check disk space (needs 2x model size free)
4. Restart llama-server: `pkill llama-server`

### Issue: Chat is slow
**Symptom**: Long latency between message and response

**Solutions**:
1. Check GPU usage: `nvidia-smi` (should see process)
2. Reduce context window: Settings → Max Context
3. Disable voice TTS: Settings → Text-to-Speech OFF
4. Use smaller model: Model Selector → smaller variant

### Issue: Mobile sync not working
**Symptom**: Changes on phone don't appear on desktop

**Solutions**:
1. Check network: Both on same WiFi
2. Verify TransferDaemon running: `ps aux | grep transfer`
3. Restart sync: Mobile → Sync → Restart
4. Check firewall: May need to allow port 6001

### Issue: Training crashes with OOM
**Symptom**: "out of memory" during training

**Solutions**:
1. Reduce batch size: Settings → Training → Batch Size 4
2. Reduce context: Settings → Max Context 2048
3. Use CPU instead: Settings → Compute → CPU
4. Use smaller model: DPO data → Sample subset

### Issue: Bot not responding in Discord
**Symptom**: BonsaiBot slash commands don't work

**Solutions**:
1. Check token: Settings → Bots → Discord → Token
2. Verify bot permissions in Discord server settings
3. Restart BonsaiBot: `cd bonsai-bot && cargo run`
4. Check logs: `tail -f ~/.bonsai/logs/bot.log`

---

## Contributing

### Development Setup

```powershell
# Clone repo
git clone https://github.com/bonsai/bonsai-workspace.git
cd bonsai-workspace

# Install dependencies
npm install -g pnpm
pnpm install

# Build
pnpm build

# Start dev server
pnpm dev
```

### Code Style

- **Rust**: `cargo fmt` and `cargo clippy`
- **JavaScript/Svelte**: `prettier` and `eslint`
- **Documentation**: Markdown with links to code files

### Pull Request Process

1. Fork repo
2. Create feature branch: `git checkout -b feat/my-feature`
3. Make changes
4. Run tests: `cargo test` & `npm test`
5. Commit: `git commit -m "feat: describe change"`
6. Push: `git push origin feat/my-feature`
7. Create PR with description

### Reporting Issues

**Create GitHub Issue** with:
- Clear title
- Reproduction steps
- Expected vs. actual behavior
- System info (`Help → System Info`)
- Logs (`~/.bonsai/logs/`)

---

## Documentation Index

### User Guides
- [Getting Started](docs/01-GETTING-STARTED.md) - First-time setup
- [IDE Features](docs/02-CORE-IDE.md) - Editor & file management
- [AI Assistant](docs/03-BONSAI-ASSISTANT.md) - Chat & tool calling
- [Model Training](docs/04-MODEL-TRAINER.md) - Fine-tuning guide
- [Collaboration](docs/07-COLLABORATION.md) - Sharing & teamwork

### System Administration
- [Security & Privacy](docs/11-SECURITY.md) - Encryption & sandboxing
- [Survival System](docs/05-SURVIVAL-SYSTEM.md) - Watchdog & recovery
- [Troubleshooting](docs/13-TROUBLESHOOTING.md) - Common issues

### Developer Guides
- [Developer Setup](docs/12-DEVELOPER.md) - Build & architecture
- [Sovereignty Roadmap](docs/10-SOVEREIGNTY.md) - Dependency replacement
- [API Reference](src-tauri/src/README.md) - Tauri command reference

### Reference
- [Glossary](docs/14-GLOSSARY.md) - All terminology defined
- [Architecture Diagrams](docs/ARCHITECTURE.md) - System design
- [Video Tutorials](docs/VIDEO_TUTORIAL_SCRIPT.md) - Step-by-step walkthroughs

---

## Support

### Getting Help

1. **Check Docs**: Most answers in [docs/](docs/) folder
2. **Search Issues**: GitHub issues often have solutions
3. **Ask Community**: GitHub Discussions or forums
4. **Report Bugs**: [GitHub Issues](https://github.com/bonsai/bonsai-workspace/issues)

### Contact

- **Email**: support@bonsai.dev
- **Discord**: [Bonsai Community Server](https://discord.gg/bonsai)
- **GitHub**: [Issues & Discussions](https://github.com/bonsai/bonsai-workspace)

---

## License & Attribution

**License**: AGPL-3.0 (with commercial licensing available)

See [LICENSE.md](LICENSE.md) for full terms.

**Credits**:
- Built with [Tauri 2](https://tauri.app/)
- UI with [Svelte 4](https://svelte.dev/)
- Models with [llama.cpp](https://github.com/ggerganov/llama.cpp)
- P2P with [libp2p](https://libp2p.io/)

---

## Roadmap

### Q3 2026
- [ ] USOS Phase 2: Cryptography replacement
- [ ] Advanced KDB with semantic search
- [ ] Web-based model serving

### Q4 2026
- [ ] USOS Phase 3: Database replacement
- [ ] Full app store integration
- [ ] Performance optimizations (40% faster)

### 2027
- [ ] USOS Phase 4-6: Complete OS replacement
- [ ] Custom silicon support
- [ ] Enterprise deployment guide

---

**Version**: 2.0 | **Last Updated**: 2026-06-04 | **Status**: Production Ready

For the latest updates, visit the [GitHub repository](https://github.com/bonsai/bonsai-workspace).
