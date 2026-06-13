# Release Checklist

Use this checklist on a real development machine after pulling `main`.

## Core Startup & Security

- [ ] Startup health gate verification.
Command/Observation: Start app with no model loaded (`npm run tauri dev`) and confirm a `startup-health` warning/event is emitted with user guidance.

- [ ] File jail rejects write outside workspace.
Command/Observation: Ask assistant to write to `../outside.txt` (or absolute external path) and confirm permission denial (outside allowed root).

- [ ] SSRF protection blocks loopback/private targets.
Command/Observation: Ask assistant to run `fetch_url` on `http://127.0.0.1:80` and confirm request is rejected by policy.

- [ ] XSS sanitization escapes hostile code-fence payloads.
Command/Observation: Send message containing fenced code with hostile language/content (e.g. `<script>alert(1)</script>`) and verify rendered block is escaped, not executed.

## Swarm Validation

- [ ] Swarm sequential strategy runs in tier order.
Command/Observation: Run a swarm turn with `chain_strategy=sequential_gate` and confirm worker execution order follows policy tiers in `swarm-debug` events.

- [ ] Swarm parallel vote includes tally in synthesis.
Command/Observation: Run with `chain_strategy=parallel_vote` and verify final synthesis contains explicit tally language (e.g. `2 of 3 workers`).

- [ ] Worker self-assessment blocks are present.
Command/Observation: Inspect worker outputs/events and confirm `<worker_assessment>` appears with confidence, evidence_sources, and gaps.

- [ ] Swarm metrics events visible in activity log.
Command/Observation: Verify `swarm-debug`, `swarm-agent-complete`, and `swarm-complete`/metrics data appear for each run.

## Retrieval & Memory

- [ ] RAG hybrid search returns relevant results.
Command/Observation: Query known repo text through `search_knowledge` and confirm returned path/snippet is relevant.

- [ ] RAG index survives app restart.
Command/Observation: Perform one successful `search_knowledge`, restart app, repeat same query, and confirm retrieval still works without full re-index delay.

- [ ] Cross-session swarm summary recall works.
Command/Observation: Run one swarm session, start a new session with related prompt, and confirm leader prompt behavior references relevant recent summary context.

## Optional Scripted Smoke

- [ ] Run scripted smoke test.
Command: `bash scripts/smoke-test-swarm.sh`

- [ ] Save smoke output for release notes.
Command/Observation: Collect script output + Activity Log screenshots and attach to release ticket.
