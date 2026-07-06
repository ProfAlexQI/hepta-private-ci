# Hepta Systems Tool Execution Operator Approval Packet - 2026-06-21

This note records the local-only Tool Execution Operator Approval Packet. The
packet consumes the execution cutover preflight blocker matrix and turns the
future cutover into an operator-review surface without invocation.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The approval
packet is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, execution adapter preflight, execution dispatch
shadow, and execution cutover preflight. It does not create or rewrite a
manifest.

Current report facts:

- `candidate_count=2`
- `operator_approval_packet_ready_count=2`
- `operator_approval_packet_blocked_count=0`
- `operator_review_required_count=2`
- `approval_request_blocked_count=2`
- `all_cutover_preflight_entries_bound_to_operator_packet=true`
- `all_operator_packets_keep_approval_guard=true`
- `tool_execution_operator_approval_packet_ready=true`
- `tool_execution_operator_approval_request_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates are represented in the operator approval
packet:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The packet is ready for operator review, but it is not an approval request and
does not record acceptance. Approval request sending, approval record writes,
operator acceptance, dispatch, invocation, ledger writes, and receipt writes all
remain disabled.

## Packet Rules

- `operator_approval_packet_ready_for_review` requires source cutover preflight
  route `cutover_preflight_blocked_until_explicit_approval`.
- The registry guard must stay `require_approval_ledger`.
- The operator packet template must be present.
- The operator session binding must be present.
- Approval request sending must remain disabled.
- Operator approval record writes must remain disabled.
- Operator acceptance must remain absent for this read-only slice.
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
- No operator approval record write.
- No operator acceptance record.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_execution_operator_approval_packet.rs`
- Report:
  `scripts/hepta-systems-tool-execution-operator-approval-packet-report.sh`
- Gate: `scripts/hepta-systems-tool-execution-operator-approval-packet-gate.sh`
- Source gate: `scripts/hepta-systems-tool-execution-cutover-preflight-gate.sh`

## Next Move

The tool execution operator approval receipt projection has been restored
downstream. The next slice should restore an operator approval decision preflight
without invocation, while keeping lookup execution, ToolRegistry registration,
adapter dispatch, tool invocation, ledger writes, ApprovalBroker requests,
result receipts, package/release/live actions, and Public GA blocked until an
explicit cutover.
