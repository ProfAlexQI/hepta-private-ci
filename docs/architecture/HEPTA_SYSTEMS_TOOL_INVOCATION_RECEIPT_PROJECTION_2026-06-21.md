# Hepta Systems Tool Invocation Receipt Projection - 2026-06-21

This note records the local-only Tool Invocation Receipt Projection. The
projection consumes the ledger and ApprovalBroker preflight and proves that both
planned plugin tool candidates must have a result receipt and readback evidence
slot after any future invocation, without execution.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The receipt
projection is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, and ledger approval
preflight. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `receipt_projection_ready_count=2`
- `receipt_projection_blocked_count=0`
- `result_receipt_projection_required_count=2`
- `readback_evidence_required_count=2`
- `all_ledger_approval_entries_bound_to_receipt_projection=true`
- `all_receipt_projection_entries_keep_approval_guard=true`
- `tool_invocation_receipt_projection_ready=true`
- `result_receipt_projection_allowed=true`
- `tool_invocation_execution_switch_enabled=false`
- `result_receipt_write_switch_enabled=false`

Both planned plugin tool candidates are bound to the receipt projection:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The allowed flag is a dry-run precondition only. It proves that the candidates
must reserve a deterministic result receipt and readback evidence path after
ledger and approval preflight, but it does not invoke a tool, finish a ledger
record, write a receipt, or open any live mutation path.

## Projection Rules

- `result_receipt_projection_required` requires a source preflight route of
  `approval_ledger_preflight_required`.
- The registry guard must stay `require_approval_ledger`.
- The receipt projection binding must be present.
- The readback evidence binding must be present.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration and invocation remain disabled.
- Ledger writes, approval requests, and result receipt writes remain disabled.

## Guardrails

- No historical patch replay.
- No plugin fixture fabrication.
- No plugin install, cache mutation, package-lock mutation, or remote sync.
- No router registration lookup execution.
- No registry lookup execution.
- No ToolRegistry registration.
- No tool invocation.
- No ledger write.
- No ApprovalBroker request.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_invocation_receipt_projection.rs`
- Report: `scripts/hepta-systems-tool-invocation-receipt-projection-report.sh`
- Gate: `scripts/hepta-systems-tool-invocation-receipt-projection-gate.sh`
- Source gate: `scripts/hepta-systems-tool-invocation-ledger-approval-preflight-gate.sh`
- Existing runtime ledger API: `codex-rs/hepta-runtime/src/tool_invocation.rs`

## Next Move

The tool execution adapter preflight is now restored downstream. The next slice
should restore the tool execution dispatch shadow without invocation, while
keeping lookup execution, ToolRegistry registration, adapter dispatch, tool
invocation, ledger writes, ApprovalBroker requests, result receipts,
package/release/live actions, and Public GA blocked until an explicit cutover.
