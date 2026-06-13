# Remaining "bonsai-" References - Comprehensive Review

**Generated:** 2026-06-06  
**Total References:** 90+  
**Status:** Detailed categorization for user review  

---

## ⚠️ CRITICAL: Crate Path References That Need Fixing

### bonsai-workspace/src-tauri/Cargo.toml (29 crate paths)
These are **BROKEN** - they reference old crate paths that no longer exist:

```toml
actors = { path = "../../crates/bonsai-actors" }              → ../../crates/actors
android-bridge = { path = "../../crates/bonsai-android-bridge" } → ../../crates/android-bridge
capability-registry = { path = "../../crates/bonsai-capability-registry" } → ../../crates/capability-registry
cas = { path = "../../crates/bonsai-cas" }                  → ../../crates/cas
chess = { path = "../../crates/bonsai-chess" }              → ../../crates/chess
coordinator = { path = "../../crates/bonsai-coordinator" }  → ../../crates/coordinator
crdt = { path = "../../crates/bonsai-crdt" }                → ../../crates/crdt
credits = { path = "../../crates/bonsai-credits" }          → ../../crates/credits
extensions = { path = "../../crates/bonsai-extensions" }    → ../../crates/extensions
fabric = { path = "../../crates/bonsai-fabric" }  (appears twice) → ../../crates/fabric
failure-finder = { path = "../../crates/bonsai-failure-finder" } → ../../crates/failure-finder
go = { path = "../../crates/bonsai-go" }                    → ../../crates/go
go-nn = { path = "../../crates/bonsai-go-nn" }              → ../../crates/go-nn
hnsw = { path = "../../crates/bonsai-hnsw" }                → ../../crates/hnsw
ir = { path = "../../crates/bonsai-ir" }                    → ../../crates/ir
kdb = { path = "../../crates/bonsai-kdb" }                  → ../../crates/kdb
knowledge = { path = "../../crates/bonsai-knowledge" }      → ../../crates/knowledge
mailbox = { path = "../../crates/bonsai-mailbox" }          → ../../crates/mailbox
marketplace = { path = "../../crates/bonsai-marketplace" }  → ../../crates/marketplace
package = { path = "../../crates/bonsai-package" }          → ../../crates/package
query = { path = "../../crates/bonsai-query" }              → ../../crates/query
skill-compiler = { path = "../../crates/bonsai-skill-compiler" } → ../../crates/skill-compiler
sns = { path = "../../crates/bonsai-sns" }                  → ../../crates/sns
swarm = { path = "../../crates/bonsai-swarm" }              → ../../crates/swarm
transfer-store = { path = "../../crates/bonsai-transfer-store" } → ../../crates/transfer-store
verify = { path = "../../crates/bonsai-verify" }            → ../../crates/verify
```

**ACTION REQUIRED:** Fix all 29 paths in `bonsai-workspace/src-tauri/Cargo.toml`

---

## 📁 User-Facing Identifiers (Safe to Keep or Update Later)

### Binary & Executable Names
- `bonsai-workspace.exe` / `bonsai-workspace` - Main app binary
- `bonsai-music-worker` / `bonsai-music-worker.exe` - Sidecar process
- `bonsai-cli` - Command-line tool reference

### Configuration Files & Directories
- `.bonsai/` - User config directory
- `bonsai-config.json` - Main config file
- `bonsai-bot-port.json` - Bot port configuration
- `bonsai-plugin.toml` - Plugin manifest format
- `bonsai-latest.gguf` - Model file naming

### Keyring/Service Names (System-level)
- `bonsai-bot` - Keyring service (BOT_KEYRING_SERVICE)
- `bonsai-assistant` - Keyring service (SERVICE)
- `x-bonsai-token` - HTTP header name

### API & User-Facing Identifiers
- `"bonsai-workspace"` - API service name / app identifier
- `"bonsai-buddy"` - AI model identifier
- `"bonsai-buddy-backup-{ts}.zip"` - Backup file naming
- `"bonsai-bot bonsai_ext envelope"` - Protocol reference
- `"bonsai-latest"` - Default model alias

### Model & Adapter Names
- `bonsai-latest` - Default model reference
- `bonsai-core-v3` - Adapter naming convention
- `bonsai-core-vX` - Adapter variants
- `bonsai-small-v1` - Model variant
- `bonsai-chat-v2` - Model variant
- `bonsai-1.7b` - Model variant
- `bonsai-cross` - Cross-training adapter
- `bonsai-self-play` - Self-play adapter
- `bonsai-music-worker` - Music synthesis service name

### Directory/Path References
- `~/.bonsai/models/bonsai-latest.gguf` - Model cache path
- `~/.bonsai/adapters/bonsai-*` - Adapter cache paths
- `.bonsai/rules/` - Rules directory
- `bonsai-workspace` (npm prefix) - Build directory

### Comments & Documentation
- `/// a structured confirm request (e.g. bonsai-bot bonsai_ext envelope)`
- `// ── Structured confirmation response (bonsai-bot protocol) ───`
- `//! Inline synthesizer (mirrors bonsai-music-worker)`
- `//! References bonsai-auth key ring`
- `//! Plugin manifest types — parsed from bonsai-plugin.toml`
- `/// Read a persisted bonsai-bot port file`
- `const BOT_KEYRING_SERVICE: &str = "bonsai-bot";`
- Various code comments and documentation

### Repository/Copyright References
- `repository = "https://github.com/bonsai/bonsai-bedf"` - Repository URL
- `"For the Tauri app: cd bonsai-workspace && cargo tauri build"` - Build instructions

---

## 📊 Summary by Type

| Category | Count | Action |
|----------|-------|--------|
| **Crate paths (BROKEN)** | 29 | ⚠️ **FIX REQUIRED** |
| Binary/executable names | 3 | Optional cosmetic |
| Config files/directories | 6 | Optional cosmetic |
| Keyring services | 2 | Optional (system-level) |
| API identifiers | 5 | Optional cosmetic |
| Model/adapter names | 8 | Optional cosmetic |
| Directory paths | 5 | Optional cosmetic |
| Comments/docs | 15+ | Optional (non-functional) |
| Repository URLs | 1 | Optional (metadata) |

---

## ✅ Recommendation

### Priority 1: FIX IMMEDIATELY
- **bonsai-workspace/src-tauri/Cargo.toml** - All 29 crate paths are broken and will cause compilation failures

### Priority 2: Update When Convenient (Non-Breaking)
- Binary names, config files, directory names, model names - These won't break compilation but should be updated for consistency
- Examples: `bonsai-workspace` → `workspace`, `bonsai-buddy` → `buddy` (if desired)

### Priority 3: Informational Only
- Comments and documentation that reference old names
- Repository URLs and metadata

---

## 🔧 Quick Fix Commands

To fix all 29 paths in bonsai-workspace/src-tauri/Cargo.toml:

```bash
sed -i 's|crates/bonsai-|crates/|g' bonsai-workspace/src-tauri/Cargo.toml
```

This will convert all references from `../../crates/bonsai-X` to `../../crates/X`

---

**Next Steps:**
1. Review this list with the team
2. Fix Priority 1 (critical)
3. Schedule Priority 2 updates for cosmetic consistency
4. Consider Priority 3 optional or deferred

