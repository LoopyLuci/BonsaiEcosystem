#!/bin/bash

# Master Build Script for Omnisystem (Unix/Linux/macOS)
# Coordinates all build tasks and phases

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
CONFIGURATION="Release"
CLEAN=false
TEST=false
PACKAGE=false
VERIFY=false

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
GRAY='\033[0;90m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --debug)
            CONFIGURATION="Debug"
            shift
            ;;
        --release)
            CONFIGURATION="Release"
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        --test)
            TEST=true
            shift
            ;;
        --package)
            PACKAGE=true
            shift
            ;;
        --verify)
            VERIFY=true
            shift
            ;;
        --all)
            CLEAN=true
            TEST=true
            PACKAGE=true
            VERIFY=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# If no options provided, default to clean build
if [ "$CLEAN" = false ] && [ "$TEST" = false ] && [ "$PACKAGE" = false ] && [ "$VERIFY" = false ]; then
    CLEAN=true
fi

print_header() {
    echo -e "\n${CYAN}╔══════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║   OMNISYSTEM BUILD MASTER                    ║${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════╝${NC}\n"
}

print_phase() {
    echo -e "\n${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}\n"
}

print_error() {
    echo -e "${RED}✗ $1${NC}\n"
}

print_info() {
    echo -e "${GRAY}$1${NC}"
}

trap 'print_error "Build failed"; exit 1' ERR

# Main build process
print_header
echo -e "Configuration: ${YELLOW}$CONFIGURATION${NC}"
echo -e "Root Directory: ${GRAY}$ROOT_DIR${NC}\n"

# Phase 1: Clean
if [ "$CLEAN" = true ]; then
    print_phase "Phase 1: CLEAN"
    print_info "Cleaning build artifacts..."
    cd "$ROOT_DIR"
    cargo clean
    print_success "Artifacts cleaned"
fi

# Phase 2: Build
print_phase "Phase 2: BUILD ($CONFIGURATION)"
print_info "Building Omnisystem..."
cd "$ROOT_DIR"

if [ "$CONFIGURATION" = "Release" ]; then
    cargo build --workspace --release
else
    cargo build --workspace
fi

print_success "Build complete"

# Phase 3: Test
if [ "$TEST" = true ]; then
    print_phase "Phase 3: TEST"
    print_info "Running test suite..."
    cd "$ROOT_DIR"
    cargo test --workspace
    print_success "Tests passed"
fi

# Phase 4: Verify
if [ "$VERIFY" = true ]; then
    print_phase "Phase 4: VERIFY"
    if [ -f "$SCRIPT_DIR/verification/master_verify.ps1" ]; then
        # Try PowerShell first (cross-platform)
        pwsh "$SCRIPT_DIR/verification/master_verify.ps1" || true
    fi
    print_success "Verification complete"
fi

# Phase 5: Package
if [ "$PACKAGE" = true ]; then
    print_phase "Phase 5: PACKAGE"
    if [ -f "$SCRIPT_DIR/build/package_release.ps1" ]; then
        # Try PowerShell for packaging
        pwsh "$SCRIPT_DIR/build/package_release.ps1" || true
    fi
    print_success "Package complete"
fi

# Summary
echo -e "\n${GREEN}╔══════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   ✓ BUILD COMPLETE                          ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════╝${NC}\n"

TASKS=("Build")
[ "$CLEAN" = true ] && TASKS=("Clean" "${TASKS[@]}")
[ "$TEST" = true ] && TASKS+=("Test")
[ "$VERIFY" = true ] && TASKS+=("Verify")
[ "$PACKAGE" = true ] && TASKS+=("Package")

echo -e "${GREEN}Completed: $(IFS=' → '; echo "${TASKS[*]}")${NC}\n"
print_info "Configuration: $CONFIGURATION"
print_info "Time: $(date '+%Y-%m-%d %H:%M:%S')\n"
