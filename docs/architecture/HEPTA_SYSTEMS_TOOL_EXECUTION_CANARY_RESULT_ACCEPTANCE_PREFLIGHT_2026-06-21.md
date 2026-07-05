# Hepta Systems Tool Execution Canary Result Acceptance Preflight - 2026-06-21

This note records the local-only Tool Execution Canary Result Acceptance
Preflight. This is a without invocation recovery slice. It consumes the canary
readback receipt projection and records that canary result acceptance remains
pending evidence.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The canary
result acceptance preflight is fed by the replacement declaration report,
manifest schema cutover preflight, invocation router preflight binding,
invocation source-of-truth plan, registration lookup cutover preflight, router
lookup shadow, ledger approval preflight, receipt projection, execution adapter
preflight, execution dispatch shadow, execution cutover preflight, operator
approval packet, operator approval receipt projection, operator approval
decision preflight, canary cutover plan, and canary readback receipt projection.
It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `canary_result_acceptance_preflight_ready_count=2`
- `canary_result_acceptance_preflight_blocked_count=0`
- `canary_result_acceptance_pending_evidence_count=2`
- `canary_result_receipt_required_count=2`
- `canary_readback_evidence_required_count=2`
- `canary_acceptance_record_write_blocked_count=2`
- `canary_acceptance_receipt_write_blocked_count=2`
- `all_readback_projections_bound_to_acceptance_preflight=true`
- `all_acceptance_preflight_entries_keep_no_invocation_guard=true`
- `tool_execution_canary_result_acceptance_preflight_ready=true`
- `tool_execution_canary_result_acceptance_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates are pending canary result evidence:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The preflight is ready, but it does not write canary result receipts, mark
readback evidence present, record canary result acceptance, write canary
acceptance receipts, start live cutover, dispatch an adapter, invoke a tool,
write the tool ledger, send an ApprovalBroker request, or write a final result
receipt.

## Preflight Rules

- `canary_result_acceptance_pending_evidence` requires source readback route
  `canary_readback_receipt_projection_ready`.
- The registry guard must stay `require_approval_ledger`.
- Canary result acceptance policy must be present.
- Operator identity binding must be present.
- Canary result receipt must be required but absent for this read-only slice.
- Canary readback evidence must be required but absent for this read-only slice.
- Operator canary result acceptance must remain absent.
- Canary acceptance record writes must remain disabled.
- Canary acceptance receipt writes must remain disabled.
- Live cutover switch must remain disabled.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, ApprovalBroker requests, result acceptance, and final result
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
- No canary result receipt write.
- No canary readback evidence write.
- No canary result acceptance record.
- No canary acceptance receipt write.
- No live cutover start.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_canary_result_acceptance_preflight.rs`
- Report:
  `scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-canary-result-acceptance-preflight-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-canary-readback-receipt-projection-gate.sh`

## Next Move

Restore the tool execution live cutover preflight without invocation. The next
slice should bind canary evidence, acceptance, explicit operator cutover, and
rollback/readback requirements into a final live-cutover blocker matrix while
keeping adapter dispatch, tool invocation, ledger writes, ApprovalBroker
requests, result receipt writes, package/release/live actions, and Public GA
blocked until an explicit cutover.
