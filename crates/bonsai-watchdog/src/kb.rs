/// Knowledge base — SQLite-backed store for issue→fix mappings.
///
/// Every entry contains:
/// - one or more symptom patterns (substrings / regexes matched against logs)
/// - a repair script (shell command or structured action)
/// - metadata: confidence, usage, who created it

use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixEntry {
    pub id:              i64,
    pub error_pattern:   String,
    pub solution_type:   String,  // "rule" | "ai" | "user"
    pub solution_script: String,
    pub confidence:      f64,
    pub usage_count:     i64,
    pub success_count:   i64,
    pub created_by:      String,  // "bonsai" | "user" | "agent"
    pub verified:        bool,
}

/// Every entry is (error_pattern_substring, solution_description).
/// Loaded at first startup so the KB works even on a fresh install.
pub const SEEDED_FIXES: &[(&str, &str)] = &[
    ("Type cast 'as' is not valid in Svelte 4 template attribute",
     "Move the TypeScript 'as' cast out of the template expression into a $: reactive variable in the <script> block."),
    ("Unexpected token TypeScript cast in Svelte template",
     "Svelte 4 does not support TypeScript casts inside template expressions. Move the cast to a helper function or $: reactive declaration."),
    ("onMount return type Promise is not assignable to void",
     "Cast onMount: (onMount as (fn: () => Promise<() => void>) => void)(async () => { ... return cleanup; })"),
    ("e.target as HTMLInputElement is not valid in Svelte template",
     "Use e.currentTarget instead of (e.target as HTMLInputElement) inside Svelte template event handlers."),
    ("Property 'test' does not exist on type 'UserConfigExport'",
     "Move Vitest config to a separate vitest.config.ts with '/// <reference types=\"vitest\" />' at the top."),
    ("invoke returns unknown not assignable to any[]",
     "Use invoke<any[]>('command_name') with an explicit type parameter."),
    ("No overload matches createEventDispatcher",
     "Add all dispatched event names to createEventDispatcher generic: createEventDispatcher<{ close: void; myEvent: void }>()"),
    ("Property content_type does not exist on type string tool_result",
     "Widen tool_result type to 'string | { content_type: string; data?: number[] } | null' and narrow via a $: reactive variable."),
    ("Property extended_thinking does not exist on type ModelDataSummary",
     "Add 'extended_thinking?: boolean;' to ModelDataSummary interface in src/lib/types/model_data.ts."),
    ("Type string not assignable to Tab",
     "Define typed constant array in script: const TABS: Tab[] = [...] and a wrapper: function setTabFromString(s: string) { setTab(s as Tab); }"),
    ("Property dataset does not exist on type Element",
     "Cast to HTMLElement: (el as HTMLElement).dataset. dataset exists on HTMLElement but not base Element."),
    ("description[..Math.min] Python-style slice",
     "Replace Python-style slice 'str[..n]' with JavaScript 'str.slice(0, n)'."),
    ("tauri_plugin_notification NotificationExt not in scope",
     "Add 'use tauri_plugin_notification::NotificationExt;' at the top of the Rust file."),
    ("gguf safetensors bin model files committed to git",
     "Add *.gguf, *.safetensors, *.bin, *.jsonl, memory_nodes.db*, survival.db* to .gitignore."),
    ("HF_HUB_OFFLINE TRANSFORMERS_OFFLINE not set in Python training script",
     "Set os.environ['TRANSFORMERS_OFFLINE']='1'; os.environ['HF_HUB_OFFLINE']='1' at the top of all training scripts. Pass local_files_only=True to all from_pretrained() calls."),
    ("survival run_script executes arbitrary shell string command injection",
     "Ensure the safety gate in ai_repair_error() validates script content. Prefer allow-listed commands over free-form shell strings for automated repair."),
    ("renderer.code type mismatch marked Renderer",
     "Cast the renderer: (renderer as any).code = ... The newer marked versions changed the code renderer signature."),
    ("JSDoc cast Expected } parse error in each block",
     "JSDoc comments in Svelte 4 {#each} block expressions cause parse errors. Define a typed constant array in script instead."),
    ("cannot find name refreshStatus in catalog.ts",
     "Import refreshStatus from ./models: add 'refreshStatus' to the import statement from '$lib/stores/models'."),
    ("dpo_train.py STATUS_ACCESS_VIOLATION exit code -1073741819",
     "Windows ACCESS_VIOLATION in PyTorch DPO training is caused by default max_length=512 creating tensors too large for the CPU allocator. Fix: pass --max-length 128 (or 256) to dpo_train.py. The script now defaults to 256."),
    ("DPO training segfault exit 139 after data pairs loaded",
     "Segfault after '[data] pairs=N' in dpo_train.py on Windows/CPU is a tensor size issue. Two model copies × max_length=512 exceeds allocatable contiguous memory. Use --max-length 128 for CPU training."),
];

pub struct KnowledgeBase {
    conn: Connection,
}

impl KnowledgeBase {
    #[cfg(test)]
    pub fn conn(&self) -> &Connection { &self.conn }
}

impl KnowledgeBase {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            CREATE TABLE IF NOT EXISTS fixes (
                id             INTEGER PRIMARY KEY AUTOINCREMENT,
                error_pattern  TEXT    NOT NULL,
                solution_type  TEXT    NOT NULL DEFAULT 'rule',
                solution_script TEXT   NOT NULL,
                confidence     REAL    NOT NULL DEFAULT 0.5,
                usage_count    INTEGER NOT NULL DEFAULT 0,
                success_count  INTEGER NOT NULL DEFAULT 0,
                created_by     TEXT    NOT NULL DEFAULT 'system',
                verified       INTEGER NOT NULL DEFAULT 0,
                created_at     DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE INDEX IF NOT EXISTS idx_fixes_pattern ON fixes(error_pattern);
        ")?;
        Ok(Self { conn })
    }

    /// Find fixes whose error_pattern appears as a substring of `log`.
    pub fn find_matching(&self, log: &str) -> Vec<FixEntry> {
        let mut stmt = self.conn.prepare(
            "SELECT id, error_pattern, solution_type, solution_script, confidence,
                    usage_count, success_count, created_by, verified
             FROM fixes
             ORDER BY success_count DESC, confidence DESC"
        ).unwrap();
        stmt.query_map([], |row| {
            Ok(FixEntry {
                id:              row.get(0)?,
                error_pattern:   row.get(1)?,
                solution_type:   row.get(2)?,
                solution_script: row.get(3)?,
                confidence:      row.get(4)?,
                usage_count:     row.get(5)?,
                success_count:   row.get(6)?,
                created_by:      row.get(7)?,
                verified:        row.get::<_, i64>(8)? != 0,
            })
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .filter(|f| log.contains(&f.error_pattern))
        .collect()
    }

    /// Record a new fix (returns its row id).
    pub fn insert_fix(
        &self,
        pattern:  &str,
        stype:    &str,
        script:   &str,
        confidence: f64,
        created_by: &str,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO fixes (error_pattern, solution_type, solution_script, confidence, created_by)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![pattern, stype, script, confidence, created_by],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Increment usage counter; increment success counter if `success` is true.
    pub fn record_outcome(&self, id: i64, success: bool) -> Result<()> {
        if success {
            self.conn.execute(
                "UPDATE fixes SET usage_count = usage_count+1, success_count = success_count+1, verified = 1 WHERE id = ?",
                params![id],
            )?;
        } else {
            self.conn.execute(
                "UPDATE fixes SET usage_count = usage_count+1 WHERE id = ?",
                params![id],
            )?;
        }
        Ok(())
    }

    /// Seed the DB with all known patterns from the project's history.
    /// Idempotent — skips patterns that already exist.
    pub fn seed_defaults(&self) -> Result<()> {
        for (pattern, script) in SEEDED_FIXES {
            let exists: bool = self.conn.query_row(
                "SELECT COUNT(*) FROM fixes WHERE error_pattern = ?1",
                params![pattern],
                |row| row.get::<_, i64>(0),
            ).unwrap_or(0) > 0;
            if !exists {
                let _ = self.insert_fix(pattern, "rule", script, 0.95, "system");
            }
        }
        Ok(())
    }

    /// Export all entries as JSONL (for training data generation).
    pub fn export_jsonl(&self) -> Vec<serde_json::Value> {
        let mut stmt = self.conn.prepare(
            "SELECT error_pattern, solution_script FROM fixes WHERE success_count > 0"
        ).unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .map(|(pattern, script)| serde_json::json!({
            "messages": [
                {"role": "system", "content": "You are an expert system administrator. Given an error log from the Bonsai application, output a single shell command that fixes the problem. Output NOT_FIXABLE if you cannot determine a fix."},
                {"role": "user",   "content": pattern},
                {"role": "assistant", "content": script},
            ]
        }))
        .collect()
    }
}
