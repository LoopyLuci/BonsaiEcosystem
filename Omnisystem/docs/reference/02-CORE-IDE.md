# Core IDE Features

Bonsai Workspace is a full-featured code editor built on Tauri + Svelte. This document explains every panel, shortcut, and configuration option.

---

## Workspace Layout

The interface is divided into resizable regions:

```
┌─[Sidebar]──┬─────────[Editor Area]──────┬─[Right Panel]──┐
│ File Tree  │  Monaco Editor (main)       │  Chat / Builder │
│ Activity   │                             │  Trainer        │
├────────────┴─────────────────────────────┴─────────────────┤
│                    Terminal (bottom)                        │
├────────────────────────────────────────────────────────────┤
│  Status Bar: model name · lane status · brain age · clock  │
└────────────────────────────────────────────────────────────┘
```

**Resizing**: Drag any panel border. Double-click a border to snap to 50/50.  
**Hiding**: Click a sidebar icon to toggle its panel. Use `Ctrl+B` to collapse the sidebar entirely.

---

## File Tree

The file tree gives you a full view of your project directory.

### Navigation
- **Click** a file to open it in the editor.
- **Double-click** a folder to expand/collapse.
- **Right-click** any item for context menu actions.

### Context Menu Actions
| Action | Description |
|---|---|
| New File | Creates a file in the selected directory |
| New Folder | Creates a folder |
| Rename | In-place rename with `F2` |
| Delete | Moves to system trash (reversible) |
| Copy Path | Copies the absolute path to clipboard |
| Open in Terminal | Opens a terminal `cd`'d to that directory |
| Send via TransferDaemon | Sends file to a peer |

### Search
Press `Ctrl+P` to open the **Quick Open** box. Type any filename — fuzzy search finds it across the whole project.

### Git Integration
Modified files show coloured indicators:
- 🟡 **M** – modified
- 🟢 **U** – untracked
- 🔴 **D** – deleted
- 🔵 **A** – added/staged

---

## Monaco Editor

Bonsai uses the same editor engine as Visual Studio Code.

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+S` | Save file |
| `Ctrl+Z` / `Ctrl+Y` | Undo / Redo |
| `Ctrl+F` | Find in file |
| `Ctrl+H` | Find and replace |
| `Ctrl+G` | Go to line |
| `Ctrl+D` | Select next occurrence |
| `Alt+Click` | Add cursor (multi-cursor editing) |
| `Ctrl+Shift+K` | Delete line |
| `Alt+↑/↓` | Move line up/down |
| `Ctrl+/` | Toggle line comment |
| `Ctrl+Shift+P` | Command Palette |

### Language Support
Syntax highlighting, folding, and bracket matching for: Rust, Python, TypeScript, JavaScript, Svelte, HTML, CSS, JSON, TOML, YAML, Markdown, SQL, Bash, Go, C/C++, Java, and more.

### Split View
Right-click a file tab → **Split Right** or **Split Down** to open two files side by side.

### Minimap
The right-edge minimap shows a scaled view of the whole file. Click any region to jump. Toggle with `Ctrl+Shift+M`.

### IntelliSense
When a language server is running (e.g., `rust-analyzer` for Rust projects), you get:
- Hover documentation
- Auto-complete
- Error squiggles
- Go-to-definition (`F12`)
- Find all references (`Shift+F12`)

### Diff View
Open **File → Compare Active File With…** to see a side-by-side diff. This is also used by the Survival System when proposing file repairs.

---

## Integrated Terminal

Press `` Ctrl+` `` or click the terminal icon to open a terminal panel.

### Features
- **Multiple sessions**: click `+` to open a new terminal tab. Each tab is an independent shell.
- **Shell selection**: defaults to PowerShell (Windows), Zsh/Bash (macOS/Linux). Configure in Settings.
- **Resize**: drag the panel border up/down.
- **Full PTY**: interactive programs like `vim`, `htop`, or `cargo test -- --nocapture` work correctly.

### Terminal in Collaboration Mode
When a [collaboration session](07-COLLABORATION.md) is active, a participant with write permission can share their terminal. Others see output in real time; only the permitted user can type. See [07-COLLABORATION.md §Shared Terminal](07-COLLABORATION.md).

---

## Activity Log

The Activity Log records every significant event in the workspace.

### Event Categories
| Category | Examples |
|---|---|
| `model` | Model loaded, inference started, token generated |
| `tool` | Tool invoked, result returned, error |
| `survival` | Crash detected, fix applied, watchdog restart |
| `training` | Epoch completed, loss reported, adapter saved |
| `transfer` | File sent, chunk received, transfer completed |
| `system` | Feature flag changed, config reloaded |

### Controls
- **Filter**: type in the search box to filter by category or keyword.
- **Pause**: click Pause to stop auto-scrolling without losing events.
- **Export**: click Export → CSV or JSON to save logs for analysis.
- **Clear**: removes displayed entries (does not delete from disk).

Logs are persisted to `~/.bonsai/activity.log` and rotated daily.

---

## Command Palette

Press `Ctrl+Shift+P` to open the Command Palette. Type any command name to fuzzy-search available actions:

| Command | Description |
|---|---|
| `Open Folder` | Open a directory in the file tree |
| `Switch Model` | Change the active inference model |
| `Train: Quick Train` | Start a quick training cycle |
| `Survival: Scan & Repair` | Run the survival engine |
| `Collaboration: Create Session` | Start a collaboration session |
| `Feature Flags: Toggle` | Enable or disable a feature |
| `KDB: List Modules` | Show knowledge modules |
| `Export Package (.bkp)` | Package the current model |

---

## Settings Panel

Click ⚙️ or press `Ctrl+,` to open Settings.

### Categories

**General**
- Theme (dark / light / system)
- Font family and size
- Auto-save interval
- Language / locale

**Models**
- List of local GGUF files
- Active model selection
- GPU layer count (offloaded to VRAM)
- Context window size
- Max generation tokens

**Training**
- Data directory
- Batch size, learning rate, epochs
- Teacher model paths
- Domain weights

**Features**
- Toggle any feature flag (see below)

**Privacy**
- Undercover Mode on/off
- Activity log retention days
- Clear all local data

**Transfer**
- Listen port for incoming connections
- Relay server URL (optional)
- Bandwidth limits

### Import / Export
Click **Export Settings** to save a `settings.json` you can move to another machine. **Import Settings** restores it.

---

## Feature Flags

Feature flags let you enable or disable experimental capabilities without rebuilding the app. Managed in `config/features.yaml` and in the Settings UI.

| Flag | Default | Description |
|---|---|---|
| `model_trainer_enabled` | `true` | Show the Model Trainer panel |
| `undercover_mode` | `false` | Hide Bonsai branding in prompts |
| `web_router` | `false` | Expose local HTTP API for external tools |
| `collaboration_enabled` | `true` | Enable collaboration features |
| `compute_fabric_enabled` | `false` | Enable distributed computing |
| `creator_studio_enabled` | `false` | Enable generative AI (image/video/audio) |
| `survival_ai_repair` | `true` | Let BonsAI generate new repair scripts |
| `eternal_training_loop` | `false` | Run background training continuously |

Changes take effect immediately for UI flags; some backend flags require a restart (indicated by a ⚠ icon).

---

*← [Getting Started](01-GETTING-STARTED.md) · [BonsAI Assistant →](03-BONSAI-ASSISTANT.md)*
