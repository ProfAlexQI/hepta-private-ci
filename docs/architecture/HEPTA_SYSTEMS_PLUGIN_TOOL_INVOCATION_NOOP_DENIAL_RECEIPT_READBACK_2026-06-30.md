# Hepta Systems Plugin Tool Invocation Noop Denial Receipt Readback

## Purpose

This readback turns the ToolRegistry registration denial receipt boundary into a
plugin tool invocation noop denial receipt boundary. It keeps
`hepta-system@hepta-local` ready-blocked while proving that plugin tool
invocation denial ids, noop result projections, noop result digests, invocation
denial receipts, ledger denial anchors, approval denial anchors, receipt denial
anchors, and invocation idempotency keys can be queried before any plugin tool
path is registered or invoked.

The readback is query-only. It does not register tools, mutate the
ToolRegistry, execute registry lookup, invoke tools, persist noop results,
write ledgers, request approval, persist receipts, start dynamic activation,
grant permissions, start MCP servers, start app connectors, install plugins,
materialize cache, write runtime event logs, or open live execution.

## Source

- Source report:
  `scripts/hepta-systems-plugin-tool-registry-registration-denial-receipt-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-tool-invocation-noop-denial-receipt-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a plugin tool invocation denial id
- a noop result projection id
- a noop result digest
- stable first and second invocation denial receipt ids
- a ledger denial anchor id
- an approval denial anchor id
- a receipt denial anchor id
- stable first and second invocation idempotency keys

Together these make invocation denial ids, noop result projections, stable invocation denial receipts, ledger denial anchors, approval denial anchors, receipt denial anchors, and idempotency keys queryable before invocation is allowed.

The source ToolRegistry registration denial receipt readback must already have
projected tool schema digests, registration denial ids, stable registration
denial receipts, router lookup blocks, registry source-of-truth blocks,
invocation denials, and zero registration, lookup, invocation, ledger, receipt,
runtime, or live side effects.

## Closed Boundary

Closed boundary: no ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, noop result persistence, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 invocation denial ids are projected.
- 2/2 noop result projections are projected.
- 2/2 noop result digests are projected.
- 2/2 invocation denial receipts are stable.
- 2/2 invocation denial receipts are unique.
- 2/2 ledger denial anchors are projected.
- 2/2 approval denial anchors are projected.
- 2/2 receipt denial anchors are projected.
- 2/2 invocation idempotency keys are projected.
- 2/2 invocation idempotency keys are stable.
- 2/2 invocation idempotency keys are unique.
- 0 ToolRegistry registrations, ToolRegistry mutations, registry lookups, tool
  invocations, noop result persistence writes, ledger writes, approval
  requests, receipt persistence writes, dynamic activations, permission grants,
  connector starts, cache mutations, plugin installs, runtime writes, SQLite
  writes, or live executions occur.

## Next Step

The recommended next step is
`hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback`:
bind plugin tool policy, approval, ledger, and receipt denial boundaries to the
noop invocation contract before any real plugin tool path can execute.
