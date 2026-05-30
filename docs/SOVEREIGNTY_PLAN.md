# BonsAI Sovereignty Plan
## Replace Every External Dependency — Complete Self-Sustaining Ecosystem

**Version:** 1.0  
**Status:** Active  
**Horizon:** Multi-year, executed in phases  
**Philosophy:** Zero supply-chain risk. Every bit of code that runs inside the Bonsai Ecosystem is owned, audited, and maintained by the project. No external Cargo registry crates, no external npm packages beyond the vendored baseline, no phone-home SDKs.

---

## 0. Immediate — sccache & Offline Vendor Lock (Week 1)

### 0.1 sccache — DONE
`sccache v0.15.0` installed and wired into `.cargo/config.toml`:
```toml
[build]
rustc-wrapper = "sccache"
```
All `rustc` invocations are now cached. Clean builds that reuse unchanged crates are near-instant.

### 0.2 Vendor All Cargo Dependencies (Week 1)
Run once, commit the result, never touch the registry again:
```powershell
cargo vendor vendor/
```
Add to `.cargo/config.toml`:
```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```
Add `vendor/` exclusions to `.gitignore` for large binary blobs (pre-built `.a`/`.so`), keep source.  
CI must add `--offline` flag to all `cargo` invocations.

### 0.3 Vendor All npm Dependencies (Week 1)
```bash
npm pack --dry-run   # audit
```
Use Vite's `build.rollupOptions.output.manualChunks` to inline Monaco and xterm as vendored bundles.  
Lock package-lock.json. Run `npm ci --prefer-offline` in CI. Never `npm audit fix --force`.

---

## 1. Full Dependency Inventory

### 1.1 Rust Crates — External Dependencies by Category

Below is the complete set of third-party Cargo crates currently used, grouped by domain.  
Internal `bonsai-*` crates are excluded — they are already ours.

#### Async Runtime & Concurrency
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `tokio` (full) | 21 crates | `bonsai-executor` |
| `tokio-cron-scheduler` | `eternal-workshop` | `bonsai-scheduler` (already: `eternal-workshop/scheduler.rs`) |
| `tokio-rusqlite` | `bonsai-bot` | `bonsai-db` async adapter |
| `tokio-socks` | `bonsai-p2p` | absorbed into `bonsai-net` |
| `tokio-stream` | `bonsai-creator`, `bonsai-workspace` | `bonsai-executor` streams |
| `tokio-tungstenite` | `bonsai-daemon` | `bonsai-websocket` |
| `tokio-util` | `bonsai-bot` | `bonsai-executor` |
| `futures` | 5 crates | `bonsai-executor` |
| `async-trait` | 10 crates | native async fn in traits (Rust 1.75+) |
| `arc-swap` | `bonsai-actors`, `bonsai-daemon`, `bonsai-tool-registry` | `bonsai-sync` |

#### Serialisation
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `serde` | 32 crates | `bonsai-serde` |
| `serde_json` | 32 crates | `bonsai-json` |
| `serde_yaml` | 2 crates | `bonsai-yaml` |
| `bincode` | `bonsai-p2p` | `bonsai-codec` (already has frame format) |
| `ciborium` | `bonsai-actors` | `bonsai-cbor` |
| `toml` | `bonsai-workspace` | `bonsai-toml` |
| `base64` | `bonsai-workspace` | `bonsai-codec` |
| `hex` | 8 crates | `bonsai-codec` |

#### Cryptography
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `aes-gcm` | `bonsai-transfer-crypto`, `bonsai-transfer-store`, `bonsai-workspace` | `bonsai-crypto` |
| `argon2` | `bonsai-transfer-crypto`, `bonsai-transfer-store` | `bonsai-crypto` |
| `blake3` | 9 crates | `bonsai-crypto` |
| `ed25519-dalek` | `bonsai-transfer-crypto` | `bonsai-crypto` |
| `x25519-dalek` | `bonsai-transfer-crypto` | `bonsai-crypto` |
| `hmac` | `bonsai-workspace` | `bonsai-crypto` |
| `sha2` | 3 crates | `bonsai-crypto` |
| `zeroize` | 2 crates | `bonsai-crypto` (zero-on-drop wrapper) |
| `bip39` | `bonsai-transfer-crypto` | `bonsai-crypto` (wordlist + entropy) |
| `rand` | 10 crates | `bonsai-rand` |
| `rand_core` | `bonsai-transfer-crypto` | `bonsai-rand` |
| `rand_distr` | `bonsai-go-nn` | `bonsai-rand` |
| `ssh-key` | `bonsai-workspace` | `bonsai-crypto` (Ed25519 SSH wire format) |
| `zxcvbn` | `bonsai-workspace` | `bonsai-crypto` (password strength heuristic) |

#### Networking & HTTP
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `reqwest` | 7 crates | `bonsai-http` |
| `hyper` | `bonsai-workspace` | `bonsai-http` |
| `axum` | `bonsai-bot`, `bonsai-workspace` | `bonsai-http` (router) |
| `tower-http` | 2 crates | `bonsai-http` (middleware) |
| `bytes` | 3 crates | `bonsai-net` (BytesMut) |
| `libp2p` | `bonsai-p2p` | `bonsai-p2p` (expand existing) |
| `webrtc` | `bonsai-p2p` | `bonsai-webrtc` (ICE+DTLS+SCTP subset) |
| `mdns-sd` | `bonsai-workspace` | absorbed into `bonsai-p2p` discovery |
| `local-ip-address` | `bonsai-workspace` | `bonsai-net` (OS network interface query) |
| `gethostname` | `bonsai-workspace` | `bonsai-net` |
| `tokio-socks` | `bonsai-p2p` | `bonsai-net` SOCKS5 |

#### Database & Storage
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `rusqlite` | `bonsai-bot`, `bonsai-query` | `bonsai-db` |
| `libsqlite3-sys` | `bonsai-workspace` | `bonsai-db` (no C bindings) |
| `sqlx` | `bonsai-cas`, `bonsai-workspace`, `eternal-workshop` | `bonsai-db` |
| `sqlparser` | `bonsai-workspace` | `bonsai-db` (integrated SQL parser) |
| `datafrog` | `bonsai-query` | `bonsai-query` (already exists, absorb) |
| `automerge` | `bonsai-workspace` | `bonsai-crdt` (already exists, expand) |
| `hashbrown` | `bonsai-crdt` | `std::collections::HashMap` (std HashMap now uses hashbrown internally) |
| `indexmap` | `bonsai-sylva` | `bonsai-collection` |
| `dashmap` | 4 crates | `bonsai-sync` (sharded RwLock map) |
| `fs2` | `bonsai-bot` | `bonsai-fs` (file locking) |

#### Logging & Diagnostics
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `tracing` | 22 crates | `bonsai-log` |
| `tracing-appender` | `bonsai-workspace` | `bonsai-log` |
| `tracing-subscriber` | 3 crates | `bonsai-log` |
| `log` | `eternal-workshop` | `bonsai-log` |
| `env_logger` | `eternal-workshop` | `bonsai-log` |
| `anyhow` | 12 crates | `bonsai-error` |
| `thiserror` | 18 crates | `bonsai-error` (derive macro) |
| `once_cell` | 4 crates | `std::sync::OnceLock` (stable in Rust 1.70) |

#### System & Platform
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `sysinfo` | 4 crates | `bonsai-native` (expand existing) |
| `dirs` | 6 crates | `bonsai-native` |
| `libc` | 2 crates | `bonsai-native` (raw syscall wrappers) |
| `windows-sys` | `bonsai-runtime` | `bonsai-native` (Win32 bindings subset) |
| `notify` | 2 crates | `bonsai-fs` (inotify/kqueue/ReadDirectoryChanges) |
| `walkdir` | 2 crates | `bonsai-fs` |
| `globset` | `bonsai-workspace` | `bonsai-fs` |
| `tempfile` | 2 crates | `bonsai-fs` |
| `which` | 2 crates | `bonsai-native` |
| `keyring` | 2 crates | `bonsai-native` (platform credential store) |
| `enigo` | `bonsai-workspace` | `bonsai-native` (synthetic input) |
| `scrap` | `bonsai-workspace` | `bonsai-native` (screen capture) |
| `battery` | `bonsai-workspace` | `bonsai-native` |
| `portable-pty` | `bonsai-workspace` | `bonsai-native` (PTY + ConPTY) |
| `cc` | `bonsai-native` (build) | eliminate C entirely |

#### Data Processing & Formats
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `chrono` | 7 crates | `bonsai-time` |
| `uuid` | 14 crates | `bonsai-id` |
| `regex` | 2 crates | `bonsai-regex` (NFA/DFA engine) |
| `image` | 2 crates | `bonsai-image` (PNG/JPEG/WebP codec) |
| `gif` | `bonsai-workspace` | `bonsai-image` |
| `hound` | `bonsai-workspace` | `bonsai-audio` (WAV r/w) |
| `rodio` | `bonsai-workspace` | `bonsai-audio` (playback via WASAPI/CoreAudio) |
| `cpal` | `bonsai-workspace` | `bonsai-audio` |
| `symphonia` | `bonsai-workspace` | `bonsai-audio` (codec suite) |
| `lopdf` | `bonsai-workspace` | `bonsai-doc` (PDF parse/write) |
| `pulldown-cmark` | `bonsai-workspace` | `bonsai-markup` (Markdown AST) |
| `diffy` | `bonsai-workspace` | `bonsai-diff` (unified diff/patch) |
| `feed-rs` | `bonsai-workspace` | `bonsai-feed` (RSS/Atom parser) |
| `qrcode` | `bonsai-workspace` | `bonsai-qr` (QR matrix generator) |
| `zip` | `bonsai-workspace` | `bonsai-archive` (ZIP + deflate) |
| `scraper` | `bonsai-workspace` | `bonsai-dom` (HTML5 tree + CSS selector) |

#### AI & ML
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `candle-core` | `bonsai-go-nn`, `bonsai-workspace` | `bonsai-tensor` (n-dimensional array + ops) |
| `candle-nn` | `bonsai-go-nn` | `bonsai-nn` (layer primitives) |
| `candle-transformers` | `bonsai-go-nn` | `bonsai-transformer` (attention, MLP, KV cache) |
| `polars` | `bonsai-dataframe` | `bonsai-dataframe` (expand existing) |

#### Runtime & Scripting
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `wasmtime` | 3 crates | `bonsai-wasm` (custom WASM interpreter or vendored wasmtime) |
| `wasmtime-wasi` | `bonsai-runtime` | `bonsai-wasm` |
| `mlua` | `bonsai-workspace` | `bonsai-lua` (embed Lua 5.4 source, no luajit) |
| `wasm-encoder` | `bonsai-skill-compiler` | `bonsai-wasm-encode` (already in `bonsai-skill-compiler`) |

#### CLI
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `clap` | `bonsai-workspace` | `bonsai-cli` |

#### Communication Bots
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `serenity` | `bonsai-bot` | `bonsai-discord` (Discord Gateway+REST client) |
| `teloxide` | `bonsai-bot` | `bonsai-telegram` (Telegram Bot API client) |
| `matrix-sdk` | `bonsai-bot` | `bonsai-matrix` (Matrix CS API client) |
| `lettre` | 2 crates | `bonsai-mail` (SMTP client) |
| `async-imap` | `bonsai-bot` | `bonsai-mail` (IMAP client) |
| `async-native-tls` | `bonsai-bot` | `bonsai-tls` (TLS 1.3 via rustls or custom) |
| `governor` | `bonsai-bot` | `bonsai-rate` (token bucket rate limiter) |

#### Chess / Game AI
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `shakmaty` | `bonsai-chess` | `bonsai-chess` (expand — absorb move gen) |

#### Miscellaneous
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `bytemuck` | `bonsai-creator` | `bonsai-codec` (safe transmute helpers) |
| `fastrand` | `bonsai-workspace` | `bonsai-rand` |
| `unicode-normalization` | `bonsai-bot` | `bonsai-text` (NFC/NFD normalization) |
| `git2` | `bonsai-workspace` | `bonsai-git` (libgit2 bindings → pure-Rust git) |
| `specta` | 2 crates | `bonsai-specta` (TypeScript type export) |
| `specta-typescript` | `bonsai-workspace` | `bonsai-specta` |

#### Tauri & WebView (Long-term)
| Crate | Used By | Replacement Crate |
|-------|---------|-------------------|
| `tauri` | `bonsai-workspace` | `bonsai-shell` (Phase 6) |
| `tauri-build` | `bonsai-workspace` | `bonsai-shell` |
| `tauri-plugin-*` (7 plugins) | `bonsai-workspace` | absorbed into `bonsai-shell` |
| `tauri-specta` | `bonsai-workspace` | `bonsai-specta` |
| `wgpu` | `bonsai-creator` | `bonsai-gfx` (GPU abstraction) |

### 1.2 npm / Frontend Dependencies

| Package | Type | Replacement |
|---------|------|-------------|
| `@tauri-apps/api` | runtime | auto-generated from `bonsai-shell` IPC spec |
| `@tauri-apps/plugin-*` (5 pkgs) | runtime | auto-generated IPC bindings |
| `svelte` | framework | `bonsai-ui` (reactive component compiler) — Phase 5 |
| `vite` | bundler | `bonsai-bundle` (esbuild wrapper → custom bundler) — Phase 5 |
| `typescript` | compiler | keep (we generate `.d.ts`; replacing TSC is out of scope) |
| `monaco-editor` | editor | `bonsai-editor` (lightweight code editor) — Phase 3 |
| `xterm` + `xterm-addon-fit` | terminal | `bonsai-term` (VT100 canvas renderer) — Phase 3 |
| `marked` | Markdown | `bonsai-markup` TS bindings (from Rust WASM) |
| `dompurify` | sanitiser | `bonsai-sanitise` (tag allowlist, pure TS) |
| `@techstark/opencv-js` | vision | `bonsai-vision` (Rust WASM, replaces OpenCV) — Phase 4 |
| `playwright` | testing | `bonsai-e2e` (custom WebDriver client) |

---

## 2. New Custom Crates to Build — Master List

| Crate Name | Replaces | Phase | Complexity | LOC Est. |
|------------|----------|-------|------------|----------|
| `bonsai-executor` | `tokio`, `futures`, `async-trait` | 0 | Very High | 8,000 |
| `bonsai-json` | `serde_json` | 0 | Medium | 3,000 |
| `bonsai-serde` | `serde` + derive macro | 1 | Very High | 6,000 |
| `bonsai-codec` | `bincode`, `ciborium`, `base64`, `hex`, `bytemuck` | 1 | Medium | 2,000 |
| `bonsai-error` | `anyhow`, `thiserror` | 1 | Low | 800 |
| `bonsai-rand` | `rand`, `rand_core`, `rand_distr`, `fastrand` | 1 | Low | 1,200 |
| `bonsai-time` | `chrono` | 1 | Low | 1,500 |
| `bonsai-id` | `uuid` | 1 | Low | 600 |
| `bonsai-log` | `tracing`, `log`, `tracing-appender`, `tracing-subscriber`, `env_logger` | 1 | Medium | 2,500 |
| `bonsai-fs` | `walkdir`, `globset`, `notify`, `tempfile`, `fs2` | 1 | Medium | 2,000 |
| `bonsai-crypto` | `aes-gcm`, `argon2`, `blake3`, `ed25519-dalek`, `x25519-dalek`, `hmac`, `sha2`, `zeroize`, `bip39`, `ssh-key`, `zxcvbn` | 2 | High | 5,000 |
| `bonsai-net` | `bytes`, `local-ip-address`, `gethostname`, `tokio-socks` | 2 | Medium | 3,000 |
| `bonsai-http` | `reqwest`, `hyper`, `axum`, `tower-http` | 2 | High | 6,000 |
| `bonsai-tls` | `async-native-tls`, rustls (vendored) | 2 | High | 4,000 |
| `bonsai-websocket` | `tokio-tungstenite` | 2 | Medium | 2,000 |
| `bonsai-db` | `rusqlite`, `libsqlite3-sys`, `sqlx`, `sqlparser`, `tokio-rusqlite` | 2 | Very High | 12,000 |
| `bonsai-sync` | `arc-swap`, `dashmap` | 2 | Low | 1,000 |
| `bonsai-collection` | `indexmap`, `hashbrown` | 2 | Low | 800 |
| `bonsai-yaml` | `serde_yaml` | 2 | Medium | 2,000 |
| `bonsai-toml` | `toml` | 2 | Medium | 2,500 |
| `bonsai-cbor` | `ciborium` | 2 | Medium | 1,800 |
| `bonsai-regex` | `regex` | 3 | High | 5,000 |
| `bonsai-image` | `image`, `gif` | 3 | High | 6,000 |
| `bonsai-audio` | `hound`, `rodio`, `cpal`, `symphonia` | 3 | High | 7,000 |
| `bonsai-doc` | `lopdf` | 3 | Medium | 3,000 |
| `bonsai-markup` | `pulldown-cmark` | 3 | Medium | 2,500 |
| `bonsai-diff` | `diffy` | 3 | Medium | 1,500 |
| `bonsai-feed` | `feed-rs` | 3 | Low | 1,200 |
| `bonsai-qr` | `qrcode` | 3 | Medium | 1,500 |
| `bonsai-archive` | `zip` | 3 | Medium | 2,000 |
| `bonsai-dom` | `scraper` | 3 | High | 4,000 |
| `bonsai-mail` | `lettre`, `async-imap` | 3 | Medium | 3,500 |
| `bonsai-rate` | `governor` | 3 | Low | 600 |
| `bonsai-text` | `unicode-normalization` | 3 | Low | 800 |
| `bonsai-cli` | `clap` | 3 | Medium | 2,000 |
| `bonsai-native` | `sysinfo`, `dirs`, `libc`, `windows-sys`, `which`, `keyring`, `enigo`, `scrap`, `battery`, `portable-pty`, `cc` | 3 | High | 6,000 |
| `bonsai-tensor` | `candle-core`, `candle-nn`, `candle-transformers` | 4 | Very High | 15,000 |
| `bonsai-git` | `git2` | 4 | Very High | 10,000 |
| `bonsai-wasm` | `wasmtime`, `wasmtime-wasi` | 4 | Very High | 10,000 |
| `bonsai-lua` | `mlua` | 4 | High | 3,000 |
| `bonsai-discord` | `serenity` | 4 | Medium | 3,000 |
| `bonsai-telegram` | `teloxide` | 4 | Medium | 2,000 |
| `bonsai-matrix` | `matrix-sdk` | 4 | High | 4,000 |
| `bonsai-specta` | `specta`, `specta-typescript`, `tauri-specta` | 4 | Medium | 2,000 |
| `bonsai-gfx` | `wgpu` | 5 | Very High | 8,000 |
| `bonsai-shell` | `tauri` + all plugins + webview | 6 | Extreme | 30,000+ |
| `bonsai-bundle` | `vite` | 6 | Very High | 8,000 |
| `bonsai-ui` | `svelte` | 6 | Extreme | 20,000+ |
| `bonsai-editor` | `monaco-editor` | 3 | High | 5,000 |
| `bonsai-term` | `xterm` | 3 | High | 4,000 |
| `bonsai-sanitise` | `dompurify` | 1 | Low | 400 |
| `bonsai-vision` | `opencv-js` | 4 | Very High | 12,000 |

**Total estimated new code:** ~220,000 LOC across 50 new crates.

---

## 3. Phased Execution Plan

### Phase 0 — Offline First (Week 1–2)
**Goal:** Build entirely offline. Zero external network calls during `cargo build` or `npm ci`.

**Tasks:**
1. `cargo vendor vendor/` — copy all crate sources into the repo
2. Update `.cargo/config.toml` with `[source.vendored-sources]`
3. `npm ci --prefer-offline` baked into all scripts
4. CI: add `--offline` flag; block outbound connections in build sandbox
5. Add `vendor/` directory to `.gitignore` exceptions selectively (source only, no binaries)
6. Audit `vendor/` for GPL/LGPL licences that conflict with Bonsai's licence

**Output:** Fully offline build verified on a machine with no internet access.

---

### Phase 1 — Foundation Crates (Month 1–3)
**Goal:** Own all utilities that touch every other crate — errors, logging, ID generation, time, random, encoding, and filesystem.

These are low-risk, low-complexity, and unblock everything else. Build them first so later phases can depend on them instead of external crates.

#### `bonsai-error` (replaces `anyhow` + `thiserror`)
- `BonsaiError<K>` enum with a `kind` discriminant and structured context chain
- `#[derive(BonsaiError)]` proc-macro generating `Display`, `From`, and `source()` impls
- Eliminates the `anyhow::bail!` / `thiserror::Error` split
- All 30 crates currently using `anyhow` or `thiserror` migrate to `bonsai-error`

#### `bonsai-log` (replaces `tracing` + `log` + subscribers)
- Structured key-value log lines: `log!(level: info, component: "cas", hash: &h)`
- Ring-buffer in-memory log for UI display (already surfaced in `SystemHealthPanel`)
- File appender with rotation; stderr fallback
- `log` crate facade: implement the `log::Log` trait so legacy code (`log::info!`) still works
- Span context via thread-local (no subscriber architecture needed)
- 22 crates migrate from `tracing::info!` to `bonsai_log::info!`

#### `bonsai-rand` (replaces `rand` + `rand_core` + `rand_distr` + `fastrand`)
- `OsRng`: reads `/dev/urandom` on Linux/Mac, `BCryptGenRandom` on Windows
- `Xoshiro256StarStar` PRNG seeded from `OsRng`
- `SecureRng`: cryptographically secure, wraps `OsRng`
- Distributions: `Uniform`, `Normal`, `Bernoulli`, `Dirichlet` (needed by `bonsai-go-nn`)
- `#[no_std]` compatible core; optional `std` feature for file-based seeding

#### `bonsai-time` (replaces `chrono`)
- `Timestamp`: UTC nanoseconds since epoch, stored as `i64`
- `Date`, `Time`, `DateTime`: calendar arithmetic without timezone database (UTC + fixed offset only)
- ISO 8601 formatting and parsing
- `Duration` arithmetic
- No IANA timezone support in v1 — use UTC offsets only

#### `bonsai-id` (replaces `uuid`)
- `BonsaiId`: 128-bit value, UUID v4 compatible wire format
- Generation: uses `bonsai-rand` `OsRng`
- Display: lowercase hex with dashes; `Display`, `FromStr`, `Serialize`/`Deserialize` via `bonsai-serde`
- `ShortId`: 8-char base62 (good for log correlation IDs)

#### `bonsai-codec` (replaces `base64`, `hex`, `bincode`, `bytemuck`)
- `base64::{encode, decode}` — standard and URL-safe alphabets
- `hex::{encode, decode}` — lowercase/uppercase
- `ByteFrame`: length-prefixed binary frame encode/decode (replaces `bincode` for `bonsai-p2p`)
- `safe_transmute`: compile-time-checked byte reinterpretation (replaces `bytemuck`)

#### `bonsai-fs` (replaces `walkdir`, `globset`, `notify`, `tempfile`, `fs2`)
- `walk(path) -> impl Iterator<Item=DirEntry>` — recursive directory traversal
- `Glob`: compile a glob pattern to an NFA matcher
- `Watcher`: file-system change notifications (inotify on Linux, ReadDirectoryChangesW on Windows, kqueue on macOS)
- `TempDir` / `TempFile`: RAII temporary paths backed by `std::fs`
- `FileLock`: advisory lock via `flock`/`LockFileEx`

#### `bonsai-sanitise` (replaces `dompurify`, TypeScript)
- ~400 LOC TypeScript allowlist sanitiser
- Configurable: allowed tags, attributes, URL schemes
- Replaces the runtime DOMPurify import in the Svelte frontend

**Phase 1 exit criteria:** `cargo check --workspace` with `anyhow`, `thiserror`, `tracing`, `log`, `rand`, `chrono`, `uuid`, `base64`, `hex`, `walkdir`, `globset`, `notify`, `tempfile` removed from all `Cargo.toml` files.

---

### Phase 2 — Crypto, Networking, Data (Month 4–8)
**Goal:** Own the security-critical and network-critical layers. These touch the trust boundary.

#### `bonsai-crypto` (replaces 11 crypto crates)
This is security-critical. Strategy: implement from RFC specifications, validate against published test vectors, and conduct an internal review before deploying.

**Primitives to implement:**

- **SHA-2 family** (SHA-256, SHA-512): FIPS 180-4. ~300 LOC. Test with NIST vectors.
- **BLAKE3**: Copy the reference pure-Rust implementation (`blake3` crate is already pure Rust — vendor the source directly, then over time simplify the SIMD paths we don't need). ~2,000 LOC with SSE2.
- **AES-GCM**: AES-128/256 using AES-NI intrinsics on x86, software fallback on ARM. GHASH polynomial multiplication. ~1,500 LOC.
- **ChaCha20-Poly1305**: Stream cipher + authenticator. ~800 LOC. Needed for TLS 1.3.
- **Argon2id**: Memory-hard KDF per RFC 9106. ~600 LOC. Test with Argon2 reference implementation vectors.
- **X25519**: Diffie-Hellman on Curve25519. Implement Montgomery ladder. ~500 LOC.
- **Ed25519**: Signature using Edwards curve. Implement scalar multiplication and point compression. ~800 LOC. Test with RFC 8032 vectors.
- **HMAC-SHA256**: ~50 LOC wrapping SHA-256.
- **BIP-39**: Mnemonic wordlist (2048 words) + entropy-to-mnemonic + mnemonic-to-seed. ~400 LOC.
- **Zeroize**: `ZeroOnDrop<T>` wrapper that calls `write_volatile` over bytes on drop. ~100 LOC.
- **Password strength**: Port `zxcvbn` algorithm (frequency lists + patterns). ~1,500 LOC.
- **SSH key format**: Ed25519 OpenSSH PEM encode/decode. ~300 LOC.

**Security policy:** No custom algorithm inventions. All implementations are transcriptions of published standards. Deviations from the spec are bugs.

#### `bonsai-net` (replaces `bytes`, socket helpers)
- `BytesMut`: growable byte buffer with split/unsplit; replaces `bytes::BytesMut`
- TCP stream wrapper with async read/write using `bonsai-executor`
- UDP socket wrapper
- SOCKS5 proxy connector (used by `bonsai-p2p`)
- `NetworkInterface`: enumerate local IP addresses (replaces `local-ip-address`)

#### `bonsai-http` (replaces `reqwest`, `hyper`, `axum`, `tower-http`)
- **Parser:** HTTP/1.1 request and response parser (headers, chunked encoding, keep-alive)
- **Client:** async HTTP client with connection pooling, redirect following, and timeout
- **Server/Router:** `axum`-compatible router API: `Router::new().route("/path", get(handler))`
- **Middleware:** logging, CORS, rate limiting, body size limits
- **TLS:** delegate to `bonsai-tls`
- Phase 2 target: feature-parity with the API surface currently used in `bonsai-daemon` and `bonsai-bot`

#### `bonsai-websocket` (replaces `tokio-tungstenite`)
- RFC 6455 WebSocket framing: handshake upgrade, frame encode/decode, masking, ping/pong
- Client and server modes
- Uses `bonsai-net` TCP and `bonsai-http` upgrade negotiation
- `bonsai-daemon` currently uses this for the assistant streaming endpoint

#### `bonsai-db` (replaces `rusqlite`, `libsqlite3-sys`, `sqlx`, `sqlparser`, `tokio-rusqlite`)
This is the most complex Phase 2 component.

**Architecture:**
```
bonsai-db
├── storage/       B-tree pager (fixed 4096-byte pages, WAL journal)
├── parser/        Recursive-descent SQL parser (SELECT/INSERT/UPDATE/DELETE/CREATE/DROP)
├── planner/       Query plan: full scan, index scan, nested loop join
├── executor/      Tuple-at-a-time evaluation engine
├── index/         B-tree index on arbitrary column sets
├── types/         NULL, INTEGER, REAL, TEXT, BLOB, TIMESTAMP
├── async/         Spawn-blocking wrapper (replaces tokio-rusqlite)
└── migrate/       Schema migration runner (replaces sqlx::migrate!)
```

**Compatibility target:** SQLite-compatible wire format for existing `.db` files. The on-disk B-tree layout follows SQLite's documented format so existing databases (survival KB, memory nodes) can be opened without conversion.

#### `bonsai-sync` (replaces `arc-swap`, `dashmap`)
- `ArcSwap<T>`: atomic pointer swap without locking readers; same API as `arc-swap`
- `ShardMap<K,V>`: 64-shard `RwLock<HashMap>` (replaces `dashmap`)
- `WatchCell<T>`: single-writer multi-reader value with change notification

#### `bonsai-tls` (replaces `async-native-tls`)
- TLS 1.3 client handshake using `bonsai-crypto` primitives (X25519 key exchange, ChaCha20-Poly1305 or AES-GCM record encryption, Ed25519 for certificate verification)
- Certificate parsing: DER/PEM X.509 v3
- System root store loading (Windows CertStore, macOS Keychain, Linux /etc/ssl)
- Server mode for `bonsai-http` and `bonsai-websocket`
- **Note:** TLS is complex and security-critical. In v1 we vendor `rustls` (pure Rust, no C) and treat it as a trusted component. Custom TLS comes in v2 after the crypto crate is fully audited.

#### `bonsai-yaml` + `bonsai-toml` + `bonsai-cbor`
- YAML: recursive descent scanner per YAML 1.2 spec; replaces `serde_yaml` usage in `bonsai-skill-compiler`
- TOML: v1.0 parser; replaces `toml` usage in `bonsai-workspace`
- CBOR: RFC 7049 encoder/decoder; replaces `ciborium` usage in `bonsai-actors`

**Phase 2 exit criteria:** No calls to `reqwest`, `hyper`, `axum`, `rusqlite`, `sqlx`, `arc-swap`, `dashmap`, `tokio-tungstenite`, or any crypto crate in workspace `Cargo.toml`.

---

### Phase 3 — Content, UI Components, Platform (Month 9–15)
**Goal:** Own media, text processing, platform integration, and the lightweight UI components.

#### `bonsai-regex` (replaces `regex`)
- Thompson NFA construction from regex syntax (character classes, repetition, alternation, anchors, capture groups)
- DFA compilation via subset construction with lazy compilation
- `Match`, `Captures`, `FindAll` iterators
- Unicode-aware character classes (`\w`, `\d`, `\s`)
- Zero-copy API: match against `&[u8]` or `&str`

#### `bonsai-image` (replaces `image`, `gif`)
- PNG: deflate decoder (DEFLATE = LZ77 + Huffman), IHDR/IDAT/IEND chunk parsing, all filter types
- JPEG: Huffman decoder, IDCT, YCbCr→RGB conversion
- WebP: lossless VP8L decoder
- GIF: LZW decoder, frame compositing
- Pixel format conversions: RGB/RGBA/Grayscale, u8/u16/f32
- Write support: PNG and JPEG only in v1

#### `bonsai-audio` (replaces `hound`, `rodio`, `cpal`, `symphonia`)
- **Platform I/O:** WASAPI (Windows), CoreAudio (macOS), ALSA (Linux) raw bindings in `bonsai-native`
- **WAV:** RIFF chunk parser, PCM/float32 codec (replaces `hound`)
- **MP3:** Layer III decoder (replaces `symphonia`)
- **Opus:** reference implementation vendored (pure C, audited; integrate via `cc` until pure-Rust replacement exists)
- **Playback engine:** push-based audio graph with mixer, volume, seek (replaces `rodio`)

#### `bonsai-dom` (replaces `scraper`)
- HTML5 tree builder (tokeniser + parser per the WHATWG spec)
- CSS selector engine: tag, class, ID, attribute, pseudo-class, combinators
- DOM query API: `document.select("div.foo > span")` → `Vec<NodeRef>`

#### `bonsai-markup` (replaces `pulldown-cmark`)
- CommonMark 0.31 compliant parser
- AST: `Block` enum (paragraph, heading, code, list, blockquote, table, hr)
- HTML renderer and plain-text renderer
- WASM compilation target for `bonsai-sanitise` (frontend use)

#### `bonsai-editor` (replaces `monaco-editor`)
- Canvas-rendered code editor with a virtual line buffer
- Syntax highlighting via Tree-sitter (already a workspace dependency; vendor it)
- Themes: minimal dark/light
- Features: cursor, selection, multi-caret, find/replace, line numbers, bracket matching
- Svelte component wrapper: `<BonsaiEditor bind:value language="rust" />`
- Target: replaces Monaco for all Sylva/Titan/skill editing panels

#### `bonsai-term` (replaces `xterm` + `xterm-addon-fit`)
- VT100/VT220/xterm-256 escape sequence parser
- Canvas renderer: glyph atlas, 256-colour palette, bold/italic/underline
- Svelte component: `<BonsaiTerm rows bind:cols on:data />`
- Used by the embedded terminal in the IDE panel

#### `bonsai-cli` (replaces `clap`)
- `#[derive(Args)]` proc-macro (using `syn` + `quote` — both vendored)
- Argument types: positional, flag, option, subcommand
- Auto-generated `--help` text from doc comments
- Shell completion generation for bash/zsh/PowerShell

#### `bonsai-native` (replaces `sysinfo`, `dirs`, `libc`, `windows-sys`, `battery`, `enigo`, `scrap`, `portable-pty`, `keyring`, `which`)
- **System info:** CPU usage, memory, processes, temperatures — direct `/proc` reads on Linux, `PDH` on Windows, `sysctl` on macOS
- **Directories:** config, cache, data, home — per-platform (XDG / Windows known folders / macOS containers)
- **Synthetic input:** `enigo`-compatible keyboard/mouse injection via Win32 `SendInput`, X11 `XSendEvent`, macOS CGEvent
- **Screen capture:** `DXGI` desktop duplication (Windows), `CGDisplayStream` (macOS), `XCB/XShmGetImage` (Linux)
- **PTY:** Windows ConPTY via `CreatePseudoConsole`, POSIX `posix_openpt`
- **Keyring:** Windows Credential Manager, macOS Keychain, Linux Secret Service (D-Bus)
- **Raw syscalls:** Thin safe wrappers — replaces `libc` and `windows-sys` in `bonsai-ring` and `bonsai-runtime`

**Phase 3 exit criteria:** `monaco-editor` and `xterm` removed from `package.json`. All `sysinfo`, `dirs`, `notify`, `walkdir`, `regex`, `chrono`, `image` crates removed from `Cargo.toml`.

---

### Phase 4 — AI, ML, Git, WASM, Bots (Month 16–24)
**Goal:** Own every layer of the intelligence stack and all protocol-level integrations.

#### `bonsai-tensor` (replaces `candle-core`, `candle-nn`, `candle-transformers`)
This is the foundation of BonsAI's on-device inference. Build it to replace `candle`, which itself is already a lean pure-Rust ML framework, but we want full ownership.

**Architecture:**
```
bonsai-tensor
├── storage/   StridedTensor<T>: shape + strides + Arc<Vec<T>>
├── ops/       matmul, softmax, layernorm, relu, gelu, rope, flash-attn (CPU)
├── nn/        Linear, Embedding, RMSNorm, MultiHeadAttention, MLP
├── gguf/      GGUF file parser (replaces llama.cpp dependency for model loading)
├── quant/     Q4_K_M, Q8_0 dequantization
└── infer/     Qwen2/LLaMA/Mistral transformer forward pass with KV cache
```

**Why own this:** The BonsAI training pipeline (DPO, SFT) already runs in Python. Once `bonsai-tensor` is mature, we rewrite the training loop in Rust too, eliminating `torch` + `peft` + Python entirely for inference. The GGUF inference server (`llama-server`) can be replaced by a `bonsai-tensor` binary.

#### `bonsai-git` (replaces `git2`)
- `git2` binds to C libgit2. Replace with a pure-Rust implementation:
  - Pack file reader/writer (delta compression, OFS/REF deltas)
  - Object store: blob, tree, commit, tag (SHA-1 and SHA-256)
  - Reference store: packed-refs + loose refs
  - Index file format
  - Config parser (replaces `git2::Config`)
  - Basic operations: init, add, commit, log, diff, checkout, push/pull (smart HTTP)
- `bonsai-workspace` uses `git2` for the skill versioning and repo management panels

#### `bonsai-wasm` (replaces `wasmtime`, `wasmtime-wasi`)
- WASM binary parser: section types, function bodies, imports/exports
- Interpreter: stack machine, linear memory, function calls, table, globals
- WASI snapshot preview1: file I/O, env, clock, random
- JIT optional: baseline single-pass x86-64 compiler for hot paths
- `bonsai-skills` and `bonsai-runtime` use WASM for sandboxed skill execution — this is the primary use case

#### `bonsai-discord`, `bonsai-telegram`, `bonsai-matrix` (replace bot SDKs)
These are protocol-level HTTP/WebSocket clients. With `bonsai-http` and `bonsai-websocket` in place, implementing them is straightforward:
- **Discord:** Gateway v10 (WebSocket), REST API v10, voice gateway (OPUS over UDP)
- **Telegram:** Bot API v7 long-polling + webhook modes
- **Matrix:** Client-Server API r0.6, room sync, E2E (using `bonsai-crypto` Olm/Megolm)

#### `bonsai-vision` (replaces `opencv-js`)
- Pure-Rust WASM module replacing the 8MB `opencv.js` bundle
- Core ops: image resize (bilinear/bicubic), crop, rotate, colour convert, threshold
- Face detection: HaarCascade or lightweight CNN model (bonsai-tensor)
- QR/barcode decode: replaces `qrcode` scan capability
- Used by the Android USB frame analysis and the vision attachment pipeline

**Phase 4 exit criteria:** `candle-*`, `git2`, `wasmtime`, `mlua`, `serenity`, `teloxide`, `matrix-sdk`, `lettre`, `async-imap` removed.

---

### Phase 5 — Async Runtime (Month 25–30)
**Goal:** Replace `tokio` entirely — the last and hardest infrastructure dependency.

#### `bonsai-executor` (replaces `tokio`, `futures`, `async-trait`)

`tokio` is used in 21 crates. This is the most invasive replacement in the plan.

**Architecture:**
```
bonsai-executor
├── reactor/     I/O event loop
│   ├── iocp/    Windows I/O Completion Ports
│   ├── epoll/   Linux epoll
│   └── kqueue/  macOS kqueue
├── task/        Future poll loop, Waker implementation, task queue
├── runtime/     Thread-pool scheduler (work-stealing, N threads = CPU count)
├── sync/        Mutex, RwLock, Semaphore, Channel, Notify, Barrier
├── io/          AsyncRead, AsyncWrite, TcpStream, UdpSocket, UnixSocket
├── time/        Sleep, Interval, Timeout
├── fs/          Spawn-blocking file I/O
└── net/         TcpListener, UdpSocket (wraps reactor I/O)
```

**Migration strategy:**
1. `bonsai-executor` implements the exact `tokio` API surface (same trait names, same macro names)
2. Replace `tokio = { ... }` with `bonsai-executor = { ... }` in `Cargo.toml`
3. Use `extern crate bonsai_executor as tokio;` for zero code-change migration
4. Once all crates compile against `bonsai-executor`, delete the alias and clean up

**Why defer to Phase 5:** Every other phase depends on async I/O working correctly. Replacing the runtime is a flag-day change — it must happen atomically across all 21 crates. Doing it after all other crates are migrated minimises integration surface.

---

### Phase 6 — Shell & UI Framework (Month 30–48)
**Goal:** Replace Tauri + Svelte with a fully owned windowing and UI framework.

**This phase is very long-term. It should not block any other work.**

#### `bonsai-shell` (replaces `tauri` + all plugins)
- **Window management:** `winit` (vendored) for cross-platform windowing, or raw Win32 `CreateWindowEx` on Windows
- **WebView:** System webview (WebView2 on Windows, WKWebView on macOS, WebKitGTK on Linux) via raw COM/ObjC/GObject bindings — same approach as Tauri's `wry`, but owned
- **IPC:** Postmessage bridge, same JSON protocol as current Tauri IPC — zero frontend changes
- **Plugins:** File system, dialog, notification, barcode scanner, shell command — all re-implemented using `bonsai-native` and `bonsai-fs`
- **Build pipeline:** Resource bundler, icon packing, NSIS/MSI/AppImage packaging

#### `bonsai-ui` (replaces `svelte`)
- Reactive component compiler targeting vanilla DOM JS
- Single-file component format: `<script>`, `<style>`, template
- Reactive statements: `$: derived = base + 1`
- Two-way binding: `bind:value`
- Slot/children composition
- TypeScript support via the vendored TypeScript compiler
- Migration: component-by-component; Svelte and `bonsai-ui` components can coexist during transition

---

## 4. Replacement Lifecycle (Per Crate)

Every replacement follows this exact process — no exceptions:

```
1. SPEC     Write a spec doc in docs/specs/<crate>.md defining:
            - API surface (public types, traits, functions)
            - Behaviour invariants
            - Performance targets
            - Test vectors (for crypto/codec/parser crates)

2. SCAFFOLD cargo new --lib crates/<crate> with workspace inheritance

3. IMPLEMENT Write the implementation against the spec.
             Use the DeepSeek/BonsAI agents to generate initial code.
             Human review for security-critical crates.

4. TEST      Unit tests covering spec invariants.
             Fuzz testing for parsers and crypto.
             Benchmark against the original crate (criterion).

5. INTEGRATE Replace one crate at a time. Use feature flags during transition:
             [dependencies]
             bonsai-json = { path = "../bonsai-json" }
             # serde_json = "..." # REMOVED

6. VERIFY    cargo test --workspace; cargo check --workspace; full CI pass.

7. DELETE    Remove old dependency from all Cargo.toml files. cargo vendor sync.

8. SURVIVE   Add patterns and fixes to survival_fixes_registry.json for any bugs
             found during migration. Seed them into bonsai-watchdog SEEDED_FIXES.
```

---

## 5. Tooling — sccache Configuration

`sccache v0.15.0` is now installed and active via `.cargo/config.toml`:

```toml
[build]
rustc-wrapper = "sccache"
```

To check cache stats after a build:
```powershell
sccache --show-stats
```

To set a larger cache directory (default is `~/.cache/sccache`, 10 GB):
```powershell
[Environment]::SetEnvironmentVariable("SCCACHE_CACHE_SIZE", "30G", "User")
[Environment]::SetEnvironmentVariable("SCCACHE_DIR", "C:\sccache", "User")
```

For distributed cache (team sharing) — configure an S3-compatible backend later:
```toml
# ~/.config/sccache/config
[cache.s3]
bucket = "bonsai-sccache"
region = "auto"
endpoint = "http://minio.internal:9000"
```

---

## 6. Effort Estimates

| Phase | Duration | Crates Built | LOC | Risk |
|-------|----------|--------------|-----|------|
| 0 — Vendor | Week 1–2 | 0 new | 0 | Low |
| 1 — Foundation | Month 1–3 | 8 crates | ~12,000 | Low |
| 2 — Crypto/Net/DB | Month 4–8 | 12 crates | ~35,000 | High |
| 3 — Content/UI/Platform | Month 9–15 | 15 crates | ~50,000 | Medium |
| 4 — AI/Git/WASM/Bots | Month 16–24 | 10 crates | ~45,000 | High |
| 5 — Async Runtime | Month 25–30 | 1 crate | ~12,000 | Very High |
| 6 — Shell & UI Framework | Month 30–48 | 3 crates | ~60,000 | Extreme |
| **Total** | **~4 years** | **~50 crates** | **~214,000** | — |

With BonsAI-assisted code generation, the AI writes the initial implementation from the spec, humans review and fix. Realistic multiplier: **2-3× faster** than pure human effort. Target: full sovereignty in ~18 months at full pace with AI augmentation.

---

## 7. Immediate Next Steps (This Week)

1. **`cargo vendor`** — run, commit, verify offline build
2. **`bonsai-error`** — first crate to build; unblocks logging and all other crates
3. **`bonsai-log`** — replace `tracing` in `eternal-workshop` as the first migration
4. **`bonsai-rand`** — replaces `rand` in `bonsai-transfer-crypto` first
5. **`bonsai-id`** — replaces `uuid` in `eternal-workshop` as the first migration
6. **Write specs** for Phase 2 crates (`bonsai-crypto`, `bonsai-db`, `bonsai-http`) in `docs/specs/`

Each new crate is a training lesson for the BonsAI model. The model is trained on the codebase it inhabits — every custom replacement teaches it the project's own idioms, making future code generation more accurate and more aligned with the sovereignty philosophy.

---

*This document is the canonical reference for the BonsAI dependency sovereignty effort. Update it as crates are completed and phases close.*
