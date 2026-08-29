# Hepta UI Development Plan v5 — 2026-08-29

## Exact binding

- Repository: `ProfHepta/hepta-private-ci`
- Base branch: `codex/ui-v4-executable-final-gap-closure-v3-20260829`
- Base commit: `304143bf362e6cc8369158309b7b9c7c9c3d3868`
- Base tree: `a28c7808e64e2cfa21a8ed3fcbc5525cbbfbac3f`
- Work branch: `codex/ui-v5-plan-and-repo-gap-closure-20260829`

This successor does not rewrite historical v4 receipts. It corrects the claim that all repository-controlled gaps were already closed while exact-head CI still contained deterministic source and workflow failures.

## Execution order

1. Apply the candidate atomically to the exact clean base.
2. Run formatter, JSON/YAML/Ruby gates and `git diff --check`.
3. Commit on the isolated v5 branch and open a Draft PR stacked on PR #63's head branch.
4. Require nonempty assigned-runner execution for baseline contract, DWM, aggregate, exact materialization/runtime, Windows Bazel, canonical v4 and cross-browser workflows.
5. Iterate only on newly observed exact-head failures.
6. Keep all physical-device and human-signed slots blocked until independently produced evidence exists.

## Claim boundary

Repository source, Hosted CI and this applicator cannot synthesize physical Windows/macOS/iOS/Android evidence, non-delegable operator acceptance or browser-human acceptance. Network, mutation, effect, product binding, production, promotion and release remain false.

## Changed paths

- `.github/workflows/hepta-ui-v4-windows-dwm-ack-producer.yml`
- `.github/workflows/hepta-ui-v4-windows-material-profile-aggregate.yml`
- `.github/workflows/windows-msvc-bazel.yml`
- `codex-rs/analytics/src/analytics_client_tests.rs`
- `codex-rs/app-server-protocol/src/protocol/v2/turn.rs`
- `codex-rs/core-plugins/src/manager_tests.rs`
- `codex-rs/exec-server/src/client.rs`
- `codex-rs/memories/mcp/src/server.rs`
- `codex-rs/thread-store/src/local/mod.rs`
- `codex-rs/thread-store/src/local/read_thread.rs`
- `codex-rs/tools/src/mcp_tool_tests.rs`
- `codex-rs/tools/src/responses_api_tests.rs`
- `scripts/hepta-ci-baseline-contract`
- `scripts/hepta-ui-v4-apply-makepad-windows-ack-patch`
- `scripts/hepta-ui-v4-exact-candidate-materialization-gate`
- `scripts/hepta-ui-v4-run-fixed-makepad-windows-ack-patch`
