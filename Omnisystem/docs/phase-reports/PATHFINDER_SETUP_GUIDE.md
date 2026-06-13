# PATHFINDER Learning Platform - Setup Guide

**Quick Start**: Get PATHFINDER running locally in 10 minutes

---

## PREREQUISITES

### System Requirements
- **OS**: Linux, macOS, or Windows (with WSL2)
- **Memory**: 16GB RAM minimum (8GB will work but be slow)
- **Disk**: 20GB free space (for Docker images and databases)

### Required Software
- [Docker Desktop](https://www.docker.com/products/docker-desktop) (v20.10+)
- [Docker Compose](https://docs.docker.com/compose/install/) (v1.29+)
- [Go](https://golang.org/dl/) (v1.21+)
- [Node.js](https://nodejs.org/) (v18+)
- [Make](https://www.gnu.org/software/make/) (included on macOS/Linux; on Windows use WSL2)

### Optional (for cloud deployment)
- [kubectl](https://kubernetes.io/docs/tasks/tools/) (for Kubernetes)
- [minikube](https://minikube.sigs.k8s.io/docs/start/) (for local K8s testing)
- [Terraform](https://www.terraform.io/downloads.html) (for infrastructure-as-code)

---

## INSTALLATION

### 1. Clone Repository

```bash
git clone https://github.com/pathfinder-learning/pathfinder.git
cd pathfinder
```

### 2. Initial Setup (Automated)

```bash
# Install dependencies, configure environment
make setup

# This will:
# ✓ Verify Docker, Go, Node.js installed
# ✓ Create .env file from .env.example
# ✓ Download Go modules
# ✓ Install npm dependencies
```

### 3. Start Local Development Environment

```bash
make dev-up

# Wait for all services to start (~2-3 minutes)
# You should see:
# ✓ postgres_1 is healthy
# ✓ redis_1 is healthy
# ✓ neo4j_1 is healthy
# ✓ kafka_1 is ready
```

### 4. Verify Installation

```bash
# Check all services are running
docker-compose ps

# Expected output:
# NAME                      STATE
# pathfinder_postgres       healthy
# pathfinder_redis          healthy
# pathfinder_neo4j          healthy
# pathfinder_kafka          Up
# pathfinder_user-service   Up
# pathfinder_content-service Up
# ...
```

### 5. Access the Services

Open in browser:
- **Web App**: http://localhost:3000 (React frontend)
- **Grafana**: http://localhost:3001 (admin/admin)
- **Prometheus**: http://localhost:9090
- **Neo4j**: http://localhost:7474 (neo4j/pathfinder_dev_password)

### 6. Verify Database Connection

```bash
# Connect to PostgreSQL
make dev-shell-postgres

# You should see: pathfinder=> prompt
# Test: SELECT count(*) FROM users;
# Expected: 0 (new database)
# Type: \q to exit
```

---

## DATABASE SETUP

### Initial Schema

```bash
# Run migrations to create schema
make db-migrate

# Seed with initial data (Spanish A1 curriculum)
make db-seed
```

### Verify Data

```bash
# Connect to database
make dev-shell-postgres

# Check tables created
\dt

# Expected: 30 tables
# including: users, skills, exercises, lessons, etc.

# Check initial curriculum
SELECT name, level, language FROM skills LIMIT 5;

# Expected output:
#  name               | level | language
# ────────────────────┼───────┼──────────
# Greetings          | A1    | es
# Numbers 1-20       | A1    | es
# Colors             | A1    | es
# Personal Pronouns  | A1    | es
# Ser vs Estar       | A1    | es
```

---

## RUNNING TESTS

### Backend Tests

```bash
# Unit tests (30 seconds)
make test-backend

# Integration tests (60 seconds, needs database)
make test-integration

# All tests with coverage
make test-coverage

# Expected: All tests passing (baseline ~100 tests)
```

### Frontend Tests

```bash
cd frontend/web

# Jest unit tests
npm test

# E2E tests (needs running app)
npm run test:e2e
```

### Load Testing

```bash
# Simulate 100K concurrent learners
make load-test

# This will show:
# - Requests/sec
# - Average latency
# - 95th percentile latency
# - Error rate
```

---

## DEVELOPMENT WORKFLOW

### Making Code Changes

#### Backend (Go)

```bash
# 1. Make changes in backend/services/user-service/main.go

# 2. Format and lint
make format-go
make lint-go

# 3. Test
make test-backend

# 4. Rebuild service (automatic if using docker-compose volumes)
# Restart service if needed:
docker-compose restart user-service

# 5. Verify API responds
curl http://localhost:8001/health
```

#### Frontend (React)

```bash
# 1. Make changes in frontend/web/src/components/LessonView.tsx

# 2. Format and lint
make format-js
make lint-js

# 3. Check live at http://localhost:3000 (hot reload)

# 4. Run tests
cd frontend/web && npm test
```

### Database Changes

```bash
# 1. Create new migration file
mkdir -p database/schema
cat > database/schema/003_add_new_table.sql <<'EOF'
CREATE TABLE example_table (
  id UUID PRIMARY KEY,
  name VARCHAR(255)
);
EOF

# 2. Run migrations
make db-migrate

# 3. Verify in database
make dev-shell-postgres
\dt example_table
```

### Viewing Logs

```bash
# All services
make dev-logs

# Specific service
docker-compose logs user-service

# Follow logs (tail)
docker-compose logs -f personalization-service

# Last 100 lines
docker-compose logs --tail=100 content-service
```

---

## API TESTING

### Using cURL

```bash
# Register new user
curl -X POST http://localhost:8000/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "learner@example.com",
    "password": "secure_password",
    "first_name": "Maria"
  }'

# Expected response:
# {"userId": "550e8400-e29b-41d4-a716-446655440000", "token": "eyJhbGc..."}

# Login
curl -X POST http://localhost:8000/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "learner@example.com",
    "password": "secure_password"
  }'

# Get learner's skill states
curl -X GET http://localhost:8000/v1/learners/{userId}/skills \
  -H "Authorization: Bearer {token}"
```

### Using Postman

1. Import collection: `docs/postman-collection.json`
2. Set environment variable: `base_url = http://localhost:8000`
3. Run requests from collection
4. Responses will show skill states, exercise data, etc.

### Using GraphQL Playground

```bash
# If GraphQL gateway is running
open http://localhost:8000/graphql
```

---

## TROUBLESHOOTING

### Issue: "docker-compose: command not found"

**Solution**: Use `docker compose` (without hyphen) if you have Docker Desktop v20.10+:
```bash
docker compose -f docker-compose.yml up -d
```

Or install Docker Compose standalone:
```bash
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose
```

### Issue: "Database connection refused"

**Solution**: Ensure PostgreSQL is healthy:
```bash
docker-compose ps postgres

# If not healthy, check logs
docker-compose logs postgres

# Restart it
docker-compose restart postgres

# Wait 30 seconds, then try again
make db-migrate
```

### Issue: "Port 5432 already in use"

**Solution**: Another PostgreSQL is running. Choose one:

**Option A**: Kill existing process
```bash
lsof -i :5432  # Find process
kill -9 <PID>
```

**Option B**: Use different port
```bash
# In docker-compose.yml, change postgres port to 5433:
ports:
  - "5433:5432"

# Update .env:
DATABASE_URL="postgres://pathfinder:...@localhost:5433/pathfinder"
```

### Issue: "Out of disk space"

**Solution**: Docker images are large (~50GB). Clean up:
```bash
make clean-docker

# If still needed:
docker system prune -a --volumes
```

### Issue: "Tests failing after database changes"

**Solution**: Reset database:
```bash
make db-reset

# This drops all data and rebuilds schema
# (only use in development!)
```

### Issue: "Service is slow / high CPU usage"

**Solution**: Check resource allocation:
```bash
# View Docker stats
docker stats

# If personalization-service is slow, it's computing BKT models
# This is expected with many learners; scale horizontally in production

# Restart service
docker-compose restart personalization-service
```

---

## COMMON TASKS

### Creating a Test User

```bash
# 1. Register via API
curl -X POST http://localhost:8000/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "Test123!@#",
    "first_name": "Test"
  }'

# OR via database
make dev-shell-postgres

INSERT INTO users (id, email, password_hash, first_name, email_verified)
VALUES (
  'test-user-id'::uuid,
  'test@example.com',
  crypt('Test123!@#', gen_salt('bf')),
  'Test',
  TRUE
);
```

### Adding New Skill & Exercises

```bash
# Via API (if endpoints exist)
POST /v1/skills
{
  "code": "spanish_a1_food",
  "name": "Food Vocabulary",
  "level": "A1",
  "category": "vocabulary"
}

# OR via database
make dev-shell-postgres

INSERT INTO skills (code, name, description, level, language, category, is_published)
VALUES (
  'spanish_a1_food',
  'Food Vocabulary',
  'Learn Spanish food-related words',
  'A1',
  'es',
  'vocabulary',
  TRUE
);
```

### Running a Single Service

```bash
# Build and run user-service with hot reload
cd backend/services/user-service
go run ./cmd/main.go

# Expected: "Server listening on :8001"

# In another terminal, test it
curl http://localhost:8001/health
```

### Accessing Database Directly

```bash
# PostgreSQL
make dev-shell-postgres

# Redis
make dev-shell-redis

# Neo4j
make dev-shell-neo4j
```

---

## PERFORMANCE TUNING (Local Dev)

### Slow Docker?

If Docker is slow on your machine:

```bash
# 1. Allocate more CPU/RAM to Docker Desktop
# Settings → Resources → Increase CPUs, Memory

# 2. Use native volume mount (macOS)
# Docker Desktop → Preferences → Resources → File Sharing
# Add project directory

# 3. Use WSL2 backend (Windows)
# Not using VirtualBox, but native Windows features
```

### Slow Database?

```bash
# Create indexes on commonly queried columns
make dev-shell-postgres

CREATE INDEX idx_learner_skill_states_next_review
ON learner_skill_states(next_review_at);

# Analyze query performance
EXPLAIN ANALYZE
SELECT * FROM learner_skill_states
WHERE user_id = 'user-id' AND next_review_at <= NOW();
```

---

## STOPPING & RESTARTING

### Stop Everything

```bash
make dev-down

# Clean up (removes volumes, careful!)
docker-compose down -v
```

### Restart Specific Service

```bash
docker-compose restart personalization-service

# Or rebuild if code changed
docker-compose up --build user-service
```

### Full Reset

```bash
# Caution: Removes all data!
make clean-docker
make setup
make dev-up
make db-migrate
make db-seed
```

---

## NEXT STEPS

1. **Read the Architecture Guide**: `PATHFINDER_ARCHITECTURE.md`
2. **Explore the Codebase**: `backend/services/user-service/`
3. **Make Your First Change**: Add a new endpoint or fix a bug
4. **Run Tests**: `make test-backend`
5. **Submit a PR**: Follow contribution guidelines in `CONTRIBUTING.md`

---

## GETTING HELP

- **Issues**: [GitHub Issues](https://github.com/pathfinder-learning/pathfinder/issues)
- **Discussions**: [GitHub Discussions](https://github.com/pathfinder-learning/pathfinder/discussions)
- **Email**: hello@pathfinder.learning
- **Chat**: [Community Discord](https://discord.gg/pathfinder-learning)

---

**Last Updated**: 2026-06-11  
**Status**: ✅ **Tested and verified**
