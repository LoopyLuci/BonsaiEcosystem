# Bonsai Sovereign Proxy Layer – Complete Architecture Design

**Status:** Design Phase (no implementation yet)  
**Target:** 500+ lines of TypeScript/Node.js proxy code  
**Audience:** Engineering team, extension developers, API consumers

---

## Executive Summary

The **Bonsai Sovereign Proxy Layer** is a TypeScript/Node.js middleware that sits between GitHub Copilot + Claude Code VS Code extensions and the Bonsai Ecosystem. Its mission is to:

1. **Intercept** outbound HTTP requests from Copilot and Claude Code (API calls, streaming responses)
2. **Marshal** Copilot tools into Bonsai MCP format, and vice versa
3. **Bridge** authentication (OAuth → capability tokens → offline identities)
4. **Unify** session state across extensions and Bonsai
5. **Route** execution intelligently across local models, hybrid, and cloud tiers
6. **Gate** sensitive operations with human-in-the-loop approval
7. **Observe** every operation via structured telemetry and fallback gracefully

The proxy runs as a **VS Code extension or standalone Node.js service**, auto-discovers the local Bonsai daemon via Echo QUIC discovery, and operates offline-first with elegant cloud fallback.

---

## 1. System Diagram & Architecture Overview

### High-Level Component Topology

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          VS Code Editor                                  │
├───────────────────────────────────────────────────────────────────────┤
│   GitHub Copilot         │        Claude Code Extension                │
│   (chat + completions)   │        (Claude Code + agent tools)          │
└────────────────┬──────────┴────────────────┬───────────────────────────┘
                 │ HTTP Requests             │ HTTP Requests
                 │ (api.github.com,          │ (api.anthropic.com,
                 │  api.openai.com)          │  claude.ai)
                 │                           │
         ┌───────▼───────────────────────────▼────────┐
         │   BONSAI SOVEREIGN PROXY LAYER              │
         │   (VS Code ext or Node.js service)         │
         ├─────────────────────────────────────────────┤
         │ HTTP Interception Engine                    │
         │ ├─ Request/Response Translator              │
         │ ├─ Streaming Handler (SSE, chunked)        │
         │ └─ Credential Manager (OAuth, token keys)  │
         │                                             │
         │ Tool Marshalling System                     │
         │ ├─ Copilot → MCP Tool Schema Mapper       │
         │ ├─ Parameter Translation Engine             │
         │ ├─ Result Format Converter                  │
         │ └─ Tool Registry & Discovery                │
         │                                             │
         │ Authentication Bridge                       │
         │ ├─ OAuth Token Exchange                     │
         │ ├─ Ed25519 Offline Identity Keys           │
         │ ├─ OS Keychain Storage                      │
         │ └─ Token Refresh & Expiry Manager          │
         │                                             │
         │ Session & State Manager                     │
         │ ├─ Unified Session Format (JSON)           │
         │ ├─ Checkpoint Persistence (SQLite)         │
         │ ├─ Multi-device Sync (Echo QUIC)           │
         │ └─ Session Recovery on Restart             │
         │                                             │
         │ Dual-Mode Execution Router                  │
         │ ├─ Performance Meter (local model latency)  │
         │ ├─ Fallback Tier Selector                   │
         │ ├─ Load Balancer (multi-device)            │
         │ └─ Overload Protection & Backpressure       │
         │                                             │
         │ HITL Approval Gate                          │
         │ ├─ Risk Classifier (low/med/high)          │
         │ ├─ VS Code Modal Dialog Manager             │
         │ ├─ Confidence Scorer                        │
         │ └─ Approval Cache                           │
         │                                             │
         │ Error Recovery & Resilience                 │
         │ ├─ Timeout Manager & Watchdog              │
         │ ├─ Exponential Backoff Retry                │
         │ ├─ Partial Failure Handler                  │
         │ ├─ Rollback & State Cleanup                 │
         │ └─ User-Friendly Error Reporting            │
         │                                             │
         │ Performance Optimization                    │
         │ ├─ Request Batching Engine                  │
         │ ├─ Response Cache (safe patterns)           │
         │ ├─ Prefetch Manager (models, knowledge)    │
         │ ├─ MCP Connection Pool                      │
         │ └─ Latency Budget Enforcer                  │
         └──────┬──────────────────────────┬───────────┘
                │ MCP HTTP                 │ Echo QUIC
                │ Protocol                 │ Discovery
                │                          │
    ┌───────────▼─────────────────────────▼───────────┐
    │   LOCAL BONSAI DAEMON (localhost:8080)          │
    │   ├─ MCP Server (HTTP endpoints)                │
    │   ├─ Tool Registry                              │
    │   ├─ RPC Handler                                │
    │   ├─ Session Store                              │
    │   └─ Local Model Inference (GGUF)               │
    └───────────┬──────────────────────────────────────┘
                │ P2P QUIC                 │ gRPC
                │ (TransferDaemon)         │ (cluster)
                │                          │
    ┌───────────▼──────────┐  ┌────────────▼────────┐
    │  Multi-Device Sync   │  │  Bonsai Cluster     │
    │  & State Replication │  │  (compute fabric)   │
    │  (phones, tablets)   │  └─────────────────────┘
    └──────────────────────┘

    FALLBACK ROUTING (if local unavailable):
    ├─ Tier 1: Local (lowest latency, privacy)
    ├─ Tier 2: Hybrid (local + cloud for large models)
    └─ Tier 3: Cloud (full cloud, if offline-first fails)
```

### Data Flow for a Typical Operation

**Scenario: User asks Copilot for code completion with context**

```
1. Copilot → HTTP POST /v1/completions
   - Sends request to api.openai.com (intercepted by proxy)
   
2. Proxy HTTP Interception Engine
   - Matches request pattern → /v1/completions
   - Looks up routing rules (local vs cloud)
   - Extracts auth token from header
   
3. Authentication Bridge
   - Checks if OAuth token is cached
   - If needed: exchanges OAuth for Bonsai capability token
   - If offline: uses Ed25519 offline identity
   
4. Tool Marshalling System
   - Extracts parameters from Copilot format
   - Converts to Bonsai MCP format
   
5. Session Manager
   - Retrieves or creates unified session
   - Attaches checkpoint context (if Claude Code)
   - Stores in session store
   
6. Dual-Mode Execution Router
   - Measures local model latency
   - Decides: local → hybrid → cloud
   - Initiates execution tier
   
7. MCP Client (proxy → local daemon)
   - Calls tool via HTTP: POST /call/{tool_name}
   - Streams response back via SSE
   
8. Response Translator
   - Converts MCP response → Copilot format
   - Applies streaming chunking if needed
   
9. Proxy → Copilot (HTTP 200)
   - Returns completion in Copilot format
   - Upstream: Copilot renders in editor
   
10. Telemetry & Monitoring
    - Logs latency, execution tier, tool success/failure
    - Updates UACS dashboard with approval/denial rates
```

---

## 2. Component Breakdown (13 Major Components)

### 2.1 HTTP Interception Engine
**Responsibility:** Intercept, pattern-match, and translate HTTP requests  
**Key Interfaces:**
- `intercept(req: Request): InterceptionResult` – returns whether to intercept
- `translateRequest(req: Request): DownstreamRequest` – convert to Bonsai format
- `translateResponse(resp: Response): UpstreamResponse` – convert back

**Key Files:**
```
src/
├── interceptor/
│   ├── patterns.ts        # Request pattern matching (regex + URI templates)
│   ├── engine.ts          # Core interception logic
│   ├── request-translator.ts
│   └── response-translator.ts
```

**Dependencies:**
- `http-proxy` or custom socket interception
- `url` module (Node.js stdlib)
- Bonsai error types

**External Dependencies:**
- `axios` (HTTP client)
- `uuid` (request tracking)

---

### 2.2 Tool Marshalling System
**Responsibility:** Map between Copilot tool schemas and Bonsai MCP schemas  
**Key Interfaces:**
- `mapCopilotToMCP(tool: CopilotTool): MCPTool` – tool schema conversion
- `marshalParameters(copilotParams: object, schema: MCPSchema): object` – parameter translation
- `unmarshalResult(mcpResult: object, schema: CopilotSchema): object` – result translation

**Key Files:**
```
src/
├── marshalling/
│   ├── tool-registry.ts       # Cache + lookup for tool definitions
│   ├── copilot-schema.ts      # Copilot tool schema types
│   ├── mcp-schema.ts          # MCP tool schema types
│   ├── schema-mapper.ts       # Bidirectional conversion logic
│   ├── parameter-marshaller.ts # Type coercion engine
│   └── result-unmarshaller.ts # Result translation
```

**Data Schema Examples:**

**Copilot Tool Definition:**
```typescript
interface CopilotTool {
  id: string;
  name: string;
  description: string;
  parameters: {
    type: "object";
    properties: {
      [key: string]: {
        type: string;      // "string", "number", "boolean", "array"
        description: string;
        enum?: string[];
        pattern?: string;  // regex
      };
    };
    required: string[];
  };
}
```

**MCP Tool Definition (from daemon):**
```typescript
interface MCPTool {
  name: string;
  description: string;
  input_schema: {
    type: "object";
    properties: {
      [key: string]: {
        type: string;      // "string", "number", "boolean", "array", "object"
        description: string;
        items?: object;    // for arrays
        enum?: any[];
      };
    };
    required: string[];
  };
}
```

**Mapping Strategy:**
- If MCP tool has no Copilot equivalent: store as "unsupported" and skip
- If Copilot tool has no MCP equivalent: use fallback cloud provider
- Type coercion: convert `string` ↔ `number`, truncate arrays if MCP expects single value, etc.
- Validation: ensure parameters match schema before calling MCP

---

### 2.3 Authentication Bridge Architecture
**Responsibility:** Manage credentials, tokens, and identity across cloud and offline modes  
**Key Interfaces:**
- `exchangeOAuthToken(oauth: string): CapabilityToken` – OAuth → Bonsai token
- `generateOfflineIdentity(): EdKey` – create Ed25519 keypair
- `refreshToken(expired: CapabilityToken): CapabilityToken` – handle token expiry
- `getCredentials(service: "github" | "anthropic"): Credentials` – retrieve from keychain

**Key Files:**
```
src/
├── auth/
│   ├── oauth-manager.ts       # OAuth flow (code grant → token)
│   ├── capability-token.ts    # Bonsai capability token (JWT-like)
│   ├── offline-identity.ts    # Ed25519 key generation & storage
│   ├── keychain-store.ts      # OS keychain integration
│   ├── token-cache.ts         # In-memory + persistent cache
│   └── credential-manager.ts  # Unified interface
```

**Credential Storage Strategy:**

| Credential Type | Storage Location | Encryption | Lifetime |
|---|---|---|---|
| OAuth Token (GitHub) | OS Keychain | OS-managed | 1 year (refresh) |
| OAuth Token (Anthropic) | OS Keychain | OS-managed | Per-session |
| Capability Token (Bonsai) | In-memory + file (~1KB) | AES-256-GCM | 24 hours (refresh) |
| Offline Ed25519 Key | `~/.bonsai/identity/offline.key` | AES-256-GCM | Indefinite (user-managed) |
| Session Token (local MCP) | In-memory only | None | Session lifetime |

**Offline Identity Mode:**
- When cloud is unavailable, proxy generates **Ed25519 keypair** and stores locally
- All requests signed with private key (ed25519ph signature)
- Local daemon validates signature against public key on file
- No cloud credentials needed once identity is created

**Token Expiry & Refresh:**
- Capability tokens expire after 24 hours
- Refresh endpoint: `POST /auth/refresh` (daemon)
- If refresh fails: fall back to OAuth re-exchange
- If all auth fails: offer offline mode to user

---

### 2.4 State Management & Session System
**Responsibility:** Maintain unified session state across extensions and devices  
**Key Interfaces:**
- `createSession(): Session` – create new session with unique ID
- `getSession(id: string): Session | null` – retrieve session
- `updateSession(id: string, state: Partial<Session>): void` – persist changes
- `syncSession(id: string, device: string): Promise<void>` – multi-device sync

**Key Files:**
```
src/
├── session/
│   ├── session-manager.ts     # Core session lifecycle
│   ├── session-schema.ts      # Unified session format (JSON)
│   ├── session-store.ts       # SQLite persistence
│   ├── checkpoint-store.ts    # Claude Code checkpoint management
│   ├── sync-engine.ts         # Multi-device sync (Echo QUIC)
│   └── session-recovery.ts    # Recovery on restart
```

**Unified Session Schema (JSON):**
```typescript
interface Session {
  // Identity
  session_id: string;           // UUID v7 (time-ordered)
  user_id: string;              // From OAuth or offline identity
  created_at: number;           // Unix ms
  last_accessed_at: number;     // Unix ms
  
  // Auth & Credentials
  auth: {
    mode: "oauth" | "offline";
    oauth_tokens?: {
      github?: string;          // access token
      anthropic?: string;
    };
    capability_token?: string;  // Bonsai daemon token
    offline_identity?: {
      public_key: string;       // base64(ed25519 pk)
      key_id: string;           // blake3(pk)
    };
  };
  
  // Execution Context
  context: {
    editor: "copilot" | "claude_code" | "both";
    file_path?: string;         // Currently open file
    language?: string;          // Language mode
    selected_text?: string;     // User selection (hashed for privacy)
    viewport?: {                // Editor viewport for context
      start_line: number;
      end_line: number;
    };
  };
  
  // Claude Code Specific
  checkpoints?: {
    checkpoint_id: string;      // UUID
    name: string;               // User-friendly name
    timestamp: number;          // Unix ms
    snapshot: object;           // Full checkpoint state
    bonsai_cas_key: string;     // CAS key for large snapshots
  }[];
  
  // State & Preferences
  settings: {
    execution_tier: "local" | "hybrid" | "cloud";  // User preference
    approval_required_for: ("file_write" | "execute" | "network")[];
    cache_enabled: boolean;
    telemetry_enabled: boolean;
  };
  
  // Tool & Execution History
  tools_used: {
    tool_name: string;
    timestamp: number;
    execution_tier: string;
    latency_ms: number;
    success: boolean;
    risk_classification?: "low" | "medium" | "high";
    approval_given?: boolean;
  }[];
}
```

**Session Storage:**
- **In-memory:** Current session (for low latency)
- **SQLite:** `~/.bonsai/sessions.db` (persistence across restarts)
- **CAS:** Large snapshots (Claude Code checkpoints) → BLAKE3 key stored in DB

**Multi-Device Sync (Echo QUIC):**
- Sync daemon establishes QUIC connection to other devices
- Session state replicated via append-only log
- Conflict resolution: last-write-wins + vector clocks
- Offline sync queue: batched on reconnection

---

### 2.5 Dual-Mode Execution Router
**Responsibility:** Decide which tier (local, hybrid, cloud) executes each operation  
**Key Interfaces:**
- `decide(operation: Operation): ExecutionTier` – select tier
- `measureLocalPerformance(): PerformanceMetrics` – benchmark local model
- `balanceLoad(devices: Device[]): DeviceAllocation` – distribute work

**Key Files:**
```
src/
├── execution-router/
│   ├── tier-selector.ts       # Tier decision logic
│   ├── performance-meter.ts   # Local model benchmarking
│   ├── load-balancer.ts       # Device load distribution
│   ├── overload-protection.ts # Backpressure & queueing
│   └── fallback-handler.ts    # Graceful tier fallback
```

**Three-Tier Fallback System:**

**Tier 1: Local** (Lowest Latency, Maximum Privacy)
- Execution: Local daemon (`localhost:8080`)
- Latency SLA: < 100ms (with model loaded)
- Model: 7B quantized (GGUF, Q4_K_M)
- Fallback trigger: daemon offline, model OOM, timeout > 5s
- Privacy: Code stays local, no upload

**Tier 2: Hybrid** (Balanced)
- Execution: Large operations split between local + cloud
- Example: semantic search on local embedding model + cloud reranking
- Model context: Large (Opus), local context: Small (7B)
- Latency SLA: < 2s
- Fallback trigger: local model performance degraded, user timeout

**Tier 3: Cloud** (Highest Capability, Lowest Privacy)
- Execution: Full operation on Claude 4+ or GPT-4
- Model: Opus/Sonnet or equivalent
- Latency SLA: < 10s
- Privacy: Code may upload to cloud (disclosed to user)
- Fallback: Only if Tier 1 and 2 fail, or user explicitly requests

**Decision Algorithm:**

```typescript
function decide(op: Operation): ExecutionTier {
  // 1. Check user preference
  if (userSettings.force_tier) return userSettings.force_tier;
  
  // 2. Check if operation requires approval (HITL)
  if (needsApproval(op) && !hasApproval(op)) {
    // Show approval modal (returns tier + approval decision)
    return askUserForApproval(op);
  }
  
  // 3. Measure local performance
  const localMetrics = await measureLocalPerformance();
  
  // 4. Estimate latency for each tier
  const localLatency = estimateLatency(op, "local");
  const hybridLatency = estimateLatency(op, "hybrid");
  const cloudLatency = estimateLatency(op, "cloud");
  
  // 5. Select tier based on latency + privacy budget
  if (localLatency < 100 && privacyBudget >= "high") {
    return "local";
  } else if (hybridLatency < 2000 && privacyBudget >= "medium") {
    return "hybrid";
  } else {
    return "cloud";
  }
}
```

**Performance Meter Details:**

Proxy benchmarks the local model on startup and periodically:
- Latency: P50, P95, P99 for typical operations
- Memory: Peak RSS during operation
- Cache hit rate: How often results are reusable

```typescript
interface PerformanceMetrics {
  latency_p50_ms: number;
  latency_p95_ms: number;
  latency_p99_ms: number;
  memory_peak_mb: number;
  cache_hit_rate: number;    // 0.0-1.0
  model_loaded: boolean;
  last_updated_at: number;   // Unix ms
}
```

**Load Balancing (Multi-Device):**

If multiple devices available (desktop + phone):
- Distribute tasks based on available resources
- Prefer lower-latency devices
- Monitor thermal/battery constraints on phones
- Migrate tasks if device becomes unavailable

---

### 2.6 HITL (Human-in-the-Loop) Approval Gate
**Responsibility:** Show approval modals, classify risk, cache decisions  
**Key Interfaces:**
- `classify(operation: Operation): RiskLevel` – categorize risk
- `needsApproval(op: Operation): boolean` – determine if approval required
- `requestApproval(op: Operation): Promise<ApprovalDecision>` – show modal
- `cacheApproval(op: Operation, decision: Approved): void` – remember decision

**Key Files:**
```
src/
├── approval-gate/
│   ├── risk-classifier.ts     # Risk scoring engine
│   ├── confidence-scorer.ts   # Confidence % (0-100)
│   ├── modal-manager.ts       # VS Code approval UI
│   ├── approval-cache.ts      # Remember decisions
│   └── uacs-reporter.ts       # Report approvals to UACS
```

**Risk Classification:**

```typescript
enum RiskLevel {
  Low = "low",        // Safe operations (read-only)
  Medium = "medium",  // Moderate (file write, network call)
  High = "high",      // Dangerous (code execution, credential access)
}

interface RiskScore {
  level: RiskLevel;
  reasoning: string[];  // Human-readable reasons
  confidence: number;   // 0-100: how confident is this classification?
  suggested_approval: boolean;
}
```

**Risk Factors:**

| Operation | Factors | Risk Level |
|---|---|---|
| Code completion (read-only) | No file mutation, no network | Low |
| File write (user-confirmed) | File path, content length | Medium |
| Execute arbitrary command | Subprocess, environment vars | High |
| Read private file | `.env`, `~/.ssh`, `/etc/passwd` | High |
| Network call to unknown domain | Domain reputation, data sent | Medium-High |
| Model inference (local only) | Tier = local, no code upload | Low |
| Inference (cloud) | Code uploads to API | Medium-High |

**Confidence Scoring:**

- **Confidence = 100%** if: File already approved, operation is in allowlist, user explicitly approved same operation before
- **Confidence = 75-99%** if: Similar operation approved before (fuzzy match)
- **Confidence = 50-74%** if: Risk factors present but mitigated (e.g., cloud tier but user set privacy level)
- **Confidence < 50%** if: Novel operation, conflicting risk signals, or user's settings uncertain

**Approval Modal (VS Code):**

```
┌──────────────────────────────────────────────┐
│ ⚠️  HUMAN-IN-THE-LOOP APPROVAL REQUIRED      │
├──────────────────────────────────────────────┤
│                                              │
│ Operation: Write file "src/main.rs"          │
│ Risk Level: MEDIUM                           │
│ Confidence: 82%                              │
│                                              │
│ Execution Tier: Local (privacy: ✅ high)    │
│ Source: Claude Code Agent                    │
│                                              │
│ Reason for approval request:                 │
│  • File write outside current workspace      │
│  • Content 2.3 KB (large for auto-approval)  │
│  • User has approved 3/5 similar ops         │
│                                              │
│ Preview of changes:                          │
│  ─ Lines 10-20 (diff)                        │
│  ─ Lines 50-60 (diff)                        │
│                                              │
│ Bonsai Dashboard: [View in UACS]             │
│                                              │
│  [Deny]  [Approve Once]  [Approve Always]   │
└──────────────────────────────────────────────┘
```

**Approval Caching:**

```typescript
interface ApprovalRecord {
  operation_type: string;    // "file_write", "execute", etc.
  operation_hash: string;    // blake3(serialized operation)
  approved_at: number;       // Unix ms
  approved_by: string;       // user_id
  expires_at?: number;       // Optional expiry
  scope: "once" | "always";  // One-time or persistent
}
```

Cache key: `blake3(operation_type + file_path + hash(content))`  
Cache expires: 24 hours or user logout (whichever first)

**UACS Integration:**

Report all approvals/denials to UACS dashboard:
- `POST /api/uacs/events` with structured event
- Event format: `{ user_id, operation_type, risk_level, approved, timestamp }`
- Enables dashboard analytics: approval rates, risk trends, user behavior

---

### 2.7 Error Recovery & Resilience
**Responsibility:** Handle failures gracefully with retry logic, timeouts, rollbacks  
**Key Interfaces:**
- `withTimeout(op: Operation, ms: number): Promise<Result>` – enforce deadline
- `withRetry(op: Operation, maxAttempts: number): Promise<Result>` – exponential backoff
- `handlePartialFailure(results: Result[]): void` – partial success handling
- `rollback(op: Operation, state: State): void` – undo changes

**Key Files:**
```
src/
├── resilience/
│   ├── timeout-manager.ts     # Watchdog timers
│   ├── retry-engine.ts        # Exponential backoff with jitter
│   ├── circuit-breaker.ts     # Fail-fast if tier is down
│   ├── partial-failure.ts     # Handle some tools succeeding, some failing
│   ├── rollback-engine.ts     # Undo state changes
│   └── error-reporter.ts      # User-friendly error messages
```

**Timeout Strategy:**

| Operation Type | Timeout (ms) | Notes |
|---|---|---|
| Code completion (local) | 500 | Model must respond quickly |
| Chat with tool calls (local) | 2000 | Some tools may be slow |
| Inference (cloud) | 10000 | Network + model latency |
| File write | 5000 | Disk I/O + approval modal |
| Model download | 120000 | Large files, network-dependent |

**Retry Strategy:**

```typescript
async function withRetry<T>(
  operation: () => Promise<T>,
  maxAttempts: number = 3,
  initialDelayMs: number = 100,
  maxDelayMs: number = 5000
): Promise<T> {
  let lastError: Error;
  
  for (let attempt = 1; attempt <= maxAttempts; attempt++) {
    try {
      return await operation();
    } catch (error) {
      lastError = error;
      
      // Don't retry if error is permanent (auth, validation)
      if (!isTransient(error)) {
        throw error;
      }
      
      if (attempt < maxAttempts) {
        // Exponential backoff with jitter: delay = min(init * 2^(attempt-1) + random, max)
        const delay = Math.min(
          initialDelayMs * Math.pow(2, attempt - 1) + Math.random() * 100,
          maxDelayMs
        );
        await sleep(delay);
      }
    }
  }
  
  throw lastError;
}

function isTransient(error: Error): boolean {
  // Don't retry: auth errors, validation errors, permanent resource not found
  if (error.code === "UNAUTHORIZED" || error.code === "VALIDATION_ERROR") {
    return false;
  }
  // Retry: network timeouts, 5xx, service unavailable
  return error.code === "TIMEOUT" || error.code === "SERVICE_UNAVAILABLE";
}
```

**Partial Failure Handling:**

When multiple tools are called in sequence and one fails:
1. Return partial results (tools 1, 2, 4 succeeded; tool 3 failed)
2. Show user which tool failed and why
3. Offer options: retry, skip, fallback to cloud
4. Don't lose work already done

```typescript
interface PartialFailureResult<T> {
  succeeded: T[];
  failed: {
    operation: T;
    error: Error;
    retryable: boolean;
  }[];
  partial_success: boolean;
}
```

**Rollback Mechanism:**

For file writes and state mutations:
- Before: snapshot current state to `.bonsai/rollback/{op_id}.json`
- During: execute operation
- On failure: restore from snapshot, remove `.bonsai/rollback/{op_id}.json`
- User can manually rollback within 1 hour via UI

**Error Messages (User-Friendly):**

Bad:
```
Error: EAI_AGAIN (name resolution failure)
```

Good:
```
Network connection lost. Bonsai tried to reach the cloud API but DNS resolution failed. 
Falling back to local mode... [Using local model] ✓

Your request will work offline, but with reduced capability (no web search).
Would you like to [Retry cloud] or [Continue with local]?
```

---

### 2.8 Performance Optimization
**Responsibility:** Minimize latency via caching, batching, prefetching, pooling  
**Key Interfaces:**
- `batchRequests(reqs: Request[]): BatchResult` – combine multiple requests
- `getCached(key: string): CachedResponse | null` – retrieve cached response
- `prefetch(resources: Resource[]): void` – proactively load
- `withConnectionPool(fn: (conn: Connection) => Promise<T>): Promise<T>` – reuse connections

**Key Files:**
```
src/
├── optimization/
│   ├── batch-engine.ts        # Request batching
│   ├── response-cache.ts      # Safe caching patterns
│   ├── prefetch-engine.ts     # Proactive loading
│   ├── connection-pool.ts     # MCP server connections
│   └── latency-budget.ts      # Enforce latency SLOs
```

**Request Batching:**

Combine multiple independent requests into single MCP call:

```typescript
// Before: 3 requests, 3 roundtrips
const file1 = await readFile("main.rs");
const file2 = await readFile("lib.rs");
const file3 = await readFile("tests.rs");

// After: 1 request, 1 roundtrip (if MCP supports batching)
const [file1, file2, file3] = await batchReadFiles(["main.rs", "lib.rs", "tests.rs"]);
```

Batching rules:
- Only batch operations with same execution tier
- Max batch size: 10 requests (diminishing returns)
- Batch timeout: 50ms (if not full, send anyway)
- Not applicable to streaming responses

**Response Caching:**

Only cache operations that are deterministic and side-effect-free:

| Operation | Cacheable? | TTL | Notes |
|---|---|---|---|
| File read (no .env, .ssh, .key) | Yes | 5 min | Invalidate on file change |
| Tool definition list | Yes | 24 hr | Rarely changes |
| Model list | Yes | 1 hr | New models added occasionally |
| Code completion | No | N/A | Context-dependent, non-deterministic |
| Tool call result | No | N/A | Side effects (file write, etc.) |
| Semantic search | Maybe | 10 min | Cache query + results, not generative |

Cache invalidation:
- **File read:** Watch for file changes via `fs.watch()`, invalidate on change
- **Time-based:** TTL expires, fetch fresh
- **User-triggered:** "Clear cache" button in settings
- **Version-based:** If daemon version changes, clear all

**Prefetching Strategy:**

Load resources before user needs them:

```typescript
// On session start, prefetch common resources
async function prefetchSession(session: Session) {
  // 1. Model list (daemon startup)
  prefetch("/tools/list");
  
  // 2. User's recent tools (from session history)
  session.tools_used.slice(0, 5).forEach(tool => {
    prefetch(`/tools/${tool.name}`);
  });
  
  // 3. Local model (if Tier 1 is preferred)
  if (session.settings.execution_tier === "local") {
    prefetch("/models/load", { model: "local-7b" });
  }
}
```

**Connection Pooling (MCP):**

Maintain persistent connections to MCP server:

```typescript
class MCPConnectionPool {
  private idle: Set<MCPConnection> = new Set();
  private active: Map<string, MCPConnection> = new Map();
  private maxSize: number = 10;
  
  async acquire(): Promise<MCPConnection> {
    if (this.idle.size > 0) {
      return this.idle.pop()!;  // Reuse idle connection
    }
    if (this.active.size < this.maxSize) {
      return new MCPConnection();  // Create new
    }
    // Wait for connection to become available
    return await this.waitForAvailable();
  }
  
  release(conn: MCPConnection): void {
    this.active.delete(conn.id);
    this.idle.add(conn);
  }
}
```

**Latency Budget Enforcement:**

Each operation has a latency budget. If exceeded, fall back to next tier:

```typescript
async function executeWithBudget<T>(
  operation: (tier: ExecutionTier) => Promise<T>,
  budget: LatencyBudget
): Promise<T> {
  const tiers: ExecutionTier[] = ["local", "hybrid", "cloud"];
  
  for (const tier of tiers) {
    const deadline = Date.now() + budget.maxMs[tier];
    try {
      return await withDeadline(operation(tier), deadline);
    } catch (error) {
      if (error.code === "DEADLINE_EXCEEDED") {
        // Tier too slow, try next
        continue;
      }
      // Other error, don't try other tiers
      throw error;
    }
  }
}
```

---

### 2.9 Streaming Response Handler
**Responsibility:** Handle SSE (Server-Sent Events) and chunked transfer encoding  
**Key Interfaces:**
- `handleSSE(response: ReadableStream): AsyncIterator<Chunk>` – parse SSE events
- `chunkResponse(data: string, chunkSize: number): string[]` – split for transport
- `reassembleChunks(chunks: string[]): string` – reconstruct on client

**Key Files:**
```
src/
├── streaming/
│   ├── sse-parser.ts          # Parse SSE event stream
│   ├── chunk-encoder.ts       # Split large responses
│   ├── stream-rewriter.ts     # Format conversion (SSE → chunked)
│   └── stream-terminator.ts   # Graceful stream close
```

**SSE Stream Handling:**

Copilot and Claude Code both use SSE for streaming responses. Proxy must:
1. Accept SSE from daemon
2. Reformat for upstream client if needed
3. Handle stream errors (timeout, close)

```typescript
async function* handleSSE(response: Response): AsyncIterator<string> {
  const reader = response.body?.getReader();
  if (!reader) throw new Error("No response body");
  
  const decoder = new TextDecoder();
  let buffer = "";
  
  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      
      buffer += decoder.decode(value, { stream: true });
      
      // Parse complete SSE events (end with \n\n)
      const lines = buffer.split("\n");
      buffer = lines.pop() || "";  // Incomplete event stays in buffer
      
      for (const line of lines) {
        if (line.startsWith("data: ")) {
          yield line.slice(6);  // Emit data payload
        }
      }
    }
  } finally {
    reader.releaseLock();
  }
}
```

---

### 2.10 Telemetry & Observability
**Responsibility:** Log and monitor all proxy operations  
**Key Interfaces:**
- `logOperation(op: Operation, result: Result): void` – structured logging
- `recordMetric(name: string, value: number, tags: Tags): void` – metrics
- `reportToUACS(event: UACSEvent): Promise<void>` – dashboard reporting

**Key Files:**
```
src/
├── telemetry/
│   ├── operation-logger.ts    # Structured logging
│   ├── metrics-collector.ts   # Prometheus/StatsD metrics
│   ├── uacs-reporter.ts       # UACS dashboard events
│   ├── trace-context.ts       # OpenTelemetry integration
│   └── log-buffer.ts          # In-memory buffer (ring)
```

**Operation Logging Schema:**

```typescript
interface OperationLog {
  timestamp: number;           // Unix ms
  operation_id: string;        // UUID
  operation_type: string;      // "completion", "file_write", "tool_call", etc.
  user_id: string;             // From session
  status: "started" | "completed" | "failed" | "timeout";
  latency_ms: number;
  execution_tier: ExecutionTier;
  
  // Detailed context
  tool_name?: string;          // If tool-based
  file_path?: string;          // If file operation
  risk_level?: RiskLevel;      // If required approval
  approval_given?: boolean;
  
  // Error details
  error_code?: string;         // e.g., "TIMEOUT", "VALIDATION_ERROR"
  error_message?: string;
  
  // Privacy-safe details
  input_hash?: string;         // blake3(input) — no data
  output_size_bytes?: number;  // Just size, not content
  
  // Telemetry
  local_model_latency_ms?: number;
  hybrid_latency_ms?: number;
  cloud_latency_ms?: number;
  cache_hit?: boolean;
}
```

**Metrics to Collect:**

```
# Latency (per tier)
bonsai_proxy_latency_ms{tier="local", operation="completion"}
bonsai_proxy_latency_ms{tier="hybrid", operation="file_write"}
bonsai_proxy_latency_ms{tier="cloud", operation="inference"}

# Success rates
bonsai_proxy_operation_success_rate{tier="local"}
bonsai_proxy_operation_success_rate{tier="cloud"}

# Approval rates
bonsai_proxy_approval_rate{risk_level="low"}
bonsai_proxy_approval_rate{risk_level="high"}
bonsai_proxy_denial_rate{risk_level="high"}

# Tier usage
bonsai_proxy_tier_usage{tier="local", operation="completion"}
bonsai_proxy_tier_usage{tier="cloud", operation="completion"}

# Cache
bonsai_proxy_cache_hit_rate
bonsai_proxy_cache_size_bytes
```

---

### 2.11 Credential Manager
**Responsibility:** Store, retrieve, and rotate credentials securely  
**Key Interfaces:**
- `store(service: string, credential: Credential): void` – save to keychain
- `retrieve(service: string): Credential | null` – load from keychain
- `delete(service: string): void` – clear credential

**Key Files:**
```
src/
├── credentials/
│   ├── keychain-adapter.ts    # OS keychain (macOS Keychain, Windows Credential Manager, Linux Secret Service)
│   ├── credential-types.ts    # GitHub OAuth, Anthropic API key, etc.
│   └── credential-validator.ts # Check validity before use
```

**OS Keychain Integration:**

| OS | Storage | Library |
|---|---|---|
| macOS | Keychain | `keytar` npm package |
| Windows | Credential Manager | `keytar` npm package |
| Linux | Secret Service (GNOME) | `keytar` npm package |
| Linux (headless) | Encrypted file | Custom AES-256-GCM store |

---

### 2.12 Discovery & Service Location
**Responsibility:** Find local daemon via Echo QUIC discovery  
**Key Interfaces:**
- `discoverDaemon(): Promise<DaemonInfo>` – find local daemon
- `validateDaemon(daemon: DaemonInfo): Promise<boolean>` – health check

**Key Files:**
```
src/
├── discovery/
│   ├── echo-discovery.ts      # Echo QUIC multicast beacon
│   ├── daemon-locator.ts      # Try common ports (8080, 8081, ...)
│   ├── fallback-registry.ts   # Remember discovered daemons
│   └── health-checker.ts      # Validate daemon is responsive
```

**Discovery Process:**

1. **Echo QUIC Beacon (preferred):** Listen for mDNS broadcasts from daemon
   - Daemon broadcasts: `_bonsai._tcp.local:8080`
   - Proxy joins multicast group, listens for beacons
   - Fast, automatic, works on local network

2. **Port Scanning (fallback):** Try well-known ports
   - `localhost:8080` (default)
   - `localhost:8081`, `8082`, ... (alternative ports)
   - Each with 500ms timeout

3. **Saved Location (memory):** Remember last discovered daemon
   - Hostname, port, last seen timestamp
   - Try last known location first on startup

4. **User Configuration (override):** `~/.bonsai/proxy.config.json`
   - Allows manual override: `BONSAI_DAEMON_URL="http://192.168.1.100:9000"`

---

### 2.13 VS Code Extension Integration
**Responsibility:** Hook into VS Code extension lifecycle  
**Key Interfaces:**
- `activate(context: ExtensionContext): void` – extension startup
- `deactivate(): void` – extension shutdown
- `registerCommands(): void` – register palette commands
- `setupStatusBar(): void` – status bar indicator

**Key Files:**
```
src/
├── vs-code-integration/
│   ├── extension.ts           # Extension entry point
│   ├── command-palette.ts     # Register commands
│   ├── status-bar.ts          # Status bar widget
│   ├── modal-dialogs.ts       # Approval modals
│   ├── webview-panel.ts       # Settings UI
│   └── event-listeners.ts     # File change watchers
```

**Extension Lifecycle:**

```typescript
export async function activate(context: vscode.ExtensionContext) {
  // 1. Initialize proxy
  const proxy = new BonsaiSovereignProxy();
  
  // 2. Discover daemon (with timeout)
  try {
    await proxy.discoverDaemon({ timeout: 5000 });
  } catch (error) {
    vscode.window.showWarningMessage("Bonsai daemon not found. Operating in cloud-only mode.");
  }
  
  // 3. Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand("bonsai.approval.approve", () => {
      proxy.approval_gate.approve();
    }),
    vscode.commands.registerCommand("bonsai.settings.open", () => {
      showSettingsPanel(context);
    })
  );
  
  // 4. Setup status bar
  setupStatusBar(proxy);
  
  // 5. Install HTTP interceptor (via extension's HTTP client)
  setupHTTPInterceptor(proxy);
  
  // 6. Watch for file changes
  watchFileChanges(proxy.session_manager);
}
```

---

## 3. Request/Response Flow for Each Operation Type

### 3.1 Simple Code Completion (Copilot)

```
USER: Types "function calculate(" in editor
     ↓
COPILOT: Sends POST /v1/completions
     {
       "model": "gpt-4",
       "messages": [{"role": "user", "content": "function calculate("}],
       "temperature": 0.7,
       "max_tokens": 100
     }
     ↓
PROXY HTTP Interception:
  - Intercepts request to api.openai.com
  - Looks up routing rule → matches /v1/completions pattern
  - Extracts OAuth token from Authorization header
     ↓
AUTHENTICATION BRIDGE:
  - Checks if token cached (yes, from earlier GitHub Copilot login)
  - Token is valid (< 24h old)
     ↓
TOOL MARSHALLING:
  - Converts Copilot schema to MCP "code_completion" tool
  - Parameters: prompt, language, context_lines
     ↓
SESSION MANAGER:
  - Retrieves current session
  - Attaches file_path, language, viewport info
     ↓
EXECUTION ROUTER:
  - Decides tier:
    - Measures local model latency (already benchmarked, ~50ms)
    - Privacy budget: high (user prefers local)
    - → Select Tier 1 (Local)
  - Risk classification: Low (read-only completion)
  - No approval needed
     ↓
MCP CLIENT:
  - POST http://localhost:8080/call/code_completion
  - Headers: Authorization: Bearer <capability_token>
  - Streams SSE response
     ↓
DAEMON (MCP Server):
  - Calls local model (7B inference)
  - Returns: {"completion": "  return a + b;", "confidence": 0.92}
     ↓
RESPONSE TRANSLATOR:
  - Converts MCP completion → OpenAI format
  - Returns chunked SSE to Copilot
     ↓
PROXY → COPILOT:
  - HTTP 200 OK
  - Streams completion tokens
     ↓
COPILOT RENDERS: "function calculate( return a + b;"
     ↓
TELEMETRY:
  - Logs: { operation: "completion", tier: "local", latency: 62ms, success: true }
```

### 3.2 Chat with Tool Calling (Claude Code)

```
USER: Asks Claude Code "Create a test file for src/main.rs"
     ↓
CLAUDE CODE: Sends streaming chat request
     POST /v1/messages
     {
       "model": "claude-3.5-sonnet",
       "messages": [...],
       "tools": [
         {"name": "write_file", "description": "..."},
         {"name": "read_file", "description": "..."}
       ]
     }
     ↓
PROXY INTERCEPTION:
  - Intercepts request to api.anthropic.com
  - Matches /v1/messages pattern
  - Extracts Anthropic API key (from keychain)
     ↓
AUTHENTICATION BRIDGE:
  - Checks API key validity
  - Wraps key in capability token (for local daemon)
     ↓
EXECUTION ROUTER:
  - Operation: "chat_with_tools"
  - Risk level: Medium (may write files)
  - Approval needed: Yes (file write)
  - Decision: Ask user first
     ↓
APPROVAL GATE:
  - Classifies risk: "Medium" (tool calling enabled)
  - Shows modal:
    │ "Claude Code wants to call tools. Approve? [Deny] [Approve]"
  - User clicks [Approve]
  - Caches decision
     ↓
TOOL MARSHALLING:
  - Converts Copilot tool definitions to MCP format
  - Maps "write_file" → Bonsai "write_file" tool
     ↓
MCP CLIENT:
  - POST http://localhost:8080/call/chat_with_tools
  - Streams SSE response from daemon
     ↓
STREAMING HANDLER:
  - Parses SSE events from daemon
  - Reformats for Claude Code client
  - Streams back to Claude Code
  - Handles: content chunks, tool_use events, stop_reason
     ↓
CLAUDE CODE RENDERS:
  - Shows chat response
  - User sees tool calls being made
  - Accepts suggestions to apply test file
     ↓
TELEMETRY:
  - Logs: { operation: "chat_with_tools", tier: "local", approved: true, latency: 1850ms }
```

### 3.3 File Write with HITL Approval

```
USER: Claude Code agent wants to write tests/test_main.rs
     ↓
MCP CALL: /call/write_file
  {
    "path": "tests/test_main.rs",
    "content": "fn test_calculate() { assert_eq!(calculate(2,3), 5); }"
  }
     ↓
PROXY RECEIVES (from daemon):
  - Intercepts MCP response about to be sent to Claude Code
  - Detects: file write operation
     ↓
RISK CLASSIFIER:
  - Path: tests/test_main.rs (safe location)
  - Content: test code (safe)
  - Size: 150 bytes (small)
  - Previous approvals: User approved 8/10 similar writes
  - Risk level: LOW (90% confidence)
  - Suggested: Approve automatically
     ↓
DECISION:
  - Auto-approved (low risk + high confidence + user history)
  - File written to disk
  - Logging: { risk: "low", auto_approved: true }
     ↓
BUT IF: High-risk file (e.g., .env, /etc/sudoers, system file)
     ↓
APPROVAL MODAL:
  ┌────────────────────────────────────┐
  │ ⚠️ APPROVAL REQUIRED               │
  │ Write file: .env                   │
  │ Risk: HIGH                         │
  │ Confidence: 95%                    │
  │                                    │
  │ Content preview (first 5 lines):   │
  │ DATABASE_URL=postgres://...        │
  │ API_KEY=sk-...                     │
  │                                    │
  │ This file contains sensitive data! │
  │                                    │
  │ [Deny]  [Approve Once]             │
  └────────────────────────────────────┘
     ↓
USER: Clicks [Approve Once]
     ↓
FILE WRITTEN:
  - Snapshot: .bonsai/rollback/{op_id}.json (for undo)
  - Content written to disk
  - Log: { operation: "write_file", path: ".env", approved: true, risk: "high" }
     ↓
UACS REPORT:
  - POST /api/uacs/events { user_id, operation: "write_file", risk: "high", approved: true }
```

### 3.4 Checkpoint Creation & Restoration (Claude Code)

```
SCENARIO 1: User creates checkpoint
     ↓
CLAUDE CODE: Calls /checkpoint/create
  {
    "name": "Working implementation",
    "snapshot": { /* full state */ }
  }
     ↓
PROXY SESSION MANAGER:
  - Creates checkpoint record in session
  - Large snapshot: Store in CAS (content-addressed storage)
  - Stores CAS key in session DB
  - Checkpoint metadata: ID, name, timestamp, bonsai_cas_key
     ↓
SNAPSHOT STORED:
  - Locally: ~/.bonsai/sessions.db (metadata)
  - CAS: ~/.bonsai/cas/ (large blob)
  - Synced: Echo QUIC to other devices (async)

SCENARIO 2: User restores checkpoint
     ↓
CLAUDE CODE: Calls /checkpoint/restore/{checkpoint_id}
     ↓
PROXY SESSION MANAGER:
  - Looks up checkpoint in session DB
  - Retrieves CAS blob via bonsai_cas_key
  - Reconstructs full state
  - Updates session.context with checkpoint state
     ↓
CLAUDE CODE:
  - Receives restored state
  - UI updates to match checkpoint (file tree, open tabs, etc.)
```

### 3.5 @-Mention Resolution (Copilot)

```
USER: Asks "@workspace how is error handling done?"
     ↓
COPILOT: Needs to resolve @workspace mention
  - Converts to: "Retrieve context about workspace code structure"
     ↓
PROXY EXECUTION ROUTER:
  - Operation: "semantic_search"
  - Decides: Local (search index is local) or Cloud (for large codebase)
  - If Local: Uses local embedding model (with similarity search)
  - If Cloud: Uploads code snippets to Claude API for understanding
     ↓
LOCAL TIER (preferred):
  - Loads embedding model locally (4B params, cached)
  - Embeds query: "error handling"
  - Searches local index (built on startup from code)
  - Retrieves top 3 files: src/error.rs, src/resilience.rs, docs/errors.md
     ↓
RESPONSE:
  - Returns file context to Copilot
  - Copilot includes in chat context
  - User sees "@workspace" resolved to relevant files
     ↓
FALLBACK (if local index unavailable):
  - Falls back to Cloud tier
  - Uploads code (with privacy warning)
  - Uses Claude's understanding to find relevant sections
```

---

## 4. Data Schemas (JSON)

### 4.1 Session Schema

```json
{
  "session_id": "7ea3b8c2-5890-4f1c-9d2e-1a3c5d7e9f1b",
  "user_id": "github:luci",
  "created_at": 1717200000000,
  "last_accessed_at": 1717213452000,
  "auth": {
    "mode": "oauth",
    "oauth_tokens": {
      "github": "ghu_16C7e42F292c6912E7710c838347Ae178B4a",
      "anthropic": "sk-ant-v3-5VTzqPj5..."
    },
    "capability_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "capability_token_expires_at": 1717286452000
  },
  "context": {
    "editor": "both",
    "file_path": "/Users/luci/project/src/main.rs",
    "language": "rust",
    "selected_text": "sha256:a1b2c3d4e5f6...",
    "viewport": {
      "start_line": 42,
      "end_line": 78
    }
  },
  "checkpoints": [
    {
      "checkpoint_id": "ckpt_20240531_001",
      "name": "Working implementation",
      "timestamp": 1717208000000,
      "snapshot": null,
      "bonsai_cas_key": "sha256:f1e2d3c4b5a6..."
    }
  ],
  "settings": {
    "execution_tier": "local",
    "approval_required_for": ["file_write", "execute"],
    "cache_enabled": true,
    "telemetry_enabled": true
  },
  "tools_used": [
    {
      "tool_name": "code_completion",
      "timestamp": 1717213400000,
      "execution_tier": "local",
      "latency_ms": 62,
      "success": true,
      "risk_classification": "low"
    }
  ]
}
```

### 4.2 Tool Definition (MCP Format)

```json
{
  "name": "write_file",
  "description": "Write content to a file (will create if doesn't exist)",
  "input_schema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "File path (relative to workspace root)"
      },
      "content": {
        "type": "string",
        "description": "File content to write"
      },
      "mode": {
        "type": "string",
        "enum": ["create", "append", "overwrite"],
        "description": "Write mode (default: overwrite)"
      }
    },
    "required": ["path", "content"]
  }
}
```

### 4.3 Checkpoint Format

```json
{
  "checkpoint_id": "ckpt_20240531_001",
  "name": "Working implementation",
  "timestamp": 1717208000000,
  "created_by": "claude_code",
  "session_id": "7ea3b8c2-5890-4f1c-9d2e-1a3c5d7e9f1b",
  "bonsai_cas_key": "sha256:f1e2d3c4b5a6...",
  "snapshot_size_bytes": 2345678,
  "snapshot_hash": "sha256:2f3e4d5c6b7a...",
  "compressed": true,
  "metadata": {
    "open_files": ["src/main.rs", "tests/test_main.rs"],
    "cursor_positions": {
      "src/main.rs": { "line": 42, "column": 15 }
    },
    "git_commit": "abc1234def567",
    "working_directory": "/Users/luci/project"
  }
}
```

### 4.4 Approval Request Format

```json
{
  "request_id": "approval_20240531_12345",
  "timestamp": 1717213452000,
  "operation_type": "file_write",
  "operation_description": "Write file 'tests/test_main.rs'",
  "risk_level": "medium",
  "confidence": 82,
  "reasoning": [
    "File write outside current workspace",
    "Content 2.3 KB (large for auto-approval)",
    "User has approved 3/5 similar operations"
  ],
  "execution_tier": "local",
  "suggested_approval": false,
  "can_auto_approve": false,
  "details": {
    "file_path": "tests/test_main.rs",
    "content_preview": "fn test_calculate() { assert_eq!(calculate(2,3), 5); }",
    "file_size_bytes": 150
  },
  "expires_at": 1717213512000,
  "ui_timeout_ms": 30000
}
```

### 4.5 Capability Token Format

```
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.
eyJ1c2VyX2lkIjoiZ2l0aHViOmx1Y2kiLCJzY29wZXMiOlsidG9vbHM6cmVhZCIsInRvb2xzOndyaXRlIl0sImV4cCI6MTcxNzI4NjQ1MiwiYXV0aF9tb2RlIjoib2F1dGgiLCJvYXV0aF9wcm92aWRlciI6ImFudGhyb3BpYyJ9.
SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c

Decoded:
{
  "alg": "HS256",
  "typ": "JWT"
}.
{
  "user_id": "github:luci",
  "scopes": ["tools:read", "tools:write"],
  "exp": 1717286452,
  "auth_mode": "oauth",
  "oauth_provider": "anthropic"
}
```

---

## 5. Error Scenarios & Handling

### 5.1 Local Model Crashes

```
Scenario: User hits code completion, local 7B model segfaults
     ↓
Local Daemon: Process crashes, MCP server becomes unresponsive
     ↓
Proxy HTTP Client: Request to localhost:8080 times out (5s timeout)
     ↓
Timeout Manager:
  - Detects timeout
  - Triggers circuit breaker (mark tier 1 as unavailable)
  - Attempts one retry (exponential backoff 100ms)
  - Still fails
     ↓
Fallback to Tier 2 (Hybrid):
  - Hybrid uses cloud inference
  - Proxy calls api.anthropic.com instead
  - Higher latency (2-3s) but works
     ↓
Recovery:
  - Proxy logs: { event: "tier1_crash", retry_count: 1, fallback_to: "tier2" }
  - Shows user: "Local model unavailable, using cloud. [Restart local model]"
  - User clicks "Restart local model" → kills process, starts fresh
     ↓
Daemon Restart:
  - User runs `bonsai daemon start` or proxy auto-restarts (configurable)
  - Proxy detects new daemon via Echo discovery
  - Marks Tier 1 available again
```

### 5.2 Network Down (Offline Mode)

```
Scenario: User requests code completion, network connection drops
     ↓
Proxy tries Tier 1 (Local): ✓ Works (no network needed)
     ↓
Proxy tries Tier 2 (Hybrid): ✗ Network required for cloud component
     ↓
Proxy tries Tier 3 (Cloud): ✗ Network down
     ↓
Execution Router:
  - All tiers failed or unavailable
  - Offline mode active
  - Uses Tier 1 (local) exclusively
     ↓
Offline Identity:
  - No OAuth token needed
  - Proxy uses offline Ed25519 keypair (stored locally)
  - Signs request with private key
  - Daemon validates signature against public key
     ↓
Local Execution:
  - All operations run locally
  - No cloud fallback
  - User sees: "Offline mode active [Retry network]"
     ↓
When Network Returns:
  - Proxy detects network (via periodic ping)
  - Resumes normal tier selection
  - Syncs any local changes to cloud (if enabled)
```

### 5.3 User Denies Approval

```
Scenario: Proxy shows HITL modal for high-risk file write
     ↓
User: Clicks [Deny]
     ↓
Approval Gate:
  - Operation rejected
  - Logs: { operation: "file_write", approved: false, risk: "high" }
     ↓
Proxy Response:
  - Returns error to Claude Code: "Operation denied by user"
  - Claude Code displays: "File write rejected. Details: [View log]"
     ↓
Claude Code:
  - Can retry with different operation (e.g., create with different name)
  - Or ask user for confirmation first
     ↓
UACS Reporting:
  - Reports denial to dashboard
  - Admin can review trends (e.g., "Why are users denying file writes?")
```

### 5.4 Tool Returns Unexpected Format

```
Scenario: MCP tool returns response not matching declared schema
     ↓
Example: Tool declares output is "string", returns object {"data": "..."}
     ↓
Response Translator:
  - Tries to coerce response to declared type
  - String coercion: JSON.stringify({"data": "..."}) ✓ Works
  - Logs warning: { event: "format_mismatch", tool: "xyz", severity: "warning" }
     ↓
Upstream:
  - Sends response to client (transformed)
  - Client may still display oddly (depends on what it expects)
     ↓
Recovery:
  - Proxy logs schema mismatch
  - User can report via [Send feedback]
  - Helps identify tool definition bugs
```

### 5.5 Session Corrupted

```
Scenario: Session database corrupted or missing
     ↓
Session Manager: Tries to load session from ~/.bonsai/sessions.db
  - SQLite read fails (database locked, corrupted)
  - Error: "database disk image is malformed"
     ↓
Session Recovery:
  - Backup exists: ~/.bonsai/sessions.db.backup
  - Restore from backup (may lose last 5 min of changes)
  - Log: { event: "session_recovery", backup_used: true, data_loss: "5m" }
     ↓
If no backup:
  - Create new empty session
  - User loses session history, checkpoints
  - Offer: "Restore from cloud?" (if synced)
     ↓
User Notification:
  - "Session was corrupted and recovered. Some data may be lost. [View details]"
```

---

## 6. Security Considerations

### 6.1 Credential Storage

| Credential | Where | How | Protection |
|---|---|---|---|
| OAuth (GitHub, Anthropic) | OS Keychain | OS API | OS-managed encryption |
| Offline Ed25519 key | `~/.bonsai/identity/offline.key` | File | AES-256-GCM (password-derived) |
| Capability token | In-memory | Variable | Stored only for active session |
| Session DB | `~/.bonsai/sessions.db` | SQLite | AES-256-GCM whole-file |

**OAuth Token Rotation:**
- Tokens expire after 1 year (GitHub) or 1 hour (Anthropic)
- Before expiry, proxy auto-refreshes using refresh token
- Refresh tokens stored in OS Keychain (must be manually revoked)

**Offline Key Generation:**
- On first offline use, proxy generates Ed25519 keypair
- Derives encryption key from user's machine identity (hardware-based if possible)
- Asks user to set a passphrase (stored hashed)
- Private key encrypted with passphrase + salt + PBKDF2

### 6.2 Preventing Unauthorized Tool Calls

**Capability Token Validation:**
- Every MCP request includes capability token (Bearer header)
- Daemon validates token signature (HMAC-SHA256)
- Token lists allowed scopes (e.g., "tools:read", "tools:write:file")
- Tool calls checked against scopes (e.g., "write_file" requires "tools:write:file")

**Tool Schema Validation:**
- Before calling tool, proxy validates parameters against MCP schema
- Type checking, required fields, enum validation
- Rejects calls with invalid parameters (prevents injection attacks)

**User Approval Gate:**
- High-risk tools (execute, file write, network) require HITL approval
- Approval can be cached, but expires after 24h
- Users can revoke blanket approvals via settings

### 6.3 Code Privacy (Not Leaking to Cloud)

**Privacy Budget:**
- User sets privacy level: "high" (local only), "medium" (local + hybrid), "low" (any tier)
- Proxy enforces: if user selects "high", never uploads code to cloud

**Operation Classification:**
- Read-only → can go to any tier (just retrieving information)
- Code generation → requires "medium" or "low" privacy
- Code execution → sensitive, requires approval + user's tier choice

**Minimal Logging:**
- Logs contain operation metadata (name, latency, success) but NOT code content
- Code hashed (blake3) for deduplication, never stored plaintext
- User can disable telemetry completely (no cloud reporting)

### 6.4 Malicious Tool Responses

**Sandbox Execution:**
- MCP tools run inside sandbox (subprocess, timeout, resource limits)
- Tool output captured and validated before returning to client
- Large responses (> 10 MB) truncated and logged

**Output Sanitization:**
- If tool returns HTML/JavaScript, escape before rendering in UI
- Prevent code injection in VS Code panels

**Tool Signature Verification:**
- (Future) Tools signed with developer key
- Proxy verifies signature before executing
- Prevents tampering with tool binaries

### 6.5 Man-in-the-Middle Protection

**HTTPS Everywhere:**
- All requests to cloud APIs use HTTPS (TLS 1.3+)
- Certificate pinning for critical endpoints (api.anthropic.com)

**Local Communication (Daemon):**
- Proxy to daemon uses HTTP over localhost
- Daemon validates capability token signature (HMAC)
- (Future) Use TLS for inter-process communication

**Device Pairing:**
- For multi-device sync, devices paired via QR code (one-time)
- Pairing generates shared secret (Ed25519 ECDH)
- All sync messages authenticated with shared secret

---

## 7. Extensibility Points

### 7.1 Adding New Tools

**Tool Provider Interface:**
```typescript
interface ToolProvider {
  listTools(): Promise<Tool[]>;
  callTool(name: string, params: object): Promise<ToolResult>;
  validateInput(toolName: string, params: object): ValidationResult;
}
```

**Steps to add tool:**
1. Create tool provider (implements interface)
2. Register in tool registry: `registry.register(provider)`
3. Define tool schema (for parameter validation)
4. Proxy auto-discovers via registry

**Examples:**
- Custom shell command tool (wrap `subprocess.run()`)
- Database query tool (SQL executor)
- API client tool (curl wrapper)

### 7.2 Adding New Models

**Model Registry:**
```typescript
interface Model {
  name: string;
  size_params: number;
  quantization: "fp16" | "q8_0" | "q4_k_m";
  download_url: string;
  inference_engine: "llama.cpp" | "vllm" | "candle";
}
```

**Steps to add model:**
1. Define model metadata in registry
2. Download GGUF file
3. Proxy auto-loads model on demand
4. Cached in memory (LRU eviction if needed)

### 7.3 Adding New Approval Policies

**Custom Classifier:**
```typescript
interface RiskClassifier {
  classify(operation: Operation): RiskScore;
}
```

**Steps:**
1. Implement `RiskClassifier` interface
2. Register with approval gate: `gate.registerClassifier(classifier)`
3. Proxy uses custom logic for risk classification

**Examples:**
- Organization-specific rules ("never approve network calls")
- Department-specific rules ("data team can write to data/ directory")
- Time-based rules ("require approval outside business hours")

### 7.4 Adding New Data Stores

**Session Store Interface:**
```typescript
interface SessionStore {
  getSession(id: string): Promise<Session | null>;
  saveSession(session: Session): Promise<void>;
  deleteSession(id: string): Promise<void>;
  listSessions(): Promise<Session[]>;
}
```

**Default:** SQLite (`~/.bonsai/sessions.db`)

**Alternatives:**
- PostgreSQL (multi-device shared state)
- Redis (distributed caching + session store)
- DuckDB (local analytics on session logs)

### 7.5 Adding Custom Extensions

**Extension Hook Interface:**
```typescript
interface ProxyExtension {
  name: string;
  version: string;
  hooks: {
    onRequestBefore?: (req: Request) => Request;
    onRequestAfter?: (req: Request, resp: Response) => Response;
    onApprovalRequest?: (req: ApprovalRequest) => ApprovalRequest;
  };
}
```

**Example use cases:**
- Log all operations to external SIEM
- Add custom headers to requests (for metrics)
- Modify approval modals (add custom buttons)

**Loading:**
- Extensions: `~/.bonsai/extensions/*.ts` or npm packages
- Proxy auto-discovers and loads on startup
- Run in isolated worker thread (safe)

---

## 8. Testing Strategy

### 8.1 Unit Tests

**What to test:**
- HTTP interception engine (pattern matching)
- Tool marshalling (schema conversion, parameter translation)
- Risk classifier (correct risk levels for operations)
- Timeout manager (enforces deadlines)
- Cache (hit/miss, TTL)

**Mocking:**
- Mock HTTP clients (intercept requests)
- Mock MCP daemon (return canned responses)
- Mock keychain (in-memory store for tests)
- Mock OS APIs (system time, process signals)

**Example:**
```typescript
describe("ToolMarshaller", () => {
  it("converts Copilot tool to MCP tool", () => {
    const copilotTool = {
      id: "completion",
      name: "completion",
      parameters: { properties: { prompt: { type: "string" } } }
    };
    const mcpTool = marshaller.toMCP(copilotTool);
    expect(mcpTool.name).toBe("code_completion");
  });
  
  it("validates parameters against schema", () => {
    const result = marshaller.validateParams("write_file", { path: "foo.rs" });
    expect(result.valid).toBe(false);  // Missing 'content'
  });
});
```

### 8.2 Integration Tests

**What to test:**
- Copilot → Proxy → Daemon → Response pipeline
- Authentication flow (OAuth exchange)
- Tool execution with real daemon
- Session persistence and recovery
- Multi-device sync (with mock network)

**Test setup:**
- Start real daemon (or mock server)
- Run proxy with test configuration
- Make actual HTTP requests from test client

**Example:**
```typescript
describe("Copilot Integration", () => {
  let daemon: DaemonTestServer;
  let proxy: BonsaiProxy;
  
  beforeAll(async () => {
    daemon = new DaemonTestServer(3001);
    proxy = new BonsaiProxy({ daemon_url: "http://localhost:3001" });
    await proxy.start();
  });
  
  it("completes code from Copilot", async () => {
    const resp = await axios.post(
      "http://localhost:3000/v1/completions",
      { prompt: "function foo(" },
      { headers: { Authorization: "Bearer test-token" } }
    );
    expect(resp.status).toBe(200);
    expect(resp.data.completion).toContain(")");
  });
});
```

### 8.3 End-to-End Tests

**What to test:**
- Full user workflow (edit → Copilot → completion → approved)
- Network failure → offline mode → recovery
- Session loss → recovery from backup
- Multi-device sync (simulate 2+ devices)

**Test environment:**
- Real VS Code extension running
- Real daemon
- Real network (or simulated with network simulator)

**Example scenario:**
```
1. User opens VS Code with Bonsai extension
2. Proxy discovers daemon (or uses fallback)
3. User types code, Copilot offers completion
4. Proxy intercepts, routes to local tier
5. Completion shown in editor
6. User accepts, file modified
7. Verify file written to disk
8. Verify telemetry logged
```

### 8.4 Testing Offline Scenarios

**Simulate network down:**
```typescript
it("operates offline with Ed25519 identity", async () => {
  network.disconnect();
  
  const operation = await proxy.execute({
    type: "code_completion",
    prompt: "function foo("
  });
  
  expect(operation.execution_tier).toBe("local");
  expect(operation.auth_mode).toBe("offline");
  expect(operation.success).toBe(true);
});
```

**Simulate daemon down:**
```typescript
it("falls back to cloud when local daemon offline", async () => {
  daemon.stop();
  
  const operation = await proxy.execute({
    type: "code_completion",
    prompt: "function foo("
  });
  
  expect(operation.execution_tier).toBe("cloud");
  expect(operation.latency_ms).toBeGreaterThan(1000);
});
```

### 8.5 Testing Multi-Device Sync

**Simulate 2 devices:**
```typescript
it("syncs session across devices", async () => {
  const device1 = new BonsaiProxy({ device_id: "laptop" });
  const device2 = new BonsaiProxy({ device_id: "phone" });
  
  // Device 1 creates checkpoint
  const ckpt = await device1.createCheckpoint("Test");
  
  // Device 2 should see checkpoint (via Echo sync)
  await delay(100);  // Allow sync
  const ckpt2 = await device2.getCheckpoint(ckpt.id);
  
  expect(ckpt2.name).toBe("Test");
});
```

---

## 9. Deployment & Distribution

### 9.1 Distribution Format

**Option A: VS Code Extension (Recommended)**
- Published on VS Code Marketplace
- Auto-installed alongside Copilot/Claude Code
- Updates via marketplace (1-2x per month)
- Can hook into native APIs (keychain, etc.)

**Option B: Standalone Node.js Service**
- Runs as daemon (like Bonsai daemon itself)
- Controlled via `bonsai proxy start/stop`
- Useful for server/headless environments
- Less convenient for end users

**Option C: Hybrid**
- Proxy bundled as optional VS Code extension
- Also available as standalone service (for non-VS Code users)
- Shared core logic, different entry points

### 9.2 Auto-Discovery of Local Services

**Echo QUIC Discovery (Preferred):**
- Daemon broadcasts mDNS beacon: `_bonsai._tcp.local:8080`
- Proxy joins multicast group, listens for broadcasts
- Zero configuration, works on local networks

**Fallback: Port Scanning**
```typescript
async function discoverDaemon(): Promise<DaemonInfo> {
  // Try Echo discovery first
  const echoResult = await tryEchoDiscovery({ timeout: 1000 });
  if (echoResult) return echoResult;
  
  // Fallback: scan well-known ports
  const ports = [8080, 8081, 8082, 8000, 9000];
  for (const port of ports) {
    const result = await tryPort(`localhost:${port}`, { timeout: 500 });
    if (result) return result;
  }
  
  // Last resort: user configuration
  const configUrl = process.env.BONSAI_DAEMON_URL;
  if (configUrl) return parseUrl(configUrl);
  
  throw new Error("Daemon not found. Please start Bonsai daemon.");
}
```

### 9.3 Independent Updates

**Proxy can update separately from extensions:**
- Proxy version: `~/.bonsai/proxy/version.txt`
- Check for updates daily (HTTP GET to release server)
- Download new version to staging directory
- Switch on next startup (atomic)
- Rollback if startup fails (use previous version)

**Version compatibility:**
- Proxy is backwards-compatible with daemon (graceful degradation)
- If proxy newer than daemon: use subset of features
- If proxy older than daemon: daemon detects, suggests upgrade

### 9.4 Working With/Without Bonsai CLI

**If Bonsai CLI installed:**
- Proxy auto-discovers daemon (via Echo or port scan)
- Uses daemon's MCP server
- All local features available

**If Bonsai CLI NOT installed:**
- Proxy still works (cloud-only mode)
- Routes all requests to cloud (Anthropic, OpenAI)
- Shows warning: "Bonsai not installed. [Download]"
- User can install Bonsai later, proxy auto-enables local features

**Graceful Degradation:**
```typescript
async function initializeProxy() {
  try {
    // Try to find local daemon
    await discoverDaemon({ timeout: 5000 });
    settings.local_mode = true;
    log.info("Local Bonsai daemon available. Privacy mode: ON");
  } catch (error) {
    // No daemon found, use cloud
    settings.local_mode = false;
    log.warn("Bonsai daemon not found. Using cloud-only mode (less private).");
  }
}
```

---

## 10. Success Metrics

### 10.1 Adoption Metrics

**How to measure:**
- Count of active proxy installations (via opt-in telemetry)
- Daily active users (DAU)
- Retention (% still using after 1 month, 3 months, 1 year)
- NPS score (Net Promoter Score survey)

**Goals (Month 1):**
- 1,000+ installations
- 500+ DAU
- 80%+ retention (1 month)

**Goals (Month 6):**
- 10,000+ installations
- 5,000+ DAU
- 70%+ retention (3 months)

### 10.2 Local vs. Cloud Usage

**How to measure:**
- Count of operations by execution tier
- Latency comparison: local vs cloud
- Cost savings: operations not sent to cloud API

**Metrics:**
```
bonsai_proxy_tier_usage_total{tier="local"} = 45,000
bonsai_proxy_tier_usage_total{tier="hybrid"} = 8,000
bonsai_proxy_tier_usage_total{tier="cloud"} = 2,000

Ratio: 90% local, 8% hybrid, 2% cloud (good privacy)
```

**Cost Savings:**
- Per operation to cloud: ~$0.01-0.10 (API cost)
- 45,000 local operations = ~$0 cost
- vs. all cloud = ~$4,500-45,000 cost saved

### 10.3 Latency Improvements

**How to measure:**
- P50, P95, P99 latency for each operation type
- Compare local vs cloud latency

**Metrics:**
```
Code completion (local):  P50=62ms,  P95=150ms,  P99=300ms
Code completion (cloud):  P50=1200ms, P95=2100ms, P99=5000ms

Speedup: ~20x faster locally
```

### 10.4 Security Metrics (Approval Rates)

**How to measure:**
- Count of approval requests
- Approval rate (% approved vs denied)
- Approval rate by risk level

**Metrics:**
```
Approvals by risk level:
  Low:      8,000 auto-approved (0% manual review)
  Medium:   1,200 submitted, 1,100 approved (92%)
  High:     300 submitted, 280 approved (93%)

Denial rate: 7% (users said "no")
Suggests: Risk classification is reasonable, users trust proxy
```

**Confidence calibration:**
- If confidence = 95% but approval rate = 50%: classifier too confident
- If confidence = 50% but approval rate = 99%: classifier too unconfident
- Adjust classifier weights based on actual approvals

### 10.5 Tool Success Rates

**How to measure:**
- Count of tool calls by tool name
- Success rate for each tool
- Error rates and types

**Metrics:**
```
Tool success rates:
  write_file:     98% success
  read_file:      99% success
  code_completion: 97% success
  execute_command: 85% success (failures mostly timeout)
  
Overall: 95% success rate (good reliability)
```

**Error breakdown:**
```
Errors by cause:
  Timeout:      30%
  Invalid params: 20%
  User denial:  15%
  Tool crash:   10%
  Other:        25%
```

**Actions:**
- High timeout rate → increase timeout limit or optimize
- High invalid params → improve parameter validation or tool schema
- High user denial → re-examine approval modal messaging

---

## Appendix A: Architecture Patterns

### A.1 Request Pipeline (Middleware Stack)

```
Request from Copilot/Claude Code
  ↓
[1] HTTP Interception (match pattern)
  ↓
[2] Authentication Bridge (extract token, validate)
  ↓
[3] Request Translator (Copilot → MCP format)
  ↓
[4] Session Manager (attach context)
  ↓
[5] Risk Classifier (determine if HITL needed)
  ↓
[6] Approval Gate (show modal if needed)
  ↓
[7] Tool Marshaller (parameter validation)
  ↓
[8] Execution Router (decide tier)
  ↓
[9] MCP Client (call daemon or cloud)
  ↓
[10] Response Translator (MCP → Copilot format)
  ↓
[11] Streaming Handler (SSE reformat)
  ↓
[12] Cache (store if cacheable)
  ↓
[13] Telemetry (log operation)
  ↓
Response to Copilot/Claude Code
```

### A.2 Error Handling Flow

```
Error occurs in tier N
  ↓
Is error transient? (network timeout, service temporarily unavailable)
  ├─ Yes: Retry with exponential backoff
  │        ├─ Success: Return result
  │        └─ Still fails: Proceed below
  └─ No (auth error, invalid request): Return error immediately
  ↓
Can we fall back to tier N+1?
  ├─ Yes: Try next tier
  │        ├─ Success: Return result
  │        └─ Fails: Continue to next tier (or give up)
  └─ No: Return error
  ↓
Final error (all tiers failed)
  ├─ Generate user-friendly error message
  ├─ Log to telemetry
  └─ Return to client
```

### A.3 State Consistency Model

**Consistency guarantees:**

1. **Strong consistency (sessions):** All devices see same session state within 100ms
   - Uses distributed locking + vector clocks
   - Conflict resolution: last-write-wins + metadata

2. **Eventual consistency (large snapshots):** CAS blobs eventually replicated
   - Asynchronous replication via QUIC
   - Safe because immutable (content-addressed)

3. **Causal consistency (tool calls):** Causally dependent operations preserve order
   - Session 1: Create checkpoint A
   - Session 2: Restore checkpoint A (must see A before restore)
   - Enforced via version vectors on each session update

---

## Conclusion

The **Bonsai Sovereign Proxy Layer** unifies Copilot and Claude Code with the Bonsai Ecosystem, enabling:

- **Privacy:** Code stays local by default, with user control
- **Performance:** 20x faster completions via local inference
- **Reliability:** Graceful fallback from local → hybrid → cloud
- **Control:** Human-in-the-loop approvals for sensitive operations
- **Transparency:** Detailed logging and observability

This design provides a clear, extensible blueprint for 500+ lines of production-ready proxy code that handles the complexity of bridging multiple extensions, models, and tiers while maintaining strong security and UX guarantees.

---

**Document Version:** 1.0  
**Last Updated:** June 1, 2026  
**Status:** Design Complete — Ready for Implementation Phase
