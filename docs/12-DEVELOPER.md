# Developer Guide

This guide covers everything you need to build, extend, and contribute to the Bonsai Ecosystem.

---

## Repository Structure

```
BonsaiWorkspace/
├── bonsai-workspace/          # Tauri desktop app
│   ├── src/                   # Svelte frontend
│   │   ├── App.svelte         # Root component, routing
│   │   ├── lib/
│   │   │   ├── components/    # UI components (50+)
│   │   │   ├── stores/        # Svelte stores (state)
│   │   │   └── types/         # TypeScript types
│   │   └── main.ts            # Entry point
│   └── src-tauri/             # Rust backend
│       ├── src/
│       │   ├── lib.rs         # AppState, setup, invoke_handler
│       │   ├── commands.rs    # Core Tauri commands
│       │   ├── kdb_commands.rs
│       │   ├── package_commands.rs
│       │   ├── collaboration_commands.rs
│       │   └── ...            # 30+ command modules
│       └── Cargo.toml
├── crates/                    # 40+ shared Rust crates
│   ├── bonsai-actors/         # Actor system
│   ├── bonsai-cas/            # Content-addressed storage
│   ├── bonsai-crdt/           # CRDT types
│   ├── bonsai-error/          # Error types (sovereignty)
│   ├── bonsai-hnsw/           # HNSW vector index
│   ├── bonsai-kdb/            # Knowledge database
│   ├── bonsai-mailbox/        # Signed message delivery
│   ├── bonsai-p2p/            # WebRTC, libp2p lanes
│   ├── bonsai-package/        # .bkp package format
│   ├── bonsai-transfer-core/  # ECF-RG multi-path engine
│   ├── bonsai-transfer-crypto/# Ed25519, X25519, AES-GCM
│   ├── bonsai-transfer-store/ # Encrypted at-rest store
│   └── ...
├── src-daemon/                # Headless daemon
├── docs/                      # This documentation
├── scripts/                   # Build, training, utility scripts
├── config/                    # YAML configuration files
├── training_data/             # Local training datasets
├── justfile                   # Task runner (like Makefile)
└── Cargo.toml                 # Workspace root
```

---

## Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust | stable ≥ 1.79 | `curl https://sh.rustup.rs -sSf \| sh` |
| Node.js | 20 LTS | [nodejs.org](https://nodejs.org) |
| pnpm | latest | `npm install -g pnpm` |
| Tauri CLI | 2.x | `cargo install tauri-cli --version "^2"` |
| `just` | latest | `cargo install just` |
| Git | any | system package manager |

**Windows only**: Microsoft Visual C++ Build Tools (from Visual Studio 2022 installer, select "Desktop development with C++").

**For Android**: Android SDK + NDK, JDK 17, `cargo-ndk`.

---

## Building from Source

### Windows

```powershell
git clone https://github.com/LoopyLuci/BonsaiWorkspace
cd BonsaiWorkspace

# Install frontend deps
cd bonsai-workspace && pnpm install && cd ..

# Build all Rust crates (uses sccache if installed)
cargo build --workspace --release

# Build desktop app (Tauri)
cargo tauri build

# Output: bonsai-workspace/src-tauri/target/release/bundle/
```

### macOS

```bash
# Same as Windows, but also install:
xcode-select --install   # command line tools
brew install create-dmg  # for packaging

cargo tauri build
```

### Linux

```bash
# Install system dependencies (Ubuntu/Debian)
sudo apt install -y libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
  libayatana-appindicator3-dev librsvg2-dev

cargo tauri build
```

---

## Development Workflow

### Hot-reload development (recommended)

```powershell
cd bonsai-workspace
cargo tauri dev
```

This starts:
1. Vite dev server for the Svelte frontend (with HMR)
2. Tauri in dev mode (watches Rust backend, rebuilds on change)

Frontend changes reflect instantly. Rust changes trigger a recompile and app reload (~5–30 seconds).

### Frontend-only development

```powershell
cd bonsai-workspace
pnpm dev   # Starts Vite only, no Tauri
```

Useful for styling and component work. Tauri APIs are mocked.

### Backend-only development

```powershell
# Run cargo check quickly without building the frontend
cargo check --workspace

# Run all Rust tests
cargo test --workspace
```

---

## Frontend Architecture

### Svelte Stores
State is managed via Svelte's reactive stores. Convention:

```typescript
// src/lib/stores/myFeature.ts
import { writable, derived } from 'svelte/store';

export const myStore = writable<MyType>(initialValue);
export const derived$ = derived(myStore, $v => transform($v));
```

### Tauri Invoke Pattern
Calling Rust from Svelte:

```typescript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke<ReturnType>('command_name', { param: value });
```

### Listening to Events
Receiving events pushed from Rust:

```typescript
import { listen } from '@tauri-apps/api/event';

const unlisten = await listen<MyPayload>('event-name', (event) => {
  // handle event.payload
});

// cleanup when component unmounts:
onDestroy(unlisten);
```

---

## Backend Architecture

### AppState
All shared state lives in `AppState` (in `lib.rs`). It is thread-safe (`Arc<T>` with interior mutability) and managed by Tauri:

```rust
#[tauri::command]
async fn my_command(state: State<'_, AppState>) -> Result<MyResult, String> {
    let value = state.some_field.lock().await;
    // ...
}
```

### Adding a Tauri Command

1. Create or extend a `*_commands.rs` file:

```rust
#[tauri::command]
pub async fn my_new_command(
    state: State<'_, MyState>,
    param: String,
) -> Result<MyReturn, String> {
    // implementation
    Ok(result)
}
```

2. Register it in `lib.rs` inside `.invoke_handler(tauri::generate_handler![...])`:

```rust
my_module::my_new_command,
```

3. If you need new managed state, initialise it in the `setup` closure:

```rust
app.manage(MyState::new());
```

### Adding a New Crate

1. Create the directory:

```bash
mkdir -p crates/bonsai-myfeature/src
```

2. Create `crates/bonsai-myfeature/Cargo.toml`:

```toml
[package]
name = "bonsai-myfeature"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
```

3. Add to workspace `Cargo.toml`:

```toml
members = [
  # ... existing members
  "crates/bonsai-myfeature",
]
```

4. Depend on it from the Tauri backend:

```toml
# bonsai-workspace/src-tauri/Cargo.toml
bonsai-myfeature = { path = "../../crates/bonsai-myfeature" }
```

---

## Testing

### Rust tests

```powershell
cargo test --workspace              # all tests
cargo test -p bonsai-hnsw          # specific crate
cargo test -- --nocapture           # show println! output
```

### Svelte type checking

```powershell
cd bonsai-workspace
npx svelte-check                    # check all .svelte files
```

### End-to-end tests (Playwright)

```powershell
cd bonsai-workspace
pnpm test:e2e
```

---

## Adding a Feature Flag

1. **`config/features.yaml`** – add the flag with its default value:

```yaml
my_new_feature: false
```

2. **`src-tauri/src/features.rs`** – add a field to `FeatureFlags`:

```rust
pub my_new_feature: bool,
```

3. **`src/lib/stores/features.ts`** – expose to the frontend:

```typescript
export interface Features {
  my_new_feature: boolean;
}
```

4. **Gate your code**:

In Rust:
```rust
if features.my_new_feature {
    // new behaviour
}
```

In Svelte:
```svelte
{#if $features.my_new_feature}
  <MyNewComponent />
{/if}
```

---

## Contributing

### Branch naming

| Type | Pattern | Example |
|---|---|---|
| Feature | `feat/short-description` | `feat/bonsai-core-phase1` |
| Bug fix | `fix/short-description` | `fix/hnsw-borrow-conflict` |
| Docs | `docs/short-description` | `docs/complete-ecosystem` |
| Sovereignty | `feat/sovereignty-phase-N` | `feat/sovereignty-phase1` |

### Commit style

```
type: short imperative description

Optional longer body explaining WHY (not WHAT).

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.

### PR process

1. Fork or create a branch.
2. Make changes. Run `cargo check --workspace` and `npx svelte-check` before pushing.
3. Write or update tests for any changed behaviour.
4. Open a PR with a clear description of what changed and why.
5. A maintainer reviews within 3 business days.

---

*← [Security](11-SECURITY.md) · [Troubleshooting →](13-TROUBLESHOOTING.md)*
