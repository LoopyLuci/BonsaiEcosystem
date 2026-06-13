# Copilot Tool Translation Engine - Design Document

**Version:** 1.0  
**Date:** 2026-06-01  
**Status:** Design Phase (Pre-implementation)

---

## Executive Summary

The **Copilot Tool Translation Engine** is the system that maps GitHub Copilot's 40+ built-in tools to Bonsai's native MCP tool ecosystem, enabling seamless bidirectional translation between tool schemas, execution models, and safety/permissions frameworks.

This document specifies:
1. A complete taxonomy of Copilot tools and their Bonsai equivalents
2. Bidirectional translation specifications for 15+ major tool categories
3. Schema translation rules with concrete examples
4. Bridge tool architecture for cloud fallback
5. Tool schema registry format and design
6. Confidence scoring for safe tool invocation
7. Tool composition and dependency tracking
8. Error handling and rollback strategies
9. Telemetry and observability hooks
10. Versioning and compatibility strategies

---

## 1. Tool Taxonomy & Classification

### 1.1 Complete Copilot Tool Inventory (40+ Tools)

This taxonomy classifies tools by semantic category, required permissions, risk level, and Bonsai mapping strategy.

| # | Copilot Tool | Category | Risk Level | Permissions | Bonsai Mapping | Strategy |
|---|---|---|---|---|---|---|
| 1 | `read_file` | File I/O | LOW | file:read | `read_file` | Direct |
| 2 | `write_file` | File I/O | MEDIUM | file:write | `write_file` | Direct |
| 3 | `delete_file` | File I/O | HIGH | file:delete | *N/A* | Bridge + approval |
| 4 | `list_files` | File I/O | LOW | file:list | *N/A* | Custom wrapper |
| 5 | `get_file_metadata` | File I/O | LOW | file:read | *N/A* | Custom wrapper |
| 6 | `search_codebase` | Code Analysis | MEDIUM | search:read | KDB semantic search | Direct |
| 7 | `find_symbol_definition` | Code Analysis | MEDIUM | search:read | KDB + syntax tree | Bridge |
| 8 | `get_symbol_references` | Code Analysis | MEDIUM | search:read | KDB + line numbers | Bridge |
| 9 | `get_implementation` | Code Analysis | MEDIUM | search:read | KDB + AST analysis | Bridge |
| 10 | `get_diagnostics` | Code Analysis | MEDIUM | linter:read | Fabric integration | Bridge |
| 11 | `get_type_info` | Code Analysis | MEDIUM | search:read | KDB + type stubs | Bridge |
| 12 | `git_commit` | VCS | HIGH | vcs:write | `git_commit` | Direct + approval |
| 13 | `git_branch` | VCS | HIGH | vcs:write | `git_branch` | Direct + approval |
| 14 | `git_diff` | VCS | LOW | vcs:read | `git_diff` | Direct |
| 15 | `git_log` | VCS | LOW | vcs:read | `git_log` | Direct |
| 16 | `git_merge` | VCS | HIGH | vcs:write | `git_merge` | Direct + approval |
| 17 | `git_rebase` | VCS | HIGH | vcs:write | *N/A* | Cloud fallback |
| 18 | `git_stash` | VCS | MEDIUM | vcs:write | `git_stash` | Direct |
| 19 | `create_pr` | VCS | HIGH | github:write | `github_create_pr` | Direct + GitHub auth |
| 20 | `merge_pr` | VCS | HIGH | github:write | `github_merge_pr` | Direct + approval |
| 21 | `create_issue` | VCS | MEDIUM | github:write | `github_create_issue` | Direct + GitHub auth |
| 22 | `add_code_review_comment` | VCS | MEDIUM | github:write | `github_add_review_comment` | Direct + GitHub auth |
| 23 | `run_tests` | Execution | MEDIUM | exec:run | `run_cargo_test` | Direct |
| 24 | `run_build` | Execution | MEDIUM | exec:run | `run_cargo_check` | Direct |
| 25 | `compile_code` | Execution | MEDIUM | exec:run | Fabric compile task | Bridge |
| 26 | `execute_shell` | Execution | HIGH | exec:shell | `shell_execute` | Direct + approval |
| 27 | `run_lint` | Execution | LOW | linter:run | Fabric lint task | Bridge |
| 28 | `get_diagnostics` | Execution | LOW | linter:read | Universe crash logs | Bridge |
| 29 | `web_search` | Network | MEDIUM | network:search | `web_search` | Direct (filtered) |
| 30 | `fetch_url` | Network | MEDIUM | network:fetch | `fetch_url` | Direct (with proxy) |
| 31 | `api_call` | Network | HIGH | network:api | Cloud fallback | Bridge + approval |
| 32 | `get_documentation` | Documentation | LOW | doc:read | KDB doc modules | Direct |
| 33 | `search_documentation` | Documentation | LOW | doc:read | KDB search | Direct |
| 34 | `get_stdlib_reference` | Documentation | LOW | doc:read | KDB doc modules | Direct |
| 35 | `generate_tests` | AI Generation | MEDIUM | inference:run | Local inference | Bridge + approval |
| 36 | `generate_documentation` | AI Generation | MEDIUM | inference:run | Local inference | Bridge + approval |
| 37 | `refactor_code` | AI Generation | HIGH | inference:run | Local inference | Bridge + approval |
| 38 | `suggest_variable_name` | AI Generation | LOW | inference:run | Local inference | Direct |
| 39 | `explain_code` | AI Generation | LOW | inference:run | Local inference | Direct |
| 40 | `fix_compile_error` | AI Generation | MEDIUM | inference:run | Universe survival system | Direct |

#### Risk Level Classifications

- **LOW:** Read-only, no user approval needed, no state modification
- **MEDIUM:** Requires careful scoping, user may want review, limited state change
- **HIGH:** Modifies critical state (code, branches, issues), requires explicit user approval

#### Mapping Strategy

- **Direct:** Bonsai has equivalent tool with compatible schema
- **Bridge:** Bonsai has partial support; wrapper required for full semantics
- **Cloud fallback:** No local equivalent; delegate to GitHub API or Anthropic API

---

### 1.2 Permission Model

Bonsai's permission system extends Copilot's flat permission model with hierarchical scoping:

```
file:
  - read
    - metadata
    - content
  - write
    - create
    - modify
    - delete
  - permissions

vcs:
  - read
    - log
    - diff
    - branch
  - write
    - commit
    - merge
    - push

github:
  - read
    - issues
    - prs
    - discussions
  - write
    - create_issue
    - create_pr
    - review
    - merge

exec:
  - run
    - tests
    - build
  - shell

search:
  - codebase
  - documentation
  - web

inference:
  - run (local models only)
  - generate_tests
  - refactor
```

---

## 2. Translation Pipeline Architecture

```
┌────────────────────────────────────────────────────────────┐
│ GitHub Copilot (External Agent)                             │
│ Generates tool calls with MCP schema                         │
└──────────────────┬─────────────────────────────────────────┘
                   │
                   ▼
         ┌─────────────────────┐
         │ Request Validator    │
         │ - Check auth token   │
         │ - Validate schema    │
         │ - Extract tool name  │
         └────────┬────────────┘
                  │
                  ▼
        ┌──────────────────────────┐
        │ Tool Schema Registry      │
        │ - Lookup tool definition │
        │ - Get translation rules  │
        │ - Get Bonsai equiv       │
        └────────┬─────────────────┘
                 │
         ┌───────┴────────┐
         │                │
         ▼                ▼
    ┌─────────┐     ┌───────────┐
    │ Direct  │     │ Bridge    │
    │ Mapper  │     │ Handler   │
    └────┬────┘     └─────┬─────┘
         │                │
         ▼                ▼
    ┌───────────────────────────────┐
    │ Parameter Translator           │
    │ - Rename fields               │
    │ - Convert types               │
    │ - Validate bounds             │
    │ - Apply defaults              │
    └──────────┬────────────────────┘
               │
    ┌──────────▼────────────────────┐
    │ Approval Gate                  │
    │ - Confidence scoring           │
    │ - Risk evaluation              │
    │ - User prompt (if needed)      │
    │ - Permission check             │
    └──────────┬─────────────────────┘
               │
    ┌──────────▼────────────────────┐
    │ Execution Dispatcher           │
    │ - Route to Bonsai MCP tools   │
    │ - Route to bridge handlers    │
    │ - Route to cloud fallback     │
    └──────────┬─────────────────────┘
               │
    ┌──────────▼─────────────────┐
    │ Result Translator           │
    │ - Bonsai result → Copilot  │
    │ - Error mapping            │
    │ - Timing data              │
    └──────────┬──────────────────┘
               │
    ┌──────────▼──────────────────────┐
    │ Telemetry & Universe Logging     │
    │ - Tool invocation event         │
    │ - Result status                 │
    │ - Approval decision             │
    │ - Latency metrics               │
    └──────────┬───────────────────────┘
               │
               ▼
    ┌──────────────────────────────┐
    │ Response to Copilot           │
    │ (MCP tool result format)       │
    └──────────────────────────────┘
```

---

## 3. Bidirectional Tool Translation Specifications

### 3.1 File I/O Tools

#### 3.1.1 `read_file`

**Copilot Schema:**
```json
{
  "name": "read_file",
  "description": "Read the contents of a file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "The file path to read (relative to workspace root or absolute)"
      },
      "encoding": {
        "type": "string",
        "enum": ["utf-8", "ascii", "utf-16"],
        "description": "File encoding (default: utf-8)"
      },
      "max_size_bytes": {
        "type": "integer",
        "description": "Optional: read at most N bytes (for large files)"
      }
    },
    "required": ["path"]
  }
}
```

**Bonsai MCP Schema:**
```json
{
  "name": "read_file",
  "description": "Read the contents of a file in the Bonsai workspace",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": {
        "type": "string",
        "description": "The file path to read"
      }
    },
    "required": ["path"]
  }
}
```

**Translation Rules:**

| Copilot Field | Bonsai Field | Logic |
|---|---|---|
| `path` | `path` | 1:1 pass-through; resolve to absolute path |
| `encoding` | *(ignored)* | Bonsai always returns UTF-8; no parameter needed |
| `max_size_bytes` | *(ignored)* | Bonsai streams large files; client paginates if needed |

**Result Mapping:**

| Bonsai Result | Copilot Result | Transform |
|---|---|---|
| `{ content: string, size: int, path: string }` | `{ content: string, encoding: "utf-8", size: int, path: string }` | Add encoding field |

**Example:**

```
Copilot call:
POST /tool/read_file
{ "path": "src/main.rs", "max_size_bytes": 10000 }

Translator:
- Resolve path: /z/Projects/BonsaiWorkspace/src/main.rs
- Drop max_size_bytes (handled by streaming)
- Call Bonsai: POST /call/read_file with { "path": "/z/Projects/BonsaiWorkspace/src/main.rs" }

Bonsai returns:
{ "content": "fn main() { ... }", "size": 2847, "path": "/z/Projects/BonsaiWorkspace/src/main.rs" }

Translator returns to Copilot:
{ "content": "fn main() { ... }", "size": 2847, "encoding": "utf-8", "path": "src/main.rs" }
```

---

#### 3.1.2 `write_file`

**Copilot Schema:**
```json
{
  "name": "write_file",
  "description": "Create or overwrite a file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": { "type": "string" },
      "content": { "type": "string", "description": "File content to write" },
      "mode": {
        "type": "string",
        "enum": ["create", "overwrite", "append"],
        "description": "Write mode (default: overwrite)"
      },
      "encoding": { "type": "string", "enum": ["utf-8", "ascii", "utf-16"], "default": "utf-8" },
      "create_parents": {
        "type": "boolean",
        "description": "Create parent directories if needed (default: true)"
      }
    },
    "required": ["path", "content"]
  }
}
```

**Bonsai MCP Schema:**
```json
{
  "name": "write_file",
  "description": "Create or overwrite a file",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": { "type": "string" },
      "content": { "type": "string" }
    },
    "required": ["path", "content"]
  }
}
```

**Translation Rules:**

| Copilot Field | Bonsai Field | Logic |
|---|---|---|
| `path` | `path` | 1:1; resolve to absolute; check no escape attacks |
| `content` | `content` | 1:1; always UTF-8 |
| `mode` | *(special handling)* | `append` → read file, concatenate, write; other modes → direct write |
| `encoding` | *(ignored)* | All output is UTF-8 |
| `create_parents` | *(ignored)* | Bonsai always creates parents |

**Approval Gate:**
- HIGH risk if path is outside workspace root
- MEDIUM risk if overwriting existing file > 10KB or critical files (Cargo.toml, .git/, etc.)
- LOW risk for new files in src/

**Result Mapping:**

| Bonsai Result | Copilot Result |
|---|---|
| `{ path: string, size: int, status: "ok" }` | `{ path: string, size: int, encoding: "utf-8", status: "ok" }` |

**Universe Event:**
```rust
UniverseEvent {
    category: EventCategory::FileChange,
    summary: format!("write_file: {} ({} bytes)", path, size),
    source: EventSource::Tool { tool: "copilot:write_file" },
    target: format!("file:{}", path),
}
```

---

#### 3.1.3 `list_files`

**Copilot Schema:**
```json
{
  "name": "list_files",
  "description": "List files in a directory",
  "inputSchema": {
    "type": "object",
    "properties": {
      "path": { "type": "string", "description": "Directory path (default: workspace root)" },
      "recursive": { "type": "boolean", "description": "Recursively list subdirs" },
      "pattern": { "type": "string", "description": "Optional: glob pattern to filter" },
      "max_results": { "type": "integer", "description": "Max files to return (default: 1000)" }
    },
    "required": []
  }
}
```

**Bonsai Implementation:**
- No direct MCP tool exists
- Create bridge: wrap `glob` pattern matching + caching
- Cache results for 60s to avoid repeated scans
- Reject patterns that escape root (e.g., `../../`)

**Bridge Implementation (Pseudocode):**
```python
def bridge_list_files(path, recursive, pattern, max_results):
    # Default to workspace root
    path = path or workspace_root()
    
    # Validate path is within workspace
    abs_path = resolve_path(path)
    if not is_subdir(abs_path, workspace_root()):
        raise PermissionError("Path outside workspace")
    
    # Build glob pattern
    if recursive:
        glob_pattern = f"{abs_path}/**/{pattern or '*'}"
    else:
        glob_pattern = f"{abs_path}/{pattern or '*'}"
    
    # Execute glob
    files = glob(glob_pattern, recursive=recursive)[:max_results]
    
    # Enrich with metadata
    result = []
    for f in files:
        result.append({
            "path": relative_to_workspace(f),
            "type": "file" if is_file(f) else "directory",
            "size": file_size(f),
            "mtime": mtime(f)
        })
    
    return result
```

---

### 3.2 Code Analysis Tools

#### 3.2.1 `search_codebase`

**Copilot Schema:**
```json
{
  "name": "search_codebase",
  "description": "Semantic search across the codebase",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": { "type": "string", "description": "Search query (natural language or regex)" },
      "language": { "type": "string", "enum": ["rust", "python", "js", "all"], "default": "all" },
      "limit": { "type": "integer", "minimum": 1, "maximum": 100, "default": 10 }
    },
    "required": ["query"]
  }
}
```

**Bonsai Mapping:**

Bonsai's KDB (Knowledge Database) provides semantic search via embeddings:

```python
def translate_search_codebase(query, language, limit):
    """Map Copilot search to KDB semantic search"""
    
    # Load active code module from KDB
    kdb = get_kdb()
    code_module = kdb.get_module("code")  # or language-specific module
    
    if not code_module:
        raise RuntimeError("Code module not loaded in KDB")
    
    # Generate embedding for query
    embedding = embed(query)
    
    # Search with KDB (returns top-k by cosine similarity)
    results = code_module.search(embedding, limit=limit)
    
    # Filter by language if specified
    if language != "all":
        results = [r for r in results if r["language"] == language]
    
    # Return results with line numbers and context
    formatted = []
    for r in results:
        formatted.append({
            "path": r["file"],
            "line": r["line_number"],
            "snippet": r["text"],  # Context around match
            "confidence": r["score"]  # 0.0-1.0 similarity score
        })
    
    return formatted
```

**Result Schema:**
```json
{
  "matches": [
    {
      "path": "src/lib.rs",
      "line": 42,
      "snippet": "fn process(data: &[u8]) -> Result<String> { ... }",
      "confidence": 0.89,
      "language": "rust"
    }
  ],
  "total_matches": 5,
  "search_time_ms": 125
}
```

**KDB Module Configuration:**

The code module should include:
- All source files tokenized and embedded
- Language metadata for each entry
- Line number mapping for accuracy
- Zstd-compressed snippets for memory efficiency

---

#### 3.2.2 `find_symbol_definition`

**Copilot Schema:**
```json
{
  "name": "find_symbol_definition",
  "description": "Find where a symbol (function, type, variable) is defined",
  "inputSchema": {
    "type": "object",
    "properties": {
      "symbol": { "type": "string", "description": "Symbol name" },
      "language": { "type": "string", "enum": ["rust", "python", "js"] },
      "file": { "type": "string", "description": "Optional: search in this file first" }
    },
    "required": ["symbol", "language"]
  }
}
```

**Bonsai Mapping:**

No direct MCP tool. Create a bridge that combines KDB + syntax analysis:

```python
def bridge_find_symbol_definition(symbol, language, file):
    """Find where a symbol is defined"""
    
    # Step 1: Search KDB for definition patterns
    patterns = {
        "rust": [
            f"fn {symbol}",        # function
            f"struct {symbol}",    # struct
            f"trait {symbol}",     # trait
            f"enum {symbol}",      # enum
            f"type {symbol} ="     # type alias
        ],
        "python": [
            f"def {symbol}",       # function
            f"class {symbol}",     # class
            f"{symbol} ="          # variable/constant
        ],
        "js": [
            f"function {symbol}",
            f"const {symbol} =",
            f"class {symbol}"
        ]
    }
    
    for pattern in patterns.get(language, []):
        results = kdb.search(pattern)
        if results:
            # Return first match
            r = results[0]
            return {
                "path": r["file"],
                "line": r["line_number"],
                "snippet": r["text"],
                "symbol_type": infer_symbol_type(r["text"])
            }
    
    raise ValueError(f"Symbol '{symbol}' not found")
```

**Result Schema:**
```json
{
  "path": "src/lib.rs",
  "line": 42,
  "column": 4,
  "symbol_type": "fn",
  "snippet": "fn process(data: &[u8]) -> Result<String> {",
  "documentation": "Process binary data and return UTF-8 string"
}
```

---

### 3.3 VCS Tools

#### 3.3.1 `git_commit`

**Copilot Schema:**
```json
{
  "name": "git_commit",
  "description": "Create a commit with the given message",
  "inputSchema": {
    "type": "object",
    "properties": {
      "message": { "type": "string", "description": "Commit message" },
      "files": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Files to stage before committing (default: all)"
      },
      "author_email": { "type": "string", "description": "Optional: override commit author email" },
      "amend": { "type": "boolean", "description": "Amend the previous commit" }
    },
    "required": ["message"]
  }
}
```

**Bonsai Mapping:**

Bonsai already has git tooling; create a wrapper that enforces safety:

```python
def translate_git_commit(message, files, author_email, amend):
    """Translate Copilot git_commit to Bonsai git tooling"""
    
    # Validation
    if len(message) < 5:
        raise ValueError("Commit message must be at least 5 chars")
    
    if amend:
        # High risk: amending changes history
        require_approval("git_commit_amend")
    
    # Stage files
    if files:
        for f in files:
            git_add(f)
    else:
        git_add(".")  # Stage all
    
    # Prepare commit
    commit_args = {
        "message": message,
        "amend": amend
    }
    
    if author_email:
        commit_args["author"] = author_email
    
    # Create commit
    commit_hash = git_commit(**commit_args)
    
    return {
        "commit_hash": commit_hash,
        "message": message,
        "files_changed": get_commit_stats(commit_hash)
    }
```

**Universe Event:**
```rust
UniverseEvent {
    category: EventCategory::VCSAction,
    summary: format!("commit: {} ({})", commit_hash[..8], message),
    source: EventSource::Tool { tool: "copilot:git_commit" },
    target: format!("commit:{}", commit_hash)
}
```

**Approval Requirements:**
- Always require user confirmation before committing
- Show diff preview
- Validate message quality (length, format)
- Check for secrets in staged files (using scanning tools)

---

#### 3.3.2 `create_pr`

**Copilot Schema:**
```json
{
  "name": "create_pr",
  "description": "Create a GitHub pull request",
  "inputSchema": {
    "type": "object",
    "properties": {
      "title": { "type": "string" },
      "description": { "type": "string" },
      "base_branch": { "type": "string", "description": "Target branch (default: main)" },
      "head_branch": { "type": "string", "description": "Source branch" },
      "draft": { "type": "boolean", "description": "Create as draft PR" },
      "labels": { "type": "array", "items": { "type": "string" } },
      "reviewers": { "type": "array", "items": { "type": "string" } }
    },
    "required": ["title", "head_branch"]
  }
}
```

**Bonsai Mapping:**

This requires GitHub authentication and API access. Create a bridge that:

1. Validates GitHub credentials are available
2. Checks that the head branch exists locally
3. Requires explicit user approval (HIGH risk)
4. Delegates to GitHub API via authenticated bridge

```python
def bridge_create_pr(title, description, base_branch, head_branch, draft, labels, reviewers):
    """Create a GitHub PR"""
    
    # Check auth
    github_token = get_github_token()
    if not github_token:
        raise PermissionError("GitHub token not configured")
    
    # Validate branches exist
    if not git_branch_exists(head_branch):
        raise ValueError(f"Branch '{head_branch}' does not exist")
    
    # Require approval (HIGH RISK)
    require_approval("create_pr", {
        "title": title,
        "base": base_branch,
        "head": head_branch,
        "draft": draft
    })
    
    # Call GitHub API
    pr = github_api(
        method="POST",
        endpoint="/repos/{owner}/{repo}/pulls",
        data={
            "title": title,
            "body": description,
            "base": base_branch,
            "head": head_branch,
            "draft": draft
        },
        token=github_token
    )
    
    # Add labels and reviewers
    if labels:
        github_api(
            method="POST",
            endpoint=f"/repos/{{owner}}/{{repo}}/issues/{pr['number']}/labels",
            data={"labels": labels}
        )
    
    if reviewers:
        github_api(
            method="POST",
            endpoint=f"/repos/{{owner}}/{{repo}}/pulls/{pr['number']}/requested_reviewers",
            data={"reviewers": reviewers}
        )
    
    return {
        "pr_number": pr["number"],
        "url": pr["html_url"],
        "status": "draft" if draft else "open"
    }
```

**Universe Event:**
```rust
UniverseEvent {
    category: EventCategory::VCSAction,
    summary: format!("PR #{}: {}", pr_number, title),
    source: EventSource::Tool { tool: "copilot:create_pr" },
    target: format!("pr:{}", pr_number)
}
```

---

### 3.4 Execution Tools

#### 3.4.1 `run_tests`

**Copilot Schema:**
```json
{
  "name": "run_tests",
  "description": "Run test suite",
  "inputSchema": {
    "type": "object",
    "properties": {
      "filter": { "type": "string", "description": "Test name filter or regex" },
      "timeout_seconds": { "type": "integer", "default": 300 },
      "verbose": { "type": "boolean", "default": false }
    },
    "required": []
  }
}
```

**Bonsai Mapping:**

Direct mapping to `run_cargo_test`:

```python
def translate_run_tests(filter, timeout_seconds, verbose):
    """Translate to Bonsai test execution"""
    
    # Build cargo test command
    args = []
    if filter:
        args.append(filter)
    
    if verbose:
        args.append("--verbose")
    
    # Set timeout
    timeout = min(timeout_seconds, 600)  # Cap at 10 min
    
    # Execute via Bonsai MCP
    result = call_bonsai_tool(
        tool="run_cargo_test",
        args={"filter": filter, "verbose": verbose},
        timeout_ms=timeout * 1000
    )
    
    return {
        "status": result.get("status"),
        "tests_run": result.get("tests_run"),
        "tests_passed": result.get("tests_passed"),
        "tests_failed": result.get("tests_failed"),
        "output": result.get("output"),
        "duration_ms": result.get("duration_ms")
    }
```

---

#### 3.4.2 `execute_shell`

**Copilot Schema:**
```json
{
  "name": "execute_shell",
  "description": "Execute a shell command",
  "inputSchema": {
    "type": "object",
    "properties": {
      "command": { "type": "string" },
      "cwd": { "type": "string", "description": "Working directory" },
      "timeout_seconds": { "type": "integer", "default": 60 },
      "capture_output": { "type": "boolean", "default": true }
    },
    "required": ["command"]
  }
}
```

**Bonsai Mapping:**

HIGH RISK. Requires approval + sandboxing:

```python
def bridge_execute_shell(command, cwd, timeout_seconds, capture_output):
    """Execute shell command with safety checks"""
    
    # Parse command
    parsed = parse_shell_command(command)
    
    # HIGH RISK: require approval
    require_approval("execute_shell", {
        "command": command,
        "cwd": cwd or ".",
        "timeout": timeout_seconds
    })
    
    # Validate command is safe
    dangerous_cmds = ["rm ", "dd ", "mkfs", "format", ":/", ":rm"]
    for danger in dangerous_cmds:
        if danger in command.lower():
            raise PermissionError(f"Command blocked: '{danger}'")
    
    # Execute in sandbox
    result = execute_command_sandboxed(
        command=command,
        cwd=cwd or workspace_root(),
        timeout=timeout_seconds,
        capture=capture_output
    )
    
    return {
        "exit_code": result.exit_code,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "duration_ms": result.duration_ms
    }
```

---

### 3.5 Network Tools

#### 3.5.1 `web_search`

**Copilot Schema:**
```json
{
  "name": "web_search",
  "description": "Search the web using Bing Search",
  "inputSchema": {
    "type": "object",
    "properties": {
      "query": { "type": "string" },
      "num_results": { "type": "integer", "minimum": 1, "maximum": 50, "default": 10 }
    },
    "required": ["query"]
  }
}
```

**Bonsai Mapping:**

Direct mapping to Bonsai's web_search tool:

```python
def translate_web_search(query, num_results):
    """Translate to Bonsai web_search"""
    
    # Validate query doesn't contain sensitive info
    if contains_secrets(query):
        raise PermissionError("Query contains sensitive information")
    
    # Call Bonsai web_search
    results = call_bonsai_tool(
        tool="web_search",
        args={"query": query, "head_limit": min(num_results, 50)}
    )
    
    return {
        "results": [
            {
                "title": r["title"],
                "url": r["url"],
                "snippet": r["snippet"]
            }
            for r in results
        ],
        "total": len(results)
    }
```

---

#### 3.5.2 `api_call`

**Copilot Schema:**
```json
{
  "name": "api_call",
  "description": "Make an HTTP API call",
  "inputSchema": {
    "type": "object",
    "properties": {
      "url": { "type": "string" },
      "method": { "type": "string", "enum": ["GET", "POST", "PUT", "DELETE"] },
      "headers": { "type": "object", "additionalProperties": { "type": "string" } },
      "body": { "type": "string" },
      "timeout_seconds": { "type": "integer", "default": 30 }
    },
    "required": ["url", "method"]
  }
}
```

**Bonsai Mapping:**

HIGH RISK. This can leak code/secrets. Implement with:
1. Require approval
2. Scan URL allowlist/denylist
3. Scan request/response for secrets
4. Log all API calls to Universe

```python
def bridge_api_call(url, method, headers, body, timeout_seconds):
    """Make an API call with security checks"""
    
    # Validate URL
    if not is_safe_url(url):
        raise PermissionError(f"URL blocked: {url}")
    
    # Scan headers for secrets
    if contains_secrets_in_dict(headers):
        raise PermissionError("Headers contain sensitive data")
    
    # Scan body for secrets
    if body and contains_secrets(body):
        raise PermissionError("Request body contains sensitive data")
    
    # Require approval for non-GET methods
    if method != "GET":
        require_approval("api_call", {
            "url": url,
            "method": method,
            "timeout": timeout_seconds
        })
    
    # Make request via proxy (never direct from Copilot process)
    response = http_proxy_request(
        url=url,
        method=method,
        headers=filter_sensitive_headers(headers),
        body=body,
        timeout=timeout_seconds
    )
    
    # Scan response for secrets before returning
    if contains_secrets(response.text):
        # Log but don't return secret
        log_universe("api_call_leaked_secrets", url)
        return {
            "status": response.status_code,
            "error": "Response contains sensitive data; blocked"
        }
    
    return {
        "status": response.status_code,
        "headers": dict(response.headers),
        "body": response.text
    }
```

---

### 3.6 AI Generation Tools

#### 3.6.1 `generate_tests`

**Copilot Schema:**
```json
{
  "name": "generate_tests",
  "description": "Generate test cases for a function",
  "inputSchema": {
    "type": "object",
    "properties": {
      "function_path": { "type": "string", "description": "File path and function name" },
      "count": { "type": "integer", "minimum": 1, "maximum": 20, "default": 5 }
    },
    "required": ["function_path"]
  }
}
```

**Bonsai Mapping:**

Use local inference + code analysis:

```python
def bridge_generate_tests(function_path, count):
    """Generate test cases via local inference"""
    
    # Parse function_path (e.g., "src/lib.rs#parse_json")
    file, func_name = parse_function_path(function_path)
    
    # Read function code
    content = read_file(file)
    func_source = extract_function(content, func_name)
    
    # Infer function signature and purpose
    signature = extract_signature(func_source)
    docstring = extract_docstring(func_source)
    
    # Build prompt for test generation
    prompt = f"""
Generate {count} unit tests for this {extract_language(file)} function:

{signature}

Documentation:
{docstring}

Source:
{func_source}

Generate {count} diverse test cases that cover:
- Normal cases
- Edge cases
- Error cases
- Boundary conditions

Return test code only.
"""
    
    # Call local inference
    model = get_active_model()
    tests = model.generate(
        prompt,
        max_tokens=2000,
        temperature=0.3  # Lower temp for deterministic tests
    )
    
    # Require approval before writing
    require_approval("generate_tests", {
        "function": func_name,
        "file": file,
        "count": count
    })
    
    return {
        "tests": tests,
        "function": func_name,
        "file": file
    }
```

---

## 4. Tool Schema Registry Format

### 4.1 YAML Registry Design

Each tool is registered in a YAML file with complete metadata:

```yaml
version: "1.0"
last_updated: "2026-06-01"

tools:
  - id: "read_file"
    name: "read_file"
    category: "file_io"
    risk_level: "low"
    
    # Copilot schema
    copilot_schema:
      description: "Read the contents of a file"
      properties:
        path:
          type: "string"
          description: "File path"
        encoding:
          type: "string"
          enum: ["utf-8", "ascii", "utf-16"]
          default: "utf-8"
        max_size_bytes:
          type: "integer"
    
    # Bonsai MCP mapping
    bonsai_mapping:
      strategy: "direct"  # direct | bridge | cloud_fallback
      tool_name: "read_file"
      
      # Parameter translation rules
      parameter_translation:
        - copilot_field: "path"
          bonsai_field: "path"
          transform: "resolve_to_absolute"
        - copilot_field: "encoding"
          bonsai_field: null
          transform: "drop"  # Always UTF-8
        - copilot_field: "max_size_bytes"
          bonsai_field: null
          transform: "drop"  # Handled by streaming
      
      # Result translation rules
      result_translation:
        - bonsai_field: "content"
          copilot_field: "content"
          transform: "as_is"
        - bonsai_field: "size"
          copilot_field: "size"
          transform: "as_is"
        - copilot_field: "encoding"
          constant: "utf-8"
    
    # Permissions required
    permissions_required:
      - "file:read"
    
    # Safety settings
    safety:
      approval_required: false
      user_prompt: null
      path_validation: "must_be_within_workspace"
      secret_scan: false
    
    # Telemetry
    telemetry:
      event_category: "tool_invocation"
      track_parameters: false  # Don't log file paths
      track_results: false
    
    # Versioning
    version: "1.0"
    deprecated: false
```

### 4.2 Registry Usage

```python
class ToolSchemaRegistry:
    def __init__(self, yaml_path: str):
        self.registry = load_yaml(yaml_path)
        self.tools_by_name = {t["name"]: t for t in self.registry["tools"]}
    
    def get_tool(self, name: str) -> dict:
        """Get tool definition by name"""
        return self.tools_by_name.get(name)
    
    def get_mapping_strategy(self, copilot_tool: str) -> str:
        """Get mapping strategy for a Copilot tool"""
        tool = self.get_tool(copilot_tool)
        return tool["bonsai_mapping"]["strategy"]
    
    def translate_parameters(self, tool_name: str, copilot_args: dict) -> dict:
        """Translate Copilot args to Bonsai args"""
        tool = self.get_tool(tool_name)
        bonsai_args = {}
        
        for rule in tool["bonsai_mapping"]["parameter_translation"]:
            copilot_field = rule["copilot_field"]
            bonsai_field = rule["bonsai_field"]
            transform = rule.get("transform", "as_is")
            
            if copilot_field not in copilot_args:
                continue
            
            if bonsai_field is None:
                # Drop this parameter
                continue
            
            value = copilot_args[copilot_field]
            
            # Apply transformation
            if transform == "as_is":
                bonsai_args[bonsai_field] = value
            elif transform == "resolve_to_absolute":
                bonsai_args[bonsai_field] = resolve_path(value)
            elif transform == "drop":
                continue
        
        return bonsai_args
    
    def translate_result(self, tool_name: str, bonsai_result: dict) -> dict:
        """Translate Bonsai result to Copilot result"""
        tool = self.get_tool(tool_name)
        copilot_result = {}
        
        for rule in tool["bonsai_mapping"]["result_translation"]:
            bonsai_field = rule.get("bonsai_field")
            copilot_field = rule.get("copilot_field")
            constant = rule.get("constant")
            
            if constant is not None:
                # Add constant field
                copilot_result[copilot_field] = constant
            elif bonsai_field and bonsai_field in bonsai_result:
                copilot_result[copilot_field] = bonsai_result[bonsai_field]
        
        return copilot_result
    
    def requires_approval(self, tool_name: str) -> bool:
        """Check if tool requires user approval"""
        tool = self.get_tool(tool_name)
        return tool["safety"]["approval_required"]
    
    def get_permissions(self, tool_name: str) -> List[str]:
        """Get required permissions for tool"""
        tool = self.get_tool(tool_name)
        return tool["permissions_required"]
```

---

## 5. Confidence Scoring Algorithm

A tool call is scored 0.0-1.0 based on:
1. User approval history (0.0-0.3)
2. Parameter safety patterns (0.0-0.2)
3. Tool risk baseline (0.2-1.0)
4. Permission availability (0.0-0.2)
5. Tool chain safety (0.0-0.1)

### 5.1 Scoring Formula

```python
def compute_confidence_score(
    tool_name: str,
    args: dict,
    user_id: str,
    context: ToolInvocationContext
) -> float:
    """Compute confidence score for tool invocation"""
    
    base_score = 0.0
    
    # 1. User approval history (0.0-0.3)
    history = get_user_approval_history(user_id, tool_name)
    if history.total_approvals > 10:
        history_score = 0.3  # Well-established trust
    elif history.approval_rate > 0.9:
        history_score = 0.2  # High approval rate
    elif history.approval_rate > 0.7:
        history_score = 0.1  # Moderate approval
    else:
        history_score = 0.0
    
    # 2. Parameter safety (0.0-0.2)
    param_score = 0.0
    
    # Read-only operations are safer
    if is_read_only_operation(tool_name, args):
        param_score += 0.1
    
    # Parameters within safe ranges
    if parameters_within_bounds(tool_name, args):
        param_score += 0.05
    
    # No dangerous patterns detected
    if not dangerous_pattern_detected(tool_name, args):
        param_score += 0.05
    
    # 3. Tool risk baseline (0.2-1.0)
    tool = get_tool_schema(tool_name)
    risk_baseline = {
        "low": 0.9,
        "medium": 0.5,
        "high": 0.2
    }.get(tool["risk_level"], 0.3)
    
    # 4. Permissions available (0.0-0.2)
    permissions_score = 0.0
    required_perms = tool["permissions_required"]
    available_perms = get_user_permissions(user_id)
    
    if all(p in available_perms for p in required_perms):
        permissions_score = 0.2
    elif any(p in available_perms for p in required_perms):
        permissions_score = 0.1
    
    # 5. Tool chain safety (0.0-0.1)
    chain_score = 0.0
    if context.is_first_tool:
        chain_score = 0.05
    elif context.previous_tool_succeeded:
        chain_score = 0.05
    
    # Combine scores
    total_score = (
        history_score +
        param_score +
        risk_baseline +
        permissions_score +
        chain_score
    )
    
    # Cap at 1.0
    return min(total_score, 1.0)


def should_require_approval(score: float, risk_level: str) -> bool:
    """Determine if user approval is needed"""
    
    # LOW risk: require approval only if score < 0.3
    if risk_level == "low":
        return score < 0.3
    
    # MEDIUM risk: require approval if score < 0.6
    elif risk_level == "medium":
        return score < 0.6
    
    # HIGH risk: always require approval
    else:
        return True
```

### 5.2 Scoring Examples

```
1. read_file("src/main.rs") — User has read 50+ files
   - History: 0.3
   - Param: 0.2 (read-only)
   - Risk: 0.9 (LOW)
   - Perms: 0.2
   - Chain: 0.05
   Total: 1.75 → capped at 1.0
   Approval needed: NO

2. write_file("src/main.rs", code) — First time, overwriting existing
   - History: 0.0
   - Param: 0.1 (overwriting > 10KB)
   - Risk: 0.5 (MEDIUM)
   - Perms: 0.2
   - Chain: 0.05
   Total: 0.85
   Approval needed: NO (score > 0.6)

3. delete_file("/z/Projects/BonsaiWorkspace") — Unusual, high-risk pattern
   - History: 0.0 (never deleted directory)
   - Param: 0.0 (dangerous pattern detected)
   - Risk: 0.2 (HIGH)
   - Perms: 0.0 (no delete permission)
   - Chain: 0.0
   Total: 0.2
   Approval needed: YES (HIGH risk)

4. execute_shell("cargo build") — User has run builds 100+ times
   - History: 0.3
   - Param: 0.15 (safe command)
   - Risk: 0.2 (HIGH)
   - Perms: 0.2
   - Chain: 0.05
   Total: 0.9
   Approval needed: NO (history is strong)
```

---

## 6. Tool Composition & Dependency Tracking

Tools can be chained (e.g., read → analyze → write). The system must:
1. Detect circular dependencies
2. Detect unsafe patterns
3. Batch independent calls
4. Rollback atomically on failure

### 6.1 Composition Graph

```python
class ToolCompositionGraph:
    def __init__(self):
        self.nodes = {}  # tool_name -> ToolNode
        self.edges = {}  # (tool_a, tool_b) -> dependency_type
    
    def add_invocation(self, tool_name: str, args: dict, position: int):
        """Add a tool to the composition"""
        self.nodes[tool_name] = ToolNode(
            name=tool_name,
            args=args,
            position=position,
            depends_on=[],
            affects=[]
        )
    
    def detect_data_flow(self, tool_sequence: List[str]):
        """Detect data flow between consecutive tools"""
        for i in range(len(tool_sequence) - 1):
            curr = tool_sequence[i]
            next_tool = tool_sequence[i + 1]
            
            # Check if output of curr feeds into input of next_tool
            curr_outputs = get_tool_outputs(curr)
            next_inputs = get_tool_inputs(next_tool)
            
            if any(out in next_inputs for out in curr_outputs):
                self.edges[(curr, next_tool)] = "data_flow"
    
    def detect_cycles(self) -> List[List[str]]:
        """Find circular dependencies"""
        return tarjan_strongly_connected_components(self.edges)
    
    def validate_composition(self) -> Tuple[bool, List[str]]:
        """Validate tool composition is safe"""
        errors = []
        
        # Check for cycles
        cycles = self.detect_cycles()
        if cycles:
            errors.append(f"Circular dependency detected: {cycles}")
        
        # Check for unsafe patterns
        for edge, dep_type in self.edges.items():
            tool_a, tool_b = edge
            
            # write → read is suspicious (possible race condition)
            if dep_type == "data_flow" and is_write_tool(tool_a) and is_read_tool(tool_b):
                errors.append(f"Suspicious: {tool_a} writes, {tool_b} reads immediately")
            
            # delete → anything is dangerous
            if is_delete_tool(tool_a):
                errors.append(f"Dangerous: delete tool {tool_a} before {tool_b}")
        
        return len(errors) == 0, errors
    
    def batch_independent_calls(self) -> List[List[str]]:
        """Group independent tools for parallel execution"""
        batches = []
        executed = set()
        
        while len(executed) < len(self.nodes):
            batch = []
            for tool_name, node in self.nodes.items():
                if tool_name in executed:
                    continue
                
                # Check if all dependencies are satisfied
                deps_satisfied = all(
                    dep in executed for dep in node.depends_on
                )
                
                if deps_satisfied:
                    batch.append(tool_name)
            
            if not batch:
                break  # Deadlock (shouldn't happen if validated)
            
            batches.append(batch)
            executed.update(batch)
        
        return batches
```

---

## 7. Error Handling & Rollback Strategy

Tools must be atomic or rollback-safe.

### 7.1 Rollback Architecture

```python
class ToolInvocationTransaction:
    def __init__(self, tool_sequence: List[str]):
        self.sequence = tool_sequence
        self.snapshots = []  # Snapshots before each tool
        self.results = []    # Results after each tool
    
    async def execute_with_rollback(self):
        """Execute tool sequence with atomic rollback on failure"""
        
        try:
            for i, tool_name in enumerate(self.sequence):
                # Take snapshot before tool
                snapshot = take_universe_snapshot(f"before_{tool_name}")
                self.snapshots.append(snapshot)
                
                # Execute tool
                result = await invoke_tool(tool_name, **args)
                self.results.append(result)
                
                # Validate result
                if not validate_tool_result(tool_name, result):
                    raise ToolExecutionError(f"Invalid result from {tool_name}")
        
        except Exception as e:
            # Rollback: restore from last good snapshot
            if self.snapshots:
                last_snapshot = self.snapshots[-1]
                restore_universe_snapshot(last_snapshot)
            
            raise
    
    async def rollback(self):
        """Rollback to initial state"""
        if self.snapshots:
            initial_snapshot = self.snapshots[0]
            restore_universe_snapshot(initial_snapshot)
```

### 7.2 Error Categorization

```python
class ToolError(Exception):
    """Base class for tool errors"""
    pass

class ToolValidationError(ToolError):
    """Invalid parameters"""
    pass

class ToolPermissionError(ToolError):
    """User lacks required permission"""
    pass

class ToolExecutionError(ToolError):
    """Tool failed during execution"""
    pass

class ToolTimeoutError(ToolError):
    """Tool exceeded timeout"""
    pass

class ToolUnavailableError(ToolError):
    """Tool mapping not implemented"""
    pass


def map_error_to_copilot_response(error: ToolError) -> dict:
    """Convert Bonsai error to Copilot error response"""
    
    if isinstance(error, ToolValidationError):
        return {
            "error": "validation_error",
            "message": str(error),
            "code": 400
        }
    elif isinstance(error, ToolPermissionError):
        return {
            "error": "permission_denied",
            "message": str(error),
            "code": 403
        }
    elif isinstance(error, ToolExecutionError):
        return {
            "error": "execution_failed",
            "message": str(error),
            "code": 500
        }
    elif isinstance(error, ToolTimeoutError):
        return {
            "error": "timeout",
            "message": f"Tool execution exceeded timeout",
            "code": 504
        }
    elif isinstance(error, ToolUnavailableError):
        return {
            "error": "tool_unavailable",
            "message": f"Tool mapping not implemented",
            "code": 501
        }
    else:
        return {
            "error": "internal_error",
            "message": "Unknown error",
            "code": 500
        }
```

---

## 8. Telemetry & Monitoring Plan

Every tool invocation is logged to the Universe event ledger.

### 8.1 Metrics to Track

| Metric | Type | Purpose |
|---|---|---|
| `tool_invocation_count` | Counter | Which tools are most used? |
| `tool_success_rate` | Gauge | Which tools fail frequently? |
| `tool_approval_rate` | Gauge | Which tools need approval? |
| `tool_execution_latency` | Histogram | Performance by tool |
| `approval_decision_distribution` | Counter | User approval/denial pattern |
| `permission_denied_count` | Counter | Missing permissions |
| `tool_chain_success_rate` | Gauge | Multi-tool composition success |

### 8.2 Logging Schema

```rust
#[derive(Serialize)]
struct ToolInvocationEvent {
    // Identity
    id: String,  // UUID
    timestamp: u64,  // UNIX timestamp (ms)
    user_id: String,
    tool_name: String,
    
    // Execution
    parameters_hash: String,  // SHA256 hash (don't log raw params)
    execution_strategy: String,  // "direct" | "bridge" | "cloud"
    execution_duration_ms: u64,
    
    // Safety
    confidence_score: f32,
    approval_required: bool,
    approval_given: bool,
    permissions_held: Vec<String>,
    
    // Result
    status: String,  // "success" | "failure" | "denied" | "timeout"
    error_code: Option<String>,
    error_message: Option<String>,
    
    // Composition
    position_in_sequence: Option<usize>,
    total_sequence_length: Option<usize>,
    previous_tool_name: Option<String>,
    next_tool_name: Option<String>,
}

// Example event
let event = ToolInvocationEvent {
    id: "evt_abc123",
    timestamp: 1717225200000,
    user_id: "user_luci",
    tool_name: "write_file",
    parameters_hash: "sha256:3f4c...",
    execution_strategy: "direct",
    execution_duration_ms: 45,
    confidence_score: 0.85,
    approval_required: false,
    approval_given: false,
    permissions_held: vec!["file:write".into()],
    status: "success",
    error_code: None,
    error_message: None,
    position_in_sequence: Some(2),
    total_sequence_length: Some(5),
    previous_tool_name: Some("read_file"),
    next_tool_name: Some("git_commit"),
};
```

### 8.3 Dashboard Queries

```sql
-- Most used tools
SELECT tool_name, COUNT(*) as count
FROM tool_invocation_events
WHERE timestamp > now() - interval '7 days'
GROUP BY tool_name
ORDER BY count DESC;

-- Tools requiring approval
SELECT tool_name, SUM(approval_required::int) / COUNT(*) as approval_rate
FROM tool_invocation_events
WHERE timestamp > now() - interval '7 days'
GROUP BY tool_name
HAVING approval_rate > 0.5;

-- User approval patterns
SELECT user_id, approval_given, COUNT(*) as count
FROM tool_invocation_events
GROUP BY user_id, approval_given;

-- Tool failure rate
SELECT tool_name, SUM((status = 'failure')::int) / COUNT(*) as failure_rate
FROM tool_invocation_events
GROUP BY tool_name
HAVING failure_rate > 0.1;
```

---

## 9. Versioning & Compatibility Strategy

### 9.1 Tool Version Management

Each tool has a version to handle evolution:

```yaml
tool:
  name: "read_file"
  version: "2.0"
  deprecation_info:
    deprecated_since: "1.5"
    will_be_removed: "2.0"
    replacement: "read_file_v2"
```

### 9.2 Schema Versioning

When Copilot adds or changes tools:

1. **Registry Update** – Add new entry to tool schema registry YAML
2. **Backward Compatibility** – Old tool versions remain functional
3. **Migration Path** – Document how to migrate from old to new
4. **Feature Detection** – Copilot can query supported tools

```python
def query_supported_tools(version: str = "latest") -> List[str]:
    """Query which tools are available at a given version"""
    registry = load_registry()
    
    compatible = [
        tool["name"]
        for tool in registry["tools"]
        if parse_version(tool["version"]) <= parse_version(version)
    ]
    
    return compatible
```

### 9.3 Deprecation Timeline

```
2026-06-01  Old tool removed from active use
2026-12-01  Old tool disabled (returns deprecation error)
2027-06-01  Old tool deleted from registry
```

---

## 10. Bridge Tool Implementation Framework

For tools without direct Bonsai equivalents, implement a bridge:

### 10.1 Bridge Template

```python
class ToolBridge:
    """Base class for bridging Copilot tools to Bonsai"""
    
    def __init__(self, tool_name: str, registry: ToolSchemaRegistry):
        self.tool_name = tool_name
        self.registry = registry
        self.tool_schema = registry.get_tool(tool_name)
    
    async def translate_and_execute(self, copilot_args: dict) -> dict:
        """Main entry point"""
        
        # 1. Validate input
        self.validate_input(copilot_args)
        
        # 2. Check approval
        if self.requires_approval():
            approval = await request_user_approval(
                tool=self.tool_name,
                args=copilot_args
            )
            if not approval:
                raise ToolPermissionError(f"User denied: {self.tool_name}")
        
        # 3. Check permissions
        self.check_permissions()
        
        # 4. Translate parameters
        bonsai_args = self.translate_parameters(copilot_args)
        
        # 5. Execute
        result = await self.execute(bonsai_args)
        
        # 6. Translate result
        copilot_result = self.translate_result(result)
        
        # 7. Log to Universe
        self.log_universe_event(copilot_result)
        
        return copilot_result
    
    def validate_input(self, args: dict):
        """Override to add custom validation"""
        pass
    
    def requires_approval(self) -> bool:
        """Override to customize approval logic"""
        return self.tool_schema["safety"]["approval_required"]
    
    def check_permissions(self):
        """Verify user has required permissions"""
        required = self.tool_schema["permissions_required"]
        available = get_user_permissions()
        
        missing = set(required) - set(available)
        if missing:
            raise ToolPermissionError(f"Missing permissions: {missing}")
    
    def translate_parameters(self, copilot_args: dict) -> dict:
        """Translate Copilot args to internal representation"""
        raise NotImplementedError
    
    async def execute(self, args: dict) -> dict:
        """Perform the actual operation"""
        raise NotImplementedError
    
    def translate_result(self, result: dict) -> dict:
        """Translate result back to Copilot format"""
        raise NotImplementedError
    
    def log_universe_event(self, result: dict):
        """Log invocation to Universe"""
        event = ToolInvocationEvent(
            tool_name=self.tool_name,
            status="success" if result.get("status") == "ok" else "failure",
            # ... other fields
        )
        universe.emitter.emit(event)
```

### 10.2 Example: list_files Bridge

```python
class ListFilesBridge(ToolBridge):
    def translate_parameters(self, copilot_args: dict) -> dict:
        return {
            "path": copilot_args.get("path", "."),
            "recursive": copilot_args.get("recursive", False),
            "pattern": copilot_args.get("pattern", "*"),
            "max_results": copilot_args.get("max_results", 1000)
        }
    
    async def execute(self, args: dict) -> dict:
        path = resolve_path(args["path"])
        recursive = args["recursive"]
        pattern = args["pattern"]
        max_results = args["max_results"]
        
        # Glob files
        glob_pattern = f"{path}/**/{pattern}" if recursive else f"{path}/{pattern}"
        files = glob.glob(glob_pattern, recursive=recursive)[:max_results]
        
        # Enrich with metadata
        results = []
        for f in files:
            results.append({
                "path": relative_to_workspace(f),
                "type": "file" if os.path.isfile(f) else "directory",
                "size": os.path.getsize(f) if os.path.isfile(f) else None,
                "mtime": os.path.getmtime(f)
            })
        
        return {"files": results}
    
    def translate_result(self, result: dict) -> dict:
        return {
            "files": result["files"],
            "count": len(result["files"])
        }
```

---

## 11. Design Principles & Constraints

### 11.1 Core Principles

1. **Completeness** – Every Copilot tool has a mapping strategy (direct, bridge, or cloud)
2. **Safety** – Dangerous tools require approval; read-only tools don't
3. **Transparency** – Every tool invocation is logged to Universe
4. **Extensibility** – Easy to add new tools without touching core
5. **Observability** – Metrics, events, and dashboards for monitoring
6. **Performance** – Batch independent operations; cache results
7. **Reliability** – Atomic execution with automatic rollback on failure

### 11.2 Constraints

- **No tool should block indefinitely** – All tools have timeouts
- **No tool should leak secrets** – Scan all I/O for sensitive data
- **No tool should escape the workspace** – Path validation is strict
- **No tool should modify without approval** – Write ops need user confirmation
- **No tool should run shell commands** – Only sandboxed execution
- **No tool should access network without filtering** – Allowlist/denylist + secret scanning

---

## 12. Implementation Roadmap

### Phase 1 (Week 1-2): Foundation
- [ ] Create tool schema registry YAML
- [ ] Implement ToolSchemaRegistry class
- [ ] Build parameter/result translation layer
- [ ] Implement confidence scoring

### Phase 2 (Week 3-4): Direct Tools (20 tools)
- [ ] Map file I/O tools
- [ ] Map VCS tools
- [ ] Map execution tools (tests, build)
- [ ] Add approval gates for HIGH risk

### Phase 3 (Week 5-6): Bridge Tools (15 tools)
- [ ] Implement KDB semantic search
- [ ] Implement code analysis bridges
- [ ] Implement network tool bridges
- [ ] Implement AI generation bridges

### Phase 4 (Week 7-8): Cloud Fallback (5 tools)
- [ ] Integrate GitHub API for PR operations
- [ ] Integrate Anthropic API for specialized generation
- [ ] Error handling and retry logic
- [ ] Secret scanning and filtering

### Phase 5 (Week 9-10): Testing & Monitoring
- [ ] End-to-end testing for each tool
- [ ] Telemetry integration
- [ ] Dashboard queries
- [ ] Performance benchmarking

### Phase 6 (Week 11-12): Documentation & Release
- [ ] User documentation
- [ ] Tool compatibility matrix
- [ ] Migration guide for extensions
- [ ] Release notes

---

## 13. Success Criteria

- [ ] 40+ Copilot tools mapped (100% coverage)
- [ ] 0 unhandled tool invocations (fallback to cloud if needed)
- [ ] < 100ms median tool invocation latency
- [ ] > 95% approval accuracy (minimize false positives)
- [ ] > 99% rollback success rate (atomic execution)
- [ ] Zero security incidents (no secrets leaked, no escapes)
- [ ] Complete telemetry (100% of invocations logged)
- [ ] User-facing documentation (all tools documented)

---

## Appendix: Glossary

- **Bridge** – Wrapper that maps a Copilot tool to a custom Bonsai implementation
- **Cloud Fallback** – Delegate tool execution to GitHub API or Anthropic API
- **Confidence Score** – 0.0-1.0 metric determining if approval is needed
- **Directly Mapped** – Bonsai MCP tool exists with compatible schema
- **KDB** – Bonsai's Knowledge Database (semantic search via embeddings)
- **MCP** – Model Context Protocol (standard for tool schemas)
- **Universe** – Bonsai's append-only event ledger for observability
- **Vault** – Secure storage for credentials and secrets

---

## Appendix: References

- [GitHub Copilot Features](https://docs.github.com/en/copilot/get-started/features)
- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Anthropic MCP Servers](https://github.com/modelcontextprotocol/servers)
- [Bonsai KDB Documentation](./06-KNOWLEDGE-DATABASE.md)
- [Bonsai MCP Server](./20-MCP-SERVER.md)

---

**Document Version:** 1.0  
**Status:** Ready for Implementation Review  
**Next Step:** Present to team for feedback and begin Phase 1
