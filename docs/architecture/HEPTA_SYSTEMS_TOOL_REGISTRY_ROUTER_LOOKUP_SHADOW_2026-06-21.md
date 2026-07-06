# Hepta Systems Tool Registry Router Lookup Shadow - 2026-06-21

This note records the local-only ToolRegistry router lookup shadow. The shadow
consumes the registration lookup cutover preflight and models the disabled
registration lookup switch without registration.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The lookup
shadow is fed by the replacement declaration report, manifest schema cutover
preflight, invocation router preflight binding, invocation source-of-truth
plan, and registration lookup cutover preflight. It does not create or rewrite
a manifest.

Current report facts:

- `candidate_count=2`
- `shadow_ready_count=2`
- `shadow_blocked_count=0`
- `disabled_lookup_shadow_count=2`
- `all_lookup_preflight_entries_shadowed=true`
- `all_shadow_entries_keep_approval_ledger_guard=true`
- `router_lookup_shadow_ready=true`
- `registration_lookup_cutover_switch_enabled=false`
- `router_registration_lookup_enabled=false`

Both planned plugin tool candidates are bound to the disabled lookup shadow:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The shadow being ready means the read-model can prove the lookup route without
opening the switch. It does not enable router lookup, registry lookup execution,
ToolRegistry registration, tool invocation, ledger writes, approval requests,
or live mutation.

## Shadow Rules

- `disabled_approval_ledger_lookup_shadow` requires a preflight route of
  `approval_ledger_lookup_dry_run`.
- The registry guard must stay `require_approval_ledger`.
- The registration lookup cutover switch must stay disabled.
- The shadow binding must be present.
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
- No approval request.
- No MCP server or app connector startup.
- No manifest rewrite or manifest schema write.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Rust contract: `codex-rs/tools/src/tool_registry_router_lookup_shadow.rs`
- Report: `scripts/hepta-systems-tool-registry-router-lookup-shadow-report.sh`
- Gate: `scripts/hepta-systems-tool-registry-router-lookup-shadow-gate.sh`
- Source gate: `scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-gate.sh`

## Next Move

The tool invocation ledger and ApprovalBroker preflight is now restored
downstream. The next slice should restore the tool invocation receipt projection
without execution, while keeping lookup execution, ToolRegistry registration,
tool invocation, ledger writes, approval requests, package/release/live actions,
and Public GA blocked until an explicit cutover.
