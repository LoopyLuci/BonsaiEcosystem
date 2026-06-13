# Bug Hunter MCP Tools - Complete Usage Guide

**Status:** 🚀 Ready to Deploy  
**Date:** 2026-06-02  
**Integration:** MCP Protocol via mcp-server

---

## Quick Start

The Bug Hunter is now available as 7 MCP tools that Claude and other agents can call directly:

```
1. bonsai_scan_repo       - Scan repository for bugs
2. bonsai_list_findings   - List findings with filtering
3. bonsai_get_finding     - Get finding details
4. bonsai_auto_fix        - Apply automatic fixes
5. bonsai_explain_diagnostic - Get AI explanations
6. bonsai_prioritize_findings - Smart prioritization
7. bonsai_generate_report - Create comprehensive reports
```

---

## Tool Reference

### 1. bonsai_scan_repo

**Purpose:** Scan a repository for bugs, vulnerabilities, and code quality issues

**Input:**
```json
{
  "path": "string (required)",           // Repository path to scan
  "mode": "quick|full|ai",               // Scan mode (default: "full")
  "ai_review": boolean,                  // Enable AI analysis (default: true)
  "output_format": "json|markdown|sarif" // Output format (default: "json")
}
```

**Output:**
```json
{
  "scan_id": "string",           // Unique scan identifier
  "status": "completed|running", // Scan status
  "summary": {
    "total_issues": number,
    "critical": number,
    "high": number,
    "medium": number,
    "low": number
  },
  "findings": [
    {
      "finding_id": "string",
      "file": "string",
      "line": number,
      "severity": "critical|high|medium|low",
      "category": "string",
      "message": "string",
      "fixable": boolean
    }
  ],
  "duration_ms": number,
  "timestamp": "ISO8601"
}
```

**Example Use:**
```
User: "Scan the BonsaiWorkspace repository for critical vulnerabilities"

Claude calls:
  bonsai_scan_repo(
    path="/home/user/Projects/BonsaiWorkspace",
    mode="full",
    ai_review=true
  )

Returns scan_id: "scan-2024-123456"
```

---

### 2. bonsai_list_findings

**Purpose:** List findings from a scan, optionally filtered by severity

**Input:**
```json
{
  "scan_id": "string (required)", // From bonsai_scan_repo response
  "severity": "critical|high|medium|low|info", // Optional filter
  "limit": number                 // Max results (default: 50)
}
```

**Output:**
```json
{
  "scan_id": "string",
  "total": number,
  "findings": [
    {
      "finding_id": "string",
      "file": "string",
      "line": number,
      "column": number,
      "severity": "string",
      "title": "string",
      "category": "string",
      "fixable": boolean,
      "confidence": 0.95  // 0.0-1.0
    }
  ]
}
```

**Example Use:**
```
Claude calls:
  bonsai_list_findings(
    scan_id="scan-2024-123456",
    severity="critical"
  )

Returns only critical findings
```

---

### 3. bonsai_get_finding

**Purpose:** Get detailed information about a specific finding

**Input:**
```json
{
  "finding_id": "string (required)"  // Finding ID from list_findings
}
```

**Output:**
```json
{
  "finding_id": "string",
  "file": "string",
  "line": number,
  "column": number,
  "severity": "string",
  "category": "string",
  "title": "string",
  "description": "string",
  "context": {
    "before": "string",   // Code before issue
    "highlight": "string", // Problem code
    "after": "string"      // Code after issue
  },
  "fix": {
    "available": boolean,
    "fix_id": "string",
    "description": "string",
    "preview": "string"
  },
  "related_findings": ["string"]
}
```

**Example Use:**
```
Claude calls:
  bonsai_get_finding(
    finding_id="find-sql-injection-789"
  )

Returns full details including suggested fix
```

---

### 4. bonsai_auto_fix

**Purpose:** Apply automatic fix for a finding

**Input:**
```json
{
  "finding_id": "string (required)",
  "confirm": boolean  // Confirm before applying (default: true)
}
```

**Output:**
```json
{
  "status": "applied|failed|needs_confirmation",
  "finding_id": "string",
  "file": "string",
  "before": "string",      // Original code
  "after": "string",       // Fixed code
  "changes": {
    "lines_changed": number,
    "additions": number,
    "deletions": number
  },
  "timestamp": "ISO8601"
}
```

**Example Use:**
```
Claude calls:
  bonsai_auto_fix(
    finding_id="find-unused-import-123",
    confirm=true
  )

Returns confirmation that fix was applied
```

---

### 5. bonsai_explain_diagnostic

**Purpose:** Get AI explanation of a finding

**Input:**
```json
{
  "finding_id": "string (required)"  // Finding ID
}
```

**Output:**
```json
{
  "finding_id": "string",
  "title": "string",
  "why_it_matters": [
    "string"  // Multiple reasons it's important
  ],
  "example_bad": "string",
  "example_good": "string",
  "severity": "string",
  "how_to_fix": "string",
  "related_links": ["string"],
  "cwe_ids": ["string"]  // CWE vulnerability IDs
}
```

**Example Use:**
```
Claude calls:
  bonsai_explain_diagnostic(
    finding_id="find-sql-injection-789"
  )

Returns detailed explanation of SQL injection risk
```

---

### 6. bonsai_prioritize_findings

**Purpose:** Prioritize findings by impact or effort

**Input:**
```json
{
  "scan_id": "string (required)",
  "strategy": "impact|effort|security|maintainability"
}
```

**Output:**
```json
{
  "scan_id": "string",
  "strategy": "string",
  "prioritized": [
    {
      "rank": 1,
      "finding_id": "string",
      "score": 0.95,
      "reason": "string",
      "file": "string",
      "severity": "string"
    }
  ]
}
```

**Example Use:**
```
Claude calls:
  bonsai_prioritize_findings(
    scan_id="scan-2024-123456",
    strategy="security"
  )

Returns findings ordered by security impact
```

---

### 7. bonsai_generate_report

**Purpose:** Generate a comprehensive scan report

**Input:**
```json
{
  "scan_id": "string (required)",
  "format": "json|markdown|html|pdf"
}
```

**Output:**
```json
{
  "report_id": "string",
  "scan_id": "string",
  "format": "string",
  "file_path": "string",
  "content": "string",
  "summary": {
    "total_issues": number,
    "by_severity": {
      "critical": number,
      "high": number,
      "medium": number,
      "low": number
    },
    "by_category": {
      "category_name": number
    }
  },
  "timestamp": "ISO8601"
}
```

**Example Use:**
```
Claude calls:
  bonsai_generate_report(
    scan_id="scan-2024-123456",
    format="markdown"
  )

Returns markdown report ready for sharing
```

---

## Workflow Examples

### Example 1: Full Security Audit

```
User: "Run a complete security scan of the repository and show me 
       all critical issues with potential fixes."

Claude workflow:

1. Call bonsai_scan_repo(path="...", mode="full", ai_review=true)
   → Gets scan_id

2. Call bonsai_list_findings(scan_id=..., severity="critical")
   → Gets critical findings

3. For each critical finding:
   a. Call bonsai_get_finding(finding_id=...)
   b. Call bonsai_explain_diagnostic(finding_id=...)
   c. If fixable: call bonsai_auto_fix(finding_id=..., confirm=true)

4. Call bonsai_generate_report(scan_id=..., format="markdown")
   → Generates comprehensive report

Result: Security audit complete with explanations and fixes applied
```

### Example 2: Focus on High-Priority Issues

```
User: "What are the most critical bugs in my codebase and can 
       you fix the easy ones?"

Claude workflow:

1. Call bonsai_scan_repo(path="...", mode="quick")
   → Quick scan for speed

2. Call bonsai_prioritize_findings(scan_id=..., strategy="security")
   → Get issues ordered by security impact

3. For top 5 findings:
   - Call bonsai_get_finding(finding_id=...)
   - Check if fixable and confidence is high
   - If auto-fixable: call bonsai_auto_fix(...)

4. Present summary to user

Result: High-priority issues identified and easy ones fixed
```

### Example 3: Understanding a Specific Issue

```
User: "Explain this vulnerability in detail" (pointing to a finding)

Claude workflow:

1. Call bonsai_get_finding(finding_id="...")
   → Get full finding details and context

2. Call bonsai_explain_diagnostic(finding_id="...")
   → Get detailed explanation

3. Present both to user with code examples

Result: User understands the vulnerability and how to fix it
```

---

## Integration Points

The Bug Hunter is integrated at these points in mcp-server:

1. **MCP Protocol Handler** (`server.rs`):
   - Receives `tools/call` requests for Bug Hunter tools
   - Routes through UACS for approval
   - Calls `bridge.rs` with tool name and args

2. **Tool Bridge** (`bridge.rs`):
   - Checks if tool is registered in McpToolRegistry
   - If registered: calls execute_tool() for async handling
   - If not: falls back to run_devkit_tool() for CLI tools

3. **Tool Registry** (`tool_registry.rs`):
   - Maintains list of all 15 tools (7 Bug Hunter + 8 Linter)
   - Provides JSON schemas for tool discovery
   - Dispatches to appropriate handlers

4. **Bug Hunt Handlers** (`bug_hunt_tools.rs`):
   - Implements all 7 tool handlers
   - Validates inputs
   - Returns structured JSON responses

---

## Configuration

### MCP Server Setup

Start the MCP server:
```bash
cd Z:\Projects\BonsaiWorkspace
cargo build --package mcp-server --release
cargo run --package mcp-server --release
```

### Claude Configuration

In Claude's MCP settings, add:
```json
{
  "name": "mcp-server",
  "command": "cargo",
  "args": ["run", "--package", "mcp-server", "--release"],
  "env": {
    "BONSAI_DAEMON_URL": "http://127.0.0.1:8080/api"
  }
}
```

### Environment Variables

- `BONSAI_DAEMON_URL` - Bonsai daemon API endpoint (default: http://127.0.0.1:8080/api)
- `RUST_LOG` - Logging level (default: info)

---

## Error Handling

All tools return structured errors:

```json
{
  "error": "string",           // Error message
  "error_code": "string",      // Machine-readable code
  "details": "string"          // Additional context
}
```

**Common Errors:**

| Error | Cause | Solution |
|-------|-------|----------|
| `invalid_path` | Repository path doesn't exist | Provide valid path |
| `scan_not_found` | scan_id is invalid | Run scan_repo first |
| `finding_not_found` | finding_id doesn't exist | Use list_findings |
| `fix_failed` | Auto-fix couldn't apply | Manual review needed |
| `timeout` | Scan took too long | Use quick mode or smaller path |

---

## Performance Tips

1. **Use quick mode for initial assessment:**
   ```json
   {
     "mode": "quick"
   }
   ```

2. **Filter findings to reduce processing:**
   ```json
   {
     "severity": "critical,high"
   }
   ```

3. **Limit results:**
   ```json
   {
     "limit": 10
   }
   ```

4. **Cache scan results:**
   - Reuse scan_id for multiple queries
   - Don't re-scan the same repo frequently

---

## Limitations & Future Enhancements

**Current:**
- ✅ Full repository scanning
- ✅ 7 core Bug Hunter tools
- ✅ Auto-fix for common issues
- ✅ AI-powered explanations
- ✅ Report generation

**Roadmap:**
- 🔮 Incremental scanning (only changed files)
- 🔮 Custom rule definition
- 🔮 Integration with CI/CD
- 🔮 GitHub issue creation
- 🔮 Trend analysis over time
- 🔮 Team collaboration features

---

## Troubleshooting

### MCP Server Not Starting

1. Check Rust installation: `rustc --version`
2. Verify cargo: `cargo --version`
3. Check workspace builds: `cargo build --workspace --release`

### Tool Call Fails

1. Verify tool name matches exactly (case-sensitive)
2. Check JSON schema compliance
3. Ensure all required fields are provided
4. Check logs: `RUST_LOG=debug`

### Scan Times Out

1. Use `mode: "quick"` instead of `mode: "full"`
2. Scan smaller directory
3. Check system resources (disk, memory, CPU)

---

## Support & Feedback

For issues or feature requests:
- Check [MCP_TOOLS_INTEGRATION.md](MCP_TOOLS_INTEGRATION.md)
- Review [FINAL_MCP_STATUS.md](FINAL_MCP_STATUS.md)
- Consult [docs/](docs/) for architecture details

---

**Status: ✅ Production Ready**

All 7 Bug Hunter tools are fully implemented, tested, and ready for use.
