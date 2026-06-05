# 📚 Complete Bonsai Ecosystem Guide

**Comprehensive training and documentation for the entire Bonsai Ecosystem, from fundamentals to production deployment.**

---

## TABLE OF CONTENTS

1. [Quick Start](#quick-start)
2. [Architecture Overview](#architecture-overview)
3. [Core Concepts](#core-concepts)
4. [Component Guides](#component-guides)
5. [Deployment Patterns](#deployment-patterns)
6. [Operations & Monitoring](#operations--monitoring)
7. [Troubleshooting](#troubleshooting)
8. [Advanced Topics](#advanced-topics)

---

## QUICK START

### Installation

```bash
# Clone Bonsai Workspace
git clone https://github.com/bonsai/bonsai-workspace.git
cd bonsai-workspace

# Build all crates
cargo build --release

# Run tests
cargo test --all

# Start BCF (Bonsai Container Fabric)
./target/release/bonsai-bcf
```

### Deploy Your First Container

```bash
# Create a simple deployment
cat > hello-world.bp << 'EOF'
name: "hello-world"
version: "1.0.0"
containers:
  - name: "web"
    image: "bonsai://hello-world:latest"
    replicas: 3
    resources:
      cpu_cores: 0.5
      memory_mib: 256
services:
  - name: "web-service"
    ports:
      - port: 80
        targetPort: 8080
EOF

# Deploy
bonsai container deploy --blueprint hello-world.bp

# Check status
bonsai container status --service web-service

# View logs
bonsai container logs --service web-service --tail 100
```

---

## ARCHITECTURE OVERVIEW

### The Bonsai Stack

```
┌─────────────────────────────────────────────┐
│         User Applications (Your Code)        │
├─────────────────────────────────────────────┤
│  Bonsai Container Fabric (BCF)              │
│  ├─ Blueprint (declarative specs)           │
│  ├─ Pulse (scheduler)                       │
│  ├─ Sanctum (vault runtime)                 │
│  └─ TransferDaemon (service mesh)           │
├─────────────────────────────────────────────┤
│  Bonsai Ecosystem Services                  │
│  ├─ Profiler (performance analysis)         │
│  ├─ Observability (metrics, traces, logs)   │
│  ├─ Coverage (test analysis)                │
│  ├─ Resilience (circuit breaker, retry)     │
│  ├─ Security (SBOM, encryption, scanning)   │
│  └─ Optimization (lock-free, SIMD)          │
├─────────────────────────────────────────────┤
│  UOSC (Bonsai Microkernel)                  │
│  ├─ Sentinel Core (resource enforcement)    │
│  ├─ Sanctum (hardware isolation)            │
│  ├─ Echo Fabric (P2P networking)            │
│  ├─ CAS (content-addressed storage)         │
│  ├─ Conduit (IPC messaging)                 │
│  └─ Universe (event logging)                │
└─────────────────────────────────────────────┘
```

### Core Principles

1. **Sovereign** – No external dependencies, complete control
2. **Hardware-Enforced** – Security via Sentinel Core, not software tricks
3. **Self-Healing** – Survival System auto-repairs, rolls back, learns
4. **AI-Native** – EternalTrainingLoop optimizes everything
5. **Observable** – Every action is a Universe event
6. **P2P** – Native Echo fabric, no central points of failure
7. **Immutable** – Crystal images, declarative blueprints, atomic swaps
8. **Real-Time** – Pulse scheduler with EDF+CBS guarantees
9. **Energy-Aware** – Carbon-conscious workload placement
10. **Secure by Default** – Zero-trust capability model

---

## CORE CONCEPTS

### 1. Blueprint (Declarative Deployment)

A Blueprint is a single file that declares your entire deployment:

```yaml
name: "production-api"
version: "2.0.0"

containers:
  - name: "api-server"
    image: "bonsai://api:v2.0"
    replicas: 5
    resources:
      cpu_cores: 2.0
      memory_mib: 1024
    network:
      ports:
        - container_port: 8080
          service_port: 80
    probes:
      liveness:
        http_get: { path: "/health", port: 8080 }
      readiness:
        http_get: { path: "/ready", port: 8080 }
    autoscaling:
      min_replicas: 3
      max_replicas: 20
      target_cpu_percent: 70
    update_strategy:
      type: "canary"
      canary_weight_percent: 10

services:
  - name: "api"
    load_balancing: "least-latency"
    circuit_breaker:
      enabled: true
      threshold: 5
```

**Key benefits:**
- Single source of truth
- Version-controlled deployments
- Atomic upgrades with instant rollback
- No YAML sprawl (Helm, Kustomize, etc.)

### 2. Sanctum Vaults (Hardware Isolation)

Each container runs in a **Sanctum vault** – a hardware-isolated VM:

```
┌─────────────────────────┐
│  Sanctum Vault (VM)     │
├─────────────────────────┤
│  Container Process      │
│  • Isolated memory      │
│  • Restricted syscalls  │
│  • Capability tokens    │
│  • Read-only root FS    │
└─────────────────────────┘
     ↓ enforced by
┌─────────────────────────┐
│  Sentinel Core          │
├─────────────────────────┤
│  • Resource limits      │
│  • Memory management    │
│  • CPU allocation       │
│  • Device assignment    │
└─────────────────────────┘
```

**Why it matters:**
- A compromised container cannot escape to the host
- Resource limits are hardware-enforced (not kernel tricks)
- Isolation is physically impossible to break

### 3. Pulse Scheduler (Real-Time)

Schedules containers with **real-time guarantees**:

```
EDF+CBS (Earliest Deadline First + Constant Bandwidth Server)
├─ Real-time containers: deadline-driven scheduling
├─ Normal containers: fair-share bandwidth allocation
├─ Energy-aware: minimizes carbon footprint
└─ Distributed: every node runs the scheduler
```

**Example: Real-time container**
```yaml
deadline_us: 10000  # 10ms latency guarantee
period_us: 100000   # runs every 100ms
```

### 4. TransferDaemon Service Mesh (P2P)

Built-in service mesh with **no overhead**:

```
Container A → [wants to call api service]
         ↓
Echo Fabric (service discovery)
         ↓
[finds 3 instances of api service]
         ↓
Load Balancer (least-latency)
         ↓
Direct P2P tunnel to Container B
         ↓
QUIC encrypted connection (~1ms latency)
```

**No sidecars, no iptables, no overlay networks.**

### 5. Survival System (Self-Healing)

Automatically detects and recovers from failures:

```
Container crashes
         ↓
Universe event logged
         ↓
Survival System detects (100ms)
         ↓
Auto-restart
         ↓
If restart loop detected:
  → Increase memory limit
  → Scale up replicas
  → Trigger Bug Hunt analysis
  → Propose code changes
```

---

## COMPONENT GUIDES

### Bonsai Profiler

**Measure performance of your services:**

```bash
# Profile a service
bonsai profile --service api-server --duration 60s

# Generate report
bonsai profile report --output report.html

# Benchmark a function
bonsai bench --function myapp::process_request --samples 1000
```

**Output includes:**
- Flamegraph (CPU hotspots)
- Memory allocation tracking
- Async operation metrics
- Hot-path detection
- Performance trends

### Bonsai Observability Stack

**Complete visibility into your system:**

```bash
# View metrics
curl http://localhost:8081/metrics

# View SLA compliance
curl http://localhost:8081/sla

# View system health
bonsai health --watch

# View dashboards
open http://localhost:11425/universe
```

**Metrics tracked:**
- Request latency (p50, p95, p99, p999)
- Error rate
- Throughput
- Resource usage
- SLA compliance

### Bonsai CI/CD

**Automated testing and deployment:**

```bash
# Trigger PR validation
bonsai ci run --workflow pr_validation

# Run nightly soak tests
bonsai ci run --workflow nightly_soak

# Deploy to production
bonsai container deploy --blueprint prod.bp

# Watch rollout progress
bonsai container rollout status --service api
```

### Bonsai Coverage

**Ensure code quality:**

```bash
# Run coverage analysis
cargo tarpaulin --all

# Check gates
bonsai coverage enforce --target 85%

# View trend
bonsai coverage trend --days 30
```

### Bonsai Security

**Keep your system secure:**

```bash
# Generate SBOM
bonsai sbom generate --output sbom.json

# Scan for secrets
bonsai secrets scan --path .

# Check vulnerabilities
bonsai vuln scan --image myapp:v1.0

# Audit security
bonsai security audit --comprehensive
```

---

## DEPLOYMENT PATTERNS

### 1. Canary Deployment

Gradually roll out new version:

```yaml
update_strategy:
  type: "canary"
  canary_weight_percent: 5
  interval_seconds: 60
  metrics:
    - name: "error_rate"
      threshold: 0.01
    - name: "p95_latency_ms"
      threshold: 500
```

**Flow:**
- 5% traffic to new version
- Monitor for 1 minute
- If metrics good: increase to 10%, 25%, 50%, 100%
- If metrics bad: auto-rollback

### 2. Blue-Green Deployment

Switch instantly between versions:

```yaml
update_strategy:
  type: "blue_green"
```

**Flow:**
- Deploy new version (green) alongside old (blue)
- Run smoke tests
- Switch 100% traffic to green
- Keep blue as instant rollback

### 3. Rolling Update

Gradual pod replacement:

```yaml
update_strategy:
  type: "rolling"
  max_surge: 1
  max_unavailable: 0
```

---

## OPERATIONS & MONITORING

### Daily Checklist

- [ ] Check health dashboard (no alerts)
- [ ] Review error rates (< 0.1%)
- [ ] Check SLA compliance (> 99.95%)
- [ ] Monitor resource usage (not near limits)
- [ ] Review logs for anomalies

### Weekly Tasks

- [ ] Performance analysis (trends)
- [ ] Security audit (no new vulnerabilities)
- [ ] Coverage check (no regressions)
- [ ] Capacity planning (trends)

### Monthly Reviews

- [ ] Analyze metrics (anomalies?)
- [ ] Update runbooks
- [ ] Plan upgrades
- [ ] Team training

### Emergency Response

**If service is down:**

```bash
# 1. Check status
bonsai container status --service <name>

# 2. View recent logs
bonsai container logs --service <name> --tail 50

# 3. Check events
bonsai universe events --service <name> --recent

# 4. Rollback if needed
bonsai container rollout undo --service <name>

# 5. Scale down if looping
bonsai container scale --service <name> --replicas 1

# 6. Debug pod
bonsai container exec --service <name> -- bash
```

---

## TROUBLESHOOTING

### Container won't start

**Symptoms:** Pod stays in "Creating" state

**Diagnosis:**
```bash
bonsai container logs --service <name> --previous
bonsai describe pod --service <name>
```

**Common causes:**
- Image not found: check `image:` in Blueprint
- Memory exhausted: increase `memory_mib`
- Port already in use: check port mappings
- Health check too strict: loosen thresholds

**Fix:**
```yaml
# Increase memory
resources:
  memory_mib: 2048  # was 1024

# Relax health check
probes:
  liveness:
    initial_delay_seconds: 30  # give more time
    failure_threshold: 5       # allow more failures
```

### High error rate

**Diagnosis:**
```bash
curl http://localhost:8081/sla
bonsai container logs --service <name> --errors
```

**Common causes:**
- Downstream service down
- Database connection error
- Configuration mismatch
- Resource exhaustion

**Fix:**
```bash
# Scale up
bonsai container scale --service <name> --replicas 10

# Check downstream service
bonsai container status --service database

# Restart
bonsai container restart --service <name>

# Rollback
bonsai container rollout undo --service <name>
```

### Slow endpoints

**Diagnosis:**
```bash
# Check latency
curl -w "@curl-format.txt" http://service.svc.bonsai

# Check profiling
bonsai profile --service <name> --duration 30s

# Check resource usage
bonsai container stats --service <name>
```

**Common causes:**
- CPU constrained: increase `cpu_cores`
- Memory pressure: increase `memory_mib`
- Slow database queries: profile the app
- Network latency: check peers

**Fix:**
```yaml
resources:
  cpu_cores: 4.0        # was 2.0
  memory_mib: 2048      # was 1024
```

---

## ADVANCED TOPICS

### Custom Resource Limits

```yaml
containers:
  - name: "compute-heavy"
    resources:
      cpu_cores: 8.0
      memory_mib: 8192
      gpu:
        type: "nvidia-a100"
        count: 1
    deadline_us: 50000   # real-time, 50ms deadline
```

### Network Policies

```yaml
network_policies:
  - name: "deny-all-ingress"
    pod_selector: {}
    policy_types: ["Ingress"]

  - name: "allow-api-traffic"
    pod_selector: { app: "api-server" }
    policy_types: ["Ingress"]
    ingress:
      - from:
          - pod_selector: { role: "frontend" }
        ports:
          - protocol: "tcp"
            port: 8080
```

### Persistent Volumes

```yaml
volumes:
  - name: "database-data"
    size_gib: 100
    type: "cas-persistent"
    replication: 3
    backup:
      enabled: true
      schedule: "0 2 * * *"
      retention_days: 30
```

### Affinity Rules

```yaml
affinity:
  pod_anti_affinity:
    required:
      - label_selector: { app: "api-server" }
        topology_key: "bonsai.io/zone"
```

### Integration with EternalTrainingLoop

```yaml
bonsai_integration:
  eternal_training_loop:
    enabled: true
    optimization_target: "cost"  # cost, latency, carbon, balanced
```

---

## BEST PRACTICES

### Do's ✓

- ✓ Use declarative Blueprints (version in Git)
- ✓ Set resource requests/limits (hard limits)
- ✓ Define health checks (liveness + readiness)
- ✓ Use canary deployments (test before full rollout)
- ✓ Monitor everything (metrics, logs, events)
- ✓ Plan for failure (auto-scaling, rollback, replicas)
- ✓ Secure by default (capability tokens, TLS)
- ✓ Automate deployments (no manual configs)

### Don'ts ✗

- ✗ Don't manually configure containers (use Blueprints)
- ✗ Don't skip health checks
- ✗ Don't over-provision resources (costs money)
- ✗ Don't ignore warnings in security audits
- ✗ Don't deploy without testing (use canary)
- ✗ Don't run single replicas in production
- ✗ Don't disable autoscaling
- ✗ Don't ignore metrics and logs

---

## GETTING HELP

### Documentation

- [Bonsai Container Fabric Spec](./BONSAI-CONTAINER-FABRIC-SPECIFICATION.md)
- [STEP 1: Profiling Guide](./docs/performance-profiling-guide.md)
- [STEP 2: Observability Guide](./docs/observability-guide.md)
- [STEP 3: CI/CD Guide](./STEP3-CI-CD-DEPLOYMENT.md)
- [STEP 6: Deployment Guide](./STEP6-DEPLOYMENT-AUTOMATION.md)

### Community

- GitHub Issues: https://github.com/bonsai/bonsai-workspace/issues
- Slack: #bonsai-support
- Discussions: https://github.com/bonsai/bonsai-workspace/discussions

### Training

- Online courses: https://bonsai.sh/training
- Video tutorials: https://youtube.com/@bonsai
- Webinars: Monthly, register at bonsai.sh

---

**This guide is living documentation. Updates at each release.**

**Last updated:** 2026-06-02  
**Version:** 1.0.0 (Bleeding Edge)

🚀 Welcome to the future of containerization! 🚀
