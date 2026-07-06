# Hepta Systems Tool Registry Registration Lookup Cutover Preflight - 2026-06-21

This note records the local-only registration lookup cutover preflight for the
ToolRegistry plugin tool path. The preflight consumes the invocation
source-of-truth report and confirms that the two planned plugin tool candidates
can reach an approval-ledger lookup dry-run path without execution.

## Current Checkout Reality

The current checkout still has no `hepta-system` manifest fixture. The
preflight remains fed by the replacement declaration report, manifest schema
cutover preflight, invocation router preflight binding, and invocation
source-of-truth plan. It does not create or rewrite a manifest.

Current report facts:

- `candidate_count=2`
- `source_invocation_ready_count=2`
- `lookup_precondition_satisfied_count=2`
- `lookup_blocked_count=0`
- `approval_ledger_lookup_dry_run_count=2`
- `all_invocation_sources_bound_to_lookup_preflight=true`
- `all_lookup_entries_keep_approval_ledger_guard=true`
- `registration_lookup_cutover_preflight_ready=true`
- `registration_lookup_cutover_allowed=true`

Both planned plugin tool candidates are bound to
`approval_ledger_lookup_dry_run`:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

The allowed flag means the local dry-run preconditions are satisfied. It does
not enable router lookup, registry lookup execution, ToolRegistry registration,
tool invocation, ledger writes, approval requests, or live mutation.

## Preflight Rules

- `approval_ledger_lookup_dry_run` requires an invocation source route of
  `approval_ledger_dry_run_source_only`.
- The registry guard must stay `require_approval_ledger`.
- Router registration lookup remains disabled.
- Registry lookup execution remains disabled.
- Registry source-of-truth enablement remains disabled.
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

- Rust contract: `codex-rs/tools/src/tool_registry_registration_lookup_cutover_preflight.rs`
- Report: `scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-report.sh`
- Gate: `scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-gate.sh`
- Source gate: `scripts/hepta-systems-tool-registry-invocation-source-of-truth-gate.sh`

## Next Move

Restore the ToolRegistry router lookup shadow without registration. The next
slice should model the disabled registration lookup switch against this
preflight output while keeping lookup execution, ToolRegistry registration,
tool invocation, ledger writes, approvals, package/release/live actions, and
Public GA blocked until an explicit cutover.
