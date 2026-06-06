# Copilot Todos

## UI Orchestrator and Generative UI
- [x] Add `ui_orchestrator` module into `bonsai-workspace/src-tauri/src/ui_orchestrator.rs`
- [x] Wire `AppState.ui_orchestrator` into `bonsai-workspace/src-tauri/src/lib.rs`
- [x] Use shared `SystemEventBus` to publish UI panel lifecycle events
- [x] Add Tauri commands for `generate_ui_panel`, `list_ui_panels`, and `reload_ui_panel`

## Runtime / UI workflow
- [ ] Add staged rollout / canary UI panel routing in the orchestrator
- [ ] Implement actual UI rendering integration and panel manifest consumption in frontend
- [ ] Add health check hooks for panel reloads and runtime validation

## Docs and planning
- [x] Add UI/UX vision notes in `docs/09-UI-UX-VISION.md`
- [ ] Expand rollout and generative UI design into the broader Bonsai roadmap
