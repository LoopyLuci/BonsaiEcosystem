# BonsaiWorkspace - Unified Build System & Integration Guide

**Status**: Foundation for Workspace Unification  
**Date**: 2026-06-11  
**Scope**: 228+ crates across 15+ major projects  

---

## Executive Summary

BonsaiWorkspace is a monorepo containing 228+ Rust crates organized into 15 major systems:

1. **Omnisystem** - Universal runtime/kernel (25 crates)
2. **PATHFINDER** - Learning platform (9 services)
3. **Network Firmware** - IoT/embedded systems
4. **IoT Control** - Device coordination
5. **OmniSearch** - Full-text search engine
6. **OmniFile** - File management system
7. **OmniLingual** - Translation engine
8. **OmniSocial** - Social networking
9. **OmniRecommend** - Recommendation engine
10. **OmniOS** - Co-Operating System
11. **BACE** - Build acceleration
12. **Polyglot Pong** - Language validation
13. **TransferDaemon** - P2P messaging
14. **Omnivault** - Secure storage
15. **AI Systems** - ML/AI infrastructure

This document provides the unified build, test, and deployment system.

---

## Workspace Structure

```
BonsaiWorkspace/
├── Cargo.toml (workspace root - 25 explicit members)
├── Cargo.lock (dependency lock)
├── Omnisystem/ (primary monorepo)
│   ├── Cargo.toml
│   ├── crates/ (228+ total crates)
│   │   ├── omnisystem-* (50 core crates)
│   │   ├── pathfinder-* (9 service crates)
│   │   ├── network-* (15 firmware crates)
│   │   ├── iot-* (20 control crates)
│   │   ├── search-* (18 search crates)
│   │   ├── file-* (12 file crates)
│   │   ├── ai-* (25 AI crates)
│   │   └── ... (80+ additional crates)
│   ├── services/ (microservices)
│   ├── infrastructure/ (deployment configs)
│   └── scripts/ (build/test scripts)
├── scripts/
│   ├── build_all.sh
│   ├── test_all.sh
│   ├── benchmark_all.sh
│   └── ci.sh
├── docker/
│   ├── Dockerfile.dev
│   ├── Dockerfile.prod
│   └── docker-compose.yml
├── kubernetes/
│   └── *.yaml (deployment manifests)
├── docs/
│   ├── ARCHITECTURE.md
│   ├── GETTING_STARTED.md
│   ├── CONTRIBUTING.md
│   └── ...
└── .github/workflows/
    ├── ci.yml (unified CI)
    ├── build.yml
    └── deploy.yml
```

---

## Crate Organization by Tier

### Tier 0: Foundation (3 crates)
Critical system foundations that all others depend on:
- `omnisystem-ums` - Universal Module System
- `omnisystem-axiom-spec` - Type specification
- Core error/logging primitives

### Tier 1: Core Runtime (8 crates)
Kernel and runtime environment:
- `omnisystem-kernel` - Executive kernel
- `omnisystem-ffi` - Foreign function interface
- `omnisystem-loader` - Dynamic loader
- `omnisystem-async` - Async runtime
- Bindings layer (Rust, Go, etc.)

### Tier 2: OS Integration (10 crates)
Platform-specific implementations:
- `omnisystem-linux` - Linux layer
- `omnisystem-windows` - Windows layer
- `omnisystem-macos` - macOS layer
- Hardware abstractions (CPU, memory, devices)

### Tier 3: Services (50+ crates)
Application-level services:
- **PATHFINDER** (9): User, Content, Progress, Teacher, Parent, Notification, Achievement, Insights, Personalization
- **OmniSearch** (15): Core, distributed, indexing, ranking, etc.
- **OmniFile** (12): Storage, access, versioning, etc.
- **Network** (15): IoT, firmware, protocols
- **AI** (25): Models, training, inference

### Tier 4: Integration (30+ crates)
Cross-system integrations and bridges

### Tier 5: Tools (50+ crates)
Testing, benchmarking, utilities

---

## Building the Workspace

### Build All Crates

```bash
# Full debug build
cargo build --workspace

# Full release build (optimized)
cargo build --workspace --release

# Specific crate
cargo build -p omnisystem-kernel

# Specific features
cargo build --workspace --features=all
```

### Build by Tier

```bash
# Build only foundation tier (fast)
cargo build -p omnisystem-ums -p omnisystem-axiom-spec

# Build foundation + core runtime
cargo build -p omnisystem-ums -p omnisystem-axiom-spec \
  -p omnisystem-kernel -p omnisystem-ffi

# Build PATHFINDER services only
cargo build -p pathfinder-user-service \
  -p pathfinder-content-service \
  -p pathfinder-progress-service \
  -p pathfinder-teacher-service
```

### Build by System

```bash
# Build all Omnisystem core
cargo build --manifest-path Omnisystem/Cargo.toml

# Build PATHFINDER
./scripts/build_pathfinder.sh

# Build OmniSearch
./scripts/build_omnisearch.sh

# Build all IoT/Network
./scripts/build_network.sh
```

---

## Testing Strategy

### Unit Tests

```bash
# Run all unit tests
cargo test --workspace --lib

# Run tests for specific crate
cargo test -p omnisystem-kernel

# Run single test
cargo test -p omnisystem-kernel kernel_init

# Run with output
cargo test --workspace -- --nocapture
```

### Integration Tests

```bash
# Run integration tests only
cargo test --workspace --test '*'

# Run integration tests for PATHFINDER
cargo test --workspace --test '*' --features=pathfinder
```

### Benchmark Tests

```bash
# Run benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench -p omnisystem-kernel -- kernel_performance
```

### Test Coverage

```bash
# Install tarpaulin for coverage
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html --output-dir coverage/

# Check coverage threshold (85%+)
cargo tarpaulin --workspace --timeout 600 --fail-under 85
```

---

## Dependency Management

### Check Dependencies

```bash
# List all dependencies
cargo tree --workspace

# Check for outdated dependencies
cargo outdated

# Audit for security vulnerabilities
cargo audit

# Fix vulnerabilities
cargo audit --fix
```

### Dependency Policies

1. **Tier 0 (Foundation)**: Zero external dependencies (except Rust std)
2. **Tier 1 (Core)**: Minimal dependencies, carefully vetted
3. **Tier 2+ (Services)**: Standard ecosystem dependencies permitted

### Lock File Management

```bash
# Update Cargo.lock
cargo update

# Update specific package
cargo update -p some-package

# Verify lock file consistency
cargo check --workspace
```

---

## Development Workflow

### Local Development

```bash
# 1. Clone repository
git clone https://github.com/omnisystem/bonsaiworkspace
cd BonsaiWorkspace

# 2. Install Rust toolchain
rustup update stable
rustup component add rustfmt clippy

# 3. Build workspace
cargo build --workspace

# 4. Run tests
cargo test --workspace

# 5. Format code
cargo fmt --all

# 6. Lint
cargo clippy --workspace -- -D warnings
```

### Feature Flags

```bash
# Build with all features
cargo build --workspace --all-features

# Build with specific feature
cargo build --workspace --features "omnisystem-core"

# Build without default features
cargo build --workspace --no-default-features
```

### Profiling

```bash
# Build with profiling symbols
RUSTFLAGS="-g" cargo build --release

# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bin some-binary

# Memory profiling
cargo install heaptrack
heaptrack ./target/release/some-binary
```

---

## Continuous Integration

### GitHub Actions Workflow

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Cache Rust
        uses: Swatinem/rust-cache@v2
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Lint
        run: cargo clippy --workspace -- -D warnings
      
      - name: Build
        run: cargo build --workspace --release
      
      - name: Test
        run: cargo test --workspace --release
      
      - name: Coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --workspace --timeout 600 --fail-under 85

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - name: Audit dependencies
        run: cargo audit
```

---

## Build Optimization

### Link-Time Optimization (LTO)

```toml
[profile.release]
lto = true              # Enable LTO (slower build, faster binary)
codegen-units = 1      # 1 unit for better optimization (slower build)
opt-level = 3          # Maximum optimization
```

### Incremental Compilation

```bash
# Enable incremental compilation (faster rebuilds)
export CARGO_INCREMENTAL=1

# Disable for maximum final optimization
export CARGO_INCREMENTAL=0 cargo build --release
```

### Parallel Compilation

```bash
# Set parallel jobs (default = number of cores)
cargo build -j 4

# Use sccache for distributed caching
export RUSTC_WRAPPER=sccache
cargo build --workspace
```

---

## Production Deployment

### Docker Build

```bash
# Build dev image
docker build -f docker/Dockerfile.dev -t bonsaiworkspace:dev .

# Build production image
docker build -f docker/Dockerfile.prod -t bonsaiworkspace:prod .

# Run container
docker run -it bonsaiworkspace:prod
```

### Kubernetes Deployment

```bash
# Apply manifests
kubectl apply -f kubernetes/

# Monitor deployment
kubectl get pods -n bonsaiworkspace

# View logs
kubectl logs -n bonsaiworkspace -l app=pathfinder-user-service
```

### Binary Distribution

```bash
# Create distributable binary
cargo build --release

# Binary location
./target/release/bonsaiworkspace

# Strip symbols (smaller size)
strip target/release/bonsaiworkspace
```

---

## Troubleshooting

### Build Issues

**Issue: "cannot find crate for..." error**
```bash
# Solution: Clean and rebuild
cargo clean
cargo build --workspace
```

**Issue: Out of memory during build**
```bash
# Solution: Reduce parallel jobs
cargo build -j 2
```

**Issue: Slow incremental builds**
```bash
# Solution: Full clean rebuild
cargo clean
cargo build --release -j $(nproc)
```

### Test Failures

**Issue: Tests fail with "thread panicked"**
```bash
# Solution: Run with backtrace
RUST_BACKTRACE=1 cargo test

# Or full backtrace
RUST_BACKTRACE=full cargo test
```

**Issue: Test timeout**
```bash
# Solution: Increase timeout
cargo test -- --test-threads=1 --nocapture
```

### Dependency Issues

**Issue: "Updating indices" takes forever**
```bash
# Solution: Use sparse index (faster)
export CARGO_UNSTABLE_SPARSE_REGISTRY=true
```

**Issue: Git dependency resolution slow**
```bash
# Solution: Use git shallow clone
git config --global credential.helper store
```

---

## Performance Benchmarking

### Benchmark Suite

```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench -p omnisystem-kernel -- kernel

# Compare against baseline
cargo bench --bench kernel -- --save-baseline main
cargo bench --bench kernel -- --baseline main
```

### Metrics to Track

1. **Compile Time**
   - Full build: < 5 minutes
   - Incremental: < 30 seconds

2. **Binary Size**
   - Release binary: < 50 MB
   - Stripped: < 30 MB

3. **Runtime Performance**
   - P95 latency: < 500ms
   - Throughput: > 10,000 req/s

4. **Memory Usage**
   - Per-service: < 500 MB
   - Peak: < 2 GB

---

## Documentation Standards

### README Requirements

Every crate must have:
```markdown
# Crate Name

## Overview
Brief description and purpose

## Features
- Feature 1
- Feature 2

## Usage
Example code

## Performance
Benchmarks and characteristics

## Safety
Safety considerations if applicable
```

### API Documentation

```bash
# Generate HTML docs
cargo doc --workspace --open

# Document private items
cargo doc --workspace --document-private-items
```

---

## Release Process

### Version Management

```bash
# Semantic versioning: MAJOR.MINOR.PATCH
# 0.1.0 = foundation release
# 1.0.0 = production release

cargo set-version --workspace 1.0.0
```

### Release Checklist

- [ ] All tests passing
- [ ] Code coverage > 85%
- [ ] Security audit passed
- [ ] Performance benchmarks met
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version bumped
- [ ] Git tag created
- [ ] Build artifacts generated
- [ ] Docker images pushed

---

## Contributing Guidelines

### Code Style

```bash
# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check
```

### Quality Gates

1. All tests must pass
2. Code coverage must be > 85%
3. No compiler warnings
4. `clippy` must pass with no warnings
5. Security audit must pass

### Pull Request Process

1. Create feature branch: `git checkout -b feature/my-feature`
2. Make changes and commit: `git commit -am "feat: description"`
3. Push to remote: `git push origin feature/my-feature`
4. Create pull request with test results
5. Address review comments
6. Merge once approved

---

## Resource Requirements

### Minimum Development Environment

- **CPU**: 4 cores (8+ recommended)
- **RAM**: 8 GB (16+ recommended)
- **Disk**: 50 GB available
- **Network**: 100 Mbps

### Build Times (on 8-core i7)

- Fresh build: 5-8 minutes
- Incremental: 10-30 seconds
- Full test suite: 10-15 minutes
- Benchmark suite: 5-10 minutes

---

## Support & Resources

- **Issues**: https://github.com/omnisystem/bonsaiworkspace/issues
- **Discussions**: https://github.com/omnisystem/bonsaiworkspace/discussions
- **Documentation**: https://docs.omnisystem.dev
- **API Docs**: Run `cargo doc --open`

---

**Last Updated**: 2026-06-11  
**Maintainers**: Omnisystem Team  
**License**: MPL-2.0
