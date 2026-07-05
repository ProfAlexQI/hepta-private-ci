# Hepta Systems Tool Execution Operator Approval Decision Preflight - 2026-06-21

This note records the local-only Tool Execution Operator Approval Decision
Preflight. This is a without invocation recovery slice. It consumes the operator
approval receipt projection and records that tool execution cutover remains
pending explicit approval.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The decision
preflight is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, execution adapter preflight, execution dispatch
shadow, execution cutover preflight, operator approval packet, and operator
approval receipt projection. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `operator_approval_decision_preflight_ready_count=2`
- `operator_approval_decision_preflight_blocked_count=0`
- `operator_decision_pending_count=2`
- `operator_decision_write_blocked_count=2`
- `operator_acceptance_blocked_count=2`
- `all_receipt_projections_bound_to_decision_preflight=true`
- `all_decision_preflight_entries_keep_approval_guard=true`
- `tool_execution_operator_approval_decision_preflight_ready=true`
- `tool_execution_operator_decision_acceptance_allowed=false`
- `tool_execution_canary_cutover_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates remain pending explicit approval:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The decision preflight is ready, but it does not write a decision, write a
decision receipt, record operator acceptance, start a canary cutover, dispatch an
adapter, invoke a tool, write the tool ledger, or write a result receipt.

## Preflight Rules

- `operator_approval_decision_pending_explicit_approval` requires source
  receipt projection route `operator_approval_receipt_projection_ready`.
- The registry guard must stay `require_approval_ledger`.
- The operator decision policy must be present.
- The operator identity binding must be present.
- Operator decision record writes must remain disabled.
- Operator decision receipt writes must remain disabled.
- Operator acceptance must remain absent for this read-only slice.
- Approval request sending must remain disabled.
- Canary and live cutover switches must remain disabled.
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
- No canary cutover start.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_operator_approval_decision_preflight.rs`
- Report:
  `scripts/hepta-systems-tool-execution-operator-approval-decision-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-operator-approval-decision-preflight-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-operator-approval-receipt-projection-gate.sh`

## Next Move

Restore the tool execution canary cutover plan without invocation. The next
slice should describe canary scope and rollback/readback requirements while
keeping lookup execution, ToolRegistry registration, adapter dispatch, tool
invocation, ledger writes, ApprovalBroker requests, result receipts,
package/release/live actions, and Public GA blocked until an explicit cutover.
