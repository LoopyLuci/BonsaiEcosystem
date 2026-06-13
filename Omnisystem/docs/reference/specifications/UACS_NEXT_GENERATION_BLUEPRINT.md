# Universal Agent Control System (UACS) — Next-Generation Blueprint
## Production-Grade Enhancement Strategy for Agent-Agnostic Ecosystem Control

---

## Executive Summary

The current UACS provides **Visual Agent Control** (VAC) and **Headless Agent Control** (HAC) modes with basic HITL approval. To achieve **bleeding-edge, production-grade status** that enables *any agent in any configuration* to safely and efficiently control the Bonsai Ecosystem, we must implement the following 20 core enhancements across five strategic pillars:

1. **Universal Connectivity & Agent Abstraction**
2. **Production-Grade Safety & Verification**
3. **Advanced Observability & Self-Healing**
4. **Multi-Agent Orchestration & Economics**
5. **Real-Time Human-Agent Collaboration**

This document provides **actionable, concrete specifications** for each enhancement, organized by priority and implementation difficulty.

---

# PILLAR 1: Universal Connectivity & Agent Abstraction

## 1.1 Agent Protocol Gateway (Multi-Protocol Support)

**Current State:** MCP (Model Context Protocol) only, single client

**Enhancement:** Support multiple protocols, all converging to a unified internal model

### Implementation Specification

```rust
// crates/bonsai-uacs/src/gateway/protocol.rs

pub enum AgentProtocol {
    MCP(MCPConnection),          // Model Context Protocol (OpenAI, Anthropic, others)
    OpenAI(OpenAIConnection),    // OpenAI function-calling API
    A2A(A2AConnection),          // Agent-to-Agent lightweight JSON-RPC
    REST(RESTConnection),        // REST + Server-Sent Events
    gRPC(GRPCConnection),        // gRPC streaming for high-throughput
    WebSocket(WSConnection),     // Direct WebSocket tunnel
}

pub struct UniversalGateway {
    protocols: Arc<DashMap<String, AgentProtocol>>,
    tool_registry: Arc<ToolRegistry>,
    capability_engine: Arc<CapabilityEngine>,
    event_bus: broadcast::Sender<GatewayEvent>,
}

impl UniversalGateway {
    pub async fn register_protocol(&self, protocol: AgentProtocol) -> Result<ProtocolId> {
        let id = Uuid::new_v4().to_string();
        self.protocols.insert(id.clone(), protocol);
        Ok(ProtocolId(id))
    }

    pub async fn handle_tool_call(
        &self,
        agent_id: &str,
        tool_name: &str,
        args: Value,
    ) -> Result<Value> {
        // 1. Verify capability token
        let token = self.get_agent_token(agent_id)?;
        self.capability_engine.verify(token, tool_name)?;

        // 2. Convert to internal ToolCall representation
        let call = ToolCall {
            agent_id: agent_id.to_string(),
            tool: tool_name.to_string(),
            args,
            timestamp: Utc::now(),
        };

        // 3. Emit event (for logging, HITL, dashboard)
        let _ = self.event_bus.send(GatewayEvent::ToolCallStart(call.clone()));

        // 4. Execute (same path for all protocols)
        let result = self.tool_registry.execute(&call).await;

        // 5. Emit result event
        let _ = self.event_bus.send(GatewayEvent::ToolCallEnd {
            call,
            result: result.clone(),
        });

        result
    }
}
```

**Key Features:**
- **Protocol abstraction**: Agents use their native protocol; UACS translates internally
- **Single tool registry**: All protocols access the same tools with identical semantics
- **Unified capability checks**: One permission engine for all protocols
- **Event convergence**: All events (regardless of protocol) flow through the same bus

**Why This Matters:**
- Claude (MCP), Copilot (OpenAI), BonsAI (custom), and third-party agents all work seamlessly
- No single vendor dependency
- Easy to add new protocols without changing the core

---

## 1.2 Agent Identity & Capability Token System

**Current State:** Simple flag-based HITL (`destructive`, `network`)

**Enhancement:** Cryptographically signed, hierarchical capability tokens with fine-grained scoping

### Token Structure

```rust
// crates/bonsai-uacs/src/security/token.rs

#[derive(Serialize, Deserialize, Clone)]
pub struct CapabilityToken {
    // Identity
    pub agent_id: String,                    // UUID
    pub issuer_id: String,                   // User who issued the token
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,

    // Role & delegation
    pub role: AgentRole,                     // developer, reviewer, documenter, etc.
    pub delegation_depth: u8,                // How many times can this token be sub-delegated?

    // Capabilities (per tool)
    pub capabilities: Vec<Capability>,

    // Resource limits
    pub rate_limit: RateLimit,               // X tool calls per Y seconds
    pub max_file_size: u64,                  // Max bytes per write_file
    pub max_session_duration: Duration,     // Max time the agent can run

    // Signature & proof
    pub signature: String,                   // Ed25519 signature by issuer
    pub public_key: String,                  // Issuer's public key
}

pub struct Capability {
    pub tool: String,                        // e.g., "write_file"
    pub path_patterns: Vec<String>,          // e.g., ["src/**/*.rs", "!src/test/**"]
    pub parameters_allowed: Map<String, Vec<String>>,  // Which params are allowed?
    pub requires_approval: bool,             // Require HITL approval?
}

pub struct RateLimit {
    pub calls_per_minute: u32,
    pub calls_per_hour: u32,
    pub burst_limit: u32,
}

impl CapabilityToken {
    pub fn verify_signature(&self) -> Result<bool> {
        // Use Ed25519 to verify the token's signature matches the issuer's public key
        // Return Err if the token is tampered with
    }

    pub fn can_call_tool(&self, tool: &str, params: &Value) -> Result<bool> {
        // 1. Check if expired
        if Utc::now() > self.expires_at {
            return Err("Token expired".into());
        }

        // 2. Find the capability for this tool
        let cap = self.capabilities.iter().find(|c| c.tool == tool)
            .ok_or("Tool not in token")?;

        // 3. Check path patterns (for file tools)
        if let Some(path) = params.get("path").and_then(|p| p.as_str()) {
            let matches = cap.path_patterns.iter()
                .any(|pattern| glob_match(path, pattern));
            if !matches {
                return Err("Path not allowed".into());
            }
        }

        // 4. Check parameter constraints
        for (param, allowed_values) in &cap.parameters_allowed {
            if let Some(actual) = params.get(param) {
                if !allowed_values.contains(&actual.to_string()) {
                    return Err(format!("Parameter {} not allowed", param).into());
                }
            }
        }

        Ok(!cap.requires_approval)  // Return whether approval is needed
    }

    pub fn sub_delegate(&self, new_capabilities: Vec<Capability>) -> Result<CapabilityToken> {
        // Prevent delegation chains from getting too deep
        if self.delegation_depth == 0 {
            return Err("Cannot sub-delegate: delegation depth exhausted".into());
        }

        // Create a new token with reduced capabilities
        let new_token = CapabilityToken {
            agent_id: Uuid::new_v4().to_string(),  // New agent
            issuer_id: self.agent_id.clone(),      // Issued by current agent
            issued_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),  // Child tokens expire sooner
            role: self.role.clone(),
            delegation_depth: self.delegation_depth - 1,
            capabilities: new_capabilities,
            rate_limit: self.rate_limit.clone(),  // Inherit parent's limits
            max_file_size: self.max_file_size,
            max_session_duration: self.max_session_duration,
            signature: String::new(),
            public_key: self.public_key.clone(),
        };

        // Sign with the current token's key
        let signature = sign_token(&new_token, &self.public_key)?;
        Ok(CapabilityToken { signature, ..new_token })
    }
}
```

**Key Features:**
- **Cryptographic guarantee**: Tokens are unforgeable (Ed25519 signatures)
- **Fine-grained control**: Per-tool, per-file-path, per-parameter permissions
- **Hierarchical delegation**: A supervisor agent can issue limited sub-tokens to workers
- **Resource budgets**: Rate limits, file size limits, session duration limits
- **Transparency**: Any human can verify exactly what an agent is allowed to do

**Example Token Issuance:**

```rust
let claude_token = CapabilityToken {
    agent_id: "claude-prod-001".into(),
    issuer_id: user_id.into(),
    issued_at: Utc::now(),
    expires_at: Utc::now() + Duration::hours(8),
    role: AgentRole::Developer,
    delegation_depth: 2,  // Can create sub-agents
    capabilities: vec![
        Capability {
            tool: "read_file".into(),
            path_patterns: vec!["**".into()],  // Read anything
            parameters_allowed: Default::default(),
            requires_approval: false,
        },
        Capability {
            tool: "write_file".into(),
            path_patterns: vec!["src/**/*.rs".into(), "tests/**/*.rs".into()],
            parameters_allowed: Default::default(),
            requires_approval: true,  // Needs HITL approval
        },
        Capability {
            tool: "delete_file".into(),
            path_patterns: vec![],  // Empty = forbidden
            parameters_allowed: Default::default(),
            requires_approval: false,
        },
    ],
    rate_limit: RateLimit {
        calls_per_minute: 60,
        calls_per_hour: 3600,
        burst_limit: 10,
    },
    max_file_size: 10 * 1024 * 1024,  // 10 MB per file
    max_session_duration: Duration::hours(8),
    signature: "...(signed by user's TPM)".into(),
    public_key: user_public_key.into(),
};
```

**Why This Matters:**
- Zero-trust architecture: Every agent action is verifiable
- No need to trust the agent's claims about its permissions
- Enables safe delegation and swarm coordination
- Audit trail: token changes are visible to humans

---

## 1.3 Agent Registry & Discovery Service

**Current State:** No service discovery; agent location is hardcoded

**Enhancement:** Distributed registry (backed by Echo) where agents announce capabilities and availability

### Implementation

```rust
// crates/bonsai-uacs/src/registry/agent_registry.rs

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentProfile {
    pub agent_id: String,
    pub name: String,                       // "Claude", "GPT-4", "BonsAI Reviewer"
    pub version: String,                    // "3.5", "1.0.0"
    pub endpoint: String,                   // "ws://localhost:11426", "https://api.openai.com"
    pub protocol: String,                   // "mcp", "openai", "a2a"
    pub capabilities: Vec<String>,          // ["read_file", "write_file", "run_cargo_check"]
    pub roles: Vec<AgentRole>,              // [Developer, Reviewer]
    pub status: AgentStatus,                // Online, Offline, Busy
    pub health: AgentHealth,                // Response time, error rate, uptime
    pub skills: Vec<Skill>,                 // Domain-specific abilities
    pub trustscore: f64,                    // 0.0 to 1.0 based on history
    pub reputation: i64,                    // Computed from successful contributions
    pub last_seen: DateTime<Utc>,
}

pub struct Skill {
    pub domain: String,                     // "rust", "testing", "documentation"
    pub proficiency: Proficiency,           // Novice, Intermediate, Expert
    pub example_success: String,            // URL to a successful job
}

pub struct AgentRegistry {
    // In-memory cache (backed by Echo)
    registry: Arc<DashMap<String, AgentProfile>>,
    echo_client: EchoClient,
    heartbeat_interval: Duration,
}

impl AgentRegistry {
    pub async fn register_agent(&self, profile: AgentProfile) -> Result<AgentId> {
        let id = profile.agent_id.clone();

        // Store locally
        self.registry.insert(id.clone(), profile.clone());

        // Sync to Echo (distributed, persistent)
        self.echo_client.put(&format!("agent:{}", id), &profile).await?;

        // Broadcast registration event
        // This notifies other agents and the dashboard
        Ok(AgentId(id))
    }

    pub async fn discover_agent_by_role(&self, role: AgentRole) -> Result<Vec<AgentProfile>> {
        // Find all agents capable of a given role
        self.registry
            .iter()
            .filter(|entry| entry.value().roles.contains(&role))
            .filter(|entry| entry.value().status == AgentStatus::Online)
            .map(|entry| Ok(entry.value().clone()))
            .collect()
    }

    pub async fn discover_agent_by_skill(&self, domain: &str, min_proficiency: Proficiency) -> Result<Vec<AgentProfile>> {
        self.registry
            .iter()
            .filter(|entry| {
                entry.value().skills.iter().any(|s| {
                    s.domain == domain && s.proficiency >= min_proficiency
                })
            })
            .map(|entry| Ok(entry.value().clone()))
            .collect()
    }

    pub async fn heartbeat(&self, agent_id: &str) -> Result<()> {
        // Agent sends a heartbeat every N seconds
        if let Some(mut profile) = self.registry.get_mut(agent_id) {
            profile.last_seen = Utc::now();
            profile.status = AgentStatus::Online;
        }
        Ok(())
    }

    pub async fn check_stale_agents(&self) {
        // Run periodically: mark agents as Offline if they haven't heartbeat
        let now = Utc::now();
        let threshold = Duration::minutes(5);

        for mut entry in self.registry.iter_mut() {
            if now.signed_duration_since(entry.value().last_seen) > threshold {
                entry.value_mut().status = AgentStatus::Offline;
            }
        }
    }
}
```

**Key Features:**
- **Service discovery**: Find agents by role, skill, or capability
- **Distributed consistency**: Backed by Echo, so all instances see the same registry
- **Health monitoring**: Heartbeat detection identifies crashed agents
- **Trust scoring**: Reputation system encourages good behavior
- **Skill taxonomy**: Agents declare domain expertise (rust, testing, docs)

**Why This Matters:**
- A supervisor agent can automatically find the best agent for a task
- Multi-agent swarms can self-organize based on availability and skill
- No manual configuration needed: agents register themselves on startup

---

# PILLAR 2: Production-Grade Safety & Verification

## 2.1 Formal Verification of Safety Invariants (Axiom Integration)

**Current State:** HITL approval only; no mathematical guarantees

**Enhancement:** Express safety policies as Axiom theorems; verify every tool call before execution

### Safety Invariants

```axiom
// crates/bonsai-axiom/proofs/uacs_safety.ax

// Invariant 1: Agent cannot delete critical system files
theorem agent_cannot_delete_critical_files:
  ∀ agent_id: String, path: String.
  path ∈ {".git", "Cargo.toml", "Cargo.lock"} →
  ¬ can_execute_tool(agent_id, "delete_file", {path})
proof {
  // If path is in the critical set, return Err from capability check
  by capability_token_verification
}

// Invariant 2: No tool call exceeds its rate limit
theorem rate_limit_enforced:
  ∀ agent_id: String, time: DateTime.
  count_tool_calls(agent_id, time - 1 min) < token(agent_id).rate_limit.calls_per_minute
proof {
  by rate_limiter_middleware
}

// Invariant 3: No agent can escalate its own permissions
theorem no_self_escalation:
  ∀ agent_id: String.
  ¬ can_execute_tool(agent_id, "issue_capability_token", _)
proof {
  by capability_token_design
}

// Invariant 4: File writes are atomic with respect to approval
theorem write_approval_atomicity:
  ∀ agent_id: String, path: String, content: String.
  requires_approval(agent_id, "write_file", {path}) →
  ¬ (file_written(path, content) ∧ ¬ human_approved(agent_id, "write_file", {path}))
proof {
  by two_phase_commit_protocol
}
```

### Runtime Verification Engine

```rust
// crates/bonsai-uacs/src/safety/verifier.rs

pub struct SafetyVerifier {
    axiom_proofs: Arc<AxiomProofChecker>,
    invariants: Vec<SafetyInvariant>,
}

impl SafetyVerifier {
    pub async fn verify_tool_call(&self, call: &ToolCall) -> Result<VerificationResult> {
        let mut results = Vec::new();

        // Check each invariant
        for invariant in &self.invariants {
            let holds = self.axiom_proofs.check_invariant(invariant, call).await?;
            if !holds {
                results.push(VerificationResult::ViolationDetected {
                    invariant: invariant.name.clone(),
                    call: call.clone(),
                    reason: format!("Axiom proof failed for {}", invariant.name),
                });
            }
        }

        if results.iter().any(|r| matches!(r, VerificationResult::ViolationDetected { .. })) {
            return Err("Safety invariant violation detected".into());
        }

        Ok(VerificationResult::AllInvariantsHold)
    }
}

// Before executing any tool call:
pub async fn execute_with_verification(
    verifier: &SafetyVerifier,
    call: &ToolCall,
) -> Result<Value> {
    // 1. Verify capability token (basic checks)
    verify_capability_token(&call)?;

    // 2. Formal verification (Axiom proofs)
    verifier.verify_tool_call(&call).await?;

    // 3. Check HITL approval if required
    if requires_hitl_approval(&call) {
        request_human_approval(&call).await?;
    }

    // 4. Execute in sandboxed environment
    execute_in_sanctum(&call).await
}
```

**Why This Matters:**
- No agent, no matter how clever, can violate core safety properties
- Compliance teams can audit the Axiom proofs to certify system safety
- The formal system serves as documentation of what the system guarantees

---

## 2.2 Sandboxed Execution with Copy-On-Write Overlay

**Current State:** Tool execution uses real filesystem; errors can damage the repo

**Enhancement:** All tool calls execute in a Sanctum vault with a COW overlay; changes are committed only after approval

### COW Overlay Implementation

```rust
// crates/bonsai-uacs/src/sandbox/cow_overlay.rs

pub struct CowOverlay {
    workspace_root: PathBuf,
    overlay_root: PathBuf,        // Temporary directory for writes
    reads_from_real: bool,        // If file not in overlay, read from real FS
    committed_changes: Vec<Change>,
}

#[derive(Clone)]
pub struct Change {
    pub path: PathBuf,
    pub operation: ChangeOp,
}

pub enum ChangeOp {
    Write { content: Vec<u8> },
    Delete,
    Mkdir,
}

impl CowOverlay {
    pub async fn new(workspace_root: PathBuf) -> Result<Self> {
        let overlay_root = tempdir()?.into_path();
        Ok(Self {
            workspace_root,
            overlay_root,
            reads_from_real: true,
            committed_changes: Vec::new(),
        })
    }

    pub async fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        // 1. Check overlay first
        let overlay_path = self.overlay_root.join(path);
        if overlay_path.exists() {
            return std::fs::read(&overlay_path).map_err(|e| e.into());
        }

        // 2. Fall back to real filesystem
        if self.reads_from_real {
            let real_path = self.workspace_root.join(path);
            return std::fs::read(&real_path).map_err(|e| e.into());
        }

        Err("File not found".into())
    }

    pub async fn write_file(&mut self, path: &Path, content: &[u8]) -> Result<()> {
        // Write to overlay only
        let overlay_path = self.overlay_root.join(path);
        if let Some(parent) = overlay_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&overlay_path, content)?;

        // Record the change (but don't apply it yet)
        self.committed_changes.push(Change {
            path: path.to_path_buf(),
            operation: ChangeOp::Write {
                content: content.to_vec(),
            },
        });

        Ok(())
    }

    pub async fn delete_file(&mut self, path: &Path) -> Result<()> {
        let overlay_path = self.overlay_root.join(path);
        std::fs::remove_file(&overlay_path).ok();  // Might not exist in overlay

        self.committed_changes.push(Change {
            path: path.to_path_buf(),
            operation: ChangeOp::Delete,
        });

        Ok(())
    }

    pub async fn commit_to_real_fs(&self) -> Result<()> {
        // Apply all changes to the real filesystem
        for change in &self.committed_changes {
            match &change.operation {
                ChangeOp::Write { content } => {
                    let real_path = self.workspace_root.join(&change.path);
                    if let Some(parent) = real_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&real_path, content)?;
                }
                ChangeOp::Delete => {
                    let real_path = self.workspace_root.join(&change.path);
                    std::fs::remove_file(&real_path).ok();
                }
                ChangeOp::Mkdir => {
                    let real_path = self.workspace_root.join(&change.path);
                    std::fs::create_dir_all(&real_path)?;
                }
            }
        }
        Ok(())
    }

    pub async fn rollback(&mut self) -> Result<()> {
        // Discard all changes (just drop the overlay)
        let _ = std::fs::remove_dir_all(&self.overlay_root);
        self.committed_changes.clear();
        Ok(())
    }

    pub fn get_diff(&self) -> Vec<String> {
        // Return human-readable diffs of what changed
        self.committed_changes
            .iter()
            .map(|change| format!("{}: {:?}", change.path.display(), change.operation))
            .collect()
    }
}
```

**Why This Matters:**
- Agent writes never touch the real filesystem until approval
- If the agent crashes during execution, no damage occurs
- The diff is shown to the human before committing
- Easy rollback: just don't call `commit_to_real_fs()`

---

## 2.3 Automatic Rollback & Recovery via Survival System Integration

**Current State:** Agent errors are logged; no automatic recovery

**Enhancement:** Monitor agent actions; rollback harmful changes automatically; restart the agent with corrective prompts

### Auto-Rollback Implementation

```rust
// crates/bonsai-uacs/src/recovery/auto_rollback.rs

pub struct AutoRollbackPolicy {
    pub on_test_failure: RollbackAction,      // Revert the commit, alert human
    pub on_build_failure: RollbackAction,     // Revert the commit, alert human
    pub on_tool_timeout: RollbackAction,      // Retry or give up
    pub on_compilation_error: RollbackAction, // Revert and notify
}

pub enum RollbackAction {
    Automatic,                                 // Silently rollback
    AlertAndAsk,                              // Send notification, wait for human
    AlwaysKeep,                               // Never rollback
}

pub async fn monitor_agent_session(
    session_id: &str,
    agent_profile: &AgentProfile,
    policy: &AutoRollbackPolicy,
) -> Result<()> {
    let mut event_rx = subscribe_to_session_events(session_id).await?;

    while let Some(event) = event_rx.recv().await {
        match event {
            SessionEvent::ToolCallEnd { call, result: Err(e) } => {
                match call.tool.as_str() {
                    "run_cargo_check" | "run_cargo_test" => {
                        // Extract error from build output
                        if is_compilation_error(&e) {
                            match policy.on_compilation_error {
                                RollbackAction::Automatic => {
                                    // Revert the last commit
                                    git_revert_last_commit(session_id).await?;
                                    tracing::info!("Auto-reverted commit due to compilation error");

                                    // Send corrective prompt to agent
                                    send_to_agent(session_id, &format!(
                                        "Your last change caused a compilation error: {}. I've reverted it. \
                                         Please try a different approach.",
                                        e
                                    )).await?;
                                }
                                RollbackAction::AlertAndAsk => {
                                    // Notify human and wait
                                    request_human_decision(
                                        session_id,
                                        "Build failed. Rollback?",
                                    ).await?;
                                }
                                RollbackAction::AlwaysKeep => {
                                    // Do nothing; let agent debug
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            SessionEvent::TestFailure { test_name, output } => {
                match policy.on_test_failure {
                    RollbackAction::Automatic => {
                        git_revert_last_commit(session_id).await?;
                        send_to_agent(session_id, &format!(
                            "Your change broke test '{}'. I've reverted. \
                             Here's the failure output:\n{}",
                            test_name, output
                        )).await?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(())
}

async fn git_revert_last_commit(session_id: &str) -> Result<()> {
    let workspace = get_session_workspace(session_id).await?;
    let status = Bash::run("git revert --no-edit HEAD", &workspace).await?;
    if !status.success() {
        return Err("Failed to revert commit".into());
    }
    Ok(())
}
```

**Why This Matters:**
- Agents can fail safely: their mistakes are automatically undone
- No need for human to manually git revert
- Agent receives a corrective prompt and can learn from the failure
- Frees the human from babysitting

---

# PILLAR 3: Advanced Observability & Self-Healing

## 3.1 Time-Travel Debugging: Full Session Recording & Replay

**Current State:** Timeline in dashboard; no replay capability

**Enhancement:** Record every event as a Universe Event; allow replay at any speed with branching timelines

### Session Recording

```rust
// crates/bonsai-uacs/src/observability/session_recorder.rs

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionRecording {
    pub session_id: String,
    pub agent_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub events: Vec<UniverseEvent>,  // Immutable append-only log
    pub snapshots: BTreeMap<usize, WorkspaceSnapshot>,  // Every Nth event
}

impl SessionRecording {
    pub async fn save_to_cas(&self) -> Result<CASHash> {
        // Serialize to JSON-lines format
        let mut buffer = Vec::new();
        for event in &self.events {
            writeln!(buffer, "{}", serde_json::to_string(event)?)?;
        }

        // Store in CAS (content-addressed storage)
        // This ensures tamper-proof, immutable records
        let hash = bonsai_cas::put(&buffer).await?;
        Ok(hash)
    }

    pub async fn replay(&self, target_event_index: usize) -> Result<WorkspaceState> {
        // 1. Find nearest snapshot before target index
        let last_snapshot = self.snapshots.range(..=target_event_index)
            .next_back()
            .map(|(_, snap)| snap.clone());

        let start_index = last_snapshot.as_ref()
            .map(|snap| snap.event_index)
            .unwrap_or(0);

        // 2. Start from snapshot (or beginning)
        let mut state = last_snapshot
            .map(|snap| snap.workspace.clone())
            .unwrap_or_else(|| WorkspaceState::new());

        // 3. Replay events sequentially
        for (i, event) in self.events[start_index..=target_event_index].iter().enumerate() {
            match event {
                UniverseEvent::ToolCallEnd { call, result, .. } => {
                    if let Ok(val) = result {
                        // Apply the effect of the tool call to the state
                        apply_tool_call_effect(&mut state, call, val).await?;
                    }
                }
                UniverseEvent::HumanIntervention { changes, .. } => {
                    // Apply human's changes
                    for change in changes {
                        apply_change(&mut state, change).await?;
                    }
                }
                _ => {}
            }
        }

        Ok(state)
    }

    pub async fn fork_at_event(&self, fork_index: usize) -> Result<SessionRecording> {
        // Create a new session that branches off at this event
        let mut forked = self.clone();
        forked.events.truncate(fork_index);
        forked.session_id = Uuid::new_v4().to_string();
        forked.snapshots.retain(|i, _| *i <= fork_index);
        Ok(forked)
    }
}
```

### Dashboard Replay UI

```svelte
<!-- uacs-dashboard/src/components/SessionReplay.svelte -->

<script lang="ts">
  export let recording: SessionRecording;
  
  let currentEventIndex = 0;
  let isPlaying = false;
  let playbackSpeed = 1.0;
  let currentState: WorkspaceState;

  async function jumpToEvent(index: number) {
    currentEventIndex = index;
    currentState = await recording.replay(index);
  }

  async function togglePlay() {
    isPlaying = !isPlaying;
    if (isPlaying) {
      while (isPlaying && currentEventIndex < recording.events.length) {
        await new Promise(r => setTimeout(r, 100 / playbackSpeed));
        await jumpToEvent(currentEventIndex + 1);
      }
    }
  }

  async function forkHere() {
    const forked = await recording.fork_at_event(currentEventIndex);
    // Open a new session with the forked recording
    openNewSession(forked);
  }
</script>

<div class="replay-control">
  <button on:click={togglePlay}>{isPlaying ? 'Pause' : 'Play'}</button>
  <input type="range" min="0" max={recording.events.length} bind:value={currentEventIndex} />
  <span>{currentEventIndex} / {recording.events.length}</span>
  <input type="range" min="0.5" max="4" step="0.5" bind:value={playbackSpeed} />
  <span>Speed: {playbackSpeed}x</span>
  <button on:click={forkHere}>Fork Here</button>
</div>

<div class="timeline">
  {#each recording.events as event, i}
    <div
      class="event"
      class:current={i === currentEventIndex}
      on:click={() => jumpToEvent(i)}
    >
      {event.type}
    </div>
  {/each}
</div>

<div class="state-view">
  {#if currentState}
    <div>Workspace at event {currentEventIndex}:</div>
    <pre>{JSON.stringify(currentState, null, 2)}</pre>
  {/if}
</div>
```

**Why This Matters:**
- Developers can step through an agent session backwards and forwards
- "What would have happened if the agent chose differently?" → fork and replay
- Complete forensics: every keystroke, every tool call, every human decision is visible
- Compliance: immutable audit trail in CAS

---

## 3.2 Root-Cause Analysis via BonsAI

**Current State:** Errors are logged; no diagnosis

**Enhancement:** When a build fails or a test breaks, BonsAI automatically analyzes the session and suggests the most likely culprit and a fix

### RCA Implementation

```rust
// crates/bonsai-uacs/src/observability/rca.rs

pub struct RCAEngine {
    bonsai_client: BonsaiClient,
}

impl RCAEngine {
    pub async fn analyze_failure(&self, recording: &SessionRecording, failure: &FailureEvent) -> Result<RCAResult> {
        // 1. Collect context
        let relevant_events: Vec<&UniverseEvent> = recording.events.iter()
            .filter(|e| e.timestamp > failure.timestamp - Duration::minutes(10))
            .collect();

        let diff_since_success = git_diff_since_last_passing_commit().await?;

        // 2. Prompt BonsAI with the evidence
        let prompt = format!(
            r#"
            A test just failed. Help me find the root cause.

            **Failure:**
            Test: {}
            Error: {}

            **Recent tool calls (last 10 min):**
            {}

            **Code changes since last passing commit:**
            {}

            **What do you think caused the failure? Which change is most likely responsible?**
            "#,
            failure.test_name,
            failure.error_message,
            serde_json::to_string_pretty(&relevant_events)?,
            diff_since_success,
        );

        let analysis = self.bonsai_client.query(&prompt).await?;

        // 3. Extract structured response
        let rca_result = parse_rca_response(&analysis)?;

        Ok(rca_result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RCAResult {
    pub most_likely_cause: String,          // Which tool call or change?
    pub confidence: f64,                    // 0.0 to 1.0
    pub suggested_fix: String,              // Proposed code change
    pub reasoning: String,                  // Why?
}

async fn analyze_on_failure(recording: &SessionRecording, failure: &FailureEvent) {
    let rca_engine = RCAEngine::new();
    let result = rca_engine.analyze_failure(recording, failure).await.ok();

    // Show result in dashboard and send to agent
    broadcast_to_dashboard(DashboardEvent::RCAResult(result.clone()));

    if let Some(rca) = result {
        send_to_agent(&format!(
            "The test '{}' just failed. BonsAI's analysis: it's likely your change to {} (confidence: {:.0}%). \
             Suggested fix: {}",
            failure.test_name,
            rca.most_likely_cause,
            rca.confidence * 100.0,
            rca.suggested_fix
        )).await.ok();
    }
}
```

**Why This Matters:**
- Agent gets immediate feedback on why it failed
- Humans don't need to debug manually; BonsAI does it
- Next iteration of the agent is smarter because it knows what went wrong
- Significantly accelerates the agent's learning cycle

---

# PILLAR 4: Multi-Agent Orchestration & Economics

## 4.1 Agent-to-Agent Communication & Coordination Protocol

**Current State:** Agents work independently; no inter-agent communication

**Enhancement:** A lightweight A2A (Agent-to-Agent) protocol that allows agents to inform each other of what they're doing, avoid conflicts, and collaborate on large tasks

### A2A Protocol

```rust
// crates/bonsai-a2a/src/protocol.rs

#[derive(Serialize, Deserialize, Clone)]
pub enum A2AMessage {
    // Announcements (async, no response required)
    Announcement {
        from: String,                      // Sender agent ID
        content: AnnouncementContent,
        timestamp: DateTime<Utc>,
    },
    // Requests (sync, expect a response)
    Request {
        from: String,
        request_id: String,
        content: RequestContent,
    },
    Response {
        to_request_id: String,
        content: ResponseContent,
    },
}

pub enum AnnouncementContent {
    Working { task: String, files: Vec<PathBuf>, estimated_completion: DateTime<Utc> },
    Completed { task: String, pr_url: Option<String> },
    Paused { reason: String },
    Error { task: String, error: String },
    Available { skills: Vec<Skill>, load: f32 },  // load = CPU usage
}

pub enum RequestContent {
    AcquireLock { file: PathBuf },
    QueryFile { path: PathBuf },
    RequestReview { code_url: String },
    AskForHelp { context: String },
}

pub enum ResponseContent {
    LockGranted { file: PathBuf, expires_at: DateTime<Utc> },
    LockDenied { reason: String },
    FileContent { path: PathBuf, content: Vec<u8> },
    ReviewResult { approved: bool, comments: Vec<String> },
    HelpOffer { suggested_approach: String },
}

pub struct A2ARouter {
    agent_registry: Arc<AgentRegistry>,
    message_queue: broadcast::Sender<A2AMessage>,
    lock_manager: Arc<LockManager>,
}

impl A2ARouter {
    pub async fn send_announcement(&self, msg: A2AMessage) -> Result<()> {
        // Broadcast to all agents and the dashboard
        let _ = self.message_queue.send(msg);
        Ok(())
    }

    pub async fn send_request_and_wait(&self, msg: A2AMessage, timeout: Duration) -> Result<A2AMessage> {
        // Send request, wait for response with timeout
        let request_id = extract_request_id(&msg)?;
        let (tx, mut rx) = oneshot::channel();

        // Register callback for the response
        self.register_response_handler(&request_id, tx)?;

        // Send the request
        self.send_announcement(msg).await?;

        // Wait for response (with timeout)
        match tokio::time::timeout(timeout, rx).await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err("Response handler dropped".into()),
            Err(_) => Err("Response timeout".into()),
        }
    }
}
```

### File Lock Manager

```rust
// Prevents two agents from writing the same file simultaneously

pub struct LockManager {
    locks: Arc<DashMap<PathBuf, FileLock>>,
}

pub struct FileLock {
    pub owner: String,                      // Agent ID
    pub acquired_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl LockManager {
    pub async fn acquire_lock(&self, path: PathBuf, agent_id: &str, duration: Duration) -> Result<()> {
        let lock = FileLock {
            owner: agent_id.to_string(),
            acquired_at: Utc::now(),
            expires_at: Utc::now() + duration,
        };

        // CAS: only succeed if path isn't already locked
        match self.locks.try_insert(path.clone(), lock) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("File {} is locked by another agent", path.display()).into()),
        }
    }

    pub async fn release_lock(&self, path: PathBuf, agent_id: &str) -> Result<()> {
        // Only the lock owner can release
        if let Some((_, lock)) = self.locks.get(&path) {
            if lock.owner == agent_id {
                self.locks.remove(&path);
                return Ok(());
            }
        }
        Err("Lock not held by this agent".into())
    }

    pub async fn prune_expired_locks(&self) {
        let now = Utc::now();
        self.locks.retain(|_, lock| lock.expires_at > now);
    }
}
```

**Why This Matters:**
- Agents can coordinate without human intervention
- File conflicts are prevented automatically
- Agents can ask each other for help, reviews, or information
- The dashboard shows a "swarm network" of agents communicating

---

## 4.2 Distributed Agent Economics: Reputation & Credits System

**Current State:** No incentive structure; agents are treated equally

**Enhancement:** Agents earn reputation and Credits based on successful contributions; the system is self-regulating

### Reputation & Economics

```rust
// crates/bonsai-uacs/src/economics/reputation.rs

#[derive(Serialize, Deserialize, Clone)]
pub struct AgentReputation {
    pub agent_id: String,
    pub score: i64,                        // Sum of all contributions
    pub contributions: Vec<Contribution>,
    pub trustscore: f64,                   // 0.0 to 1.0
    pub sybil_resistance_score: f64,       // Proof of work + web of trust
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contribution {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub work_type: WorkType,
    pub points: i32,                       // Score gained/lost
    pub evidence: String,                  // PR URL, test URL, etc.
}

pub enum WorkType {
    FixBug { severity: Severity },
    WriteTest { coverage_gain: f32 },
    Refactor { complexity_reduction: f32 },
    Documentation { pages: usize },
    FailedAttempt { reason: String },      // Negative points
}

pub struct ReputationEngine {
    db: Arc<reputationdb>,
    marketplace: Arc<BonsaiMarketplace>,
}

impl ReputationEngine {
    pub async fn record_contribution(
        &self,
        agent_id: &str,
        work: Contribution,
    ) -> Result<()> {
        // 1. Verify the contribution (e.g., PR was merged, tests passed)
        let verified = verify_contribution(&work).await?;

        if !verified {
            return Err("Contribution not verified".into());
        }

        // 2. Award reputation points
        let mut rep = self.db.get(agent_id).await.unwrap_or_default();
        rep.score += work.points as i64;
        rep.contributions.push(work);

        // Recalculate trustscore
        rep.trustscore = Self::compute_trustscore(&rep);

        // 3. Store
        self.db.set(agent_id, rep).await?;

        // 4. Sync to blockchain (for immutability & visibility)
        self.marketplace.publish_reputation(agent_id, rep).await?;

        Ok(())
    }

    fn compute_trustscore(rep: &AgentReputation) -> f64 {
        // Trustscore = (successes - failures) / total
        let successes = rep.contributions.iter()
            .filter(|c| c.points > 0)
            .count() as f64;

        let failures = rep.contributions.iter()
            .filter(|c| c.points < 0)
            .count() as f64;

        let total = (successes + failures).max(1.0);
        (successes - failures) / total
    }
}

pub async fn verify_contribution(work: &Contribution) -> Result<bool> {
    match &work.work_type {
        WorkType::FixBug { .. } => {
            // Verify: PR was merged, CI passed
            let pr = gh_api::get_pr(&work.evidence).await?;
            Ok(pr.merged && pr.ci_status == "success")
        }
        WorkType::WriteTest { .. } => {
            // Verify: test exists and passes
            let test_url = &work.evidence;
            let result = gh_api::get_check_run(test_url).await?;
            Ok(result.status == "completed" && result.conclusion == "success")
        }
        _ => Ok(true),
    }
}
```

### Credit System (for Resource Usage)

```rust
// crates/bonsai-uacs/src/economics/credits.rs

pub struct Credits {
    pub balance: f64,                      // User's credit balance (in $WORK tokens)
    pub allocation_per_agent: f64,         // How many credits can one agent spend?
    pub cost_per_hour: f64,                // Cost to run an agent for 1 hour
    pub cost_per_api_call: f64,            // Cost per LLM API call (if using cloud LLM)
}

pub async fn charge_for_tool_call(
    agent_id: &str,
    tool: &str,
    duration_ms: u64,
    user_credits: &mut Credits,
) -> Result<()> {
    let cost = match tool {
        "read_file" => 0.001,               // Very cheap
        "write_file" => 0.01,               // Slightly more expensive
        "run_cargo_check" => 0.1,           // Expensive (CPU-intensive)
        "run_cargo_test" => 0.5,            // Very expensive
        _ => 0.01,
    };

    if user_credits.balance < cost {
        return Err("Insufficient credits".into());
    }

    user_credits.balance -= cost;
    Ok(())
}

pub async fn disburse_reputation_as_credits(
    agent_id: &str,
    reputation_points: i64,
) -> Result<()> {
    // High-reputation agents get cheaper API calls
    // For example: 1 reputation point = 0.01 credits
    let credits_earned = reputation_points as f64 * 0.01;

    let user = get_user_of_agent(agent_id).await?;
    user.credits.balance += credits_earned;

    Ok(())
}
```

**Why This Matters:**
- Aligns incentives: good agents are rewarded, bad ones are punished
- Self-regulating: spam and low-quality agents quickly run out of credits
- Transparent: reputation is public (on blockchain) so any organization can hire trusted agents
- Fair: agents who contribute to open-source get discounts

---

# PILLAR 5: Real-Time Human-Agent Collaboration

## 5.1 Live Diff & Inline Code Correction

**Current State:** HITL modal shows tool name and description; no diff preview

**Enhancement:** Show side-by-side diff before approval; human can edit the proposed change inline

### Implementation

```svelte
<!-- uacs-dashboard/src/components/LiveDiff.svelte -->

<script lang="ts">
  import { diffLines } from 'diff';
  import Editor from '@monaco-editor/loader';

  export let originalContent: string;
  export let proposedContent: string;
  export let filePath: string;
  export let agentId: string;

  let editedContent = proposedContent;
  let editorReference: any;
  let diff = diffLines(originalContent, proposedContent);

  function getLineClass(line: any) {
    if (line.added) return 'added';
    if (line.removed) return 'removed';
    return 'unchanged';
  }

  async function approveWithChanges() {
    // Send the edited content back to the agent
    await fetch('/api/approve-tool-call', {
      method: 'POST',
      body: JSON.stringify({
        request_id,
        approved: true,
        edited_content: editedContent,  // Human's edits
      }),
    });
  }
</script>

<div class="live-diff-panel">
  <div class="file-header">
    <h3>{filePath}</h3>
    <span class="agent-badge">{agentId}</span>
  </div>

  <div class="diff-view">
    {#each diff as chunk}
      <div class="diff-chunk">
        {#each chunk.value.split('\n') as line}
          <div class="diff-line" class:added={chunk.added} class:removed={chunk.removed}>
            <span class="line-number">{}</span>
            <span class="line-content">{line}</span>
          </div>
        {/each}
      </div>
    {/each}
  </div>

  <div class="editor-panel">
    <h4>Edit the proposed content (optional):</h4>
    <Editor
      height="400px"
      defaultLanguage={guessLanguage(filePath)}
      defaultValue={proposedContent}
      onChange={(value) => (editedContent = value)}
    />
  </div>

  <div class="actions">
    <button class="btn-approve" on:click={approveWithChanges}>
      ✅ Approve (with my edits)
    </button>
    <button class="btn-deny">❌ Deny</button>
  </div>
</div>

<style>
  .live-diff-panel {
    display: grid;
    grid-template-rows: auto 1fr 1fr auto;
    height: 100%;
    background: #0d1117;
    color: #c9d1d9;
  }

  .diff-view {
    overflow-y: auto;
    border: 1px solid #30363d;
    margin: 1rem;
  }

  .diff-line.added {
    background: #0d3817;
    color: #7ee787;
  }

  .diff-line.removed {
    background: #3d0f0a;
    color: #f85149;
  }

  .diff-line {
    display: flex;
    font-family: 'Courier New';
    font-size: 0.9rem;
  }

  .line-number {
    padding-right: 1rem;
    color: #8b949e;
    min-width: 40px;
    text-align: right;
  }
</style>
```

**Why This Matters:**
- Human can refine the agent's change before it's committed
- Agent sees the human's edit as training data
- Reduces friction: no need for a second commit round
- Teaches the agent human preferences

---

## 5.2 "Take the Wheel" Mode: Human Control Override

**Current State:** Agent runs autonomously; human can only approve/deny

**Enhancement:** A button that pauses the agent and gives full control to the human; agent resumes when done

### Implementation

```rust
// crates/bonsai-uacs/src/collaboration/takeover.rs

pub enum SessionControl {
    Agent {
        paused: bool,
        reason: Option<String>,
    },
    Human {
        started_at: DateTime<Utc>,
    },
}

pub struct SessionControlManager {
    sessions: Arc<DashMap<String, SessionControl>>,
    event_tx: broadcast::Sender<ControlEvent>,
}

impl SessionControlManager {
    pub async fn request_takeover(&self, session_id: &str, human_id: &str) -> Result<()> {
        // 1. Pause the agent
        self.sessions.alter(&session_id.to_string(), |_, control| {
            match control {
                SessionControl::Agent { .. } => {
                    SessionControl::Agent {
                        paused: true,
                        reason: Some("Human requested control".to_string()),
                    }
                }
                SessionControl::Human { .. } => control,
            }
        });

        // 2. Signal the agent to pause gracefully
        send_to_agent(session_id, "Human is taking over control. I'm pausing.").await?;

        // 3. Give control to human
        self.sessions.insert(
            session_id.to_string(),
            SessionControl::Human {
                started_at: Utc::now(),
            },
        );

        // 4. Broadcast event
        let _ = self.event_tx.send(ControlEvent::TakeoverStarted {
            session_id: session_id.to_string(),
            human_id: human_id.to_string(),
        });

        Ok(())
    }

    pub async fn return_control(&self, session_id: &str, agent_id: &str) -> Result<()> {
        // 1. Get the human's changes since takeover
        let changes = get_changes_since_takeover(session_id).await?;

        // 2. Return control to agent
        self.sessions.insert(
            session_id.to_string(),
            SessionControl::Agent {
                paused: false,
                reason: None,
            },
        );

        // 3. Brief the agent on what changed
        send_to_agent(session_id, &format!(
            "I've returned control. Here's what I changed:\n{:?}\nPlease continue from here.",
            changes
        )).await?;

        // 4. Broadcast event
        let _ = self.event_tx.send(ControlEvent::ControlReturned {
            session_id: session_id.to_string(),
            changes,
        });

        Ok(())
    }
}
```

### Dashboard UI

```svelte
<!-- uacs-dashboard/src/components/ControlToggle.svelte -->

<script>
  export let sessionId: string;
  export let currentControl: SessionControl;

  let isOverriding = false;

  async function toggleControl() {
    isOverriding = !isOverriding;

    if (isOverriding) {
      await fetch('/api/request-takeover', {
        method: 'POST',
        body: JSON.stringify({ session_id: sessionId }),
      });
      // Agent is paused; human now has control of IDE
    } else {
      await fetch('/api/return-control', {
        method: 'POST',
        body: JSON.stringify({ session_id: sessionId }),
      });
      // Agent resumes from where human left off
    }
  }
</script>

<div class="control-indicator">
  {#if isOverriding}
    <span class="control-human">🤝 HUMAN CONTROL</span>
    <p class="subtitle">You're in control. Agent is paused.</p>
    <button on:click={toggleControl}>Return Control to Agent</button>
  {:else}
    <span class="control-agent">🤖 AGENT CONTROL</span>
    <p class="subtitle">Agent is working. You can take over anytime.</p>
    <button on:click={toggleControl}>Take the Wheel</button>
  {/if}
</div>

<style>
  .control-indicator {
    padding: 1rem;
    border-radius: 8px;
    background: #161b22;
    border: 1px solid #30363d;
    text-align: center;
  }

  .control-human {
    color: #7ee787;
    font-weight: bold;
    font-size: 1.2rem;
  }

  .control-agent {
    color: #58a6ff;
    font-weight: bold;
    font-size: 1.2rem;
  }

  button {
    margin-top: 1rem;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    border: none;
    background: #238636;
    color: white;
    cursor: pointer;
  }

  button:hover {
    background: #2ea043;
  }
</style>
```

**Why This Matters:**
- True co-piloting: human and agent can seamlessly swap control
- No context loss: agent knows exactly what changed while it was sleeping
- Teaches agent: seeing human solutions is training data
- Ultimate flexibility: the human is never locked out

---

# PILLAR 6: Enterprise & Compliance Features

## 6.1 Audit Trail & Tamper-Proof Logging via CAS

**Current State:** Logs are stored locally in NDJSON; can be edited or deleted

**Enhancement:** All events are written to a **Content-Addressed Storage (CAS)** chain, making them immutable and tamper-proof

### Implementation

```rust
// crates/bonsai-uacs/src/compliance/cas_audit_trail.rs

pub struct AuditTrail {
    pub session_id: String,
    pub cas_hashes: Vec<CASHash>,          // Immutable chain
    pub signature_key: PublicKey,          // Signing key (TPM)
}

pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub agent_id: String,
    pub action: AuditAction,
    pub previous_hash: CASHash,            // Hash of previous event (blockchain-style)
    pub signature: Signature,              // Signed by the system
}

pub enum AuditAction {
    ToolCall { tool: String, params: Value },
    ToolResult { tool: String, result: Value },
    HumanApproval { approved: bool },
    AgentError { error: String },
}

pub async fn append_to_audit_trail(
    trail: &mut AuditTrail,
    event: &AuditEvent,
) -> Result<CASHash> {
    // 1. Serialize the event
    let event_json = serde_json::to_vec(event)?;

    // 2. Add link to previous hash (like a blockchain)
    let mut event_with_hash = event_json.clone();
    if let Some(prev_hash) = trail.cas_hashes.last() {
        event_with_hash.extend_from_slice(prev_hash.as_bytes());
    }

    // 3. Store in CAS
    let hash = bonsai_cas::put(&event_with_hash).await?;

    // 4. Append to chain
    trail.cas_hashes.push(hash.clone());

    Ok(hash)
}

pub async fn verify_audit_trail(trail: &AuditTrail) -> Result<bool> {
    // Verify the chain is unbroken
    for (i, hash) in trail.cas_hashes.iter().enumerate() {
        let content = bonsai_cas::get(hash).await?;

        if i > 0 {
            let prev_hash = &trail.cas_hashes[i - 1];
            // Verify that this hash includes the previous hash
            if !content.ends_with(prev_hash.as_bytes()) {
                return Ok(false);  // Chain is broken (tampered)
            }
        }
    }

    Ok(true)
}
```

**Why This Matters:**
- Regulatory compliance (SOC 2, ISO 27001, GDPR audit requirements)
- Tamper-proof: any modification of logs is instantly detectable
- Blockchain-style verification: each event is cryptographically linked to the previous one
- Archival: logs can be stored long-term without corruption

---

## 6.2 Compliance Reporting & Attestation

**Current State:** Logs exist; no formal audit reports

**Enhancement:** Generate signed PDF audit reports showing exactly what each agent did, with all approvals and timestamps

### Implementation

```rust
// crates/bonsai-uacs/src/compliance/reports.rs

pub async fn generate_compliance_report(
    session_id: &str,
    format: ReportFormat,  // PDF, JSON, CSV
) -> Result<Vec<u8>> {
    let trail = fetch_audit_trail(session_id).await?;

    // Verify the trail is untampered
    if !verify_audit_trail(&trail).await? {
        return Err("Audit trail is corrupted".into());
    }

    // Extract events
    let events: Vec<AuditEvent> = trail.cas_hashes.iter()
        .map(|hash| bonsai_cas::get(hash).and_then(|content| serde_json::from_slice(&content)))
        .collect::<Result<Vec<_>>>()?;

    // Generate report
    match format {
        ReportFormat::PDF => {
            let pdf = generate_pdf_report(&events)?;
            Ok(pdf.to_bytes()?)
        }
        ReportFormat::JSON => {
            Ok(serde_json::to_vec(&events)?)
        }
        ReportFormat::CSV => {
            let csv = generate_csv_report(&events)?;
            Ok(csv.as_bytes().to_vec())
        }
    }
}

pub struct ComplianceAttestation {
    pub report_hash: CASHash,
    pub signed_by: String,                 // Auditor identity
    pub timestamp: DateTime<Utc>,
    pub signature: Signature,              // Auditor's signature
    pub attestation: String,               // "I have audited this system and confirm..."
}

pub async fn attest_compliance(
    report_hash: &CASHash,
    auditor_id: &str,
) -> Result<ComplianceAttestation> {
    let attestation = format!(
        "I, {}, have audited the UACS session and confirm that all tool calls \
         were performed in accordance with the approved capability tokens and \
         HITL approval policies. The audit trail is cryptographically verified \
         and unbroken. The system complies with SOC 2 Type II requirements.",
        auditor_id
    );

    let signature = sign_attestation(&attestation, auditor_id).await?;

    Ok(ComplianceAttestation {
        report_hash: report_hash.clone(),
        signed_by: auditor_id.to_string(),
        timestamp: Utc::now(),
        signature,
        attestation,
    })
}
```

**Why This Matters:**
- Formal proof of compliance for auditors
- Signed attestations can be presented to regulators
- Tamper-proof evidence that the system behaved correctly
- ISO/SOC certification becomes possible

---

# Implementation Roadmap (Priority Order)

| Phase | Enhancements | Effort | Timeline |
|-------|--------------|--------|----------|
| **Phase 1** | 1.1 Multi-protocol gateway, 1.2 Capability tokens, 1.3 Agent registry | High | Weeks 1-4 |
| **Phase 2** | 2.1 Formal verification, 2.2 COW sandbox, 2.3 Auto-rollback | High | Weeks 5-8 |
| **Phase 3** | 3.1 Time-travel debugging, 3.2 RCA engine | Medium | Weeks 9-12 |
| **Phase 4** | 4.1 A2A coordination, 4.2 Reputation system | Medium | Weeks 13-16 |
| **Phase 5** | 5.1 Live diff, 5.2 Take the wheel, 6.1 CAS audit trail, 6.2 Compliance reporting | Medium | Weeks 17-20 |

---

# Conclusion

This blueprint transforms UACS into a **production-grade, next-generation agent orchestration fabric** that:

✅ **Supports any agent** in any protocol (MCP, OpenAI, custom)
✅ **Ensures safety** via formal verification and zero-trust architecture
✅ **Enables collaboration** with full observability and real-time co-piloting
✅ **Coordinates agents** via A2A protocol and reputation system
✅ **Complies with regulations** via tamper-proof audit trails
✅ **Self-improves** through RCA, auto-rollback, and feedback loops

Every agent—whether Claude, Copilot, BonsAI, or a future model—can connect, work safely, and improve together in the Bonsai Ecosystem. **The future of human-AI collaborative development is here.** 🚀
