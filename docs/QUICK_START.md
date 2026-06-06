# Quick Start – Get Running in 5 Minutes

This guide walks you through cloning, building, and sending your first P2P message in the Bonsai Ecosystem.

## Prerequisites

- **Rust**: 1.70+ ([install](https://rustup.rs/))
- **Git**: For cloning the repository
- **5 minutes**: And a terminal

## Step 1: Clone the Repository (1 min)

```bash
git clone https://github.com/LoopyLuci/BonsaiWorkspace
cd BonsaiWorkspace
```

## Step 2: Build (2–3 min)

Build the entire workspace without optional AI features (production-grade, deterministic-only):

```bash
cargo build --release --workspace --no-default-features
```

**What you just built:**
- `bonsai-nexus` – The core daemon
- `polyglot-pong-orchestrator` – Language validation framework
- `polyglot-pong-dashboard` – Real-time metrics UI
- All supporting crates (TransferDaemon, compression, etc.)

## Step 3: Verify Installation (30 sec)

Check that the build succeeded:

```bash
./target/release/bonsai-nexus --help
```

You should see:
```
Bonsai Nexus – Sovereign Operating System
USAGE: bonsai-nexus [OPTIONS]
OPTIONS:
  --port <PORT>           Listen on port (default: 8000)
  --identity-file <FILE>  Path to self-certifying identity
  --help                  Show this message
```

## Step 4: Run Your First Test (1 min)

### Option A: Start the Polyglot Pong Dashboard

In one terminal, start the dashboard:

```bash
cd polyglot-pong
cargo run --release --bin polyglot-pong-dashboard
```

The WebSocket server listens on `0.0.0.0:8080`. Open a browser to `http://localhost:8080` and you'll see a real-time metrics display.

### Option B: Send a Message via TransferDaemon

In another terminal, create a test message:

```bash
./target/release/bonsai-nexus --port 9000 --identity-file ~/.bonsai/my-identity
```

This spawns a P2P node that:
- Generates a self-certifying identity (or loads from file)
- Listens on port 9000
- Is ready to send/receive encrypted messages

To send a message to another peer, you'd normally use:

```bash
bonsai-transfer send \
  --to <peer-identity> \
  --message "Hello, Bonsai!" \
  --endpoint 127.0.0.1:9000
```

(This requires a second peer running in another window or on another machine.)

## Step 5: Run a Quick Test Suite (1 min)

Verify everything works:

```bash
cargo test --workspace --no-default-features -- --nocapture
```

You'll see:
- ✅ Deterministic Pong trace tests
- ✅ SovereignService tier tests
- ✅ TransferDaemon identity tests
- ✅ Compression algorithm tests

---

## What Just Happened

You've built the **complete Bonsai Ecosystem**:

1. **Deterministic-First Core**: All systems work without AI. Try `--no-default-features` vs `--all-features` and both produce identical results.
2. **Polyglot Pong**: A test framework that validates 750+ languages against a canonical fixed-point spec.
3. **TransferDaemon v2**: P2P networking with self-certifying identities and post-quantum crypto.
4. **SovereignService**: A trait that ensures graceful degradation (AI → Heuristic → Deterministic → Safe Stub).

---

## Next Steps

- **Understand the Architecture**: Read [ARCHITECTURE.md](ARCHITECTURE.md)
- **Learn the AI-Optional Philosophy**: Read [DETERMINISTIC_BACKBONE.md](DETERMINISTIC_BACKBONE.md)
- **Run Polyglot Pong at Scale**: See [POLYGLOT_PONG.md](POLYGLOT_PONG.md)
- **Deploy to Production**: See [DEPLOYMENT.md](DEPLOYMENT.md)
- **Contribute**: See [CONTRIBUTING.md](CONTRIBUTING.md)

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `error: could not find Cargo.toml in parent directories` | Make sure you're in the `BonsaiWorkspace` root directory. |
| `error: no default toolchain installed` | Run `rustup default stable` and try again. |
| `error: failed to resolve: use of undeclared crate` | Run `cargo update && cargo clean && cargo build --release` |
| Build takes >10 min | You may be in debug mode. Use `--release` flag. |

---

**You're ready! 🚀 Read [ARCHITECTURE.md](ARCHITECTURE.md) or [DETERMINISTIC_BACKBONE.md](DETERMINISTIC_BACKBONE.md) next.**
