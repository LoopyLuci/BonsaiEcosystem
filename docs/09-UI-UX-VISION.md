# UI / UX Vision for Bonsai Workspace

This document captures the next-generation UI strategy for Bonsai.

## Goals
- Enable adaptive, generative UI panels driven by the Bonsai runtime.
- Support staged rollout and canary deployments for UI components.
- Provide a unified runtime hook for panel manifest generation, reload, and health validation.
- Keep UI state modular, versioned, and recoverable through CAS-backed panel manifests.

## Core capabilities
- `generate_ui_panel`: create a new panel manifest in CAS with a generated UI specification.
- `list_ui_panels`: inspect active panel manifests and review panel metadata.
- `reload_ui_panel`: trigger a runtime refresh for a live UI panel.

## Implementation direction
- Store every generated UI panel manifest in CAS for deduplicated versioning.
- Use a shared `SystemEventBus` to broadcast lifecycle events across the runtime.
- Introduce orchestrator-level staging so that canary panels can be promoted gradually.
- Keep the frontend decoupled from generation logic: load panel manifests by CAS hash.

## Future work
- Add `ui.start_rollout` / `ui.abort_rollout` RPCs with rollout progress state.
- Implement health-check and fallback logic for panel renderers.
- Build a frontend dashboard to manage active panel versions and rollout strategy.
