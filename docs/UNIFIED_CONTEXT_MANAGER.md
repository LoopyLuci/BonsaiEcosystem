# Unified Context Manager (UCM) Design Document

## Executive Summary

The Unified Context Manager (UCM) is a synchronization service that enables GitHub Copilot and Claude Code VS Code extensions to function as complementary AI agents rather than isolated tools. The UCM coordinates:

1. **Real-time file state** – Changes made by one extension are immediately visible to the other
2. **Conversation continuity** – Chat history and context travel seamlessly between extensions
3. **Approval deduplication** – User permissions granted in one extension avoid re-prompting in the other
4. **Checkpoint coordination** – Universe timeline snapshots are accessible from both extensions
5. **Knowledge module sharing** – @-mentions and loaded modules apply globally

The system is built on top of Bonsai's existing infrastructure:
- **Universe** (event logging + time travel)
- **AssistantStore** (conversation persistence)
- **SystemEventBus** (pub/sub messaging)
- **UACS** (approval cache)
- **EncryptedStore** (secure key-value storage)

---

## 1. Architecture Overview

### System Diagram

```
┌──────────────────────────────────────────────────────────────┐
│                  VS Code Host Process                        │
├──────────────────────────────────────────┬──────────────────┤
│                                          │                  │
│  ┌────────────────────────┐   ┌─────────┴──────────┐        │
│  │  Copilot Extension     │   │  Claude Code       │        │
│  │                        │   │  Extension         │        │
│  │  - Copilot Chat        │   │  - CC Chat Panel   │        │
│  │  - Inline suggestions  │   │  - File editor     │        │
│  └────────┬───────────────┘   └──────────┬─────────┘        │
│           │                              │                  │
│           │      LocalIPC (Unix socket)  │                  │
│           │      or Named Pipes (Win)    │                  │
│           └──────────────┬───────────────┘                  │
│                          │                                  │
│                    ┌─────▼─────────────┐                   │
│                    │   UCM Client Lib  │                   │
│                    │  (extension-side) │                   │
│                    └─────┬─────────────┘                   │
└─────────────────────────┼────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
        │      LocalIPC / Conduit           │
        │                 │                 │
        ▼                 │                 ▼
┌───────────────────┐    │    ┌────────────────────────┐
│  Bonsai Daemon    │    │    │  EncryptedStore (KDB)  │
│                   │    │    │  - UCM state           │
│  ┌─────────────┐  │    │    │  - Approval cache      │
│  │    UCM      │  │────┘    │  - Conversation index  │
│  │  Service    │  │         └────────────────────────┘
│  └──┬──────────┘  │
│     │             │         ┌────────────────────────┐
│  ┌──▼────────┐    │         │  SQLite Database       │
│  │ Universe  │    │         │  - Messages            │
│  │  (logs)   │    │         │  - Sessions            │
│  └───────────┘    │         │  - Snapshots           │
│                   │         │  - Operations log      │
│  ┌─────────────┐  │         └────────────────────────┘
│  │  System    │  │
│  │ EventBus   │  │
│  └─────────────┘  │
│                   │
│  ┌─────────────┐  │
│  │    UACS     │  │
│  │  (approvals)│  │
│  └─────────────┘  │
└───────────────────┘
```

### Data Flow Diagram

**Scenario: User edits file in Copilot, Claude Code sees change**

```
1. User edits src/main.rs in Copilot Chat inline suggestion
   │
   ├─> Copilot Extension calls: ucm_client.notify_file_change()
   │   - URI: file:///path/to/src/main.rs
   │   - Content hash: blake3(new_content)
   │   - Version: 42
   │   - Source: copilot
   │
   ├─> UCM Client (Copilot) sends to daemon via IPC
   │
   ├─> UCM Service receives, processes:
   │   - Validates change (content hash matches)
   │   - Records event in Universe
   │   - Updates ActiveFileState in EncryptedStore
   │   - Broadcasts to SystemEventBus
   │
   ├─> SystemEventBus notifies subscribers
   │
   ├─> UCM Client (Claude Code) receives notification
   │
   └─> Claude Code Extension:
       - Updates editor view
       - Refreshes file outline
       - Updates inline references
       - Shows notification "Updated by Copilot"
```

**Scenario: User approves operation in Copilot, Claude Code doesn't re-ask**

```
1. User in Copilot Chat: "write file foo.rs"
   │
   ├─> Copilot calls UACS approval flow
   │   - Request: write_file("foo.rs", content_hash)
   │   - User approves in visual modal
   │
   ├─> UACS caches approval:
   │   - Key: approval_signature(operation_type, target, content_hash)
   │   - Value: (approved=true, timestamp=now, ttl=3600s)
   │   - Stored in EncryptedStore
   │
   ├─> 30 seconds later, Claude Code Chat: "write to foo.rs"
   │
   ├─> Claude Code calls approval_cache.check_approval()
   │   - Signature matches previous approval
   │   - TTL still valid (3570s remaining)
   │
   └─> Claude Code automatically proceeds, no user prompt
```

---

## 2. State Model

### Core Data Structures

```rust
/// The unified state synchronized between extensions.
/// All fields are encrypted at rest in EncryptedStore.
pub struct UnifiedContextState {
    /// Current active file being edited
    pub active_file: ActiveFileState,
    
    /// Conversation history from both extensions
    pub conversation: ConversationState,
    
    /// Loaded knowledge modules and @-mentions
    pub knowledge_context: KnowledgeModuleState,
    
    /// Cached user approvals
    pub approval_cache: ApprovalCacheState,
    
    /// Universe timeline references
    pub checkpoints: CheckpointState,
    
    /// Metadata about synchronization itself
    pub sync_meta: SyncMetadata,
}

/// Represents the currently active file across both extensions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveFileState {
    /// URI: file:///absolute/path/to/file.rs
    pub file_uri: String,
    
    /// Current text content (may be large; stored separately in CAS)
    pub content: String,
    
    /// BLAKE3 hash of content for integrity checking
    pub content_hash: String,
    
    /// Monotonically increasing version; used for conflict detection
    pub version: u64,
    
    /// CAS reference to full content if > 1MB
    pub content_cas_ref: Option<String>,
    
    /// Cursor position (line, col) where last edit occurred
    pub cursor_position: (u32, u32),
    
    /// Timestamp of last change (Unix nanoseconds)
    pub last_modified_ns: u64,
    
    /// Which extension made the last change
    pub source_extension: ExtensionId,
    
    /// List of pending edits not yet applied
    pub pending_changes: Vec<TextDelta>,
}

/// Source identifier for edits
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExtensionId {
    Copilot,
    ClaudeCode,
    System,
}

/// Incremental text change (operational transformation delta)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextDelta {
    /// Offset in bytes from start of file
    pub offset: usize,
    
    /// Bytes to remove
    pub delete_len: usize,
    
    /// Bytes to insert
    pub insert: String,
    
    /// Timestamp of change (for causal ordering)
    pub timestamp_ns: u64,
    
    /// Which extension made this change
    pub source: ExtensionId,
}

/// Aggregates conversation from both extensions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationState {
    /// Map of session_id -> merged conversation
    pub sessions: HashMap<String, MergedConversation>,
    
    /// Currently active session
    pub active_session_id: Option<String>,
    
    /// Cross-extension conversation references
    pub linked_conversations: Vec<ConversationLink>,
}

/// A single conversation merged from both sources.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MergedConversation {
    /// UUID of this conversation
    pub id: String,
    
    /// Human-readable title
    pub title: String,
    
    /// All messages in order
    pub messages: Vec<Message>,
    
    /// Which extension created this conversation
    pub origin_extension: ExtensionId,
    
    /// Linked session in other extension (if any)
    pub linked_session_id: Option<String>,
    
    /// When merged (if originally in two extensions)
    pub merged_at_ns: Option<u64>,
}

/// A single message (user or assistant)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// Unique message ID
    pub id: String,
    
    /// "user" | "assistant" | "system"
    pub role: String,
    
    /// Message text
    pub content: String,
    
    /// Files @-mentioned or discussed
    pub files: Vec<String>,
    
    /// Knowledge modules @-mentioned
    pub knowledge_modules: Vec<String>,
    
    /// Tool calls made in this message
    pub tool_calls: Vec<ToolCallRecord>,
    
    /// CAS hash of content (for deduplication)
    pub content_hash: String,
    
    /// Timestamp (Unix nanoseconds)
    pub timestamp_ns: u64,
    
    /// Universe event IDs that reference this message
    pub universe_events: Vec<String>,
}

/// Record of a tool call (approval-relevant)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallRecord {
    pub id: String,
    pub tool_name: String,
    pub args: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub approval_signature: String,
    pub approved_at_ns: Option<u64>,
    pub approved_by: Option<String>,
}

/// Knowledge modules and @-mentions
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KnowledgeModuleState {
    /// Loaded knowledge modules with versions
    pub loaded_modules: HashMap<String, LoadedModule>,
    
    /// @-mentions of files to prioritize in context
    pub at_mentions: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoadedModule {
    pub name: String,
    pub version: String,
    pub cas_hash: String,
    pub loaded_at_ns: u64,
    pub scope: ModuleScope,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModuleScope {
    /// Applies to both extensions
    Global,
    /// Only applies to source extension
    Local,
}

/// Approval cache state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApprovalCacheState {
    /// Map of approval_signature -> cached approval
    pub approvals: HashMap<String, CachedApproval>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedApproval {
    /// Approval decision
    pub approved: bool,
    
    /// Timestamp of approval (Unix seconds)
    pub approved_at: u64,
    
    /// TTL in seconds (default: 3600)
    pub ttl_seconds: u32,
    
    /// Which extension requested (for audit)
    pub requested_by: ExtensionId,
    
    /// User's note about why they approved
    pub note: Option<String>,
}

/// Approval signature = hash(operation_type, target, content_hash)
/// Example:
///   write_file("src/main.rs", blake3("fn main() { ... }"))
///   => "a7b3c2f9e..."
/// This deduplicates: same file, same content = no re-ask even if
/// the other extension makes the request.

/// Universe checkpoint tracking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointState {
    /// All Universe snapshots accessible to both extensions
    pub snapshots: Vec<SnapshotReference>,
    
    /// Currently active snapshot (if any)
    pub active_snapshot_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotReference {
    pub snapshot_id: String,
    pub label: String,
    pub description: String,
    pub timestamp_ns: u64,
    pub created_by: ExtensionId,
    pub file_hashes: HashMap<String, String>,
}

/// Metadata about the synchronization system itself
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncMetadata {
    /// Version of UCM protocol
    pub protocol_version: String,
    
    /// Timestamp of last full sync
    pub last_full_sync_ns: u64,
    
    /// Queue of pending changes waiting to apply
    pub pending_events: Vec<SyncEvent>,
    
    /// Are we currently offline?
    pub is_offline: bool,
    
    /// Causality vector for CRDT
    pub causality_vector: HashMap<ExtensionId, u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncEvent {
    pub event_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub timestamp_ns: u64,
    pub source: ExtensionId,
}
```

### Update Semantics

Each field is updated according to these rules:

| Field | Updated by | Frequency | Conflict Resolution |
|-------|-----------|-----------|-------------------|
| `active_file.content` | Either extension | On every edit | See Section 7 |
| `active_file.version` | UCM Service | On every edit | Monotonic increment |
| `conversation.messages` | Either extension | On send | Last-write-wins + dedup |
| `knowledge_context.loaded_modules` | Either extension | On @-mention | Union + version check |
| `approval_cache.approvals` | UACS | On user approval | TTL-based expiration |
| `checkpoints.snapshots` | Universe | On checkpoint | Append-only |
| `sync_meta.pending_events` | UCM | During offline | FIFO queue |

---

## 3. Synchronization Protocol

### IPC Transport Layer

**Unix/Linux/macOS:**
```
Unix domain socket: /tmp/bonsai-ucm-{user_id}.sock
Message format: [4-byte length prefix] [JSON body]
TLS: Not needed (same user, same process, local filesystem)
```

**Windows:**
```
Named pipe: \\.\pipe\bonsai-ucm-{user_id}
Message format: [4-byte length prefix] [JSON body]
TLS: Not needed (same user, same process)
```

### Message Format

```json
{
  "message_id": "uuid",
  "message_type": "FileChange|ConversationUpdate|ApprovalCache|CheckpointRestore|...",
  "timestamp_ns": 1234567890,
  "source": "copilot|claude_code|system",
  "payload": { /* type-specific */ },
  "causality": {
    "copilot": 42,
    "claude_code": 37
  }
}
```

### Request-Response vs. Pub/Sub

**One-way notifications (pub/sub):**
- File changes
- Checkpoint created
- Module loaded

**Request-response (RPC):**
- Check approval (request → cache lookup → response)
- Restore checkpoint (request → apply all files → response)
- Query conversation history (request → database → response)

### Synchronization Algorithm

**Phase 1: Extension connects**

1. Extension calls `ucm_client.connect(extension_id)`
2. UCM Service checks if other extension is already connected
3. UCM loads full `UnifiedContextState` from EncryptedStore
4. If offline queue exists, starts applying pending events
5. Returns state snapshot to extension

**Phase 2: Normal operation**

1. Extension makes change (file edit, message sent, approval)
2. Extension calls `ucm_client.notify_change(event)`
3. UCM applies change to in-memory state
4. UCM checks for conflicts (see Section 7)
5. UCM persists to EncryptedStore + Universe
6. UCM broadcasts to all subscribers via SystemEventBus
7. Other extension receives event, updates UI

**Phase 3: Offline handling**

1. Network connection lost (or IPC socket closes)
2. All further `ucm_client.notify_change()` calls queue locally
3. Queue persisted to EncryptedStore for recovery across restarts
4. When connection restored, apply queued events in FIFO order

### Causality Tracking (CRDT-style)

To handle offline scenarios and ensure deterministic conflict resolution, track a causality vector:

```rust
pub struct CausalityVector {
    /// Last sequence number seen from each extension
    pub last_seen: HashMap<ExtensionId, u64>,
}

impl CausalityVector {
    /// Check if event is out-of-order
    pub fn is_next(&self, source: ExtensionId, seq: u64) -> bool {
        self.last_seen.get(&source).copied().unwrap_or(0) + 1 == seq
    }
    
    /// Advance after processing event
    pub fn advance(&mut self, source: ExtensionId, seq: u64) {
        self.last_seen.insert(source, seq);
    }
}
```

When reconnecting after offline:
1. Exchange causality vectors
2. Replay only events newer than last known sequence
3. For conflicting events (both extensions modified same file), use deterministic rule (see Section 7)

---

## 4. Conversation Context Merging

### Problem Statement

- User starts conversation in Copilot Chat: "explain Rust ownership"
- Later, user opens Claude Code Chat and asks: "what's a lifetime?"
- Should Claude Code have context from the Copilot conversation?

### Solution: Conversation Linking

**When user connects new extension:**

1. UCM checks if there's an active Copilot session
2. If yes, creates `ConversationLink` in Claude Code session
3. Claude Code Chat UI shows: "Linked to Copilot session 'Rust Questions'"
4. When user @-mentions context, both conversations are searched
5. If Claude Code responds to linked conversation, message appears in both

**Conversation Link Structure:**

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationLink {
    /// Source conversation
    pub source_session_id: String,
    pub source_extension: ExtensionId,
    
    /// Linked conversation
    pub linked_session_id: String,
    pub linked_extension: ExtensionId,
    
    /// Last message synchronized
    pub last_synced_message_id: String,
    pub synced_at_ns: u64,
}
```

### Continuation Context Injection

When Claude Code sends a message, include continuation context:

```json
{
  "role": "system",
  "content": "Linked conversation context from Copilot:\n\n" +
             "[5 most recent messages from linked session]"
}
```

### Duplicate Avoidance

If both extensions append messages to same conversation:
1. Deduplicate by content_hash
2. Keep first message chronologically
3. Log both sources in `Message.universe_events`

---

## 5. Approval Cache & Deduplication

### Approval Signature

An approval is identified by a signature combining:
- Operation type (e.g., "write_file", "delete_file", "network_request")
- Target (e.g., file path, URL)
- Content hash (for writes, hash of content being written)
- Optional args hash (for complex operations)

```rust
pub fn approval_signature(
    operation: &str,
    target: &str,
    content_hash: Option<&str>,
) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(operation.as_bytes());
    hasher.update(b"|");
    hasher.update(target.as_bytes());
    if let Some(hash) = content_hash {
        hasher.update(b"|");
        hasher.update(hash.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

// Example:
// write_file("src/auth.rs", blake3("pub fn login()..."))
// => signature: "a7b2c9f1e..."
// This remains valid for 1 hour (TTL).
// If the content changes, signature changes, and user is re-prompted.
```

### Cache Lookup Flow

```
1. Extension makes tool call: write_file("src/auth.rs", content)
2. Compute signature: sig = approval_signature("write_file", "src/auth.rs", content_hash)
3. Query approval cache: cache.get(sig)
4. If hit AND not expired:
   - Immediately proceed
   - Log tool execution to Universe
5. If miss OR expired:
   - Show approval dialog to user
   - On approval, cache with TTL=3600
   - Proceed with tool
```

### Cache Expiration

- Default TTL: 3600 seconds (1 hour)
- Configurable per operation category:
  - Destructive (write/delete): 1 hour
  - Network: 15 minutes
  - Model mutation: 4 hours
  - System modification: 1 hour
- Stored in EncryptedStore with expiration timestamp
- Background task periodically prunes expired entries

### UACS Integration

The existing UACS system (approval cache) is extended:

```rust
impl UacsManager {
    /// Check if approval exists and is valid
    pub async fn check_cached_approval(
        &self,
        signature: &str,
    ) -> Result<Option<CachedApproval>, String> {
        if let Some(approval) = self.ucm_state.approval_cache.approvals.get(signature) {
            if approval.expired() {
                self.ucm_state.approval_cache.approvals.remove(signature);
                return Ok(None);
            }
            return Ok(Some(approval.clone()));
        }
        Ok(None)
    }
    
    /// Request approval, handling UCM cache
    pub async fn request_approval(
        &self,
        category: ApprovalCategory,
        signature: &str,
    ) -> Result<bool, String> {
        // Check cache first
        if let Some(cached) = self.check_cached_approval(signature).await? {
            return Ok(cached.approved);
        }
        
        // Not cached — show dialog
        let approved = self.show_approval_dialog(category).await?;
        
        // Cache result
        if approved {
            self.ucm_state.approval_cache.approvals.insert(
                signature.to_string(),
                CachedApproval {
                    approved: true,
                    approved_at: now_unix_seconds(),
                    ttl_seconds: category.default_ttl(),
                    requested_by: self.extension_id,
                    note: None,
                }
            );
        }
        
        Ok(approved)
    }
}
```

---

## 6. Checkpoint Coordination

### Problem Statement

User creates checkpoint in Claude Code:
```
Checkpoint: "refactor complete"
- src/main.rs: v42
- src/lib.rs: v18
- Cargo.toml: v5
```

Later, in Copilot, user says "restore refactor checkpoint". Should work seamlessly.

### Design

**Creating a checkpoint:**

1. User clicks "Create Checkpoint" in Claude Code
2. Claude Code calls: `ucm_client.create_checkpoint(label, description)`
3. UCM:
   - Captures full `UnifiedContextState`
   - Calls `Universe.create_snapshot()`
   - Stores in `CheckpointState.snapshots`
   - Broadcasts to Copilot

**Restoring a checkpoint:**

1. User in Copilot asks: "restore refactor checkpoint"
2. Copilot calls: `ucm_client.restore_checkpoint(checkpoint_id)`
3. UCM:
   - Loads snapshot from Universe
   - Atomically updates all files
   - Updates `UnifiedContextState.active_file`
   - Records event in Universe (with causality links to original snapshot)
   - Broadcasts to both extensions
4. Both extensions reload their file views

### Atomic Restore

Critical: all files must be updated together.

```rust
pub async fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<(), String> {
    // 1. Load snapshot
    let snapshot = self.universe.store.get_snapshot(checkpoint_id).await?;
    
    // 2. Begin transaction
    let mut tx = self.db.begin().await?;
    
    // 3. For each file in snapshot, apply content
    for (file_uri, content_cas_ref) in snapshot.file_hashes {
        let content = self.load_cas_blob(content_cas_ref).await?;
        tokio::fs::write(&file_uri, &content).await?;
        
        // Update UCM state
        self.ucm_state.active_file = ActiveFileState {
            file_uri,
            content,
            version: snapshot.version + 1,
            // ...
        };
    }
    
    // 4. Commit transaction
    tx.commit().await?;
    
    // 5. Record event in Universe
    self.universe.record_checkpoint_restore(checkpoint_id).await;
    
    // 6. Broadcast to extensions
    self.event_bus.publish(SystemEvent::CheckpointRestored {
        checkpoint_id: checkpoint_id.to_string(),
        files_count: snapshot.file_hashes.len() as u32,
    });
}
```

### Checkpoint History

Both extensions can browse all checkpoints:

```rust
pub async fn list_checkpoints(&self) -> Result<Vec<CheckpointInfo>, String> {
    Ok(self.ucm_state.checkpoints.snapshots
        .iter()
        .map(|sr| CheckpointInfo {
            id: sr.snapshot_id.clone(),
            label: sr.label.clone(),
            created_at: sr.timestamp_ns,
            created_by: sr.created_by,
            file_count: sr.file_hashes.len(),
        })
        .collect())
}
```

---

## 7. Conflict Resolution Strategies

### Conflict Types

**Type A: Simultaneous file edits**
- Copilot and Claude Code both editing same file
- Different ranges (e.g., Copilot edits line 1-10, CC edits line 50-60)
- Could merge with 3-way diff

**Type B: Conversation message deduplication**
- Both extensions send similar message (copy-paste error?)
- Detected by content_hash

**Type C: Approval contradiction**
- User approved "write src/auth.rs" in Copilot
- 30 seconds later, different file, but user denied in Claude Code
- Should we re-ask? (depends on TTL and signature)

### Resolution Strategies

#### Option A: Last-Write-Wins (LWW)

**Simplest, deterministic.**

```rust
pub fn resolve_file_conflict(
    copilot_state: &ActiveFileState,
    cc_state: &ActiveFileState,
) -> ActiveFileState {
    if copilot_state.last_modified_ns > cc_state.last_modified_ns {
        copilot_state.clone()
    } else {
        cc_state.clone()
    }
}
```

**Pros:**
- Simple to implement
- Deterministic (no user interaction)
- Fast (<1ms)

**Cons:**
- Loses one version (could be important work)
- May confuse user ("where did my changes go?")

#### Option B: 3-Way Merge

**Smarter, handles non-overlapping edits.**

```rust
pub fn resolve_file_conflict(
    original: &str,
    copilot: &str,
    cc: &str,
) -> Result<String, MergeConflict> {
    let diff_copilot = compute_diff(&original, copilot);
    let diff_cc = compute_diff(&original, cc);
    
    // Check for overlapping regions
    if diff_copilot.overlaps_with(&diff_cc) {
        return Err(MergeConflict {
            region: overlapping_region,
            copilot_content: copilot.clone(),
            cc_content: cc.clone(),
        });
    }
    
    // Non-overlapping edits can merge
    Ok(apply_all_diffs(&original, vec![diff_copilot, diff_cc]))
}
```

**Pros:**
- Preserves non-conflicting edits
- Works for most real scenarios

**Cons:**
- Complex to implement correctly
- May produce syntactically invalid code
- Requires diff library (git3, similar)

#### Option C: Prompt User

**Safest but high friction.**

```rust
pub async fn resolve_file_conflict(
    copilot_state: &ActiveFileState,
    cc_state: &ActiveFileState,
) -> Result<ActiveFileState, String> {
    let choice = show_conflict_dialog()
        .await?
        .prompt("Both extensions modified this file. Which version keep?");
    
    match choice {
        Choice::Copilot => Ok(copilot_state.clone()),
        Choice::ClaudeCode => Ok(cc_state.clone()),
        Choice::Merge => try_merge(copilot_state, cc_state),
        Choice::Cancel => Err("User cancelled".to_string()),
    }
}
```

**Pros:**
- Safe (no data loss)
- User always in control

**Cons:**
- Interrupts workflow
- Requires user decision for every conflict

### Recommendation

**Use hybrid strategy:**

1. **Default: Last-Write-Wins** for speed and simplicity
2. **On conflict, attempt 3-way merge** if both diffs are computable
3. **If merge produces conflict markers**, wait before committing:
   - Log conflict to Universe
   - Show notification: "Merge conflict in src/main.rs — review before next save"
   - User resolves manually
4. **Conversation messages: Always deduplicate** by content_hash
5. **Approvals: Treat as immutable** (TTL-based expiration instead of conflict)

**Implementation:**

```rust
pub async fn apply_change(
    &mut self,
    change: FileChange,
) -> Result<(), String> {
    let current = self.get_file_content(&change.file_uri).await?;
    let last_version = self.ucm_state.active_file.version;
    
    // Fast path: no conflict (only one extension touching this file)
    if change.version == last_version {
        // Apply change and increment version
        let new_content = self.apply_delta(&current, &change.delta);
        self.ucm_state.active_file.content = new_content;
        self.ucm_state.active_file.version += 1;
        return Ok(());
    }
    
    // Slow path: potential conflict
    let original = self.get_file_at_version(last_version - 1).await?;
    let merged = self.try_merge(&original, &current, &change.delta)?;
    
    if merged.has_conflict_markers() {
        // Log and wait
        self.universe.log_event(UniverseEvent::new(
            EventSource::System { component: "UCM".to_string() },
            EventCategory::FileChange,
            "Merge conflict detected, awaiting manual resolution",
            &change.file_uri,
            self.device_id.clone(),
        ));
        self.ucm_state.active_file.pending_changes.push(TextDelta {
            offset: 0,
            delete_len: current.len(),
            insert: merged.clone(),
            timestamp_ns: now_ns(),
            source: change.source,
        });
        return Err("Merge conflict — manual resolution required".to_string());
    }
    
    // Success: no conflict markers
    self.ucm_state.active_file.content = merged;
    self.ucm_state.active_file.version += 1;
    Ok(())
}
```

---

## 8. Offline & Sync Behavior

### Local Queue

When extension loses connection to UCM daemon:

```rust
pub struct OfflineQueue {
    /// FIFO queue of pending events
    pub events: Vec<SyncEvent>,
    
    /// Timestamp when disconnect detected
    pub disconnected_at_ns: u64,
    
    /// File path for persistence
    pub queue_file: PathBuf,
}

impl OfflineQueue {
    pub async fn push(&mut self, event: SyncEvent) -> Result<(), String> {
        self.events.push(event);
        self.persist().await?;
        Ok(())
    }
    
    pub async fn persist(&self) -> Result<(), String> {
        let json = serde_json::to_string(&self.events)?;
        tokio::fs::write(&self.queue_file, json).await?;
        Ok(())
    }
}
```

### Reconnection & Replay

When extension reconnects:

```rust
pub async fn on_reconnect(&mut self) -> Result<(), String> {
    // 1. Load queued events from disk
    let queued = self.load_offline_queue().await?;
    
    // 2. Exchange causality vectors with UCM
    let cc_vector = self.ucm_state.sync_meta.causality_vector.clone();
    let server_vector = self.request_causality_vector().await?;
    
    // 3. Filter queued events: only apply if newer than server knows
    let new_events: Vec<_> = queued
        .into_iter()
        .filter(|e| {
            let server_seq = server_vector.last_seen.get(&e.source).copied().unwrap_or(0);
            e.sequence > server_seq
        })
        .collect();
    
    // 4. Replay in order
    for event in new_events {
        self.apply_event(&event).await?;
    }
    
    // 5. Clear offline queue
    tokio::fs::remove_file(&self.offline_queue_file).await.ok();
    
    Ok(())
}
```

### CRDT-Style Ordering

To ensure deterministic conflict resolution offline, use **Lamport timestamps** + **extension tie-breaker**:

```rust
/// A globally ordered event timestamp
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OrderedTimestamp {
    /// Logical clock value (max seen + 1)
    pub logical_clock: u64,
    
    /// Tie-breaker: extension ID as integer
    pub extension_priority: u8,
}

impl OrderedTimestamp {
    pub fn next(last: u64, extension_id: ExtensionId) -> Self {
        Self {
            logical_clock: last + 1,
            extension_priority: extension_id as u8,
        }
    }
    
    pub fn from_network(logical: u64) -> u64 {
        logical + 1  // Advance on receiving external events
    }
}

// Example:
// Copilot edits at clock=5 => OrderedTimestamp(5, Copilot=1)
// CC edits at clock=5 => OrderedTimestamp(5, ClaudeCode=2)
// Total order: Copilot edit comes first (priority 1 < 2)
```

### Eventual Consistency

After any transient disconnect, the system converges to the same state:

```
Time:  t0         t1              t2              t3
       |          |               |               |
Copilot: edit A  (offline)       (reconnect)     (replay A+B)
CC:     edit B   (offline)  edit C (reconnect)   (replay B+C)

By t3:
- Copilot state = CC state = (original + A + B + C)
- File version incremented 3 times
- All events logged to Universe with causal links
```

---

## 9. Database Schema

### EncryptedStore (KDB) Tables

```sql
-- UCM state snapshots (one row)
CREATE TABLE ucm_unified_state (
    key TEXT PRIMARY KEY,  -- "unified_state"
    value BLOB NOT NULL,   -- serialized UnifiedContextState (encrypted)
    updated_at INTEGER NOT NULL
);

-- Active file tracking
CREATE TABLE ucm_active_files (
    file_uri TEXT PRIMARY KEY,
    content_hash TEXT NOT NULL,
    version INTEGER NOT NULL,
    source_extension TEXT NOT NULL,
    cursor_line INTEGER,
    cursor_col INTEGER,
    last_modified_ns INTEGER NOT NULL,
    pending_changes BLOB,  -- JSON array of TextDelta
    updated_at INTEGER NOT NULL
);
CREATE INDEX idx_ucm_active_files_modified ON ucm_active_files(last_modified_ns DESC);

-- Approval cache
CREATE TABLE ucm_approvals (
    signature TEXT PRIMARY KEY,
    approved INTEGER NOT NULL,  -- 0 or 1
    approved_at INTEGER NOT NULL,  -- Unix seconds
    ttl_seconds INTEGER NOT NULL,  -- 3600
    requested_by TEXT NOT NULL,    -- "copilot" or "claude_code"
    note TEXT,
    created_at INTEGER NOT NULL
);
CREATE INDEX idx_ucm_approvals_expiry 
    ON ucm_approvals(approved_at + ttl_seconds);

-- Conversation index (for fast lookup)
CREATE TABLE ucm_conversation_index (
    session_id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    origin_extension TEXT NOT NULL,
    linked_session_id TEXT,
    message_count INTEGER NOT NULL DEFAULT 0,
    merged_at_ns INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
Create INDEX idx_ucm_conv_linked ON ucm_conversation_index(linked_session_id);

-- Conversation links
CREATE TABLE ucm_conversation_links (
    link_id TEXT PRIMARY KEY,
    source_session_id TEXT NOT NULL,
    source_extension TEXT NOT NULL,
    linked_session_id TEXT NOT NULL,
    linked_extension TEXT NOT NULL,
    last_synced_message_id TEXT,
    synced_at_ns INTEGER,
    created_at INTEGER NOT NULL
);

-- Offline queue (for disconnected state)
CREATE TABLE ucm_offline_queue (
    event_id TEXT PRIMARY KEY,
    event_type TEXT NOT NULL,
    payload BLOB NOT NULL,
    timestamp_ns INTEGER NOT NULL,
    source_extension TEXT NOT NULL,
    applied INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL
);
Create INDEX idx_ucm_queue_applied ON ucm_offline_queue(applied);
```

### SQLite (AssistantStore) Integration

Extend existing tables:

```sql
-- Extend assistant_messages to link to UCM
ALTER TABLE assistant_messages ADD COLUMN ucm_message_id TEXT;
ALTER TABLE assistant_messages ADD COLUMN ucm_synced_at INTEGER;

-- Link conversations to UCM
ALTER TABLE assistant_sessions ADD COLUMN ucm_session_id TEXT;
ALTER TABLE assistant_sessions ADD COLUMN ucm_linked_session_id TEXT;
ALTER TABLE assistant_sessions ADD COLUMN ucm_origin_extension TEXT;

CREATE INDEX idx_messages_ucm_id ON assistant_messages(ucm_message_id);
```

### Universe Extensions

Add new event categories to `UniverseEvent`:

```rust
pub enum EventCategory {
    // ... existing ...
    SyncConflict,
    OfflineQueueFlush,
    ApprovalCacheMiss,
    CheckpointRestore,
}
```

---

## 10. API & Integration

### UCM Client Library (Extension-side)

```rust
/// Loaded by both extensions
pub struct UcmClient {
    socket_path: PathBuf,
    extension_id: ExtensionId,
    message_tx: tokio::sync::mpsc::UnboundedSender<SyncMessage>,
    event_rx: broadcast::Receiver<SyncEvent>,
}

impl UcmClient {
    /// Connect to UCM daemon
    pub async fn connect(extension_id: ExtensionId) -> Result<Self, String> {
        let socket_path = Self::socket_path();
        let stream = tokio::net::UnixStream::connect(&socket_path).await?;
        
        // Send identity
        let (tx, mut rx) = tokio::io::split(stream);
        let (message_tx, message_rx) = tokio::sync::mpsc::unbounded_channel();
        let (event_tx, event_rx) = broadcast::channel(1000);
        
        // Spawn reader task
        tokio::spawn(async move {
            while let Some(msg) = rx.read_frame().await {
                let _ = event_tx.send(msg);
            }
        });
        
        Ok(Self { socket_path, extension_id, message_tx, event_rx })
    }
    
    /// Notify file change
    pub async fn notify_file_change(
        &self,
        file_uri: &str,
        content: &str,
    ) -> Result<(), String> {
        self.message_tx.send(SyncMessage {
            message_type: "FileChange".to_string(),
            source: self.extension_id,
            payload: json!({
                "file_uri": file_uri,
                "content_hash": blake3::hash(content.as_bytes()).to_hex(),
                "version": self.get_current_version(),
            }),
        })?;
        Ok(())
    }
    
    /// Notify new conversation message
    pub async fn notify_message(
        &self,
        session_id: &str,
        message: &Message,
    ) -> Result<(), String> {
        self.message_tx.send(SyncMessage {
            message_type: "ConversationUpdate".to_string(),
            source: self.extension_id,
            payload: serde_json::to_value(message)?,
        })?;
        Ok(())
    }
    
    /// Check cached approval
    pub async fn check_approval(
        &self,
        signature: &str,
    ) -> Result<Option<bool>, String> {
        self.send_request(SyncMessage {
            message_type: "CheckApproval".to_string(),
            payload: json!({ "signature": signature }),
            // ...
        }).await
    }
    
    /// Cache approval
    pub async fn cache_approval(
        &self,
        signature: &str,
        approved: bool,
        ttl_seconds: u32,
    ) -> Result<(), String> {
        self.message_tx.send(SyncMessage {
            message_type: "CacheApproval".to_string(),
            payload: json!({
                "signature": signature,
                "approved": approved,
                "ttl_seconds": ttl_seconds,
            }),
        })?;
        Ok(())
    }
    
    /// Subscribe to file changes
    pub fn subscribe_file_changes(&mut self) -> broadcast::Receiver<FileChangeEvent> {
        // Returns filtered event stream for file changes only
        todo!()
    }
    
    /// Subscribe to conversation updates
    pub fn subscribe_conversation(&mut self) -> broadcast::Receiver<ConversationEvent> {
        // Returns filtered event stream
        todo!()
    }
    
    /// Restore checkpoint
    pub async fn restore_checkpoint(&self, checkpoint_id: &str) -> Result<(), String> {
        self.send_request(SyncMessage {
            message_type: "RestoreCheckpoint".to_string(),
            payload: json!({ "checkpoint_id": checkpoint_id }),
        }).await
    }
    
    /// List available checkpoints
    pub async fn list_checkpoints(&self) -> Result<Vec<CheckpointInfo>, String> {
        self.send_request(SyncMessage {
            message_type: "ListCheckpoints".to_string(),
            payload: json!({}),
        }).await
    }
}
```

### UCM Server (Daemon-side)

```rust
pub struct UcmServer {
    state: Arc<Mutex<UnifiedContextState>>,
    universe: Arc<Universe>,
    event_bus: Arc<SystemEventBus>,
    clients: Arc<Mutex<HashMap<ExtensionId, ClientSession>>>,
}

impl UcmServer {
    /// Main event loop
    pub async fn run(db_path: &Path, listen_addr: &str) -> Result<(), String> {
        let server = Self::new(db_path).await?;
        
        // Listen on Unix socket
        let listener = tokio::net::UnixListener::bind(listen_addr)?;
        
        loop {
            let (stream, _) = listener.accept().await?;
            let server = server.clone();
            
            tokio::spawn(async move {
                if let Err(e) = server.handle_client(stream).await {
                    tracing::error!("Client error: {}", e);
                }
            });
        }
    }
    
    /// Handle client connection
    async fn handle_client(&self, mut stream: UnixStream) -> Result<(), String> {
        // Identify which extension
        let identity_msg = stream.read_frame().await?;
        let extension_id: ExtensionId = serde_json::from_value(
            identity_msg.payload["extension_id"].clone()
        )?;
        
        // Register client
        let (tx, mut rx) = tokio::io::split(stream);
        let client_tx = tokio::sync::mpsc::unbounded_channel().0;
        self.clients.lock().await.insert(extension_id, ClientSession { tx: client_tx });
        
        // Message loop
        while let Some(msg) = rx.read_frame().await {
            self.process_message(&msg, extension_id).await?;
        }
        
        // Unregister on disconnect
        self.clients.lock().await.remove(&extension_id);
        Ok(())
    }
    
    /// Dispatch message to handler
    async fn process_message(
        &self,
        msg: &SyncMessage,
        source: ExtensionId,
    ) -> Result<(), String> {
        match msg.message_type.as_str() {
            "FileChange" => self.handle_file_change(msg, source).await,
            "ConversationUpdate" => self.handle_conversation_update(msg, source).await,
            "CheckApproval" => self.handle_approval_check(msg, source).await,
            "CacheApproval" => self.handle_approval_cache(msg, source).await,
            "RestoreCheckpoint" => self.handle_checkpoint_restore(msg, source).await,
            "ListCheckpoints" => self.handle_list_checkpoints(msg, source).await,
            _ => Err(format!("Unknown message type: {}", msg.message_type)),
        }
    }
    
    /// Broadcast to all connected clients except source
    async fn broadcast(
        &self,
        event: SyncEvent,
        except: Option<ExtensionId>,
    ) -> Result<(), String> {
        let clients = self.clients.lock().await;
        for (ext_id, session) in clients.iter() {
            if Some(*ext_id) == except {
                continue;
            }
            session.tx.send(event.clone())?;
        }
        Ok(())
    }
}
```

### Error Handling

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcmError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl From<std::io::Error> for UcmError {
    fn from(e: std::io::Error) -> Self {
        Self {
            code: "IO_ERROR".to_string(),
            message: e.to_string(),
            details: None,
        }
    }
}

pub enum ErrorCode {
    NotConnected,
    InvalidMessage,
    ConflictDetected,
    OfflineQueueFull,
    ApprovalDenied,
    CheckpointNotFound,
}
```

---

## 11. Testing Strategy

### Unit Tests

**File: `crates/bonsai-ucm/tests/unit_conflict_resolution.rs`**

```rust
#[tokio::test]
async fn test_conflict_resolution_lww() {
    let copilot = ActiveFileState {
        content: "fn main() {\n  println!(\"copilot\");\n}".to_string(),
        last_modified_ns: 1000,
        ..Default::default()
    };
    let cc = ActiveFileState {
        content: "fn main() {\n  println!(\"cc\");\n}".to_string(),
        last_modified_ns: 2000,
        ..Default::default()
    };
    
    let resolved = resolve_conflict_lww(&copilot, &cc);
    assert_eq!(resolved.content, cc.content);  // CC is newer
}

#[tokio::test]
async fn test_3way_merge_non_overlapping() {
    let original = "line1\nline2\nline3\n";
    let copilot_edit = "line1_copilot\nline2\nline3\n";  // Edit line 1
    let cc_edit = "line1\nline2\nline3_cc\n";           // Edit line 3
    
    let result = try_merge(original, copilot_edit, cc_edit).unwrap();
    let expected = "line1_copilot\nline2\nline3_cc\n";
    assert_eq!(result, expected);
}

#[tokio::test]
async fn test_approval_cache_hit() {
    let mut cache = ApprovalCacheState::default();
    let sig = approval_signature("write_file", "src/main.rs", Some("hash123"));
    
    cache.approvals.insert(sig.clone(), CachedApproval {
        approved: true,
        approved_at: now_unix_seconds(),
        ttl_seconds: 3600,
        requested_by: ExtensionId::Copilot,
        note: None,
    });
    
    assert!(cache.approvals.get(&sig).is_some());
    assert!(!cache.approvals.get(&sig).unwrap().is_expired());
}

#[tokio::test]
async fn test_approval_cache_expiration() {
    let mut cache = ApprovalCacheState::default();
    let sig = approval_signature("write_file", "src/main.rs", Some("hash123"));
    
    cache.approvals.insert(sig.clone(), CachedApproval {
        approved: true,
        approved_at: now_unix_seconds() - 5000,  // 5000 seconds ago
        ttl_seconds: 3600,  // 1 hour
        requested_by: ExtensionId::Copilot,
        note: None,
    });
    
    assert!(cache.approvals.get(&sig).unwrap().is_expired());
}

#[tokio::test]
async fn test_conversation_deduplication() {
    let msg1 = Message {
        content: "What is Rust?".to_string(),
        content_hash: blake3::hash(b"What is Rust?").to_hex().to_string(),
        ..Default::default()
    };
    let msg2 = msg1.clone();
    
    let merged = dedup_messages(vec![msg1, msg2]);
    assert_eq!(merged.len(), 1);
}
```

### Integration Tests

**File: `crates/bonsai-ucm/tests/integration_sync.rs`**

```rust
#[tokio::test]
async fn test_file_change_propagation() {
    let (server, mut copilot_client, mut cc_client) = setup_test_env().await;
    
    // Copilot edits file
    copilot_client.notify_file_change(
        "file:///test.rs",
        "fn main() {}",
    ).await.unwrap();
    
    // CC receives change
    let event = cc_client.next_event().await.unwrap();
    assert_eq!(event.message_type, "FileChange");
}

#[tokio::test]
async fn test_offline_queue_and_replay() {
    let (server, mut client) = setup_test_env().await;
    
    // Simulate disconnect
    server.disconnect_client(ExtensionId::Copilot).await;
    
    // Queue changes while offline
    for i in 0..5 {
        client.notify_file_change(
            &format!("file:///test{}.rs", i),
            &format!("fn main() {{ // {} }}", i),
        ).await.unwrap();
    }
    
    // Reconnect
    server.reconnect_client(ExtensionId::Copilot).await;
    
    // Verify all queued events were replayed
    for i in 0..5 {
        let event = client.next_event().await.unwrap();
        assert!(event.payload["file_uri"].as_str().unwrap().contains(&i.to_string()));
    }
}

#[tokio::test]
async fn test_checkpoint_restore_atomic() {
    let (server, copilot, cc) = setup_test_env().await;
    
    // Create checkpoint
    let checkpoint_id = server.create_checkpoint("test").await.unwrap();
    
    // Modify files
    copilot.notify_file_change("file:///a.rs", "modified a").await.unwrap();
    copilot.notify_file_change("file:///b.rs", "modified b").await.unwrap();
    
    // Restore from CC
    cc.restore_checkpoint(&checkpoint_id).await.unwrap();
    
    // Verify both files reverted atomically
    let state_a = server.get_file_state("file:///a.rs").await.unwrap();
    let state_b = server.get_file_state("file:///b.rs").await.unwrap();
    assert_eq!(state_a.version, state_b.version);  // Same restore point
}

#[tokio::test]
async fn test_approval_deduplication() {
    let (server, copilot, cc) = setup_test_env().await;
    
    // Copilot approves write
    let sig = approval_signature("write_file", "src/main.rs", Some("hash123"));
    copilot.cache_approval(&sig, true, 3600).await.unwrap();
    
    // CC checks same approval
    let cached = cc.check_approval(&sig).await.unwrap();
    assert_eq!(cached, Some(true));
}

#[tokio::test]
async fn test_conversation_linking() {
    let (server, copilot, cc) = setup_test_env().await;
    
    // Copilot creates conversation
    let session1 = copilot.new_conversation("Test").await.unwrap();
    copilot.send_message(&session1, "What is Rust?").await.unwrap();
    
    // CC links to conversation
    let conversations = cc.list_conversations().await.unwrap();
    assert_eq!(conversations.len(), 1);
    assert_eq!(conversations[0].linked_session_id, Some(session1));
}
```

### Offline Scenario Testing

```rust
#[tokio::test]
async fn test_offline_scenario_both_extensions() {
    let (server, copilot, cc) = setup_test_env().await;
    
    // Both go offline
    server.disconnect_client(ExtensionId::Copilot).await;
    server.disconnect_client(ExtensionId::ClaudeCode).await;
    
    // Both make changes independently
    copilot.notify_file_change("file:///test.rs", "copilot edit").await.unwrap();
    cc.notify_file_change("file:///test.rs", "cc edit").await.unwrap();
    
    // Copilot reconnects first
    server.reconnect_client(ExtensionId::Copilot).await;
    let copilot_version = copilot.get_file_version("file:///test.rs").await.unwrap();
    
    // CC reconnects
    server.reconnect_client(ExtensionId::ClaudeCode).await;
    let cc_version = cc.get_file_version("file:///test.rs").await.unwrap();
    
    // Versions converge to deterministic order
    assert_eq!(copilot_version, cc_version);
}
```

### Replay Testing (Using Universe Logs)

```rust
#[tokio::test]
async fn test_replay_from_universe_events() {
    let (server, copilot, cc) = setup_test_env().await;
    
    // Perform some operations
    copilot.notify_file_change("file:///a.rs", "content a").await.unwrap();
    cc.notify_file_change("file:///b.rs", "content b").await.unwrap();
    copilot.send_message("session1", "test message").await.unwrap();
    
    // Export universe events
    let events = server.universe.store.all_events().await.unwrap();
    
    // Create fresh UCM with same database
    let fresh_server = UcmServer::restore_from_universe(&events).await.unwrap();
    
    // Verify state matches
    let original_file = server.get_file_state("file:///a.rs").await.unwrap();
    let restored_file = fresh_server.get_file_state("file:///a.rs").await.unwrap();
    assert_eq!(original_file.content_hash, restored_file.content_hash);
}
```

---

## 12. Security Considerations

### Encryption at Rest

All UCM state in EncryptedStore is encrypted with `ChaCha20-Poly1305`:

```rust
pub fn encrypt_state(
    state: &UnifiedContextState,
    key: &[u8; 32],
) -> Result<Vec<u8>, String> {
    let plaintext = serde_json::to_vec(state)?;
    let nonce = chacha20poly1305::Nonce::default();  // Use proper nonce generation
    let cipher = ChaCha20Poly1305::new(key.into());
    
    cipher.encrypt(&nonce, plaintext.as_ref())
        .map_err(|_| "Encryption failed".to_string())
}
```

### IPC Socket Permissions

Unix socket created with `0600` (read/write owner only):

```rust
// On Unix
let socket = tokio::net::UnixListener::bind(&socket_path)?;
let metadata = std::fs::metadata(&socket_path)?;
let mut perms = metadata.permissions();
perms.set_mode(0o600);
std::fs::set_permissions(&socket_path, perms)?;
```

### No Plaintext in Transit

- All messages on IPC are encrypted (TLS not needed, same user/process)
- File content in messages only includes hashes; full content loaded from CAS
- Conversation messages include only hash references

### Access Control

UCM does **not** enforce access control between extensions. Assumptions:

1. Both extensions run as same user
2. Both extensions run in same VS Code instance
3. User fully trusts both extensions

If different trust levels needed, add capability tokens:

```rust
pub struct CapabilityToken {
    pub extension_id: ExtensionId,
    pub permissions: Vec<Permission>,
    pub issued_at: u64,
    pub signature: Vec<u8>,
}

pub enum Permission {
    ReadConversation,
    WriteFile,
    RestoreCheckpoint,
    ViewApprovals,
}
```

### Audit Logging

All state changes logged to Universe with:

```rust
pub struct AuditEvent {
    pub event_id: String,
    pub operation: String,
    pub source_extension: ExtensionId,
    pub target: String,
    pub approved: bool,
    pub timestamp_ns: u64,
    pub signature: Vec<u8>,  // Ed25519
}
```

Users can query audit log:
```rust
pub async fn audit_log(
    &self,
    since_ns: u64,
    extension: Option<ExtensionId>,
) -> Result<Vec<AuditEvent>, String> {
    self.universe.store.query_events()
        .category(EventCategory::Checkpoint)  // or similar
        .since_timestamp(since_ns)
        .filter_extension(extension)
        .execute()
        .await
}
```

### Data Privacy

No UCM state is sent outside user's device:

- All encryption keys remain on device
- Universe events stay in user's database
- Approval cache never leaves EncryptedStore
- IPC communication is local-only (no network)

If Bonsai Cloud integration added later:

```
UCM State (sensitive)
    ↓
Anonymize & aggregate
    ↓
Send to cloud (metrics only)
```

---

## 13. Implementation Roadmap

### Phase 1: Foundation (2-3 weeks)

1. **Data structures** – Rust types for `UnifiedContextState`, messages, etc.
2. **EncryptedStore integration** – Extend KDB with UCM tables
3. **IPC communication** – Unix socket / named pipe setup
4. **Basic file sync** – Single file change propagation

**Success criteria:** Editing file in Copilot → Claude Code sees change within 100ms

### Phase 2: Approval & Conversation (2 weeks)

1. **Approval cache** – Implement deduplication, TTL, UACS integration
2. **Conversation merging** – Link sessions, deduplicate messages
3. **Knowledge module sharing** – Broadcast loaded modules

**Success criteria:** User approves operation in Copilot, CC doesn't re-ask

### Phase 3: Checkpoints & Offline (2 weeks)

1. **Checkpoint restore** – Atomic multi-file restore
2. **Offline queue** – Persist changes, replay on reconnect
3. **Causality tracking** – CRDT-style ordering

**Success criteria:** Both extensions offline, make changes, reconnect → deterministic merge

### Phase 4: Conflict Resolution (1 week)

1. **LWW implementation** – Simple last-write-wins
2. **3-way merge** – For non-overlapping edits
3. **Conflict markers** – Detect and warn user

**Success criteria:** Simultaneous edits resolved deterministically

### Phase 5: Testing & Polish (2 weeks)

1. **Unit tests** – Conflict resolution, approval cache, conversation merging
2. **Integration tests** – Both extensions simultaneously modifying state
3. **Offline scenarios** – Queue, replay, causality
4. **Performance tuning** – <100ms propagation latency

**Success criteria:** 100+ tests passing, <100ms propagation, no data loss

---

## 14. Metrics & Observability

### Key Metrics

```rust
pub struct UcmMetrics {
    /// File change propagation latency (milliseconds)
    pub file_change_latency_ms: Histogram,
    
    /// Number of active clients connected
    pub active_clients: Gauge,
    
    /// Approval cache hit rate (0-100%)
    pub approval_cache_hit_rate: Gauge,
    
    /// Offline event queue length
    pub offline_queue_size: Gauge,
    
    /// Conflict resolution rate (LWW, merge, manual)
    pub conflict_type_counter: Counter,
    
    /// Conversation merge quality (dedup rate)
    pub message_dedup_rate: Gauge,
}

impl UcmMetrics {
    pub fn record_file_change(&self, latency_ms: u64) {
        self.file_change_latency_ms.observe(latency_ms as f64);
    }
}
```

### Logging

All UCM operations logged with structured logging:

```rust
tracing::info!(
    event = "file_change_received",
    file_uri = "file:///src/main.rs",
    source_extension = "copilot",
    version = 42,
    latency_ms = 15,
);

tracing::warn!(
    event = "conflict_detected",
    file_uri = "file:///src/lib.rs",
    copilot_version = 42,
    cc_version = 41,
    resolution_strategy = "lww",
);
```

### Dashboard

Add UCM section to UACS dashboard:

```
Unified Context Manager Status
├─ Connected Clients: 2 (Copilot, Claude Code)
├─ File Sync Latency: 12ms (p99: 45ms)
├─ Approval Cache: 847 entries, 92% hit rate
├─ Offline Queue: 0 events
├─ Recent Conflicts: 0 in past hour
└─ Conversation Sync: 23 merged conversations
```

---

## Appendix: Example Workflows

### Workflow 1: User Switches Extensions Mid-Conversation

```
1. User: [Copilot Chat] "Explain Rust ownership"
2. Copilot: [sends message to UCM] new conversation created
3. User: [5 minutes later] Opens Claude Code
4. Claude Code: [connects to UCM] loads Copilot conversation
5. User: [Claude Code] "What about lifetimes?" (@-mentions linked conversation)
6. Claude Code: [sends message with context from Copilot chat]
7. Claude Code: [renders message in both Copilot and CC]
8. User sees seamless continuation
```

### Workflow 2: Simultaneous Edits with Merge

```
1. User: [Copilot] Modifies lines 1-10 in src/main.rs
2. User: [Claude Code] Simultaneously modifies lines 50-60
3. UCM detects: version mismatch (copilot=42, cc=41)
4. UCM attempts 3-way merge: original + copilot changes + cc changes
5. Result: non-overlapping edits → merge succeeds
6. Copilot sees: file updated with CC's edits (total 3 changes)
7. Claude Code sees: file updated with Copilot's edits
8. Both show version 43, content_hash matches
9. User continues unaware of merge (it worked!)
```

### Workflow 3: Approval Deduplication

```
1. User: [Copilot Chat] "write_file('src/auth.rs', content)"
2. Copilot shows approval dialog: "Write auth.rs?"
3. User clicks "Approve" (1 hour)
4. Copilot executes write_file tool
5. UACS caches approval: signature=hash("write_file|src/auth.rs|content_hash")
6. User: [Claude Code Chat] "Update the auth module in src/auth.rs"
7. Claude Code calls approval cache: check(signature)
8. Cache hit! Approval valid for another 50 minutes
9. Claude Code immediately proceeds, zero user interaction
10. Both extensions can call write_file(src/auth.rs, SAME content) without re-asking
```

### Workflow 4: Offline Scenario

```
1. User edits src/main.rs in Copilot
2. Network disconnect (IPC socket closes)
3. User continues editing in Claude Code (queued locally)
4. User adds checkpoint in Claude Code (queued)
5. User sends message in Copilot Chat (queued)
6. Network reconnect (IPC socket reopens)
7. UCM replays queued events:
   - src/main.rs edit #1 (Copilot)
   - src/lib.rs edit #2 (Claude Code)
   - Checkpoint created
   - Message sent
8. Both extensions converge to same state
9. File versions: src/main.rs=44, src/lib.rs=18, checkpoint_state=updated
```

---

## Conclusion

The Unified Context Manager enables GitHub Copilot and Claude Code to function as true complementary agents, seamlessly synchronizing state while maintaining simplicity, performance, and offline robustness.

Key design principles:
- **User-transparent** – Sync happens automatically, user never thinks about it
- **Deterministic** – Same conflicts always resolve the same way
- **Offline-first** – Extensions work fully offline, sync when reconnected
- **Auditable** – Every state change logged to Universe
- **Fast** – <100ms propagation latency, <1ms cache lookups
- **Secure** – Encrypted at rest, local IPC only, audit trails

The system leverages Bonsai's proven infrastructure (Universe, UACS, SystemEventBus) while remaining agnostic to the extensions themselves, making it extensible to other AI tools in the future.

