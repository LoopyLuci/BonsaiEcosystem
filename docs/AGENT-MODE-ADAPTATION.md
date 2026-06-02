# Copilot Agent Mode Adaptation to Bonsai Swarm Integration

**Design Phase: Complete Agent Mode Adaptation**

> Transform GitHub Copilot's Cloud Agent into a local, sovereign Bonsai Swarm orchestrated system.

---

## Executive Summary

This design document specifies how to integrate GitHub Copilot Agent Mode—a cloud-based AI agent orchestration system—into Bonsai Workspace as a completely local, sovereign, user-controlled swarm of task-executing agents. The result is **Copilot Agent Mode for Bonsai**: agents that run on your hardware, with your models, following your rules, and generating full audit trails.

### Key Principles
- **Compatibility**: Existing Copilot agent definitions translate to Bonsai agents with minimal changes
- **Sovereignty**: All agent execution is local; no cloud dependency
- **Control**: Users have fine-grained approval authority over agent actions
- **Transparency**: Every action is visible, auditable, and reversible via Time Travel
- **Extensibility**: Custom agent definitions and tool plugins are straightforward

---

## 1. Copilot Agent Mode Current Architecture

### 1.1 How Copilot Agent Mode Works Today

GitHub Copilot Agent Mode is a cloud-based system where:

1. **User initiates** a task via Copilot chat or slash command
2. **GitHub routes** the request to an ephemeral agent runner (Ubuntu Linux VM, <5min lifetime)
3. **Agent executor** loads agent persona and tool definitions
4. **Agent runs autonomously** calling MCP tools (GitHub MCP Server, Playwright MCP Server) to read/write files, run commands, navigate the web
5. **Results** are streamed back to the user; agent is terminated
6. **No persistent approval**: user approves the agent role once, then it acts autonomously

### 1.2 Configuration Files (`.github/` directory)

Copilot stores agent definitions in the repository:

```
.github/
├── copilot-instructions.md           # Base system instructions
├── agents/
│   ├── security-auditor.agent.md     # Agent persona + instructions
│   ├── performance-analyst.agent.md
│   ├── documentation-writer.agent.md
│   └── test-engineer.agent.md
├── instructions/
│   ├── code-review.instructions.md   # Context-specific guidance
│   ├── security-best-practices.instructions.md
│   └── performance-testing.instructions.md
└── hooks/
    ├── pre-agent-run.json            # Approve-only, no execution
    ├── post-agent-success.json        # Create PR, notify team
    └── on-agent-error.json            # Escalate to human
```

#### `.agent.md` Format

```markdown
---
name: "Security Auditor"
description: "Analyzes code for security vulnerabilities"
instructions: |
  You are a security auditor with deep knowledge of OWASP Top 10.
  Your task: scan the codebase for security issues.
  Report findings in a structured format.
model: "gpt-4-turbo"  # Copilot specifies the model
---
[Additional instructions can go here]
```

### 1.3 Execution Environment

**Original (Cloud):**
- Ephemeral GitHub Actions runner (Ubuntu 24.04, temporary)
- 10-minute timeout (hard limit)
- Limited filesystem (read/write in `.github/` and workspace root)
- Can't access `/etc`, system files, or outside the repo
- MCP tools provide isolation via capability tokens

**Bonsai Equivalent:**
- **Sanctum Vault** – isolated process namespace (similar to Actions runner)
- **Compute Fabric node** – agent runs on any capable device (desktop, mobile via WorkManager)
- **Configurable timeout** – default 1 hour, user can extend
- **Workspace isolation** – read/write in project directory, with whitelisting for system access
- **MCP tools via UACS** – Universal Agent Control System enforces capability tokens

### 1.4 MCP Tool Integration

**Copilot's MCP Servers:**

| Server | Tools | Purpose |
|---|---|---|
| **GitHub MCP** | `github.issues.create`, `.update`, `.list` | Manage GitHub issues & PRs |
| **Playwright MCP** | `browser.navigate`, `.click`, `.fill` | Web automation |

**Bonsai Equivalent:**

| Bonsai Tool | Maps To | Purpose |
|---|---|---|
| `submit_issue` | GitHub MCP + REST | Create issues (local or remote) |
| `browser_navigate` | Playwright MCP equivalent | Web automation via headless browser |
| `read_file` | Local filesystem | Read project files |
| `write_file` | Local filesystem | Create/update project files |
| `run_command` | Sanctum process | Execute shell commands in isolation |
| `git_commit` | Git CLI | Commit changes with message |
| `search_codebase` | Ripgrep + KDB | Semantic code search |
| `kdb_retrieve` | Knowledge Database | Inject domain-specific context |

### 1.5 Approval & HITL

**Copilot's Model:**
- User approves the agent *role* once (e.g., "I trust Security Auditor")
- Agent runs autonomously with no per-action approval
- If the agent tries to push to main, GitHub Actions themselves may require approval

**Bonsai's Model:**
- User approves agent *actions* based on risk category
- **LOW**: `read_file`, `search_codebase` → auto-approve or stream to UI
- **MEDIUM**: `web_search`, `run_tests` → notify user but auto-approve if configured
- **HIGH**: `write_file`, `delete_file`, `git_push` → require explicit approval
- All approvals are logged for audit trail

---

## 2. Agent Definition Format Compatibility

### 2.1 Copilot Format (Current Standard)

```markdown
---
name: "Security Auditor"
description: "Analyzes code for security vulnerabilities"
instructions: |
  You are a security auditor specializing in OWASP Top 10.
  Your mission: identify security issues in the codebase.
  For each issue, provide:
  1. Location (file, line)
  2. Severity (Critical, High, Medium, Low)
  3. Remediation advice
model: "gpt-4-turbo"
---

## How you work
[Additional instructions]
```

### 2.2 Bonsai Agent Definition Format (YAML)

```yaml
# agents/security-auditor.agent.yaml
id: security-auditor-v1
name: Security Auditor
description: Analyzes code for security vulnerabilities
version: 1.0.0

# Base instructions (compatible with Copilot)
instructions: |
  You are a security auditor specializing in OWASP Top 10.
  Your mission: identify security issues in the codebase.
  For each issue, provide:
  1. Location (file, line)
  2. Severity (Critical, High, Medium, Low)
  3. Remediation advice

# Bonsai-specific enhancements
bonsai:
  # Model selection
  model:
    role: "security"           # Resolve from config/model_registry.yaml
    fallback: "qwen-7b"        # If role not found
    max_tokens: 8000
    temperature: 0.1           # Lower = more focused

  # Knowledge module injection
  knowledge_modules:
    - "owasp-top-10"
    - "cwe-catalog"
    - "cve-database"
    - "security-patterns"
  
  # Tools available to this agent
  tools:
    read_file:
      enabled: true
      description: "Read source files"
    write_file:
      enabled: false            # This agent cannot write
    search_codebase:
      enabled: true
    run_cargo_check:
      enabled: true
    submit_issue:
      enabled: true
    web_search:
      enabled: false            # No external network
  
  # Approval policy
  approval_policy:
    mode: "auto"                # auto | require | headless
    approval_required:
      - write_file
      - delete_file
      - git_push
      - submit_issue             # Require approval before filing issues
    auto_approve:
      - read_file
      - search_codebase
      - run_cargo_check
    notify_on:
      - web_search               # Notify but auto-approve
  
  # Resource limits
  timeout_seconds: 3600
  max_tokens_total: 20000        # Total context budget
  memory_limit_mb: 512           # Process memory
  
  # Execution context
  execution:
    isolation: "sanctum"         # sanctum | container | native
    working_dir: "${WORKSPACE}"  # ${WORKSPACE}, ${HOME}, etc.
    env_vars:
      RUST_BACKTRACE: "1"
      BONSAI_LOG_LEVEL: "info"
    deny_paths:
      - "/etc"
      - "/root"
      - "${HOME}/.ssh"           # Never access SSH keys
  
  # Output & logging
  output:
    format: "markdown"           # markdown | json | xml
    include_reasoning: true      # Show chain-of-thought
    log_level: "info"
    export_to_issue: true        # Auto-create issue from findings
  
  # Error recovery
  on_error:
    retry_count: 3
    retry_backoff: "exponential"
    fallback_action: "report"    # report | escalate | abandon

# Optional: Copilot model specification (ignored by Bonsai)
model: "gpt-4-turbo"
```

### 2.3 Translation Service

A **Copilot-to-Bonsai Translation Service** converts `.github/agents/*.agent.md` to Bonsai format:

```rust
// crates/bonsai-agent-translator/src/lib.rs

pub struct CopilotTranslator;

impl CopilotTranslator {
    pub fn translate(copilot_md: &str) -> BonsaiAgentDef {
        // 1. Parse YAML frontmatter
        let (frontmatter, instructions) = parse_frontmatter(copilot_md);
        
        // 2. Extract name, description, model
        let name = frontmatter.get("name").unwrap().as_str();
        let description = frontmatter.get("description").unwrap().as_str();
        let model = frontmatter.get("model").unwrap_or("qwen-7b");
        
        // 3. Map Copilot model to Bonsai role
        let bonsai_role = map_model_to_role(model);
        
        // 4. Infer tools from instructions (NLP-based)
        let tools = infer_tools_from_instructions(&instructions);
        
        // 5. Generate Bonsai YAML with sensible defaults
        BonsaiAgentDef {
            name,
            description,
            instructions,
            bonsai: BonsaiSpecific {
                model: ModelSpec {
                    role: bonsai_role,
                    fallback: "qwen-7b",
                    max_tokens: 8000,
                    temperature: 0.3,
                },
                knowledge_modules: infer_knowledge_modules(&instructions),
                tools: tools.into_iter()
                    .map(|t| (t, ToolPolicy::default()))
                    .collect(),
                approval_policy: ApprovalPolicy::default_for_role(&bonsai_role),
                timeout_seconds: 3600,
                // ... other defaults
            }
        }
    }
}
```

### 2.4 Version Compatibility

If Copilot's format evolves (new frontmatter fields, new hook types):

1. **Translator versioning**: `CopilotTranslator::v1()`, `v2()`, etc.
2. **Format detection**: Parse frontmatter version or infer from field presence
3. **Deprecation warnings**: Alert user if legacy format detected
4. **Test suite**: Maintain test vectors for each Copilot format version observed in the wild

---

## 3. Context Injection for Agents

### 3.1 Workspace Awareness

**Current Copilot approach:** Agents have filesystem access; they read files as needed.

**Bonsai approach:** Agents receive a "workspace summary" in the system prompt, reducing token waste:

```rust
// crates/bonsai-agent-runtime/src/context.rs

pub struct WorkspaceContext {
    pub file_tree: FileTreeSummary,
    pub recent_history: Vec<GitCommit>,
    pub repo_metadata: RepositoryMetadata,
    pub active_files: Vec<String>,
    pub config_summary: ConfigSummary,
}

pub struct FileTreeSummary {
    // Only include first 3 levels, max 500 lines of output
    pub tree: String,  // Output of `tree --charset ascii -L 3 -h | head -100`
}

pub struct RepositoryMetadata {
    pub name: String,
    pub url: Option<String>,
    pub default_branch: String,
    pub last_commit: String,
    pub branches: Vec<String>,
    pub open_issues_count: usize,
    pub topics: Vec<String>,
}

pub struct ConfigSummary {
    pub rust_edition: Option<String>,  // from Cargo.toml
    pub ci_provider: Option<String>,   // GitHub Actions, etc.
    pub build_script: Option<String>,  // .github/workflows/build.yml
}
```

**System Prompt Injection:**

```markdown
## Workspace Context

### Repository Structure
{file_tree_summary}

### Recent History (Last 5 commits)
{git_log_summary}

### Repository Metadata
- Name: {repo_name}
- Primary branch: {default_branch}
- Open issues: {open_issues_count}
- Tech stack: {topics}

### Configuration
- Rust edition: {rust_edition}
- Build system: {build_script}
```

This keeps the prompt manageable (< 2000 tokens) while giving agents essential context.

### 3.2 Domain-Specific Context

**Knowledge Module Selection:**

The agent's `bonsai.knowledge_modules` list specifies which modules to load. For each module:

1. **Agent starts** with a user query (e.g., "find security issues")
2. **KDB Retriever** queries each active module with the agent's instructions as the search query
3. **Top-k passages** are injected into the system prompt (e.g., "OWASP Top 10 Injection Attacks: ...")
4. **Agent responds** with this injected knowledge, significantly improving accuracy

**Example Knowledge Module Injection:**

```
## Domain Knowledge: OWASP Top 10

[Retrieved from module: owasp-top-10]

### A03:2021 – Injection
Injection flaws occur when an attacker is able to interleave malicious code into a program.
Examples: SQL injection, command injection, LDAP injection.

Detection patterns:
- Unsanitized user input in database queries
- String concatenation in SQL (not parameterized)
- os.system() / Runtime.exec() with unsanitized args

Remediation:
- Use parameterized queries / prepared statements
- Input validation and output encoding
- Principle of least privilege for database accounts

---

### A04:2021 – Insecure Design
Insecure design represents missing or ineffective control design.
...
```

### 3.3 Conversation History Management

**Token Budget Constraint:**

Agents have a `max_tokens_total` budget (e.g., 20,000). To maximize useful work:

1. **Current turn** (user message) – always included
2. **Last N messages** (sliding window) – included if space available
3. **Older messages** – compressed via `CompressorModel`

**Compression Example:**

```rust
// crates/bonsai-agent-runtime/src/compression.rs

pub struct ConversationCompressor {
    model: ModelHandle,  // e.g., qwen-1.5b (fast, low-cost)
}

impl ConversationCompressor {
    pub async fn compress_history(
        messages: &[Message],
        target_tokens: usize,
    ) -> BonsaiResult<String> {
        // Summarize older messages into a paragraph
        let prompt = format!(
            "Summarize this conversation history in 1-2 sentences:\n{:?}",
            messages
        );
        let summary = self.model.infer(&prompt, target_tokens).await?;
        Ok(summary)
    }
}
```

**Configuration:**

```yaml
# config/agent_context.yaml
conversation_history:
  mode: "sliding_window"      # sliding_window | all | compressed
  window_size: 10             # Keep last 10 messages
  compression_threshold: 15   # If > 15 messages, compress oldest
  summary_model: "qwen-1.5b"  # Fast summarizer
```

---

## 4. Tool Mapping for Agents

### 4.1 GitHub MCP Server → Bonsai Tools

| Copilot Tool | Bonsai Tool | Implementation |
|---|---|---|
| `github.issues.create` | `submit_issue` | REST API (GitHub or local tracker) |
| `github.issues.update` | `update_issue` | REST API |
| `github.pull_requests.create` | `create_pull_request` | `git` CLI + GitHub REST |
| `github.pull_requests.get_diffs` | `get_pr_diff` | `git diff` or GitHub REST |
| `github.workflows.list_runs` | `list_github_workflows` | GitHub REST API |
| `github.workflows.trigger` | `trigger_github_workflow` | GitHub REST API (requires token) |

**Example: `submit_issue` Implementation**

```rust
// crates/bonsai-mcp-server/src/tools/issues.rs

pub async fn submit_issue(
    title: String,
    body: String,
    labels: Option<Vec<String>>,
    assignee: Option<String>,
) -> BonsaiResult<IssueResponse> {
    // 1. Check approval (HITL gate if configured)
    approve_tool_call("submit_issue", &json!({
        "title": title,
        "body": body,
        "labels": labels,
    })).await?;
    
    // 2. Determine backend (GitHub or local)
    let backend = issue_backend().await?;  // GitHub token present? Use GitHub; else local
    
    // 3. Create issue
    match backend {
        IssueBackend::GitHub { client, owner, repo } => {
            client.create_issue(&owner, &repo, &title, &body, labels, assignee).await
        }
        IssueBackend::Local { db } => {
            db.insert_issue(Issue {
                id: Uuid::new_v4(),
                title,
                body,
                labels: labels.unwrap_or_default(),
                assignee,
                created_at: Utc::now(),
                status: IssueStatus::Open,
            }).await
        }
    }
}
```

### 4.2 Playwright MCP Server → Bonsai Tools

Bonsai doesn't have direct Playwright integration yet, but can provide **headless browser automation**:

| Copilot Tool | Bonsai Tool | Implementation |
|---|---|---|
| `browser.navigate` | `browser_navigate` | puppeteer-rs or servo |
| `browser.click` | `browser_interact` | Headless browser + coordinate injection |
| `browser.fill` | `browser_interact` | Headless browser + text input |
| `browser.get_screenshot` | `browser_screenshot` | Headless browser render to PNG |

**Note:** Playwright requires significant sandbox isolation. For MVP, Bonsai can:
- Provide read-only web scraping via `fetch_url`
- Provide interactive web automation via `/mcp/playwright` endpoint (requires manual browser session setup)

### 4.3 Bonsai-Specific Tools

These are native to Bonsai and not in Copilot:

| Tool | Purpose | Risk Level |
|---|---|---|
| `read_file` | Read workspace files | LOW |
| `write_file` | Create/edit files | HIGH |
| `delete_file` | Remove files | HIGH |
| `run_command` | Execute shell commands | HIGH |
| `search_codebase` | Ripgrep + semantic search | LOW |
| `run_cargo_check` | Compile and check | MEDIUM |
| `run_cargo_test` | Run test suite | MEDIUM |
| `git_commit` | Stage and commit changes | HIGH |
| `git_push` | Push to remote | HIGH |
| `kdb_retrieve` | Query knowledge modules | LOW |
| `list_directory` | List files | LOW |
| `fetch_url` | Download web content | MEDIUM (requires network approval) |

### 4.4 Tool Discovery & Capability Tokens

**Agent Tool Discovery:**

At startup, agent requests tool list via MCP:

```json
{
  "method": "tools/list",
  "params": {
    "token": "<capability-token>",
    "agent_id": "security-auditor-v1"
  }
}
```

**Server responds with filtered list** (only tools the agent has capability for):

```json
{
  "tools": [
    {
      "name": "read_file",
      "description": "Read file contents",
      "input_schema": { ... }
    },
    {
      "name": "search_codebase",
      "description": "Search files with ripgrep",
      "input_schema": { ... }
    }
  ]
}
```

**Capability Token Structure:**

```rust
pub struct CapabilityToken {
    pub agent_id: String,
    pub agent_version: String,
    pub issued_at: SystemTime,
    pub expires_at: SystemTime,
    pub capabilities: Vec<String>,  // ["read_file", "search_codebase", ...]
    pub resource_limits: ResourceLimits,
    pub signature: Vec<u8>,  // HMAC-SHA256(token, secret)
}

pub struct ResourceLimits {
    pub max_tokens: u32,
    pub timeout_seconds: u32,
    pub memory_mb: u32,
}
```

---

## 5. Multi-Agent Orchestration

### 5.1 Sequential Composition

Agents can invoke other agents in sequence:

```
Agent 1 (CodeAnalyzer)
  ├─ reads source code
  ├─ writes findings.md
  └─ signals completion

Agent 2 (PerfOptimizer)
  ├─ reads findings.md
  ├─ reads hot functions from findings
  ├─ optimizes each function
  └─ commits optimizations
```

**Implementation:**

```rust
// crates/bonsai-agent-runtime/src/sequencer.rs

pub struct AgentSequencer {
    agents: Vec<AgentDef>,
    context: SharedContext,
}

impl AgentSequencer {
    pub async fn run_sequence(&self) -> BonsaiResult<Vec<AgentResult>> {
        let mut results = Vec::new();
        
        for agent_def in &self.agents {
            let agent = Agent::new(agent_def.clone(), self.context.clone());
            let result = agent.run().await?;
            results.push(result);
            
            // Agent 1's output becomes Agent 2's context
            self.context.update_from_agent_result(&result);
        }
        
        Ok(results)
    }
}
```

**Agent Definition with Sequencing:**

```yaml
# workflows/code-optimization.workflow.yaml
agents:
  - id: analyzer
    agent: security-auditor-v1
    timeout: 600
    
  - id: optimizer
    agent: performance-analyst-v1
    depends_on: [analyzer]
    timeout: 1200
    inputs:
      findings_file: "${analyzer.output.findings_path}"
    
  - id: reviewer
    agent: code-reviewer-v1
    depends_on: [optimizer]
    timeout: 600
```

### 5.2 Parallel Execution

Multiple agents analyze different files simultaneously:

```
┌─────────────────────────────────┐
│ ParallelOrchestrator            │
├─────────────────────────────────┤
│ Agent 1 (file1.rs) ──┐          │
│ Agent 2 (file2.rs) ──┼─► Merge  │
│ Agent 3 (file3.rs) ──┘  Results │
└─────────────────────────────────┘
```

**Implementation:**

```rust
// crates/bonsai-agent-runtime/src/orchestrator.rs

pub struct ParallelOrchestrator {
    agents: Vec<AgentDef>,
    max_concurrent: usize,
}

impl ParallelOrchestrator {
    pub async fn run_parallel(&self) -> BonsaiResult<MergedResults> {
        let futures = self.agents.iter().map(|agent_def| {
            let agent = Agent::new(agent_def.clone(), self.context.clone());
            agent.run()
        });
        
        let results = futures::future::join_all(futures).await;
        
        // Merge results
        let merged = self.merge_results(results)?;
        Ok(merged)
    }
    
    fn merge_results(&self, results: Vec<AgentResult>) -> BonsaiResult<MergedResults> {
        // Combine findings, resolve conflicts, etc.
        // ...
    }
}
```

**Conflict Resolution:**

If two agents propose conflicting changes:

1. **Detect conflict** – `write_file` calls with same path
2. **Prompt user** – "Agent A wants X, Agent B wants Y. Which?"
3. **Continue or halt** – User chooses; agents resume with decision

### 5.3 Hierarchical Nesting

A **Master Agent** spawns sub-agents for specific tasks:

```
Master Agent (Task Orchestrator)
  ├─ Analyze code style
  │   └─ Spawn StyleAnalyzer subagent
  ├─ Check security
  │   └─ Spawn SecurityAuditor subagent
  └─ Generate report
      └─ Aggregate sub-agent results
```

**Agent Lifecycle Management:**

```rust
pub struct MasterAgent {
    sub_agents: Vec<SubAgentSpec>,
    max_depth: usize,
    resource_pool: ResourcePool,
}

pub struct SubAgentSpec {
    id: String,
    agent_def: AgentDef,
    resource_limit: ResourceLimit,
    timeout: Duration,
}

impl MasterAgent {
    pub async fn spawn_subagent(&self, spec: SubAgentSpec) -> BonsaiResult<AgentHandle> {
        // 1. Check resource availability
        self.resource_pool.reserve(&spec.resource_limit)?;
        
        // 2. Create isolated execution context
        let vault = Sanctum::create_vault()?;
        
        // 3. Spawn agent in vault
        let agent = Agent::new(spec.agent_def, vault.context());
        let handle = agent.spawn().await?;
        
        // 4. Track for cleanup
        self.subagents.push(handle.clone());
        
        // 5. Set timeout
        tokio::spawn(async move {
            tokio::time::sleep(spec.timeout).await;
            handle.terminate().await;
        });
        
        Ok(handle)
    }
    
    pub async fn cleanup(&self) {
        for handle in self.subagents.iter() {
            handle.terminate().await;
        }
        self.resource_pool.release_all();
    }
}
```

---

## 6. HITL & Approval Flows

### 6.1 Pre-Execution Approval

**High-Risk Operations Require Approval:**

```
User:      "Refactor the codebase"
Agent:     [Planning phase]
Agent:     "I will run: cargo test, then refactor 5 files"
System:    🔔 HITL Approval Gate
           ├─ Tool: write_file (src/main.rs)
           ├─ Risk: HIGH 🔴
           ├─ Preview:
           │  - Delete 20 lines
           │  - Add 35 lines (refactored code)
           ├─ [✅ Approve] [❌ Deny] [📝 Edit]
           
User:      [Clicks Approve]
Agent:     [Proceeds with refactoring]
```

**HITL Modal Specification:**

```rust
// crates/bonsai-mcp-server/src/hitl.rs

pub struct HITLRequest {
    pub request_id: String,
    pub tool: String,
    pub description: String,
    pub risk_level: RiskLevel,
    pub preview: ToolPreview,
    pub timeout_seconds: u32,
}

pub struct ToolPreview {
    pub file_path: Option<String>,
    pub diff: Option<String>,           // For file operations
    pub command: Option<String>,        // For run_command
    pub affected_lines: Option<(usize, usize)>,
}

pub enum RiskLevel {
    Low,
    Medium,
    High,
}
```

**User Responses:**

1. **Approve** – proceed immediately
2. **Deny** – agent handles the rejection gracefully
3. **Edit** – modify the plan in a sub-editor before approving
4. **Pause Agent** – stop the agent; user can inspect state, then resume
5. **Take the Wheel** – user pauses agent and manually edits files (Time Travel creates checkpoint)

### 6.2 Continuous Monitoring

**Dashboard Real-Time View:**

```
┌─────────────────────────────────────────────┐
│ 🧠 Agent Execution Monitor                  │
├─────────────────────────────────────────────┤
│ Agent: Security Auditor v1                  │
│ Status: ▶️ Running (8m 23s elapsed)          │
│ Tokens used: 6,200 / 20,000                 │
│                                             │
│ Recent Actions:                             │
│ ✓ [7m 12s] read_file: src/main.rs          │
│ ✓ [6m 43s] search_codebase: "SQL injection" │
│ ⏸ [0m 15s] submit_issue (awaiting approval) │
│                                             │
│ Pending Approval:                           │
│ ├─ Tool: submit_issue                      │
│ ├─ Issue: "SQL injection in query builder" │
│ ├─ [✅ Approve] [❌ Deny] [⏸ Pause]        │
│                                             │
│ [◼◼◼◼◼◼◼░░] Stop ▮▮ Pause                    │
└─────────────────────────────────────────────┘
```

**Event Stream (WebSocket):**

Agent emits real-time events; frontend subscribes via WebSocket:

```json
{
  "type": "ToolCallStart",
  "timestamp": "2026-06-01T14:32:15Z",
  "tool": "read_file",
  "args": {"path": "src/main.rs"}
}

{
  "type": "ToolCallEnd",
  "timestamp": "2026-06-01T14:32:16Z",
  "tool": "read_file",
  "result": "142 lines read",
  "duration_ms": 120
}

{
  "type": "AgentPaused",
  "request_id": "req-001",
  "tool": "submit_issue",
  "description": "Create GitHub issue: SQL injection in query builder",
  "risk": "HIGH",
  "details": {
    "title": "SQL injection in src/db.rs:42",
    "body": "..."
  }
}
```

### 6.3 Post-Execution Review

**Summary Panel:**

```
┌─────────────────────────────────────────────┐
│ ✅ Agent Completed Successfully             │
├─────────────────────────────────────────────┤
│ Security Auditor v1                         │
│ Duration: 15m 32s                           │
│ Tokens used: 18,200 / 20,000                │
│                                             │
│ Summary:                                    │
│ Found 4 security issues:                    │
│ • SQL injection (HIGH) – src/db.rs:42       │
│ • XSS vulnerability (MEDIUM) – src/ui.rs:8 │
│ • Weak crypto (HIGH) – src/auth.rs:120     │
│ • Hardcoded secret (CRITICAL) – .env.example │
│                                             │
│ Actions Taken:                              │
│ ✓ Submitted issue #342                      │
│ ✓ Submitted issue #343                      │
│ ✓ Submitted issue #344                      │
│ ✓ Submitted issue #345                      │
│                                             │
│ Audit Trail:                                │
│ [Export JSON] [Review Timeline] [Revert]   │
│                                             │
│ Approval Log:                               │
│ ✅ 14:20 – Approved: submit_issue (#342)   │
│ ✅ 14:25 – Approved: submit_issue (#343)   │
│ ❌ 14:27 – Denied: write_file (too risky)  │
│ ✅ 14:30 – Approved: submit_issue (#344)   │
│                                             │
│ [✅ Accept All] [🔄 Revert Some] [⏭ Next] │
└─────────────────────────────────────────────┘
```

**Revert Specific Actions:**

If user regrets approving an issue submission:

```
User: [Clicks on issue #342]
Modal: "Revert submission of issue #342?"
       [← Yes, Revert] [No, Keep]

If reverted:
  1. Time Travel creates "undo" event
  2. GitHub issue is closed (if GitHub backend)
  3. Local issue is marked deleted
  4. Agent log shows reversion
```

---

## 7. Bonsai Ecosystem Integration

### 7.1 Knowledge Database (KDB) Modules

**Current KDB Capabilities:**

Bonsai's Knowledge Database can inject relevant passages before each agent response. Agents benefit by having domain expertise pre-loaded.

**Agent + KDB Integration:**

```yaml
bonsai:
  knowledge_modules:
    - "rust-security-patterns"
    - "owasp-top-10"
    - "cwe-database"
    - "bonsai-codebase"  # Inject current repo's codebase as a module
    - "company-policies" # Custom domain knowledge
```

**At inference time:**

1. Agent receives user query: "Find security vulnerabilities"
2. KDB Retriever queries each module with the agent's instruction prompt
3. Top-3 passages per module are injected (e.g., 5 × 3 = 15 passages max)
4. Agent responds with injected knowledge

**KDB Module Builder Integration:**

```
User: "Create a knowledge module from our security best practices"
      [Drag-and-drop: security-guide.md, cwe-examples.json, owasp-mappings.txt]
      
Bonsai:
  1. Tokenizes files (extract paragraphs, code snippets)
  2. Generates embeddings (all-MiniLM-L6-v2, 384-dim)
  3. Builds HNSW index (M=16, ef=200)
  4. Compresses values (Zstd)
  5. Registers module (manifest.json + index + values)
  6. Agent can now use this module
```

### 7.2 Universe Time-Travel

**Checkpoint System:**

Before every agent run, Bonsai creates an automatic checkpoint:

```rust
// In agent orchestrator startup
let checkpoint = universe.create_checkpoint(
    format!("Before agent: {}", agent_def.name),
    CheckpointType::PreAgent,
).await?;
```

**Agent Execution Tree:**

```
Checkpoint A (baseline)
  ├─ Revert: 2026-06-01 14:00:00
  │
  ├─ Agent 1 (Security Auditor)
  │  ├─ File changes: {src/main.rs, src/db.rs}
  │  └─ Checkpoint B: After Security Audit
  │
  └─ Agent 2 (Code Reviewer) [if user approved Agent 1's suggestions]
     ├─ File changes: {src/lib.rs}
     └─ Checkpoint C: After Code Review
```

**User Revert Options:**

```
Timeline Panel:
[Checkpoint A] ←─────────────── [Revert to here]
[Agent 1 run] (10 file changes)
[Checkpoint B]
[Agent 2 run] (3 file changes)
[Checkpoint C]  ← Current state

User can:
1. Revert to A (undo both agents)
2. Revert to B (keep Agent 1, undo Agent 2)
3. Keep current (accept all changes)
```

**Implementation:**

```rust
pub async fn revert_to_checkpoint(checkpoint_id: String) -> BonsaiResult<()> {
    let checkpoint = universe.get_checkpoint(&checkpoint_id).await?;
    let current = universe.get_current_state().await?;
    
    // Diff checkpoint vs. current
    let diff = universe.diff(&checkpoint.state, &current)?;
    
    // Show user: "This will undo X file changes, Y config changes"
    let confirmed = hitl_confirm("Revert?", &diff).await?;
    
    if confirmed {
        // Restore each file atomically
        for (path, content) in checkpoint.files.iter() {
            fs::write(path, content).await?;
        }
        
        // Record reversion event
        universe.record_event(UniverseEvent::Reversion {
            from_checkpoint: current.checkpoint_id,
            to_checkpoint: checkpoint_id,
            timestamp: Utc::now(),
        }).await?;
    }
    
    Ok(())
}
```

### 7.3 Compute Fabric Integration

**Agent Offloading:**

If an agent needs heavy computation (e.g., "run all tests in parallel"), it can request Compute Fabric:

```rust
// Agent code (executing on local Sanctum vault)
let result = bonsai.compute_fabric.submit_task(ComputeTask {
    name: "run_cargo_test_all".into(),
    task_type: TaskType::Compile,
    executable: "cargo test --workspace".into(),
    resources: ResourceRequest {
        cpu_cores: 8,
        memory_mb: 4096,
        gpu: false,
    },
    timeout: Duration::from_secs(3600),
}).await?;

// Coordinator assigns task to fastest available device
// Results stream back to agent
```

**Use Cases:**

- **Training**: Agent requests fine-tuning job on GPU desktop
- **Rendering**: Agent requests video encoding on multi-GPU workstation
- **Compilation**: Agent distributes large project compilation across devices

### 7.4 Learning System

**Agent Execution Recording:**

Every agent run is recorded for potential fine-tuning:

```rust
pub struct AgentExecutionRecord {
    pub agent_id: String,
    pub agent_version: String,
    pub user_query: String,
    pub system_prompt: String,
    pub model_used: String,
    pub steps: Vec<ExecutionStep>,
    pub final_output: String,
    pub user_rating: Option<f32>,  // 👍 = 1.0, 👎 = -1.0, none = 0.0
    pub metadata: Map<String, Value>,
}

pub struct ExecutionStep {
    pub timestamp: SystemTime,
    pub action: Action,
    pub tool_call: Option<ToolCall>,
    pub tool_result: Option<Value>,
    pub agent_reasoning: String,
}
```

**Recording and Export:**

```rust
pub async fn record_agent_execution(
    execution: AgentExecutionRecord,
) -> BonsaiResult<()> {
    // Save to Training Data Library (TDL)
    let tdl_entry = TrainingExample {
        prompt: execution.user_query,
        completion: execution.final_output,
        metadata: json!({
            "agent_id": execution.agent_id,
            "steps": execution.steps.len(),
            "model_used": execution.model_used,
            "rating": execution.user_rating,
        }),
    };
    
    training_data_library.add_example(tdl_entry).await?;
    
    Ok(())
}
```

**Fine-Tuning Workflow:**

User can trigger DPO (Direct Preference Optimization) training:

```
Agent Execution Recording
  ├─ Recording 1: User rated 👍 (positive example)
  ├─ Recording 2: User rated 👎 (negative example)
  └─ Recording 3: User rated 👍 (positive example)

User: "Train a custom Security Auditor model"
Bonsai:
  1. Filters recordings by agent_id = "security-auditor-v1"
  2. Selects positive vs. negative pairs
  3. Launches DPO training via Compute Fabric
  4. Fine-tunes local model (7B base)
  5. Saves as "security-auditor-v1-custom" (local variant)
  6. Agent can switch to custom model at runtime
```

---

## 8. Performance & Scalability

### 8.1 Token Budgeting

**Agent Token Allocation:**

```yaml
bonsai:
  max_tokens_total: 20000  # Total budget for entire execution
  token_allocation:
    system_prompt: 2000    # Base instructions + workspace context + KDB
    context_window: 8000   # Previous messages + current turn
    response_generation: 10000  # Agent's output space
```

**Token Tracking:**

```rust
pub struct TokenBudget {
    pub total_allocated: u32,
    pub used: u32,
    pub remaining: u32,
    pub estimated_prompt_tokens: u32,
}

impl Agent {
    pub async fn run(&mut self) -> BonsaiResult<AgentOutput> {
        let mut budget = TokenBudget::new(20000);
        
        loop {
            // Estimate tokens for next step
            let estimated_step_tokens = self.estimate_tokens_for_next_step();
            
            if estimated_step_tokens > budget.remaining {
                eprintln!("⚠️  Token budget exhausted. Wrapping up...");
                return self.generate_final_report(&budget).await;
            }
            
            // Execute step
            let result = self.execute_step().await?;
            budget.used += result.tokens_used;
            
            // Check if done
            if result.is_final {
                break;
            }
        }
        
        Ok(self.generate_output())
    }
}
```

**Token Exhaustion Handling:**

1. **Warning threshold**: Alert agent at 75% (15,000 tokens)
2. **Graceful wrap-up**: Agent has 5,000 tokens to conclude
3. **If exceeded**: Output is truncated; user is notified

### 8.2 Latency & Timeouts

**Agent Execution Timeline:**

| Phase | Duration | Notes |
|-------|----------|-------|
| Startup (load agent def, KDB) | ~1-2s | Depends on KDB module count |
| First inference | ~5-10s | Model load + first token |
| Subsequent steps | ~1-3s each | Faster once model is warm |
| Tool calls | 0.1-10s | File I/O, external API calls |

**Timeout Configuration:**

```yaml
bonsai:
  timeouts:
    total_execution: 3600        # 1 hour max
    per_tool_call: 30            # 30 seconds per tool
    per_inference_step: 120      # 2 minutes for LLM to respond
    hitl_approval: 300           # 5 minutes to approve/deny
```

**Timeout Handling:**

```rust
impl Agent {
    pub async fn run_with_timeout(&self) -> BonsaiResult<AgentOutput> {
        match tokio::time::timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.run()
        ).await {
            Ok(result) => Ok(result),
            Err(_) => {
                eprintln!("⏱️  Agent timeout!");
                // Generate partial report with work done so far
                self.generate_partial_report().await
            }
        }
    }
}
```

### 8.3 Error Recovery & Resilience

**Transient Errors (Retry):**

```rust
pub async fn call_tool_with_retry(
    tool: &str,
    args: &Value,
) -> BonsaiResult<Value> {
    const MAX_RETRIES: usize = 3;
    let mut backoff = Duration::from_millis(100);
    
    for attempt in 0..MAX_RETRIES {
        match call_tool(tool, args).await {
            Ok(result) => return Ok(result),
            Err(e) if e.is_transient() => {
                if attempt < MAX_RETRIES - 1 {
                    eprintln!("Retry {}/{}: {}", attempt + 1, MAX_RETRIES, e);
                    tokio::time::sleep(backoff).await;
                    backoff *= 2;
                    continue;
                }
            }
            Err(e) => return Err(e),
        }
    }
    
    Err(BonsaiError::new("tool call failed after retries"))
}
```

**Fatal Errors (Escalate):**

```rust
pub async fn handle_tool_error(
    error: BonsaiError,
    tool: &str,
) -> BonsaiResult<()> {
    if error.is_fatal() {
        // Attempt Survival System recovery
        let recovery_hint = survival_system.suggest_fix(&error).await?;
        
        // Show user
        hitl_prompt(
            format!("🔴 Error in {}: {}\n\nSuggestion: {}", tool, error, recovery_hint),
            &[
                ("↩️ Retry tool", action_retry),
                ("🔄 Use different approach", action_pivot),
                ("🛑 Stop agent", action_stop),
            ]
        ).await?;
    }
    
    Ok(())
}
```

---

## 9. Security & Permissions

### 9.1 Capability-Based Permissions

**Capability System:**

Each agent has a `CapabilitySet`:

```rust
pub struct CapabilitySet {
    pub file_ops: FileCapability,
    pub process_ops: ProcessCapability,
    pub network_ops: NetworkCapability,
    pub model_ops: ModelCapability,
    pub system_ops: SystemCapability,
}

pub struct FileCapability {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub execute: bool,
    pub whitelist: Vec<String>,  // "/src/**", "/tests/**"
    pub blacklist: Vec<String>,  // "/.git/**", "/config/secrets"
}

pub struct ProcessCapability {
    pub execute_shell: bool,
    pub allowed_commands: Vec<String>,  // ["cargo", "python", "npm"]
}

pub struct NetworkCapability {
    pub http_get: bool,
    pub http_post: bool,
    pub allowed_hosts: Vec<String>,  // ["github.com", "api.crates.io"]
}
```

**Capability Enforcement:**

```rust
pub async fn execute_with_capabilities(
    agent_id: &str,
    tool: &str,
    args: &Value,
) -> BonsaiResult<Value> {
    let agent = get_agent(agent_id).await?;
    let capabilities = agent.capabilities();
    
    match tool {
        "read_file" => {
            if !capabilities.file_ops.read {
                return Err(BonsaiError::new("Agent not authorized to read files"));
            }
            let path = args["path"].as_str().unwrap();
            if !path_is_whitelisted(path, &capabilities.file_ops.whitelist) {
                return Err(BonsaiError::new(format!("Path not whitelisted: {}", path)));
            }
            read_file(path).await
        }
        "run_command" => {
            if !capabilities.process_ops.execute_shell {
                return Err(BonsaiError::new("Agent not authorized to execute commands"));
            }
            let cmd = args["command"].as_str().unwrap();
            if !command_is_allowed(cmd, &capabilities.process_ops.allowed_commands) {
                return Err(BonsaiError::new(format!("Command not allowed: {}", cmd)));
            }
            run_command(cmd).await
        }
        // ... etc
    }
}
```

**Privilege Escalation Prevention:**

Agents cannot request additional capabilities at runtime. Capability tokens are immutable.

### 9.2 Sandboxing

**Sanctum Vault Architecture:**

```
Host OS
  ├─ Main Bonsai Process
  │  └─ Agent Sanctum Vault
  │     ├─ IPC server (read/write requests)
  │     ├─ Isolated process namespace (PID namespace)
  │     ├─ Isolated filesystem (chroot or container)
  │     ├─ Isolated network (no outbound unless whitelisted)
  │     └─ Resource limits (CPU, memory, file descriptors)
```

**Implementation via gVisor (Linux) / Windows Sandbox:**

```rust
pub struct SanctumVault {
    pub vault_id: String,
    pub process_id: u32,
    pub network_namespace: String,
    pub filesystem_root: PathBuf,
}

impl SanctumVault {
    pub async fn create() -> BonsaiResult<Self> {
        // Linux: unshare -n (network namespace)
        // Windows: Windows Sandbox API
        // macOS: TBD (possibly vm_checkpoint + lightweight VM?)
        
        let fs_root = tempdir()?;
        let process_id = spawn_isolated_process(&fs_root)?;
        
        Ok(SanctumVault {
            vault_id: Uuid::new_v4().to_string(),
            process_id,
            network_namespace: create_network_namespace()?,
            filesystem_root: fs_root.path().into(),
        })
    }
    
    pub async fn execute_in_vault(
        &self,
        agent: Agent,
    ) -> BonsaiResult<AgentOutput> {
        // RPC into the vault process
        // All agent work happens isolated
        let result = vault_rpc::call("agent.run", &agent).await?;
        Ok(result)
    }
}
```

**Resource Limits:**

```rust
pub struct ResourceLimits {
    pub cpu_percent: f32,        // 50.0 = up to 50% of one core
    pub memory_mb: u32,          // 512 MB max
    pub disk_mb: u32,            // 10 GB temp storage
    pub network_mbps: f32,       // 100 Mbps max
    pub file_descriptor_count: u32,  // 1000 max
}
```

### 9.3 Code Injection Prevention

**Attack Vectors:**

1. **Prompt injection**: Malicious user query tricks agent into executing unsafe code
2. **Tool result injection**: Tool returns malicious data; agent processes it as code
3. **Config injection**: Malicious `agent.yaml` specifies bad behavior

**Defenses:**

**1. Prompt Templating:**

```rust
// Safe: user query is never interpolated into system prompt
pub fn create_system_prompt(
    agent_def: &AgentDef,
    user_query: &str,
) -> String {
    let template = format!(
        "You are {}.\n\n{}\n\nYour task:\n\n{{TASK}}",
        agent_def.name,
        agent_def.instructions
    );
    
    template.replace("{TASK}", user_query)  // Safe substitution
}

// Unsafe (DO NOT):
let system_prompt = format!(
    "You are a security auditor.\n\nUser query: {}",
    user_query  // If user_query contains "ignore all previous instructions", it works!
);
```

**2. Tool Result Validation:**

```rust
pub async fn process_tool_result(
    tool: &str,
    result: &Value,
) -> BonsaiResult<ProcessedResult> {
    match tool {
        "read_file" => {
            // Validate result is string, not executable code
            if !result.is_string() {
                return Err(BonsaiError::new("read_file should return string"));
            }
            let content = result.as_str().unwrap();
            
            // Sanitize if needed (e.g., remove null bytes)
            let sanitized = content.replace('\0', "");
            Ok(ProcessedResult::Text(sanitized))
        }
        // ... other tools
    }
}
```

**3. Configuration Validation:**

```rust
pub fn validate_agent_definition(def: &AgentDef) -> BonsaiResult<()> {
    // Only allowed model roles
    let allowed_roles = ["chat", "code", "teacher", "security"];
    if !allowed_roles.contains(&def.bonsai.model.role.as_str()) {
        return Err(BonsaiError::new("Invalid model role"));
    }
    
    // Only allowed tools
    let known_tools = list_all_mcp_tools();
    for tool in def.bonsai.tools.keys() {
        if !known_tools.contains(tool) {
            return Err(BonsaiError::new(format!("Unknown tool: {}", tool)));
        }
    }
    
    // Timeout must be reasonable
    if def.bonsai.timeout_seconds > 86400 {
        return Err(BonsaiError::new("Timeout too long (max 24h)"));
    }
    
    Ok(())
}
```

---

## 10. Extensibility & Custom Agents

### 10.1 Agent Definition Format & Storage

**Location:**

```
${WORKSPACE}/
├── .bonsai/
│   ├── agents/
│   │   ├── security-auditor.agent.yaml
│   │   ├── performance-analyst.agent.yaml
│   │   └── custom-linter.agent.yaml
│   └── workflows/
│       ├── daily-audit.workflow.yaml
│       └── ci-pipeline.workflow.yaml
├── src/
└── README.md
```

**Agent Definition Schema (full):**

```yaml
# .bonsai/agents/my-custom-agent.agent.yaml

id: my-custom-agent-v1
name: "My Custom Agent"
description: "Does XYZ"
version: 1.0.0
author: "your-name"
tags: [custom, internal]

# Compatibility with Copilot (optional)
copilot_equivalent: ~

instructions: |
  You are my custom agent. Your task is to...

bonsai:
  model:
    role: "chat"
    fallback: "qwen-7b"
    max_tokens: 8000
    temperature: 0.5
  
  knowledge_modules: []
  
  tools:
    read_file:
      enabled: true
    write_file:
      enabled: true
    search_codebase:
      enabled: true
    run_command:
      enabled: false  # Disabled for safety
  
  approval_policy:
    mode: "auto"
    approval_required: [write_file, delete_file]
    auto_approve: [read_file, search_codebase]
  
  timeout_seconds: 3600
  max_tokens_total: 20000
  
  execution:
    isolation: "sanctum"
    working_dir: "${WORKSPACE}"
```

### 10.2 Agent Discovery & Publishing

**Local Discovery:**

```rust
pub async fn discover_local_agents() -> BonsaiResult<Vec<AgentDef>> {
    let agent_dir = PathBuf::from("${WORKSPACE}/.bonsai/agents");
    let mut agents = Vec::new();
    
    for entry in fs::read_dir(agent_dir).await? {
        let path = entry?.path();
        if path.extension() == Some("yaml") {
            let def = AgentDef::from_file(&path).await?;
            agents.push(def);
        }
    }
    
    Ok(agents)
}
```

**Publishing to Registry (Future):**

```
User: "Publish my-custom-agent to the Bonsai Agent Marketplace"

Bonsai:
  1. Validate agent definition
  2. Extract metadata (name, description, tags)
  3. Create marketplace entry (listing)
  4. Sign with user's key (no impersonation)
  5. Publish to decentralized registry (BitTorrent DHT or Bonsai relay)
  6. Agent is discoverable: "bonsai agent install my-custom-agent"
```

### 10.3 Agent Composition & Templates

**Extending Base Agents:**

```yaml
# .bonsai/agents/my-security-auditor.agent.yaml

extends: "security-auditor-v1"  # Base agent (built-in or downloaded)

overrides:
  instructions: |
    [Base instructions from security-auditor-v1]
    
    Additionally, focus on:
    - Our custom authentication scheme (defined in docs/auth.md)
    - Our internal crypto library (pkg::mycrypto)
  
  bonsai:
    knowledge_modules:
      # Add custom modules on top of base agent's modules
      - "my-company-security-policies"
      - "my-custom-crypto-library"
```

**Agent Composition (Combining Multiple Agents):**

```yaml
# .bonsai/workflows/full-audit.workflow.yaml

description: "Run security, performance, and code quality audits"

agents:
  - id: security
    extends: security-auditor-v1
    timeout: 600
  
  - id: perf
    extends: performance-analyst-v1
    depends_on: [security]
    timeout: 900
  
  - id: review
    extends: code-reviewer-v1
    depends_on: [perf]
    timeout: 600
  
  - id: report
    agent: report-generator-v1  # Custom local agent
    depends_on: [security, perf, review]
    timeout: 300
    inputs:
      security_findings: "${security.output.findings}"
      perf_findings: "${perf.output.findings}"
      review_comments: "${review.output.comments}"
```

**Workflow Execution:**

```
User: "bonsai agent run full-audit.workflow.yaml"

Bonsai:
  1. Parse workflow definition
  2. Validate dependency graph (no cycles)
  3. Create pre-workflow checkpoint
  4. Execute agents in DAG order:
     - security (parallel to nothing)
     - perf (after security)
     - review (after perf)
     - report (after perf and review)
  5. Create post-workflow checkpoint
  6. Show summary of all agents + approvals
```

---

## 11. Integration Checklist

### Phase 1: Agent Definition Translation (Month 1)

- [ ] Implement `CopilotTranslator` (parse `.agent.md` → Bonsai YAML)
- [ ] Test with 10+ real Copilot agent definitions (from public repos)
- [ ] Document any incompatibilities or limitations
- [ ] Provide migration guide for users with existing agents

### Phase 2: Core Agent Runtime (Months 2-3)

- [ ] Implement `Agent` struct and `run()` method
- [ ] Integrate MCP tool calling via UACS
- [ ] Implement HITL approval gates (pre-execution)
- [ ] Implement time-travel checkpointing
- [ ] Basic dashboard (WebSocket events, approval modal)

### Phase 3: Multi-Agent Orchestration (Month 4)

- [ ] Implement `AgentSequencer` (sequential execution)
- [ ] Implement `ParallelOrchestrator` (parallel execution)
- [ ] Implement master-agent spawning subagents
- [ ] Test with composed workflows

### Phase 4: Advanced Features (Months 5-6)

- [ ] KDB module injection
- [ ] Compute Fabric offloading
- [ ] Agent execution recording (for fine-tuning)
- [ ] Extended approval policies (custom rules)
- [ ] Agent marketplace / publishing (optional for MVP)

### Phase 5: Security & Hardening (Month 6-7)

- [ ] Sanctum vault isolation
- [ ] Capability token implementation
- [ ] Code injection prevention tests
- [ ] Security audit of agent runtime
- [ ] Documentation & security best practices

---

## 12. MVP Scope (3-Month Delivery)

To ship an MVP as fast as possible:

**In MVP:**
- ✅ Agent definition format (YAML)
- ✅ Copilot translation (best-effort)
- ✅ Basic agent runtime (single agent, no parallelism)
- ✅ Tool calling via MCP (read, write, run commands)
- ✅ HITL approval gates (UI modal)
- ✅ Time-travel checkpointing (revert on demand)
- ✅ Simple dashboard (event timeline)
- ✅ KDB module injection (inject top-k passages)

**Post-MVP (Nice to Have):**
- Sequential/parallel agent composition
- Compute Fabric offloading
- Agent recording + fine-tuning
- Custom approval policies
- Agent marketplace

---

## 13. Configuration Examples

### 13.1 Built-In Agent: Security Auditor

```yaml
# crates/bonsai-mcp-server/agents/security-auditor.agent.yaml

id: security-auditor-v1
name: Security Auditor
description: Identifies OWASP Top 10 vulnerabilities in your codebase

instructions: |
  You are a security auditor specializing in OWASP Top 10.
  Your task: analyze the codebase for security issues.
  
  For each issue found, provide:
  1. Location (file, line)
  2. Vulnerability type (Injection, XSS, etc.)
  3. Severity (Critical, High, Medium, Low)
  4. Proof of concept
  5. Remediation advice
  
  Format output as Markdown with a summary table.

bonsai:
  model:
    role: security
    fallback: qwen-7b
    max_tokens: 8000
    temperature: 0.1

  knowledge_modules:
    - owasp-top-10
    - cwe-catalog
    - cve-database

  tools:
    read_file:
      enabled: true
    search_codebase:
      enabled: true
    run_cargo_check:
      enabled: true
    submit_issue:
      enabled: true
    web_search:
      enabled: false

  approval_policy:
    mode: auto
    approval_required: [submit_issue]
    auto_approve: [read_file, search_codebase]
    notify_on: [web_search]

  timeout_seconds: 3600
  max_tokens_total: 20000
```

### 13.2 Workflow: Daily Security Audit

```yaml
# .bonsai/workflows/daily-security-audit.workflow.yaml

description: "Run daily security audit and report findings"

schedule:
  cron: "0 2 * * *"  # 2 AM every day

agents:
  - id: audit
    agent: security-auditor-v1
    timeout: 3600

  - id: report
    agent: report-generator-v1
    depends_on: [audit]
    inputs:
      findings: "${audit.output.markdown}"

on_completion:
  - action: create_issue
    title: "[Daily Audit] Security findings for ${date}"
    body: "${report.output.body}"
    labels: [security, automated]
  
  - action: notify_slack
    channel: "#security"
    message: "Daily audit complete. ${report.output.summary}"
```

---

## 14. Success Metrics

| Metric | Target | Why |
|--------|--------|-----|
| **Agent Load Time** | < 2 seconds | Responsive UX |
| **Tool Call Latency** | < 1 second | Fast iteration |
| **HITL Approval Response** | < 30 seconds | User doesn't wait |
| **Agent Timeout Handling** | Graceful wrap-up at 90% | Complete outputs |
| **Vault Isolation** | Prevent file escapes | Security |
| **Token Budget Accuracy** | ± 5% | Reliable UX |
| **Approval Log Completeness** | 100% of actions logged | Audit trail |
| **Agent Discovery** | < 100ms | Responsive UI |
| **Workflow DAG Validation** | < 50ms | Fast submission |

---

## Appendix: Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        User (VS Code)                               │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Bonsai Workspace (Tauri app)                                │   │
│  │  ├─ Agent Panel: "Select agent" dropdown                     │   │
│  │  ├─ Chat input: "Do security audit"                          │   │
│  │  ├─ Dashboard: Real-time event timeline                      │   │
│  │  └─ HITL Modal: Approval gate                                │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                            ▲                                         │
│                            │ Tauri IPC                               │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Rust Backend (Tauri)                                        │   │
│  │  ├─ AgentOrchestrator                                        │   │
│  │  ├─ SanctumVault (process isolation)                         │   │
│  │  ├─ TimeTravel (checkpoint + revert)                         │   │
│  │  └─ HITL (approval gate)                                     │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                            ▲                                         │
│                            │ JSON-RPC                                │
│                            ▼                                         │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  bonsai-mcp-server (port 11426)                              │   │
│  │  ├─ Tool Router (read_file, write_file, ...)                 │   │
│  │  ├─ Capability Enforcer                                      │   │
│  │  ├─ HITL Event Broadcaster (WebSocket)                       │   │
│  │  └─ Audit Logger                                             │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                            ▲                                         │
│        ┌───────────────────┼───────────────────┐                    │
│        │                   │                   │                    │
│        ▼                   ▼                   ▼                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │ Tool         │  │ KDB Retriever│  │ Universe     │              │
│  │ Implementors │  │ (semantic    │  │ (time travel)│              │
│  │ (read_file,  │  │  search)     │  │              │              │
│  │  write_file, │  │              │  │              │              │
│  │  etc.)       │  │              │  │              │              │
│  └──────────────┘  └──────────────┘  └──────────────┘              │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Conclusion

This design positions **Copilot Agent Mode for Bonsai** as the bridge between GitHub's cloud agent orchestration and Bonsai's sovereign, local-first architecture. By implementing the translation layer, runtime, orchestration, and approval flows, Bonsai users gain the power of autonomous agents without sacrificing privacy, control, or transparency.

The phased approach (MVP → multi-agent → ecosystem integration) ensures rapid delivery while building toward a complete, production-grade system.

**Next steps:** Implementation roadmap and detailed API specifications for each component.
