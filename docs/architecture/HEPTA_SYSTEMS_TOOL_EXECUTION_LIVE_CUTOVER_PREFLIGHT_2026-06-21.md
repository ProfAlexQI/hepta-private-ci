# Hepta Systems Tool Execution Live Cutover Preflight - 2026-06-21

This note records the local-only Tool Execution Live Cutover Preflight. This is
a without invocation recovery slice. It consumes the canary result acceptance
preflight and assembles the final live cutover blocker matrix without starting
live cutover.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The live
cutover preflight is fed by the replacement declaration report, manifest schema
cutover preflight, invocation router preflight binding, invocation
source-of-truth plan, registration lookup cutover preflight, router lookup
shadow, ledger approval preflight, receipt projection, execution adapter
preflight, execution dispatch shadow, execution cutover preflight, operator
approval packet, operator approval receipt projection, operator approval
decision preflight, canary cutover plan, canary readback receipt projection, and
canary result acceptance preflight. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `live_cutover_preflight_ready_count=2`
- `live_cutover_preflight_blocked_count=0`
- `explicit_live_cutover_approval_required_count=2`
- `explicit_live_cutover_approval_missing_count=2`
- `rollback_anchor_present_count=2`
- `kill_switch_present_count=2`
- `observability_readback_required_count=2`
- `all_acceptance_preflight_entries_bound_to_live_cutover_preflight=true`
- `all_live_cutover_entries_keep_no_invocation_guard=true`
- `tool_execution_live_cutover_preflight_ready=true`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates are ready for live cutover review but remain
blocked:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The preflight is ready, but it does not record explicit live cutover approval,
enable the live switch, enable adapter dispatch, enable invocation, start live
cutover, write ledger entries, send ApprovalBroker requests, or write final
result receipts.

## Preflight Rules

- `live_cutover_preflight_ready_pending_approval` requires source canary result
  acceptance route `canary_result_acceptance_pending_evidence`.
- The registry guard must stay `require_approval_ledger`.
- Operator identity binding must be present.
- Explicit live cutover approval is required but absent for this read-only
  slice.
- Rollback anchor must be present.
- Kill-switch must be present.
- Observability readback must be required.
- Live cutover switch must remain disabled.
- Adapter dispatch switch must remain disabled.
- Tool invocation execution switch must remain disabled.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, ApprovalBroker requests, live cutover acceptance records, and
  final result receipt writes remain disabled.

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
- No operator decision record write.
- No operator decision receipt write.
- No operator acceptance record.
- No live cutover approval record.
- No live cutover start.
- No live cutover acceptance record.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_live_cutover_preflight.rs`
- Report:
  `scripts/hepta-systems-tool-execution-live-cutover-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-live-cutover-preflight-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-gate.sh`

## Next Move

Restore the tool execution live cutover operator packet without invocation. The
next slice should produce the operator-facing approval packet and remaining
blocker readback while keeping adapter dispatch, tool invocation, ledger writes,
ApprovalBroker requests, result receipt writes, package/release/live actions,
and Public GA blocked until explicit cutover.
