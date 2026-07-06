# Hepta Systems Tool Execution Live Cutover Final Gate - 2026-06-21

This note records the local-only Tool Execution Live Cutover Final Gate. This
is a without invocation recovery slice. It consumes the live cutover receipt
rollback packet and collapses the full dry-run chain into the final blocker
matrix before any live execution path can exist.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The final
gate is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, execution adapter preflight, execution dispatch
shadow, execution cutover preflight, operator approval packet, operator approval
receipt projection, operator approval decision preflight, canary cutover plan,
canary readback receipt projection, canary result acceptance preflight, live
cutover preflight, live cutover operator packet, live cutover operator receipt
projection, live cutover operator decision preflight, and live cutover receipt
rollback packet. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `live_cutover_final_gate_ready_count=2`
- `live_cutover_final_gate_blocked_count=0`
- `explicit_live_cutover_approval_required_count=2`
- `explicit_live_cutover_approval_missing_count=2`
- `final_operator_readback_required_count=2`
- `live_cutover_blocked_count=2`
- `approval_request_blocked_count=2`
- `operator_acceptance_blocked_count=2`
- `execution_switch_blocked_count=2`
- `rollback_execution_blocked_count=2`
- `result_receipt_write_blocked_count=2`
- `all_receipt_rollback_packets_bound_to_final_gate=true`
- `all_live_cutover_final_gate_entries_keep_no_invocation_guard=true`
- `tool_execution_live_cutover_final_gate_ready=true`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

Both planned plugin tool candidates are final-gate ready but blocked:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The final gate is ready, but explicit live cutover approval is missing. It does
not send an approval request, write a cutover decision receipt, write readback
evidence, record cutover acceptance, start live cutover, execute rollback, write
rollback receipts, write result receipts, dispatch an adapter, invoke a tool,
write the tool ledger, or send an ApprovalBroker request.

## Gate Rules

- `live_cutover_final_gate_ready_blocked` requires source live cutover receipt
  rollback packet route `live_cutover_receipt_rollback_packet_ready`.
- The registry guard must stay `require_approval_ledger`.
- Final gate policy must be present.
- Final cutover ticket must be present.
- Final operator readback must be required.
- Explicit live cutover approval is required and currently missing.
- Approval request sending remains blocked.
- Operator acceptance remains absent.
- Live cutover remains blocked.
- Execution switches remain blocked.
- Rollback execution remains blocked.
- Result receipt writes remain blocked.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, ApprovalBroker requests, result receipt writes, Public GA, and
  live mutation remain disabled.

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
- No rollback execution.
- No rollback receipt write.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_live_cutover_final_gate.rs`
- Report:
  `scripts/hepta-systems-tool-execution-live-cutover-final-gate-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-live-cutover-final-gate-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-live-cutover-receipt-rollback-packet-gate.sh`

## Next Move

This dry-run chain now ends at `manual_operator_live_cutover_approval_required`.
Any future live cutover must be a separate explicitly approved operator action
that creates acceptance, receipt, rollback, dispatch, invocation, ledger, and
readback writes deliberately, with Public GA still blocked by its own gate.
