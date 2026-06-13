# Copilot Agent Mode Adaptation – Quick Navigation

**Full Document**: [AGENT-MODE-ADAPTATION.md](AGENT-MODE-ADAPTATION.md)

## Key Sections

### 1. **Copilot Agent Mode Current Architecture** (§1)
- How Copilot Agent Mode works (cloud ephemeral runners)
- Configuration file structure (`.github/agents/`, `.github/instructions/`)
- Copilot's agent definition format (`.agent.md` with YAML frontmatter)
- MCP tool integration (GitHub MCP, Playwright MCP)
- Current approval & HITL model (role-based, no per-action approval)

### 2. **Agent Definition Format Compatibility** (§2)
- Copilot format (current standard, backwards compatible)
- **Bonsai Agent Definition Format (YAML)** – new standard with Bonsai extensions
- Translation service (`CopilotTranslator`) to convert Copilot → Bonsai
- Version compatibility strategy

**Key Artifact**: Agent definition with model selection, knowledge modules, tool policies, approval rules, and execution isolation.

### 3. **Context Injection for Agents** (§3)
- Workspace awareness (file tree, git history, repo metadata)
- Domain-specific context via Knowledge Database (KDB) modules
- Conversation history management (sliding window, compression)

**Key Insight**: Inject top-k relevant passages from KDB modules to enhance agent reasoning without bloating context.

### 4. **Tool Mapping** (§4)
- GitHub MCP → Bonsai tools (issues, PRs, workflows)
- Playwright MCP → Headless browser automation
- Bonsai-specific tools (read_file, write_file, search_codebase, etc.)
- Tool discovery & capability tokens (immutable, enforce at runtime)

**Key Mechanism**: Capability-based security prevents privilege escalation and unauthorized actions.

### 5. **Multi-Agent Orchestration** (§5)
- Sequential composition (Agent 1 output → Agent 2 input)
- Parallel execution (multiple agents analyze different files)
- Hierarchical nesting (master agent spawns subagents)

**Key Feature**: Agents can compose to solve complex tasks; master agents manage lifecycle and resource pooling.

### 6. **HITL & Approval Flows** (§6)
- Pre-execution approval (modal for HIGH risk operations)
- Continuous monitoring (real-time event dashboard)
- Post-execution review (summary, audit trail, per-action revert)

**Key Principle**: User has fine-grained control; every action is visible and reversible.

### 7. **Bonsai Ecosystem Integration** (§7)
- Knowledge Database (KDB) injection for domain expertise
- Universe Time-Travel (checkpoint before agent runs, revert if needed)
- Compute Fabric (offload expensive operations to remote devices)
- Learning System (record executions for fine-tuning custom agents)

**Key Synergy**: Agents leverage Bonsai's unique features (time travel, KDB, distributed compute).

### 8. **Performance & Scalability** (§8)
- Token budgeting (total allocation, per-phase tracking)
- Latency targets and timeout handling
- Error recovery (transient retry, fatal escalation)

**Key Constraint**: Agents must complete within token budget and wall-clock timeout.

### 9. **Security & Permissions** (§9)
- Capability-based permissions (file ops, process ops, network ops)
- Sandboxing via Sanctum vaults (isolated namespaces, resource limits)
- Code injection prevention (prompt templating, tool result validation)

**Key Defense**: Agents cannot escape sandbox or escalate privileges.

### 10. **Extensibility & Custom Agents** (§10)
- Agent definition format & storage (`.bonsai/agents/`)
- Agent discovery (local filesystem, future marketplace)
- Agent composition & templates (extend base agents, compose workflows)

**Key Capability**: Users can define custom agents and share them.

## Quick-Reference Tables

### Risk Levels & Approval Requirements
- **LOW** (green): `read_file`, `search_codebase` → auto-approve
- **MEDIUM** (orange): `run_tests`, `web_search` → notify but auto-approve
- **HIGH** (red): `write_file`, `delete_file`, `git_push`, `submit_issue` → require approval

### Agent Execution Timeline
| Phase | Duration | Notes |
|-------|----------|-------|
| Startup | ~1-2s | Load agent, KDB modules |
| First token | ~5-10s | Model warm-up |
| Per step | ~1-3s | Each LLM inference |
| Tool calls | 0.1-10s | File I/O, API calls |

### Bonsai Tools (MCP)
| Tool | Purpose | Risk | Approval |
|------|---------|------|----------|
| `read_file` | Read workspace files | LOW | Auto |
| `write_file` | Create/edit files | HIGH | Required |
| `search_codebase` | Ripgrep + semantic | LOW | Auto |
| `run_command` | Execute shell | HIGH | Required |
| `git_commit` | Stage & commit | HIGH | Required |
| `git_push` | Push to remote | HIGH | Required |
| `submit_issue` | File issues | MEDIUM | Required |
| `kdb_retrieve` | Query KDB modules | LOW | Auto |

## MVP Scope (3-Month Delivery)

**In MVP:**
- ✅ Agent definition (YAML format)
- ✅ Copilot translation (`.agent.md` → Bonsai YAML)
- ✅ Basic agent runtime (single agent, sequential execution)
- ✅ Tool calling via MCP
- ✅ HITL approval gates (pre-execution modal)
- ✅ Time-travel checkpointing
- ✅ Dashboard (event timeline, approval UI)
- ✅ KDB module injection

**Post-MVP:**
- Parallel/hierarchical agent composition
- Compute Fabric offloading
- Agent recording & fine-tuning
- Custom approval policies
- Agent marketplace

## Key Design Decisions

1. **Copilot Compatibility First**: Existing Copilot agents translate to Bonsai with minimal friction.
2. **Sovereignty & Control**: All execution is local; user approves significant actions.
3. **Transparency**: Every action is logged, visible, and reversible (Time Travel).
4. **Sandboxing**: Agents run in Sanctum vaults; cannot escape or escalate privileges.
5. **Knowledge Injection**: KDB modules inject domain expertise without retraining.
6. **Ecosystem Leverage**: Agents use Bonsai's unique features (time travel, KDB, compute fabric).

## Integration Checklist

- [ ] Implement `CopilotTranslator` (Phase 1)
- [ ] Implement `Agent` runtime (Phase 2)
- [ ] Implement orchestrators (Phase 3)
- [ ] Implement advanced features (Phase 4)
- [ ] Security hardening (Phase 5)

## Success Metrics

| Metric | Target |
|--------|--------|
| Agent load time | < 2 seconds |
| Tool call latency | < 1 second |
| HITL approval response | < 30 seconds |
| Vault isolation | 100% prevent escapes |
| Token budget accuracy | ± 5% |
| Approval log completeness | 100% audit trail |

---

**Document Created**: 2026-06-01  
**Status**: Design Phase Complete (Ready for Implementation)  
**Word Count**: ~4,000 words  
**Code Examples**: 20+ Rust examples, 15+ YAML configs
