# Hepta Systems Tool Registry Read-Only Dispatch Preflight - 2026-06-27

This note records Phase 2 of the Hepta systems convergence plan. The
Read-Only Dispatch Preflight uses the plugin lifecycle state machine as the
plugin source of truth, then projects a ToolRegistry dispatch path without
invocation.

## Sources

The preflight consumes these current local surfaces:

- `scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh`
- `scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh`
- `scripts/hepta-systems-tool-registry-router-lookup-shadow-report.sh`
- `scripts/hepta-systems-tool-invocation-ledger-approval-preflight-report.sh`
- `scripts/hepta-systems-tool-invocation-receipt-projection-report.sh`
- `codex-rs/tools/src/tool_registry_read_only_dispatch_preflight.rs`

The dispatch chain is intentionally narrow:

- registry lookup preview
- ledger preview
- approval preflight
- receipt projection

Both current `hepta-system` plugin tool candidates are carried through the
chain: one MCP server candidate and one app connector candidate.

This is a without invocation contract: it only projects the dispatch preflight
and receipt shape.

## Boundary

This surface is report-only. It does not:

- install plugins
- mutate plugin cache
- rewrite manifests
- enable ToolRegistry dispatch switches
- execute router registration lookup
- enable registry source-of-truth mutation
- register tools
- invoke tools
- write ToolInvocationLedger entries
- mutate ApprovalBroker state
- request approvals
- write result receipts
- start MCP servers
- start app connectors
- mutate workflow event logs or SQLite state
- read credentials
- invoke providers or models
- mutate gateway/auth or Native POST routing
- send channels
- package, release, or promote Public GA

## Next Move

Phase 3 should rebuild the Temporal-lite durable workflow adapter behind a
feature gate. The next workflow surface should provide append-only event-log
planning, lease/idempotency/checkpoint metadata, replay validation, and
rollback metadata without enabling workflow event-log writes or live execution.
