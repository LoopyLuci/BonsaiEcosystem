**Runtime Security & Hardening Spec**

- **Goal:** Harden the `/runtime/*` admin endpoints by limiting what scripts may be executed and by enforcing per-runtime limits (timeouts and per-user concurrency). Also provide a short migration recipe to move `tools/bb.exe` into Git LFS and remove it from repository history.

- **Config model (`bonsai-bot`):**
  - `allowed_script_paths: Vec<String>` — filesystem path prefixes that are permitted as script locations. If empty, the server falls back to the repository `runtimes/` locations and executable-parent `runtimes/`.
  - `runtime_limits: { max_runtime_secs?: u64, max_instances_per_user?: u32 }` — optional caps applied to new runtime starts.

- **Runtime start policy (enforced in `POST /runtime/start`):**
  1. Canonicalize the requested `script` path and require it to be under one of the configured `allowed_script_paths` (or fallback allowed dirs when the list is empty).
  2. If `user` is supplied in the request, count active runtime instances for that user and reject the start if it would exceed `max_instances_per_user`.
  3. If `timeout_secs` is supplied, require it to be <= `max_runtime_secs` when that limit is set; otherwise use the configured default `max_runtime_secs` as the runtime timeout.
  4. If a timeout is in effect, the admin server spawns an internal monitor that will kill the process after the timeout elapses and audit the event.

- **Audit & Observability:** runtime start/stop/timeout events are written to the `admin-audit.log` (JSON-lines) in the config dir.

- **Testing:** integration tests spawn real processes for `python` and `bb` when those binaries are present on the test host. Tests are guarded to skip when the runtime binary is not available.

- **Git LFS migration recipe (Windows / PowerShell):**
  1. Ensure `git-lfs` is installed and available on `PATH`.
  2. Add `.gitattributes` containing `tools/bb.exe filter=lfs diff=lfs merge=lfs -text`.
  3. `git add .gitattributes && git commit -m "track tools/bb.exe with git-lfs"`
  4. `git lfs migrate import --include="tools/bb.exe" --exclude-ref=refs/remotes/origin/HEAD` (review and run on a clean working tree)
  5. Force-push the rewritten branches: `git push --force --all` and `git push --force --tags`.

Note: Rewriting history affects collaborators — communicate and coordinate before force-pushing.
