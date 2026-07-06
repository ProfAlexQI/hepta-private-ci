# Hepta Systems Tool Execution Dispatch Shadow - 2026-06-21

This note records the local-only Tool Execution Dispatch Shadow. The shadow
consumes the execution adapter preflight and proves that both planned plugin
tool candidates can be represented as future dispatch paths while the dispatch
and invocation switches remain disabled.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The dispatch
shadow is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, router lookup shadow, ledger approval
preflight, receipt projection, and execution adapter preflight. It does not
create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `dispatch_shadow_ready_count=2`
- `dispatch_shadow_blocked_count=0`
- `disabled_execution_dispatch_shadow_count=2`
- `all_execution_adapter_preflight_entries_shadowed=true`
- `all_dispatch_shadow_entries_keep_approval_guard=true`
- `tool_execution_dispatch_shadow_ready=true`
- `execution_dispatch_shadow_allowed=true`
- `tool_invocation_execution_switch_enabled=false`
- `adapter_dispatch_switch_enabled=false`

Both planned plugin tool candidates are bound to the disabled execution dispatch shadow:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp` ->
  `mcp_tool_call_adapter`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app` ->
  `app_connector_invocation_adapter`

The allowed flag is a dry-run precondition only. It proves that a future dispatch
path is known and shadowed, but it does not dispatch an adapter, invoke a tool,
write a ledger entry, request approval, write a result receipt, or open any live
mutation path.

## Shadow Rules

- `disabled_execution_dispatch_shadow` requires a source adapter route of
  `disabled_execution_adapter_preflight`.
- The registry guard must stay `require_approval_ledger`.
- The dispatch shadow binding must be present.
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

- Rust contract: `codex-rs/tools/src/tool_execution_dispatch_shadow.rs`
- Report: `scripts/hepta-systems-tool-execution-dispatch-shadow-report.sh`
- Gate: `scripts/hepta-systems-tool-execution-dispatch-shadow-gate.sh`
- Source gate: `scripts/hepta-systems-tool-execution-adapter-preflight-gate.sh`
- Existing executor API: `codex-rs/tools/src/tool_executor.rs`

## Next Move

The tool execution cutover preflight is now restored downstream. The next slice
should restore the operator approval packet without invocation, while keeping
lookup execution, ToolRegistry registration, adapter dispatch, tool invocation,
ledger writes, ApprovalBroker requests, result receipts, package/release/live
actions, and Public GA blocked until an explicit cutover.
