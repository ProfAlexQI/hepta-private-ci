# Hepta Systems Tool Execution Live Cutover Receipt Rollback Packet - 2026-06-21

This note records the local-only Tool Execution Live Cutover Receipt Rollback
Packet. This is a without invocation recovery slice. It consumes the live
cutover operator decision preflight and reserves rollback, result receipt,
rollback receipt, operator summary, and kill-switch requirements without
starting live cutover.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The live
cutover receipt rollback packet is fed by the replacement declaration report,
manifest schema cutover preflight, invocation router preflight binding,
invocation source-of-truth plan, registration lookup cutover preflight, router
lookup shadow, ledger approval preflight, receipt projection, execution adapter
preflight, execution dispatch shadow, execution cutover preflight, operator
approval packet, operator approval receipt projection, operator approval
decision preflight, canary cutover plan, canary readback receipt projection,
canary result acceptance preflight, live cutover preflight, live cutover
operator packet, live cutover operator receipt projection, and live cutover
operator decision preflight. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `live_cutover_receipt_rollback_packet_ready_count=2`
- `live_cutover_receipt_rollback_packet_blocked_count=0`
- `rollback_anchor_present_count=2`
- `rollback_readback_required_count=2`
- `result_receipt_required_count=2`
- `rollback_receipt_required_count=2`
- `operator_summary_required_count=2`
- `live_cutover_start_blocked_count=2`
- `rollback_execution_blocked_count=2`
- `result_receipt_write_blocked_count=2`
- `all_live_cutover_operator_decision_preflight_entries_bound_to_receipt_rollback_packet=true`
- `all_live_cutover_receipt_rollback_packets_keep_no_invocation_guard=true`
- `tool_execution_live_cutover_receipt_rollback_packet_ready=true`
- `tool_execution_live_cutover_start_allowed=false`
- `tool_execution_live_cutover_rollback_allowed=false`
- `tool_execution_live_cutover_result_receipt_write_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates have rollback and result receipt packets:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The packet is ready, but it does not send an approval request, write a cutover
decision receipt, write readback evidence, record cutover acceptance, start live
cutover, execute rollback, write rollback receipts, write result receipts,
dispatch an adapter, invoke a tool, write the tool ledger, or send an
ApprovalBroker request.

## Packet Rules

- `live_cutover_receipt_rollback_packet_ready` requires source live cutover
  operator decision route `live_cutover_operator_decision_pending_explicit_approval`.
- The registry guard must stay `require_approval_ledger`.
- Rollback anchor must be present.
- Rollback readback channel must be present.
- Result receipt schema must be present.
- Operator summary template must be present.
- Kill switch must be present.
- Rollback plan, rollback readback, result receipt, rollback receipt, and
  operator summary are required.
- Live cutover start remains blocked.
- Rollback execution remains blocked.
- Result receipt writes remain blocked.
- Live cutover, adapter dispatch, and tool invocation switches must remain
  disabled.
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
  `codex-rs/tools/src/tool_execution_live_cutover_receipt_rollback_packet.rs`
- Report:
  `scripts/hepta-systems-tool-execution-live-cutover-receipt-rollback-packet-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-live-cutover-receipt-rollback-packet-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-live-cutover-operator-decision-preflight-gate.sh`

## Next Move

Restore the tool execution live cutover final gate without invocation. The next
slice should collapse this packet and all upstream blockers into a final
cutover gate while keeping approval request, decision receipt writes, readback
evidence writes, rollback, result receipt writes, adapter dispatch, tool
invocation, ledger writes, ApprovalBroker requests, package/release/live
actions, and Public GA blocked until explicit cutover.
