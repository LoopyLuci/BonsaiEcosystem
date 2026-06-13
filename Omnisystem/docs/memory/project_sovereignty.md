---
name: project-sovereignty
description: BonsAI Sovereignty Plan + AFAP Speedrun — replace all 50+ external deps with custom crates in 8-12 months using AI agent assembly line and Training Agent model
metadata: 
  node_type: memory
  type: project
  originSessionId: 3ea2ae9d-7998-4d81-9af5-ec92ef7a7519
---

**Canonical docs:**
- `docs/SOVEREIGNTY_PLAN.md` — full dependency inventory, 50 crates, 6 phases
- `docs/SPEEDRUN_PLAN.md` — AFAP execution strategy, 8-month timeline, Training Agent design
- `docs/specs/TRAINING_AGENT.md` — Training Agent model spec (base: Qwen2.5-14B-Instruct)
- `docs/specs/bonsai-error.md` — first crate to build (Phase 1, replaces anyhow+thiserror)
- `docs/specs/TEMPLATE.md` — spec template for all future crates

**Scripts:**
- `scripts/generate_crate.ps1` — AI factory loop: spec -> teacher generation -> compile/test iteration
- `scripts/export_training_logs.py` — convert training runs to Training Agent dataset examples
- `scripts/generate_training_agent_dpo.py` — generate preference pairs for Training Agent

**Training Agent dataset (seeded):**
- `~/.bonsai/training_agent/bonsai_logs.jsonl` — 1 project-specific training log (safety DPO)
- `~/.bonsai/training_agent/tier3_dpo_pairs.jsonl` — 20 handcrafted ground-truth DPO pairs

**Phase 1 immediate next steps:**
1. `cargo vendor vendor/` to enable offline builds
2. Run `generate_crate.ps1 -Crate bonsai-error` (spec is written, ready to go)
3. Write remaining 7 Phase 1 specs (bonsai-log, bonsai-rand, bonsai-time, bonsai-id, bonsai-codec, bonsai-fs, bonsai-sanitise)
4. Run Training Agent Stage 1 SFT after teacher data collection completes

**Why:** Zero supply-chain risk, full code ownership, self-improving AI training loop.
**How to apply:** When starting a new crate, first check `docs/specs/` for an existing spec, then run `generate_crate.ps1`.
