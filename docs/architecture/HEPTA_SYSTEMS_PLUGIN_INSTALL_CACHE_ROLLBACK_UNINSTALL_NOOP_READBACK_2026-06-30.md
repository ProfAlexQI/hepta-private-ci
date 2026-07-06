# Hepta Systems Plugin Install Cache Rollback Uninstall Noop Readback

## Purpose

This readback turns the install-cache idempotency and denial receipt projection
into a rollback/uninstall noop boundary. It keeps `hepta-system@hepta-local`
ready-blocked while proving that the plugin install-cache path exposes stable rollback/uninstall plan ids, rollback noop routes, uninstall noop routes, guard keys, cache-restore blocks, and denial receipt anchors.

The readback is query-only. It does not execute rollback or uninstall, persist
plans, write idempotency indexes, persist denial receipts, materialize cache,
install plugins, start connectors, or register tools.

## Source

- Source report:
  `scripts/hepta-systems-plugin-install-cache-idempotency-denial-receipt-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a first and second rollback/uninstall plan-id readback
- a stable rollback/uninstall plan comparison
- a uniqueness check for rollback/uninstall plan ids
- a rollback noop route
- an uninstall noop route
- a rollback guard key
- an uninstall guard key
- a cache-restore block key
- a denial receipt anchor for the rollback/uninstall noop path

The source idempotency/denial receipt readback must already have projected
stable idempotency keys, stable denial receipt ids, idempotency-denial anchors,
and zero writes. All rollback/uninstall execution, plan persistence, install,
cache, connector, tool, ledger, receipt, runtime, and live counters remain zero.

## Closed Boundary

Closed boundary: no rollback/uninstall execution, rollback plan persistence, uninstall plan persistence, idempotency index write, denial receipt persistence, noop preflight execution, plugin install, cache mutation, install-cache materialization, rollback cache restore, uninstall execution, manifest rewrite, manifest schema write, dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 rollback/uninstall plan ids are stable.
- 2/2 rollback/uninstall plan ids are unique.
- 2/2 rollback noop routes are projected.
- 2/2 uninstall noop routes are projected.
- 2/2 rollback guard keys are projected.
- 2/2 uninstall guard keys are projected.
- 2/2 cache-restore block keys are projected.
- 2/2 denial receipt anchors are projected.
- mismatch and duplicate counts stay at 0.
- 0 rollback/uninstall executions, plan persistence writes, idempotency index
  writes, denial receipt persistence, cache materializations, cache mutations,
  plugin installs, dynamic activations, tool registrations, ledger writes,
  approval requests, receipt persistence, runtime writes, or live executions
  occur.

## Next Step

The recommended next step is
`hepta_systems_plugin_dynamic_activation_connector_start_boundary_readback`: make
dynamic activation and connector startup boundaries queryable while continuing
to block plugin install, cache mutation, permission grants, connector startup,
ToolRegistry registration, runtime writes, and live paths.
