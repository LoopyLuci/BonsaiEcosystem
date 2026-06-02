# Bug Hunter MCP Test Scenario

**Objective:** Demonstrate Bug Hunter functionality through the MCP server  
**Status:** Ready to Execute  
**Date:** 2026-06-02

---

## Test Setup

### Prerequisites

1. ✅ MCP server running on port 3000
2. ✅ Claude connected to MCP server
3. ✅ Repository to scan: `/Projects/BonsaiWorkspace`

### Starting the MCP Server

```bash
cd Z:\Projects\BonsaiWorkspace
cargo run --package bonsai-mcp-server --release
```

Expected output:
```
Starting MCP server on http://127.0.0.1:3000
Registering tools:
  ✓ bonsai_scan_repo
  ✓ bonsai_list_findings
  ✓ bonsai_get_finding
  ✓ bonsai_auto_fix
  ✓ bonsai_explain_diagnostic
  ✓ bonsai_prioritize_findings
  ✓ bonsai_generate_report
  ... and 8 Linter tools
MCP server ready
```

---

## Test Scenario 1: Quick Security Scan

### Objective
Perform a quick security scan of the repository to identify high-severity vulnerabilities.

### Command to Claude

```
Perform a quick security scan of Z:\Projects\BonsaiWorkspace 
and show me all high and critical severity findings.
```

### Expected Flow

1. **Scan Initiation**
   ```
   Claude calls: bonsai_scan_repo({
     path: "Z:\Projects\BonsaiWorkspace",
     mode: "quick",
     ai_review: true
   })
   
   Response: {
     scan_id: "scan-2024-060201",
     status: "completed",
     summary: {
       total_issues: 45,
       critical: 3,
       high: 12,
       medium: 18,
       low: 12
     },
     duration_ms: 2500
   }
   ```

2. **Filter Critical Findings**
   ```
   Claude calls: bonsai_list_findings({
     scan_id: "scan-2024-060201",
     severity: "critical,high",
     limit: 10
   })
   
   Response: [
     {
       finding_id: "find-sql-injection-001",
       file: "src-daemon/src/rpc.rs",
       line: 157,
       severity: "critical",
       title: "Potential SQL Injection",
       category: "security"
     },
     ...
   ]
   ```

3. **Get Detailed Information**
   ```
   Claude calls: bonsai_get_finding({
     finding_id: "find-sql-injection-001"
   })
   
   Response: {
     finding_id: "find-sql-injection-001",
     file: "src-daemon/src/rpc.rs",
     line: 157,
     title: "Potential SQL Injection",
     description: "User input is directly interpolated...",
     context: {
       before: "let query = \"SELECT * FROM users WHERE id=\"",
       highlight: "\" + user_id +",
       after: "\";\"" 
     },
     fix: {
       available: true,
       description: "Use parameterized query",
       preview: "SELECT * FROM users WHERE id = ?"
     }
   }
   ```

4. **Present Results**
   ```
   ✓ Found 3 critical vulnerabilities:
   
   1. [CRITICAL] SQL Injection in src-daemon/src/rpc.rs:157
      Risk: User input directly in SQL query
      Fix: Available (Parameterize query)
   
   2. [CRITICAL] Unvalidated File Path in ...
   
   3. [CRITICAL] Hardcoded API Key in ...
   ```

### Success Criteria

- ✅ Scan completes in < 3 seconds (quick mode)
- ✅ All critical findings are identified
- ✅ Each finding has severity, location, and fix info
- ✅ Claude can explain the security implications

---

## Test Scenario 2: Auto-Fix Demonstration

### Objective
Identify fixable issues and automatically apply corrections.

### Command to Claude

```
Find all issues in Z:\Projects\BonsaiWorkspace that can be 
automatically fixed, and apply the fixes.
```

### Expected Flow

1. **Scan Repository**
   ```
   bonsai_scan_repo({
     path: "Z:\Projects\BonsaiWorkspace",
     mode: "full"
   })
   → scan_id: "scan-2024-060202"
   ```

2. **List All Findings**
   ```
   bonsai_list_findings({
     scan_id: "scan-2024-060202"
   })
   → [45 findings total]
   ```

3. **Check Each Finding for Fixes**
   ```
   For each finding:
     bonsai_get_finding(finding_id)
     
     If fix.available == true:
       bonsai_auto_fix({
         finding_id: "...",
         confirm: true
       })
   ```

4. **Generate Summary**
   ```
   ✓ Auto-fixed 8 issues:
   
   - 5 unused imports (src/lib.rs, bridge.rs, etc.)
   - 2 unused variables (tool_registry.rs)
   - 1 incorrect type annotation (bug_hunt_tools.rs)
   
   Remaining manual fixes: 37
   ```

### Success Criteria

- ✅ Identifies all auto-fixable issues
- ✅ Applies fixes without errors
- ✅ Reports before/after code
- ✅ Summary shows all changes

---

## Test Scenario 3: Detailed Vulnerability Explanation

### Objective
Get detailed explanations of security findings.

### Command to Claude

```
Explain the top 3 security vulnerabilities found in the 
BonsaiWorkspace repository in detail.
```

### Expected Flow

1. **Scan and Prioritize**
   ```
   bonsai_scan_repo(...)
   → scan_id: "scan-2024-060203"
   
   bonsai_prioritize_findings({
     scan_id: "scan-2024-060203",
     strategy: "security"
   })
   → Ranked by security impact
   ```

2. **Get Explanations**
   ```
   For top 3 findings:
     bonsai_explain_diagnostic({
       finding_id: "..."
     })
   ```

3. **Present Detailed Analysis**
   ```
   ## 1. SQL Injection in RPC Handler
   
   **Why It Matters:**
   - Attackers can execute arbitrary SQL commands
   - Can extract sensitive data (passwords, tokens)
   - Can modify/delete database records
   - Complete system compromise possible
   
   **Example Bad Code:**
   ```rust
   let query = format!("SELECT * FROM users WHERE id = {}", user_id);
   ```
   
   **Example Good Code:**
   ```rust
   sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
     .bind(user_id)
     .fetch_one(&pool)
     .await
   ```
   
   **CWE IDs:** CWE-89 (SQL Injection)
   **CVSS Score:** 9.8 (Critical)
   ```

### Success Criteria

- ✅ Explanations include "why it matters"
- ✅ Code examples show bad vs good patterns
- ✅ CWE and severity information provided
- ✅ Clear remediation guidance given

---

## Test Scenario 4: Report Generation

### Objective
Generate a comprehensive report of findings.

### Command to Claude

```
Generate a detailed report of all issues found in the 
BonsaiWorkspace repository in markdown format.
```

### Expected Flow

1. **Scan Repository**
   ```
   bonsai_scan_repo({
     path: "Z:\Projects\BonsaiWorkspace",
     mode: "full"
   })
   ```

2. **Generate Report**
   ```
   bonsai_generate_report({
     scan_id: "scan-2024-060204",
     format: "markdown"
   })
   
   Returns: report.md file with:
   - Executive summary
   - Findings by severity
   - Detailed issue breakdown
   - Remediation recommendations
   - Statistics and trends
   ```

3. **Present Report**
   ```
   # BonsaiWorkspace Security Audit Report
   
   **Generated:** 2026-06-02 14:35:20 UTC
   **Repository:** Z:\Projects\BonsaiWorkspace
   **Scan Mode:** Full
   **Duration:** 4.2 seconds
   
   ## Executive Summary
   
   Total Issues Found: 45
   - Critical: 3
   - High: 12
   - Medium: 18
   - Low: 12
   
   Risk Assessment: HIGH
   Immediate Action Required: YES
   
   ## Issues by Severity
   
   ### Critical (3)
   1. SQL Injection in src-daemon/src/rpc.rs:157
   2. Unvalidated File Path in bonsai-workspace/...
   3. Hardcoded Credentials in src-tauri/config.rs:42
   
   ### High (12)
   ...
   
   ## Recommendations
   
   1. Fix all critical vulnerabilities immediately
   2. Address high-severity items within 1 week
   3. Schedule medium issues for next sprint
   
   [Full detailed report...]
   ```

### Success Criteria

- ✅ Report includes all key sections
- ✅ Proper formatting and structure
- ✅ Clear severity breakdown
- ✅ Actionable recommendations
- ✅ Statistics and metrics

---

## Test Scenario 5: Continuous Integration Integration

### Objective
Show how Bug Hunter can be used in CI/CD pipeline.

### Mock CI Command

```bash
# In CI/CD pipeline
cargo run --package bonsai-mcp-server -- \
  --mode ci \
  --repo Z:\Projects\BonsaiWorkspace \
  --fail-on-critical \
  --output reports/security-scan.json
```

### Expected Behavior

```
Security Scan Report
====================
Scanning: Z:\Projects\BonsaiWorkspace
Mode: CI
Started: 2026-06-02 14:40:00

[████░░░░░] 40% Complete
[████████░] 80% Complete
[██████████] 100% Complete

Results:
--------
Total Issues: 45
Critical: 3  ← CI will FAIL
High: 12
Medium: 18
Low: 12

Status: FAILED (Critical vulnerabilities found)
Report: reports/security-scan.json

Exit Code: 1

Note: Fix critical vulnerabilities before merging.
```

---

## Troubleshooting Guide

### Issue: "Tool not found: bonsai_scan_repo"

**Cause:** MCP server not running or tools not registered  
**Solution:**
```bash
# Restart MCP server
cargo run --package bonsai-mcp-server --release

# Verify tools are listed
curl http://127.0.0.1:3000/tools
```

### Issue: "Scan timed out"

**Cause:** Repository too large or system resources low  
**Solution:**
```
Use quick mode instead:
bonsai_scan_repo({
  mode: "quick"  // Skip detailed analysis
})

Or scan specific directory:
bonsai_scan_repo({
  path: "Z:\Projects\BonsaiWorkspace\crates\bonsai-mcp-server"
})
```

### Issue: "Auto-fix failed: permission denied"

**Cause:** File permissions prevent modification  
**Solution:**
```
# Ensure write permissions
chmod 644 <file>

# Or manually apply the fix shown in bonsai_get_finding
```

### Issue: "Finding not found: find-abc123"

**Cause:** finding_id is invalid or expired  
**Solution:**
```
# Re-run scan to get fresh findings
bonsai_scan_repo(...)
→ New scan_id

# List findings again
bonsai_list_findings(scan_id=...)
```

---

## Performance Metrics

| Operation | Quick Mode | Full Mode | Notes |
|-----------|-----------|-----------|-------|
| Scan Repo | 1-3s | 5-10s | Depends on repo size |
| List Findings | <100ms | <500ms | With 100 findings |
| Get Finding | <50ms | <50ms | Single lookup |
| Auto-Fix | <200ms | <200ms | Depends on file size |
| Generate Report | 1-2s | 2-5s | Rendering time |

---

## Next Steps

After successful testing:

1. **Integrate with CI/CD:**
   - Add to GitHub Actions
   - Add to pre-commit hooks
   - Add to deployment pipeline

2. **Team Rollout:**
   - Share usage guide with team
   - Set up security baseline
   - Define remediation SLAs

3. **Monitoring:**
   - Track issue trends
   - Monitor fix rate
   - Alert on critical findings

4. **Continuous Improvement:**
   - Tune detection rules
   - Reduce false positives
   - Add custom rules

---

**Test Status:** ✅ Ready to Execute

All scenarios are designed to test complete Bug Hunter workflows.
Start with Scenario 1 and progress through each one.
