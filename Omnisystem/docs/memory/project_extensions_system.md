---
name: project-extensions-system
description: bonsai-extensions crate + Extensions Browser + Extension Conversion System design and implementation status
metadata: 
  node_type: memory
  type: project
  originSessionId: 3ea2ae9d-7998-4d81-9af5-ec92ef7a7519
---

# Extensions System

## Implemented (compile-verified)

- `crates/bonsai-extensions/` — manifest schema, security scanner (9 rules), installer (GitHub ZIP), registry (DashMap)
- `bonsai-workspace/src-tauri/src/extensions_commands.rs` — 14 Tauri commands (ext_install_from_github, ext_list_all, ext_rescan, etc.)
- `bonsai-workspace/src/lib/panels/ExtensionsPanel.svelte` — 4-tab UI: Browse (card grid), Installed (list+detail), Import (VSCode/JetBrains), Submit
- Extensions wired into lib.rs: `mod extensions_commands`, `app.manage(ExtensionsState::new())`, all 14 commands in invoke_handler
- 🧩 Ext toolbar button added to App.svelte, opens as right-side overlay panel

## Extension Converter (Phase 1)

- `crates/bonsai-extension-converter/` — new crate (compile-verified)
- `src/ir.rs` — Unified Extension IR (ExtensionIr, Capability enum with Command/LanguageSupport/View/Tool/Theme/Snippet/Keybinding/Custom)
- `src/import/vscode.rs` — parses .vsix, maps package.json contributes → IR
- `src/export/mcp.rs` — generates standalone MCP server (Rust source + Cargo.toml) from Bonsai extension tools
- Added to workspace Cargo.toml members

## Conversion Tier Strategy

- **Tier 1** Manifest-only: themes, snippets, keybindings — automatic
- **Tier 2** Shim-based: standard VSCode APIs compiled against bonsai-vscode-shim — semi-automatic
- **Tier 3** AI-assisted: complex extensions via Conversion Agent swarm

**Why:** MCP export is highest-ROI first target (zero IDE coupling, works with Claude/Continue/Cursor immediately).

## Key Bug Fixed

`['"]` in raw strings `r"..."` — the `"` inside `\"` terminates the raw string. Fix: use `r#"..."#` or swap to `["']`.
