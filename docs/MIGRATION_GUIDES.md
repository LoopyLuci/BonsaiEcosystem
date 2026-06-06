# Migration Guides – Upgrading Between Major Versions

This document provides step-by-step instructions for upgrading to new major versions of the Bonsai Ecosystem.

---

## v0.1.x → v0.2.0 (Expected Q3 2026)

### Overview
v0.2.0 will introduce:
- Full `ai-advisor` crate implementation
- BACE (Atomic Compilation Engine) integration
- Sanctuary TEE support for secure execution
- Optional Nexus Core blockchain

### Breaking Changes
None expected. v0.2.0 is fully backward-compatible with v0.1.x.

### Migration Steps

1. **Update Dependencies**
   ```toml
   # Cargo.toml
   ai-advisor = "0.2"  # New version with full SovereignService impl
   p2p-core = "0.2"  # Minor updates
   ```

2. **Enable New Features** (Optional)
   ```bash
   cargo build --release --features="ai-enhancements,nexus-core,tee-sanctum"
   ```

3. **No Code Changes Required**
   - Existing SovereignService implementations work as-is
   - New optional features are additive
   - Existing APIs unchanged

4. **Performance Improvements**
   - BACE reduces rebuild time by ~50%
   - No configuration needed; automatic
   - Measured via `cargo build --release`

5. **Testing**
   ```bash
   cargo test --workspace --all-features
   # Expected: All tests pass, new features tested
   ```

### Rollback (if needed)
```bash
cargo update --precise "ai-advisor:0.1.0"
cargo build --release
```

---

## v0.2.x → v1.0.0 (Expected Q4 2026)

### Overview
v1.0.0 is the production-ready release:
- 750+ languages fully validated
- TransferDaemon v2 P2P mesh stable
- All formal verification proofs complete
- Private name sanitization verified

### Breaking Changes
**None expected**. v1.0.0 is a stability release, not a feature release.

### Migration Steps

1. **Upgrade to v1.0.0**
   ```bash
   cargo update bonsai-*  # Updates all Bonsai crates to ^1.0
   cargo build --release --all-features
   ```

2. **Verify All Features**
   ```bash
   cargo test --workspace --all-features -- --nocapture
   ```

3. **Run Integration Tests**
   ```bash
   # Polyglot Pong full matrix (750 languages)
   cargo run --release --manifest-path polyglot-pong/Cargo.toml -- \
     --manifest languages.yaml --nodes 8 --rounds 100
   ```

4. **Deployment**
   - v1.0.0 is production-ready
   - See [DEPLOYMENT.md](DEPLOYMENT.md) for platform-specific instructions
   - No special migration needed from v0.2.x

---

## v1.x → v2.0.0 (Estimated 2027)

### Overview
v2.0.0 is a major architecture change:
- Full UOSC operating system integration
- 50+ external dependencies replaced with bonsai-* crates
- Supply-chain integrity guarantees
- Advanced features: ZK-STARK proofs, advanced fuzzing, chaos testing

### Expected Breaking Changes

1. **Module Reorganization**
   ```rust
   // OLD (v1.x)
   use bonsai_transfer::Daemon;
   use bonsai_compression::BuceClient;

   // NEW (v2.0)
   use bonsai_Uosc::transfer::Daemon;
   use bonsai_Uosc::compression::BuceClient;
   ```

2. **Dependency Removals**
   - External crates removed (tokio, serde, etc.)
   - Replaced with bonsai-* equivalents
   - Config migration guide will be provided

3. **API Changes**
   - SovereignService trait remains, but new methods may be added
   - Existing implementations will need minor updates

### Pre-Migration Checklist

Before upgrading to v2.0.0:

- [ ] Back up your configuration files
- [ ] Review this migration guide fully
- [ ] Test in a staging environment first
- [ ] Have rollback plan ready (see below)

### Migration Steps

1. **Prepare for Major Changes**
   ```bash
   # Freeze your current version
   cargo update --freeze  # Locks dependencies
   git commit -m "Pre-v2.0.0 dependency freeze"
   ```

2. **Create Upgrade Branch**
   ```bash
   git checkout -b upgrade/v2.0.0
   ```

3. **Update Cargo.toml**
   - Replace all external crate dependencies with bonsai-* equivalents
   - See migration manifest: `docs/v2.0.0_dependency_map.md` (to be published)

   ```toml
   # OLD
   [dependencies]
   tokio = { version = "1", features = ["rt-multi-thread"] }
   serde = { version = "1", features = ["derive"] }

   # NEW
   bonsai-runtime = { version = "2.0", features = ["async"] }
   bonsai-serialization = "2.0"
   ```

4. **Update Code**
   ```bash
   # Use migration script (to be provided)
   cargo run --bin bonsai-migrate-v1-to-v2 -- --path .
   ```

   Manual changes:
   ```rust
   // Existing SovereignService implementations need minor updates
   // (Usually just re-exporting from new module paths)
   ```

5. **Run Tests**
   ```bash
   cargo test --workspace --all-features
   # Fix any failures using the v2.0.0 troubleshooting guide
   ```

6. **Integration Testing**
   ```bash
   cargo run --release --manifest-path polyglot-pong/Cargo.toml -- \
     --manifest languages.yaml --nodes 8 --rounds 50
   ```

7. **Deploy**
   - Test in staging first
   - Gradual rollout (10% → 50% → 100%)
   - Monitor for issues (see [OBSERVABILITY.md](OBSERVABILITY.md))

### Rollback (if needed)

If v2.0.0 doesn't work for you:

```bash
# Keep the v2.0.0 branch for reference
git stash

# Revert to v1.x
git checkout main
cargo update --lock   # Restore original versions

# Deploy v1.x
cargo build --release
```

**Note**: v2.0.0 database migrations may not be reversible. Test in staging before production.

### Extended Support for v1.x

v1.x will receive security updates until **2028-01-01**. After that, v2.0.0 is required for updates.

---

## General Guidelines for All Migrations

### 1. Testing
- Always test in a staging environment first
- Run full test suite: `cargo test --workspace --all-features`
- Test your specific use case with real data

### 2. Backups
- Back up your data, configuration, and identity files
- Keep a copy of old Bonsai binaries until confirmed v2.0.0 works

### 3. Communication
- Join the discussion for your migration version in GitHub Discussions
- Report bugs with details: OS, Rust version, exact steps to reproduce

### 4. Rollback Plan
- Always have a plan to revert to the previous version
- Document the rollback procedure for your specific setup
- Test the rollback procedure before deploying

### 5. Gradual Adoption
- Don't deploy to all systems at once
- Test with a subset first (10% of users/nodes)
- Monitor for issues before full deployment
- Use feature flags to enable new features incrementally

---

## Frequently Asked Questions

**Q: Will my v1.0.0 code work in v2.0.0?**  
A: Mostly, with minor changes. See the specific breaking changes for your version transition above.

**Q: How long will v1.x be supported?**  
A: Until 2028-01-01. After that, only v2.0.0+ will receive security updates.

**Q: Can I stay on v0.1.0 forever?**  
A: Technically yes, but not recommended. You'll miss security patches and improvements. Upgrade to v1.0.0 (production-ready, stable) at minimum.

**Q: What if my code breaks during migration?**  
A: Post in GitHub Discussions with the error. The maintainers will help (best-effort for last version). File issues to help improve the migration guide.

**Q: Should I wait for v2.0.0 to deploy Bonsai?**  
A: No. v1.0.0 is fully production-ready. Migrate to v2.0.0 when it's released and you're ready.

---

## Related Documents

- [CHANGELOG.md](CHANGELOG.md) – Full release history
- [DEPLOYMENT.md](DEPLOYMENT.md) – Deployment to production
- [TROUBLESHOOTING.md](TROUBLESHOOTING.md) – Common issues and fixes

---

**Last Updated**: 2026-06-04  
**Next Major Version**: v1.0.0 (Q4 2026)  
**Questions?**: Post in GitHub Discussions or contact governance@bonsai.ecosystem
