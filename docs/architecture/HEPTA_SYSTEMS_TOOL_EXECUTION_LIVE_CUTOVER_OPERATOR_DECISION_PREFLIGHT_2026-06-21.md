# Hepta Systems Tool Execution Live Cutover Operator Decision Preflight - 2026-06-21

This note records the local-only Tool Execution Live Cutover Operator Decision
Preflight. This is a without invocation recovery slice. It consumes the live
cutover operator receipt projection and records that live cutover remains
pending explicit approval.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The live
cutover operator decision preflight is fed by the replacement declaration
report, manifest schema cutover preflight, invocation router preflight binding,
invocation source-of-truth plan, registration lookup cutover preflight, router
lookup shadow, ledger approval preflight, receipt projection, execution adapter
preflight, execution dispatch shadow, execution cutover preflight, operator
approval packet, operator approval receipt projection, operator approval
decision preflight, canary cutover plan, canary readback receipt projection,
canary result acceptance preflight, live cutover preflight, live cutover
operator packet, and live cutover operator receipt projection. It does not
create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `live_cutover_operator_decision_preflight_ready_count=2`
- `live_cutover_operator_decision_preflight_blocked_count=0`
- `operator_cutover_decision_pending_count=2`
- `operator_cutover_decision_write_blocked_count=2`
- `operator_cutover_acceptance_blocked_count=2`
- `all_live_cutover_operator_receipt_projections_bound_to_decision_preflight=true`
- `all_live_cutover_operator_decision_preflight_entries_keep_no_invocation_guard=true`
- `tool_execution_live_cutover_operator_decision_preflight_ready=true`
- `tool_execution_live_cutover_operator_decision_acceptance_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates remain pending explicit approval:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The decision preflight is ready, but it does not send an approval request,
write a cutover decision receipt, write readback evidence, record cutover
acceptance, enable live cutover, dispatch an adapter, invoke a tool, write the
tool ledger, send an ApprovalBroker request, or write final result receipts.

## Preflight Rules

- `live_cutover_operator_decision_pending_explicit_approval` requires source
  live cutover operator receipt projection route
  `live_cutover_operator_receipt_projection_ready`.
- The registry guard must stay `require_approval_ledger`.
- Operator cutover decision policy must be present.
- Operator identity binding must be present.
- Approval request sending remains blocked.
- Operator cutover decision receipt writes remain disabled.
- Operator cutover readback evidence writes remain disabled.
- Operator cutover acceptance remains absent.
- Live cutover switch must remain disabled.
- Adapter dispatch switch must remain disabled.
- Tool invocation execution switch must remain disabled.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, ApprovalBroker requests, and result receipt writes remain
  disabled.

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
- No approval request send.
- No operator cutover decision receipt write.
- No operator cutover readback evidence write.
- No operator cutover acceptance record.
- No live cutover start.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_live_cutover_operator_decision_preflight.rs`
- Report:
  `scripts/hepta-systems-tool-execution-live-cutover-operator-decision-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-live-cutover-operator-decision-preflight-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-live-cutover-operator-receipt-projection-gate.sh`

## Next Move

Restore the tool execution live cutover receipt rollback packet without
invocation. The next slice should assemble rollback and receipt requirements
for the future cutover path while keeping approval request, decision receipt
writes, readback evidence writes, adapter dispatch, tool invocation, ledger
writes, ApprovalBroker requests, result receipts, package/release/live actions,
and Public GA blocked until explicit cutover.
