# Hepta Systems Plugin Tool Manifest Schema Cutover Preflight - 2026-06-21

This note records the local-only manifest schema cutover preflight for plugin
tool candidates. The preflight combines ToolRegistry source-of-truth dry-run
candidates with parsed manifest tool metadata and decides whether each
candidate can only be forwarded as an approval-ledger dry-run or must be
blocked before registration.

## Current Checkout Reality

The current checkout has two planned plugin-tool candidates from the
source-of-truth dry-run. A manifest fixture readback source supplies complete
schema and policy metadata without rewriting `plugin.json`.

Current report facts:

- `planned_candidate_count=2`
- `parsed_manifest_declared_candidate_count=2`
- `registration_precondition_satisfied_count=2`
- `missing_manifest_precondition_count=0`
- `all_missing_manifest_preconditions_blocked=true`
- `manifest_schema_cutover_preflight_ready=true`
- `registration_cutover_allowed=true`
- live mutation disabled

Both planned candidates currently resolve to
`forward_require_approval_ledger_dry_run`. Forwarding remains dry-run only:
registration execution, invocation, ledgers, approvals, and live mutation are
still disabled.

## Preflight Decision Rules

The preflight forwards a candidate only when:

- the ToolRegistry source-of-truth dry-run is ready,
- the parsed manifest declaration is bound to a planned candidate,
- input and output schema declarations are complete,
- permission, activation, approval, ledger, and timeout policy declarations are
  complete,
- the candidate keeps the `require_approval_ledger` guard route.

Forwarding is still dry-run only. It does not enable registration, invocation,
ledger writes, approval requests, MCP startup, or app connector startup.

## Guardrails

- `block_manifest_preconditions` is used for missing schema or policy metadata.
- `block_source_registry` is used when source registry dry-run is not ready.
- Unknown manifest candidate ids fail closed as unbound declarations.
- Registration cutover execution disabled.
- Tool registration disabled.
- Tool invocation disabled.
- Ledger writes disabled.
- Approval requests disabled.
- Manifest rewrites and manifest schema writes disabled.
- Plugin cache, package lock, remote sync, workflow event log, local storage,
  SQLite, Telegram/provider/model/gateway/Native POST, package, release, and
  Public GA actions disabled.

## Files

- Rust adapter: `codex-rs/tools/src/plugin_tool_manifest_schema_cutover_preflight.rs`
- Report: `scripts/hepta-systems-plugin-tool-manifest-schema-cutover-preflight-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-manifest-schema-cutover-preflight-gate.sh`

## Next Move

Restore the tool registry invocation source-of-truth path without execution.
The router binding now consumes this preflight report and forwards the current
two planned candidates only as approval-ledger dry-runs.

## 2026-06-25 Fixture Readback Update

The schema cutover preflight now forwards the two complete manifest fixture
readback candidates as approval-ledger dry-runs. `registration_cutover_allowed`
can be true at the plan level, but registration execution, tool invocation,
ledger writes, approval requests, MCP startup, app connector startup, external
delivery, and release promotion remain disabled.
