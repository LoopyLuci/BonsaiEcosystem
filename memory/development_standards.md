---
name: development-standards
description: Development standards, guidelines, code quality expectations, and contribution procedures
metadata:
  type: feedback
---

# DEVELOPMENT STANDARDS & GUIDELINES

**Status**: ✅ COMPREHENSIVE STANDARDS DEFINED

---

## ✅ CODE QUALITY REQUIREMENTS

### Zero Unsafe Code
- ✅ **NEVER** use `unsafe {}` blocks
- ✅ Prefer safe abstractions (Arc, Mutex, DashMap)
- ✅ Use type system for safety
- ✅ All 2,413 crates must be 100% safe

**Why**: The entire system must be formally verifiable and safe-by-design for enterprise deployments.

### 100% Error Handling
- ✅ Every operation returns `Result<T>`
- ✅ Never use `.unwrap()` or `.expect()`
- ✅ Handle all error cases explicitly
- ✅ Custom error types for each module

**Why**: Enterprise systems cannot panic in production.

### Comprehensive Testing
- ✅ Minimum 4-7 tests per crate
- ✅ Unit tests for all public functions
- ✅ Integration tests for workflows
- ✅ 100% test pass rate required

**Why**: 7,715+ tests ensure reliability and prevent regressions.

---

## 🏗️ ARCHITECTURAL PATTERNS

### Module Interface Pattern
Every module implements `ModuleInterface`:
```rust
#[async_trait]
pub trait ModuleInterface: Send + Sync {
    fn id(&self) -> &str;
    fn version(&self) -> &str;
    async fn initialize(&mut self) -> Result<()>;
    async fn execute(&self, cmd: &str, args: &str) -> Result<String>;
    async fn shutdown(&mut self) -> Result<()>;
    fn status(&self) -> ModuleStatus;
}
```

**Requirement**: Every system/feature module MUST implement this trait.

### Lock-Free Concurrency
- ✅ Use `DashMap` instead of `Mutex<HashMap>`
- ✅ Use `Arc` for shared ownership
- ✅ Async/await with tokio
- ✅ No blocking operations in async code

**Why**: Lock-free design enables true concurrency without deadlocks.

### Result-Based Error Handling
```rust
pub fn operation(&self) -> Result<String, ModuleError> {
    // Never panic, always return Result
}
```

**Requirement**: Use `Result<T>` for all fallible operations.

### Structured Configuration
- ✅ Use `serde` for serialization
- ✅ YAML/JSON for config files
- ✅ Validate config on startup
- ✅ No hardcoded values

**Why**: Configuration must be external and mutable per environment.

---

## 📊 CODE STANDARDS

### Naming Conventions
- ✅ CamelCase for types, traits
- ✅ snake_case for variables, functions
- ✅ SCREAMING_SNAKE_CASE for constants
- ✅ Descriptive names (no abbreviations unless widely known)

### Code Organization
- ✅ `lib.rs` - Public API
- ✅ `error.rs` - Error types
- ✅ `types.rs` - Data structures
- ✅ `bin/cli.rs` - CLI tool
- ✅ `tests/` - Integration tests

### Documentation Standards
- ✅ Doc comments on all public items
- ✅ Example in documentation when non-obvious
- ✅ No "This function does X" comments (code speaks for itself)
- ✅ Comments explain WHY, not WHAT

### Commit Message Standards
```
feat: Add new feature

Detailed explanation of why this change was made
and what it accomplishes.

Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>
```

**Types**: feat, fix, docs, refactor, test, perf, chore

---

## 🔒 SECURITY STANDARDS

### Module Signing
- ✅ All production modules must be signed
- ✅ Use trusted signer certificates
- ✅ Verify signatures before loading
- ✅ Revoke compromised keys immediately

### RBAC Implementation
- ✅ Capability-based access control
- ✅ Principle of least privilege
- ✅ Audit log every permission check
- ✅ Regular permission audit

### Data Protection
- ✅ Encrypt sensitive data at rest
- ✅ Use TLS for all network communication
- ✅ No hardcoded secrets (use Vault)
- ✅ Secure random number generation

### Input Validation
- ✅ Validate ALL external input
- ✅ Use type system for validation
- ✅ Whitelist instead of blacklist
- ✅ Sanitize data before use

---

## 📈 PERFORMANCE STANDARDS

### Latency Targets
| Operation | Target | Required |
|-----------|--------|----------|
| Registry lookup | < 1µs | Yes |
| Module search | < 5ms | Yes |
| Module load | < 100ms | Yes |
| API response | < 100ms p99 | Yes |

### Throughput Targets
- ✅ 1M+ events/second for analytics
- ✅ 10,000+ concurrent connections
- ✅ 1,000+ req/sec per API endpoint
- ✅ Sub-100ms p99 latency

### Resource Limits
- ✅ Memory: < 256MB per module (default)
- ✅ CPU: No busy-wait, respect time slices
- ✅ Disk: Efficient I/O, batch operations
- ✅ Network: Compression, batch updates

---

## 🧪 TESTING STANDARDS

### Unit Tests
```rust
#[test]
fn test_operation_success() {
    // Happy path
}

#[test]
fn test_operation_failure() {
    // Error case
}

#[tokio::test]
async fn test_async_operation() {
    // Async test
}
```

### Integration Tests
- ✅ Test module-to-module interactions
- ✅ Test with real dependencies
- ✅ Test error scenarios
- ✅ Test recovery procedures

### Coverage Requirements
- ✅ All public functions tested
- ✅ All error paths tested
- ✅ Happy path and error cases
- ✅ Minimum 80% code coverage

---

## 📝 DOCUMENTATION STANDARDS

### Module Documentation
Each module MUST include:
- ✅ README.md - Overview and quick start
- ✅ API documentation - All public functions
- ✅ Architecture document - Design decisions
- ✅ Example code - How to use the module

### Code Examples
- ✅ Include error handling
- ✅ Follow best practices
- ✅ Test your examples
- ✅ Keep examples simple

### API Documentation
```rust
/// Performs operation on the module.
///
/// # Arguments
/// * `input` - The input value
///
/// # Returns
/// Returns a result with the output string
///
/// # Errors
/// Returns error if operation fails
///
/// # Examples
/// ```
/// let result = module.execute("command", "args")?;
/// assert_eq!(result, "expected");
/// ```
pub async fn execute(&self, cmd: &str, args: &str) -> Result<String>;
```

---

## 🔄 CONTRIBUTION WORKFLOW

### 1. Setup Development Environment
```bash
# Clone repository
git clone https://github.com/yourname/omnisystem.git
cd omnisystem

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify setup
cargo build --all
cargo test --all
```

### 2. Create Feature Branch
```bash
git checkout -b feature/your-feature-name

# Follow naming: feature/, fix/, docs/, refactor/, etc.
```

### 3. Develop & Test
```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all --lib

# Run tests
cargo test --all --release

# Build release
cargo build --release --all
```

### 4. Commit Changes
```bash
git add src/...
git commit -m "feat: Your feature description"

# Or for fixes, docs, refactors:
git commit -m "fix: Fix description"
git commit -m "docs: Documentation update"
git commit -m "refactor: Refactoring description"
```

### 5. Create Pull Request
```bash
git push origin feature/your-feature-name

# Create PR with:
# - Clear title
# - Description of changes
# - Testing performed
# - Related issues
```

### 6. Review & Merge
- ✅ All tests passing
- ✅ Code review approved
- ✅ No unsafe code
- ✅ Documentation updated

---

## ✅ DEPLOYMENT STANDARDS

### Pre-Deployment Verification
- ✅ All 7,715+ tests passing
- ✅ All 2,413 crates compile
- ✅ No unsafe code
- ✅ Security scan passed
- ✅ Compliance check passed
- ✅ Performance targets met
- ✅ Documentation updated
- ✅ Backup verified

### Release Process
1. ✅ Merge to main
2. ✅ Create release tag
3. ✅ Generate release notes
4. ✅ Build release artifacts
5. ✅ Deploy to staging
6. ✅ Run smoke tests
7. ✅ Deploy to production
8. ✅ Monitor metrics

### Rollback Procedures
- ✅ Automatic rollback on error detection
- ✅ Manual rollback via `kubectl rollout undo`
- ✅ Verified backup available
- ✅ Data integrity checked

---

## 📊 QUALITY METRICS

### Required Metrics
| Metric | Target | Method |
|--------|--------|--------|
| Test coverage | 100% core | `cargo tarpaulin` |
| Test pass rate | 100% | CI pipeline |
| Code compilation | 0 warnings | `cargo clippy` |
| Unsafe code | 0 blocks | `cargo forbid` |
| Latency p99 | < 100ms | Load testing |
| Memory | < 256MB | Profiling |
| Uptime | > 99.9% | Monitoring |

---

## 🎯 CODE REVIEW CHECKLIST

When reviewing code:
- ✅ No unsafe code?
- ✅ All errors handled?
- ✅ Tests included?
- ✅ Documentation updated?
- ✅ Performance acceptable?
- ✅ Security implications considered?
- ✅ Follows code standards?
- ✅ No hardcoded values?
- ✅ Backwards compatible?
- ✅ Commit message clear?

---

## 📚 RELATED DOCUMENTATION

- DOCS_CONTRIBUTING.md - Contribution guide
- BUILD_TO_PERFECTION_ROADMAP.md - Development timeline
- OMNISYSTEM_SECURITY_COMPLIANCE.md - Security standards
- DOCS_OMNISYSTEM_BUILD.md - Build procedures

---

## 💡 KEY PRINCIPLES

1. **Safety First** - Zero unsafe code, 100% error handling
2. **Test Everything** - 100% test coverage required
3. **Document Thoroughly** - Every module has docs
4. **Secure by Default** - Security standards apply to all code
5. **Performance Matters** - Meet latency targets
6. **Comply Always** - Follow compliance frameworks
7. **Quality Non-Negotiable** - Code review and standards required

**These standards apply to ALL 2,413 crates without exception.**
