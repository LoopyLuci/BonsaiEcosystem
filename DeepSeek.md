# DeepSeek Handbook

## Current State & Progress

Merged baseline now includes the recent launcher/runtime hardening and feature rollouts:

- PR #20 merged on main for launcher and model readiness reliability.
- Vite launcher flow stabilized for Tauri startup path.
- Slot readiness race reduced with health-probe fallback and assistant retry behavior.
- Model Data system integrated for richer model metadata and model UX.
- Quick Options and task queue UI/status integration landed.
- BonsaiBot multi-platform integration path is available for Discord, Telegram, Email, and Matrix.

## Key Features & User Flows

### Model Data

- Model entries include enriched metadata such as capability strengths, context window, tier, and RAM estimates.
- User flow: open Model Selector, inspect metadata, load/select a model, confirm Active state.

### Quick Options

- Chat area includes quick-launch prompts for Weather, Time, Files, Sys Stats, and Web.
- User flow: pick option, review prefilled prompt, send, and inspect response/activity log.

### Task Queue

- Runtime queue tracks pending and active workloads with visible status on the bottom bar.
- User flow: submit one or many prompts, monitor queue counts, observe queue drain as work completes.

### BonsaiBot

- Messaging adapters support Discord, Telegram, Email, and Matrix.
- User flow: configure tokens/credentials, run bot service, validate health and platform status via admin API.

## Build, Test & Deployment

### Local Build

- Primary launcher:
  - `node bonsai-workspace/src/launch-all.mjs --mode desktop`

### Desktop Artifact Builder

- Use the dedicated builder scripts from repository root:
  - `BonsaiExeLauncherBuilder.ps1`
  - `BonsaiExeLauncherBuilder.cmd`

These scripts standardize frontend plus Tauri build orchestration and artifact output handling.

### Validation Checklist

- `cargo check` in relevant Rust crates.
- Targeted runtime/feature tests for new behavior.
- Launcher smoke validation for Vite + Tauri startup.

## 19. Development Cycle Closeout - 2026-05-04

### Merged PRs (This Cycle)
| PR | Title | Status |
|----|-------|--------|
| #19 | Warmup crash fix (--no-warmup) | ✅ Merged |
| #20 | Vite launcher crash + slot-ready race fix | ✅ Merged |
| #21 | Documentation update (README, manual, handbook) | ✅ Merged |
| #22 | Clojure/Python P1 hardening (4 commits) | ✅ Merged |
| #23 | GPU crash auto-fallback to CPU | ✅ Merged |
| #24 | Inference mode system + notification UX + terminal fix | ✅ Merged |

### Build Status
- `cargo check -p bonsai-workspace` - clean
- `npm run build` - clean

### Unstaged Artifacts
- latest.json, preflight-cache.json, BonsaiWorkspace.exe, .kotlin/

---

## 20. Comprehensive Audit Implementation Complete - 2026-05-05

All items from the comprehensive security and quality audit (P0–P3) are implemented and merged into `main`.

### Merged PRs

| PR | Batch | Contents |
|----|-------|----------|
| #25 | P0 — Critical Security | Path traversal fix (`globset` allowlist on `/run`), prompt injection sanitiser, Tauri CSP tightened to `default-src 'self'`, reqwest redirect policy set to `none` |
| #26 | P1 — High Priority | `chrono` deprecated UTC offset fixed, model orchestrator tests, assistant policy enforcement tests, admin API bound to `127.0.0.1` |
| #27 | P2 — Medium Priority | File-logging with `tracing-appender`, port manager with random fallback, Python `/run` endpoint, CI `cargo audit` + `cargo clippy` jobs |
| #28 | P3 — Future / Quality | Structured JSON logging (daily rotation), unified `BonsaiError` enum (12 variants, Tauri-serialisable), STRIDE threat model (`docs/threat-model.md`), Buddy API contract (`docs/api-contract.md`), Windows Job Objects for Python resource limits (`bonsai-runtime`) |

### Security Vulnerabilities Fixed (P0/P1: 7 total)
- **P0-1** Path traversal via `/run` endpoint — `globset` allowlist
- **P0-2** Prompt injection — `sanitizer.rs` strips `<tool_call>` injection patterns
- **P0-3** WebView CSP too permissive — tightened to `default-src 'self'`
- **P0-4** SSRF via redirect following — `reqwest` redirect policy → `none`
- **P1-1** `chrono` `Local::now()` deprecated offset — migrated to `Utc::now()`
- **P1-3** Admin API accessible on `0.0.0.0` — bound to `127.0.0.1` only
- **P2-3** Python `/run` missing endpoint — implemented with allowlisted path validation

### Tests Added
- Model orchestrator unit tests (load / switch / concurrent access)
- Assistant policy enforcement tests (tool gate, scope enforcement)
- bonsai-bot integration tests fixed (axum 0.7 serve, `CircuitBreakerConfig` fields)

### Documentation Added
- `docs/threat-model.md` — STRIDE methodology, 8 threats (T-1–T-8), risk matrix, review cadence
- `docs/api-contract.md` — Full Buddy API spec: endpoints, `bonsai_ext` confirmation protocol, SSE streaming, error envelope

### Code Quality Improvements
- `tracing` migration from `println!` across hot paths
- `globset`-based path allowlisting replaces ad-hoc string checks
- Unified `BonsaiError` type eliminates `String` errors in Tauri commands

### Runtime Improvements
- Windows Job Objects: hard CPU-time and memory limits applied to Python worker PIDs via `bonsai-runtime` (`create_job_for_pid`)
- POSIX `resource.setrlimit` on Linux/macOS
- Port manager with random fallback (avoids collision on busy systems)
- Log rotation: daily rolling files via `tracing-appender`

### Infrastructure
- CI: `cargo audit` job added (RUSTSEC advisory check)
- CI: `cargo clippy -- -D warnings` job added
- All three crates compile clean (no errors, no warnings promoted to errors)

### Deferred Items
- **P3-6**: Nonce-based CSP for inline scripts — requires Tauri WebView nonce injection; XL effort
- **P3-7**: Playwright E2E smoke suite — requires CI browser environment setup; XL effort

### Final Build Status (2026-05-05)
- `bonsai-workspace/src-tauri` — `cargo check` ✅ clean, `cargo test` 137 passed
- `bonsai-bot` — `cargo check` ✅ clean, `cargo test` 26 passed (23 unit + 3 integration)
- `bonsai-runtime` — `cargo check` ✅ clean, `cargo test` 2 passed

### Unstaged Artifacts
- latest.json, preflight-cache.json, BonsaiWorkspace.exe, .kotlin/, runtimes/python/__pycache__/
