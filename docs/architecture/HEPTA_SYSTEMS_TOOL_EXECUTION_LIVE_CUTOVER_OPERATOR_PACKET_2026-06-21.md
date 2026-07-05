# Hepta Systems Tool Execution Live Cutover Operator Packet - 2026-06-21

This note records the local-only Tool Execution Live Cutover Operator Packet.
This is a without invocation recovery slice. It consumes the live cutover
preflight and produces the operator-facing review packet plus remaining blocker
readback without sending an approval request.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The live
cutover operator packet is fed by the replacement declaration report, manifest
schema cutover preflight, invocation router preflight binding, invocation
source-of-truth plan, registration lookup cutover preflight, router lookup
shadow, ledger approval preflight, receipt projection, execution adapter
preflight, execution dispatch shadow, execution cutover preflight, operator
approval packet, operator approval receipt projection, operator approval
decision preflight, canary cutover plan, canary readback receipt projection,
canary result acceptance preflight, and live cutover preflight. It does not
create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `live_cutover_operator_packet_ready_count=2`
- `live_cutover_operator_packet_blocked_count=0`
- `operator_review_required_count=2`
- `remaining_blocker_readback_required_count=2`
- `approval_request_blocked_count=2`
- `all_live_cutover_preflight_entries_bound_to_operator_packet=true`
- `all_live_cutover_operator_packets_keep_no_invocation_guard=true`
- `tool_execution_live_cutover_operator_packet_ready=true`
- `tool_execution_live_cutover_approval_request_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates have operator review packets:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The packet is review-ready, but it does not render/send a packet, send an
approval request, write an approval record, record operator acceptance, enable
live cutover, dispatch an adapter, invoke a tool, write the tool ledger, send an
ApprovalBroker request, or write final result receipts.

## Packet Rules

- `live_cutover_operator_packet_ready_for_review` requires source live cutover
  route `live_cutover_preflight_ready_pending_approval`.
- The registry guard must stay `require_approval_ledger`.
- Operator packet template must be present.
- Operator session binding must be present.
- Remaining blocker readback must be required.
- The operator-facing packet must include remaining blocker readback.
- Approval request sending remains blocked.
- Operator cutover approval record writes remain disabled.
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
- No operator packet send/render.
- No operator cutover approval record write.
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
  `codex-rs/tools/src/tool_execution_live_cutover_operator_packet.rs`
- Report:
  `scripts/hepta-systems-tool-execution-live-cutover-operator-packet-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-live-cutover-operator-packet-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-live-cutover-preflight-gate.sh`

## Next Move

Restore the tool execution live cutover operator receipt projection without
invocation. The next slice should reserve approval receipt/readback evidence
for the future operator decision while keeping approval request, adapter
dispatch, tool invocation, ledger writes, ApprovalBroker requests, result
receipt writes, package/release/live actions, and Public GA blocked until
explicit cutover.
