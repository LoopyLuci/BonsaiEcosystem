# 🔌 MCP Tools Integration Guide - Complete Setup

**Status:** Ready for immediate integration  
**Date:** 2026-06-02  
**Purpose:** Enable Claude to use all Bonsai Bug Hunter and Linter tools via MCP protocol

---

## 📋 What Has Been Created

### Three New Tool Handler Files
1. **`crates/mcp-server/src/bug_hunt_tools.rs`** (280 LOC)
   - `handle_scan_repo()` – Full/incremental/AI-powered scanning
   - `handle_list_findings()` – Filter findings by severity
   - `handle_get_finding()` – Detailed finding information
   - `handle_auto_fix()` – Apply automatic fixes
   - `handle_explain_diagnostic()` – AI explanations
   - `handle_prioritize_findings()` – Smart prioritization
   - `handle_generate_report()` – Comprehensive reports

2. **`crates/mcp-server/src/lint_tools.rs`** (240 LOC)
   - `handle_lint_file()` – Lint single files
   - `handle_lint_repo()` – Lint entire repository
   - `handle_generate_lint_rule()` – AI rule generation
   - `handle_explain_diagnostic()` – Rule explanations
   - `handle_apply_fix()` – Apply fixes
   - `handle_dismiss_diagnostic()` – Mark false positives
   - `handle_report_false_positive()` – Improve confidence

3. **`crates/mcp-server/src/tool_registry.rs`** (350 LOC)
   - Complete tool registry with 15+ tools
   - Tool definitions with JSON schemas
   - Tool execution dispatcher
   - Integration point for MCP protocol

---

## 🚀 Integration Steps

### Step 1: Update MCP Server Main File

Edit `crates/mcp-server/src/lib.rs` (or `main.rs`) and add:

```rust
// Add these module declarations at the top
pub mod bug_hunt_tools;
pub mod lint_tools;
pub mod tool_registry;

// Import the registry
use tool_registry::McpToolRegistry;

// In your MCP server initialization:
let tool_registry = McpToolRegistry::new();
```

### Step 2: Wire Tools into MCP Protocol Handler

In your MCP request handler (typically in `src/server.rs` or `src/main.rs`), add:

```rust
// When handling tools/list request:
match request.method {
    "tools/list" => {
        let tools = tool_registry.list_tools();
        let response = json!({
            "tools": tools.iter().map(|t| json!({
                "name": t.name,
                "description": t.description,
                "inputSchema": t.input_schema
            })).collect::<Vec<_>>()
        });
        send_response(response);
    },
    "tools/call" => {
        let tool_name = request.params["name"].as_str().unwrap();
        let arguments = &request.params["arguments"];
        
        match tool_registry.execute_tool(tool_name, arguments.clone()).await {
            Ok(result) => send_response(json!({
                "status": "success",
                "result": result
            })),
            Err(e) => send_error(&format!("Tool execution failed: {}", e))
        }
    },
    _ => {}
}
```

### Step 3: Build and Test

```bash
cd Z:\Projects\BonsaiWorkspace

# Build the MCP server
cargo build --package mcp-server --release

# Run the MCP server
cargo run --package mcp-server --release
```

### Step 4: Verify Tools Available

Once the server is running, verify tools are available:

```bash
# List all registered tools
curl http://localhost:3000/tools

# Should return something like:
# {
#   "tools": [
#     {"name": "bonsai_scan_repo", ...},
#     {"name": "bonsai_list_findings", ...},
#     ... (15+ tools total)
#   ]
# }
```

---

## 📊 Available Tools (15 Total)

### Bug Hunter Tools (7)
| Tool | Purpose |
|------|---------|
| `bonsai_scan_repo` | Scan repository for bugs |
| `bonsai_list_findings` | List findings by severity |
| `bonsai_get_finding` | Get finding details |
| `bonsai_auto_fix` | Apply automatic fix |
| `bonsai_explain_diagnostic` | AI explanation |
| `bonsai_prioritize_findings` | Smart prioritization |
| `bonsai_generate_report` | Generate reports |

### Linter Tools (8)
| Tool | Purpose |
|------|---------|
| `bonsai_lint_file` | Lint single file |
| `bonsai_lint_repo` | Lint repository |
| `bonsai_generate_lint_rule` | Generate rule from description |
| `bonsai_explain_diagnostic` | Explain lint rule |
| `bonsai_apply_fix` | Apply fix from diagnostic |
| `bonsai_dismiss_diagnostic` | Mark false positive |
| `bonsai_report_false_positive` | Improve rule confidence |

---

## 💬 How to Use (Examples)

### Example 1: Full Vulnerability Scan

**Ask Claude:**
```
"Scan the BonsaiWorkspace repository for all critical and high-severity 
vulnerabilities. Show me the findings and auto-fix the ones you can."
```

**Claude automatically:**
1. Calls `bonsai_scan_repo(path="Z:\Projects\BonsaiWorkspace", mode="full", ai_review=true)`
2. Gets `scan_id`
3. Calls `bonsai_list_findings(scan_id=..., severity="critical,high")`
4. For each finding, calls `bonsai_get_finding(finding_id=...)`
5. Calls `bonsai_auto_fix(finding_id=..., confirm=true)` for fixable findings
6. Calls `bonsai_explain_diagnostic(finding_id=...)` for others

**Claude shows you:**
```
✓ Found 12 critical/high issues
✓ Auto-fixed 7 issues
⚠ 5 issues require manual review

[Details and explanations...]
```

### Example 2: Lint Repository

**Ask Claude:**
```
"Lint the entire BonsaiWorkspace repository and show me a summary."
```

**Claude calls:**
- `bonsai_lint_repo(quick=false)`

**Claude shows:**
```
✓ Scanned 256 files
✓ Found 87 issues
  - 45 warnings
  - 23 errors
  - 19 hints

Top violations:
  1. unused-import (23 occurrences)
  2. unread-variable (15 occurrences)
  3. missing-docstring (12 occurrences)
```

### Example 3: Generate New Rule

**Ask Claude:**
```
"Generate a lint rule that detects potential null pointer dereferences in Rust."
```

**Claude calls:**
- `bonsai_generate_lint_rule(description="Detect null pointer dereferences", language="rust")`

**Claude shows:**
```
Created new rule: custom-rule-abc123
Name: Null Pointer Detection
Language: Rust
Confidence: 0.75
Status: Pending Review
```

---

## 🔧 Architecture

```
Claude Agent
    ↓
MCP Client
    ↓
MCP Server (mcp-server)
    ├─ Tool Registry
    │  ├─ Bug Hunt Tools Handler
    │  │  └─ [7 tool functions]
    │  ├─ Lint Tools Handler
    │  │  └─ [8 tool functions]
    │  └─ Tool Definitions (JSON schemas)
    ↓
Bonsai Ecosystem
    ├─ bonsai-bug-hunt
    ├─ bonsai-lint
    └─ audit-log
```

---

## ✅ Verification Checklist

After integration:

- [ ] Code compiles: `cargo build --package mcp-server`
- [ ] MCP server starts: `cargo run --package mcp-server`
- [ ] Tools listed: `curl http://localhost:3000/tools`
- [ ] Claude can discover MCP server
- [ ] Can call `bonsai_scan_repo` successfully
- [ ] Can call `bonsai_lint_repo` successfully
- [ ] Can chain multiple tools together

---

## 📚 File Locations

- **Bug Hunt Tools:** `crates/mcp-server/src/bug_hunt_tools.rs`
- **Lint Tools:** `crates/mcp-server/src/lint_tools.rs`
- **Registry:** `crates/mcp-server/src/tool_registry.rs`
- **Setup Guide:** `mcp/SETUP_MCP_SERVER.md`
- **Agent Integration:** `mcp/AGENT_INTEGRATION.md`
- **This Guide:** `MCP_TOOLS_INTEGRATION.md`

---

## 🎯 Next Steps

1. ✅ Integrate the three new files into mcp-server
2. ✅ Update lib.rs/main.rs with module declarations
3. ✅ Wire tool registry into MCP protocol handler
4. ✅ Build and test
5. ✅ Verify tools are available
6. ✅ Ask Claude to use tools

---

**Status: Ready for integration** ✅

All code provided. Ready to be added to mcp-server crate.

