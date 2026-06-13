# Security Audit Checklist

This document lists the high-level security checks and mitigations to perform
before a public release.

1. Input validation
   - Validate all JSON-RPC and HTTP inputs.
   - Reject unexpected/malformed messages.

2. Sandbox Enforcement
   - Ensure `sandbox_executor` runs untrusted code in isolated venvs.
   - Limit network access and file system scope per plugin.

3. Dependency Scanning
   - Run `cargo audit` and `cargo deny` on CI.
   - Run `npm audit` for JS dependencies.

4. Secrets Management
   - Do not log secrets. Use OS keychain via `secrets_store`.
   - Rotate pairing tokens and require confirmation for high-risk actions.

5. Network Exposure
   - Bind admin endpoints to localhost by default.
   - Require explicit opt-in for remote A2A and P2P features.

6. WebSocket Origin and Auth
   - Validate origin headers for browser-initiated WS.
   - Require authentication tokens for MCP/A2A endpoints.

7. P2P Trust Model
   - Treat peers as untrusted. Require explicit user approval before installing models.
   - Verify checksums and signatures for received model artifacts.

8. CRDT/Collaboration Integrity
   - Limit shared documents to authenticated participants.
   - Sanitize and audit tool outputs stored in shared docs.

9. Fuzzing and Sanitizers
   - Fuzz input parsers and the prompt sanitizer.
   - Use address/undefined behavior sanitizers in CI for native builds.

10. LoRA & Plugin Safety
   - Load LoRA adapters in a sandbox; disallow arbitrary code execution.
   - Require manifest capability declarations for plugins.

Automated checks are included in `.github/workflows/security.yml`.
