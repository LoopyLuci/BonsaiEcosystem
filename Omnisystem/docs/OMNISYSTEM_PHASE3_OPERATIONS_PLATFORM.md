# Omnisystem Phase 3: Operations Platform

## Executive Summary

**Status**: ✅ SPECIFICATION COMPLETE - Ready for execution

**Phase 3 Objective**: Deploy and manage 1,039+ crates across production Kubernetes infrastructure with complete observability, security, compliance, and disaster recovery.

**Deliverables**:
- Infrastructure-as-Code (Terraform)
- Deployment automation (Helm charts)
- GitOps pipeline (ArgoCD)
- Secrets management (Vault)
- Backup & disaster recovery (Velero)
- RBAC & security policies
- Certificate management (Cert-Manager)

---

## Operations Platform Architecture

### Three-Tier Operations Model

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer (1,039+ crates)        │
├─────────────────────────────────────────────────────────────┤
│                    Kubernetes Platform Layer                 │
│  ┌──────────┬──────────┬────────────┬──────────┬───────┐    │
│  │ ArgoCD   │ Helm     │ Ingress    │ Service  │ RBAC  │    │
│  │ (GitOps) │ (Deploy) │ (Routes)   │ Mesh    │ (Auth)│    │
│  └──────────┴──────────┴────────────┴──────────┴───────┘    │
├─────────────────────────────────────────────────────────────┤
│                    Infrastructure Layer                      │
│  ┌──────────┬──────────┬────────────┬──────────┬───────┐    │
│  │ Terraform│ Vault    │ Velero     │ Cert-Mgr │Prometh│    │
│  │(Provision)│(Secrets) │(Backup)    │(Certs)   │(Metrics)   │
│  └──────────┴──────────┴────────────┴──────────┴───────┘    │
└─────────────────────────────────────────────────────────────┘
```

---

## 1. Infrastructure-as-Code (Terraform)

### File Structure

```
infrastructure/
├── terraform/
│   ├── main.tf                          # Main Kubernetes cluster
│   ├── networking.tf                    # VPC, subnets, security groups
│   ├── database.tf                      # PostgreSQL, Redis
│   ├── monitoring.tf                    # Prometheus, Grafana stack
│   ├── vault.tf                         # HashiCorp Vault setup
│   ├── velero.tf                        # Backup infrastructure
│   ├── variables.tf                     # Input variables
│   ├── outputs.tf                       # Output values
│   └── terraform.tfvars                 # Environment-specific values
│
└── k8s/
    ├── namespaces.yaml                  # Kubernetes namespaces
    ├── rbac.yaml                        # Role-based access control
    ├── network-policies.yaml             # Network segmentation
    └── security-policies.yaml            # Pod security policies
```

### Terraform Configuration

**main.tf**: Kubernetes Cluster Provisioning

```hcl
# GKE Cluster
resource "google_container_cluster" "omnisystem" {
  name     = "omnisystem-cluster"
  location = var.region
  
  # Node configuration
  node_pool {
    autoscaling {
      min_node_count = 3
      max_node_count = 100
    }
    
    node_config {
      machine_type = "n1-standard-4"
      disk_size_gb = 100
      oauth_scopes = [
        "https://www.googleapis.com/auth/cloud-platform"
      ]
    }
  }
  
  # Enable network policies
  network_policy {
    enabled = true
  }
  
  # Enable Workload Identity
  workload_identity_config {
    workload_pool = "${var.project_id}.svc.id.goog"
  }
}
```

**database.tf**: PostgreSQL & Redis

```hcl
# CloudSQL PostgreSQL
resource "google_sql_database_instance" "postgres" {
  database_version = "POSTGRES_15"
  
  settings {
    tier              = "db-custom-4-16384"
    availability_type = "REGIONAL"
    backup_configuration {
      enabled                        = true
      point_in_time_recovery_enabled = true
      backup_retention_days          = 30
    }
  }
}

# Cloud Memorystore Redis
resource "google_redis_instance" "cache" {
  name              = "omnisystem-cache"
  memory_size_gb    = 16
  tier              = "standard_ha"  # High availability
  transit_encryption_mode = "SERVER_AUTHENTICATION"
  auth_enabled      = true
}
```

**vault.tf**: HashiCorp Vault

```hcl
# Vault instance for secrets management
resource "google_compute_instance" "vault" {
  name         = "vault-server"
  machine_type = "n1-standard-2"
  
  boot_disk {
    initialize_params {
      image = "ubuntu-2004-lts"
      size  = 50
    }
  }
  
  # Startup script installs and configures Vault
  metadata_startup_script = file("${path.module}/vault_init.sh")
}

# Vault secret backend
resource "vault_mount" "omnisystem_secrets" {
  path        = "omnisystem"
  type        = "kv-v2"
  description = "Omnisystem application secrets"
}
```

**velero.tf**: Backup Infrastructure

```hcl
# Google Cloud Storage bucket for backups
resource "google_storage_bucket" "backup" {
  name          = "omnisystem-backups"
  location      = var.region
  force_destroy = false
  
  versioning {
    enabled = true
  }
  
  lifecycle_rule {
    action {
      type          = "Delete"
      storage_class = ["NEARLINE"]
    }
    condition {
      age = 90  # Delete old backups after 90 days
    }
  }
}

# Service account for Velero
resource "google_service_account" "velero" {
  account_id = "velero"
}
```

---

## 2. Deployment Automation (Helm)

### Helm Chart Structure

```
helm/
└── omnisystem/
    ├── Chart.yaml                       # Chart metadata
    ├── values.yaml                      # Default values
    ├── values-prod.yaml                 # Production overrides
    ├── values-staging.yaml              # Staging overrides
    │
    ├── templates/
    │   ├── namespace.yaml               # Create namespace
    │   ├── configmap.yaml               # Application config
    │   ├── secrets.yaml                 # Encrypted secrets
    │   ├── pvc.yaml                     # Persistent volumes
    │   ├── deployment.yaml              # Crate deployments
    │   ├── service.yaml                 # Kubernetes services
    │   ├── ingress.yaml                 # Ingress routing
    │   ├── hpa.yaml                     # Horizontal Pod Autoscaler
    │   ├── pdb.yaml                     # Pod Disruption Budget
    │   ├── networkpolicy.yaml           # Network policies
    │   ├── podsecuritypolicy.yaml       # Pod security
    │   ├── rbac.yaml                    # RBAC configuration
    │   ├── serviceaccount.yaml          # Service accounts
    │   └── monitors.yaml                # Prometheus monitors
    │
    └── charts/
        ├── postgresql/                  # PostgreSQL subchart
        ├── redis/                       # Redis subchart
        ├── prometheus/                  # Prometheus subchart
        └── grafana/                     # Grafana subchart
```

### Helm Chart: values.yaml

```yaml
# Omnisystem Helm Chart Values
global:
  environment: production
  domain: omnisystem.example.com

image:
  repository: docker.io/omnisystem
  tag: "1.0.0"
  pullPolicy: IfNotPresent

replicaCount: 3

resources:
  requests:
    memory: "512Mi"
    cpu: "250m"
  limits:
    memory: "2Gi"
    cpu: "1000m"

autoscaling:
  enabled: true
  minReplicas: 3
  maxReplicas: 100
  targetCPUUtilizationPercentage: 70
  targetMemoryUtilizationPercentage: 80

postgresql:
  enabled: true
  auth:
    username: omnisystem
    database: omnisystem
  primary:
    persistence:
      size: 100Gi
  metrics:
    enabled: true
    serviceMonitor:
      enabled: true

redis:
  enabled: true
  auth:
    enabled: true
  replica:
    replicaCount: 3
  metrics:
    enabled: true

prometheus:
  enabled: true
  retention: 30d
  
grafana:
  enabled: true
  adminPassword: ${GRAFANA_PASSWORD}

vault:
  enabled: true
  address: https://vault.example.com:8200
  role: omnisystem-role

velero:
  enabled: true
  schedule: "0 2 * * *"  # Daily at 2 AM
  retention: 30d

ingress:
  enabled: true
  className: nginx
  hosts:
    - host: api.omnisystem.example.com
      paths:
        - path: /
          pathType: Prefix

tls:
  enabled: true
  issuer: letsencrypt-prod
```

### Helm Deployment Commands

```bash
# Add Helm repositories
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
helm repo add grafana https://grafana.github.io/helm-charts

# Install Omnisystem with custom values
helm install omnisystem ./helm/omnisystem \
  --namespace omnisystem \
  --create-namespace \
  --values helm/omnisystem/values-prod.yaml \
  --set image.tag=1.0.0

# Upgrade deployment
helm upgrade omnisystem ./helm/omnisystem \
  --namespace omnisystem \
  --values helm/omnisystem/values-prod.yaml

# Rollback if needed
helm rollback omnisystem 1

# List all releases
helm list -n omnisystem
```

---

## 3. GitOps Pipeline (ArgoCD)

### ArgoCD Setup

**File**: `argocd/omnisystem-app.yaml`

```yaml
apiVersion: argoproj.io/v1alpha1
kind: Application
metadata:
  name: omnisystem
  namespace: argocd
spec:
  project: default
  
  source:
    repoURL: https://github.com/omnisystem/infrastructure
    targetRevision: main
    path: helm/omnisystem
    helm:
      releaseName: omnisystem
      values: |
        environment: production
        image:
          tag: "1.0.0"
  
  destination:
    server: https://kubernetes.default.svc
    namespace: omnisystem
  
  syncPolicy:
    automated:
      prune: true
      selfHeal: true
    syncOptions:
    - CreateNamespace=true
    
  # Automatically sync on changes
  syncOptions:
  - ApplyOutOfSyncOnly=true
```

**Continuous Deployment Workflow**:

1. Developer commits to main branch
2. CI/CD pipeline builds and tests crates
3. If all tests pass, Docker image is pushed
4. ArgoCD detects Git change
5. ArgoCD automatically syncs Helm chart
6. New version rolls out to cluster

---

## 4. Secrets Management (Vault)

### Vault Configuration

**File**: `vault/config.hcl`

```hcl
storage "file" {
  path = "/vault/data"
}

listener "tcp" {
  address       = "0.0.0.0:8200"
  tls_cert_file = "/vault/certs/tls.crt"
  tls_key_file  = "/vault/certs/tls.key"
}

api_addr     = "https://vault.example.com:8200"
cluster_addr = "https://vault.omnisystem.svc.cluster.local:8201"
```

### Kubernetes Integration

**Service Account & RBAC**:

```yaml
# Allow Kubernetes pods to authenticate with Vault
apiVersion: v1
kind: ServiceAccount
metadata:
  name: omnisystem
  namespace: omnisystem

---

# Vault authentication method
apiVersion: v1
kind: Secret
metadata:
  name: vault-auth
  namespace: omnisystem
type: kubernetes.io/service-account-token
automountServiceAccountToken: true

---

# ClusterRoleBinding for pod authentication
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: omnisystem-vault-auth
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: system:auth-delegator
subjects:
- kind: ServiceAccount
  name: omnisystem
  namespace: omnisystem
```

### Secrets Storage

```hcl
# Database credentials
path "omnisystem/data/database/postgres" {
  capabilities = ["read", "list"]
}

# API keys
path "omnisystem/data/api/keys" {
  capabilities = ["read", "list"]
}

# TLS certificates
path "omnisystem/data/tls/certificates" {
  capabilities = ["read", "list"]
}

# Encryption keys
path "omnisystem/data/encryption/keys" {
  capabilities = ["read", "list"]
}
```

---

## 5. Backup & Disaster Recovery (Velero)

### Velero Installation

```bash
# Install Velero
velero install \
  --provider gcp \
  --bucket omnisystem-backups \
  --secret-file ./credentials-velero
```

### Backup Policies

**File**: `velero/schedules.yaml`

```yaml
# Daily backup at 2 AM
apiVersion: velero.io/v1
kind: Schedule
metadata:
  name: omnisystem-daily
  namespace: velero
spec:
  schedule: "0 2 * * *"
  template:
    ttl: "720h"  # 30 days retention
    includedNamespaces:
    - omnisystem
    storageLocation: default
    volumeSnapshotLocation: default

---

# Hourly backup (last 24 hours only)
apiVersion: velero.io/v1
kind: Schedule
metadata:
  name: omnisystem-hourly
  namespace: velero
spec:
  schedule: "0 * * * *"
  template:
    ttl: "24h"
    includedNamespaces:
    - omnisystem

---

# Weekly full backup with retention
apiVersion: velero.io/v1
kind: Schedule
metadata:
  name: omnisystem-weekly
  namespace: velero
spec:
  schedule: "0 3 * * 0"  # Sunday 3 AM
  template:
    ttl: "2160h"  # 90 days retention
    includedNamespaces:
    - omnisystem
```

### Disaster Recovery Procedures

**Recovery Point Objective (RPO)**: 1 hour  
**Recovery Time Objective (RTO)**: 30 minutes

**Full Cluster Recovery**:

```bash
# List available backups
velero backup get

# Restore from specific backup
velero restore create --from-backup omnisystem-daily-20240101

# Monitor restoration
velero restore describe <restore-name>
```

---

## 6. Security & Compliance

### RBAC Configuration

**File**: `k8s/rbac.yaml`

```yaml
# Admin role for cluster administrators
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: omnisystem-admin
rules:
- apiGroups: ["*"]
  resources: ["*"]
  verbs: ["*"]

---

# Reader role for monitoring systems
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: omnisystem-reader
rules:
- apiGroups: [""]
  resources: ["pods", "pods/log", "services"]
  verbs: ["get", "list", "watch"]
- apiGroups: ["apps"]
  resources: ["deployments", "statefulsets"]
  verbs: ["get", "list", "watch"]

---

# Developer role for namespace-scoped operations
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: omnisystem-developer
  namespace: omnisystem
rules:
- apiGroups: ["apps"]
  resources: ["deployments"]
  verbs: ["get", "list", "patch"]
- apiGroups: [""]
  resources: ["pods", "logs"]
  verbs: ["get", "list"]
```

### Network Policies

**File**: `k8s/network-policies.yaml`

```yaml
# Deny all ingress by default
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: default-deny-ingress
  namespace: omnisystem
spec:
  podSelector: {}
  policyTypes:
  - Ingress

---

# Allow Prometheus to scrape metrics
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-prometheus
  namespace: omnisystem
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: monitoring
    ports:
    - protocol: TCP
      port: 9090

---

# Allow inter-pod communication within namespace
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: allow-same-namespace
  namespace: omnisystem
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  ingress:
  - from:
    - podSelector: {}
```

### Pod Security Policies

**File**: `k8s/security-policies.yaml`

```yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: omnisystem-restricted
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
  - ALL
  volumes:
  - configMap
  - emptyDir
  - projected
  - secret
  - downwardAPI
  - persistentVolumeClaim
  
  hostNetwork: false
  hostIPC: false
  hostPID: false
  
  runAsUser:
    rule: 'MustRunAsNonRoot'
  
  seLinux:
    rule: 'MustRunAs'
    seLinuxOptions:
      level: "s0:c123,c456"
```

---

## 7. Certificate Management (Cert-Manager)

### Cert-Manager Installation

```bash
# Install Cert-Manager
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml

# Create Let's Encrypt Issuer
kubectl apply -f - <<EOF
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@omnisystem.com
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
    - http01:
        ingress:
          class: nginx
EOF
```

### Certificate Lifecycle

```yaml
apiVersion: cert-manager.io/v1
kind: Certificate
metadata:
  name: omnisystem-tls
  namespace: omnisystem
spec:
  secretName: omnisystem-tls
  commonName: api.omnisystem.example.com
  dnsNames:
  - api.omnisystem.example.com
  - dashboard.omnisystem.example.com
  issuerRef:
    name: letsencrypt-prod
    kind: ClusterIssuer
  # Auto-renew 30 days before expiry
  renewBefore: 720h
```

---

## Monitoring & Alerting

### Prometheus AlertRules

```yaml
apiVersion: monitoring.coreos.com/v1
kind: PrometheusRule
metadata:
  name: omnisystem-alerts
  namespace: omnisystem
spec:
  groups:
  - name: omnisystem
    interval: 30s
    rules:
    # Alert: High error rate
    - alert: CrateHighErrorRate
      expr: |
        (sum(rate(http_requests_total{job="omnisystem"}[5m])) by (crate) > 0) and
        (sum(rate(http_requests_total{job="omnisystem",status=~"5.."}[5m])) by (crate) / 
         sum(rate(http_requests_total{job="omnisystem"}[5m])) by (crate)) > 0.05
      for: 5m
      annotations:
        summary: "Crate {{ $labels.crate }} has high error rate"
        description: "Error rate > 5% for 5 minutes"

    # Alert: High latency
    - alert: CrateHighLatency
      expr: |
        histogram_quantile(0.99, 
          sum(rate(http_request_duration_seconds_bucket[5m])) by (crate, le)) > 1
      for: 10m
      annotations:
        summary: "Crate {{ $labels.crate }} has high latency"

    # Alert: Database connection pool exhaustion
    - alert: DatabaseConnectionPoolExhausted
      expr: |
        (pg_stat_activity_count / pg_settings_max_connections) > 0.9
      for: 5m

    # Alert: Service down
    - alert: ServiceDown
      expr: up{job="omnisystem"} == 0
      for: 1m
```

---

## Operations Runbooks

### Scaling Operations

```bash
# Scale deployment up
kubectl scale deployment omnisystem-gateway \
  --replicas=50 -n omnisystem

# Monitor scaling progress
kubectl get deployment omnisystem-gateway \
  -n omnisystem -w

# Monitor HPA decisions
kubectl describe hpa omnisystem-gateway -n omnisystem
```

### Backup & Recovery

```bash
# Initiate backup
velero backup create omnisystem-manual

# Monitor backup
velero backup describe omnisystem-manual --details

# Restore from backup
velero restore create --from-backup omnisystem-daily-20240115

# Verify restoration
kubectl get pods -n omnisystem
```

### Log Collection & Analysis

```bash
# Stream logs from all gateway pods
kubectl logs -f -n omnisystem -l app=omnisystem-gateway

# Get logs from specific pod
kubectl logs -f <pod-name> -n omnisystem

# Export logs for analysis
kubectl logs -n omnisystem -l app=omnisystem \
  --timestamps=true > omnisystem-logs.txt
```

---

## Deployment Timeline

### Phase 3 Execution (8-12 hours)

1. **Infrastructure Provisioning** (2-3 hours)
   - Run Terraform
   - Provision Kubernetes cluster
   - Set up VPC and networking

2. **Database Setup** (1-2 hours)
   - Create PostgreSQL instance
   - Create Redis instance
   - Run migrations

3. **Secret Management** (1 hour)
   - Deploy Vault
   - Configure service authentication
   - Store secrets

4. **Helm Deployment** (1-2 hours)
   - Deploy Helm charts
   - Configure ingress
   - Set up TLS certificates

5. **Monitoring Stack** (1-2 hours)
   - Deploy Prometheus
   - Configure Grafana
   - Set up alerting

6. **Backup Infrastructure** (1 hour)
   - Deploy Velero
   - Configure backup schedules
   - Test recovery

7. **Security Hardening** (1-2 hours)
   - Apply RBAC policies
   - Configure network policies
   - Enable audit logging

---

## Success Criteria

✅ Kubernetes cluster provisioned with Terraform  
✅ All 1,039+ crates deployed with Helm  
✅ ArgoCD GitOps pipeline operational  
✅ Vault secrets management active  
✅ Velero backup schedules running  
✅ RBAC policies enforced  
✅ Network policies restrict traffic  
✅ TLS certificates auto-renewed  
✅ Prometheus collecting metrics  
✅ Grafana dashboards operational  
✅ Alerting rules triggered correctly  

---

**Omnisystem Phase 3: Operations Platform Ready for Production Deployment** ✅
