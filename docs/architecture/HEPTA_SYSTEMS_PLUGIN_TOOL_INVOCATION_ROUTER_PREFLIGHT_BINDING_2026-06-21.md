# Hepta Systems Plugin Tool Invocation Router Preflight Binding - 2026-06-21

This note records the local-only invocation router preflight binding for plugin
tool candidates. The binding consumes manifest schema cutover preflight
decisions and turns them into router planning decisions without registration,
invocation, ledgers, approvals, or live mutation.

## Current Checkout Reality

The current checkout has a read-only `hepta-system` manifest fixture. Manifest
fixture readback supplies complete metadata without registration, so the router
binding can validate the forward dry-run path.

Current report facts:

- `candidate_count=2`
- `router_bound_candidate_count=2`
- `router_blocked_candidate_count=0`
- `router_blocked_by_manifest_precondition_count=0`
- `router_forward_require_approval_ledger_count=2`
- `registration_cutover_allowed=true`
- `invocation_router_preflight_binding_ready=true`

The router forwards both candidates as `forward_require_approval_ledger_dry_run`
while router lookup, registration, invocation, ledger writes, approvals, and
live mutation remain disabled.

## Router Rules

- `forward_require_approval_ledger_dry_run` is only possible when manifest
  schema and policy preconditions are complete and the source registry guard
  remains `require_approval_ledger`.
- `block_manifest_preconditions` is used when a planned candidate is missing
  manifest schema or policy metadata.
- `block_source_registry` is used when the source registry dry-run is not ready.
- Forwarding remains dry-run only and does not enable router registration
  lookup, ToolRegistry registration, tool invocation, ledger writes, or approval
  requests.

## Guardrails

- Router registration lookup disabled.
- Registration cutover execution disabled.
- Tool registration disabled.
- Tool invocation disabled.
- Ledger writes disabled.
- Approval requests disabled.
- MCP server and app connector startup disabled.
- Manifest rewrites and manifest schema writes disabled.
- Plugin cache, package lock, remote sync, workflow event log, local storage,
  SQLite, Telegram/provider/model/gateway/Native POST, package, release, and
  Public GA actions disabled.

## Files

- Rust contract: `codex-rs/tools/src/plugin_tool_invocation_router_preflight_binding.rs`
- Report: `scripts/hepta-systems-plugin-tool-invocation-router-preflight-binding-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-invocation-router-preflight-binding-gate.sh`

## Next Move

Restore the tool registry invocation source-of-truth path without execution.
The next slice should connect the router dry-run surface to a single invocation
source-of-truth plan while keeping router lookup, registration, invocation,
ledgers, approvals, package/release/live actions, and Public GA blocked until an
explicit cutover.

## 2026-06-25 Fixture Readback Update

The router preflight binding now receives two complete manifest fixture
readback candidates and forwards them only as `forward_require_approval_ledger`
dry-runs. Router lookup, tool registration, tool invocation, ledger writes,
approval requests, and live mutation remain disabled.
