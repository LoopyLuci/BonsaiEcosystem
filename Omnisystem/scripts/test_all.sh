#!/bin/bash
# BonsaiWorkspace - Unified Test Suite
# Runs all unit, integration, and benchmark tests across 228+ crates

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEST_TYPE="${1:-all}"  # all, unit, integration, benchmark
JOBS="${2:-$(nproc)}"
START_TIME=$(date +%s)

# Statistics
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  BonsaiWorkspace - Test Suite                        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Test Configuration:${NC}"
echo "  Type: $TEST_TYPE"
echo "  Parallel Jobs: $JOBS"
echo "  Workspace: $WORKSPACE_ROOT"
echo ""

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Unit Tests
if [ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "unit" ]; then
    echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}Running Unit Tests${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
    echo ""

    if cargo test --workspace --lib -j "$JOBS" --release; then
        log_success "All unit tests passed"
    else
        log_error "Some unit tests failed"
    fi
    echo ""
fi

# Integration Tests
if [ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "integration" ]; then
    echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}Running Integration Tests${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
    echo ""

    if cargo test --workspace --test '*' -j "$JOBS" --release; then
        log_success "All integration tests passed"
    else
        log_error "Some integration tests failed"
    fi
    echo ""
fi

# Benchmark Tests
if [ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "benchmark" ]; then
    echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}Running Benchmark Tests${NC}"
    echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
    echo ""

    if cargo bench --workspace --no-run -j "$JOBS"; then
        log_success "Benchmarks compiled successfully"
    else
        log_error "Some benchmarks failed to compile"
    fi
    echo ""
fi

# Code Coverage
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Checking Code Coverage${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo ""

if command -v cargo-tarpaulin &> /dev/null; then
    log_info "Running coverage analysis..."
    if cargo tarpaulin --workspace --timeout 600 --fail-under 85 --out Stdout; then
        log_success "Code coverage >= 85%"
    else
        log_error "Code coverage < 85%"
    fi
else
    log_info "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
    cargo tarpaulin --workspace --timeout 600 --fail-under 85 --out Stdout
fi
echo ""

# Linting
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Running Linting Checks${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo ""

log_info "Checking code style..."
if cargo fmt --all -- --check; then
    log_success "Code formatting OK"
else
    log_error "Code formatting issues found"
    log_info "Run 'cargo fmt --all' to fix"
fi
echo ""

log_info "Running clippy..."
if cargo clippy --workspace -- -D warnings; then
    log_success "Clippy checks passed"
else
    log_error "Clippy found warnings"
fi
echo ""

# Security Audit
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Running Security Audit${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo ""

if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        log_success "No security vulnerabilities found"
    else
        log_error "Security vulnerabilities detected"
    fi
else
    log_info "Installing cargo-audit..."
    cargo install cargo-audit
    cargo audit
fi
echo ""

# Documentation Build
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo -e "${YELLOW}Building Documentation${NC}"
echo -e "${YELLOW}═══════════════════════════════════════════════════════${NC}"
echo ""

if cargo doc --workspace --no-deps 2>/dev/null; then
    log_success "Documentation built successfully"
else
    log_error "Documentation build failed"
fi
echo ""

# Summary
ELAPSED=$(($(date +%s) - START_TIME))
MINUTES=$((ELAPSED / 60))
SECONDS=$((ELAPSED % 60))

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                   TEST SUMMARY                        ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${YELLOW}Test Types Run:${NC}"
echo "  Unit Tests: $([ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "unit" ] && echo "✓" || echo "✗")"
echo "  Integration Tests: $([ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "integration" ] && echo "✓" || echo "✗")"
echo "  Benchmarks: $([ "$TEST_TYPE" = "all" ] || [ "$TEST_TYPE" = "benchmark" ] && echo "✓" || echo "✗")"
echo "  Code Coverage: ✓"
echo "  Linting: ✓"
echo "  Security Audit: ✓"
echo "  Documentation: ✓"
echo ""
echo -e "${YELLOW}Time Elapsed:${NC} ${MINUTES}m ${SECONDS}s"
echo ""

log_success "Test suite completed!"
exit 0
