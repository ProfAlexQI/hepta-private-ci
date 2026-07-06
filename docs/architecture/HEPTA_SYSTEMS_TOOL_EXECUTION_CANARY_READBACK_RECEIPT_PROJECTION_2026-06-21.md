# Hepta Systems Tool Execution Canary Readback Receipt Projection - 2026-06-21

This note records the local-only Tool Execution Canary Readback Receipt
Projection. This is a without invocation recovery slice. It consumes the canary
cutover plan and declares the readback channel, receipt digest, trace
correlation, rollback readback, and operator summary requirements that a future
canary would need before result acceptance can be considered.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The readback
receipt projection is fed by the replacement declaration report, manifest schema
cutover preflight, invocation router preflight binding, invocation
source-of-truth plan, registration lookup cutover preflight, router lookup
shadow, ledger approval preflight, receipt projection, execution adapter
preflight, execution dispatch shadow, execution cutover preflight, operator
approval packet, operator approval receipt projection, operator approval
decision preflight, and canary cutover plan. It does not create or rewrite a
manifest.

Current report facts:

- `candidate_count=2`
- `canary_readback_receipt_projection_ready_count=2`
- `canary_readback_receipt_projection_blocked_count=0`
- `canary_readback_channel_declared_count=2`
- `canary_result_receipt_digest_required_count=2`
- `canary_trace_correlation_required_count=2`
- `rollback_readback_required_count=2`
- `operator_summary_required_count=2`
- `canary_result_receipt_write_blocked_count=2`
- `all_canary_plan_entries_bound_to_readback_projection=true`
- `all_canary_readback_entries_keep_no_invocation_guard=true`
- `tool_execution_canary_readback_receipt_projection_ready=true`
- `tool_execution_canary_result_receipt_write_allowed=false`
- `tool_execution_canary_result_acceptance_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates have readback receipt projections, but
receipt writes and result acceptance remain blocked:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The projection is ready, but it does not start canary execution, write canary
result receipts, write readback projections, accept canary results, dispatch an
adapter, invoke a tool, write the tool ledger, send an ApprovalBroker request,
run rollback, or write a final result receipt.

## Projection Rules

- `canary_readback_receipt_projection_ready` requires source canary plan route
  `canary_cutover_plan_ready`.
- The registry guard must stay `require_approval_ledger`.
- Canary readback channel must be declared.
- Canary result receipt digest must be required.
- Canary trace correlation must be required.
- Rollback readback must be required.
- Operator summary must be required.
- Canary and live cutover switches must remain disabled.
- Canary execution, canary receipt writes, readback projection writes, and
  rollback execution must remain disabled.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, ApprovalBroker requests, decision writes, decision receipt
  writes, canary result receipts, canary result acceptance, and final result
  receipt writes remain disabled.

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
- No canary result receipt write.
- No canary readback projection write.
- No canary result acceptance.
- No rollback execution.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_canary_readback_receipt_projection.rs`
- Report:
  `scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-canary-cutover-plan-gate.sh`

## Next Move

Restore the tool execution canary result acceptance preflight without
invocation. The next slice should make result acceptance depend on explicit
canary receipts and readback evidence while keeping acceptance, live cutover,
adapter dispatch, tool invocation, ledger writes, ApprovalBroker requests,
result receipt writes, package/release/live actions, and Public GA blocked until
an explicit cutover.
