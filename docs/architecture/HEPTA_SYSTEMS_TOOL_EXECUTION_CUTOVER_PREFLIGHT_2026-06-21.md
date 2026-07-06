# Hepta Systems Tool Execution Cutover Preflight - 2026-06-21

This note records the local-only Tool Execution Cutover Preflight. The preflight
consumes the execution dispatch shadow and gathers the plugin tool path into one
cutover blocker matrix without invocation.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The cutover
preflight is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, execution adapter preflight, and execution
dispatch shadow. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `cutover_preflight_ready_count=2`
- `cutover_preflight_blocked_count=0`
- `explicit_cutover_approval_required_count=2`
- `live_cutover_blocked_count=2`
- `all_dispatch_shadow_entries_bound_to_cutover_preflight=true`
- `all_cutover_entries_keep_approval_guard=true`
- `tool_execution_cutover_preflight_ready=true`
- `tool_execution_live_cutover_allowed=false`
- `explicit_cutover_approval_present=false`
- `live_cutover_switch_enabled=false`

Both planned plugin tool candidates are represented in the cutover matrix:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The cutover preflight is ready, but live cutover is deliberately not allowed.
The blockers are explicit cutover approval, the live cutover switch, dispatch,
invocation, ledger writes, approval requests, and result receipt writes.

## Preflight Rules

- `cutover_preflight_blocked_until_explicit_approval` requires a source dispatch
  shadow route of `disabled_execution_dispatch_shadow`.
- The registry guard must stay `require_approval_ledger`.
- The cutover matrix binding must be present.
- Explicit cutover approval must be absent for this read-only slice.
- Live cutover and execution switches must remain disabled.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, approval requests, and result receipt writes remain disabled.

## Guardrails

- No historical patch replay.
- No plugin fixture fabrication.
- No plugin install, cache mutation, package-lock mutation, or remote sync.
- No router registration lookup execution.
- No registry lookup execution.
- No ToolRegistry registration.
- No execution adapter dispatch.
- No tool invocation.
- No ledger write.
- No ApprovalBroker request.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_execution_cutover_preflight.rs`
- Report: `scripts/hepta-systems-tool-execution-cutover-preflight-report.sh`
- Gate: `scripts/hepta-systems-tool-execution-cutover-preflight-gate.sh`
- Source gate: `scripts/hepta-systems-tool-execution-dispatch-shadow-gate.sh`

## Next Move

The tool execution operator approval packet has been restored downstream. The
next slice should project an operator approval receipt/readback requirement
without invocation, while keeping lookup execution, ToolRegistry registration,
adapter dispatch, tool invocation, ledger writes, ApprovalBroker requests,
result receipts, package/release/live actions, and Public GA blocked until an
explicit cutover.
