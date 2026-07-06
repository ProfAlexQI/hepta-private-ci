# Hepta Systems Plugin Install Cache Noop Preflight Readback

## Purpose

This readback turns the operator evidence/acceptance packet into a queryable
install-cache noop preflight shape. It keeps `hepta-system@hepta-local` in the
ready-blocked state while making the cache path, artifact digest, rollback/uninstall plan, idempotency key, and denial receipt explicit.

The readback is a dry shape only. It does not execute the noop preflight,
materialize an install cache, install the plugin, persist receipts, start
connectors, or register tools.

## Source

- Source report:
  `scripts/hepta-systems-plugin-operator-evidence-acceptance-packet-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-install-cache-noop-preflight-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each hepta-system candidate contribution, this readback projects:

- a noop install-cache preflight route
- a deterministic install-cache path
- a deterministic artifact digest
- a rollback/uninstall plan id
- an idempotency key
- a denial receipt id

The source operator packet, checklist, and non-acceptance receipt must already
be projected. Evidence recording and acceptance remain at zero.

## Closed Boundary

Closed boundary: no noop preflight execution, preflight persistence, denial receipt persistence, plugin install, cache mutation, install-cache materialization, rollback/uninstall execution, manifest rewrite, manifest schema write, dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 cache paths are projected.
- 2/2 artifact digests are projected.
- 2/2 rollback/uninstall plans are projected.
- 2/2 idempotency keys are projected.
- 2/2 denial receipts are projected.
- 2/2 noop preflight entries are ready.
- 0 noop preflight executions occur.
- 0 preflight records or denial receipts are persisted.
- 0 cache materialization, cache mutation, plugin install, dynamic activation,
  rollback/uninstall execution, tool registration, ledger write, approval
  request, receipt persistence, runtime write, or live execution occurs.

## Next Step

The recommended next step is
`hepta_systems_plugin_install_cache_idempotency_denial_receipt_readback`: verify
that noop preflight idempotency keys and denial receipt ids stay stable across
readbacks while continuing to block cache materialization, plugin install,
dynamic activation, connector startup, tool registration, runtime writes, and
live paths.
