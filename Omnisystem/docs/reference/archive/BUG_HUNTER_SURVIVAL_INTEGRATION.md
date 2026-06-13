# Bug Hunter → Survival System → Knowledge Database Integration

**Status:** Implementation Ready (Build-Blocked by Workspace Dependencies)  
**Date:** 2026-06-02  
**Scope:** Complete workflow for finding, fixing, and learning from bugs

---

## Overview

The Bug Hunter will automatically scan the codebase, find issues, apply fixes, and feed the results into the Survival System and Knowledge Database for continuous learning.

```
┌──────────────────┐
│   Bug Hunter     │  1. Scan repository
│  (MCP Tools)     │     - Find vulnerabilities
└────────┬─────────┘     - Find code quality issues
         │
         ├─ Issue Details ──────────────────────────────┐
         │                                              │
         ├─ Generated Fixes ──────────────────────────┐ │
         │                                            │ │
         ▼                                            ▼ ▼
┌──────────────────────────────────────────────────────────────┐
│              Fix Validation & Application                    │
│  - Verify fix soundness                                      │
│  - Test fix doesn't break existing tests                     │
│  - Apply fix to codebase                                     │
└────────┬────────────────────────────────────────────────────┘
         │
         ├─ Fix Details ────────────────────────────┐
         │                                          │
         ▼                                          │
┌──────────────────────────────────────────┐      │
│   Survival System (survival.rs)           │      │
│  - stores("error_pattern" → "solution")  │◄─────┘
│  - tracks confidence & success rates     │
│  - learns from outcomes                  │
└────────┬─────────────────────────────────┘
         │
         ├─ Learned Rules ─────────────────────────┐
         │                                         │
         ▼                                         │
┌──────────────────────────────────────────┐      │
│  Knowledge Database (bonsai-kdb)         │      │
│  - Aggregates rules across projects      │◄─────┘
│  - Measures rule confidence              │
│  - Provides feedback for improvement     │
└──────────────────────────────────────────┘
```

---

## Workflow: Finding a Bug to Recording the Fix

### Step 1: Bug Discovery

**Tool:** `bonsai_scan_repo()`

```json
Request: {
  "path": "Z:\Projects\BonsaiWorkspace",
  "mode": "full",
  "ai_review": true
}

Response: {
  "scan_id": "scan-2024-060201",
  "findings": [
    {
      "finding_id": "find-sql-injection-001",
      "file": "src-daemon/src/rpc.rs",
      "line": 157,
      "severity": "critical",
      "title": "SQL Injection Vulnerability",
      "description": "User input directly interpolated into SQL query...",
      "fix": {
        "available": true,
        "description": "Use parameterized query",
        "preview": "?"
      }
    },
    // ... more findings
  ]
}
```

### Step 2: Fix Generation & Application

**Tool:** `bonsai_auto_fix()`

```json
Request: {
  "finding_id": "find-sql-injection-001",
  "confirm": true
}

Response: {
  "status": "applied",
  "file": "src-daemon/src/rpc.rs",
  "line": 157,
  "before": "let query = format!(\"SELECT * FROM users WHERE id = {}\", user_id);",
  "after": "sqlx::query_as::<_, User>(\"SELECT * FROM users WHERE id = ?\").bind(user_id).fetch_one(&pool).await",
  "changes": {
    "lines_changed": 1,
    "additions": 3,
    "deletions": 1
  }
}
```

### Step 3: Store in Survival System

**Function:** `save_fix_to_survival()`

```rust
pub async fn save_fix_to_survival(
    pool: &SqlitePool,
    finding: &BugFinding,
    fix_applied: &FixResult,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO fixes (
            error_pattern, 
            solution_type, 
            solution_script, 
            confidence, 
            created_by
        ) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&finding.title)  // error_pattern: "SQL Injection Vulnerability"
    .bind("fix")           // solution_type
    .bind(fix_applied.after)  // solution_script: The fixed code
    .bind(0.8)             // confidence: High (code change verified)
    .bind("bug-hunter")    // created_by
    .execute(pool)
    .await?;
    
    Ok(())
}
```

**What gets stored in survival.rs:**

```sql
INSERT INTO fixes VALUES (
    NULL,  -- id (auto)
    'SQL Injection Vulnerability',  -- error_pattern
    'fix',  -- solution_type
    'sqlx::query_as(...).bind(user_id)...',  -- solution_script
    0.8,  -- confidence
    1,    -- usage_count
    0,    -- success_count
    'bug-hunter',  -- created_by
    0,    -- verified
    CURRENT_TIMESTAMP  -- created_at
);
```

### Step 4: Record in Knowledge Database

**Function:** `record_fix_to_kdb()`

```rust
pub async fn record_fix_to_kdb(
    finding: &BugFinding,
    fix_applied: &FixResult,
) -> Result<()> {
    // Create structured entry for KDB
    let kdb_entry = KdbEntry {
        rule_id: format!("fix-{}", uuid::Uuid::new_v4()),
        issue_type: finding.category.clone(),
        issue_pattern: finding.title.clone(),
        solution: fix_applied.after.clone(),
        severity: finding.severity.clone(),
        language: detect_language(&finding.file),
        confidence: 0.8,
        verified: false,
        created_at: Utc::now(),
        source: "bug-hunter".to_string(),
        metadata: json!({
            "finding_id": finding.finding_id,
            "file": finding.file,
            "line": finding.line,
            "before": fix_applied.before,
            "after": fix_applied.after,
        }),
    };
    
    // Store in KDB with aggregation
    kdb.insert(kdb_entry).await?;
    
    Ok(())
}
```

### Step 5: Feedback Loop

**Survival System Updates:**

When the fix is later encountered in production:
- ✅ If it works → `success_count++`, `confidence += 0.05`
- ❌ If it fails → `confidence -= 0.1`, mark for review
- Every use → `usage_count++`

**Knowledge Database Updates:**

- Aggregates fixes across all repositories
- Measures rule confidence across projects
- Identifies most-effective fixes (by success rate)
- Provides feedback to improve rules

---

## Complete Integration Workflow

### Automated Scanning Process

```rust
pub async fn run_bug_hunter_scan_cycle(
    workspace_path: &str,
    survival_pool: &SqlitePool,
    kdb: &KnowledgeDatabase,
) -> Result<ScanReport> {
    
    // PHASE 1: Scan Repository
    println!("🔍 Scanning repository for bugs...");
    let scan_result = bh_registry.execute_tool("bonsai_scan_repo", json!({
        "path": workspace_path,
        "mode": "full",
        "ai_review": true
    })).await?;
    
    let scan_id = scan_result["scan_id"].as_str().unwrap();
    let findings = scan_result["findings"].as_array().unwrap();
    
    println!("✓ Found {} issues", findings.len());
    
    // PHASE 2: List High-Priority Findings
    println!("📊 Filtering critical issues...");
    let critical_findings = bh_registry.execute_tool("bonsai_list_findings", json!({
        "scan_id": scan_id,
        "severity": "critical,high"
    })).await?;
    
    let mut fixes_applied = 0;
    let mut fixes_failed = 0;
    
    // PHASE 3: Process Each Finding
    for finding_id in critical_findings.as_array().unwrap() {
        let finding_id = finding_id["finding_id"].as_str().unwrap();
        
        // Get Details
        let finding = bh_registry.execute_tool("bonsai_get_finding", json!({
            "finding_id": finding_id
        })).await?;
        
        // Get Explanation
        let explanation = bh_registry.execute_tool("bonsai_explain_diagnostic", json!({
            "finding_id": finding_id
        })).await?;
        
        // Check if auto-fixable
        if finding["fix"]["available"].as_bool().unwrap_or(false) {
            
            // Apply Fix
            println!("🔧 Applying fix for: {}", finding["title"]);
            let fix_result = bh_registry.execute_tool("bonsai_auto_fix", json!({
                "finding_id": finding_id,
                "confirm": true
            })).await?;
            
            if fix_result["status"].as_str().unwrap_or("") == "applied" {
                // PHASE 4: Store in Survival System
                save_fix_to_survival(survival_pool, &finding, &fix_result).await?;
                
                // PHASE 5: Record in Knowledge Database
                record_fix_to_kdb(&finding, &fix_result).await?;
                
                fixes_applied += 1;
                println!("  ✓ Fix applied and recorded");
                
            } else {
                fixes_failed += 1;
                println!("  ✗ Fix failed to apply");
            }
        } else {
            println!("⚠️  Requires manual review: {}", finding["title"]);
        }
    }
    
    // PHASE 6: Generate Report
    println!("📈 Generating report...");
    let report = bh_registry.execute_tool("bonsai_generate_report", json!({
        "scan_id": scan_id,
        "format": "markdown"
    })).await?;
    
    Ok(ScanReport {
        scan_id: scan_id.to_string(),
        total_issues: findings.len(),
        critical_issues: critical_findings.as_array().unwrap().len(),
        fixes_applied,
        fixes_failed,
        fixes_reviewed: findings.len() - fixes_applied - fixes_failed,
        report_md: report["content"].as_str().unwrap().to_string(),
    })
}
```

---

## Data Structures

### Bug Finding (from Bug Hunter)

```rust
pub struct BugFinding {
    pub finding_id: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub severity: Severity,  // critical, high, medium, low
    pub category: String,     // sql-injection, xss, etc.
    pub title: String,        // Human readable
    pub description: String,
    pub fix: FixSuggestion,
}

pub struct FixSuggestion {
    pub available: bool,
    pub fix_id: String,
    pub description: String,
    pub preview: String,
}
```

### Survival System Entry

```sql
CREATE TABLE fixes (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    error_pattern   TEXT    NOT NULL,           -- What to recognize
    solution_type   TEXT    NOT NULL,           -- 'rule', 'fix', 'patch'
    solution_script TEXT    NOT NULL,           -- The actual fix/rule
    confidence      REAL    NOT NULL,           -- 0.0-1.0, improves over time
    usage_count     INTEGER NOT NULL DEFAULT 0, -- How many times tried
    success_count   INTEGER NOT NULL DEFAULT 0, -- How many times worked
    created_by      TEXT    NOT NULL,           -- 'bug-hunter', 'user', etc.
    verified        INTEGER NOT NULL DEFAULT 0, -- 1 if human verified
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### Knowledge Database Entry

```rust
pub struct KdbEntry {
    pub rule_id: String,           // Unique identifier
    pub issue_type: String,         // Category
    pub issue_pattern: String,      // What issue this fixes
    pub solution: String,           // The fix code
    pub severity: String,           // critical, high, etc.
    pub language: String,           // rust, python, js, etc.
    pub confidence: f32,            // 0.0-1.0
    pub verified: bool,             // Human review status
    pub created_at: DateTime,
    pub source: String,             // Where it came from
    pub metadata: serde_json::Value,// Extra info (before/after, etc.)
}
```

---

## Integration Points

### 1. Bug Hunter → Survival System

**Location:** `bonsai-workspace/src-tauri/src/bug_hunter_integration.rs` (NEW)

```rust
pub async fn integrate_bug_findings_to_survival(
    finding: &BugFinding,
    fix_applied: &FixResult,
    survival_pool: &SqlitePool,
) -> Result<()> {
    // Convert finding format to survival format
    let error_pattern = format!(
        "{}: {}",
        finding.category,
        finding.title
    );
    
    // Insert into fixes table
    sqlx::query(
        "INSERT INTO fixes (error_pattern, solution_type, solution_script, 
                            confidence, created_by) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(error_pattern)
    .bind("fix")
    .bind(&fix_applied.after)
    .bind(0.8)  // Initial high confidence
    .bind("bug-hunter")
    .execute(survival_pool)
    .await?;
    
    Ok(())
}
```

### 2. Survival System → Knowledge Database

**Location:** `crates/bonsai-kdb/src/survival_sync.rs` (NEW)

```rust
pub async fn sync_survival_to_kdb(
    survival_pool: &SqlitePool,
    kdb: &KnowledgeDatabase,
) -> Result<()> {
    // Get recent fixes from survival
    let fixes = sqlx::query_as::<_, SurvivalFix>(
        "SELECT * FROM fixes WHERE created_at > ? ORDER BY created_at DESC"
    )
    .bind(Utc::now() - Duration::hours(1))
    .fetch_all(survival_pool)
    .await?;
    
    // Convert to KDB entries
    for fix in fixes {
        let kdb_entry = KdbEntry {
            rule_id: format!("survival-{}", uuid::Uuid::new_v4()),
            issue_type: extract_issue_type(&fix.error_pattern),
            issue_pattern: fix.error_pattern.clone(),
            solution: fix.solution_script.clone(),
            confidence: fix.confidence,
            source: fix.created_by.clone(),
            // ... rest of fields
        };
        
        kdb.insert(kdb_entry).await?;
    }
    
    Ok(())
}
```

### 3. Feedback Loop

**Location:** `crates/bonsai-kdb/src/feedback_loop.rs` (NEW)

```rust
pub async fn update_confidence_from_outcomes(
    survival_pool: &SqlitePool,
    kdb: &KnowledgeDatabase,
) -> Result<()> {
    // For each fix in survival:
    // - success_count / usage_count = success_rate
    // - Update KDB confidence based on success_rate
    
    let fixes = sqlx::query_as::<_, SurvivalFix>(
        "SELECT * FROM fixes"
    )
    .fetch_all(survival_pool)
    .await?;
    
    for fix in fixes {
        let success_rate = fix.success_count as f32 / fix.usage_count.max(1) as f32;
        let new_confidence = success_rate * 0.95 + fix.confidence * 0.05;
        
        kdb.update_confidence(&fix.id, new_confidence).await?;
    }
    
    Ok(())
}
```

---

## Execution Plan

### Phase 1: Setup (Once workspace builds)

```bash
# 1. Build MCP server
cargo build --package mcp-server --release

# 2. Start MCP server
./target/release/mcp-server &

# 3. Verify tools available
curl http://127.0.0.1:3000/tools
```

### Phase 2: Initial Scan

```bash
# Run Bug Hunter scan
cargo run --package bonsai-workspace -- \
  --command scan-with-bug-hunter \
  --path . \
  --save-to-survival \
  --save-to-kdb
```

### Phase 3: Automated Cycles

```bash
# Schedule periodic scans
# Every 6 hours: Run full scan
# Daily: Sync survival → KDB
# Weekly: Generate trends report
```

---

## Expected Results

### After First Run

```
Bug Hunter Scan Report
====================

Total Issues Found: 47
├─ Critical: 3
├─ High: 12
├─ Medium: 18
└─ Low: 14

Fixes Applied: 23
├─ Auto-fixed: 20
├─ Requires Review: 3
└─ Failed: 1

Survival System Updated:
├─ New rules: 20
├─ Updated confidence: 3
└─ Total rules: 127

Knowledge Database Updated:
├─ New entries: 20
├─ Aggregated confidence: 0.82
└─ Most effective fixes: sql-injection (0.95), unused-import (0.91)
```

---

## Monitoring & Metrics

### Survival System Health

```sql
SELECT 
    COUNT(*) as total_fixes,
    SUM(usage_count) as total_uses,
    SUM(success_count) as total_successes,
    AVG(confidence) as avg_confidence
FROM fixes
WHERE created_by = 'bug-hunter'
    AND created_at > datetime('now', '-7 days');
```

**Expected Output:**
```
total_fixes: 47
total_uses: 156
total_successes: 138
avg_confidence: 0.81
```

### Knowledge Database Trends

```
Most-Used Fixes:
1. SQL Injection Prevention (used: 34x, success: 92%)
2. Unused Import Removal (used: 28x, success: 98%)
3. Unhandled Error Wrapping (used: 18x, success: 78%)

Lowest-Confidence Rules:
1. Complex Business Logic (confidence: 0.45)
2. Async-Await Ordering (confidence: 0.52)
3. Database Transaction Handling (confidence: 0.61)
```

---

## Status: ✅ READY FOR IMPLEMENTATION

All integration points are designed and documented. Once workspace builds:

1. ✅ Copy integration modules
2. ✅ Wire into Tauri commands
3. ✅ Start automated cycle
4. ✅ Monitor results

**Estimated Setup Time:** 30 minutes  
**Estimated First Scan:** 5-10 minutes  
**Estimated Time to First Learning Cycle:** < 1 hour

