# Claude Code to Bonsai Integration Layer – Design Document

**Version:** 1.0  
**Date:** June 2026  
**Status:** Design Phase (No Implementation)

---

## Executive Summary

This document defines the architectural design for adapting the Anthropic Claude Code VS Code extension to run entirely on the Bonsai ecosystem. The goal is **feature parity with the original CLI** while leveraging Bonsai's capabilities (local inference, knowledge modules, time-travel debugging, mobile support) to create a superior developer experience.

**Key Design Principles:**
- **Protocol Equivalence**: Replace the text-based CLI protocol with a Bonsai-native MCP-based protocol
- **Zero Data Loss**: All Claude Code sessions migrate seamlessly to Bonsai
- **Enhanced UX**: Bonsai-specific features (@knowledge, @agent, @module) enhance, not replace, original functionality
- **Performance First**: Local inference should be faster than cloud roundtrips
- **Security by Default**: All file mutations require explicit user approval
- **Extensibility**: Custom tools, models, and approval policies via Bonsai plugin system

---

## 1. Current Claude Code Architecture

### 1.1 How Claude Code Works Today

Claude Code (Anthropic's official VS Code extension) uses a **process-based protocol**:

```
┌──────────────────┐
│  VS Code Ext     │
│                  │
│ ┌──────────────┐ │
│ │ Claude Code  │ │
│ │ Panel (UI)   │ │
│ └──────┬───────┘ │
└────────┼──────────┘
         │ spawn `claude` CLI
         ▼
┌──────────────────────────────────────────┐
│  Claude Code CLI (Node.js binary)        │
│  ~/.claude/sessions/<id>.json stored here│
│  ~/.claude/checkpoints/<id>.json         │
│  JSON stdin/stdout protocol              │
└──────────────────────────────────────────┘
         │ HTTP to Anthropic API
         ▼
┌──────────────────────────────────────────┐
│  Anthropic Claude API                    │
│  Processes requests, returns diffs       │
└──────────────────────────────────────────┘
```

### 1.2 Protocol Components

**Extension → CLI (stdin):**
```json
{
  "type": "edit_request",
  "session_id": "sess_abc123",
  "prompt": "Add error handling to foo()",
  "context": {
    "files": [
      {
        "path": "src/main.rs",
        "content": "fn foo() { ... }"
      }
    ],
    "mentions": [
      {
        "type": "file",
        "path": "src/errors.rs",
        "lines": "1-50"
      }
    ]
  },
  "model": "claude-opus",
  "system_prompt": "..."
}
```

**CLI → Extension (stdout):**
```json
{
  "type": "edit_response",
  "session_id": "sess_abc123",
  "diffs": [
    {
      "path": "src/main.rs",
      "unified_diff": "--- a/src/main.rs\n+++ b/src/main.rs\n..."
    }
  ],
  "reasoning": "I added error handling using Result<T, E>...",
  "token_count": 1234,
  "timestamp": 1685000000
}
```

### 1.3 Session State

Claude Code stores two types of state:

**Session file** (`~/.claude/sessions/{id}.json`):
```json
{
  "session_id": "sess_abc123",
  "created_at": "2026-05-30T12:00:00Z",
  "model": "claude-opus",
  "message_history": [
    {"role": "user", "content": "..."},
    {"role": "assistant", "content": "..."}
  ],
  "checkpoint_ids": ["cp_1", "cp_2"],
  "settings": {
    "auto_approve_reads": false,
    "model_override": null
  }
}
```

**Checkpoint file** (`~/.claude/checkpoints/{id}.json`):
```json
{
  "checkpoint_id": "cp_1",
  "session_id": "sess_abc123",
  "created_at": "2026-05-30T12:15:00Z",
  "label": "Initial file state",
  "file_snapshots": {
    "src/main.rs": {
      "content": "...",
      "hash": "blake3_hash_here"
    }
  }
}
```

### 1.4 @-mention Resolution

Original syntax:
- `@file.rs` → injects entire file
- `@file.rs#L100-L120` → injects specific lines
- `@symbol:function_name` → injects all definitions/usages

Resolution is done by the CLI before sending to API.

---

## 2. Bonsai-Native Protocol Specification

### 2.1 Initialization Handshake

**Original Claude Code:**
```
Extension starts: `claude --daemon`
CLI prints: "Ready. Session: sess_xyz. Version: 1.0"
```

**Bonsai Replacement:**
```
Extension discovers MCP server via Bonsai daemon registry
  → GET /mcp/registry → lists available MCP servers
  → GET /mcp/servers/claude-code → returns {"endpoint": "http://127.0.0.1:3000", "protocol": "http"}
  
Extension obtains capability token:
  → POST /auth/issue {"tool": "claude_code", "scope": "code_edit,file_read,git_commit"}
  → Returns: {"token": "cap_xyz123", "expires_at": 1234567890}

Extension makes initial connection:
  → GET /health (with token header)
  → Returns: {"status": "ready", "version": "2.0", "protocol": "claude-code-mcp-v1"}
```

**Protocol Definition:**

| Method | Endpoint | Auth | Purpose |
|--------|----------|------|---------|
| GET | `/health` | Bearer token | Health check, version negotiation |
| GET | `/sessions` | Bearer token | List active sessions |
| POST | `/sessions` | Bearer token | Create new session |
| GET | `/sessions/{id}` | Bearer token | Get session metadata |
| POST | `/sessions/{id}/edit` | Bearer token | Request code edit |
| POST | `/sessions/{id}/checkpoint/create` | Bearer token | Create checkpoint |
| GET | `/sessions/{id}/checkpoint/list` | Bearer token | List checkpoints |
| POST | `/sessions/{id}/checkpoint/restore` | Bearer token | Restore checkpoint |
| POST | `/sessions/{id}/mention-resolve` | Bearer token | Resolve @-mentions |
| POST | `/sessions/{id}/approve` | Bearer token | Approve pending operation |
| DELETE | `/sessions/{id}` | Bearer token | Archive/delete session |

### 2.2 Edit Request / Response

**Extension sends to Bonsai MCP:**
```json
POST /sessions/{id}/edit

{
  "prompt": "Add error handling to foo()",
  "model": "qwen-7b",
  "temperature": 0.7,
  "context": {
    "files": [
      {
        "path": "src/main.rs",
        "content": "fn foo() { ... }",
        "encoding": "utf-8"
      }
    ],
    "mentions": [
      {
        "type": "file_range",
        "path": "src/errors.rs",
        "start_line": 1,
        "end_line": 50
      },
      {
        "type": "knowledge",
        "module": "rust-patterns",
        "query": "error handling"
      },
      {
        "type": "agent",
        "agent_id": "code-reviewer",
        "prompt": "Review this code for errors"
      }
    ],
    "system_prompt_overrides": {
      "tone": "educational",
      "format": "concise"
    }
  }
}
```

**MCP Server Response Stream:**
```json
[Streaming SSE or chunked JSON responses]

{
  "type": "edit_response",
  "id": "edit_abc123",
  "status": "pending_approval",
  "diffs": [
    {
      "path": "src/main.rs",
      "unified_diff": "--- a/src/main.rs\n+++ b/src/main.rs\n@@ -10,5 +10,10 @@\n...",
      "hunk_id": "hunk_1",
      "language": "rust",
      "change_type": "modify",
      "added_lines": 8,
      "removed_lines": 2
    }
  ],
  "reasoning": "I added error handling using Result<T, E>...",
  "approval_required": true,
  "approval_reason": "File write requires explicit approval",
  "token_count": {
    "input": 1200,
    "output": 340
  },
  "universe_event_id": "evt_xyz123"
}
```

### 2.3 Checkpoint Operations

**Create Checkpoint:**
```json
POST /sessions/{id}/checkpoint/create

{
  "label": "Before refactoring",
  "description": "Baseline state before optimizing hot path",
  "trigger": "manual"
}

Response:
{
  "checkpoint_id": "cp_abc123",
  "label": "Before refactoring",
  "created_at": "2026-05-30T12:45:00Z",
  "universe_snapshot_id": "snap_xyz789",
  "file_count": 47,
  "total_size_bytes": 2_345_678
}
```

**Restore Checkpoint:**
```json
POST /sessions/{id}/checkpoint/restore

{
  "checkpoint_id": "cp_abc123",
  "verify_hash": true
}

Response:
{
  "status": "restored",
  "checkpoint_id": "cp_abc123",
  "restored_file_count": 47,
  "restoration_event_id": "evt_restore_123"
}
```

**List Checkpoints:**
```json
GET /sessions/{id}/checkpoint/list?limit=20&offset=0

Response:
{
  "checkpoints": [
    {
      "checkpoint_id": "cp_1",
      "label": "Initial state",
      "created_at": "2026-05-30T12:00:00Z",
      "file_count": 47,
      "universe_snapshot_id": "snap_1"
    },
    {
      "checkpoint_id": "cp_2",
      "label": "Before refactoring",
      "created_at": "2026-05-30T12:15:00Z",
      "file_count": 47,
      "universe_snapshot_id": "snap_2"
    }
  ],
  "total_count": 42
}
```

**Design Notes:**
- Each checkpoint maps directly to a `UniverseSnapshot` in the Bonsai Universe ledger
- Checkpoints are cryptographically verified (BLAKE3 hashes)
- Restoring a checkpoint emits a `FileChange` event to the Universe
- Checkpoints are searchable by label in the timeline UI

### 2.4 @-mention Resolution

**Original Syntax (preserved):**
```
@file.rs
@file.rs#L100-L120
@symbol:my_function
```

**New Bonsai Syntax:**
```
@knowledge:rust-ownership        → Load KDB module and inject top-5 chunks
@knowledge:rust-ownership:advanced → Load specific submodule
@agent:code-reviewer             → Spawn code reviewer agent, inject findings
@module:auth                      → All files tagged with "auth" domain
@symbol:MyStruct:usages          → All usages of MyStruct (transitive)
@git:origin/main                 → Diff summary of HEAD vs branch
@git:origin/main:files           → List of changed files since branch
```

**Resolution Request:**
```json
POST /sessions/{id}/mention-resolve

{
  "mentions": [
    {
      "syntax": "@file.rs#L100-L120",
      "type": "file_range"
    },
    {
      "syntax": "@knowledge:rust-patterns",
      "type": "knowledge",
      "params": {
        "module": "rust-patterns",
        "top_k": 5,
        "score_threshold": 0.3
      }
    },
    {
      "syntax": "@agent:code-reviewer",
      "type": "agent",
      "params": {
        "agent_id": "code-reviewer",
        "prompt": "Review this code for errors"
      }
    }
  ]
}

Response:
{
  "mentions": [
    {
      "syntax": "@file.rs#L100-L120",
      "resolved_content": "fn foo(x: i32) { ... }",
      "source_type": "file",
      "char_count": 234,
      "tokens_estimated": 78
    },
    {
      "syntax": "@knowledge:rust-patterns",
      "resolved_content": "[Knowledge: rust-patterns]\n- Use `?` operator...",
      "source_type": "knowledge",
      "module": "rust-patterns",
      "entries_injected": 5,
      "tokens_estimated": 156
    },
    {
      "syntax": "@agent:code-reviewer",
      "resolved_content": "## Code Review Findings\n1. Missing error handling in...",
      "source_type": "agent",
      "agent_id": "code-reviewer",
      "tokens_estimated": 340
    }
  ],
  "total_tokens_estimated": 574
}
```

**Design Notes:**
- All mentions are resolved **before** sending to the model (matching original behavior)
- KDB modules are injected using the same retrieval mechanism as regular chat
- Agent spawning is synchronous during @-mention resolution (timeout: 5 seconds)
- Symbol resolution uses Bonsai's code indexing (if available, else fallback to regex)

### 2.5 Terminal Integration

**Spawn Subprocess:**
```json
POST /sessions/{id}/exec

{
  "command": "cargo check --workspace",
  "timeout_ms": 30000,
  "cwd": "/project/root",
  "env": {
    "RUST_BACKTRACE": "1"
  },
  "stdin": null,
  "interactive": false
}

Response (streaming):
{
  "status": "running",
  "process_id": 12345,
  "pid_str": "12345"
}

[Event stream:]
{
  "type": "stdout",
  "data": "   Compiling bonsai-core v0.1.0\n"
}
{
  "type": "stdout",
  "data": "    Finished ...\n"
}
{
  "type": "exit_code",
  "code": 0
}
```

**Design Notes:**
- Uses Bonsai's `sandboxed_shell_exec` MCP tool under the hood
- Interactive commands (git rebase -i) are rejected with clear error message
- Output is streamed in real-time to the extension
- Process output is captured in Universe for replay

### 2.6 Session Persistence

**Session Storage Format:**
```json
{
  "version": "2.0",
  "session_id": "sess_abc123",
  "created_at": "2026-05-30T12:00:00Z",
  "updated_at": "2026-05-30T12:45:00Z",
  "workspace_path": "/home/user/my-project",
  "model_config": {
    "model": "qwen-7b",
    "temperature": 0.7,
    "max_tokens": 4096,
    "system_prompt": "You are a helpful code assistant..."
  },
  "approval_policy": {
    "auto_approve_reads": true,
    "auto_approve_mentions": true,
    "require_approval_writes": true,
    "require_approval_deletes": true
  },
  "message_history": [
    {
      "id": "msg_1",
      "role": "user",
      "content": "Add error handling to foo()",
      "mentions": ["@file.rs#L10-20"],
      "timestamp": "2026-05-30T12:00:30Z"
    },
    {
      "id": "msg_2",
      "role": "assistant",
      "content": "I'll add error handling...",
      "diffs": ["diff_1"],
      "token_count": {"input": 450, "output": 340},
      "timestamp": "2026-05-30T12:00:45Z"
    }
  ],
  "diffs": [
    {
      "id": "diff_1",
      "file": "src/main.rs",
      "status": "pending_approval",
      "unified_diff": "--- a/src/main.rs\n...",
      "created_at": "2026-05-30T12:00:45Z",
      "universe_event_id": "evt_xyz"
    }
  ],
  "checkpoints": [
    {
      "checkpoint_id": "cp_1",
      "label": "Initial state",
      "created_at": "2026-05-30T12:00:00Z",
      "universe_snapshot_id": "snap_1"
    }
  ],
  "universe_sync": {
    "last_sync_event_id": "evt_latest_123",
    "sync_timestamp": "2026-05-30T12:45:00Z"
  }
}
```

**Storage Location:**
```
~/.bonsai/claude-code/
├── sessions/
│   └── {session_id}.json        (SQLite w/ Echo sync for cross-device)
├── cache/
│   └── {session_id}/
│       ├── file_content_cache/
│       └── knowledge_module_cache/
└── local_backups/
    └── {session_id}/
        ├── message_history.jsonl
        └── diffs.jsonl
```

---

## 3. Feature-by-Feature Adaptation

### 3.1 Inline Diff Rendering

**Current Claude Code:**
- Shows unified diff in a code editor view
- Allows accept/reject per hunk
- Applies only accepted hunks to disk

**Bonsai Adaptation:**

```typescript
// Extension code (TypeScript/Svelte)
interface DiffHunk {
  id: string;
  startLine: number;
  endLine: number;
  added: string[];
  removed: string[];
  context: string[];
  accepted: boolean;
}

async function renderDiff(diff: UnifiedDiff): Promise<void> {
  // 1. Parse unified diff into hunks
  const hunks = parseUnifiedDiff(diff);
  
  // 2. Create VS Code DiffEditor
  const editor = await vscode.commands.executeCommand(
    'vscode.openDiff',
    vscode.Uri.file(originalPath),
    vscode.Uri.file(modifiedPath),
    `Claude Code: ${file}`
  );
  
  // 3. Inject accept/reject buttons via decoration providers
  const decorationProvider = createHunkDecorationProvider(hunks);
  vscode.window.registerFileDecorationProvider(decorationProvider);
  
  // 4. Track hunk acceptance
  hunks.forEach(hunk => {
    editor.onHunkAccepted(hunk.id, async () => {
      // Apply only this hunk
      await applyHunk(originalPath, hunk);
      
      // Emit to Bonsai MCP
      await bonsaiMcp.post('/sessions/{id}/hunk-apply', {
        diff_id: diffId,
        hunk_id: hunk.id
      });
    });
  });
}

function parseUnifiedDiff(diff: string): DiffHunk[] {
  const lines = diff.split('\n');
  const hunks: DiffHunk[] = [];
  
  let i = 0;
  while (i < lines.length) {
    // Look for @@ -start,count +start,count @@
    const headerMatch = lines[i].match(/^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/);
    if (headerMatch) {
      const [, oldStart, , newStart] = headerMatch.map(x => parseInt(x || '1'));
      const hunk: DiffHunk = {
        id: `hunk_${hunks.length}`,
        startLine: newStart,
        endLine: newStart + 10, // TODO: calculate from hunk
        added: [],
        removed: [],
        context: [],
        accepted: false
      };
      
      // Parse hunk body
      i++;
      while (i < lines.length && !lines[i].startsWith('@@')) {
        const line = lines[i];
        if (line.startsWith('+')) hunk.added.push(line.slice(1));
        else if (line.startsWith('-')) hunk.removed.push(line.slice(1));
        else if (line.startsWith(' ')) hunk.context.push(line.slice(1));
        i++;
      }
      
      hunks.push(hunk);
    }
    i++;
  }
  
  return hunks;
}
```

**Hunk Application Logic:**
```json
POST /sessions/{id}/apply-hunks

{
  "diff_id": "diff_1",
  "accepted_hunk_ids": ["hunk_1", "hunk_3"],
  "verify": true
}

Response:
{
  "status": "applied",
  "hunks_applied": 2,
  "hunks_rejected": 1,
  "affected_files": ["src/main.rs"],
  "universe_event_id": "evt_apply_123"
}
```

### 3.2 Checkpoints (Universe Snapshots)

**Original Claude Code:**
- Checkpoints stored in JSON files
- No versioning or CRDT support
- Manual creation or pre-edit automatic

**Bonsai Adaptation:**

```typescript
interface Checkpoint {
  checkpointId: string;
  label: string;
  description?: string;
  createdAt: ISO8601;
  trigger: 'manual' | 'pre_edit' | 'pre_swarm' | 'auto_periodic';
  universeSnapshotId: string;
  
  // Metadata for timeline UI
  fileChangeSummary: {
    filesModified: number;
    filesAdded: number;
    filesDeleted: number;
  };
  size: number; // bytes
}

// Timeline rendering
function renderTimeline(checkpoints: Checkpoint[]): void {
  const svg = createSVG({
    height: 400,
    width: 100
  });
  
  // Vertical line with checkpoint markers
  const spacing = 400 / checkpoints.length;
  checkpoints.forEach((cp, i) => {
    const y = i * spacing;
    
    // Circle marker
    svg.circle({
      cx: 50,
      cy: y,
      r: 4,
      fill: cp.trigger === 'manual' ? 'blue' : 'gray'
    });
    
    // Tooltip
    svg.title(
      `${cp.label} (${cp.trigger})\n` +
      `Modified: ${cp.fileChangeSummary.filesModified} files`
    );
    
    // Click to restore
    svg.element.addEventListener('click', () => {
      restoreCheckpoint(cp.checkpointId);
    });
  });
  
  return svg;
}

async function restoreCheckpoint(checkpointId: string): Promise<void> {
  // Show preview modal
  const preview = await bonsaiMcp.post('/sessions/{id}/checkpoint-preview', {
    checkpoint_id: checkpointId
  });
  
  const confirmed = await showModal({
    title: `Restore to "${preview.label}"?`,
    body: `This will revert:\n${preview.changes.map(c => `- ${c}`).join('\n')}`,
    buttons: ['Cancel', 'Restore']
  });
  
  if (confirmed) {
    await bonsaiMcp.post('/sessions/{id}/checkpoint/restore', {
      checkpoint_id: checkpointId,
      verify_hash: true
    });
    
    // Reload editor state
    reloadWorkspace();
  }
}
```

**Timeline Query API:**
```json
GET /sessions/{id}/timeline?category=file_change&limit=50

Response:
{
  "events": [
    {
      "event_id": "evt_1",
      "event_type": "FileChange",
      "timestamp": "2026-05-30T12:00:00Z",
      "file": "src/main.rs",
      "change_type": "modify",
      "before_hash": "abc123",
      "after_hash": "def456"
    },
    {
      "event_id": "evt_2",
      "event_type": "Checkpoint",
      "timestamp": "2026-05-30T12:05:00Z",
      "checkpoint_id": "cp_1",
      "label": "Initial state",
      "universe_snapshot_id": "snap_1"
    }
  ]
}
```

### 3.3 @-mention Resolution (Enhanced)

**Bonsai provides three categories of mentions:**

1. **File Mentions** (original):
   ```
   @src/main.rs
   @src/main.rs#L10-50
   @symbol:MyFunction:usages
   ```
   - Resolved via filesystem + optional code indexing
   - Instant (< 100ms)

2. **Knowledge Mentions** (new):
   ```
   @knowledge:rust-patterns
   @knowledge:rust-patterns:error-handling
   ```
   - Uses KDB semantic search
   - Top-5 results by default
   - Can be tuned: `@knowledge:rust-patterns:5:0.5` (5 results, 0.5 threshold)

3. **Agent Mentions** (new):
   ```
   @agent:code-reviewer
   @agent:security-auditor
   ```
   - Spawns a lightweight agent to analyze code
   - Timeout: 5 seconds
   - Result cached for duration of edit session

**Resolution Pseudocode:**
```typescript
async function resolveMention(mention: string): Promise<ResolvedContent> {
  if (mention.startsWith('@knowledge:')) {
    // Parse: @knowledge:module-name[:submodule][:top_k][:threshold]
    const [module, submodule, topK, threshold] = parseMentionArgs(mention);
    const query = getEditorContext(); // Current file + selection
    
    const result = await bonsaiMcp.post('/kdb/retrieve', {
      module: module,
      submodule: submodule,
      query: query,
      top_k: parseInt(topK) || 5,
      score_threshold: parseFloat(threshold) || 0.3
    });
    
    return formatKnowledgeResults(result);
    
  } else if (mention.startsWith('@agent:')) {
    const agentId = mention.slice(7); // "agent:foo" → "foo"
    const context = getEditorContext();
    
    const result = await bonsaiMcp.post('/agents/run', {
      agent_id: agentId,
      context: context,
      timeout_ms: 5000
    });
    
    return result.findings || result.output;
    
  } else {
    // File or symbol mention (original behavior)
    return resolveFileMention(mention);
  }
}

function getEditorContext(): EditContext {
  return {
    file: vscode.window.activeTextEditor.document.fileName,
    content: vscode.window.activeTextEditor.document.getText(),
    selection: vscode.window.activeTextEditor.selection.active.line
  };
}
```

### 3.4 Terminal Integration

**Design:**
- Uses Bonsai's `sandboxed_shell_exec` MCP tool
- Supports background processes (async execution)
- Interactive commands (vim, git rebase -i) are rejected
- Output is captured to Universe for replay

```typescript
async function runCommand(command: string): Promise<CommandResult> {
  // Validate command safety
  if (isInteractiveCommand(command)) {
    throw new Error(
      `Interactive commands not supported via Claude Code.\n` +
      `Run this in the terminal manually: ${command}`
    );
  }
  
  // Execute via MCP
  const response = await bonsaiMcp.post('/sessions/{id}/exec', {
    command: command,
    timeout_ms: 30000,
    cwd: vscode.workspace.rootPath,
    interactive: false
  });
  
  // Stream output to output panel
  const outputChannel = vscode.window.createOutputChannel('Claude Code');
  for await (const chunk of response.stream) {
    if (chunk.type === 'stdout') {
      outputChannel.append(chunk.data);
    } else if (chunk.type === 'stderr') {
      outputChannel.append(`[ERROR] ${chunk.data}`);
    } else if (chunk.type === 'exit_code') {
      outputChannel.appendLine(`\n[Process exited with code ${chunk.code}]`);
      return { exitCode: chunk.code, output: outputChannel.value };
    }
  }
}

const INTERACTIVE_PATTERNS = [
  /^vim\s/,
  /^nano\s/,
  /^less\s/,
  /^git rebase -i/,
  /^git commit\s/,
  /^node --inspect/,
  /^python -i/,
];

function isInteractiveCommand(command: string): boolean {
  return INTERACTIVE_PATTERNS.some(pattern => pattern.test(command));
}
```

---

## 4. UI/UX Design

### 4.1 Extension Layout

```
┌────────────────────────────────────────────────────────┐
│  Bonsai Workspace (Tauri)                              │
├────────────────────────────────────────────────────────┤
│ File Tree  │  Code Editor       │ Claude Code Panel    │
│            │                    │                      │
│ src/       │  fn foo() {        │ ┌──────────────────┐ │
│ ├─ main.rs │    bar();          │ │ Sessions & Chat  │ │
│ ├─ lib.rs  │    baz();          │ ├──────────────────┤ │
│ └─ err.rs  │  }                 │ │                  │ │
│            │                    │ │ [Session: dev]   │ │
│            │  [Accept] [Reject] │ │                  │ │
│            │  +++ fn foo() {...}│ │ You:             │ │
│            │  === added error.. │ │ Add error hdlg.. │ │
│            │                    │ │                  │ │
│            │                    │ │ BonsAI:          │ │
│            │                    │ │ I'll add Result..│ │
│            │                    │ │                  │ │
│            │                    │ │ [✓ Applied]      │ │
│            │                    │ └──────────────────┘ │
│            │                    │ ⏱ Timeline  📸 Snap  │
├────────────────────────────────────────────────────────┤
│  Terminal                                              │
└────────────────────────────────────────────────────────┘
```

### 4.2 Claude Code Panel Components

**Session Management:**
```svelte
<script>
  import { onMount } from 'svelte';
  
  let sessions = [];
  let activeSession = null;
  
  onMount(async () => {
    sessions = await bonsaiMcp.get('/sessions');
    activeSession = sessions[0]; // TODO: persist selection
  });
</script>

<div class="panel">
  <div class="session-selector">
    <select bind:value={activeSession}>
      {#each sessions as session}
        <option value={session.session_id}>
          {session.workspace_path.split('/').pop()} ({session.model_config.model})
        </option>
      {/each}
    </select>
    <button on:click={createNewSession}>+ New Session</button>
  </div>
  
  <div class="message-history">
    {#each activeSession.message_history as msg}
      <div class="message {msg.role}">
        <div class="content">{msg.content}</div>
        {#if msg.diffs}
          <div class="diffs">
            {#each msg.diffs as diff}
              <DiffPreview {diff} />
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
  
  <div class="input-area">
    <textarea
      placeholder="Describe what you want to change..."
      bind:value={userInput}
    />
    <MentionCompleter bind:value={userInput} />
    <button on:click={sendMessage}>Send</button>
  </div>
</div>

<style>
  .message.user {
    background: #e3f2fd;
    border-left: 4px solid #2196f3;
  }
  .message.assistant {
    background: #f5f5f5;
    border-left: 4px solid #666;
  }
</style>
```

**Timeline Panel:**
```svelte
<script>
  let checkpoints = [];
  let timeline = [];
  
  onMount(async () => {
    timeline = await bonsaiMcp.get(`/sessions/${sessionId}/timeline?limit=50`);
    checkpoints = timeline.filter(e => e.event_type === 'Checkpoint');
  });
</script>

<div class="timeline-panel">
  <div class="timeline-scrubber">
    <svg height="300" width="600">
      <!-- Vertical axis with events -->
      {#each timeline as event, i}
        {#if event.event_type === 'Checkpoint'}
          <circle
            cx="300"
            cy={(i / timeline.length) * 300}
            r="6"
            fill="blue"
            on:click={() => previewCheckpoint(event.checkpoint_id)}
          />
          <title>{event.label}</title>
        {:else}
          <circle
            cx="300"
            cy={(i / timeline.length) * 300}
            r="3"
            fill="gray"
          />
        {/if}
      {/each}
    </svg>
  </div>
  
  <div class="checkpoint-list">
    {#each checkpoints as cp}
      <div class="checkpoint-card">
        <div class="label">{cp.label}</div>
        <div class="metadata">
          {cp.fileChangeSummary.filesModified} files modified
          · {formatBytes(cp.size)}
        </div>
        <button on:click={() => restoreCheckpoint(cp.checkpointId)}>
          ⏪ Restore
        </button>
      </div>
    {/each}
  </div>
</div>
```

### 4.3 Approval UI

**Quick Approval Modal:**
```svelte
<Modal title="Approve Changes?" open={showApprovalModal}>
  <div class="approval-content">
    <p>Apply this change to <code>{diff.path}</code>?</p>
    
    <DiffPreview {diff} maxLines={10} />
    
    <div class="buttons">
      <button on:click={reject}>Reject</button>
      <button on:click={accept} class="primary">Accept</button>
    </div>
  </div>
</Modal>
```

**Detailed Approval (High-Risk Operations):**
```svelte
<Modal title="Review & Approve" open={showDetailedApproval}>
  <div class="approval-detailed">
    <p>This operation requires explicit approval:</p>
    <p class="warning">⚠️ {operation.risk_reason}</p>
    
    <CodeDiffEditor
      original={operation.before}
      modified={operation.after}
      readOnly={true}
    />
    
    <p>Type <code>APPROVE</code> to confirm:</p>
    <input type="text" placeholder="APPROVE" bind:value={approvalText} />
    
    <div class="buttons">
      <button on:click={reject}>Cancel</button>
      <button
        disabled={approvalText !== 'APPROVE'}
        on:click={approve}
        class="primary"
      >
        Approve
      </button>
    </div>
  </div>
</Modal>
```

---

## 5. Data Format Compatibility

### 5.1 Session Migration

**From Claude Code CLI to Bonsai:**

```typescript
// Read old Claude Code session
const oldSession = JSON.parse(
  fs.readFileSync('~/.claude/sessions/sess_abc123.json')
);

// Transform to Bonsai format
const newSession = {
  version: '2.0',
  session_id: generateUUID(),
  created_at: oldSession.created_at,
  workspace_path: currentWorkspace,
  
  model_config: {
    model: oldSession.model || 'qwen-7b', // map claude-opus → qwen-7b
    temperature: 0.7,
    max_tokens: 4096,
    system_prompt: defaultSystemPrompt
  },
  
  message_history: oldSession.message_history.map(msg => ({
    id: generateUUID(),
    role: msg.role,
    content: msg.content,
    mentions: extractMentions(msg.content),
    timestamp: new Date().toISOString()
  })),
  
  checkpoints: oldSession.checkpoint_ids.map(cpId => ({
    checkpoint_id: generateUUID(),
    label: `Imported from Claude Code (${cpId})`,
    universe_snapshot_id: generateUUID(), // Create snapshot in Universe
    created_at: oldSession.updated_at
  }))
};

// Store in Bonsai
await bonsaiMcp.post('/sessions', newSession);
```

### 5.2 Checkpoint Format Migration

```typescript
// Read old checkpoint
const oldCheckpoint = JSON.parse(
  fs.readFileSync('~/.claude/checkpoints/cp_xyz.json')
);

// Create Universe snapshot
const snapshot = await universe.snapshots.create({
  label: oldCheckpoint.label || 'Imported checkpoint',
  description: `Imported from Claude Code checkpoint ${oldCheckpoint.checkpoint_id}`,
  trigger: 'manual',
  files: oldCheckpoint.file_snapshots
});

// Map to new format
const newCheckpoint = {
  checkpoint_id: generateUUID(),
  label: oldCheckpoint.label,
  universe_snapshot_id: snapshot.id,
  created_at: oldCheckpoint.created_at,
  fileChangeSummary: calculateSummary(snapshot.files)
};
```

---

## 6. Approval & HITL Integration

### 6.1 Approval Policies

**Define per-session:**
```json
{
  "approval_policy": {
    "auto_approve_reads": true,
    "auto_approve_mentions": true,
    "auto_approve_lines_under_50": true,
    
    "require_approval": {
      "file_writes": true,
      "file_deletes": true,
      "git_commits": true,
      "git_force_push": true,
      "command_execution": true,
      "sensitive_paths": [
        ".env*",
        "*.key",
        "*.pem",
        "**/secrets/*"
      ]
    },
    
    "auto_approve_paths": [
      "tests/**",
      "docs/**",
      "*.md"
    ]
  }
}
```

**Risk Scoring:**
```rust
fn score_operation_risk(operation: &Operation) -> RiskScore {
  match operation {
    Operation::ReadFile => RiskScore::Low,
    Operation::MentionResolve => RiskScore::Low,
    
    Operation::WriteFile { path, size, .. } => {
      if path.matches_sensitive_pattern() { RiskScore::High }
      else if size > 100_000 { RiskScore::Medium }
      else { RiskScore::Low }
    },
    
    Operation::DeleteFile { path, .. } => {
      if !path.can_be_recovered() { RiskScore::Critical }
      else { RiskScore::High }
    },
    
    Operation::GitCommit { .. } => RiskScore::Medium,
    Operation::GitForcePush { .. } => RiskScore::Critical,
    
    Operation::ExecuteCommand { command } => {
      if command.is_dangerous() { RiskScore::Critical }
      else { RiskScore::Low }
    }
  }
}
```

### 6.2 Approval Workflow

```
┌─ Operation Requested
│
├─ Score Risk Level
│
├─ Risk == Low?
│  └─ Auto-Approve → Execute
│
├─ Risk == Medium?
│  └─ Show Quick Modal ("Approve?")
│     ├─ Approve (1-click)
│     └─ Reject
│
└─ Risk == High/Critical?
   └─ Show Detailed Review Modal
      ├─ Full diff with context
      ├─ Require typed confirmation
      └─ Log approval to Universe
```

**Approval Logging:**
```json
{
  "approval_id": "appr_123",
  "operation_id": "op_456",
  "risk_level": "high",
  "operation_type": "write_file",
  "path": "src/main.rs",
  "approval_timestamp": "2026-05-30T12:45:00Z",
  "user_action": "approved",
  "approval_method": "typed_confirmation",
  "approval_text_hash": "blake3_hash",
  "universe_event_id": "evt_appr_123"
}
```

---

## 7. Error Handling & Rollback Strategy

### 7.1 Atomic File Writes

```rust
pub async fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
  // Write to temporary sibling
  let tmp_path = path.with_extension("tmp");
  
  {
    let mut file = tokio::fs::File::create(&tmp_path).await?;
    file.write_all(content).await?;
    file.flush().await?;
    file.sync_all().await?; // Ensure disk sync
  }
  
  // Atomic rename (POSIX guarantees atomicity)
  tokio::fs::rename(&tmp_path, path).await?;
  
  // Emit to Universe
  universe.emit_file_change(path, before_hash, after_hash)?;
  
  Ok(())
}

// If process crashes between write and rename:
// → tmp file left on disk (harmless)
// → original file unchanged
// → on next startup, clean up leftover tmp files
```

### 7.2 Tool Failure Handling

```typescript
async function executeTool(tool: string, args: any): Promise<Result> {
  const startTime = Date.now();
  const timeout = 30_000; // 30 seconds
  
  try {
    const result = await Promise.race([
      bonsaiMcp.call(tool, args),
      sleep(timeout).then(() => {
        throw new Error(`Tool timeout after ${timeout}ms`);
      })
    ]);
    
    return { status: 'success', result };
    
  } catch (error) {
    // Categorize failure
    if (error.message.includes('timeout')) {
      return {
        status: 'timeout',
        error: `Tool ${tool} did not respond within ${timeout}ms`,
        recovery: 'The operation was cancelled. You can retry or check system resources.'
      };
    } else if (error.message.includes('permission')) {
      return {
        status: 'permission_denied',
        error: error.message,
        recovery: 'Grant permission in settings and retry.'
      };
    } else {
      return {
        status: 'error',
        error: error.message,
        recovery: 'Check the error details and try again.'
      };
    }
  }
}
```

### 7.3 Git Merge Conflict Recovery

```typescript
async function gitCommit(message: string): Promise<void> {
  try {
    await bonsaiMcp.post('/git/commit', {
      message: message,
      verify_signature: false
    });
  } catch (error) {
    if (error.includes('conflict')) {
      // Show conflict resolution UI
      const conflicts = await extractConflictMarkers();
      const resolution = await showConflictResolutionUI(conflicts);
      
      // Apply user's resolution
      for (const conflict of conflicts) {
        const resolved = resolution[conflict.id];
        await applyResolution(conflict.file, conflict.lineRange, resolved);
      }
      
      // Retry commit
      await bonsaiMcp.post('/git/commit', {
        message: message,
        verify_signature: false
      });
    } else {
      throw error;
    }
  }
}
```

---

## 8. Performance & Latency Design

### 8.1 Latency Budgets

| Operation | Budget | Notes |
|-----------|--------|-------|
| Code completion | < 100ms | Local model inference |
| Diff generation | < 500ms | Model + diff computation |
| File write | < 1s | Includes atomic write + Universe event |
| Checkpoint create | < 100ms | Should be instant (background snapshot) |
| Timeline query | < 500ms | SQLite HNSW index, < 50 events |
| @mention resolution | < 2s | File (instant) + KDB (< 500ms) + Agent (< 5s timeout) |
| Terminal exec | N/A | Real-time streaming, timeout 30s |

### 8.2 Optimization Strategies

**File Content Caching:**
```typescript
const fileCache = new Map<string, CacheEntry>();

async function readFileWithCache(path: string): Promise<string> {
  // Check cache
  const cached = fileCache.get(path);
  if (cached && cached.mtime === (await getFileMtime(path))) {
    return cached.content;
  }
  
  // Cache miss → read from disk
  const content = await fs.readFile(path, 'utf-8');
  fileCache.set(path, {
    content,
    mtime: await getFileMtime(path)
  });
  
  return content;
}

// Invalidate cache on file change
vscode.workspace.onDidChangeTextDocument(event => {
  fileCache.delete(event.document.fileName);
});
```

**Knowledge Module Prefetching:**
```typescript
// On session start, load commonly used modules
async function prefetchModules(sessionId: string): Promise<void> {
  const modules = ['rust-patterns', 'bonsai-docs', 'python-stdlib'];
  
  for (const module of modules) {
    await bonsaiMcp.get(`/kdb/modules/${module}/load`, {
      async: true // Non-blocking load
    });
  }
}
```

**Diff Computation Optimization:**
```rust
// Instead of full unified diff, use incremental diff
pub fn incremental_diff(original: &str, modified: &str) -> Vec<Hunk> {
  // 1. Quick binary comparison (if identical, return empty)
  if blake3_hash(original) == blake3_hash(modified) {
    return Vec::new();
  }
  
  // 2. Split by lines
  let orig_lines = original.lines().collect::<Vec<_>>();
  let mod_lines = modified.lines().collect::<Vec<_>>();
  
  // 3. Myers' diff algorithm (O(n*m) but fast for small diffs)
  let hunks = myers_diff(&orig_lines, &mod_lines);
  
  // 4. Trim common prefix/suffix
  trim_unchanged_hunks(&mut hunks);
  
  hunks
}
```

### 8.3 Resource Management

**Memory Usage:**
```
Session state:            < 10 MB (message history, checkpoints)
File cache:              < 100 MB (actively edited files)
KDB module index:        < 500 MB (HNSW index in memory)
Total per session:       < 610 MB
Per-device capacity:     (RAM / 2) / num_sessions
```

**Disk Usage:**
```
~/.bonsai/claude-code/sessions/  : < 1 GB (session metadata)
~/.bonsai/claude-code/cache/     : < 10 GB (file content + knowledge modules)
~/.bonsai/universe.db             : < 100 GB (10-year retention of all events)
```

**Cleanup Policy:**
```
Every 7 days:
  - Delete sessions older than 30 days
  - Compact Universe database (remove old events beyond retention window)
  - Clear file content cache (re-fetch from disk as needed)

Manual cleanup:
  - Right-click session → "Delete & Archive"
  - Exports session.json + universe snapshots for backup
```

---

## 9. Extensibility Plan

### 9.1 Custom MCP Tools

**Register a custom tool:**
```json
POST /tools/register

{
  "name": "my_custom_tool",
  "description": "Does something special",
  "input_schema": {
    "type": "object",
    "properties": {
      "param1": { "type": "string" }
    },
    "required": ["param1"]
  },
  "handler": {
    "type": "subprocess",
    "command": "/usr/local/bin/my-tool",
    "args": ["--param", "{param1}"]
  }
}
```

**Use in Claude Code:**
```
You: Use @my_custom_tool to analyze the code

BonsAI: I'll use that custom tool...
```

### 9.2 Custom Models

**Load domain-specific model:**
```json
POST /models/load

{
  "model_name": "rust-code-7b",
  "source": "huggingface://my-org/rust-code-7b.gguf",
  "alias": "rust-expert"
}
```

**Switch model mid-session:**
```typescript
await bonsaiMcp.post(`/sessions/${sessionId}/model`, {
  model: 'rust-expert'
});

// All subsequent requests use the new model
```

### 9.3 Custom Approval Policies

**Define approval rules:**
```json
POST /sessions/{id}/approval-policy

{
  "rules": [
    {
      "id": "auto_approve_tests",
      "condition": "path.startsWith('tests/')",
      "action": "auto_approve",
      "reason": "Test changes are low-risk"
    },
    {
      "id": "require_review_migrations",
      "condition": "path.includes('migrations/')",
      "action": "require_review",
      "reason": "Database migrations need careful review"
    }
  ]
}
```

**Policy Evaluation:**
```rust
fn should_approve(operation: &Operation, policy: &ApprovalPolicy) -> ApprovalDecision {
  for rule in &policy.rules {
    if rule.condition.matches(operation) {
      return match rule.action {
        Action::AutoApprove => ApprovalDecision::Approved,
        Action::RequireReview => ApprovalDecision::PendingReview(rule.reason.clone()),
        Action::Block => ApprovalDecision::Blocked(rule.reason.clone()),
      };
    }
  }
  
  // Fall back to default policy
  policy.default_decision.clone()
}
```

---

## 10. Migration Path

### 10.1 User Onboarding

**Phase 1: Discovery (Day 1)**
```
1. User opens Bonsai Workspace
2. Claude Code panel shows "Claude Code v2.0 — Bonsai Edition" banner
3. "Migrate existing sessions?" button appears
4. User clicks → discovers MCP server automatically
5. Auth flow: grant capability token (one-time)
```

**Phase 2: Migration (Day 1-7)**
```
1. User imports old Claude Code sessions
   - Select ~/.claude/sessions/ directory
   - Review list of sessions to import
   - Click "Import All" or select specific sessions
2. Bonsai reads old JSON files
3. Transforms to new format
4. Creates Universe snapshots for all old checkpoints
5. Shows migration report: "5 sessions, 23 checkpoints imported"
```

**Phase 3: Validation (Day 7-14)**
```
1. User tests with old sessions
2. Verifies diffs, checkpoints work correctly
3. If issues found, file bug report
4. Bonsai team patches compatibility layer
```

### 10.2 Gradual Feature Rollout

**v2.0 (GA)**
- Feature parity with original Claude Code
- Basic @file, @symbol mentions
- Checkpoints working
- Session persistence

**v2.1 (1 month post-GA)**
- @knowledge mentions (KDB semantic search)
- Timeline UI (Bonsai Universe integration)
- Advanced approval policies

**v2.2 (2 months post-GA)**
- @agent mentions (code reviewer, security auditor)
- @module mentions (domain tagging)
- @git mentions (diff summaries)
- Mobile companion app (Bonsai Buddy)

**v2.3+ (Ongoing)**
- Custom tool registration
- Model hot-swapping
- Cross-device session sync
- Collaborative editing

### 10.3 Fallback & Rollback

**If Bonsai MCP server is unavailable:**
```typescript
// Gracefully degrade to no-op or cached mode
if (!await bonsaiMcp.health()) {
  showWarning('Bonsai MCP unavailable. Operating in offline mode.');
  
  // Disable features that require MCP
  disableButton('send-message');
  disableButton('apply-hunk');
  
  // Allow viewing cached sessions / checkpoints
  loadCachedSessions();
}
```

**If user wants to revert to original Claude Code CLI:**
```
Settings → Extensions → Claude Code → "Use legacy CLI mode"
→ Falls back to process-based protocol
→ Requires `~/.claude/` directory with original CLI binary
→ Warning: No Bonsai features available in this mode
```

---

## 11. Security Considerations

### 11.1 Capability Token Scope

```json
{
  "token": "cap_xyz123",
  "scope": [
    "code_edit",
    "file_read",
    "file_write",
    "git_commit",
    "git_push",
    "terminal_exec",
    "knowledge_retrieve"
  ],
  "constraints": {
    "max_file_size_bytes": 10_000_000,
    "denied_paths": [".env*", "*.key", "*.pem"],
    "exec_timeout_ms": 30_000,
    "rate_limit": {
      "requests_per_minute": 60,
      "tokens_per_hour": 1_000_000
    }
  }
}
```

### 11.2 Sandboxing

- All subprocess execution is sandboxed via `sandboxed_shell_exec`
- File writes are checked against `.bonsaiignore`
- Git operations require explicit approval
- Sensitive environment variables (AWS_KEY, etc.) are never passed to subprocesses

### 11.3 Audit Logging

```json
{
  "audit_log": [
    {
      "timestamp": "2026-05-30T12:45:00Z",
      "operation": "write_file",
      "path": "src/main.rs",
      "user": "user@example.com",
      "approval_status": "approved_by_user",
      "universe_event_id": "evt_123"
    }
  ]
}
```

---

## 12. Success Criteria

| Criterion | Metric | Target |
|-----------|--------|--------|
| Feature Parity | All original features work in Bonsai | 100% |
| Migration Smooth | Users can import old sessions without data loss | 0% loss |
| Performance | Code completion < 100ms (local) vs < 2s (cloud) | 20x faster |
| Reliability | Session continuity across extension restarts | 99.9% |
| Approval UX | Quick approvals < 2 seconds | < 2s |
| Timeline UI | Render 100 events in < 500ms | < 500ms |
| Extensibility | Custom tools can be registered without code changes | ✓ |

---

## 13. Open Questions & Future Work

1. **Mobile Companion**: Should Bonsai Buddy (mobile app) be able to continue Claude Code sessions from mobile? Design needed.

2. **Real-Time Collaboration**: Can multiple users work on the same session simultaneously using CRDT? Requires conflict resolution design.

3. **Model Hot-Swapping**: How to handle in-progress requests if user switches models mid-session?

4. **Cross-Platform Sync**: Should sessions sync from Windows desktop to Linux server automatically? Echo integration design needed.

5. **Custom @-mention Types**: How to let users define new mention types (e.g., `@review:peer-name`)?

6. **Offline Mode**: Full offline support when Bonsai MCP is unavailable? Requires local queuing + eventual sync.

---

## References

- [00-OVERVIEW.md](00-OVERVIEW.md) – Bonsai core architecture
- [06-KNOWLEDGE-DATABASE.md](06-KNOWLEDGE-DATABASE.md) – KDB semantic search
- [15-TIME-TRAVEL.md](15-TIME-TRAVEL.md) – Universe events & snapshots
- [20-MCP-SERVER.md](20-MCP-SERVER.md) – MCP server protocol
- [09-UI-UX-VISION.md](09-UI-UX-VISION.md) – UI panel generation

---

**Document Status:** Design complete. Ready for implementation planning.

**Next Steps:**
1. Present design to team for feedback
2. Identify implementation priorities (MVP vs Phase 2)
3. Create implementation tickets from design sections
4. Begin MCP endpoint implementation
5. Prototype extension UI components
