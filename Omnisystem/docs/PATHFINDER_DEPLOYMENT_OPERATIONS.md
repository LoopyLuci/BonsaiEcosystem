# PATHFINDER Deployment & Operations Guide

**Version**: 1.0.0  
**Status**: Production Ready  
**Last Updated**: 2026-06-11  

---

## Table of Contents

1. [Pre-Deployment Checklist](#pre-deployment-checklist)
2. [Infrastructure Setup](#infrastructure-setup)
3. [Database Setup](#database-setup)
4. [Service Deployment](#service-deployment)
5. [Monitoring & Observability](#monitoring--observability)
6. [Health Checks](#health-checks)
7. [Scaling & Auto-Scaling](#scaling--auto-scaling)
8. [Backup & Recovery](#backup--recovery)
9. [Incident Response](#incident-response)
10. [Maintenance Windows](#maintenance-windows)

---

## Pre-Deployment Checklist

### Infrastructure Requirements

- [ ] Kubernetes cluster (v1.27+) provisioned
- [ ] Load balancer configured with SSL/TLS
- [ ] DNS records pointed to load balancer
- [ ] SSL certificates valid for 12+ months
- [ ] VPC/Network security groups configured
- [ ] Database backups enabled
- [ ] Object storage (S3/GCS) configured
- [ ] CDN (CloudFlare/Akamai) configured
- [ ] Email service (SMTP) tested
- [ ] SMS service (Twilio) API key verified
- [ ] Push notification service (Firebase) configured

### Code Requirements

- [ ] All tests passing (500+)
- [ ] Code coverage > 85%
- [ ] Security scan completed (Trivy/SonarQube)
- [ ] OWASP Top 10 vulnerabilities addressed
- [ ] Dependencies audited (no critical CVEs)
- [ ] Performance benchmarks met (P95 < 200ms)
- [ ] Docker images built and scanned
- [ ] Kubernetes manifests validated
- [ ] Database migrations tested

### Team Requirements

- [ ] On-call rotation established
- [ ] Incident response plan reviewed
- [ ] Runbooks created for common issues
- [ ] Team trained on deployment process
- [ ] Communication channels set up (Slack, PagerDuty)
- [ ] Escalation policy defined

---

## Infrastructure Setup

### 1. Kubernetes Cluster Provisioning

```bash
# Using kubeadm (on-premises)
kubeadm init --pod-network-cidr=10.244.0.0/16

# Or cloud provider CLIs
# AWS EKS
aws eks create-cluster --name pathfinder --version 1.27 \
  --role-arn arn:aws:iam::ACCOUNT:role/eks-service-role

# GCP GKE
gcloud container clusters create pathfinder \
  --zone us-central1-a --num-nodes 5 \
  --machine-type n1-standard-4

# Azure AKS
az aks create --resource-group myResourceGroup \
  --name pathfinder --node-count 5
```

### 2. Network Configuration

```bash
# Install network plugin (Flannel)
kubectl apply -f https://raw.githubusercontent.com/coreos/flannel/master/Documentation/kube-flannel.yml

# Create namespace
kubectl create namespace pathfinder-prod

# Create network policies
kubectl apply -f kubernetes_deployment_config.yaml
```

### 3. Storage Setup

```bash
# Create persistent volumes
kubectl apply -f - <<EOF
apiVersion: v1
kind: PersistentVolume
metadata:
  name: postgres-pv
spec:
  capacity:
    storage: 100Gi
  accessModes:
    - ReadWriteOnce
  persistentVolumeReclaimPolicy: Retain
  storageClassName: fast
  hostPath:
    path: "/data/postgres"
EOF
```

### 4. Ingress Controller Setup

```bash
# Install NGINX Ingress Controller
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm install ingress-nginx ingress-nginx/ingress-nginx \
  -n ingress-nginx --create-namespace

# Install cert-manager for Let's Encrypt
helm repo add jetstack https://charts.jetstack.io
helm install cert-manager jetstack/cert-manager \
  --namespace cert-manager --create-namespace \
  --set installCRDs=true
```

---

## Database Setup

### 1. PostgreSQL Initialization

```bash
# Connect to PostgreSQL
kubectl exec -it postgres-0 -n pathfinder-prod -- psql

# Run migrations
psql -h postgres.pathfinder.svc.cluster.local \
  -U pathfinder -d pathfinder \
  -f database_migrations.sql

# Verify schema
\dt  # List tables
\l   # List databases
```

### 2. Database Configuration

```sql
-- Configure connection pooling
ALTER SYSTEM SET max_connections = 1000;
ALTER SYSTEM SET shared_buffers = '4GB';
ALTER SYSTEM SET effective_cache_size = '12GB';

-- Restart PostgreSQL
SELECT pg_reload_conf();
```

### 3. Backup Configuration

```bash
# Configure automated backups
kubectl apply -f - <<EOF
apiVersion: batch/v1
kind: CronJob
metadata:
  name: postgres-backup
  namespace: pathfinder-prod
spec:
  schedule: "0 2 * * *"  # 2 AM daily
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: backup
            image: postgres:15-alpine
            command:
            - /bin/sh
            - -c
            - |
              pg_dump -h postgres.pathfinder.svc.cluster.local \
                -U pathfinder pathfinder | \
                gzip > /backups/pathfinder-$(date +%Y%m%d).sql.gz
            volumeMounts:
            - name: backups
              mountPath: /backups
          volumes:
          - name: backups
            persistentVolumeClaim:
              claimName: backup-pvc
          restartPolicy: OnFailure
EOF
```

### 4. Redis Setup

```bash
# Deploy Redis with persistence
kubectl apply -f docker_compose_production.yaml

# Verify Redis connectivity
kubectl exec -it redis-0 -n pathfinder-prod -- redis-cli ping
# PONG
```

### 5. Neo4j Graph Database

```bash
# Initialize Neo4j
NEO4J_INITIAL_PASSWORD=your_secure_password \
kubectl apply -f - <<EOF
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: neo4j
  namespace: pathfinder-prod
spec:
  serviceName: neo4j
  replicas: 1
  selector:
    matchLabels:
      app: neo4j
  template:
    metadata:
      labels:
        app: neo4j
    spec:
      containers:
      - name: neo4j
        image: neo4j:5-enterprise
        ports:
        - containerPort: 7687
        - containerPort: 7474
        env:
        - name: NEO4J_AUTH
          value: "neo4j/your_secure_password"
        - name: NEO4J_dbms_memory_pagecache_size
          value: "2G"
EOF
```

---

## Service Deployment

### 1. Deploy All Services

```bash
# Apply configuration
kubectl apply -f kubernetes_deployment_config.yaml

# Verify deployments
kubectl get deployments -n pathfinder-prod
kubectl get pods -n pathfinder-prod

# Check service status
kubectl get services -n pathfinder-prod
```

### 2. Verify Service Readiness

```bash
# Check service endpoints
kubectl get endpoints -n pathfinder-prod

# Test health endpoints
kubectl port-forward -n pathfinder-prod svc/user-service 8001:8001
curl http://localhost:8001/health
# {
#   "status": "healthy",
#   "timestamp": "2026-06-11T15:30:00Z"
# }
```

### 3. Database Migrations

```bash
# Run migrations in pod
kubectl exec -it user-service-0 -n pathfinder-prod -- \
  ./migrate -path db/migrations -database $DATABASE_URL up

# Verify migrations
kubectl logs -f -n pathfinder-prod \
  deployment/user-service | grep -i migration
```

### 4. Service Configuration

```bash
# Update service environment variables
kubectl set env deployment/user-service \
  LOG_LEVEL=info \
  MAX_CONNECTIONS=1000 \
  -n pathfinder-prod

# Verify configuration
kubectl describe deployment user-service -n pathfinder-prod
```

---

## Monitoring & Observability

### 1. Prometheus Setup

```bash
# Deploy Prometheus
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm install prometheus prometheus-community/prometheus \
  -n pathfinder-prod \
  -f prometheus_monitoring_config.yaml

# Access Prometheus UI
kubectl port-forward -n pathfinder-prod svc/prometheus-server 9090:80
# Navigate to http://localhost:9090
```

### 2. Grafana Dashboards

```bash
# Deploy Grafana
helm repo add grafana https://grafana.github.io/helm-charts
helm install grafana grafana/grafana \
  -n pathfinder-prod \
  -f grafana_dashboards.json

# Get admin password
kubectl get secret -n pathfinder-prod grafana -o jsonpath="{.data.admin-password}" | base64 --decode

# Access Grafana
kubectl port-forward -n pathfinder-prod svc/grafana 3000:80
# Navigate to http://localhost:3000 (admin / <password>)
```

### 3. Metrics Configuration

```yaml
# prometheus_monitoring_config.yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'pathfinder-services'
    kubernetes_sd_configs:
      - role: pod
        namespaces:
          names:
            - pathfinder-prod
    relabel_configs:
      - source_labels: [__meta_kubernetes_pod_label_app]
        action: keep
        regex: (user|content|teacher|parent|notification|achievement|insights)-service

  - job_name: 'kubernetes-apiservers'
    kubernetes_sd_configs:
      - role: endpoints
    scheme: https
    tls_config:
      ca_file: /var/run/secrets/kubernetes.io/serviceaccount/ca.crt
    bearer_token_file: /var/run/secrets/kubernetes.io/serviceaccount/token
```

### 4. Logging Setup

```bash
# Deploy ELK Stack
helm repo add elastic https://helm.elastic.co
helm install elasticsearch elastic/elasticsearch \
  --namespace pathfinder-prod
helm install logstash elastic/logstash \
  --namespace pathfinder-prod
helm install kibana elastic/kibana \
  --namespace pathfinder-prod

# Configure service logging
kubectl apply -f - <<EOF
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluent-bit-config
  namespace: pathfinder-prod
data:
  fluent-bit.conf: |
    [SERVICE]
        Flush         5
        Daemon        off
        Log_Level     info

    [INPUT]
        Name              tail
        Path              /var/log/containers/*_pathfinder-prod_*.log
        Parser            docker
        Tag               kube.*

    [OUTPUT]
        Name            es
        Match           kube.*
        Host            elasticsearch.pathfinder.svc.cluster.local
        Port            9200
        HTTP_User       elastic
        HTTP_Passwd     password
        Logstash_Format On
        Retry_Limit     5
EOF
```

---

## Health Checks

### 1. Kubernetes Liveness Probes

```yaml
livenessProbe:
  httpGet:
    path: /health
    port: 8001
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3
```

### 2. Readiness Probes

```yaml
readinessProbe:
  httpGet:
    path: /ready
    port: 8001
  initialDelaySeconds: 10
  periodSeconds: 5
  timeoutSeconds: 3
  failureThreshold: 3
```

### 3. Custom Health Endpoint

```go
// backend_user_service_main.go
func healthHandler(w http.ResponseWriter, r *http.Request) {
  db := getDatabase()
  redis := getRedis()
  
  status := "healthy"
  if !db.Ping() {
    status = "unhealthy"
  }
  if !redis.Ping() {
    status = "unhealthy"
  }
  
  w.Header().Set("Content-Type", "application/json")
  json.NewEncoder(w).Encode(map[string]interface{}{
    "status": status,
    "timestamp": time.Now(),
    "database": map[string]bool{"connected": db.Ping()},
    "cache": map[string]bool{"connected": redis.Ping()},
  })
}
```

### 4. Endpoint Health Checks

```bash
# Script to monitor all endpoints
#!/bin/bash

SERVICES=("user-service" "content-service" "teacher-service" "parent-service" 
          "notification-service" "achievement-service" "insights-service")

for service in "${SERVICES[@]}"; do
  response=$(curl -s http://$service:8001/health)
  status=$(echo $response | jq -r '.status')
  
  if [ "$status" != "healthy" ]; then
    echo "⚠️  $service is unhealthy"
    # Send alert
    curl -X POST https://hooks.slack.com/... \
      -d "{\"text\": \"$service health check failed\"}"
  else
    echo "✅ $service is healthy"
  fi
done
```

---

## Scaling & Auto-Scaling

### 1. Horizontal Pod Autoscaling

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: user-service-hpa
  namespace: pathfinder-prod
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: user-service
  minReplicas: 3
  maxReplicas: 20
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
  behavior:
    scaleUp:
      stabilizationWindowSeconds: 0
      policies:
      - type: Percent
        value: 100
        periodSeconds: 30
    scaleDown:
      stabilizationWindowSeconds: 300
      policies:
      - type: Percent
        value: 50
        periodSeconds: 60
```

### 2. Load Testing & Scaling Verification

```bash
# Run load test
k6 run performance_load_tests.js --vus 100 --duration 60s

# Monitor scaling
kubectl get hpa -n pathfinder-prod -w

# Check replica count
kubectl get pods -n pathfinder-prod -l app=user-service
```

### 3. Database Scaling

```bash
# Scale PostgreSQL replicas
kubectl scale statefulset postgres --replicas=5 -n pathfinder-prod

# Verify replication
kubectl exec -it postgres-0 -n pathfinder-prod -- psql -c "SELECT * FROM pg_stat_replication;"
```

---

## Backup & Recovery

### 1. Automated Backups

```bash
# Create backup job
kubectl apply -f - <<EOF
apiVersion: batch/v1
kind: CronJob
metadata:
  name: database-backup
  namespace: pathfinder-prod
spec:
  schedule: "0 2 * * *"
  jobTemplate:
    spec:
      template:
        spec:
          serviceAccountName: backup-sa
          containers:
          - name: backup
            image: postgres:15-alpine
            command:
            - /bin/sh
            - -c
            - |
              BACKUP_FILE="/backups/pathfinder-\$(date +%Y%m%d-%H%M%S).sql.gz"
              pg_dump -h postgres.pathfinder.svc.cluster.local \
                -U pathfinder pathfinder | gzip > \$BACKUP_FILE
              
              # Upload to S3
              aws s3 cp \$BACKUP_FILE s3://pathfinder-backups/
          restartPolicy: OnFailure
EOF
```

### 2. Testing Backups

```bash
# Verify backup integrity
pg_restore --list /backups/pathfinder-20260611.sql.gz | head -20

# Test restore (on test database)
pg_restore -h localhost -U pathfinder -d pathfinder_test \
  /backups/pathfinder-20260611.sql.gz
```

### 3. Recovery Procedures

```bash
# Step 1: Identify backup to restore
aws s3 ls s3://pathfinder-backups/ | tail -5

# Step 2: Download backup
aws s3 cp s3://pathfinder-backups/pathfinder-20260611.sql.gz /tmp/

# Step 3: Restore to new database
gunzip /tmp/pathfinder-20260611.sql.gz
psql -h postgres.pathfinder.svc.cluster.local \
  -U pathfinder -d pathfinder_restore < /tmp/pathfinder-20260611.sql

# Step 4: Verify restore
psql -h postgres.pathfinder.svc.cluster.local \
  -U pathfinder -d pathfinder_restore -c "SELECT COUNT(*) FROM users;"
```

---

## Incident Response

### 1. Incident Severity Levels

| Level | Response Time | Impact | Examples |
|-------|---------------|--------|----------|
| **Critical (P1)** | 5 minutes | Complete service outage | Database down, all endpoints 500 |
| **High (P2)** | 15 minutes | Significant degradation | 50%+ latency increase, error rate >5% |
| **Medium (P3)** | 30 minutes | Partial impact | Single service slow, 1-5% error rate |
| **Low (P4)** | 4 hours | Minimal impact | Non-critical feature slow |

### 2. Incident Response Process

```bash
#!/bin/bash
# incident_response.sh

INCIDENT_ID=$(date +%s)
SEVERITY=$1  # P1, P2, P3, P4

echo "🚨 Incident $INCIDENT_ID - Severity: $SEVERITY"

# 1. Acknowledge incident
curl -X POST https://api.pagerduty.com/incidents \
  -H "Authorization: Token token=$PAGERDUTY_TOKEN" \
  -d "{\"incident\": {\"type\": \"incident\", \"title\": \"PATHFINDER Alert\", \"service\": {\"type\": \"service_reference\", \"id\": \"PATHFINDER\"}, \"urgency\": \"$SEVERITY\"}}"

# 2. Notify team
curl -X POST $SLACK_WEBHOOK \
  -d "{\"text\": \"🚨 Incident $INCIDENT_ID detected - Severity: $SEVERITY\"}"

# 3. Gather diagnostics
kubectl get pods -n pathfinder-prod --field-selector=status.phase!=Running
kubectl get events -n pathfinder-prod --sort-by='.lastTimestamp' | tail -20
kubectl logs -n pathfinder-prod --all-containers=true -l app=user-service --tail=100

# 4. Start incident war room
# Create Google Meet / Slack huddle link

# 5. Document incident
echo "Incident $INCIDENT_ID started at $(date)" >> incident_log.txt
```

### 3. Common Issues & Solutions

#### Issue: Pod Restarts Continuously

```bash
# Check pod logs
kubectl logs -n pathfinder-prod user-service-xyz --previous

# Check resource limits
kubectl top pods -n pathfinder-prod

# Check events
kubectl describe pod user-service-xyz -n pathfinder-prod

# Solution: Increase resources or fix application
kubectl set resources deployment user-service \
  --limits=cpu=1000m,memory=512Mi \
  -n pathfinder-prod
```

#### Issue: High Latency

```bash
# Check database connections
kubectl exec -it postgres-0 -n pathfinder-prod -- \
  psql -c "SELECT count(*) FROM pg_stat_activity;"

# Check Redis memory
kubectl exec -it redis-0 -n pathfinder-prod -- \
  redis-cli info memory

# Check network connectivity
kubectl exec -it user-service-0 -n pathfinder-prod -- \
  ping postgres.pathfinder.svc.cluster.local
```

#### Issue: Memory Leak

```bash
# Monitor memory over time
kubectl top pods -n pathfinder-prod --containers

# Check for goroutine leaks (Go services)
curl http://user-service:8001/debug/pprof/goroutine

# Enable memory profiling
kubectl set env deployment/user-service \
  ENABLE_PROFILING=true \
  -n pathfinder-prod

# Download and analyze profile
kubectl exec -it user-service-0 -n pathfinder-prod -- \
  curl http://localhost:6060/debug/pprof/heap > heap.prof
go tool pprof heap.prof
```

---

## Maintenance Windows

### 1. Zero-Downtime Deployment

```bash
# Update image
kubectl set image deployment/user-service \
  user-service=ghcr.io/pathfinder/user-service:v1.2.0 \
  -n pathfinder-prod

# Monitor rollout
kubectl rollout status deployment/user-service -n pathfinder-prod

# Verify deployment
kubectl get pods -n pathfinder-prod -l app=user-service

# Rollback if needed
kubectl rollout undo deployment/user-service -n pathfinder-prod
```

### 2. Database Migrations

```bash
# Run migration in maintenance window
kubectl exec -it postgres-0 -n pathfinder-prod -- \
  psql -c "
    BEGIN TRANSACTION;
    ALTER TABLE users ADD COLUMN new_column VARCHAR(255);
    COMMIT;
  "

# Verify migration
kubectl exec -it postgres-0 -n pathfinder-prod -- \
  psql -c "\\d users"
```

### 3. Scheduled Maintenance

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: maintenance-schedule
  namespace: pathfinder-prod
data:
  schedule: |
    # Maintenance windows (UTC)
    # Tuesday 2:00-3:00 AM UTC
    # Friday 2:00-3:00 AM UTC
```

---

## Runbooks

### Runbook: Service Restart

1. Identify problematic service
2. `kubectl delete pod <pod-name> -n pathfinder-prod`
3. Monitor new pod startup (kubectl get pods -w)
4. Verify service health (curl /health)
5. Check logs for startup errors

### Runbook: Database Failover

1. Identify primary database failure
2. Promote replica: `SELECT pg_ctl('promote')`
3. Update connection strings in services
4. Restart affected services
5. Verify data consistency

### Runbook: Certificate Renewal

1. Check cert expiration: `kubectl get certificate -n pathfinder-prod`
2. cert-manager auto-renews, but can force: `kubectl delete certificate pathfinder-tls -n pathfinder-prod`
3. Monitor renewal: `kubectl get certificaterequests -n pathfinder-prod`
4. Verify new cert: `kubectl get secret pathfinder-tls -o jsonpath='{.data.tls\.crt}' | base64 -d | openssl x509 -noout -dates`

---

## Contact & Escalation

**Primary On-Call**: [Contact info]  
**Backup On-Call**: [Contact info]  
**Engineering Lead**: [Contact info]  
**CTO**: [Contact info]  

**Escalation Path**:
- P4 (Low): Team lead (4 hours)
- P3 (Medium): Engineering lead (30 min)
- P2 (High): Team + Engineering lead (15 min)
- P1 (Critical): All + CTO (5 min)

---

**Last Updated**: 2026-06-11  
**Next Review**: 2026-07-11
