# Hepta Systems Tool Execution Adapter Preflight - 2026-06-21

This note records the local-only Tool Execution Adapter Preflight. The preflight
consumes the tool invocation receipt projection and proves that both planned
plugin tool candidates can only enter a disabled execution adapter path, without
invocation.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The execution
adapter preflight is fed by the replacement declaration report, manifest schema
cutover preflight, invocation router preflight binding, invocation
source-of-truth plan, registration lookup cutover preflight, router lookup
shadow, ledger approval preflight, and receipt projection. It does not create or
rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `execution_adapter_preflight_ready_count=2`
- `execution_adapter_preflight_blocked_count=0`
- `disabled_execution_adapter_preflight_count=2`
- `mcp_tool_call_adapter_preflight_count=1`
- `app_connector_invocation_adapter_preflight_count=1`
- `all_receipt_projection_entries_bound_to_execution_adapter_preflight=true`
- `all_execution_adapter_entries_keep_approval_guard=true`
- `tool_execution_adapter_preflight_ready=true`
- `execution_adapter_preflight_allowed=true`
- `tool_invocation_execution_switch_enabled=false`
- `adapter_dispatch_switch_enabled=false`

Both planned plugin tool candidates are bound to disabled execution adapters:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp` ->
  `mcp_tool_call_adapter`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app` ->
  `app_connector_invocation_adapter`

The allowed flag is a dry-run precondition only. It proves that receipt-projected
candidates have a known future adapter route, but it does not dispatch an
adapter, invoke a tool, write a ledger entry, request approval, write a result
receipt, or open any live mutation path.

## Preflight Rules

- `disabled_execution_adapter_preflight` requires a source projection route of
  `result_receipt_projection_required`.
- The registry guard must stay `require_approval_ledger`.
- The execution adapter binding must be present.
- `mcp_server` maps to `mcp_tool_call_adapter`.
- `app_connector` maps to `app_connector_invocation_adapter`.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration, adapter dispatch, and invocation remain disabled.
- Ledger writes, approval requests, and result receipt writes remain disabled.

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
- No result receipt write.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_execution_adapter_preflight.rs`
- Report: `scripts/hepta-systems-tool-execution-adapter-preflight-report.sh`
- Gate: `scripts/hepta-systems-tool-execution-adapter-preflight-gate.sh`
- Source gate: `scripts/hepta-systems-tool-invocation-receipt-projection-gate.sh`
- Existing executor API: `codex-rs/tools/src/tool_executor.rs`

## Next Move

The tool execution dispatch shadow is now restored downstream. The next slice
should restore the tool execution cutover preflight without invocation, while
keeping lookup execution, ToolRegistry registration, adapter dispatch, tool
invocation, ledger writes, ApprovalBroker requests, result receipts,
package/release/live actions, and Public GA blocked until an explicit cutover.
