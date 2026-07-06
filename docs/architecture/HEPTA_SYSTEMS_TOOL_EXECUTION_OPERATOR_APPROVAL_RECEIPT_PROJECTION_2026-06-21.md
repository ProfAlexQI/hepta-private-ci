# Hepta Systems Tool Execution Operator Approval Receipt Projection - 2026-06-21

This note records the local-only Tool Execution Operator Approval Receipt
Projection. The projection consumes the operator approval packet and reserves
readback evidence slots for any future operator approval decision without
invocation.
This is a without invocation recovery slice.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The receipt
projection is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, execution adapter preflight, execution dispatch
shadow, execution cutover preflight, and operator approval packet. It does not
create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `operator_approval_receipt_projection_ready_count=2`
- `operator_approval_receipt_projection_blocked_count=0`
- `operator_decision_receipt_required_count=2`
- `operator_decision_readback_evidence_required_count=2`
- `operator_decision_receipt_write_blocked_count=2`
- `all_operator_packets_bound_to_receipt_projection=true`
- `all_operator_receipt_projections_keep_approval_guard=true`
- `tool_execution_operator_approval_receipt_projection_ready=true`
- `tool_execution_operator_decision_write_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates require future operator decision receipt and
readback evidence:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The projection is ready, but it does not write a decision, write a decision
receipt, record operator acceptance, send an approval request, dispatch an
adapter, invoke a tool, write the tool ledger, or write a result receipt.

## Projection Rules

- `operator_approval_receipt_projection_ready` requires source operator approval
  packet route `operator_approval_packet_ready_for_review`.
- The registry guard must stay `require_approval_ledger`.
- The operator decision receipt projection must be present.
- The operator decision readback evidence slot must be present.
- Operator decision record writes must remain disabled.
- Operator decision receipt writes must remain disabled.
- Operator acceptance must remain absent for this read-only slice.
- Approval request sending must remain disabled.
- Live cutover and execution switches must remain disabled.
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
- No operator decision record write.
- No operator decision receipt write.
- No operator acceptance record.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_operator_approval_receipt_projection.rs`
- Report:
  `scripts/hepta-systems-tool-execution-operator-approval-receipt-projection-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-operator-approval-receipt-projection-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-operator-approval-packet-gate.sh`

## Next Move

The tool execution operator approval decision preflight has been restored
downstream. The next slice should restore a canary cutover plan without
invocation, while keeping lookup execution, ToolRegistry registration, adapter
dispatch, tool invocation, ledger writes, ApprovalBroker requests, result
receipts, package/release/live actions, and Public GA blocked until an explicit
cutover.
