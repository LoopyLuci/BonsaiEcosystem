#!/bin/bash
#
# Omnisystem Alpha 0.1 Installation Script (Linux / macOS)
#
# Usage:
#   curl https://omnilang.org/install.sh | bash
#   ./install.sh /opt/omnisystem
#

set -e

OUT_PATH="${1:-$HOME/omnisystem}"
PYTHON_CMD=""
SHELL_RC=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} ERROR: $1"
    exit 1
}

info() {
    echo -e "${CYAN}→${NC} $1"
}

warn() {
    echo -e "${YELLOW}!${NC} $1"
}

# Header
echo ""
echo -e "${CYAN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║    OMNISYSTEM ALPHA 0.1 — INSTALLATION (Linux / macOS)     ║${NC}"
echo -e "${CYAN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# [1/5] Check prerequisites
info "[1/5] Checking prerequisites..."

# Check Python
for cmd in python3 python; do
    if command -v $cmd &> /dev/null; then
        PYTHON_CMD=$cmd
        PY_VERSION=$($PYTHON_CMD --version 2>&1 | awk '{print $2}')
        success "  Python found: $PYTHON_CMD ($PY_VERSION)"
        break
    fi
done

if [ -z "$PYTHON_CMD" ]; then
    error "Python not found on PATH. Please install Python 3.10+"
fi

# Check Git
if ! command -v git &> /dev/null; then
    error "Git not found. Please install Git (apt/brew/yum)"
fi
success "  Git found: $(git --version | awk '{print $3}')"

# [2/5] Create installation directory
info "[2/5] Setting up installation directory..."

mkdir -p "$OUT_PATH"
success "  Directory: $OUT_PATH"

# [3/5] Clone or update repository
info "[3/5] Downloading Omnisystem source..."

if [ -d "$OUT_PATH/.git" ]; then
    info "  Repository exists, updating..."
    cd "$OUT_PATH"
    git pull origin main 2>&1 | head -3
else
    info "  Cloning repository..."
    git clone https://github.com/omnilang/omnisystem.git "$OUT_PATH" 2>&1 | head -3
fi
success "  Source ready at: $OUT_PATH"
cd "$OUT_PATH"

# [4/5] Create and configure virtual environment
info "[4/5] Setting up Python environment..."

VENV_PATH="$OUT_PATH/.venv"

if [ -d "$VENV_PATH" ]; then
    success "  Virtual environment exists"
else
    info "  Creating virtual environment..."
    $PYTHON_CMD -m venv "$VENV_PATH"
fi

# Activate venv
source "$VENV_PATH/bin/activate"

info "  Installing Python dependencies..."
pip install --upgrade pip setuptools > /dev/null 2>&1
pip install llvmlite==0.47.0 pytest > /dev/null 2>&1
success "  Python environment ready"

# [5/5] Configure PATH
info "[5/5] Configuring PATH..."

# Detect shell
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
fi

VENV_BIN="$VENV_PATH/bin"
OMNI_CMD="$VENV_BIN/build"

# Create wrapper script
cat > "$OMNI_CMD" << 'EOF'
#!/bin/bash
python "$VENV_PATH/tools/build/main.py" "$@"
EOF
chmod +x "$OMNI_CMD"
success "  Created wrapper: $OMNI_CMD"

# Add to PATH
if [ -f "$SHELL_RC" ]; then
    if ! grep -q "$VENV_BIN" "$SHELL_RC"; then
        echo "export PATH=\"$VENV_BIN:\$PATH\"" >> "$SHELL_RC"
        success "  Added to $SHELL_RC"
    else
        success "  Already in $SHELL_RC"
    fi
fi

# Export for current session
export PATH="$VENV_BIN:$PATH"
success "  Added to current PATH"

# Success message
echo ""
echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║             ✓ INSTALLATION COMPLETE                         ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${CYAN}Next steps:${NC}"
echo ""
echo "1. Reload your shell:"
echo -e "   ${YELLOW}source $SHELL_RC${NC}"
echo ""
echo "2. Verify installation:"
echo -e "   ${YELLOW}build --version${NC}"
echo ""
echo "3. Get started:"
echo -e "   ${YELLOW}build new myapp${NC}"
echo -e "   ${YELLOW}cd myapp${NC}"
echo -e "   ${YELLOW}build run examples/hello_world.build${NC}"
echo ""
echo "4. Try the REPL:"
echo -e "   ${YELLOW}build repl${NC}"
echo ""
echo -e "${CYAN}Documentation: https://omnilang.org/getting-started${NC}"
echo -e "${CYAN}Community: https://github.com/omnilang/omnisystem${NC}"
echo ""
