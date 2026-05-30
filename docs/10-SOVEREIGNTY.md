# Sovereignty – Replacing External Dependencies

> **"Bonsai will eventually be built entirely from custom code, with zero external dependencies."**

Sovereignty means owning every line of code your software depends on. It means any bug can be traced, any behaviour can be audited, and any component can be replaced — without waiting for an external maintainer.

---

## Why This Matters

Today's Rust ecosystem has thousands of excellent crates. But each external dependency is:
- A potential supply chain attack vector
- A version conflict waiting to happen
- A crate that may be abandoned or change its API
- A line of code you didn't write and can't fully audit

The Bonsai sovereignty plan replaces every external dependency with a custom `bonsai-*` crate, purpose-built for the ecosystem, with:
- Full test coverage
- Formal verification where appropriate (using `bonsai-verify-lean`, `bonsai-verify-fstar`, etc.)
- Stable APIs that will never break
- Zero transitive dependencies

---

## Current Status

| Layer | Status |
|---|---|
| **Phase 0** – Vendor all deps, CI offline | ✅ Complete |
| **Phase 1** – Error, logging, RNG, time, IDs, codecs, FS | 🔄 In progress (`bonsai-error` ✅) |
| **Phase 2** – Crypto, HTTP, WebSocket, TLS, database | 📋 Planned (Month 4–8) |
| **Phase 3** – Regex, image, audio, DOM, Markdown, editor | 📋 Planned (Month 9–15) |
| **Phase 4** – ML inference, git, WASM runtime, bot clients | 📋 Planned (Month 16–24) |
| **Phase 5** – Async runtime | 📋 Planned (Month 25–30) |
| **Phase 6** – App framework (Tauri, Svelte) | 📋 Planned (Month 30–48) |

---

## Completed: `bonsai-error`

The first sovereignty crate is complete. It replaces `anyhow` and `thiserror`.

**What it provides:**
- `BonsaiError` – structured error with `ErrorKind`, message, source chain, context stack, recovery hint
- `BonsaiResult<T>` – type alias for `Result<T, BonsaiError>`
- `ResultExt` trait – `.context()` and `.kind()` adapters for any `Result<T, E>`
- `bail!` and `ensure!` macros
- `ErrorKind::is_transient()` – for retry logic
- Zero external dependencies

**Usage:**
```rust
use bonsai_error::{BonsaiError, BonsaiResult, ErrorKind, ResultExt, bail};

fn load_module(name: &str) -> BonsaiResult<Module> {
    let path = find_module(name)
        .context("locating module directory")?;  // ResultExt trait

    if !path.exists() {
        bail!(ErrorKind::NotFound, "module '{}' not found at {}", name, path.display());
    }

    Module::load(&path)
        .kind(ErrorKind::Io)  // promote io::Error to specific kind
}
```

---

## Phase 1 Roadmap

### `bonsai-log` (Month 1)
Replaces: `tracing`, `tracing-subscriber`, `log`

- Structured logging with levels (trace, debug, info, warn, error)
- Ring-buffer backend for Activity Log
- File rotation backend
- JSON formatter for machine-readable output
- No-alloc hot path (fixed-size stack buffers)

### `bonsai-rand` (Month 1)
Replaces: `rand`, `rand_chacha`

- ChaCha20 CSPRNG
- `Rng` trait with `gen::<T>()`, `gen_range()`, `shuffle()`
- Seeded from OS entropy (`getrandom`)
- `Deterministic` mode (fixed seed for reproducible tests)

### `bonsai-time` (Month 2)
Replaces: `chrono`, `time`

- UTC timestamp (`UnixMs`, `UnixSec`)
- `Duration` with saturating arithmetic
- `Instant` for monotonic elapsed measurement
- ISO 8601 formatting / parsing
- No timezone database (use UTC or offset-only)

### `bonsai-id` (Month 2)
Replaces: `uuid`

- UUID v4 (random), v7 (time-ordered for database performance)
- `Blake3Id` – 32-byte content-addressed identifier
- `ShortId` – human-readable 8-character base58 identifier
- Serde support

### `bonsai-codec` (Month 3)
Replaces: `serde_json`, `serde`, `bincode`, `base64`

- `Encode` / `Decode` traits (derive macros)
- JSON backend (compliant, minimal allocations)
- MessagePack backend (binary, compact)
- Base58, Base64 URL-safe encoding
- CBOR backend (for protocol messages)

### `bonsai-fs` (Month 3)
Replaces: `tokio::fs`, `std::fs` ergonomics

- `read_to_string`, `write`, `copy`, `create_dir_all`, etc.
- Atomic write (write to temp, rename)
- Path manipulation without `std::path` verbosity
- CAS-aware: `write_cas(data)` returns BLAKE3 key

### `bonsai-sanitise` (Month 3)
Replaces: `sanitize-filename`, `percent-encoding`, input validation helpers

- Path sanitisation (strip traversal attacks)
- URL percent-encoding / decoding
- UTF-8 validation and normalisation
- HTML entity encoding (for webview content)

---

## Phase 2–6 (Abbreviated)

**Phase 2** – `bonsai-crypto` (replaces `aes-gcm`, `ed25519-dalek`, `x25519-dalek`, `argon2`), `bonsai-http` (replaces `reqwest`), `bonsai-ws` (replaces `tokio-tungstenite`), `bonsai-db` (replaces `rusqlite` and `sqlx`).

**Phase 3** – `bonsai-regex`, `bonsai-image`, `bonsai-audio`, `bonsai-markdown`.

**Phase 4** – `bonsai-infer` (replaces `llama-cpp-rs`, `candle`), `bonsai-git`, `bonsai-wasm`, `bonsai-bot`.

**Phase 5** – `bonsai-executor` (async runtime, replaces Tokio). This is the hardest phase — Tokio has an enormous surface area and the executor is deeply integrated.

**Phase 6** – Replace Tauri's webview shell and Svelte with a native Bonsai UI framework (working title: `bonsai-ui`). This makes Bonsai a completely self-contained binary with no JavaScript runtime.

---

## The AI Crate Factory

Generating sovereignty crates by hand would take years. Bonsai uses **BonsAI itself** to accelerate the process.

### `scripts/generate_crate.ps1`

```powershell
./scripts/generate_crate.ps1 -name bonsai-rand -replaces rand -description "CSPRNG for Bonsai"
```

This script:
1. Asks the teacher model to generate `Cargo.toml`, `src/lib.rs`, and `tests/` from the specification.
2. Runs `cargo check` and `cargo test`.
3. If tests fail, feeds the errors back to the model for correction (up to 5 iterations).
4. On success, opens a PR-ready branch with the new crate.

The script uses the **Training Agent** — a model fine-tuned specifically on Rust systems programming and crate design — for higher quality output than the general BonsAI model.

---

## Training Agent

The Training Agent is a custom 7B model fine-tuned on:
- Rust crate documentation and source code
- The Bonsai codebase itself
- Formal verification patterns (from `bonsai-verify-*` crates)
- Systems programming papers and RFCs

It is specialised for:
- Generating safe, idiomatic Rust
- Writing correct `unsafe` code with explicit safety invariants
- Designing zero-copy, allocation-free APIs
- Writing formal proofs for critical properties

Train it yourself from the provided dataset:
```bash
just train-agent
```

---

## How to Contribute a Replacement Crate

1. Check the phase roadmap to find an unassigned crate.
2. Read the existing external crate's documentation and tests to understand its full API surface.
3. Use `scripts/generate_crate.ps1` as a starting point.
4. Ensure 100% test coverage of the public API.
5. For security-critical crates, add a formal proof (Lean 4 or F★) for the core invariant.
6. Open a PR against the `feat/sovereignty-phase-N` branch.

---

*← [Mobile Apps](09-MOBILE.md) · [Security →](11-SECURITY.md)*
