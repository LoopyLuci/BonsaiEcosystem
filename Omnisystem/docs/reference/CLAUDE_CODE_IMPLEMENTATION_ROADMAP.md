# Claude Code Adaptation – Implementation Roadmap

**Design Reference:** [CLAUDE_CODE_ADAPTATION.md](CLAUDE_CODE_ADAPTATION.md)

This roadmap breaks the design into concrete implementation phases with dependencies and effort estimates.

---

## Phase 0: Foundation (Weeks 1-4)

### 0.1 MCP Endpoint Scaffolding

**Goal:** Build the HTTP interface that the extension will call

**Tasks:**
- [ ] Create `crates/bonsai-claude-code/` crate
- [ ] Define endpoint handlers in `mcp_server/src/tools.rs`:
  ```rust
  POST /sessions
  POST /sessions/{id}/edit
  GET /sessions/{id}
  POST /sessions/{id}/checkpoint/create
  GET /sessions/{id}/checkpoint/list
  POST /sessions/{id}/checkpoint/restore
  POST /sessions/{id}/mention-resolve
  POST /sessions/{id}/approve
  DELETE /sessions/{id}
  ```
- [ ] Implement basic request/response serialization (Serde)
- [ ] Add authentication (Bearer token validation)
- [ ] Error handling & HTTP status codes

**Effort:** 2 weeks  
**Dependencies:** MCP server already exists

---

### 0.2 Session State Storage

**Goal:** Persist Claude Code sessions to SQLite with Echo sync

**Tasks:**
- [ ] Create `sessions` table schema in `universe.db`:
  ```sql
  CREATE TABLE claude_code_sessions (
    session_id TEXT PRIMARY KEY,
    workspace_path TEXT NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    model_config JSON,
    approval_policy JSON,
    message_history JSONL,
    universe_sync_event_id TEXT,
    metadata JSON
  )
  ```
- [ ] Implement session CRUD operations (create, read, update, delete)
- [ ] Hook into Echo/CRDT for cross-device sync
- [ ] Implement session migration from old Claude Code format

**Effort:** 1 week  
**Dependencies:** Universe/Echo already exists

---

### 0.3 Basic Edit Request Handler

**Goal:** Accept edit requests, call model, return diffs

**Tasks:**
- [ ] Implement `/sessions/{id}/edit` endpoint
- [ ] Input validation (prompt length, file count)
- [ ] Call model orchestrator (Qwen-7B by default)
- [ ] Parse model response → unified diff
- [ ] Return diff with hunk IDs for approval
- [ ] Emit `FileChangeProposed` event to Universe

**Effort:** 1.5 weeks  
**Dependencies:** Model orchestrator exists

---

### 0.4 Approval Framework

**Goal:** Risk-score operations and enforce approval policies

**Tasks:**
- [ ] Define `ApprovalRisk` enum (Low, Medium, High, Critical)
- [ ] Implement risk scoring function (see section 6.1 of design)
- [ ] Store pending approvals in SQLite
- [ ] Implement `/sessions/{id}/approve` endpoint
- [ ] Log approvals to Universe
- [ ] Test with multiple risk levels

**Effort:** 1 week  
**Dependencies:** Session storage complete

**Output:** By end of Phase 0, extension can send requests and get back unsigned diffs.

---

## Phase 1: Checkpoints & Timeline (Weeks 5-8)

### 1.1 Checkpoint Creation

**Goal:** Map Claude Code checkpoints to Universe snapshots

**Tasks:**
- [ ] Implement `/sessions/{id}/checkpoint/create` endpoint
- [ ] Call `Universe::snapshots::take_snapshot()` with label + metadata
- [ ] Store checkpoint_id → snapshot_id mapping in sessions table
- [ ] Calculate file change summary (added/modified/deleted counts)
- [ ] Return checkpoint metadata to extension

**Effort:** 1 week  
**Dependencies:** Phase 0 complete

---

### 1.2 Checkpoint Restoration

**Goal:** Restore workspace state from a checkpoint

**Tasks:**
- [ ] Implement `/sessions/{id}/checkpoint/restore` endpoint
- [ ] Fetch snapshot from Universe by snapshot_id
- [ ] Diff snapshot.files vs current workspace state
- [ ] Show preview of changes to user (via approval flow)
- [ ] Apply restoration (atomically write all files)
- [ ] Emit `FileChangeApplied` event to Universe
- [ ] Test recovery with corrupted/missing files

**Effort:** 1.5 weeks  
**Dependencies:** Checkpoint creation complete

---

### 1.3 Timeline Query API

**Goal:** Return checkpoint + file change events for UI timeline

**Tasks:**
- [ ] Implement `/sessions/{id}/timeline` endpoint
- [ ] Query Universe for all `FileChange` + `Checkpoint` events
- [ ] Filter by category, file path, time range
- [ ] Sort chronologically with pagination support
- [ ] Include metadata for UI rendering (file counts, sizes)
- [ ] Cache frequent queries (5-minute TTL)

**Effort:** 1 week  
**Dependencies:** Phase 0 + Checkpoint creation complete

---

### 1.4 Extension UI Components

**Goal:** Build Svelte components for diff rendering and timeline

**Tasks:**
- [ ] Parse unified diffs into hunk objects (TypeScript)
- [ ] Render hunks with syntax highlighting (use Monaco)
- [ ] Implement accept/reject buttons per hunk
- [ ] Build timeline visualization (horizontal scrubber)
- [ ] Connect to `/sessions/{id}/checkpoint-list`
- [ ] Test with sample diffs (various languages)

**Effort:** 2 weeks  
**Dependencies:** Phase 0-1 backend complete

**Output:** By end of Phase 1, user can create checkpoints and see timeline. Diffs are rendered inline with accept/reject.

---

## Phase 2: @-mentions (Weeks 9-12)

### 2.1 File & Symbol Mention Resolution

**Goal:** Support original Claude Code @-mention syntax

**Tasks:**
- [ ] Parse `@file.rs`, `@file.rs#L10-50`, `@symbol:function` syntax
- [ ] Resolve file mentions by reading files from disk
- [ ] Resolve symbol mentions via code indexing (if available) or regex fallback
- [ ] Return content chunks with context
- [ ] Implement `/sessions/{id}/mention-resolve` endpoint
- [ ] Inject resolved content into message before sending to model

**Effort:** 1 week  
**Dependencies:** Phase 0 complete

---

### 2.2 Knowledge Module Integration

**Goal:** Support `@knowledge:` mentions via KDB semantic search

**Tasks:**
- [ ] Extend mention parser to recognize `@knowledge:module-name` syntax
- [ ] Implement KDB query in mention-resolve handler
- [ ] Call retriever with user query (editor context)
- [ ] Format top-k results as "Knowledge:" section in system prompt
- [ ] Support custom scoring/filtering (e.g., `@knowledge:rust:5:0.5`)
- [ ] Test with existing knowledge modules (rust-patterns, bonsai-docs)

**Effort:** 1 week  
**Dependencies:** Phase 0, KDB exists

---

### 2.3 Agent Mention Support

**Goal:** Support `@agent:` mentions to run code reviewers, etc.

**Tasks:**
- [ ] Define agent interface (agent_id, input context, output format)
- [ ] Implement lightweight code reviewer agent
- [ ] Implement security auditor agent
- [ ] Add agent spawning to mention-resolve handler
- [ ] Set timeout (5 seconds) for agent completion
- [ ] Format agent output as "Agent findings:" in context
- [ ] Cache agent results for session duration

**Effort:** 2 weeks  
**Dependencies:** Phase 0, agents framework exists

---

### 2.4 Extended Mention Syntax

**Goal:** Add `@module:`, `@git:` syntax

**Tasks:**
- [ ] Parse `@module:domain-name` → find all files tagged with domain
- [ ] Parse `@git:branch-name` → generate diff summary vs branch
- [ ] Implement file tagging system (metadata in file attributes or KDB)
- [ ] Implement git diff summary (files changed, stats)
- [ ] Format output for inclusion in message

**Effort:** 1.5 weeks  
**Dependencies:** Phase 0, 2.1-2.3 complete

**Output:** By end of Phase 2, @-mentions are fully functional (file, symbol, knowledge, agent, module, git).

---

## Phase 3: Terminal Integration (Weeks 13-15)

### 3.1 Command Execution Endpoint

**Goal:** Allow subprocess execution via `/sessions/{id}/exec`

**Tasks:**
- [ ] Implement `/sessions/{id}/exec` endpoint
- [ ] Validate command safety (reject interactive commands)
- [ ] Call MCP `sandboxed_shell_exec` tool
- [ ] Stream output via chunked JSON or Server-Sent Events
- [ ] Set timeout (30 seconds default)
- [ ] Capture stdout + stderr + exit code
- [ ] Log execution to Universe

**Effort:** 1 week  
**Dependencies:** Phase 0, MCP sandboxed_shell_exec available

---

### 3.2 Interactive Command Detection

**Goal:** Detect and reject vim, git rebase -i, etc.

**Tasks:**
- [ ] Define regex patterns for interactive commands
- [ ] Check command before execution
- [ ] Return user-friendly error message with alternative
- [ ] Store rejected commands in audit log

**Effort:** 0.5 weeks  
**Dependencies:** 3.1 complete

---

### 3.3 Extension Output Streaming

**Goal:** Show real-time command output in extension UI

**Tasks:**
- [ ] Build output channel in extension
- [ ] Connect to `/sessions/{id}/exec` stream
- [ ] Display stdout in real-time
- [ ] Show exit code on completion
- [ ] Test with cargo check, git status, etc.

**Effort:** 1 week  
**Dependencies:** 3.1 complete

**Output:** By end of Phase 3, users can run cargo, git, and other commands from within Claude Code.

---

## Phase 4: Session Migration (Weeks 16-17)

### 4.1 Claude Code CLI Migration

**Goal:** Seamlessly import old sessions and checkpoints

**Tasks:**
- [ ] Detect existing `~/.claude/` directory
- [ ] Parse old session JSON format
- [ ] Transform to v2.0 schema
- [ ] Create Universe snapshots for each old checkpoint
- [ ] Validate no data loss
- [ ] Show migration report (X sessions, Y checkpoints)
- [ ] Handle failures gracefully (partial import, rollback)

**Effort:** 1 week  
**Dependencies:** Phase 0-1 complete

---

### 4.2 Extension Auto-Migration

**Goal:** Detect old Claude Code CLI and offer migration

**Tasks:**
- [ ] Add detection logic to extension startup
- [ ] Show migration banner if old sessions found
- [ ] Implement "Import All" / "Select Specific" flows
- [ ] Display progress during import
- [ ] Verify import success (list imported sessions)

**Effort:** 1 week  
**Dependencies:** 4.1 complete

**Output:** By end of Phase 4, users can import from original Claude Code without manual work.

---

## Phase 5: Error Handling & Reliability (Weeks 18-20)

### 5.1 Atomic File Writes

**Goal:** Ensure no partial writes on crash

**Tasks:**
- [ ] Implement write-to-temp-then-rename pattern in file apply handler
- [ ] Sync to disk before returning to extension
- [ ] Clean up stale temp files on startup
- [ ] Test crash recovery (simulate process kill mid-write)

**Effort:** 1 week  
**Dependencies:** Phase 0 complete

---

### 5.2 Tool Failure Handling

**Goal:** Graceful degradation when tools fail or timeout

**Tasks:**
- [ ] Catch timeout exceptions on MCP calls
- [ ] Implement retry logic with exponential backoff
- [ ] Show helpful error messages to user
- [ ] Log failures to Universe for debugging
- [ ] Test with intentional timeouts

**Effort:** 1 week  
**Dependencies:** Phase 0 complete

---

### 5.3 Git Conflict Recovery

**Goal:** Handle merge conflicts gracefully

**Tasks:**
- [ ] Detect conflict markers in git commit response
- [ ] Parse conflict hunks
- [ ] Show resolution UI in extension
- [ ] Let user choose which version
- [ ] Auto-retry commit with resolution
- [ ] Test with real git conflicts

**Effort:** 1 week  
**Dependencies:** Phase 3 complete

**Output:** By end of Phase 5, system is resilient to failures and recovers gracefully.

---

## Phase 6: Performance Optimization (Weeks 21-22)

### 6.1 Caching & Prefetching

**Goal:** Reduce latency for common operations

**Tasks:**
- [ ] Implement file content cache (LRU, mtime-invalidated)
- [ ] Prefetch common knowledge modules on session start
- [ ] Cache diff computations (incremental algorithm)
- [ ] Cache timeline queries (5-minute TTL)
- [ ] Profile and benchmark

**Effort:** 1.5 weeks  
**Dependencies:** Phase 0-2 complete

---

### 6.2 Resource Management

**Goal:** Prevent memory/disk bloat

**Tasks:**
- [ ] Monitor session memory usage
- [ ] Implement cache eviction policy
- [ ] Implement session cleanup (delete old sessions)
- [ ] Prune Universe database (retention policy)
- [ ] Test with 100+ sessions

**Effort:** 0.5 weeks  
**Dependencies:** 6.1 complete

**Output:** By end of Phase 6, system meets latency targets (< 100ms completion, < 500ms timeline).

---

## Phase 7: Extensibility (Weeks 23-24)

### 7.1 Custom Tool Registration

**Goal:** Allow users to register custom MCP tools

**Tasks:**
- [ ] Design tool registration API
- [ ] Implement `/tools/register` endpoint
- [ ] Support subprocess + HTTP handler types
- [ ] Validate tool inputs
- [ ] Make tools discoverable in mention autocomplete

**Effort:** 1 week  
**Dependencies:** Phase 0 complete

---

### 7.2 Custom Approval Policies

**Goal:** Allow per-project approval rules

**Tasks:**
- [ ] Define approval policy schema (conditions, actions)
- [ ] Implement policy evaluation engine
- [ ] Store policies in session
- [ ] Test with various rule combinations

**Effort:** 1 week  
**Dependencies:** Phase 0 (approval framework) complete

**Output:** By end of Phase 7, users can extend Claude Code with custom tools and policies.

---

## Phase 8: Mobile Support (Weeks 25-28)

### 8.1 Bonsai Buddy Integration

**Goal:** Enable Claude Code on mobile

**Tasks:**
- [ ] Extend MCP protocol for mobile clients
- [ ] Implement lighter-weight mobile session format
- [ ] Build mobile UI (Svelte Native or React Native)
- [ ] Sync sessions from desktop to mobile
- [ ] Test cross-device editing

**Effort:** 2 weeks  
**Dependencies:** Phase 0-7 complete

---

### 8.2 Cross-Device Sync

**Goal:** Seamless session continuity across devices

**Tasks:**
- [ ] Integrate with Echo/CRDT for session sync
- [ ] Conflict resolution (user edited on both devices)
- [ ] Test with network delays/offline scenarios
- [ ] Show sync status in UI

**Effort:** 1.5 weeks  
**Dependencies:** 8.1 complete, Echo framework

**Output:** By end of Phase 8, Claude Code works on mobile with cross-device sync.

---

## Critical Path

```
Phase 0 (Foundation)
├─ MCP Endpoints
├─ Session Storage
├─ Edit Request Handler
└─ Approval Framework
   ↓
Phase 1 (Checkpoints)
├─ Checkpoint Creation
├─ Restoration
├─ Timeline
└─ Extension UI
   ↓
Phase 2 (@-mentions)
├─ File/Symbol
├─ Knowledge
├─ Agent
└─ Extended Syntax
   ↓
Phase 3 (Terminal)
│
Phase 4 (Migration) ← Can start in parallel with Phase 3
│
Phase 5 (Reliability) ← Must complete before GA release
│
Phase 6 (Performance) ← Tuning phase before GA
│
GA Release (Week 24)
│
Phase 7 (Extensibility) ← Post-GA enhancement
│
Phase 8 (Mobile) ← Future phase
```

---

## MVP Scope (GA Release)

**Weeks 1-22 (5.5 months)**

**In Scope:**
- ✅ All Phase 0-6 work
- ✅ Feature parity with original Claude Code
- ✅ Migration from old CLI
- ✅ All three approval risk levels
- ✅ Performance targets met

**Out of Scope (v2.1+):**
- ❌ Mobile support (Phase 8)
- ❌ Real-time collaboration
- ❌ Custom tool registration (v2.1)
- ❌ Model hot-swapping (v2.1)

---

## Dependencies & Blockers

| Phase | Blocker | Mitigation |
|-------|---------|-----------|
| 0-2 | MCP server framework | Already exists in codebase |
| 0-1 | Universe/SQLite | Already implemented |
| 2.2 | KDB retrieval | Already implemented |
| 2.3 | Agent framework | Partially exists, may need extension |
| 3 | sandboxed_shell_exec | Already exists in MCP |
| 5 | Git command availability | Windows PowerShell has git |
| 6 | Performance profiling tools | Need to set up benchmarks |

---

## Testing Strategy

### Unit Tests (Phase 0)
- Endpoint routing
- Request validation
- Session CRUD

### Integration Tests (Phase 1-3)
- End-to-end edit request
- Checkpoint create/restore
- @-mention resolution
- Terminal command execution

### E2E Tests (Phase 4+)
- Migration workflow
- Multi-phase approval
- Error recovery
- Performance under load (100+ sessions)

### Regression Tests (Phase 5+)
- Old Claude Code session compatibility
- Diff hunk accuracy
- Universe event consistency

---

## Success Criteria

| Criterion | Acceptance | Phase |
|-----------|-----------|-------|
| Diff accuracy | 100% hunk correctness vs original | 1 |
| Migration success | 0% data loss | 4 |
| Latency | Diff generation < 500ms | 6 |
| Reliability | 99.9% session persistence | 5 |
| Feature parity | All original features work | 6 |
| Adoption | 50% of Claude Code users on Bonsai | 4 weeks post-GA |

---

## Resources Required

**Team Composition:**
- 1 Backend Engineer (Rust) – Phases 0-6
- 1 Frontend Engineer (TypeScript/Svelte) – Phases 1, 4, 8
- 1 QA Engineer – Phases 4-6
- 1 Product Manager – Roadmap + prioritization
- 0.5 DevOps (for benchmarking, infrastructure)

**Infrastructure:**
- CI/CD pipeline for MCP server testing
- Performance benchmarking environment
- Test database with sample datasets

---

## Appendix: Component Checklist

**Backend Components to Build:**
- [ ] `crates/bonsai-claude-code/` crate
- [ ] Session storage layer (SQLite schema)
- [ ] Edit request handler (model orchestration)
- [ ] Approval risk scorer
- [ ] Checkpoint manager (Universe integration)
- [ ] Timeline query builder
- [ ] Mention resolver (file, KDB, agent)
- [ ] Command executor (shell sandbox)
- [ ] Migration importer (from old CLI)
- [ ] Error handler + logging
- [ ] Performance cache layer

**Frontend Components to Build:**
- [ ] Claude Code panel (Svelte)
- [ ] Diff renderer with hunk accept/reject
- [ ] Timeline visualization
- [ ] Checkpoint manager UI
- [ ] Approval modals (quick + detailed)
- [ ] @-mention autocompleter
- [ ] Output channel for terminal
- [ ] Session selector
- [ ] Migration wizard
- [ ] Settings/preferences UI

**MCP Endpoints to Add (to existing server):**
- [ ] `/sessions` (POST, GET, DELETE)
- [ ] `/sessions/{id}/edit` (POST)
- [ ] `/sessions/{id}/approve` (POST)
- [ ] `/sessions/{id}/checkpoint/*` (POST, GET)
- [ ] `/sessions/{id}/mention-resolve` (POST)
- [ ] `/sessions/{id}/exec` (POST)
- [ ] `/sessions/{id}/timeline` (GET)

---

**Next Step:** Create Jira/GitHub issues from each phase. Target sprint velocity: 1 phase per month.
