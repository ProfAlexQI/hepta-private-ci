# Hepta Systems Tool Registry Invocation Source Of Truth - 2026-06-21

This note records the local-only ToolRegistry invocation source-of-truth plan
for plugin tool candidates. The plan consumes invocation router preflight
binding output and turns the forwarded candidates into one read-only source
surface without execution.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The chain is
therefore fed by the replacement declaration report, the manifest schema
cutover preflight, and the invocation router preflight binding. This slice does
not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `source_router_forward_count=2`
- `invocation_source_ready_count=2`
- `invocation_source_blocked_count=0`
- `approval_ledger_dry_run_source_count=2`
- `all_forwarded_candidates_bound_to_invocation_source=true`
- `all_invocation_sources_keep_approval_ledger_guard=true`
- `invocation_source_of_truth_plan_ready=true`

The two candidates remain the planned local plugin tool surfaces:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

Both are bound to an approval-ledger dry-run source route. This is a planning
source only; it does not enable registry lookup, ToolRegistry registration,
tool invocation, ledger writes, approval requests, or live mutation.

## Source Rules

- A candidate can become `approval_ledger_dry_run_source_only` only when the
  router preflight decision is `forward_require_approval_ledger_dry_run`.
- The source route must keep `require_approval_ledger` as the registry guard.
- Router registration lookup remains disabled.
- Registry source-of-truth enablement remains disabled.
- Tool registration and invocation remain disabled.
- Ledger writes and approval requests remain disabled.

## Guardrails

- No historical patch replay.
- No plugin fixture fabrication.
- No plugin install, cache mutation, package-lock mutation, or remote sync.
- No ToolRegistry registration.
- No tool invocation.
- No ledger write.
- No approval request.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_registry_invocation_source_of_truth.rs`
- Report: `scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh`
- Gate: `scripts/hepta-systems-tool-registry-invocation-source-of-truth-gate.sh`
- Source router gate: `scripts/hepta-systems-plugin-tool-invocation-router-preflight-binding-gate.sh`

## Next Move

Restore the ToolRegistry registration lookup cutover preflight without
execution. The next slice should model the disabled lookup switch and lookup
result path, still keeping registration, invocation, ledger writes, approvals,
package/release/live actions, and Public GA blocked until an explicit cutover.
