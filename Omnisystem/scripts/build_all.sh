#!/bin/bash
# BonsaiWorkspace - Unified Build Script
# Builds all 228+ crates with proper error handling and progress tracking

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BUILD_MODE="${1:-debug}"
JOBS="${2:-$(nproc)}"
START_TIME=$(date +%s)

# Statistics
TOTAL_CRATES=0
BUILT_CRATES=0
FAILED_CRATES=0
SKIPPED_CRATES=0

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  BonsaiWorkspace - Unified Build System              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Build Configuration:${NC}"
echo "  Mode: $BUILD_MODE"
echo "  Parallel Jobs: $JOBS"
echo "  Workspace: $WORKSPACE_ROOT"
echo ""

# Function to log messages
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

# Function to build crate with error handling
build_crate() {
    local crate_name=$1
    local crate_path=$2

    echo -n "Building $crate_name... "

    if [ ! -f "$crate_path/Cargo.toml" ]; then
        log_warning "Cargo.toml not found, skipping"
        ((SKIPPED_CRATES++))
        return
    fi

    if cargo build -p "$crate_name" -j "$JOBS" --"$BUILD_MODE" 2>/dev/null; then
        log_success "Built"
        ((BUILT_CRATES++))
    else
        log_error "Build failed"
        ((FAILED_CRATES++))
    fi
    ((TOTAL_CRATES++))
}

# Build Foundation Tier (Tier 0)
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 0: Foundation Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

build_crate "omnisystem-ums" "$WORKSPACE_ROOT/Omnisystem/crates/omnisystem-ums"
build_crate "omnisystem-axiom-spec" "$WORKSPACE_ROOT/Omnisystem/crates/omnisystem-axiom-spec"

# Build Core Runtime Tier (Tier 1)
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 1: Core Runtime Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in omnisystem-kernel omnisystem-ffi omnisystem-loader \
             omnisystem-async omnisystem-rust-bindings omnisystem-go-bindings \
             omnisystem-sylva-core omnisystem-sylva-phase2; do
    build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
done

# Build OS Integration Tier (Tier 2)
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 2: OS Integration Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in omnisystem-linux omnisystem-windows omnisystem-macos \
             omnisystem-cpu omnisystem-memory omnisystem-interrupt \
             omnisystem-device omnisystem-sylva-phase3 omnisystem-sylva-phase4; do
    build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
done

# Build Service Tier (Tier 3)
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 3: Service Crates (PATHFINDER)${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in pathfinder-user-service pathfinder-content-service \
             pathfinder-progress-service pathfinder-teacher-service \
             pathfinder-parent-service pathfinder-notification-service \
             pathfinder-achievement-service pathfinder-insights-service \
             pathfinder-personalization-service; do
    build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
done

# Build OmniSearch Tier
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 3: OmniSearch Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in omnisearch-core omnisearch-distributed omnisearch-indexing \
             omnisearch-ranking omnisearch-query omnisearch-aggregation; do
    if [ -d "$WORKSPACE_ROOT/Omnisystem/crates/$crate" ]; then
        build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
    fi
done

# Build OmniFile Tier
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 3: OmniFile Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in omnifile-core omnifile-storage omnifile-access omnifile-versioning; do
    if [ -d "$WORKSPACE_ROOT/Omnisystem/crates/$crate" ]; then
        build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
    fi
done

# Build Network/IoT Tier
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 3: Network & IoT Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in network-firmware iot-core iot-zigbee iot-zwave iot-router; do
    if [ -d "$WORKSPACE_ROOT/Omnisystem/crates/$crate" ]; then
        build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
    fi
done

# Build AI Systems Tier
echo ""
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}TIER 3: AI Systems Crates${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"

for crate in ai-advisor ai-inference ai-training; do
    if [ -d "$WORKSPACE_ROOT/Omnisystem/crates/$crate" ]; then
        build_crate "$crate" "$WORKSPACE_ROOT/Omnisystem/crates/$crate"
    fi
done

# Summary
echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                  BUILD SUMMARY                        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"

ELAPSED=$(($(date +%s) - START_TIME))
MINUTES=$((ELAPSED / 60))
SECONDS=$((ELAPSED % 60))

echo ""
echo -e "${YELLOW}Statistics:${NC}"
echo "  Total Crates: $TOTAL_CRATES"
echo -e "  Built: ${GREEN}$BUILT_CRATES${NC}"
echo -e "  Failed: ${RED}$FAILED_CRATES${NC}"
echo -e "  Skipped: ${YELLOW}$SKIPPED_CRATES${NC}"
echo ""
echo -e "${YELLOW}Time Elapsed:${NC} ${MINUTES}m ${SECONDS}s"
echo ""

if [ $FAILED_CRATES -eq 0 ]; then
    log_success "All crates built successfully!"
    exit 0
else
    log_error "$FAILED_CRATES crates failed to build"
    exit 1
fi
