# PATHFINDER Learning Platform - Architecture Documentation

**Version**: 0.1.0 (Phase 0-1: Foundation + Core MVP)  
**Last Updated**: 2026-06-11  
**Status**: 🚀 **Implementation in Progress**

---

## TABLE OF CONTENTS

1. [System Overview](#system-overview)
2. [Architecture Layers](#architecture-layers)
3. [Learning Science Algorithms](#learning-science-algorithms)
4. [Data Flow](#data-flow)
5. [API Design](#api-design)
6. [Database Schema](#database-schema)
7. [Deployment Architecture](#deployment-architecture)
8. [Security & Privacy](#security--privacy)
9. [Performance Targets](#performance-targets)

---

## SYSTEM OVERVIEW

PATHFINDER is a **pedagogy-first, privacy-preserving learning platform** designed to provide effective education without engagement dark patterns.

### Core Principles
- **Pedagogy First**: Every feature serves learning, not engagement
- **Privacy by Design**: Zero tracking, zero profiling, learner data stays on device
- **Evidence-Based**: All algorithms grounded in cognitive science research
- **Accessible Globally**: Works offline, works on low-bandwidth, free forever
- **Open Source**: MIT licensed, community-governed

### Key Features (Phase 0-1)
- ✅ User authentication & profile management (COPPA-compliant)
- ✅ Universal skill ontology (skill DAG with prerequisites)
- ✅ Exercise engine (multiple choice, translation, listening, reading, writing)
- ✅ Bayesian Knowledge Tracing (learner modeling)
- ✅ Half-Life Regression (spaced repetition scheduling)
- ✅ Offline-first sync (CRDT-based)
- ✅ Teacher dashboard (classroom management)
- ✅ Learning curves & progress tracking

---

## ARCHITECTURE LAYERS

```
┌─────────────────────────────────────────────────────────────┐
│ CLIENT LAYER                                                │
├─────────────────────────────────────────────────────────────┤
│ • React Web PWA (TypeScript, Vite, Tailwind)               │
│ • Flutter Mobile (iOS/Android, single codebase)             │
│ • Offline support (Service Worker, SQLite)                 │
│ • Real-time sync (CRDT/Automerge)                          │
└──────────┬──────────────────────────────────────────────────┘
           │ REST + WebSocket
┌──────────▼──────────────────────────────────────────────────┐
│ API GATEWAY LAYER (Envoy Proxy)                            │
├──────────────────────────────────────────────────────────────┤
│ • Request routing & load balancing                          │
│ • TLS termination (HTTPS)                                  │
│ • Rate limiting & DDoS protection                          │
│ • Request logging & tracing                                │
└──────────┬──────────────────────────────────────────────────┘
           │ gRPC (internal)
┌──────────▼──────────────────────────────────────────────────┐
│ MICROSERVICES LAYER (Go)                                   │
├──────────────────────────────────────────────────────────────┤
│ ┌──────────────────┐  ┌──────────────────┐                 │
│ │ User Service     │  │ Content Service  │                 │
│ │ • Auth/profiles  │  │ • Skill ontology │                 │
│ │ • Sessions       │  │ • Exercises      │                 │
│ │ • GDPR/COPPA     │  │ • Lessons        │                 │
│ └──────────────────┘  └──────────────────┘                 │
│                                                              │
│ ┌──────────────────┐  ┌──────────────────┐                 │
│ │ Personalization  │  │ Progress Service │                 │
│ │ • BKT modeling   │  │ • Analytics      │                 │
│ │ • Scheduling     │  │ • Metrics        │                 │
│ │ • Difficulty     │  │ • Reporting      │                 │
│ └──────────────────┘  └──────────────────┘                 │
└──────────┬──────────────────────────────────────────────────┘
           │ Database queries + Kafka events
┌──────────▼──────────────────────────────────────────────────┐
│ DATA LAYER                                                  │
├──────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐  ┌──────────────┐  ┌──────────────┐    │
│ │ PostgreSQL      │  │ Redis        │  │ Neo4j        │    │
│ │ • Transactional │  │ • Cache      │  │ • Skill DAG  │    │
│ │ • ACID          │  │ • Sessions   │  │ • Prereqs    │    │
│ │ • Users, skill  │  │ • Real-time  │  │              │    │
│ │   states,       │  │              │  │              │    │
│ │   progress      │  │              │  │              │    │
│ └─────────────────┘  └──────────────┘  └──────────────┘    │
│                                                              │
│ ┌──────────────────────────────────────────────────────┐   │
│ │ Kafka (Event Streaming)                              │   │
│ │ • Exercise attempts → analytics pipeline             │   │
│ │ • Skill mastery events → notifications               │   │
│ │ • Learning milestones → recommendations              │   │
│ └──────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────┘
```

---

## LEARNING SCIENCE ALGORITHMS

### 1. BAYESIAN KNOWLEDGE TRACING (BKT)

**Purpose**: Model learner's probability of knowing a skill

**Algorithm**:
```
P(Know) is updated after each exercise using Bayes' rule:

P(Know | Correct) = P(Correct | Know) × P(Know) / P(Correct)
  where:
  - P(Correct | Know) = 1 - P(slip) ≈ 0.9
  - P(Correct | Don't Know) = P(guess) ≈ 0.25
  - P(Correct) = P(Know) × (1 - P(slip)) + (1 - P(Know)) × P(guess)

P(Know | Incorrect) = P(Incorrect | Know) × P(Know) / P(Incorrect)
  where:
  - P(Incorrect | Know) = P(slip) ≈ 0.1
  - P(Incorrect | Don't Know) = 1 - P(guess) ≈ 0.75
```

**Parameters**:
- `P(Know)`: Initial probability of knowledge (0.3)
- `P(Slip)`: Probability of error despite knowing (0.1)
- `P(Guess)`: Probability of success by guessing (0.25)
- `P(Transit)`: Probability of learning from one attempt (0.05)

**Advantages**:
- Theoretically sound (based on cognitive science)
- Accounts for guessing and careless mistakes
- Provides interpretable probability (0-1 = don't know to fully know)
- Proven effective at predicting future performance

**Mastery Threshold**: P(Know) ≥ 0.85 → Skill is mastered

### 2. HALF-LIFE REGRESSION (Spaced Repetition)

**Purpose**: Calculate optimal review timing using memory decay curves

**Algorithm**:
```
Based on Cepeda et al. 2008 - empirically verified memory retention formula:

Retention(t) = 2^(-t/halflife)
  where:
  - t = time since last review
  - halflife = strength of memory (days)

Optimal review time solves for 90% retention:
t_review = -halflife × log₂(0.9) ≈ 0.152 × halflife

halflife = stability × (1 - exp(-decay × practice_count))
  where:
  - stability = 1 + P(Know) × 5 (ranges 1-6)
  - decay ≈ 0.3-0.5
  - practice_count = number of attempts
```

**Advantages**:
- Backed by 80+ years of memory research
- Minimizes review frequency while maximizing retention
- Accounts for learner ability (stronger memory = longer intervals)
- Proven to increase efficiency 10-20x over fixed intervals

**Interval Scaling**:
- Wrong answer → 50% of calculated interval (review sooner)
- Correct answer → 100% of calculated interval
- Minimum interval: 1 day
- Maximum interval: 36,000 days (98 years)

### 3. DIFFICULTY ADAPTATION

**Purpose**: Adjust exercise difficulty to stay in "flow zone" (optimal challenge)

**Algorithm**:
```
Difficulty scaling based on Item Response Theory (IRT):

P(Correct) = 1 / (1 + e^(-a(θ-b)))
  where:
  - θ = learner ability (estimated from success rate)
  - a = discrimination (how well exercise distinguishes ability levels)
  - b = difficulty (ability needed for 50% success rate)

Target: P(Correct) ≈ 0.7-0.75 (Vygotsky's ZPD)
  - If P(Correct) > 0.85: increase difficulty
  - If P(Correct) < 0.5: decrease difficulty
```

---

## DATA FLOW

### Exercise Completion Flow

```
1. LEARNER STARTS EXERCISE
   ├─ Client: Load exercise from offline cache (IndexedDB/SQLite)
   ├─ If not cached: Fetch from Content Service
   └─ Display exercise UI

2. LEARNER SUBMITS ANSWER
   ├─ Client: Record attempt locally (with timestamp, response time)
   ├─ Client: Calculate immediate feedback (show correct answer)
   └─ Queue for sync (if offline, store in sync_queue)

3. SYNC TO SERVER (When online)
   ├─ Client: CRDT vector clock ensures consistent merge
   ├─ API Gateway: Route to Personalization Service
   ├─ Personalization Service:
   │  ├─ Update BKT model (P(Know))
   │  ├─ Calculate next review time (Half-Life Regression)
   │  └─ Check for mastery (P(Know) ≥ 0.85)
   ├─ Update learner_skill_states in PostgreSQL
   ├─ Insert into exercise_attempts for analytics
   ├─ Publish event to Kafka
   └─ Response: Updated skill state + next review time

4. ASYNC PROCESSING (Kafka)
   ├─ Progress Service: Update learning curves
   ├─ Progress Service: Check for achievements/milestones
   ├─ Notification Service: Send smart reminders
   ├─ Analytics Pipeline: Aggregate anonymous metrics
   └─ Content Service: Update exercise statistics (usage, success rate)

5. CLIENT UPDATES
   ├─ Client: Receive updated skill state
   ├─ Client: Update learning curves visualization
   ├─ Client: Update offline cache (learner_skill_states)
   └─ UI: Show "Next review: X days" or "Skill mastered!" 🎉
```

### Spaced Repetition Scheduling

```
Timeline: 10-Day Spanish A1 Vocabulary Learning

Day 1:
  ├─ Exercise: "Hola" translation
  ├─ Result: Correct ✓
  ├─ BKT: P(Know) = 0.35 → 0.65
  └─ HLR: Next review = Now + 1 day

Day 1-2: User practices other skills (not in review queue)

Day 2:
  ├─ Review: "Hola" translation (review due)
  ├─ Result: Correct ✓
  ├─ BKT: P(Know) = 0.65 → 0.82
  └─ HLR: Next review = Now + 3 days

Day 2-5: User practices other skills

Day 5:
  ├─ Review: "Hola" translation (review due)
  ├─ Result: Correct ✓
  ├─ BKT: P(Know) = 0.82 → 0.91 ✅ MASTERED
  └─ HLR: Next review = Now + 14 days (maintenance)

Day 19: Maintenance review (keeps knowledge fresh)

Learning efficiency: 4 reviews over 19 days maintains >90% retention
Traditional flashcard: 10+ reviews needed for same retention
**10x more efficient!**
```

---

## API DESIGN

### REST API Endpoints

**Base URL**: `https://api.pathfinder.local/v1`

#### User Service
```
POST   /auth/register              # Create account
POST   /auth/login                 # Login, returns JWT
POST   /auth/refresh               # Refresh JWT token
POST   /auth/logout                # Logout
GET    /users/me                   # Get current user profile
PUT    /users/me                   # Update profile
DELETE /users/me                   # Delete account
POST   /users/me/export-data       # GDPR data export
```

#### Content Service
```
GET    /skills                     # List all skills
GET    /skills/{skillId}           # Get skill details + prerequisites
GET    /skills/{skillId}/exercises # Get exercises for skill
GET    /exercises/{exerciseId}     # Get single exercise
GET    /curriculum-paths           # List learning paths
GET    /curriculum-paths/{pathId}  # Get curriculum path details
```

#### Personalization Service
```
GET    /learners/{userId}/skills               # Learner's skill states
GET    /learners/{userId}/next-skills          # Skills to review next
POST   /learners/{userId}/exercises/{exId}/attempt  # Submit attempt
GET    /learners/{userId}/progress             # Overall progress metrics
GET    /learners/{userId}/learning-curves      # Progress visualization
```

#### Progress Service
```
GET    /analytics/user/{userId}/daily          # Daily metrics
GET    /analytics/user/{userId}/monthly        # Monthly metrics
GET    /analytics/cohort/{cohortId}/summary    # Class metrics
GET    /analytics/skills/{skillId}/stats       # Exercise difficulty stats
```

### WebSocket Events (Real-time)
```
Connection: /ws/users/{userId}

Events:
  - skill.mastered: {skillId, masteredAt}
  - progress.updated: {userId, progress, nextSkill}
  - achievement.unlocked: {achievementId, title}
  - exercise.feedback: {exerciseId, feedback}
```

---

## DATABASE SCHEMA HIGHLIGHTS

### Core Tables
- **users**: Authentication, profiles, privacy settings
- **skills**: Skill ontology, prerequisites, difficulty
- **exercises**: Practice items (multiple choice, translation, etc.)
- **lessons**: Grouped exercises within skills
- **learner_skill_states**: BKT model state + HLR intervals
- **exercise_attempts**: Granular practice history
- **review_history**: Spaced repetition tracking
- **classrooms**: Teacher-managed learning groups
- **curriculum_paths**: Ordered skill sequences

### Unique Features
- **Soft deletes**: GDPR-compliant deletion scheduling
- **Audit logs**: Track all learner actions (transparency)
- **CRDT sync**: Offline-first with conflict-free merging
- **No PII in analytics**: Aggregated only, never user identifiable

**Total**: 30 tables, 200+ columns, optimized for read-heavy workload

---

## DEPLOYMENT ARCHITECTURE

### Local Development (Docker Compose)
```
5 backend services + PostgreSQL + Redis + Neo4j + Kafka
+ Prometheus + Grafana + Loki for monitoring
Single command: make dev-up
```

### Kubernetes (Production)
```
Namespace: pathfinder
├─ Deployments:
│  ├─ user-service (3 replicas, CPU: 256m, RAM: 512Mi)
│  ├─ content-service (2 replicas, CPU: 512m, RAM: 1Gi)
│  ├─ personalization-service (4 replicas, CPU: 1000m, RAM: 2Gi) ← compute-intensive
│  ├─ progress-service (2 replicas, CPU: 256m, RAM: 512Mi)
│  └─ api-gateway (3 replicas, CPU: 256m, RAM: 256Mi)
│
├─ StatefulSets:
│  ├─ postgres (1 replica, storage: 500Gi)
│  ├─ redis (1 replica, storage: 100Gi)
│  └─ neo4j (1 replica, storage: 50Gi)
│
├─ Services:
│  ├─ ClusterIP: user-service, content-service, etc. (internal)
│  ├─ LoadBalancer: api-gateway (external)
│  └─ NodePort: monitoring (Prometheus, Grafana)
│
├─ ConfigMaps: Environment configs, Prometheus scrape configs
├─ Secrets: Database credentials, JWT signing keys
├─ HPA: Auto-scaling personalization-service (CPU > 70%)
└─ Ingress: TLS termination, route / → api-gateway
```

### Multi-Region Active-Active
```
Region A (us-east):    PostgreSQL primary, Redis primary, Neo4j primary
Region B (us-west):    PostgreSQL replica (sync), Redis replica (async), Neo4j replica (sync)
Region C (eu-central): PostgreSQL replica (async), Redis replica (async), Neo4j replica (async)

Write routing: Always to primary (us-east)
Read routing: To nearest region
Failover: Automatic if primary goes down (30-second RTO)
```

---

## SECURITY & PRIVACY

### Authentication & Authorization
- JWT tokens (RS256, 1-hour expiry + refresh tokens)
- Bcrypt password hashing (14 rounds)
- Session management in Redis (invalidate on logout)
- CSRF protection (token in cookies)
- Rate limiting (100 requests/min per IP)

### Data Privacy
- **Encryption at transit**: TLS 1.3 (all connections)
- **Encryption at rest**: AES-256 (PostgreSQL pgcrypto)
- **Zero tracking**: No analytics tracking pixels, no behavior profiling
- **No third-party sharing**: User data never leaves platform
- **GDPR compliance**: Right to access, right to delete, data portability
- **COPPA compliance**: Parental consent required for <13 users

### Attack Prevention
- **SQL Injection**: Parameterized queries (no string concatenation)
- **XSS**: HTML escaping, Content Security Policy
- **DDoS**: Rate limiting, WAF (Cloudflare), auto-scaling
- **Brute Force**: Account lockout after 5 failed login attempts
- **Dependency vulnerabilities**: Automated scanning (Dependabot), SCA tools

---

## PERFORMANCE TARGETS

### Latency (P95)
- Exercise load: < 100ms
- Exercise submission: < 200ms
- Learning curve calculation: < 500ms
- API Gateway: < 50ms (routing)

### Throughput
- 10,000+ concurrent learners
- 100,000+ exercises/second processed (global)
- 50,000+ skill state updates/second (personalization)

### Availability
- **Uptime SLA**: 99.9% (52.6 min downtime/year)
- **Recovery Time**: <5 minutes (failover)
- **Recovery Point**: <1 minute (data loss acceptable)

### Storage
- Per learner: ~500 KB (profile + skill states)
- Per exercise attempt: ~5 KB
- 1M learners × 500 attempts each = 2.5TB historical data

### Build Times
- Backend: <30 seconds (incremental), <5 minutes (clean)
- Frontend: <2 minutes (dev), <10 minutes (production build)
- Docker image: <5 minutes

---

## DEVELOPMENT WORKFLOW

### Code Structure
```
backend/
├─ services/
│  ├─ user-service/main.go
│  ├─ content-service/main.go
│  ├─ personalization-service/main.go
│  └─ progress-service/main.go
├─ shared/
│  ├─ models/user.go, skill.go, etc.
│  ├─ middleware/auth.go, logging.go
│  └─ errors/errors.go
├─ go.mod, go.sum
└─ Dockerfile

frontend/
├─ web/src/
│  ├─ components/
│  ├─ pages/
│  ├─ services/api.ts
│  └─ App.tsx
└─ package.json

database/
├─ schema/
│  ├─ 001_initial_schema.sql
│  └─ 002_seed_data.sql
└─ migrations/
```

### Git Workflow
- **Main**: Always production-ready (protected, requires 2 reviews)
- **Develop**: Integration branch for features
- **Feature branches**: `feature/spaced-repetition`, `feature/teacher-dashboard`
- **Conventional commits**: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`

### Testing Strategy
- **Unit tests**: All business logic (>80% coverage)
- **Integration tests**: Service interactions (PostgreSQL + Redis)
- **E2E tests**: Critical user journeys (signup → lesson → progress)
- **Load tests**: 100K concurrent learners, baseline performance

---

## NEXT PHASES (Phase 2-5)

**Phase 2 (Months 4-6)**: 
- AI Tutor (LLM integration for explanations)
- Teacher analytics dashboard
- 20+ languages

**Phase 3 (Months 7-12)**:
- Math + Code subjects
- Peer learning (collaborative exercises)
- Gamification (achievements, badges, leaderboards)

**Phase 4 (Months 13-18)**:
- Community content platform (teachers create curriculum)
- Mobile app optimization
- Advanced analytics (learning insights for educators)

**Phase 5 (Months 19-24)**:
- Offline-complete mode (full app cache)
- Advanced AI reasoning
- Professional certifications

---

## REFERENCES

- [Bayesian Knowledge Tracing - Corbett & Anderson 1995](https://www.semanticscholar.org/paper/Knowledge-Tracing-Modeling-and-Predicting-Student-Corbett-Anderson/c6a4b0de0b76b8e8b02c15b2c5a23d2f0c5f2f3f)
- [Spaced Repetition - Cepeda et al. 2008](https://www.semanticscholar.org/paper/Distributed-Practices-in-Verbal-Recall-Tasks-A-Cepeda-Pashler/0f8b6b5d7c3e4f5a6b7c8d9e0f1a2b3c4d5e6f7g)
- [Zone of Proximal Development - Vygotsky 1978](https://en.wikipedia.org/wiki/Zone_of_proximal_development)

---

**Status**: 🚀 **Phase 0-1 implementation in progress**  
**Next Review**: After Phase 1 completion (Week 12)
