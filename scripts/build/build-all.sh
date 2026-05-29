#!/usr/bin/env bash
# Cross-platform build script for Bonsai Workspace
# Usage:
#   ./build-all.sh              — build for the current host platform
#   ./build-all.sh --universal  — macOS only: build arm64 + x86_64 and lipo them
#   ./build-all.sh --watchdog   — also build the survival watchdog binary
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TAURI_DIR="$ROOT/bonsai-workspace/src"
CARGO_TAURI_DIR="$ROOT/bonsai-workspace/src-tauri"
WATCHDOG_MANIFEST="$ROOT/crates/bonsai-watchdog/Cargo.toml"

BUILD_UNIVERSAL=false
BUILD_WATCHDOG=false

for arg in "$@"; do
    case $arg in
        --universal) BUILD_UNIVERSAL=true ;;
        --watchdog)  BUILD_WATCHDOG=true ;;
    esac
done

OS="$(uname -s)"
ARCH="$(uname -m)"

echo "==> Bonsai build — OS: $OS  ARCH: $ARCH"

# ── Prerequisites ─────────────────────────────────────────────────────────────
require_cmd() {
    if ! command -v "$1" &>/dev/null; then
        echo "ERROR: '$1' not found. Install it and re-run." >&2
        exit 1
    fi
}
require_cmd cargo
require_cmd node
require_cmd npm

if command -v sccache &>/dev/null; then
    export RUSTC_WRAPPER=sccache
    echo "    sccache enabled"
fi

# ── macOS system dependencies (Tauri uses WKWebView — no webkit2gtk needed) ──
if [ "$OS" = "Darwin" ]; then
    if ! xcode-select -p &>/dev/null; then
        echo "ERROR: Xcode Command Line Tools not found. Run: xcode-select --install" >&2
        exit 1
    fi
fi

# ── Linux system dependencies ─────────────────────────────────────────────────
if [ "$OS" = "Linux" ]; then
    if ! pkg-config --exists webkit2gtk-4.1 2>/dev/null && \
       ! pkg-config --exists webkit2gtk-4.0 2>/dev/null; then
        echo "WARNING: webkit2gtk not found. On Debian/Ubuntu:"
        echo "  sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev"
    fi
fi

# ── Frontend ──────────────────────────────────────────────────────────────────
echo "==> Installing frontend dependencies"
npm --prefix "$TAURI_DIR" install --prefer-offline --no-audit --no-fund --loglevel=error

# ── Tauri build ───────────────────────────────────────────────────────────────
if $BUILD_UNIVERSAL && [ "$OS" = "Darwin" ]; then
    echo "==> Building macOS universal binary (arm64 + x86_64)"
    rustup target add aarch64-apple-darwin x86_64-apple-darwin 2>/dev/null || true

    npm --prefix "$TAURI_DIR" run build

    cargo build --release --manifest-path "$CARGO_TAURI_DIR/Cargo.toml" \
        --target aarch64-apple-darwin
    cargo build --release --manifest-path "$CARGO_TAURI_DIR/Cargo.toml" \
        --target x86_64-apple-darwin

    OUT_DIR="$CARGO_TAURI_DIR/target/universal-apple-darwin/release"
    mkdir -p "$OUT_DIR"
    lipo -create -output "$OUT_DIR/bonsai-workspace" \
        "$CARGO_TAURI_DIR/target/aarch64-apple-darwin/release/bonsai-workspace" \
        "$CARGO_TAURI_DIR/target/x86_64-apple-darwin/release/bonsai-workspace"
    echo "==> Universal binary: $OUT_DIR/bonsai-workspace"
else
    echo "==> Building Tauri app for host platform"
    (cd "$TAURI_DIR" && npx --no-install tauri build 2>/dev/null) || \
    (cd "$TAURI_DIR" && npx tauri build)
fi

# ── Watchdog ──────────────────────────────────────────────────────────────────
if $BUILD_WATCHDOG && [ -f "$WATCHDOG_MANIFEST" ]; then
    echo "==> Building bonsai-watchdog"
    cargo build --release --manifest-path "$WATCHDOG_MANIFEST"
    WATCHDOG_SRC="$ROOT/crates/bonsai-watchdog/target/release/bonsai-watchdog"
    if [ -f "$WATCHDOG_SRC" ]; then
        mkdir -p "$ROOT/target/release"
        cp "$WATCHDOG_SRC" "$ROOT/target/release/bonsai-watchdog"
        echo "==> Watchdog: $ROOT/target/release/bonsai-watchdog"
    fi
fi

echo ""
echo "✓ Build complete."
if [ "$OS" = "Darwin" ]; then
    echo "  App bundle: bonsai-workspace/src-tauri/target/release/bundle/macos/Bonsai\ Workspace.app"
elif [ "$OS" = "Linux" ]; then
    echo "  AppImage:   bonsai-workspace/src-tauri/target/release/bundle/appimage/"
fi
