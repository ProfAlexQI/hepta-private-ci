# Hepta Systems Tool Execution Canary Cutover Plan - 2026-06-21

This note records the local-only Tool Execution Canary Cutover Plan. This is a
without invocation recovery slice. It consumes the operator approval decision
preflight and declares canary scope, budget, rollback, and readback requirements
without accepting operator approval or starting a canary.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The canary
plan is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, execution adapter preflight, execution dispatch
shadow, execution cutover preflight, operator approval packet, operator approval
receipt projection, and operator approval decision preflight. It does not create
or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `canary_cutover_plan_ready_count=2`
- `canary_cutover_plan_blocked_count=0`
- `canary_scope_declared_count=2`
- `canary_budget_declared_count=2`
- `rollback_plan_required_count=2`
- `canary_readback_receipt_required_count=2`
- `canary_start_blocked_count=2`
- `all_decision_preflight_entries_bound_to_canary_plan=true`
- `all_canary_plan_entries_keep_no_invocation_guard=true`
- `tool_execution_canary_cutover_plan_ready=true`
- `tool_execution_canary_cutover_start_allowed=false`
- `tool_execution_canary_result_receipt_write_allowed=false`
- `tool_execution_live_cutover_allowed=false`

Both planned plugin tool candidates have canary plans, but canary start remains
blocked:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The plan is ready, but it does not record operator acceptance, enable a canary
switch, start canary execution, dispatch an adapter, invoke a tool, write the
tool ledger, send an ApprovalBroker request, write canary result receipts, run a
rollback, or write a final result receipt.

## Plan Rules

- `canary_cutover_plan_ready` requires source decision route
  `operator_approval_decision_pending_explicit_approval`.
- The registry guard must stay `require_approval_ledger`.
- Canary scope must be declared for each candidate.
- Canary budget must be declared for each candidate.
- Rollback plan must be declared before any start can be considered.
- Canary readback receipt and result receipt schema must be present.
- Operator acceptance must remain absent for this read-only slice.
- Canary and live cutover switches must remain disabled.
- Canary execution and rollback execution must remain disabled.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, ApprovalBroker requests, decision writes, decision receipt
  writes, canary result receipts, and result receipt writes remain disabled.

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
- No rollback execution.
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract:
  `codex-rs/tools/src/tool_execution_canary_cutover_plan.rs`
- Report:
  `scripts/hepta-systems-tool-execution-canary-cutover-plan-report.sh`
- Gate:
  `scripts/hepta-systems-tool-execution-canary-cutover-plan-gate.sh`
- Source gate:
  `scripts/hepta-systems-tool-execution-operator-approval-decision-preflight-gate.sh`

## Next Move

Restore the tool execution canary readback receipt projection without
invocation. The next slice should describe the receipts and readback evidence
that would be required after a future canary, while keeping canary start,
adapter dispatch, tool invocation, ledger writes, ApprovalBroker requests,
result receipt writes, package/release/live actions, and Public GA blocked until
an explicit cutover.
