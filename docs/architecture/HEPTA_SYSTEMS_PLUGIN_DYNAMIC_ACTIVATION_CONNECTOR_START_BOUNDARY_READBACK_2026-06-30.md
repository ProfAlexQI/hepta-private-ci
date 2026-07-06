# Hepta Systems Plugin Dynamic Activation Connector Start Boundary Readback

## Purpose

This readback turns the rollback/uninstall noop boundary into a dynamic
activation and connector-start boundary. It keeps `hepta-system@hepta-local`
ready-blocked while proving that plugin activation can expose manual activation events, permission gates, connector start plans, ToolRegistry registration denials, ledger denials, receipt denials, and activation denial receipts.

The readback is query-only. It does not install the plugin, materialize cache,
start dynamic activation, grant permissions, start MCP servers, start app
connectors, register tools, write ledgers, request approval, persist receipts,
write runtime event logs, or open live execution.

## Source

- Source report:
  `scripts/hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a manual activation event
- a permission gate
- a connector start plan
- a connector start route
- a ToolRegistry registration denial id
- a ledger denial id
- a receipt denial id
- an activation denial receipt id

The source rollback/uninstall noop readback must already have projected stable
rollback/uninstall plan ids, guard keys, denial receipt anchors, and zero
execution or persistence side effects. This boundary keeps activation,
connector start, ToolRegistry registration, tool invocation, ledger writes,
approval requests, receipt persistence, runtime writes, and live execution
closed.

## Closed Boundary

Closed boundary: no dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, plugin install, cache mutation, install-cache materialization, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 manual activation events are projected.
- 2/2 permission gates are projected.
- 2/2 connector start plans are projected.
- 1/1 MCP server start plan is projected.
- 1/1 app connector start plan is projected.
- 2/2 ToolRegistry registration denials are projected.
- 2/2 ledger denials are projected.
- 2/2 receipt denials are projected.
- 2/2 activation denial receipts are projected.
- 0 dynamic activations, permission grants, connector starts, tool
  registrations, tool invocations, ledger writes, approval requests, receipt
  persistence writes, runtime writes, cache mutations, plugin installs, or live
  executions occur.

## Next Step

The recommended next step is
`hepta_systems_plugin_tool_registry_registration_denial_receipt_readback`: make
ToolRegistry registration denial ids and receipts queryable and stable before
any plugin tool contribution can be registered or invoked.
