# Bonsai Workspace — common tasks
# Install: cargo install just
# Usage:   just <recipe>
#
# Works on Windows (PowerShell), macOS, and Linux.
# On Windows, recipes that use shell scripts fall back to PowerShell equivalents.

workspace_root := justfile_directory()
os := os()

# List available recipes
default:
    @just --list

# ── Build ─────────────────────────────────────────────────────────────────────

# Build for the current platform (recommended)
build:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/build/build-all.ps1"
    else
        bash "{{workspace_root}}/scripts/build/build-all.sh"
    fi

# Build macOS universal binary (arm64 + x86_64, macOS only)
build-universal:
    bash "{{workspace_root}}/scripts/build/build-all.sh" --universal

# Build including survival watchdog
build-with-watchdog:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/build/build-all.ps1" -Watchdog
    else
        bash "{{workspace_root}}/scripts/build/build-all.sh" --watchdog
    fi

# Build only the survival watchdog binary
build-watchdog:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/build/Build-Watchdog.ps1"
    else
        cargo build --release \
            --manifest-path "{{workspace_root}}/crates/bonsai-watchdog/Cargo.toml"
    fi

# ── Launch ────────────────────────────────────────────────────────────────────

# Launch full Bonsai Ecosystem (IDE + Buddy chat window)
launch:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/launch/Launch-Ecosystem.ps1"
    else
        bash "{{workspace_root}}/scripts/launch/launch-ecosystem.sh"
    fi

# Launch Bonsai Workspace IDE only
launch-workspace:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/launch/Launch-Workspace.ps1"
    else
        bash "{{workspace_root}}/scripts/launch/launch-workspace.sh"
    fi

# Launch Bonsai Buddy chat window only
launch-buddy:
    #!/usr/bin/env sh
    if [ "{{os}}" = "windows" ]; then
        powershell -NoProfile -ExecutionPolicy Bypass \
            -File "{{workspace_root}}/scripts/launch/Launch-Buddy.ps1"
    else
        bash "{{workspace_root}}/scripts/launch/launch-buddy.sh"
    fi

# Start the Tauri dev server with HMR (workspace mode)
dev:
    cd bonsai-workspace/src && npx tauri dev

# Start the Tauri dev server in buddy mode
dev-buddy:
    cd bonsai-workspace/src && BONSAI_LAUNCH_MODE=buddy npx tauri dev

# Start the Tauri dev server in ecosystem mode
dev-ecosystem:
    cd bonsai-workspace/src && BONSAI_LAUNCH_MODE=ecosystem npx tauri dev

# Start the headless daemon
daemon:
    cargo run -p bonsai-daemon

# ── Test ──────────────────────────────────────────────────────────────────────

# Run all Rust workspace tests
test:
    cargo test --workspace

# Run watchdog tests only
test-watchdog:
    cargo test --manifest-path crates/bonsai-watchdog/Cargo.toml -- --nocapture

# Run frontend tests
test-frontend:
    npm --prefix bonsai-workspace/src run test

# Run integration tests
test-integration:
    python tests/integration/test_daemon_local.py

# ── Lint / Check ──────────────────────────────────────────────────────────────

# Fast workspace check (no codegen)
check:
    cargo check --workspace

# Clippy + fmt check + frontend lint
lint:
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    npm --prefix bonsai-workspace/src run lint

# ── Release ───────────────────────────────────────────────────────────────────

# Tag and push a release: just release VERSION=v0.2.1
release VERSION="":
    #!/usr/bin/env sh
    if [ -z "{{VERSION}}" ]; then echo "Usage: just release VERSION=v0.x.y"; exit 1; fi
    git tag -a "{{VERSION}}" -m "{{VERSION}}"
    git push origin "{{VERSION}}"
    gh release create "{{VERSION}}" --title "{{VERSION}}" --notes-file CHANGELOG.md
