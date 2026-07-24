# Hepta Upstream Codex R4 Semantic Absorption Receipt

## Boundary

- Source intake: `HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4.json` at local commit `77867f0f04ba`.
- Integration mode: selective semantic transplant only.
- No merge, rebase, cherry-pick, deployment, restart, publication, or live enablement occurred.
- R4 remains an immutable observation with zero imports at observation time; this receipt records later positive integration evidence.

## Absorbed Lanes

- Security: upstream `a59a419afa34` maps to Hepta `e259297b3076`. Shell approval keys now bind the working directory as a file URI, with an opaque collision-separated fallback for unrepresentable or NUL-bearing paths. Focused result: `2 passed`.
- Protocol/app-server: upstream `0d4910331db5` maps to Hepta `2a91f2c7f832`. External session imports preserve earliest/latest valid source timestamps after rollout-compatible name reconciliation. Focused results: external-session parser `18 passed`; app-server import regression `1 passed`.
- Tools/apps/runtime: upstream `205d37a20f74` maps to Hepta `db39bc7d6aff`. Namespaced `DirectModelOnly` capabilities remain model-direct and are excluded from nested code mode, which preserves the upstream sleep-tool invariant without pretending Hepta already owns the upstream clock subsystem. Focused result: `1 passed`.

## Tooling

- Package fixes completed for `codex-external-agent-sessions`, `codex-app-server`, and `codex-core`.
- `just fmt` completed; only existing stable-toolchain `imports_granularity` warnings and pre-existing lint warnings were emitted.
- Machine-readable provenance, exact commit IDs, changed files, and commands are in the adjacent JSON receipt.

## Non-Claims

- This receipt proves three selected semantic absorptions, not full consumption of the 97-commit R4 range.
- It does not promote the remaining R4 candidates to imported.
- It does not establish production readiness, controlled-live readiness, or upstream history compatibility.
