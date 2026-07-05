# Hepta Systems Plugin ToolRegistry Registration Denial Receipt Readback

## Purpose

This readback turns the dynamic activation and connector-start boundary into a
ToolRegistry registration denial receipt boundary. It keeps
`hepta-system@hepta-local` ready-blocked while proving that plugin tool schemas
can be mapped to deterministic ToolRegistry registration denial ids, denial
receipts, router lookup blocks, registry source-of-truth blocks, and invocation
denials.

The readback is query-only. It does not register tools, mutate the
ToolRegistry, execute registry lookup, invoke tools, write ledgers, request
approval, persist receipts, start dynamic activation, grant permissions, start
MCP servers, start app connectors, install plugins, materialize cache, write
runtime event logs, or open live execution.

## Source

- Source report:
  `scripts/hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-tool-registry-registration-denial-receipt-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a tool schema digest
- a ToolRegistry registration denial id
- stable first and second registration denial receipt ids
- a router lookup block key
- a registry source-of-truth block key
- an invocation denial id

Together these make tool schema digests, registration denial ids, stable registration denial receipts, router lookup blocks, registry source-of-truth blocks, and invocation denials queryable before registration is allowed.

The source dynamic activation boundary must already have projected manual
activation events, permission gates, connector start plans, ToolRegistry
registration denials, ledger denials, receipt denials, activation denial
receipts, and zero activation, connector, registration, ledger, receipt,
runtime, or live side effects.

## Closed Boundary

Closed boundary: no ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, ledger write, approval request, receipt persistence, dynamic activation, permission grant, MCP server start, app connector start, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 tool schemas are bound.
- 2/2 tool schema digests are projected.
- 2/2 ToolRegistry registration denial ids are projected.
- 2/2 registration denial receipts are stable.
- 2/2 registration denial receipts are unique.
- 2/2 router lookup blocks are projected.
- 2/2 registry source-of-truth blocks are projected.
- 2/2 invocation denials are projected.
- 0 ToolRegistry registrations, ToolRegistry mutations, registry lookups, tool
  invocations, ledger writes, approval requests, receipt persistence writes,
  dynamic activations, permission grants, connector starts, cache mutations,
  plugin installs, runtime writes, SQLite writes, or live executions occur.

## Next Step

The recommended next step is
`hepta_systems_plugin_tool_invocation_noop_denial_receipt_readback`: make
tool invocation noop denial receipts queryable and stable before any plugin
tool path can execute.
