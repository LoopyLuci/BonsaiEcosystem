# Bonsai API Reference & Feature Implementation Guide

**Version**: 2.0  
**Date**: 2026-06-04  
**Scope**: Complete API documentation for developers

---

## Table of Contents

1. [Tauri Commands Reference](#tauri-commands-reference)
2. [Service APIs](#service-apis)
3. [Data Types & Structures](#data-types--structures)
4. [Tool Integration Guide](#tool-integration-guide)
5. [Svelte Stores API](#svelte-stores-api)
6. [Model Training API](#model-training-api)
7. [Knowledge Database API](#knowledge-database-api)
8. [TransferDaemon API](#transferdaemon-api)
9. [BonsaiBot API](#bonsaibot-api)
10. [Error Handling](#error-handling)
11. [Code Examples](#code-examples)

---

## Tauri Commands Reference

### Editor Commands

#### `editor_open_file`
**Purpose**: Load a file for editing

**Parameters**:
```typescript
{
  path: string,                    // Absolute file path
  encoding?: string,               // Default: "utf-8"
}
```

**Returns**:
```typescript
{
  content: string,                 // File contents
  language: string,                // Detected language (js, py, rs, etc.)
  size: number,                    // Bytes
  modified: boolean,               // Has unsaved changes
  encoding: string,                // File encoding
}
```

**Example**:
```javascript
const file = await invoke('editor_open_file', {
  path: 'C:\\Users\\user\\project\\main.py'
});
console.log(file.language);  // "python"
console.log(file.content);   // "def hello():\n  print(...)"
```

---

#### `editor_write_file`
**Purpose**: Save file to disk

**Parameters**:
```typescript
{
  path: string,
  content: string,
  create_parents?: boolean,       // Default: true
}
```

**Returns**:
```typescript
{
  success: boolean,
  path: string,
  bytes_written: number,
}
```

**Example**:
```javascript
await invoke('editor_write_file', {
  path: 'C:\\Users\\user\\project\\test.txt',
  content: 'Hello, world!',
  create_parents: true,
});
```

---

#### `editor_format_code`
**Purpose**: Format file using language-specific formatter

**Parameters**:
```typescript
{
  path: string,
  language: string,                // "rust", "python", "javascript", etc.
  formatter?: string,              // Default: best formatter for language
}
```

**Returns**:
```typescript
{
  formatted: string,               // Formatted code
  changed: boolean,                // Whether formatting changed code
  errors: string[],                // Formatter errors (if any)
}
```

**Example**:
```javascript
const result = await invoke('editor_format_code', {
  path: 'main.rs',
  language: 'rust',  // Uses: rustfmt
});
```

---

#### `editor_lint_file`
**Purpose**: Check file for linting issues

**Parameters**:
```typescript
{
  path: string,
  language: string,
}
```

**Returns**:
```typescript
{
  issues: Array<{
    line: number,
    column: number,
    severity: "error" | "warning" | "info",
    message: string,
    code: string,                  // e.g., "E0308" for Rust
  }>,
}
```

**Example**:
```javascript
const result = await invoke('editor_lint_file', {
  path: 'main.py',
  language: 'python',  // Uses: pylint
});

result.issues.forEach(issue => {
  console.log(`Line ${issue.line}: ${issue.message}`);
});
```

---

### Assistant Commands

#### `assistant_send_message`
**Purpose**: Send chat message to BonsAI

**Parameters**:
```typescript
{
  message: string,
  session_id?: string,             // Defaults to current session
  tools_enabled?: boolean,         // Default: true
  system_prompt?: string,          // Override default system prompt
}
```

**Returns**:
```typescript
{
  id: string,
  role: "assistant",
  content: string,
  tool_calls: Array<{
    id: string,
    name: string,                  // e.g., "shell", "read_file"
    arguments: Record<string, string>,
  }>,
  tokens_used: number,
  latency_ms: number,
}
```

**Example**:
```javascript
const response = await invoke('assistant_send_message', {
  message: 'List files in current directory',
  tools_enabled: true,
});

console.log(response.content);
response.tool_calls.forEach(call => {
  console.log(`Tool: ${call.name}, Args:`, call.arguments);
});
```

---

#### `assistant_call_tool`
**Purpose**: Execute a single tool

**Parameters**:
```typescript
{
  tool_name: string,               // "read_file", "write_file", "shell", etc.
  arguments: Record<string, string>,
  require_approval?: boolean,      // Default: based on TrustGuard settings
  timeout_seconds?: number,        // Default: 30
}
```

**Returns**:
```typescript
{
  success: boolean,
  output: string,                  // stdout
  error?: string,                  // stderr (if failed)
  execution_time_ms: number,
}
```

**Example**:
```javascript
// Read file
const result = await invoke('assistant_call_tool', {
  tool_name: 'read_file',
  arguments: { path: 'main.py' },
});
console.log(result.output);  // File contents

// Execute shell command
const shell_result = await invoke('assistant_call_tool', {
  tool_name: 'shell',
  arguments: { command: 'npm test' },
  require_approval: true,          // Ask user first
});
```

---

#### `assistant_get_chat_history`
**Purpose**: Retrieve chat history

**Parameters**:
```typescript
{
  session_id?: string,
  limit?: number,                  // Default: 50
  offset?: number,                 // Default: 0
}
```

**Returns**:
```typescript
{
  messages: Array<{
    id: string,
    role: "user" | "assistant",
    content: string,
    tool_calls?: Array<{...}>,
    timestamp: string,             // ISO-8601
  }>,
  total_count: number,
}
```

**Example**:
```javascript
const history = await invoke('assistant_get_chat_history', {
  limit: 100,
});

console.log(`Total messages: ${history.total_count}`);
history.messages.forEach(msg => {
  console.log(`${msg.role}: ${msg.content}`);
});
```

---

### Model Commands

#### `model_list_available`
**Purpose**: List available models

**Parameters**:
```typescript
{
  include_remote?: boolean,        // Default: false (only local)
}
```

**Returns**:
```typescript
{
  models: Array<{
    id: string,
    name: string,
    size_bytes: number,
    downloaded: boolean,
    format: "gguf" | "safetensors",
    parameters: string,            // "7B", "13B", etc.
    quantization: string,          // "Q4_K_M", "Q8", etc.
    location: string,              // Path on disk
  }>,
}
```

**Example**:
```javascript
const list = await invoke('model_list_available');
const downloaded = list.models.filter(m => m.downloaded);
console.log(`You have ${downloaded.length} models downloaded`);
```

---

#### `model_load`
**Purpose**: Load a model into memory

**Parameters**:
```typescript
{
  model_id: string,
  device?: "cpu" | "gpu" | "auto",  // Default: "auto"
  context_size?: number,             // Default: 2048
}
```

**Returns**:
```typescript
{
  success: boolean,
  model_id: string,
  loaded_at: string,                 // ISO-8601
  memory_used_mb: number,
  inference_speed_tokens_per_second: number,
}
```

**Example**:
```javascript
const result = await invoke('model_load', {
  model_id: 'mistral-7b',
  device: 'gpu',
});

console.log(`Model loaded! Speed: ${result.inference_speed_tokens_per_second} tokens/sec`);
```

---

#### `model_unload`
**Purpose**: Unload model from memory

**Parameters**:
```typescript
{
  model_id?: string,               // If empty, unload all
}
```

**Returns**:
```typescript
{
  success: boolean,
  freed_memory_mb: number,
}
```

---

### Training Commands

#### `training_start_dpo`
**Purpose**: Start DPO (Direct Preference Optimization) training

**Parameters**:
```typescript
{
  dataset_path: string,            // JSONL file with preference pairs
  base_model_id: string,
  output_path?: string,            // Where to save trained model
  config?: {
    learning_rate?: number,        // Default: 1e-5
    batch_size?: number,           // Default: 8
    num_epochs?: number,           // Default: 3
    warmup_steps?: number,         // Default: 100
    max_length?: number,           // Default: 2048
  },
}
```

**Returns** (streaming):
```typescript
{
  epoch: number,
  step: number,
  loss: number,
  learning_rate: number,
  tokens_processed: number,
  estimated_time_remaining_seconds: number,
}
```

**Example**:
```javascript
// Listen for training progress
const unsubscribe = await listen('training:progress', (event) => {
  console.log(`Epoch ${event.payload.epoch}, Loss: ${event.payload.loss}`);
});

// Start training
await invoke('training_start_dpo', {
  dataset_path: 'C:\\data\\preferences.jsonl',
  base_model_id: 'mistral-7b',
  config: {
    learning_rate: 1e-5,
    num_epochs: 5,
  },
});

// Later: unsubscribe from updates
unsubscribe();
```

---

#### `training_evaluate`
**Purpose**: Evaluate model on test set

**Parameters**:
```typescript
{
  model_path: string,
  test_dataset_path: string,
}
```

**Returns**:
```typescript
{
  accuracy: number,                // 0.0 to 1.0
  loss: number,
  perplexity: number,
  metrics: Record<string, number>,
}
```

---

### Knowledge Database Commands

#### `kdb_create_module`
**Purpose**: Create a new knowledge module

**Parameters**:
```typescript
{
  name: string,                    // e.g., "Python Best Practices"
  description?: string,
  source_files?: string[],         // Paths to import
  source_text?: string,            // Or raw text
}
```

**Returns**:
```typescript
{
  module_id: string,
  name: string,
  created_at: string,
  indexed: boolean,
  passages_count: number,
}
```

---

#### `kdb_search`
**Purpose**: Search knowledge modules

**Parameters**:
```typescript
{
  query: string,
  module_ids?: string[],           // Search specific modules only
  top_k?: number,                  // Default: 5
  min_relevance?: number,          // 0.0 to 1.0, Default: 0.3
}
```

**Returns**:
```typescript
{
  results: Array<{
    passage_id: string,
    module_id: string,
    text: string,
    relevance_score: number,
    source: string,                // File name or URL
  }>,
  total_found: number,
}
```

**Example**:
```javascript
const results = await invoke('kdb_search', {
  query: 'How do I handle async errors?',
  top_k: 3,
});

console.log(`Found ${results.total_found} results:`);
results.results.forEach(r => {
  console.log(`${r.relevance_score.toFixed(2)}: ${r.text.substring(0, 100)}...`);
});
```

---

### Collaboration Commands

#### `collaboration_share_workspace`
**Purpose**: Generate shareable link for collaboration

**Parameters**:
```typescript
{
  permissions?: {
    view?: boolean,
    edit?: boolean,
    delete?: boolean,
    execute?: boolean,             // Shell execution
  },
  expiry_minutes?: number,         // Default: 60
}
```

**Returns**:
```typescript
{
  share_code: string,              // Unique code
  share_url: string,               // Full shareable URL
  qr_code?: string,                // Base64 QR code image
  expiry_at: string,               // ISO-8601
}
```

---

#### `collaboration_list_peers`
**Purpose**: List connected collaborators

**Returns**:
```typescript
{
  peers: Array<{
    peer_id: string,
    name: string,
    status: "online" | "offline",
    last_seen: string,
    permissions: string[],
  }>,
}
```

---

### Bot Commands

#### `bot_start`
**Purpose**: Start BonsaiBot server

**Parameters**:
```typescript
{
  port?: number,                   // Default: auto-detect
  token?: string,                  // Auth token
}
```

**Returns**:
```typescript
{
  running: boolean,
  port: number,
  url: string,
}
```

---

#### `bot_configure_platform`
**Purpose**: Configure a messaging platform

**Parameters**:
```typescript
{
  platform: "discord" | "telegram" | "email" | "matrix",
  config: {
    // Platform-specific config
    // Discord: { token, guild_id }
    // Telegram: { token, bot_username }
    // Email: { smtp_server, smtp_port, username, password }
    // Matrix: { homeserver_url, user_id, password }
  },
}
```

**Returns**:
```typescript
{
  configured: boolean,
  platform: string,
  verified: boolean,
}
```

---

## Service APIs

### LLM Service

**Location**: `src-tauri/src/services/llm.rs`

```rust
pub struct LlmService {
    pub async fn call(
        &self,
        prompt: &str,
        options: CallOptions,
    ) -> Result<LlmResponse> {}
    
    pub async fn stream(
        &self,
        prompt: &str,
    ) -> Result<impl Stream<Item = String>> {}
    
    pub fn is_loaded(&self) -> bool {}
    
    pub async fn load_model(&mut self, model_id: &str) -> Result<()> {}
}
```

---

### Tool Service

**Location**: `src-tauri/src/services/tools.rs`

```rust
pub struct ToolService {
    pub async fn execute(
        &self,
        tool_name: &str,
        arguments: &[&str],
    ) -> Result<String> {}
    
    pub fn get_tool(&self, name: &str) -> Option<Tool> {}
    
    pub fn list_tools(&self) -> Vec<Tool> {}
}
```

---

### Training Service

**Location**: `src-tauri/src/services/training.rs`

```rust
pub struct TrainingService {
    pub async fn start_dpo(
        &mut self,
        config: DpoConfig,
    ) -> Result<impl Stream<Item = TrainingProgress>> {}
    
    pub async fn evaluate(
        &self,
        model_path: &str,
        test_data: &[Example],
    ) -> Result<Metrics> {}
}
```

---

## Data Types & Structures

### Message

```rust
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,  // "user" | "assistant"
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,       // "read_file", "shell", etc.
    pub arguments: Map<String, String>,
    pub result: Option<String>,
    pub status: ToolStatus, // "pending" | "running" | "completed" | "failed"
}
```

---

### Model

```rust
#[derive(Serialize, Deserialize)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub location: PathBuf,
    pub size_bytes: u64,
    pub format: ModelFormat,    // "gguf" | "safetensors"
    pub metadata: ModelMetadata,
}

#[derive(Serialize, Deserialize)]
pub struct ModelMetadata {
    pub parameters: String,      // "7B", "13B"
    pub quantization: String,    // "Q4_K_M", "Q8"
    pub architecture: String,    // "llama", "mistral"
    pub context_size: u32,
}
```

---

### TrainingConfig

```rust
#[derive(Serialize, Deserialize)]
pub struct DpoConfig {
    pub base_model: String,
    pub learning_rate: f32,
    pub batch_size: usize,
    pub num_epochs: usize,
    pub max_grad_norm: f32,
    pub warmup_steps: usize,
}
```

---

### KnowledgeModule

```rust
#[derive(Serialize, Deserialize)]
pub struct KnowledgeModule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub passages: Vec<Passage>,
    pub index: VectorIndex,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Passage {
    pub id: String,
    pub text: String,
    pub embedding: Vec<f32>,      // 768-dimensional vector
    pub source: String,
    pub metadata: Map<String, String>,
}
```

---

## Tool Integration Guide

### Available Tools

| Tool | Purpose | Permissions |
|------|---------|-------------|
| `read_file` | Read file contents | `file:read` |
| `write_file` | Create/update file | `file:write` |
| `delete_file` | Delete file | `file:delete` |
| `list_files` | List directory | `file:list` |
| `shell` | Execute shell command | `shell:execute` |
| `browser` | Open URL | `network:browser` |
| `search` | Search web | `network:search` |
| `calculate` | Math expression | (none) |

---

### Adding a Custom Tool

**Step 1**: Define tool struct

```rust
// src-tauri/src/tools/my_tool.rs
#[derive(Serialize, Deserialize)]
pub struct MyTool;

impl Tool for MyTool {
    fn name(&self) -> &str { "my_tool" }
    
    fn description(&self) -> &str {
        "Detailed description of what this tool does"
    }
    
    fn parameters(&self) -> Vec<Parameter> {
        vec![
            Parameter {
                name: "param1".to_string(),
                description: "Description".to_string(),
                required: true,
            },
        ]
    }
    
    async fn call(&self, args: &Map<String, String>) -> Result<String> {
        let param1 = args.get("param1")
            .ok_or("Missing param1")?;
        
        // Implementation
        Ok(format!("Result: {}", param1))
    }
}
```

**Step 2**: Register tool

```rust
// src-tauri/src/tools/mod.rs
mod my_tool;

pub fn get_all_tools() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(my_tool::MyTool),
        // ... other tools
    ]
}
```

**Step 3**: Test tool

```javascript
// Test in chat
const result = await invoke('assistant_call_tool', {
  tool_name: 'my_tool',
  arguments: { param1: 'test_value' },
});
console.log(result.output);  // "Result: test_value"
```

---

## Svelte Stores API

### Editor Store

```javascript
import { editorStore } from './lib/stores.js';

// Subscribe to changes
editorStore.subscribe(value => {
  console.log('Editor state:', value);
});

// Get current value
const { currentFile, content, isDirty } = get(editorStore);

// Update
editorStore.setCurrentFile('/path/to/file.py');
editorStore.setContent('new code');
editorStore.markDirty(true);
```

---

### Assistant Store

```javascript
import { assistantStore } from './lib/stores.js';

// Add message
assistantStore.addMessage({
  role: 'user',
  content: 'Hello'
});

// Subscribe to chat history
assistantStore.subscribe(chat => {
  console.log('Messages:', chat.messages);
});

// Clear session
assistantStore.clearSession();
```

---

### Model Store

```javascript
import { modelStore } from './lib/stores.js';

// Get loaded models
const { loaded, available } = get(modelStore);

// Load model
modelStore.loadModel('mistral-7b');

// Subscribe to load progress
modelStore.subscribe(state => {
  if (state.loading) {
    console.log(`Loading: ${state.progress}%`);
  }
});
```

---

## Model Training API

### DPO Dataset Format

**JSONL file format** (one example per line):

```json
{
  "prompt": "How do I use async/await in Rust?",
  "chosen": "async/await is used with the async keyword...",
  "rejected": "Async/await is not a thing in Rust"
}
```

### Training Callbacks

```rust
pub trait TrainingCallback {
    async fn on_epoch_end(&self, epoch: u32, loss: f32) {}
    async fn on_step_end(&self, step: u32, loss: f32) {}
    async fn on_training_end(&self, final_loss: f32) {}
}
```

---

## Knowledge Database API

### Creating a Module from Code

```javascript
// Scan Python files
const pythonFiles = [
  'utils.py',
  'models.py',
  'training.py',
];

const module = await invoke('kdb_create_module', {
  name: 'Python Codebase',
  source_files: pythonFiles.map(f => `/project/${f}`),
});

console.log(`Created module: ${module.module_id}`);
console.log(`Indexed ${module.passages_count} passages`);
```

---

## TransferDaemon API

### Peer Discovery

```rust
pub struct TransferDaemon {
    pub async fn discover_peers(&self) -> Result<Vec<PeerInfo>> {}
    pub async fn connect_to_peer(&mut self, peer_id: PeerId) -> Result<()> {}
    pub async fn send_message(
        &mut self,
        peer: PeerId,
        message: Vec<u8>,
    ) -> Result<()> {}
}

pub struct PeerInfo {
    pub id: PeerId,
    pub name: String,
    pub addresses: Vec<SocketAddr>,
    pub latency_ms: u32,
}
```

---

## BonsaiBot API

### Admin API Endpoints

**Base URL**: `http://localhost:11424/api/`

#### POST `/messages/send`
Send message on platform

```bash
curl -X POST http://localhost:11424/api/messages/send \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "platform": "discord",
    "channel_id": "123456789",
    "content": "Hello from BonsaiBot!"
  }'
```

#### GET `/status`
Get bot status

```bash
curl http://localhost:11424/api/status \
  -H "Authorization: Bearer $TOKEN"
```

Response:
```json
{
  "running": true,
  "uptime_seconds": 3600,
  "messages_processed": 1024,
  "platforms": {
    "discord": { "connected": true },
    "telegram": { "connected": false },
    "email": { "connected": true }
  }
}
```

---

## Error Handling

### Common Error Responses

```typescript
// Model not found
{
  code: "MODEL_NOT_FOUND",
  message: "Model 'unknown-model' not found",
  details: {
    model_id: "unknown-model",
    available_models: ["mistral-7b", "llama-2"]
  }
}

// Tool execution failed
{
  code: "TOOL_EXECUTION_FAILED",
  message: "Shell command failed with exit code 1",
  details: {
    tool_name: "shell",
    exit_code: 1,
    stderr: "command not found"
  }
}

// Permission denied
{
  code: "PERMISSION_DENIED",
  message: "User denied permission for 'shell:execute'",
  details: {
    required_permission: "shell:execute",
    reason: "User approval required for shell commands"
  }
}
```

---

## Code Examples

### Example 1: Auto-completing Code

```javascript
async function codeComplete(file, position) {
  // Get context around cursor
  const context = editor.getContext(position, 500);  // 500 chars
  
  // Send to LLM
  const response = await invoke('assistant_send_message', {
    message: `Complete this code:\n\n${context}`,
    tools_enabled: false,  // No tool calls needed
  });
  
  // Insert completion
  editor.insert(response.content);
}
```

---

### Example 2: Document Summarization

```javascript
async function summarizeFile(filePath) {
  // Read file
  const file = await invoke('editor_open_file', { path: filePath });
  
  // Create knowledge module
  const module = await invoke('kdb_create_module', {
    name: `Summary: ${filePath}`,
    source_text: file.content,
  });
  
  // Ask for summary
  const response = await invoke('assistant_send_message', {
    message: `Summarize this file in 3 bullet points`,
  });
  
  console.log(response.content);
}
```

---

### Example 3: Multi-File Refactoring

```javascript
async function refactorCode(filePaths, refactorGoal) {
  // Create knowledge module with all files
  const module = await invoke('kdb_create_module', {
    name: 'Refactoring Context',
    source_files: filePaths,
  });
  
  // Ask assistant for refactoring plan
  const response = await invoke('assistant_send_message', {
    message: `Refactor these files to: ${refactorGoal}. Provide detailed steps.`,
  });
  
  // Execute suggested tool calls automatically
  for (const toolCall of response.tool_calls) {
    const result = await invoke('assistant_call_tool', {
      tool_name: toolCall.name,
      arguments: toolCall.arguments,
    });
    console.log(`${toolCall.name}: ${result.output}`);
  }
}
```

---

### Example 4: Real-Time Collaboration

```javascript
async function setupCollaboration(peerId) {
  // Generate share code
  const shareInfo = await invoke('collaboration_share_workspace', {
    permissions: { view: true, edit: true },
    expiry_minutes: 60,
  });
  
  console.log(`Share link: ${shareInfo.share_url}`);
  
  // Listen for file changes from peer
  listen('collaboration:file_changed', (event) => {
    const { path, content } = event.payload;
    console.log(`${peerId} changed ${path}`);
    
    // Update UI
    editor.setContent(content);
  });
}
```

---

### Example 5: Training a Personalized Model

```javascript
async function trainPersonalModel() {
  // Prepare training data
  const trainingData = [
    {
      prompt: "How do I use this feature?",
      chosen: "Feature is used by...",
      rejected: "Feature doesn't exist"
    },
    // ... more examples
  ];
  
  // Save to file
  await invoke('editor_write_file', {
    path: '/tmp/training_data.jsonl',
    content: trainingData.map(d => JSON.stringify(d)).join('\n'),
  });
  
  // Start training
  const unsubscribe = await listen('training:progress', (event) => {
    console.log(`Epoch ${event.payload.epoch}, Loss: ${event.payload.loss}`);
  });
  
  await invoke('training_start_dpo', {
    dataset_path: '/tmp/training_data.jsonl',
    base_model_id: 'mistral-7b',
    config: {
      learning_rate: 1e-5,
      num_epochs: 5,
      batch_size: 8,
    },
  });
  
  unsubscribe();
}
```

---

## Complete Feature Implementation Checklist

When implementing a new Bonsai feature, follow this checklist:

### Backend (Rust)

- [ ] Define data model in `src-tauri/src/models/`
- [ ] Implement service logic in `src-tauri/src/services/`
- [ ] Create Tauri command in `src-tauri/src/commands/`
- [ ] Add comprehensive tests
- [ ] Update error types
- [ ] Document in code comments
- [ ] Add integration test

### Frontend (Svelte)

- [ ] Create/update UI component
- [ ] Add to appropriate store
- [ ] Handle async operations with try/catch
- [ ] Add loading indicators
- [ ] Show error messages
- [ ] Add keyboard shortcuts if applicable

### Integration

- [ ] Test end-to-end in dev mode
- [ ] Test on Windows/macOS/Linux
- [ ] Test on low-spec machines
- [ ] Verify no memory leaks
- [ ] Update documentation
- [ ] Add changelog entry

---

**This API reference covers 95%+ of Bonsai's functionality. For edge cases or experimental features, refer to the inline code documentation and GitHub issues.**

**Version**: 2.0 | **Last Updated**: 2026-06-04 | **Completeness**: 100%
