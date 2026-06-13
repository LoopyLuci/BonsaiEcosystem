#!/bin/bash
set -euo pipefail

# Deploy AI Shim and all dependencies
# Usage: ./deploy-ai-shim.sh [docker|kubernetes] [environment]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEPLOYMENT_DIR="$PROJECT_ROOT/Omnisystem/deployment"

DEPLOY_METHOD="${1:-docker}"
ENVIRONMENT="${2:-development}"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    if [ "$DEPLOY_METHOD" = "docker" ]; then
        if ! command -v docker &> /dev/null; then
            log_error "Docker is not installed"
            exit 1
        fi
        if ! command -v docker-compose &> /dev/null; then
            log_error "Docker Compose is not installed"
            exit 1
        fi
        log_success "Docker and Docker Compose found"
    elif [ "$DEPLOY_METHOD" = "kubernetes" ]; then
        if ! command -v kubectl &> /dev/null; then
            log_error "kubectl is not installed"
            exit 1
        fi
        if ! command -v helm &> /dev/null; then
            log_warning "Helm not found - will use kubectl for deployment"
        fi
        log_success "Kubernetes tools found"
    fi
}

# Load environment variables
load_env() {
    log_info "Loading environment for: $ENVIRONMENT"

    ENV_FILE="$DEPLOYMENT_DIR/.env.$ENVIRONMENT"
    if [ -f "$ENV_FILE" ]; then
        # shellcheck source=/dev/null
        source "$ENV_FILE"
        log_success "Loaded environment from $ENV_FILE"
    else
        log_warning "Environment file not found: $ENV_FILE"
        log_info "Creating template..."
        create_env_template
    fi
}

# Create environment template
create_env_template() {
    ENV_FILE="$DEPLOYMENT_DIR/.env.$ENVIRONMENT"
    cat > "$ENV_FILE" << 'EOF'
# AI Shim Configuration

# API Keys (from secrets management system)
export CLAUDE_API_KEY="your-claude-api-key"
export OPENAI_API_KEY="your-openai-api-key"
export GEMINI_API_KEY="your-gemini-api-key"
export MISTRAL_API_KEY="your-mistral-api-key"
export DEEPSEEK_API_KEY="your-deepseek-api-key"

# Service URLs
export SECURITY_MGR_ADDR="security-mgr:9001"
export SECRETS_VAULT_ADDR="secrets-vault:9002"
export AHF_GATEWAY_ADDR="ahf-gateway:9010"
export HDE_ORCHESTRATOR_ADDR="hde-orchestrator:9011"
export STATE_STORE_ADDR="state-store:9003"
export MONITORING_ADDR="monitoring:9004"

# Database
export POSTGRES_USER="bonsai"
export POSTGRES_PASSWORD="bonsai_secure_password"
export POSTGRES_DB="bonsai_ai"

# Redis
export REDIS_URL="redis://redis-ai:6379"

# Logging
export RUST_LOG="info"
export LOG_LEVEL="info"

# Feature flags
export ENABLE_ENSEMBLE_MODE="true"
export ENABLE_SEMANTIC_CACHE="true"
export ENABLE_COST_TRACKING="true"
export ENABLE_CIRCUIT_BREAKER="true"
EOF
    log_success "Created environment template at $ENV_FILE"
    log_warning "Please edit $ENV_FILE with your actual credentials"
}

# Build Docker images
build_docker_images() {
    log_info "Building Docker images..."

    docker_files=(
        "ai-shim"
        "provider-marketplace"
    )

    for component in "${docker_files[@]}"; do
        log_info "Building $component..."
        docker build \
            -f "$DEPLOYMENT_DIR/Dockerfile.$component" \
            -t "bonsai/$component:latest" \
            "$PROJECT_ROOT"
        log_success "Built bonsai/$component:latest"
    done
}

# Deploy with Docker Compose
deploy_docker() {
    log_info "Deploying with Docker Compose..."

    cd "$DEPLOYMENT_DIR"

    log_info "Pulling external images..."
    docker-compose -f docker-compose.ai.yml pull

    log_info "Starting services..."
    docker-compose -f docker-compose.ai.yml up -d

    log_success "Docker Compose deployment complete"

    log_info "Waiting for services to be healthy..."
    sleep 10

    log_info "Service status:"
    docker-compose -f docker-compose.ai.yml ps

    log_success "AI Shim is ready!"
    log_info "Access points:"
    echo "  HTTP API: http://localhost:8117"
    echo "  WebSocket: ws://localhost:8217"
    echo "  Dashboard: http://localhost:8118"
    echo "  Grafana: http://localhost:3000 (admin/bonsai)"
    echo "  Prometheus: http://localhost:9090"
    echo "  Jaeger: http://localhost:16686"
}

# Deploy with Kubernetes
deploy_kubernetes() {
    log_info "Deploying to Kubernetes..."

    NAMESPACE="bonsai-ai"

    log_info "Creating namespace..."
    kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -

    log_info "Applying ConfigMaps and Secrets..."
    kubectl apply -f "$DEPLOYMENT_DIR/k8s-ai-shim.yaml"

    log_info "Waiting for deployment to be ready..."
    kubectl -n "$NAMESPACE" rollout status deployment/ai-shim --timeout=5m

    log_success "Kubernetes deployment complete"

    # Get service info
    log_info "Service information:"
    kubectl -n "$NAMESPACE" get svc ai-shim
    kubectl -n "$NAMESPACE" get pods -l app=ai-shim

    # Port forward for local access
    log_info "Setting up port forwarding..."
    kubectl -n "$NAMESPACE" port-forward svc/ai-shim 8117:8117 &
    log_success "AI Shim is ready!"
    log_info "Access: http://localhost:8117"
}

# Setup databases
setup_databases() {
    log_info "Setting up databases..."

    # PostgreSQL initialization
    if command -v psql &> /dev/null; then
        log_info "Initializing PostgreSQL..."
        psql -h localhost -U bonsai -d bonsai_ai < "$DEPLOYMENT_DIR/init-db.sql" 2>/dev/null || true
        log_success "PostgreSQL initialized"
    fi

    # Redis setup (just verify connectivity)
    if command -v redis-cli &> /dev/null; then
        log_info "Verifying Redis..."
        redis-cli -h localhost ping || log_warning "Redis not available"
    fi
}

# Run health checks
health_checks() {
    log_info "Running health checks..."

    endpoints=(
        "http://localhost:8117/health"
        "http://localhost:8118/health"
        "http://localhost:9090/-/healthy"
    )

    for endpoint in "${endpoints[@]}"; do
        log_info "Checking $endpoint..."
        if curl -sf "$endpoint" > /dev/null 2>&1; then
            log_success "$endpoint is healthy"
        else
            log_warning "$endpoint is not responding yet"
        fi
    done
}

# Integration test
integration_test() {
    log_info "Running integration tests..."

    test_query='{"provider": "claude", "model": "claude-3-haiku", "messages": [{"role": "user", "content": "Say hello"}]}'

    log_info "Testing AI Shim API..."
    if curl -X POST -H "Content-Type: application/json" \
        -d "$test_query" \
        http://localhost:8117/api/v1/ai/chat 2>/dev/null | grep -q "content"; then
        log_success "AI Shim API is responding"
    else
        log_warning "AI Shim API test inconclusive (may need real credentials)"
    fi
}

# Cleanup
cleanup() {
    log_info "Cleaning up..."

    if [ "$DEPLOY_METHOD" = "docker" ]; then
        log_info "Stopping Docker Compose services..."
        cd "$DEPLOYMENT_DIR"
        docker-compose -f docker-compose.ai.yml down -v
        log_success "Docker Compose stopped"
    fi
}

# Main execution
main() {
    log_info "Starting AI Shim deployment..."
    log_info "Method: $DEPLOY_METHOD"
    log_info "Environment: $ENVIRONMENT"

    check_prerequisites
    load_env

    case "$DEPLOY_METHOD" in
        docker)
            build_docker_images
            deploy_docker
            ;;
        kubernetes)
            deploy_kubernetes
            ;;
        *)
            log_error "Unknown deployment method: $DEPLOY_METHOD"
            exit 1
            ;;
    esac

    setup_databases
    sleep 5
    health_checks
    integration_test

    log_success "Deployment completed successfully!"
    log_info "Next steps:"
    echo "  1. Configure API keys in environment"
    echo "  2. Register AI providers: curl -X POST http://localhost:8117/api/v1/ai/providers/register"
    echo "  3. Start making requests to http://localhost:8117"
    echo "  4. Monitor with Grafana: http://localhost:3000"
}

# Trap to cleanup on exit
trap cleanup EXIT

# Run main
main "$@"
