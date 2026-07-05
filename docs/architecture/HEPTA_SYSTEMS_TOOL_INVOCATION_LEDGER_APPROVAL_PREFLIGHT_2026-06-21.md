# Hepta Systems Tool Invocation Ledger Approval Preflight - 2026-06-21

This note records the local-only Tool Invocation Ledger Approval Preflight. The
preflight consumes the ToolRegistry router lookup shadow and maps both planned
plugin tool candidates to the required `ToolInvocationLedger` plus
`ApprovalBroker` planning surface without execution.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The preflight
is fed by the replacement declaration report, manifest schema cutover preflight,
invocation router preflight binding, invocation source-of-truth plan,
registration lookup cutover preflight, and router lookup shadow. It does not
create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `ledger_approval_preflight_ready_count=2`
- `ledger_approval_preflight_blocked_count=0`
- `approval_ledger_preflight_required_count=2`
- `all_shadow_entries_bound_to_ledger_approval_preflight=true`
- `all_ledger_approval_entries_keep_approval_guard=true`
- `tool_invocation_ledger_approval_preflight_ready=true`
- `ledger_approval_preflight_allowed=true`
- `ledger_write_switch_enabled=false`
- `approval_request_switch_enabled=false`

Both planned plugin tool candidates are bound to the ledger approval preflight:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The allowed flag is a dry-run precondition only. It proves that the candidates
must route through ledger and ApprovalBroker planning before execution, but it
does not write the ledger, request approval, invoke a tool, register a tool, or
open any live mutation path.

## Preflight Rules

- `approval_ledger_preflight_required` requires a source shadow route of
  `disabled_approval_ledger_lookup_shadow`.
- The registry guard must stay `require_approval_ledger`.
- The `ToolInvocationLedger` binding must be present.
- The `ApprovalBroker` preflight binding must be present.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Tool registration and invocation remain disabled.
- Ledger writes and approval requests remain disabled.

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
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_invocation_ledger_approval_preflight.rs`
- Report: `scripts/hepta-systems-tool-invocation-ledger-approval-preflight-report.sh`
- Gate: `scripts/hepta-systems-tool-invocation-ledger-approval-preflight-gate.sh`
- Source gate: `scripts/hepta-systems-tool-registry-router-lookup-shadow-gate.sh`
- Existing runtime ledger API: `codex-rs/hepta-runtime/src/tool_invocation.rs`
- Existing runtime approval API: `codex-rs/hepta-runtime/src/approval_broker.rs`

## Next Move

The tool invocation receipt projection is now restored downstream. The next
slice should restore the tool execution adapter preflight without invocation,
while keeping lookup execution, ToolRegistry registration, tool invocation,
ledger writes, ApprovalBroker requests, result receipt writes,
package/release/live actions, and Public GA blocked until an explicit cutover.
