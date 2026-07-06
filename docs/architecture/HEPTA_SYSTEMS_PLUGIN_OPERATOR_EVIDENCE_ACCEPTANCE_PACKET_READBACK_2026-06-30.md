# Hepta Systems Plugin Operator Evidence Acceptance Packet Readback

## Purpose

This readback turns the plugin signature/trust/install-cache boundary into an
operator-visible packet without sending or persisting that packet. It keeps the
`hepta-system@hepta-local` plugin in the ready-blocked state while making the
required evidence and acceptance checklist explicit.

The packet covers signature artifact, trust root, install-cache plan, rollback/uninstall plan, and operator acceptance checklist.

## Source

- Source report:
  `scripts/hepta-systems-plugin-signature-trust-install-cache-boundary-readback-report.sh`
- Readback gate:
  `scripts/hepta-systems-plugin-operator-evidence-acceptance-packet-readback-gate.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`

## Contract

For each canonical hepta-system candidate contribution, this readback projects:

- an operator packet route
- a checklist route
- signature artifact evidence requirement
- trust-root evidence requirement
- install-cache plan evidence requirement
- rollback/uninstall plan evidence requirement
- five explicit acceptance checks: signature, trust root, install cache,
  rollback/uninstall, and dynamic activation
- an explicit non-acceptance receipt projection without persistence

This layer is deliberately a packet/readback only. It does not record evidence,
accept trust, verify signatures, install the plugin, materialize install cache,
start connectors, or register tools.

## Closed Boundary

Closed boundary: no operator packet send, operator packet persistence, checklist persistence, evidence recording, acceptance recording, signature acceptance, trust-root acceptance, plugin install, cache mutation, install-cache materialization, rollback/uninstall execution, manifest rewrite, manifest schema write, dynamic activation, permission grant, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 operator packets are projected.
- 2/2 acceptance checklists are projected.
- 8 evidence items are required and 0 are recorded.
- 10 acceptance checks are required and 0 are recorded.
- 2/2 non-acceptance receipts are projected and 0 are persisted.
- 0 operator packets are sent or persisted.
- 0 plugin install, cache mutation, install-cache materialization,
  rollback/uninstall execution, dynamic activation, tool registration, ledger
  write, approval request, receipt persistence, runtime write, or live execution
  occurs.

## Next Step

The recommended next step is
`hepta_systems_plugin_install_cache_noop_preflight_readback`: make the install
cache preflight shape queryable as a noop/dry-run plan while continuing to block
cache materialization, plugin install, dynamic activation, connector startup,
tool registration, runtime writes, and live paths.
