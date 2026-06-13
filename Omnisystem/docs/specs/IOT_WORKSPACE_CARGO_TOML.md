# IoT Control System - Complete Cargo.toml Workspace Configuration
## Ready-to-Use Workspace Setup for All 85+ Crates

This document provides the complete root Cargo.toml for the entire 24-week IoT project, organized by phase.

---

## ROOT Cargo.toml (Copy to z:\Projects\BonsaiWorkspace\Cargo.toml)

```toml
[workspace]
resolver = "2"

members = [
    # ================================================================================
    # PHASE 16: CORE IoT INFRASTRUCTURE (18 crates)
    # ================================================================================
    
    # Foundation types and interfaces
    "crates/omnisystem-iot-types",
    "crates/omnisystem-iot-core",
    
    # Registry and state management
    "crates/omnisystem-iot-registry",
    "crates/omnisystem-iot-state",
    "crates/omnisystem-iot-scheduler",
    
    # Device drivers (7 crates)
    "crates/omnisystem-iot-driver-light",
    "crates/omnisystem-iot-driver-thermostat",
    "crates/omnisystem-iot-driver-lock",
    "crates/omnisystem-iot-driver-sensor",
    "crates/omnisystem-iot-driver-blind",
    "crates/omnisystem-iot-driver-switch",
    "crates/omnisystem-iot-driver-custom",
    
    # Support systems
    "crates/omnisystem-iot-transport",
    "crates/omnisystem-iot-addressing",
    "crates/omnisystem-iot-discovery",
    "crates/omnisystem-iot-mesh",
    "crates/omnisystem-iot-gateway",
    "crates/omnisystem-iot-api",

    # ================================================================================
    # PHASE 17A: TITANIUM PHYSICAL LAYER (6 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-titanium-phy-types",
    "crates/omnisystem-titanium-radio",
    "crates/omnisystem-titanium-modulation",
    "crates/omnisystem-titanium-driver-cc26xx",
    "crates/omnisystem-titanium-driver-nrf52",
    "crates/omnisystem-titanium-driver-custom",

    # ================================================================================
    # PHASE 17B: TITANIUM MAC LAYER (7 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-titanium-mac-core",
    "crates/omnisystem-titanium-frame",
    "crates/omnisystem-titanium-addressing",
    "crates/omnisystem-titanium-ack-optimization",
    "crates/omnisystem-titanium-qos",
    "crates/omnisystem-titanium-power",
    "crates/omnisystem-titanium-mac-diagnostics",

    # ================================================================================
    # PHASE 17C: TITANIUM NETWORK LAYER (8 crates, 2 weeks)
    # ================================================================================
    
    "crates/omnisystem-titanium-6lowpan-core",
    "crates/omnisystem-titanium-routing",
    "crates/omnisystem-titanium-neighbor-discovery",
    "crates/omnisystem-titanium-icmpv6",
    "crates/omnisystem-titanium-fragmentation",
    "crates/omnisystem-titanium-rpl",
    "crates/omnisystem-titanium-mesh-repair",
    "crates/omnisystem-titanium-network-diagnostics",

    # ================================================================================
    # PHASE 17D: TITANIUM APS LAYER (5 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-titanium-aps-core",
    "crates/omnisystem-titanium-endpoints",
    "crates/omnisystem-titanium-binding",
    "crates/omnisystem-titanium-aps-security",
    "crates/omnisystem-titanium-aps-diagnostics",

    # ================================================================================
    # PHASE 17E: TITANIUM ZCL (9 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-titanium-zcl-core",
    "crates/omnisystem-titanium-zcl-basic",
    "crates/omnisystem-titanium-zcl-lighting",
    "crates/omnisystem-titanium-zcl-hvac",
    "crates/omnisystem-titanium-zcl-lock",
    "crates/omnisystem-titanium-zcl-sensor",
    "crates/omnisystem-titanium-zcl-window",
    "crates/omnisystem-titanium-zcl-diagnostic",
    "crates/omnisystem-titanium-zcl-custom",

    # ================================================================================
    # PHASE 17F: TITANIUM DEVICE ROLES (4 crates, 1 week)
    # ================================================================================
    
    "crates/omnisystem-titanium-coordinator",
    "crates/omnisystem-titanium-router",
    "crates/omnisystem-titanium-enddevice",
    "crates/omnisystem-titanium-sleepy",

    # ================================================================================
    # PHASE 17G: TITANIUM SECURITY (6 crates, 1 week)
    # ================================================================================
    
    "crates/omnisystem-titanium-key-management",
    "crates/omnisystem-titanium-encryption",
    "crates/omnisystem-titanium-authentication",
    "crates/omnisystem-titanium-trust",
    "crates/omnisystem-titanium-audit",
    "crates/omnisystem-titanium-tls",

    # ================================================================================
    # PHASE 18A: AETHER PHYSICAL LAYER (6 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-aether-phy-types",
    "crates/omnisystem-aether-radio",
    "crates/omnisystem-aether-modulation",
    "crates/omnisystem-aether-driver-zw0503",
    "crates/omnisystem-aether-driver-ti",
    "crates/omnisystem-aether-driver-custom",

    # ================================================================================
    # PHASE 18B: AETHER MAC LAYER (6 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-aether-mac-core",
    "crates/omnisystem-aether-frame",
    "crates/omnisystem-aether-priority",
    "crates/omnisystem-aether-addressing",
    "crates/omnisystem-aether-duty-cycle",
    "crates/omnisystem-aether-mac-diagnostics",

    # ================================================================================
    # PHASE 18C: AETHER ROUTING (7 crates, 2 weeks)
    # ================================================================================
    
    "crates/omnisystem-aether-routing-core",
    "crates/omnisystem-aether-neighbors",
    "crates/omnisystem-aether-topology",
    "crates/omnisystem-aether-healing",
    "crates/omnisystem-aether-multipath",
    "crates/omnisystem-aether-latency",
    "crates/omnisystem-aether-topology-optimize",

    # ================================================================================
    # PHASE 18D: AETHER TRANSPORT (4 crates, 1 week)
    # ================================================================================
    
    "crates/omnisystem-aether-transport",
    "crates/omnisystem-aether-session",
    "crates/omnisystem-aether-flow-control",
    "crates/omnisystem-aether-reliability",

    # ================================================================================
    # PHASE 18E: AETHER COMMAND CLASSES (12 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-aether-cmd-core",
    "crates/omnisystem-aether-cmd-basic",
    "crates/omnisystem-aether-cmd-switch-binary",
    "crates/omnisystem-aether-cmd-switch-multilevel",
    "crates/omnisystem-aether-cmd-color",
    "crates/omnisystem-aether-cmd-thermostat",
    "crates/omnisystem-aether-cmd-sensor-binary",
    "crates/omnisystem-aether-cmd-sensor-multilevel",
    "crates/omnisystem-aether-cmd-lock",
    "crates/omnisystem-aether-cmd-battery",
    "crates/omnisystem-aether-cmd-wakeup",
    "crates/omnisystem-aether-cmd-custom",

    # ================================================================================
    # PHASE 18F: AETHER DEVICE ROLES (3 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-aether-controller",
    "crates/omnisystem-aether-repeater",
    "crates/omnisystem-aether-device",

    # ================================================================================
    # PHASE 18G: AETHER SECURITY (6 crates, 1.5 weeks)
    # ================================================================================
    
    "crates/omnisystem-aether-key-management",
    "crates/omnisystem-aether-encryption",
    "crates/omnisystem-aether-s2",
    "crates/omnisystem-aether-authentication",
    "crates/omnisystem-aether-provisioning",
    "crates/omnisystem-aether-audit",

    # ================================================================================
    # PHASE 19: INTEGRATION & TRANSFERDAEMON BRIDGE (9 crates, 2 weeks)
    # ================================================================================
    
    "crates/omnisystem-iot-multi-protocol",
    "crates/omnisystem-iot-bridging",
    "crates/omnisystem-iot-fallback",
    "crates/omnisystem-iot-edge-compute",
    "crates/omnisystem-iot-sync",
    "crates/omnisystem-iot-mesh-network",
    "crates/omnisystem-iot-api-gateway",
    "crates/omnisystem-iot-automation",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Omnisystem IoT Team <iot@omnisystem.dev>"]
license = "MPL-2.0"
repository = "https://github.com/omnisystem/omnisystem-iot"
homepage = "https://omnisystem.dev"

# ================================================================================
# WORKSPACE-WIDE BUILD SETTINGS
# ================================================================================

[profile.dev]
opt-level = 1
debug = true
debug-assertions = true
overflow-checks = true
lto = false
incremental = true

[profile.release]
opt-level = 3
debug = false
debug-assertions = false
overflow-checks = false
lto = "thin"
codegen-units = 16
strip = false
panic = "abort"

[profile.test]
opt-level = 1
debug = true
debug-assertions = true
overflow-checks = true
lto = false
incremental = true

[profile.bench]
opt-level = 3
lto = "thin"
codegen-units = 1

# ================================================================================
# WORKSPACE DEFAULT DEPENDENCIES
# ================================================================================
# These are inherited by all crates that don't override them

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1.6", features = ["v4", "serde"] }
parking_lot = "0.12"
async-trait = "0.1"
rand = "0.8"
chrono = { version = "0.4", features = ["serde"] }

# ================================================================================
# LINTS - STRICT SAFETY & QUALITY
# ================================================================================

[workspace.lints.rust]
unsafe_code = "warn"
missing_docs = "warn"

[workspace.lints.clippy]
all = "warn"
pedantic = "warn"
perf = "warn"
correctness = "deny"
suspicious = "warn"

# ================================================================================
# METADATA
# ================================================================================

[workspace.metadata]
# CI/CD configuration reference
ci = "github-actions"
python-version = "3.11"
rust-version = "1.70"

# Testing configuration
[workspace.metadata.test]
unit-tests = true
integration-tests = true
network-simulation = true
hardware-tests = false  # Enable in CI with real hardware

# Performance targets
[workspace.metadata.performance]
latency-target-ms = 50
uptime-target-pct = 99.99
scalability-devices = 500000

# Security targets
[workspace.metadata.security]
encryption = "AES-256"
authentication = "HMAC-SHA256"
post-quantum-ready = true
```

---

## INDIVIDUAL CRATE Cargo.toml TEMPLATE

Use this template for each new crate:

```toml
[package]
name = "omnisystem-<system>-<component>"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
description = "Brief description of this crate's functionality"
keywords = ["iot", "system", "wireless"]
categories = ["embedded", "network-programming"]

[dependencies]
# Core dependencies from workspace
tokio = { workspace = true, features = ["rt", "sync", "time"] }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }

# Local dependencies on other crates
omnisystem-iot-types = { path = "../omnisystem-iot-types" }
omnisystem-iot-core = { path = "../omnisystem-iot-core" }

# Optional: Add crate-specific dependencies
parking_lot = { workspace = true, optional = true }
async-trait = { workspace = true, optional = true }

[dev-dependencies]
tokio = { workspace = true, features = ["full"] }

[features]
default = ["std"]
std = []
# Additional feature gates
advanced-diagnostics = []
experimental = []

# Documentation settings
[package.metadata.docs.rs]
all-features = true
```

---

## BUILD SCRIPT CHECKLIST

Create `.cargo/config.toml` in the root:

```toml
[build]
# Use LLD for faster linking (if available)
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[profile.release]
split-debuginfo = "packed"

[alias]
# Useful aliases
ck = "check --all"
tst = "test --all"
doc = "doc --no-deps --all"
clippy = "clippy --all -- -D warnings"
fmt-check = "fmt --all -- --check"
```

---

## CI/CD GITHUB ACTIONS WORKFLOW

Create `.github/workflows/test.yml`:

```yaml
name: Test & Lint

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - uses: dtolnay/rust-toolchain@stable
      
      - uses: Swatinem/rust-cache@v2
      
      - name: Format Check
        run: cargo fmt --all -- --check
      
      - name: Clippy
        run: cargo clippy --all -- -D warnings
      
      - name: Tests
        run: cargo test --all --verbose
      
      - name: Doc Tests
        run: cargo test --doc --all

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - uses: taiki-e/install-action@cargo-tarpaulin
      - run: cargo tarpaulin --all --timeout 300 --out xml
      - uses: codecov/codecov-action@v3
```

---

## SCRIPTS FOR COMMON TASKS

Create `scripts/build.sh`:

```bash
#!/bin/bash
set -e

echo "Building Omnisystem IoT Platform..."

# Clean
cargo clean

# Format
cargo fmt --all

# Check
cargo check --all

# Tests
cargo test --all --verbose

# Docs
cargo doc --all --no-deps

echo "✅ All checks passed!"
```

Create `scripts/test-networks.sh`:

```bash
#!/bin/bash

echo "Running network simulation tests..."

# Run with specific test patterns
cargo test --all -- --test-threads=4 --nocapture \
  network_simulation \
  mesh \
  routing \
  scale

echo "✅ Network tests complete!"
```

---

## MAKEFILE FOR CONVENIENCE

Create `Makefile`:

```makefile
.PHONY: build test check fmt lint clean doc all

all: check test doc

build:
	cargo build --all

check:
	cargo check --all --verbose

test:
	cargo test --all --verbose

test-network:
	./scripts/test-networks.sh

fmt:
	cargo fmt --all

lint:
	cargo clippy --all -- -D warnings

doc:
	cargo doc --all --no-deps --open

clean:
	cargo clean

install-hooks:
	cp .git/hooks/pre-commit.sample .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit
```

---

## DIRECTORY STRUCTURE TO CREATE

```
z:\Projects\BonsaiWorkspace\
├── Cargo.toml (root - use above)
├── Cargo.lock (auto-generated)
├── .cargo/
│   └── config.toml (use above)
├── .github/
│   └── workflows/
│       └── test.yml (use above)
├── crates/
│   ├── omnisystem-iot-types/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   ├── omnisystem-iot-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   # ... (83 more crates following same pattern)
├── tests/
│   ├── integration_tests.rs
│   ├── network_simulation.rs
│   └── hardware_tests.rs
├── scripts/
│   ├── build.sh
│   ├── test-networks.sh
│   └── deploy.sh
├── docs/
│   ├── ARCHITECTURE.md
│   ├── API.md
│   ├── DEPLOYMENT.md
│   └── SECURITY.md
├── examples/
│   ├── smart_home.rs
│   ├── factory_control.rs
│   └── multi_location.rs
└── Makefile
```

---

## COMPILATION INSTRUCTIONS

### First-Time Setup

```bash
# Clone/download the project
cd z:\Projects\BonsaiWorkspace

# Verify Rust installation
rustc --version  # Should be 1.70+
cargo --version

# Download dependencies (will take 5-10 minutes first time)
cargo fetch

# Initial compile (will take 2-3 minutes)
cargo build --all

# Run all tests
cargo test --all
```

### Daily Development

```bash
# Format code
make fmt

# Check for issues
make lint

# Run tests
make test

# Build release
cargo build --release

# Document
make doc
```

### Phase-Specific Compilation

```bash
# Compile only Phase 16
cargo build -p omnisystem-iot-types \
            -p omnisystem-iot-core \
            -p omnisystem-iot-registry \
            # ... etc

# Test only Phase 17 (Titanium)
cargo test --all --test 'titanium*'

# Build just the MAC layer
cargo build \
  -p omnisystem-titanium-mac-core \
  -p omnisystem-titanium-frame \
  -p omnisystem-titanium-ack-optimization \
  # ... etc
```

---

## EXPECTED BUILD TIMES

| Scenario | Time |
|----------|------|
| **Initial fetch** | 5-10 min |
| **First build** | 2-3 min |
| **Incremental (1 crate)** | 5-30 sec |
| **Incremental (5 crates)** | 30-90 sec |
| **Full rebuild** | 1-2 min |
| **Full test suite** | 5-10 min |
| **Release build** | 2-3 min |

---

## TROUBLESHOOTING BUILD ISSUES

### Out of Memory During Compilation

```bash
# Limit parallel compilation
cargo build -j 2

# Use incremental compilation
CARGO_INCREMENTAL=1 cargo build
```

### Linker Errors

```bash
# Use system linker
rustflags="" cargo build

# Use mold (faster linker)
RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build
```

### Test Timeouts

```bash
# Increase test timeout
cargo test --all -- --test-threads=1
```

---

## NEXT STEPS AFTER SETUP

1. **Verify all 85+ crates compile**:
   ```bash
   cargo check --all
   ```

2. **Run Phase 16 tests**:
   ```bash
   cargo test -p omnisystem-iot-types
   cargo test -p omnisystem-iot-core
   cargo test -p omnisystem-iot-registry
   # ... etc
   ```

3. **Start Phase 17A implementation**:
   - Begin with `omnisystem-titanium-phy-types`
   - Then `omnisystem-titanium-radio`
   - Follow the week-by-week schedule

4. **Continuous integration**:
   - Push to GitHub to trigger CI
   - Monitor test results
   - Fix failures immediately

---

**Status**: ✅ Complete workspace configuration ready for setup

**Next Action**: Copy root Cargo.toml to z:\Projects\BonsaiWorkspace\Cargo.toml and run `cargo check --all`

