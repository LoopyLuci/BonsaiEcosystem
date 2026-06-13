# STEP 6: Production Deployment Automation

## Overview

This document covers automated production deployment of Bonsai to Kubernetes with canary rollouts, health checks, and automatic rollback.

**Status:** ✅ Ready for Production  
**Components:** 4 (Deployment manifest, Canary config, Deployment script, Monitoring)  
**Timeline:** 2-3 days  

---

## Architecture

### Deployment Stack

```
Bonsai Production Deployment
├── Kubernetes Cluster
│   ├── bonsai-deployment.yaml (3-10 replicas, HPA)
│   ├── bonsai-canary-deployment.yaml (gradual rollout)
│   └── Service + Monitoring
│
├── Deployment Automation
│   ├── deploy-to-kubernetes.ps1 (orchestrator)
│   ├── Backup creation
│   ├── Health checks
│   └── Rollback on failure
│
├── Traffic Management (Istio)
│   ├── VirtualService (routing rules)
│   ├── DestinationRule (circuit breaking)
│   └── Retry policies
│
└── Monitoring & Alerting
    ├── ServiceMonitor (Prometheus scraping)
    ├── Canary alerts
    └── Performance metrics
```

### Deployment Stages

```
Push Image to Registry
    ↓
Run deploy-to-kubernetes.ps1
    ↓
Create Namespace (bonsai)
    ↓
Apply Base Deployment (3 replicas)
    ↓
Update Image
    ↓
Wait for Rollout
    ↓
Health Check
    ↓
(Canary) Gradual Scale 5% → 50% → 100%
    ↓
(Canary) Monitor Metrics (p95 latency, error rate)
    ↓
Success → Running
Failure → Automatic Rollback
```

---

## Prerequisites

### Cluster Requirements

- Kubernetes 1.24+
- 3+ nodes (for HA)
- 2GB+ memory per node
- 1+ CPU per node
- kubectl installed and configured
- (Optional) Istio 1.14+ (for advanced traffic management)
- (Optional) Flagger 1.10+ (for canary deployments)
- (Optional) Prometheus (for monitoring)

### Local Setup

```powershell
# Install kubectl
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/windows/amd64/kubectl.exe"

# Configure cluster access
kubectl config use-context <your-cluster>

# Verify connection
kubectl cluster-info
kubectl get nodes
```

---

## Deployment Steps

### Step 1: Build & Push Image

```powershell
# Build container image
docker build -t bonsai/bonsai-core:v0.1.0 .
docker build -t bonsai/bonsai-core:latest .

# Push to registry
docker push bonsai/bonsai-core:v0.1.0
docker push bonsai/bonsai-core:latest

# For AWS ECR
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account>.dkr.ecr.us-east-1.amazonaws.com
docker tag bonsai/bonsai-core:latest <account>.dkr.ecr.us-east-1.amazonaws.com/bonsai-core:latest
docker push <account>.dkr.ecr.us-east-1.amazonaws.com/bonsai-core:latest
```

### Step 2: Dry-Run Deployment

```powershell
# Validate deployment without applying
pwsh .\scripts\deploy-to-kubernetes.ps1 `
  -Cluster "production" `
  -Namespace "bonsai" `
  -Image "bonsai/bonsai-core:v0.1.0" `
  -DryRun

# Review output to ensure correctness
```

### Step 3: Deploy to Kubernetes

```powershell
# Standard deployment (with canary by default)
pwsh .\scripts\deploy-to-kubernetes.ps1 `
  -Cluster "production" `
  -Namespace "bonsai" `
  -Image "bonsai/bonsai-core:v0.1.0" `
  -Canary:$true `
  -CanaryWeight 5 `
  -WaitForReady:$true

# Without canary (immediate full rollout)
pwsh .\scripts\deploy-to-kubernetes.ps1 `
  -Cluster "production" `
  -Namespace "bonsai" `
  -Image "bonsai/bonsai-core:v0.1.0" `
  -Canary:$false
```

### Step 4: Monitor Deployment

```powershell
# Watch rollout progress
kubectl rollout status deployment/bonsai-core -n bonsai

# Real-time pod updates
kubectl get pods -n bonsai -l app=bonsai,component=core -w

# Check events
kubectl get events -n bonsai --sort-by='.lastTimestamp'

# View pod logs
kubectl logs -n bonsai -l app=bonsai -f --tail=100

# Check service endpoints
kubectl get endpoints bonsai-core -n bonsai
```

---

## Canary Deployment

### How It Works

1. **Initial** - 5% traffic to new version
2. **Monitor** - Track metrics (latency, error rate, success)
3. **Gradual** - If good: 5% → 10% → 25% → 50% → 100%
4. **Rollback** - If metrics fail: automatic rollback to previous version

### Canary Configuration

Edit `deploy/kubernetes/bonsai-canary-deployment.yaml`:

```yaml
analysis:
  interval: 1m          # Check metrics every 1 minute
  threshold: 5          # Allow 5 check failures before rollback
  maxWeight: 50         # Max traffic to canary
  stepWeight: 5         # Increase by 5% each successful check

  metrics:
  - name: request-success-rate
    thresholdRange:
      min: 99           # Require 99%+ success rate
    interval: 1m
  - name: request-duration
    thresholdRange:
      max: 500          # p95 latency < 500ms
    interval: 1m
```

### Manual Canary Control

```powershell
# Pause canary
kubectl patch canary bonsai-core -n bonsai --type merge -p '{"spec":{"skipAnalysis":true}}'

# Resume canary
kubectl patch canary bonsai-core -n bonsai --type merge -p '{"spec":{"skipAnalysis":false}}'

# Watch canary progress
kubectl get canary -n bonsai -w

# Get canary status
kubectl describe canary bonsai-core -n bonsai
```

---

## Health Checks

### Liveness Probe

Checks if pod is alive (kills and restarts if failing):

```
GET /health/live HTTP/1.1
Host: 127.0.0.1:8082

Response: 200 OK
```

### Readiness Probe

Checks if pod is ready for traffic:

```
GET /health/ready HTTP/1.1
Host: 127.0.0.1:8082

Response: 200 OK
```

### Manual Health Check

```powershell
# Get service IP
$svc = kubectl get service bonsai-core -n bonsai -o json | ConvertFrom-Json
$ip = $svc.status.loadBalancer.ingress[0].ip

# Or for ClusterIP
$ip = $svc.spec.clusterIP

# Check liveness
curl http://$ip:8082/health/live

# Check readiness
curl http://$ip:8082/health/ready
```

---

## Rollback Procedures

### Automatic Rollback

Triggered by:
- Canary metrics exceeding thresholds
- Pod crash loop (>3 restarts)
- Readiness probe failure (>2 failures)
- Manual rollback command

```powershell
# Automatic rollback (to previous revision)
kubectl rollout undo deployment/bonsai-core -n bonsai

# Rollback to specific revision
kubectl rollout history deployment/bonsai-core -n bonsai
kubectl rollout undo deployment/bonsai-core -n bonsai --to-revision=3

# Via deployment script
pwsh .\scripts\deploy-to-kubernetes.ps1 -Cluster production -Namespace bonsai -Image <image> -Rollback
```

### Manual Recovery

```powershell
# 1. Check status
kubectl get deployment bonsai-core -n bonsai
kubectl get pods -n bonsai -o wide

# 2. Scale down problematic version
kubectl scale deployment bonsai-core -n bonsai --replicas=0

# 3. Update image to known-good version
kubectl set image deployment/bonsai-core `
  -n bonsai `
  bonsai-core=bonsai/bonsai-core:v0.0.1

# 4. Scale back up
kubectl scale deployment bonsai-core -n bonsai --replicas=3

# 5. Verify
kubectl rollout status deployment/bonsai-core -n bonsai
```

---

## Scaling

### Horizontal Pod Autoscaling

Automatically scales 3-10 replicas based on:
- CPU utilization > 70%
- Memory utilization > 80%

```powershell
# Check HPA status
kubectl get hpa -n bonsai

# Manual scaling
kubectl scale deployment bonsai-core -n bonsai --replicas=5

# Update HPA limits
kubectl patch hpa bonsai-core -n bonsai -p '{"spec":{"maxReplicas":15}}'
```

---

## Monitoring

### Metrics Collected

- Request count
- Request latency (p50, p95, p99)
- Error rate
- Pod memory/CPU
- Pod restart count
- Deployment status

### Prometheus Scraping

```yaml
scrape_configs:
- job_name: 'bonsai-core'
  kubernetes_sd_configs:
  - role: pod
    namespaces:
      names:
      - bonsai
  relabel_configs:
  - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_scrape]
    action: keep
    regex: true
  - source_labels: [__meta_kubernetes_pod_annotation_prometheus_io_path]
    action: replace
    target_label: __metrics_path__
    regex: (.+)
```

### Grafana Dashboards

Create dashboards to visualize:
- Deployment status (replicas, ready pods)
- Request metrics (throughput, latency, errors)
- Resource usage (CPU, memory, disk)
- Pod events (restarts, warnings)

---

## Troubleshooting

### Deployment Stuck in Progressing

```powershell
# Check status
kubectl get deployment bonsai-core -n bonsai -o yaml | grep -A 20 status:

# View events
kubectl describe deployment bonsai-core -n bonsai

# Check pods
kubectl get pods -n bonsai -o wide
kubectl logs -n bonsai <pod-name>

# Possible causes:
# - Image pull failure: check image registry access
# - Resource limits exceeded: check node capacity
# - Probe failures: check health check endpoints
```

### Pods Crashing

```powershell
# View crash logs
kubectl logs -n bonsai <pod-name> --previous

# Check events
kubectl describe pod -n bonsai <pod-name>

# Check resource limits
kubectl top pod -n bonsai

# Check container startup
kubectl get events -n bonsai --sort-by='.lastTimestamp' | grep Warning
```

### Service Not Accessible

```powershell
# Check service
kubectl get svc bonsai-core -n bonsai

# Check endpoints
kubectl get endpoints bonsai-core -n bonsai

# Check network policy
kubectl get networkpolicies -n bonsai

# Test connectivity
kubectl run -it --rm debug --image=busybox:1.28 --restart=Never -- sh
# Inside pod: wget http://bonsai-core:80/health/live
```

---

## Production Checklist

- [ ] Cluster prerequisites met (K8s 1.24+, 3+ nodes)
- [ ] Image built and pushed to registry
- [ ] Namespace created (bonsai)
- [ ] Dry-run deployment successful
- [ ] Monitoring configured (Prometheus/Grafana)
- [ ] Alert rules set up
- [ ] Rollback procedure tested
- [ ] Team trained on deployment
- [ ] Incident response plan ready
- [ ] Backup strategy in place

---

## Maintenance

### Daily
- Monitor pod status
- Check error rates
- Verify metrics collection

### Weekly
- Review canary deployments
- Check resource usage trends
- Update security patches

### Monthly
- Analyze deployment metrics
- Plan capacity upgrades
- Review incidents and learnings

---

## Related Documentation

- [MASTER_ACTION_PLAN.md](../MASTER_ACTION_PLAN.md) - 10-step implementation roadmap
- [IMPLEMENTATION_GUIDE_STEPS_2_10.md](../IMPLEMENTATION_GUIDE_STEPS_2_10.md) - Detailed specs
- [STEP3-CI-CD-DEPLOYMENT.md](./STEP3-CI-CD-DEPLOYMENT.md) - CI/CD setup
