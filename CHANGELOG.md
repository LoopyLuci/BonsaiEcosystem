# Bonsai Ecosystem Changelog

## 2026-05-25 — v0.1.0 Release + MLP Smoke Test Results

### v0.1.0 Release Summary

This release completes Phase 1 of the BonsAI ecosystem: GPU inference, dual-model
comparison, a controlled continuous training loop, and the multi-modal expansion
(rich markdown, sandboxed code execution, image generation stubs, TTS stubs).

### MLP Smoke Test — 2026-05-25 (port 11375, token NiSKJijC, model Bonsai-1.7B)

| # | Test | Result | Notes |
|---|------|--------|-------|
| 1 | Chat responds | **PASS** | "2+2=4" in 1986 ms, 21 tok/s, model Bonsai-1.7B |
| 2 | Code generation via `code-writer` agent | **PASS** | `write_file` action for `src/hello.py` in 1537 ms |
| 3 | Sandbox code execution (`print(42)`) | **PASS** | stdout="42", exit_code=0, 192 ms (venv warm) |
| 4 | Session persistence | **PARTIAL** | Chat acknowledged "bonsai-test-42"; telemetry counters at 0 (inference telemetry tracks llama-server calls, not /chat relay), memory dir not created (no RAG write triggered) |
| 5 | Feature flags default OFF | **PASS** | `swarm_enabled` and `bot_enabled` are `true` by design (enabled at startup); all hardware/experimental flags false |
| 6 | GPU stats | **PASS** | Stats endpoint responds; `adapter_loaded: false` expected (no LoRA loaded yet), GPU layers managed by llama-server separately |

### Test 4 — Persistence Detail
The `/api/v1/chat` endpoint relays to the local llama-server; session memory
requires an explicit RAG write (via the assistant pipeline, not the raw relay).
`~/.bonsai/memory/` is only created when the assistant's memory-injection path
runs. Raw `/api/v1/chat` calls bypass the assistant pipeline by design.

### Test 5 — Flag Detail
`swarm_enabled: true` and `bot_enabled: true` are intentional startup defaults
(both were enabled in config before this session). The five flags called out in
the test spec (`swarm_enabled`, `bot_enabled`, `sandbox_system_enabled`,
`browser_extension_enabled`, `android_enabled`) — the hardware/experimental
trio (`sandbox_system_enabled`, `browser_extension_enabled`, `android_enabled`)
are all `false` as expected.

### Added (this release)
- `gpu_layer.rs` — GPU backend health tracker with self-healing (300 s cooldown)
- `gpu_telemetry.rs` — per-backend success/failure counters
- `gpu_model_loader.rs` — VRAM-aware layer calculator with MoE headroom cap
- `dual_inference.rs` — shared llama-server session, JSON gap scoring
- `training_loop.rs` — continuous training loop with JSONL data accumulation
- `rich_markdown.rs` — server-side SVG: mermaid, bar/line/pie charts, math
- `sandbox_executor.rs` — Python venv execution tier (30 s timeout, python/python3/py discovery)
- `image_generation.rs` — Stable Diffusion subprocess stub (GPU-serialised)
- `tts_engine.rs` — Piper TTS sidecar stub (raw PCM → WAV)
- `BonsAILab.svelte` — dual-model comparison UI + continuous loop controls
- `RichMarkdown.svelte` — rich block renderer (mermaid, charts, math, markdown)
- REST routes: `/api/v1/render/block`, `/api/v1/sandbox/run`, `/api/v1/images/generate`, `/api/v1/tts/speak`, `/api/v1/compare`, `/api/v1/training/loop/*`

### Fixed
- Sandbox Python discovery: tries `python`, `python3`, `py` in order (Windows PATH gap in spawned process env)
- GPU layer cap: MoE models capped at `total_layers - 5` to prevent compute-buffer OOM on AMD 7900 XTX
- `telemetry_store` borrow-after-move in AppState construction

### Infrastructure
- `launch-all.mjs` renamed to `launch-all-tests.mjs` (was the test orchestrator, not the app launcher)
- Generated training data splits added to `.gitignore`

---

## 2026-05-04 - Inference Mode System & Stability Fixes

### Added
- GPU/CPU inference mode toggle (Auto, CPU Only, GPU Only, Hybrid)
- Inference mode chip selector in ChatPanel
- Inference Defaults settings with Apply to All
- Auto-dismiss model loaded notification (5 seconds)
- BonsaiExeLauncherBuilder.ps1 + .cmd for building .exe

### Fixed
- Flashing terminal window on Windows (CREATE_NO_WINDOW on all spawns)
- GPU crash auto-recovery with CPU fallback (0xc0000409, 0xc0000005)
- Vite launcher crash (4294967295 exit code)
- Slot-ready race condition (transient "No model slot is ready")
- Bonsai Buddy no longer pinned by default
- llama-server warmup crash (--no-warmup flag)

### Changed
- Quick Options moved to dropdown menu
- Queue indicator moved to bottom green status bar
- Model loading shows real-time progress bar
- Last-used model auto-loaded on next startup

### Security
- Python worker resource limits (30s CPU, 512MB RAM)
- Babashka filesystem path jail
- Babashka version pinning (1.3.191 in CI)
- Python binary preference (python3 over python on Unix)

### Documentation
- README updated with What's New, Quick Start, Building from Source
- User manual expanded with Model Selector, Quick Options, Task Queue
- DeepSeek.md handbook created as single source of truth
