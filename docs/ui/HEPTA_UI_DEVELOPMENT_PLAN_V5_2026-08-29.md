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

## Implementation candidate binding

- Commit: `4171b464e49708a44418241fe857986dce0edb53`
- Tree: `f52543dc64b1c831e261bb9b8ac07b6e363d231d`
- Branch: `codex/ui-v5-plan-and-repo-gap-closure-20260829`
- State: `IMPLEMENTATION_COMMITTED_HOSTED_QUALIFICATION_REQUIRED`

The one-shot bootstrap transport was removed from this implementation tree. Bootstrap run `33264125711`, job `99131005577`, executed on an assigned runner with nonempty successful steps, applied all 20 intended paths, passed Python, shell, Ruby, YAML, JSON and `git diff --check` validation, pushed the non-workflow staging commit, and published artifact `9718170325` with digest `sha256:ecc88d56bcaac8b4baa725aae8121a558c314b03ed45542fc4ef4049a4686564`. The three retained workflow files were then published as exact SHA-256-verified Git blobs in the implementation commit.

This binding proves repository materialization and local source validation only. It does not claim exact-head Hosted qualification, physical-device evidence, browser-human acceptance, operator acceptance, product binding, production, promotion or release authority.

## Pull-request qualification request

Draft PR #78 was opened on `2026-08-29T17:09:35Z`, targeting the exact PR #63 head branch. The request was created from commit `762449926187605c61fd0a97908b1e01ad9d6177` / tree `12c2a7e30f5dcd7cb13d58ad0e55561c04b404d9`.

A PR being open is not qualification. Each repository gap remains open until an exact current PR head receives a completed successful job with a nonzero runner identity and nonempty successful steps. Queued, skipped, cancelled, empty-step and runnerless results remain fail-closed and cannot set `sourceControllableGapsClosed=true`.

Any follow-up commit must:

1. remain a non-force fast-forward descendant of the current v5 branch;
2. preserve the base branch and PR #63 history;
3. change only paths required by newly observed exact-head failures;
4. update the repository gap ledger before claiming closure;
5. preserve all external evidence and authority fields as false.
