# 🌳 Bonsai Ecosystem & Unified Secure Operating System (UOSC)
## Complete Official Documentation

**Version:** 2.0  
**Updated:** June 2, 2026  
**For:** Developers, Users, and AI Agents  
**Goal:** Explain everything about Bonsai in the simplest, clearest way possible.

---

## 📖 Start Here

### What is Bonsai?

Bonsai is a **complete, sovereign computing ecosystem** that runs entirely on your own hardware. It includes:

- An operating system (UOSC)
- An AI assistant (BonsAI V2)
- A blockchain (Nexus Core)
- Tools for coding, media, servers, and more
- Everything you need to compute independently

**Think of it like this:** If your computer is a tree, Bonsai is the entire ecosystem around it — the soil (OS), the light (AI), the water (networking), and the tools for shaping the tree (development tools).

### Why Bonsai?

| Problem | Bonsai Solution |
|---------|-----------------|
| You depend on cloud services | All computing happens locally |
| Your data leaves your device | No data leaves unless you allow it |
| AI models are huge and slow | Models are small, efficient, and local |
| Crashes require manual restart | Crashes are automatically fixed |
| Updates can break things | Updates are atomic — safe or rolled back |
| Servers are hard to manage | AI manages servers for you |
| Code changes require restart | Code updates without restarting |

### This Guide is Organized Like This

1. **UOSC** — The operating system foundation
2. **Core Systems** — Blockchain, AI models, databases
3. **Development Tools** — Compilers, emulators, bug finders
4. **Applications** — Chat bots, media, servers
5. **Security** — How everything is protected
6. **How to Use It** — Practical examples

---

## 🖥️ Section 1: UOSC — The Foundation

### What is UOSC?

**UOSC** = Unified Secure Operating System.

It is the foundation. Every other part of Bonsai runs on UOSC. But UOSC itself can run in three ways:

| Mode | What it means | When to use |
|------|---------------|------------|
| **Application mode** | Runs as a program inside Windows, macOS, or Linux | Right now — try Bonsai without changing your OS |
| **Integrated mode** | Runs as a lightweight VM alongside your existing OS | When you want UOSC features without full commitment |
| **Standalone mode** | Boots directly on your hardware as the only OS | For maximum security and performance |

### The UOSC Stack (from bottom to top)

```
┌──────────────────────────────────────────┐
│  User Applications (your programs)        │
├──────────────────────────────────────────┤
│  System Services (file system, network)   │
├──────────────────────────────────────────┤
│  Weave (component framework)              │
├──────────────────────────────────────────┤
│  Sanctum (isolated vaults/containers)     │
├──────────────────────────────────────────┤
│  Pulse (task scheduler)                   │
├──────────────────────────────────────────┤
│  Sentinel Core (microkernel)              │
└──────────────────────────────────────────┘
```

### Sentinel Core — The Heart

**What it does:** Manages memory, security, and the CPU scheduler.

**Why it's special:** It is mathematically proven to be correct (formally verified). This means there are no hidden bugs that could let a hacker break in.

**Key features:**
- Capability-based security (permissions are cryptographic tokens)
- Hardware-enforced isolation between programs
- Efficient memory management

### Sanctum — Isolated Vaults

**What it is:** A vault is a secure "box" where one program runs. It has its own memory, files, and devices. One program cannot escape and affect another.

**Why it matters:** If a web browser has a bug, the browser crashes — but your word processor, your AI assistant, and your banking app keep running unaffected.

**How it works:**
- Each vault is a lightweight virtual machine
- Starts instantly (< 100ms)
- Consumes very little memory
- Vaults can pass capabilities (permissions) to each other

**Example:**
```
Your UOSC system has 3 vaults running right now:

Vault 1: Web Browser (BWIF)
  ├─ Can access: display, network, files in ~/Downloads
  └─ Cannot access: files in ~/Private, camera, microphone

Vault 2: Email Client
  ├─ Can access: network, files in ~/Mail
  └─ Cannot access: files elsewhere, hardware devices

Vault 3: Video Game
  ├─ Can access: GPU, audio, controller
  └─ Cannot access: files, network, microphone
```

### Weave — Component Framework

Weave is the program manager. It is the system that:

- Starts and stops programs
- Monitors their health (are they running or crashed?)
- Grants capabilities (permissions) to programs
- Restarts failed programs

You describe what you want to run in a **Blueprint** file (YAML format). Weave reads it and makes it happen.

**Blueprint example:**
```yaml
system:
  name: "my-home-server"
  
components:
  - name: "web-server"
    image: "bonsai/web-engine:latest"
    capabilities: [network, file-read]
    replicas: 3
    
  - name: "database"
    image: "bonsai/postgres:latest"
    capabilities: [file-read, file-write, network]
    resources: { cpu: 2, memory: 8GB }
    
  - name: "ai-assistant"
    image: "bonsai/bonsai-v2:latest"
    capabilities: [ai-models, file-read, network]
    resources: { memory: 16GB }
```

### Blueprint & Crystal — Immutable System Images

**Blueprint:** The recipe file (what you want).

**Crystal:** The actual system (what you built from the recipe).

When you update your system:

1. You edit the Blueprint
2. UOSC builds a new Crystal from the Blueprint
3. The system switches to the new Crystal instantly
4. If the new Crystal fails, UOSC automatically switches back

This means **updates never break your system**. Worst case: you go back to the previous version.

### Echo — Peer-to-Peer Networking

Echo is how Bonsai devices find and talk to each other — without a central server.

**What it does:**
- Device A can discover Device B on the same network
- They can send messages directly (P2P)
- Files can be transferred directly
- Services on Device A can call services on Device B

**Example:**
```
Your home network has 3 UOSC devices:

Desktop (192.168.1.10)
  └─ Runs: web-server, database, ai-assistant

Laptop (192.168.1.20)
  └─ Runs: bonsai workspace (IDE)

Raspberry Pi (192.168.1.30)
  └─ Runs: home-automation

Echo lets them discover each other. The Laptop can call the 
Desktop's web-server directly. The Pi can store backups on 
the Desktop's database. No central server needed.
```

### TransferDaemon — Secure Network Bridge

TransferDaemon handles all network connections for UOSC. It:

- Encrypts everything (AES-256-GCM)
- Authenticates peers (Ed25519 signatures)
- Manages connections (P2P or through relay)
- Enforces per-request permissions (capabilities)

Result: Every byte leaving your device is encrypted, and only authorized recipients can read it.

---

## 💎 Section 2: Core Systems

### Nexus Core — The Blockchain

Nexus is a **private blockchain** built into UOSC. It powers:

- **Private payments** (send money anonymously)
- **Smart contracts** (programs that run on the blockchain)
- **Governance** (voting on system changes)

**Four Token Types:**

| Token | Use | Earned by | Notes |
|-------|-----|-----------|-------|
| **Stable** | Buy and sell things | Trading | 1 Stable ≈ $1 USD |
| **Stake** | Secure the network | Running a validator | Earn fees |
| **Work** | Contribute computing power | Mining/contribution | Proof of Useful Work |
| **Govern** | Vote on changes | Community participation | 1 vote per token |

**Key Features:**

- ✅ Transactions are instant (< 1 second)
- ✅ Completely private (nobody sees sender, receiver, or amount)
- ✅ No central authority (peer-to-peer)
- ✅ Very efficient (< 1 watt per transaction)

---

### 🧠 BonsAI V2 — The AI Model

BonsAI V2 is the AI that powers:

- Chat conversations
- Code generation and review
- Knowledge retrieval
- Content creation
- Server management (through Octopus AI)

**How BonsAI V2 is Different:**

| Feature | Bonsai | ChatGPT | Claude |
|---------|--------|---------|--------|
| **Runs locally** | ✅ | ❌ | ❌ |
| **Offline** | ✅ | ❌ | ❌ |
| **Learns your knowledge** | ✅ | ❌ | ❌ |
| **No data sent to cloud** | ✅ | ❌ | ❌ |
| **Updates without retraining** | ✅ (KDB) | ❌ | ❌ |
| **Adapts instantly to feedback** | ✅ | ❌ | ❌ |

**How it Works:**

```
User asks: "How do I fix a Docker container error?"

1. BonsAI V2 receives the question
2. It looks up Docker knowledge from KDB
3. It looks up the specific error pattern
4. It generates an answer using the knowledge
5. The answer cites sources from KDB

→ You get an answer backed by YOUR knowledge, not guesses
```

**Versions:**

- **Production (public)**: Safe, helpful, aligned
- **Research (private)**: Unrestricted version for internal development only

**Adaptive Scaling:**

BonsAI V2 automatically uses the right size for each task:

- **Simple question** ("What's 2+2?") → 100M parameter model
- **Moderate task** ("Write a function") → 1B parameter model
- **Complex reasoning** ("Design a system") → 7B parameter model
- **Expert-level** (rare) → 100B parameter model

You never wait for the 100B model if a smaller one can do the job.

---

### 📚 Knowledge Database (KDB)

**What is KDB?**

Instead of storing facts inside the AI model (which makes it huge), KDB stores facts separately. The AI looks them up when needed.

**Benefits:**

- AI model stays small and fast
- You can add new knowledge instantly (no retraining)
- AI always cites sources
- No hallucinations (AI can't make up facts)

**What can go in KDB:**

- Documentation
- Code snippets
- Tutorials
- Product manuals
- Internal procedures
- Personal notes
- Video transcripts
- Chat history

**How KDB is Organized:**

Every piece of knowledge has:

| Property | Example | Why it matters |
|----------|---------|---------------|
| **Domain** | "Docker", "Linux", "Medical" | Find relevant knowledge for the task |
| **Type** | "Fact", "Tutorial", "Troubleshooting" | Use the right kind of knowledge |
| **Difficulty** | "Beginner", "Expert" | Avoid overwhelming novices |
| **Audience** | "Developer", "System Admin", "Manager" | Match the person asking |
| **Quality score** | 0.0 to 1.0 | Trust the most accurate sources |
| **Links** | "Solves error #1042", "Requires: Docker basics" | Understand prerequisites |

**Searching KDB:**

```bash
# Search by keyword
bonsai kdb search "docker container restart"

# Search by domain
bonsai kdb search --domain docker --difficulty beginner

# Add your own knowledge
bonsai kdb add --file my-notes.md --domain "my-company"
```

---

### 🗂️ Knowledge Module Registry (KMDB)

KMDB is the **catalog** of all knowledge modules. It:

- Organizes modules by domain, type, and quality
- Detects contradictions (two modules saying opposite things)
- Scores quality (accurate, clear, complete, fresh)
- Suggests what to read next (prerequisites and related topics)

**Example KMDB Entry:**

```
Module: "Docker Container Lifecycle"
├─ Domain: container_orchestration
├─ Type: tutorial
├─ Difficulty: intermediate
├─ Quality: 0.94 (very high)
├─ Prerequisites:
│  └─ "What is Docker?" (0.92)
├─ Related:
│  ├─ "Docker Volumes" (0.91)
│  └─ "Docker Networking" (0.89)
└─ Contradictions:
   └─ "Lifecycle Best Practices v1" (confidence: 0.73, needs review)
```

---

## 🔧 Section 3: Development Tools

### BACE — The Atomic Compiler

BACE is a next-generation compiler that gives you the **best of both worlds:**

- The **instant feedback** of Python
- The **blazing performance** of Rust

**How it Works — Three Tiers:**

```
Tier 1: INTERPRETER (instant, flexible)
  ↓ (code gets hot, automatic transition)
Tier 2: JIT COMPILER (fast, optimized)
  ↓ (for production, or on demand)
Tier 3: AOT COMPILER (final, fully optimized)
```

**The Magic: Hot-Reload**

```rust
// Original function in your running program
fn calculate_interest(amount: f64, rate: f64) -> f64 {
    amount * rate / 100.0
}

// You notice a bug: should be divided by 365
// You fix it:
fn calculate_interest(amount: f64, rate: f64) -> f64 {
    amount * rate / 36500.0  // Fixed!
}

// You save the file. The running program updates INSTANTLY.
// No restart. No data loss. No waiting.
```

**Supported Languages:**

- Rust (full support, most efficient)
- C/C++ (good support)
- Python (interpreted tier only)
- JavaScript/TypeScript (JIT compilation)
- Go (good support)
- Java (JIT + AOT)
- Any WASM language (instant)

**P2P Compilation Cache:**

When your colleague compiles the same function, they get it from you instantly — not from a central server. This saves bandwidth and speeds up builds.

---

### BUSH — Universal Emulator

BUSH lets you run **any software on any hardware**. Some examples:

| Want to run | On | BUSH makes it work |
|-----------|--|--------------------|
| Raspberry Pi OS | Desktop | ✅ Perfect |
| PlayStation 2 games | Laptop | ✅ 95% speed |
| Windows XP | Linux | ✅ Fully compatible |
| iPhone app | Android | ✅ Converted on-the-fly |
| Future quantum code | Today's computer | ✅ Simulator included |

**How it Works:**

```
Your Hardware: x86 64-bit CPU
You want to run: ARM 32-bit program

BUSH does:
1. Reads the ARM program
2. Translates ARM instructions to x86
3. Compiles to native x86 code (JIT)
4. Runs at ~95% of native speed

→ The program runs as if it were designed for your hardware
```

**Use Cases:**

- Test code on a Raspberry Pi without owning one
- Run legacy software (DOS games, old Windows apps)
- Simulate IoT devices
- Debug hardware-specific bugs

---

### 🐛 Bug Hunter & Code Sweeper

The Bug Hunter finds bugs automatically. The Code Sweeper fixes many of them automatically.

**What They Find:**

| Category | Tool | Example |
|----------|------|---------|
| **Static bugs** | Clippy (Rust) | Variable never used, wrong function arg |
| **Security** | Bandit (Python), Semgrep | SQL injection, hardcoded passwords |
| **Performance** | Perf profiler | Memory leaks, infinite loops |
| **Race conditions** | ThreadSanitizer | Data race between threads |
| **Crashes** | Fuzzer | Input that causes crash |
| **Network attacks** | Penetration tester | XSS, CSRF, path traversal |

**How to Use:**

```bash
# Quick check (takes 10 seconds)
bonsai sweep

# Full check with AI code review (takes 5 minutes)
bonsai sweep --full --ai

# Fix issues automatically
bonsai sweep --full --auto-fix

# Only check for security issues
bonsai sweep --security
```

**How AI Code Review Works:**

1. BonsAI V2 reads your code
2. It looks for design issues, inefficiencies, and bugs
3. It explains what's wrong in plain English
4. It suggests fixes
5. It offers to apply the fixes automatically

---

## 🌐 Section 4: Applications & Services

### Bonsai Container Fabric (BCF)

BCF is how you run applications. Think of it as **Docker on steroids**.

**Why BCF is Better Than Docker:**

| Feature | BCF | Docker |
|---------|-----|--------|
| **Startup time** | <100ms | 1-5 seconds |
| **Isolation** | Hardware (VM) | Software (cgroups) |
| **Image distribution** | P2P | Central registry |
| **Updates** | Atomic, instant | Downtime possible |
| **Resource limits** | Hard guarantees | Soft limits |

**How to Deploy:**

```bash
# 1. Write a Blueprint
cat > my-app.bp << EOF
components:
  - name: web-server
    image: bonsai/web-engine:latest
    replicas: 3
    capabilities: [network, file-read]
    resources: { cpu: 2, memory: 4GB }
    
  - name: database
    image: bonsai/postgres:latest
    capabilities: [file-read, file-write]
    resources: { memory: 8GB }
EOF

# 2. Deploy it
bonsai container deploy --blueprint my-app.bp

# 3. It's running now
bonsai container list
```

**Managing Containers:**

```bash
# Scale a service
bonsai container scale web-server --replicas 5

# Check logs
bonsai container logs web-server

# Rollback to previous version
bonsai container rollback my-app

# Health check
bonsai container health
```

---

### Bonsai Web Engine (BWE)

BWE is a **replacement for Node.js**. A high-performance web server that you can write in Rust, JavaScript, TypeScript, or Python.

**Performance:**

- Handles 500,000+ requests per second (vs ~30,000 for Node.js)
- Single-thread latency: < 1ms
- Memory efficient: < 50MB base

**Writing a Handler:**

```javascript
// handlers.js - runs on BWE
export async function handleRequest(request, context) {
  if (request.path === '/api/data') {
    return new Response(JSON.stringify({ 
      message: 'Hello, World!' 
    }), {
      headers: { 'Content-Type': 'application/json' }
    });
  }
  return new Response('Not found', { status: 404 });
}
```

```bash
# Deploy it
bonsai web deploy --handler handlers.js

# Visit it
curl http://localhost:8080/api/data
```

**Hot-Reload:**

Edit your handler, save, and the changes take effect **instantly** — no restart, no request loss.

---

### 🎥 Bonsai Media Nexus (BMN)

BMN is a **complete media production studio** in software.

**What it Replaces:**

- OBS Studio (screen capture + streaming)
- Streamlabs (streaming + overlays)
- FFmpeg (video encoding)
- VLC (media playback)

**Key Features:**

- **Distributed encoding**: Split video encoding across multiple devices
- **P2P streaming**: Viewers relay the stream (no need for expensive CDN)
- **AI enhancement**: Real-time upscaling, background removal, noise suppression
- **Multi-user production**: Multiple people control one stream simultaneously
- **Immutable recording**: Every stream is saved forever

**Quick Example:**

```bash
# 1. Capture your screen and audio
bonsai media capture --display 1 --audio built-in

# 2. Add AI enhancement
bonsai media enhance --upscale 2x --remove-background

# 3. Stream to multiple platforms
bonsai media stream --output youtube://mystream \
                     --output twitch://mystream \
                     --output rtmp://custom-server

# 4. Real-time stats
bonsai media stats  # Shows bitrate, viewers, encoder load
```

---

### 🐙 Octopus AI — Server Manager

Octopus AI is a specialized AI that **manages your servers**. It understands:

- What every container does
- What services depend on what
- Common error patterns
- Security vulnerabilities
- How to fix problems

**How to Use:**

You can talk to Octopus through:
- **Discord** (send a message in a channel)
- **Telegram** (private chat)
- **Web console** (dashboard in Bonsai Workspace)

**Example Conversations:**

```
You: "Why did the api container crash?"
Octopus: "Memory leak in the request handler. Restarted at 14:32.
         I've applied a temporary memory limit. Full fix needed
         by tomorrow. Shall I create a task?"

You: "Check for security updates"
Octopus: "6 updates available, 2 critical. CVE-2024-1234 affects
         the postgres container. Patch ready. Apply? (Y/n)"

You: "Show me the top 3 most expensive containers"
Octopus: "1. ml-inference (12.5 GB, 4 CPUs)
         2. database (8.2 GB, 2 CPUs)
         3. cache (3.1 GB, 1 CPU)"
```

**Autonomous Healing:**

```
Timeline:
  14:30 - web-server container crashes
  14:31 - Octopus detects the crash
  14:32 - Octopus checks if it's a known pattern
  14:32 - Pattern found: "Memory exhaustion after 8 hours"
  14:32 - Applied known fix: restart + increase memory
  14:33 - Container restarted successfully
  14:34 - Octopus notifies you: "Fixed automatically. Details: ..."
  14:35 - User approves fix / or asks for more investigation
```

---

### 💬 OmniBot

OmniBot is a unified chat bot accessible from **anywhere**.

**Platforms:**
- Telegram
- Discord
- Slack
- Web console
- Email (reply-based)

**What it Can Do:**

```
You: "Deploy my app"
OmniBot: "Which app? (1) web-server (2) api-service"
You: "1"
OmniBot: Deploys web-server

You: "What's the status?"
OmniBot: Shows full system status

You: "Fix the database connection error"
OmniBot: Analyzes error, applies fix, asks for confirmation

You: "Train the model on new data"
OmniBot: Starts training, provides real-time progress
```

**Capability Tokens:**

Different users have different permissions. An admin can restart services; a developer can only view logs.

---

### 🌐 Bonsai API Bridge

The API Bridge is the **universal gateway** to everything. Any application can talk to any Bonsai service.

**Protocols Supported:**
- REST (HTTP)
- gRPC
- WebSocket
- MCP (Model Context Protocol)

**Example:**

```bash
# Query the AI
curl -X POST http://localhost:8081/api/bonsai-v2 \
  -H "Authorization: Bearer YOUR_CAPABILITY_TOKEN" \
  -d '{"query": "How do I fix a Docker error?"}'

# Deploy a container
curl -X POST http://localhost:8081/api/container/deploy \
  -H "Authorization: Bearer YOUR_CAPABILITY_TOKEN" \
  -d '{"blueprint": "...content..."}'

# Stream media
curl -X POST http://localhost:8081/api/media/stream \
  -H "Authorization: Bearer YOUR_CAPABILITY_TOKEN" \
  -d '{"output": "youtube://..."}'
```

---

## 🛡️ Section 5: Security & Privacy

### Zero-Trust Architecture

Every action requires permission. Nothing is trusted by default.

```
Request: "Read file /home/user/document.txt"

Security checks:
  1. Is the requester authenticated? (signature valid?)
  2. Does their capability token allow file reads?
  3. Does the token allow reading this specific path?
  4. Has the token expired?
  5. Is the token revoked?

Only if ALL checks pass → grant access
```

### Capability Tokens

A capability token is like a **digitally signed permission slip**.

**Example token:**

```json
{
  "subject": "claude-agent",
  "actions": ["read", "execute"],
  "resources": ["/home/user/code/*"],
  "expires": "2026-06-09T12:00:00Z",
  "signature": "ed25519:abc123..."
}
```

**You can give an agent a token that says:**

- "Read any file in /home/user/code"
- "For 7 days"
- "After that, the permission expires automatically"
- "I can revoke it anytime"

### Encryption

**In transit:** All network traffic is encrypted with AES-256-GCM (military-grade).

**At rest:** All stored data is encrypted with your key.

**Example:**

```
You send: "Deploy my app"
Bonsai encrypts it using your public key
Network (untrusted WiFi, ISP, etc.) sees: [encrypted blob]
Bonsai receives and decrypts using your private key
Only Bonsai can read what you sent
```

### Immutable Audit Trail (Universe)

Every significant action is recorded in **Universe** — an immutable event store.

**You can replay any past session:**

```bash
# Find a specific event
bonsai universe grep "api-error"

# Replay a deployment from 2 hours ago
bonsai universe replay --timestamp 2026-06-02T10:00:00Z

# Verify nothing was tampered with
bonsai universe verify --start 2026-06-01 --end 2026-06-02
```

### Privacy by Default

✅ **All computation is local** (no data sent to cloud)  
✅ **No telemetry** (Bonsai never phones home)  
✅ **No ads** (no tracking for advertising)  
✅ **PII redaction** (logs are scanned for sensitive data)  
✅ **You control everything** (explicit permission for any network action)  

---

## 💡 Section 6: Quick Start Examples

### Example 1: Deploy a Web App

```bash
# Step 1: Create a blueprint
cat > my-blog.bp << 'EOF'
components:
  - name: web-server
    image: bonsai/web-engine:latest
    capabilities: [network, file-read]
    resources: { memory: 2GB, cpu: 1 }
    
  - name: database
    image: bonsai/postgres:latest
    capabilities: [file-read, file-write]
    resources: { memory: 4GB, cpu: 1 }
EOF

# Step 2: Deploy
bonsai container deploy --blueprint my-blog.bp

# Step 3: Check status
bonsai container list
bonsai container health my-blog

# Step 4: Access your app
open http://localhost:8080
```

### Example 2: Train the AI on Your Code

```bash
# Step 1: Import your codebase
bonsai kdb import --directory ./my-project --domain "my-code"

# Step 2: Train the AI
bonsai bonsai-v2 train --source kdb://my-code --duration 1h

# Step 3: Test it
bonsai chat "Explain my project structure"
# → AI explains your code based on your actual files

# Step 4: Continuous improvement
# Every time you give thumbs-up/down, the AI learns
```

### Example 3: Fix Bugs Automatically

```bash
# Step 1: Scan for bugs
bonsai sweep --full --ai

# Step 2: Review findings
# Shows all bugs found with explanations

# Step 3: Auto-fix safe issues
bonsai sweep --auto-fix

# Step 4: Manual review for complex ones
# You review and approve each fix

# Step 5: Verify
bonsai test  # Run your test suite to confirm
```

### Example 4: Stream Live Content

```bash
# Step 1: Start capture and encoding
bonsai media capture --display 1 --audio built-in

# Step 2: Start streaming to YouTube
bonsai media stream --output youtube://your-stream-key

# Step 3: Monitor
bonsai media stats  # Real-time viewer count, bitrate, etc.

# Step 4: Stop
bonsai media stop
```

### Example 5: Query Your Server with Octopus

```bash
# Start a conversation
bonsai chat --mode octopus

# Ask questions
> "List all running containers"
Octopus responds with list

> "Why is postgres using 80% CPU?"
Octopus analyzes logs and explains

> "Apply all security patches"
Octopus shows patch list, asks for confirmation

> "Create a backup"
Octopus backs up database, shows completion
```

---

## 📖 Section 7: Glossary

| Term | Definition |
|------|-----------|
| **BACE** | Atomic Compiler Engine — compile code with instant feedback |
| **BCF** | Bonsai Container Fabric — run applications in secure containers |
| **Blueprint** | Configuration file describing an entire system |
| **BWE** | Bonsai Web Engine — high-performance web server |
| **BUSH** | Universal Simulation Engine — emulate any hardware |
| **Capability** | Digital permission token (signed, time-limited, revocable) |
| **CAS** | Content-Addressed Store — files identified by content hash |
| **Crystal** | Immutable system image built from a Blueprint |
| **Crystal Swap** | Atomic system update (new version or rollback) |
| **Echo** | Peer-to-peer discovery and messaging |
| **ETL** | EternalTrainingLoop — continuous AI learning |
| **KDB** | Knowledge Database — searchable repository of facts |
| **KMDB** | Knowledge Module Registry — catalog of all knowledge |
| **MCP** | Model Context Protocol — standard for AI tool calling |
| **Sanctum** | Hardware-isolated vault for running programs |
| **Sentinel Core** | Microkernel at the heart of UOSC |
| **Survival System** | Auto-detects and repairs crashes |
| **TransferDaemon** | Handles all peer-to-peer networking |
| **Universe** | Immutable event store for audit and replay |
| **UOSC** | Unified Secure Operating System |
| **Weave** | Component framework (program manager) |

---

## 🎓 Learning Path

**New to Bonsai?**

1. Read: **Section 1 (UOSC)** — understand the foundation
2. Read: **Section 2 (Core Systems)** — understand AI and knowledge
3. Try: **Section 6, Example 1** — deploy your first app
4. Read: **Section 4 (Applications)** — explore specific tools you care about
5. Try: **Section 6, Example 5** — manage servers with Octopus AI

**Developer?**

1. Read: **Section 3 (Development Tools)** — BACE compiler, BUSH emulator, Bug Hunter
2. Try: **Section 6, Example 2** — train AI on your code
3. Read: **Section 4 (BWE, BCF)** — how to write and deploy services
4. Practice: Use hot-reload with BACE

**System Administrator?**

1. Read: **Section 1 (UOSC)** and **Section 4 (BCF, Octopus AI)**
2. Try: **Section 6, Examples 1 & 5** — deploy and manage containers
3. Read: **Section 5 (Security)** — understand access control

---

## ❓ FAQ

**Q: Can I use Bonsai right now?**  
A: Yes. Download Bonsai Workspace and run it in Application Mode on top of your existing OS.

**Q: Does Bonsai work offline?**  
A: Yes. Everything runs locally. No internet required (except for features you explicitly enable).

**Q: How much storage do I need?**  
A: UOSC core: 500 MB. BonsAI V2: 4-40 GB depending on model size. Plus your data.

**Q: Can I use Bonsai on my phone?**  
A: Android: Yes (lightweight components). iOS: No (not yet).

**Q: What if I find a bug?**  
A: Report it via OmniBot or open an issue on GitHub. The Bug Hunter helps us find issues proactively.

**Q: Is my data really private?**  
A: Yes. No data leaves your device unless you explicitly allow it. You can verify this by checking the audit log in Universe.

---

## 📞 Getting Help

- **In-app help**: Press `?` in Bonsai Workspace
- **OmniBot**: Message it on Telegram, Discord, or web console
- **Documentation**: Read more at [docs.bonsai.sh](https://docs.bonsai.sh)
- **Community**: Join Discord at [discord.gg/bonsai](https://discord.gg/bonsai)

---

## 📝 Document Info

- **Version:** 2.0
- **Last updated:** June 2, 2026
- **License:** Public domain
- **Authors:** Bonsai team + community contributors

This documentation is continuously updated. Feedback is welcome.

