# AI Shim Integration Guide

## Overview

The **Universal AI Agent Shim** is fully integrated into the Bonsai Ecosystem across three layers:

1. **UOSC (Microkernel Layer)** – System-level capabilities and scheduling
2. **Omnisystem (OS Services Layer)** – Core AI routing, caching, resilience
3. **BonsaiEcosystem (Application Layer)** – User-facing AI tools and interfaces

---

## Architecture

### Layer Integration

```
┌─────────────────────────────────────────────────────┐
│ BonsaiEcosystem (Layer 3)                           │
│  • AI Chat UI (Web & Desktop)                       │
│  • Provider Marketplace                              │
│  • Cost Dashboard                                    │
│  • AI Buddy Integration                             │
└────────────────────┬────────────────────────────────┘
                     │ HTTP/WebSocket
┌────────────────────▼────────────────────────────────┐
│ Omnisystem (Layer 2)                                │
│  • Enhanced AI Shim (ai_shim_enhanced.ti)          │
│  • Provider Adapters (6 providers)                  │
│  • Semantic Cache & Deduplication                   │
│  • Circuit Breaker & Resilience                     │
│  • Cost Tracking & Rate Limiting                    │
│  • Ensemble Routing                                 │
└────────────────────┬────────────────────────────────┘
                     │ IPC (Aether)
┌────────────────────▼────────────────────────────────┐
│ UOSC (Layer 1)                                      │
│  • Memory Management (AI request buffers)          │
│  • Process Scheduling (AI task priority)           │
│  • IPC Capabilities (secure message passing)       │
│  • Hardware Resource Allocation                    │
└─────────────────────────────────────────────────────┘
```

---

## Provider Adapters

### Integrated Providers

| Provider | Status | Streaming | Cost Tracking | Fallback |
|----------|--------|-----------|---------------|----------|
| Claude (Anthropic) | ✅ Prod | Yes | Yes | HDE |
| GPT-4 (OpenAI) | ✅ Prod | Yes | Yes | HDE |
| Gemini (Google) | ✅ Prod | Yes | Yes | HDE |
| Mistral | ✅ Prod | Yes | Yes | HDE |
| DeepSeek | ✅ Prod | No | Yes | HDE |
| Ollama (Local) | ✅ Prod | Yes | No | Local |

### Adding New Providers

1. Create `Omnisystem/services/ai/providers/myservice_adapter.ti`
2. Implement `AiProvider` trait with:
   - `name()` – unique identifier
   - `list_models()` – available models
   - `chat()` – synchronous completion
   - `stream_chat()` – streaming completion
   - `health_check()` – connectivity verification
   - `cost_per_token()` – pricing info
3. Register in `EnhancedAiShim::register_provider_with_cb()`

---

## Deployment

### Docker Compose (Development)

```bash
cd Omnisystem/deployment
docker-compose -f docker-compose.ai.yml up -d
```

**Includes:**
- AI Shim (HTTP + WebSocket)
- Ollama (local models)
- Provider Marketplace
- Prometheus + Grafana (monitoring)
- Redis (caching)
- PostgreSQL (analytics)
- Jaeger (distributed tracing)

### Kubernetes (Production)

```bash
bash Omnisystem/scripts/deploy-ai-shim.sh kubernetes production
```

**Features:**
- High availability (3 replicas min)
- Auto-scaling (up to 10 replicas)
- Network policies for security
- Pod disruption budgets
- Health checks and readiness probes
- Distributed tracing integration

---

## API Usage

### REST API

```bash
# Health check
curl http://localhost:8117/health

# List providers
curl http://localhost:8117/api/v1/ai/providers

# Chat completion
curl -X POST http://localhost:8117/api/v1/ai/chat \
  -H "Content-Type: application/json" \
  -d '{
    "provider": "claude",
    "model": "claude-3-haiku",
    "messages": [{"role": "user", "content": "Hello!"}],
    "options": {
      "temperature": 0.7,
      "max_tokens": 1000,
      "cache_ttl_secs": 300
    }
  }'
```

### WebSocket (Streaming)

```javascript
const ws = new WebSocket("ws://localhost:8217");

ws.send(JSON.stringify({
  command: "stream",
  provider: "claude",
  model: "claude-3-haiku",
  messages: [{"role": "user", "content": "Write a poem"}],
  options: {temperature: 0.9}
}));

ws.onmessage = (event) => {
  const chunk = JSON.parse(event.data);
  console.log(chunk.delta); // Token received
};
```

### Cost Tracking

```bash
# Get cost report for a caller
curl "http://localhost:8117/api/v1/ai/cost/report?caller_id=user-123"

# Response:
{
  "total_requests": 42,
  "total_input_tokens": 15420,
  "total_output_tokens": 8920,
  "total_cost_dollars": 0.47
}
```

### Ensemble Queries

```bash
curl -X POST http://localhost:8117/api/v1/ai/ensemble \
  -H "Content-Type: application/json" \
  -d '{
    "messages": [{"role": "user", "content": "What is AI?"}],
    "options": {"model": "gpt-4", "temperature": 0.5}
  }'

# Response:
{
  "consensus_response": "AI is...",
  "confidence": 0.94,
  "provider_responses": [
    {"provider": "claude", "content": "..."},
    {"provider": "gpt-4", "content": "..."}
  ]
}
```

---

## Advanced Features

### 1. Circuit Breaker

Automatically stops sending requests to failing providers and retries after timeout.

```rust
// Configured per provider
circuit_breakers.insert("claude", CircuitBreaker::new(
  failure_threshold: 5,  // Open after 5 failures
  success_threshold: 2,  // Close after 2 successes
  timeout_ms: 30000     // Retry after 30 seconds
));
```

### 2. Semantic Caching

Cache responses by semantic similarity, not just exact text match.

```rust
// 92% similarity threshold
semantic_cache.retrieve(&similar_question)
  // Returns cached response from "similar but different" question
```

### 3. Request Deduplication

Deduplicate identical requests within a time window.

```
User 1: "What is the capital of France?"
User 2: "What is the capital of France?" (within 60s)
       → Returns User 1's cached response instantly
```

### 4. Ensemble Routing

Query multiple providers and aggregate consensus responses.

```rust
// Majority voting strategy
ensemble.query([claude, gpt4, mistral])
  // Returns: consensus response + individual responses
```

### 5. Cost Attribution

Track per-caller costs and enforce budgets.

```
User A budget: $10/day
  Request 1: $0.05
  Request 2: $0.03
  Request 3: $9.92
  → Budget exhausted, requests denied

User B budget: $50/month
  Used: $12.47 of $50.00
```

### 6. Rate Limiting

Per-caller, per-provider rate limits with token bucket.

```
claude: 100 req/min per caller
gpt-4: 50 req/min per caller
local: unlimited
```

---

## Monitoring & Observability

### Prometheus Metrics

```
ai_requests_total{provider="claude",model="claude-3-haiku"}
ai_tokens_total{provider="gpt-4",direction="input"}
ai_cost_total{caller_id="user-123"}
ai_latency_seconds{provider="mistral",quantile="0.95"}
circuit_breaker_state{provider="deepseek"} # 0=closed, 1=open, 2=half-open
```

### Grafana Dashboards

**Pre-built dashboards:**
1. **Provider Health** – Circuit breaker state, error rates, latency p95/p99
2. **Cost Analysis** – Per-provider costs, per-caller spending, budget usage
3. **Request Analytics** – Throughput, token rates, cache hit rates
4. **Ensemble Performance** – Consensus accuracy, voting patterns

### Jaeger Tracing

Distributed tracing across all layers:

```
user_request
  ├─ security_check (5ms)
  ├─ cache_lookup (1ms) → MISS
  ├─ provider_call
  │   ├─ circuit_breaker_check (0.2ms)
  │   ├─ http_request (450ms)
  │   └─ ahf_verification (12ms)
  ├─ cache_store (2ms)
  └─ response (469ms total)
```

---

## Security Integration

### Capability-Based Access Control

```
Caller: "user-app-1"
Capabilities:
  ✅ ai.chat:claude (max 100 req/min)
  ✅ ai.chat:gpt-4 (max 50 req/min)
  ❌ ai.chat:custom-provider (denied)
  ✅ ai.cost:view (own usage only)
  ❌ ai.admin:* (denied)
```

### Secrets Management

API keys stored in Omnisystem's SecretsVault:
- Never logged
- Encrypted at rest
- Rotated on schedule
- Audit logged on access

### Request Validation

- Input sanitization (prompt injection prevention)
- Output filtering (PII redaction)
- AHF hallucination detection
- HDE deterministic fallback

---

## Integration with BonsaiEcosystem

### 1. Bonsai Buddy (Universal Assistant)

```typescript
// BonsaiEcosystem/buddy/src/ai.ts
import { AiShim } from '@omnisystem/ai-shim';

const shim = new AiShim('http://ai-shim:8117');

async function askBuddy(question: string) {
  const response = await shim.chat({
    provider: 'claude',  // or user's preferred provider
    model: 'claude-3-haiku',
    messages: [{role: 'user', content: question}],
    options: {temperature: 0.7, max_tokens: 1000}
  });
  
  return response.content;
}
```

### 2. Bonsai Workspace (IDE)

```typescript
// BonsaiEcosystem/workspace/src/ai-assistant.ts
class CodeAssistant {
  async getCompletions(context: CodeContext) {
    const prompt = `
      Complete the following code:
      ${context.prefix}|${context.suffix}
    `;
    
    return await this.aiShim.chat({
      provider: 'claude',
      model: 'claude-3-haiku',
      messages: [{role: 'user', content: prompt}],
      options: {temperature: 0.3, max_tokens: 500}  // Low temp for code
    });
  }
}
```

### 3. Control Panel

```rust
// BonsaiEcosystem/control-panel/src/ai_config.rs
pub async fn configure_ai(config: AiConfig) {
  // Register providers
  shim.register_provider_with_cb(
    claude_adapter, 
    failure_threshold: 5
  );
  
  // Set cost budgets
  shim.set_budget("workspace-user", 50_000_000);  // $50/month
  
  // Enable features
  shim.enable_ensemble_mode();
  shim.enable_semantic_cache();
  
  // Save configuration
  state_store.set("ai_config", serialize(config));
}
```

---

## Testing

### Unit Tests

```bash
cd Omnisystem/services/ai
cargo test --lib
```

### Integration Tests

```bash
cd Omnisystem
cargo test --test ai_shim_integration
```

### End-to-End Tests

```bash
bash scripts/deploy-ai-shim.sh docker development
bash scripts/e2e-tests.sh
```

### Load Testing

```bash
# Using k6
k6 run scripts/load-test-ai-shim.js \
  --vus 50 \
  --duration 5m
```

---

## Configuration

### Environment Variables

```bash
# Service discovery
SECURITY_MGR_ADDR=security-mgr:9001
SECRETS_VAULT_ADDR=secrets-vault:9002
AHF_GATEWAY_ADDR=ahf-gateway:9010
HDE_ORCHESTRATOR_ADDR=hde-orchestrator:9011
STATE_STORE_ADDR=state-store:9003
MONITORING_ADDR=monitoring:9004

# Feature flags
ENABLE_ENSEMBLE_MODE=true
ENABLE_SEMANTIC_CACHE=true
ENABLE_CIRCUIT_BREAKER=true
ENABLE_COST_TRACKING=true

# API Keys (from secrets system)
CLAUDE_API_KEY=sk-...
OPENAI_API_KEY=sk-...
GEMINI_API_KEY=...
```

### Configuration File (`ai-shim.toml`)

```toml
[server]
http_port = 8117
ws_port = 8217
request_timeout_secs = 30
max_concurrent_requests = 1000

[resilience]
circuit_breaker_threshold = 5
circuit_breaker_timeout_secs = 30
retry_max_attempts = 3
retry_initial_delay_ms = 100
retry_max_delay_ms = 10000

[cache]
semantic_cache_enabled = true
semantic_cache_threshold = 0.92
deduplication_window_secs = 60
max_cache_entries = 10000

[ensemble]
enabled = true
min_providers = 2
consensus_threshold = 0.8
voting_strategy = "weighted_by_confidence"

[cost]
tracking_enabled = true
budget_enforcement = true
cost_per_million_tokens = {claude_input: 3, claude_output: 15}

[providers]
claude_enabled = true
gpt_enabled = true
gemini_enabled = true
mistral_enabled = true
deepseek_enabled = true
ollama_enabled = true
ollama_url = "http://ollama:11434"
```

---

## Troubleshooting

### Services Won't Start

```bash
# Check logs
docker logs ai-shim
docker logs security-mgr
docker logs secrets-vault

# Check network
docker network ls
docker network inspect bonsai-net
```

### API Timeouts

```bash
# Check circuit breaker status
curl http://localhost:8117/api/v1/ai/health

# Increase timeout in config
request_timeout_secs = 60  # from 30
```

### High Latency

```bash
# Enable tracing
JAEGER_AGENT_HOST=localhost RUST_LOG=debug ./ai-shim

# Check provider latency
curl http://localhost:9090/api/v1/query?query=ai_latency_seconds

# Consider local Ollama fallback
enable_ollama = true
```

### Cost Budget Exceeded

```bash
# Increase budget
POST /api/v1/ai/budget/set
{
  "caller_id": "user-123",
  "limit_cents": 50_000_000  # $50.00
}

# Or use ensemble with cheaper providers
ensemble_min_providers = 2  # Get consensus from cheaper models
```

---

## Next Steps

1. **Deploy** – Run `deploy-ai-shim.sh`
2. **Configure** – Set API keys in SecretsVault
3. **Test** – Run integration tests
4. **Monitor** – Access Grafana at http://localhost:3000
5. **Integrate** – Use in BonsaiEcosystem apps

For issues: See logs in `Omnisystem/logs/` or Jaeger UI at http://localhost:16686

