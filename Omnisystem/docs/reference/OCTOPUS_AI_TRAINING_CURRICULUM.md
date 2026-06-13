# 🐙 Octopus AI — Detailed Training Curriculum

This document specifies the **exact data, exercises, and validation** for each of the 12 capability domains.

---

## Domain 1: Server Monitoring & Operations

### 1.1 Learning Objectives

By the end of training, Octopus AI must be able to:
- Parse Prometheus metrics and interpret trends.
- Analyze journalctl logs to identify errors and anomalies.
- Calculate system health score from multiple metrics.
- Predict resource exhaustion (disk, memory, CPU).
- Explain the root cause of performance degradation.

### 1.2 Training Data Sources

| Source | Count | Format |
|--------|-------|--------|
| Prometheus docs (official) | 500 | HTML → Markdown |
| Grafana dashboard examples | 1,000 | YAML/JSON |
| Real /proc/meminfo dumps | 2,000 | key=value pairs |
| journalctl output samples | 5,000 | Raw logs |
| Stack Overflow (monitoring) | 10,000 | Q&A pairs |
| Incident reports | 500 | (Symptom → diagnosis) |
| Academic papers on monitoring | 50 | Summaries + excerpts |

**Total**: ~20,000 examples.

### 1.3 Exercises & Validation

| Exercise | Count | Validation |
|----------|-------|------------|
| "This CPU is at 80%. Is that normal?" | 100 | Answer depends on workload context |
| "Parse this /proc/meminfo. How much is available?" | 50 | Exact number match |
| "This container exited with code 137. Why?" | 50 | OOM killer diagnosis |
| "Anomaly detection: Find unusual patterns in these logs" | 50 | Precision/recall scoring |
| "Capacity planning: Will we run out of disk in 30 days?" | 50 | Linear extrapolation accuracy |

### 1.4 LoRA Adapter: `adapter-monitoring`

- **Rank**: 16
- **Training data**: 15,000 examples
- **Validation set**: 2,000 examples
- **Test set**: 3,000 examples
- **Pass criteria**: >92% accuracy on test set

---

## Domain 2: Container Orchestration

### 2.1 Learning Objectives

- Execute Docker commands correctly (build, run, exec, logs).
- Debug container failures (exit codes, resource limits, networking).
- Design docker-compose.yml for multi-container applications.
- Optimize image layers and minimize image size.
- Implement health checks and restart policies.

### 2.2 Training Data

| Source | Count |
|--------|-------|
| Docker official docs | 2,000 |
| Dockerfile best practices | 1,000 |
| docker-compose examples | 5,000 |
| Real Dockerfiles from GitHub | 20,000 |
| Stack Overflow (docker) | 50,000 |
| Container escape CVEs | 200 |
| Docker networking guide | 500 |

**Total**: ~80,000 examples.

### 2.3 Exercises

| Exercise | Count |
|----------|-------|
| "Fix this Dockerfile" (syntax errors, inefficiencies) | 100 |
| "Why won't my container connect to another?" | 50 |
| "Design a docker-compose.yml for WordPress + MySQL" | 20 |
| "Write a health check for this app" | 30 |
| "Optimize this 1GB image down to 200MB" | 20 |
| "Container keeps crashing. Diagnose it." | 100 |

### 2.4 LoRA Adapter: `adapter-containers`

- **Rank**: 16
- **Training data**: 60,000 examples
- **Validation/test**: 20,000 examples
- **Pass criteria**: >93% accuracy

---

## Domain 3: Configuration Management & IaC

### 3.1 Learning Objectives

- Write valid NixOS configurations.
- Understand Nix language (functions, sets, derivations).
- Use flakes for reproducible environments.
- Write systemd units.
- Manage secrets securely.

### 3.2 Training Data

| Source | Count |
|--------|-------|
| NixOS manual | 3,000 |
| nixpkgs source (comments) | 5,000 |
| Nix language tutorial | 500 |
| Real NixOS configs (github) | 10,000 |
| systemd.unit man pages | 1,000 |
| Flakes examples | 2,000 |
| Secret management guides | 500 |

**Total**: ~22,000 examples.

### 3.3 Exercises

| Exercise | Count |
|----------|-------|
| "Write a NixOS module for PostgreSQL" | 50 |
| "Debug this Nix derivation (it won't compile)" | 50 |
| "Convert this systemd unit to a NixOS option" | 30 |
| "Create a flake.nix for a Rust project" | 50 |
| "Encrypt a secret and use it in NixOS" | 20 |

### 3.4 LoRA Adapter: `adapter-nixos`

- **Rank**: 16
- **Training data**: 20,000 examples
- **Pass criteria**: >90% on validation

---

## Domain 4: Networking & Connectivity

### 4.1 Learning Objectives

- Understand TCP/IP, DNS, routing.
- Troubleshoot network connectivity (ping, traceroute, tcpdump).
- Configure firewalls (iptables, nftables).
- Debug container networking issues.
- Understand Echo fabric and peer discovery.

### 4.2 Training Data

| Source | Count |
|--------|-------|
| TCP/IP textbook excerpts | 5,000 |
| DNS RFC docs | 1,000 |
| iptables man pages + guides | 3,000 |
| tcpdump examples | 2,000 |
| Stack Overflow (networking) | 20,000 |
| Echo fabric docs | 1,000 |
| Network simulation scenarios | 1,000 |

**Total**: ~33,000 examples.

### 4.3 Exercises

| Exercise | Count |
|----------|-------|
| "Why can't I reach 8.8.8.8? Trace the issue." | 50 |
| "Write iptables rules to allow SSH but block HTTP" | 30 |
| "Parse this tcpdump. What's happening?" | 50 |
| "Why is DNS slow?" | 30 |
| "Configure WireGuard for a VPN" | 20 |

### 4.4 LoRA Adapter: `adapter-networking`

- **Rank**: 16
- **Training data**: 30,000 examples
- **Pass criteria**: >91% accuracy

---

## Domain 5: Security & CVE Analysis

### 5.1 Learning Objectives

- Understand CVE scoring, severity, exploitability.
- Identify vulnerable software and patch it.
- Scan containers for vulnerabilities.
- Implement security hardening (SELinux, AppArmor, seccomp).
- Respond to security incidents.

### 5.2 Training Data

| Source | Count |
|--------|-------|
| NVD database (JSON) | 200,000 |
| OWASP Top 10 | 1,000 |
| CIS Benchmarks | 2,000 |
| CVE security advisories | 10,000 |
| Trivy docs + examples | 1,000 |
| SELinux/AppArmor policies | 5,000 |
| Penetration testing guides | 1,000 |

**Total**: ~220,000 examples.

### 5.3 Exercises

| Exercise | Count |
|--------|-------|
| "Is CVE-2024-12345 critical for me?" | 100 |
| "Scan this image for vulnerabilities. Suggest patches." | 100 |
| "Write an AppArmor profile for Nginx" | 30 |
| "Detect a privilege escalation attempt in these logs" | 50 |

### 5.4 LoRA Adapter: `adapter-security`

- **Rank**: 16
- **Training data**: 150,000 examples
- **Pass criteria**: >94% accuracy (safety is critical)

---

## Domain 6: Backup, Disaster Recovery & Data Integrity

### 6.1 Learning Objectives

- Design backup strategies (RTO/RPO).
- Create and restore snapshots.
- Implement incremental backups.
- Use content-addressed storage.
- Plan disaster recovery scenarios.

### 6.2 Training Data

| Source | Count |
|--------|-------|
| Snapshot docs (LVM, ZFS, Btrfs) | 2,000 |
| rsync man pages + guides | 1,000 |
| Database backup procedures | 3,000 |
| CAS principles | 1,000 |
| Real disaster recovery plans | 500 |
| Backup validation guides | 500 |

**Total**: ~8,000 examples.

### 6.3 Exercises

| Exercise | Count |
|----------|-------|
| "Create a backup strategy for 10 TB of data" | 50 |
| "Restore this database from a 3-day-old snapshot" | 50 |
| "Design a multi-region failover plan" | 20 |
| "Verify this backup is valid" | 30 |

### 6.4 LoRA Adapter: `adapter-backup`

- **Rank**: 16
- **Training data**: 6,000 examples
- **Pass criteria**: >90% accuracy

---

## Domain 7: Performance Tuning & Optimization

### 7.1 Learning Objectives

- Identify performance bottlenecks (CPU, memory, disk, network).
- Use profiling tools (perf, flame graphs).
- Tune kernel parameters for specific workloads.
- Optimize application code and queries.
- Measure and improve cache efficiency.

### 7.2 Training Data

| Source | Count |
|--------|-------|
| perf manual + examples | 1,000 |
| Systems Performance textbook | 5,000 |
| Flame graph guides | 500 |
| sysctl tuning guides | 2,000 |
| Query optimization (SQL) | 3,000 |
| CPU pinning + NUMA | 1,000 |

**Total**: ~12,500 examples.

### 7.3 Exercises

| Exercise | Count |
|----------|-------|
| "This app is slow. Profile it and find the bottleneck." | 100 |
| "Tune kernel parameters for a database server" | 30 |
| "Optimize this SQL query (currently 10s, target <100ms)" | 100 |
| "Cache hit rate is 5%. How to improve to 95%?" | 50 |

### 7.4 LoRA Adapter: `adapter-performance`

- **Rank**: 16
- **Training data**: 10,000 examples
- **Pass criteria**: >88% accuracy

---

## Domain 8: Computer Science Fundamentals

### 8.1 Learning Objectives

- Understand algorithms (sorting, searching, graph algorithms).
- Know data structures and when to use each.
- Analyze complexity (Big-O).
- Understand concurrency (threads, locks, atomics).
- Design scalable distributed systems.
- Understand operating system concepts.

### 8.2 Training Data

| Source | Count |
|--------|-------|
| Introduction to Algorithms (CLRS) | 30,000 |
| LeetCode solutions | 30,000 |
| Operating Systems textbooks | 20,000 |
| Distributed Systems papers | 10,000 |
| Programming language specs | 20,000 |
| Database theory | 10,000 |
| Academic papers (arXiv) | 30,000 |

**Total**: ~150,000 examples.

### 8.3 Exercises

| Exercise | Count |
|----------|-------|
| "Implement binary search" | 100 |
| "What's the time complexity? How to optimize?" | 200 |
| "Design a thread-safe queue" | 50 |
| "Explain Raft consensus algorithm" | 50 |
| "Fix this deadlock scenario" | 50 |

### 8.4 LoRA Adapters: `adapter-cs-theory` + `adapter-algorithms`

- **Rank**: 16 each
- **Training data**: 100,000 examples per
- **Pass criteria**: >85% accuracy (tests are hard)

---

## Domain 9: Programming Languages & Code

### 9.1 Learning Objectives

- Write correct, idiomatic code in Rust, Python, Go, C, JavaScript, Bash, SQL, Nix.
- Understand language-specific idioms and best practices.
- Debug syntax errors and type errors.
- Review code for bugs and inefficiencies.
- Refactor code safely.

### 9.2 Training Data

| Source | Count |
|--------|-------|
| Official language docs (Rust, Python, Go, C, JS) | 50,000 |
| Open-source code (GitHub, top repos) | 250,000 |
| Stack Overflow (code snippets) | 100,000 |
| Code review guidelines | 5,000 |
| Language idioms guides | 10,000 |

**Total**: ~415,000 examples.

### 9.3 Exercises

| Exercise | Count |
|----------|-------|
| "Fix this Rust borrow checker error" | 200 |
| "Write a Python async function" | 100 |
| "Implement a Go goroutine pool" | 50 |
| "Review this code for bugs" | 200 |
| "Refactor this into idiomatic Rust" | 100 |

### 9.4 LoRA Adapters: `adapter-programming` + per-language

- **Rank**: 16
- **Training data**: 250,000+ examples
- **Pass criteria**: >90% accuracy

---

## Domain 10: AI, Machine Learning & Model Operations

### 10.1 Learning Objectives

- Understand transformer architecture, MoE, attention.
- Design training pipelines, choose loss functions.
- Implement fine-tuning (full, LoRA, QLoRA, instruction).
- Optimize inference (quantization, KV cache, batching).
- Evaluate and benchmark models fairly.

### 10.2 Training Data

| Source | Count |
|--------|-------|
| Transformer paper + variants | 2,000 |
| Hugging Face docs + examples | 10,000 |
| ML textbooks | 20,000 |
| Prompting techniques | 3,000 |
| Fine-tuning guides (LoRA, DPO, RLHF) | 5,000 |
| Quantization methods | 2,000 |
| Research papers (arXiv) | 20,000 |

**Total**: ~62,000 examples.

### 10.3 Exercises

| Exercise | Count |
|----------|-------|
| "Design a model for language understanding" | 50 |
| "Why is training loss plateauing? How to fix?" | 50 |
| "Implement LoRA fine-tuning for a model" | 30 |
| "Quantize this model. Measure accuracy loss." | 50 |
| "Optimize inference latency 10× without losing accuracy" | 50 |

### 10.4 LoRA Adapter: `adapter-ml`

- **Rank**: 16
- **Training data**: 50,000 examples
- **Pass criteria**: >85% accuracy

---

## Domain 11: Bonsai Ecosystem Specific

### 11.1 Learning Objectives

- Write Blueprints for multi-component orchestration.
- Understand Weave state management and event handling.
- Use MCP tools effectively.
- Query Survival KB for incident solutions.
- Optimize with BPCF-Pre.

### 11.2 Training Data

| Source | Count |
|--------|-------|
| Blueprint docs + examples | 2,000 |
| Weave component examples | 1,000 |
| Universe event documentation | 500 |
| MCP tool reference | 1,000 |
| Bonsai source code comments | 2,000 |
| Internal runbooks | 500 |

**Total**: ~7,000 examples.

### 11.3 Exercises

| Exercise | Count |
|----------|-------|
| "Write a Blueprint for a web stack (frontend, API, DB)" | 50 |
| "Implement a Weave component that monitors disk space" | 20 |
| "Query Survival KB for solutions to this error pattern" | 50 |

### 11.4 LoRA Adapter: `adapter-bonsai`

- **Rank**: 16
- **Training data**: 5,000 examples
- **Pass criteria**: >90% accuracy

---

## Domain 12: Systems Thinking & Incident Response

### 12.1 Learning Objectives

- Diagnose complex system failures (multi-cause, cascading).
- Perform root-cause analysis (5 Whys, fault trees).
- Design resilient systems (redundancy, failover, circuit breakers).
- Respond to incidents (triage, escalation, recovery).
- Write effective postmortems and capture learnings.

### 12.2 Training Data

| Source | Count |
|--------|-------|
| Real postmortems (Incident.io, etc.) | 2,000 |
| System design interview questions | 5,000 |
| Chaos engineering scenarios | 3,000 |
| Reliability engineering guides | 2,000 |
| Root cause analysis frameworks | 500 |
| Synthetic incident scenarios | 10,000 |

**Total**: ~22,500 examples.

### 12.3 Exercises

| Exercise | Count |
|----------|-------|
| "Cascading failure: Service A crashes → B → C. Fix it." | 100 |
| "Design a system with 99.99% uptime for 1M users" | 50 |
| "Write a postmortem for this outage" | 50 |
| "Incident: Database corrupted. Root cause? Recovery?" | 50 |

### 12.4 LoRA Adapters: `adapter-incident` + `adapter-distributed`

- **Rank**: 16 each
- **Training data**: 40,000 examples combined
- **Pass criteria**: >88% accuracy, expert scoring >4.0/5.0

---

## Summary: Total Training Data

| Domain | Examples |
|--------|----------|
| Server Monitoring | 20,000 |
| Containers | 80,000 |
| Configuration Mgmt | 22,000 |
| Networking | 33,000 |
| Security | 220,000 |
| Backup/DR | 8,000 |
| Performance | 12,500 |
| CS Fundamentals | 150,000 |
| Programming | 415,000 |
| ML/AI | 62,000 |
| Bonsai Ecosystem | 7,000 |
| Systems & Incident | 22,500 |
| **Total** | **1,052,000** |

After deduplication and quality filtering (keeping only >0.7 quality score): **~650,000 high-quality examples**.

---

## Validation & Grading

### Rubric for Each Exercise

| Criterion | Excellent (5) | Good (4) | Fair (3) | Poor (1-2) |
|-----------|--------------|---------|---------|-----------|
| **Correctness** | Exact answer | Minor ambiguity | Mostly correct | Wrong |
| **Safety** | Safe + secure | Safe but inefficient | Potentially unsafe | Dangerous |
| **Clarity** | Well-explained | Clear enough | Unclear | Incomprehensible |
| **Completeness** | Full solution | Minor gaps | Major gaps | Incomplete |
| **Efficiency** | Optimal | Good | Acceptable | Inefficient |

### Pass Criteria by Domain

- **High-stakes domains** (Security, Incident Response): ≥ 4.2/5.0 average
- **Medium-stakes** (Containers, Networking): ≥ 4.0/5.0
- **Theory** (CS, ML): ≥ 3.8/5.0
- **Overall model**: ≥ 4.0/5.0 across all domains

---

**Curriculum Version**: 1.0
**Last Updated**: 2026-06-02
**Status**: Ready for Implementation
