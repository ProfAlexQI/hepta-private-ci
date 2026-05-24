# Hepta Upstream Codex Doctor Environment Diagnostics Absorption

Gate id: `upstream-codex-doctor-environment-diagnostics-absorption`

Source upstream range:
`7d47056ea42636271ac020b86347fbbef49490aa..9f42c89c0112771dc29100a6f3fc904049b2655f`

Source upstream commit:
`9f42c89c0112771dc29100a6f3fc904049b2655f`
`feat(doctor): add environment diagnostics (#24261)`

This packet translates the latest upstream Codex doctor-environment delta into
Hepta-owned doctor dry-run contracts. It does not cherry-pick Codex CLI/TUI code
into the active runtime and does not reconnect the active `hepta-cli` service to
Codex engine dependencies.

Absorbed Hepta contract checks:

- `system-environment-redacted-local-only`
- `git-environment-redacted-local-only`
- `terminal-environment-redacted-local-only`
- `terminal-title-redacted-local-only`
- `startup-warning-count-redacted-local-only`

Current contract:

- Source snapshot intake ready: `true`
- Source narrow diff ledger ready: `true`
- Required doctor environment check count: `5`
- Doctor dry-run check count: `11`
- Doctor dry-run checks passed: `11`
- Raw environment value exposed: `false`
- Credential value read: `false`
- External network read: `false`
- Package manager invoked: `false`
- Plugin installed: `false`
- Listener started: `false`
- Active runtime auto-rebase allowed: `false`
- Active runtime Codex engine dependency allowed: `false`
- Public release claim allowed: `false`

Safety boundary:

- No upstream merge
- No upstream checkout
- No active service restart
- No credential or secret read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release publication
- No release artifact write

The active `codex-rs` compatibility surface remains an intake and regression
oracle. Promotion still requires Hepta-native command contracts, dependency
isolation, preflight, watchdog, and soak evidence.
