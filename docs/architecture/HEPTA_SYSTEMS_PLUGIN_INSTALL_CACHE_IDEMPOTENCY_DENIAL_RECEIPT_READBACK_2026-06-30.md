# Hepta Systems Plugin Install Cache Idempotency Denial Receipt Readback

## Purpose

This readback turns the install-cache noop preflight shape into a stability
check for idempotency and denial receipts. It keeps `hepta-system@hepta-local`
ready-blocked while proving that the noop preflight exposes stable idempotency keys, stable denial receipt ids, uniqueness checks, and idempotency-denial anchors.

The readback is query-only. It does not execute the noop preflight, write an
idempotency index, persist a denial receipt, materialize install cache, install
the plugin, start connectors, or register tools.

## Source

- Source report:
  `scripts/hepta-systems-plugin-install-cache-noop-preflight-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-install-cache-idempotency-denial-receipt-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a first and second idempotency-key readback
- a first and second denial-receipt-id readback
- a stable idempotency-key comparison
- a stable denial-receipt-id comparison
- a uniqueness check for idempotency keys
- a uniqueness check for denial receipt ids
- an idempotency-denial anchor
- a projected idempotency index entry without writing it

The source noop preflight must already have projected cache paths, artifact
digests, rollback/uninstall plans, idempotency keys, and denial receipt ids. All
execution, persistence, install, cache, and live counters remain zero.

## Closed Boundary

Closed boundary: no idempotency index write, denial receipt persistence, noop preflight execution, plugin install, cache mutation, install-cache materialization, rollback/uninstall execution, manifest rewrite, manifest schema write, dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 idempotency keys are stable.
- 2/2 idempotency keys are unique.
- 2/2 denial receipt ids are stable.
- 2/2 denial receipt ids are unique.
- 2/2 idempotency-denial anchors are projected.
- mismatch and duplicate counts stay at 0.
- 2/2 idempotency index entries are projected and 0 are written.
- 2/2 denial receipts are projected and 0 are persisted.
- 0 noop preflight executions, cache materializations, cache mutations, plugin
  installs, dynamic activations, tool registrations, ledger writes, approval
  requests, receipt persistence, runtime writes, or live executions occur.

## Next Step

The recommended next step is
`hepta_systems_plugin_install_cache_rollback_uninstall_noop_readback`: make the
rollback/uninstall noop plan queryable and stable while continuing to block
rollback execution, cache materialization, plugin install, dynamic activation,
connector startup, tool registration, runtime writes, and live paths.
