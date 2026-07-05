# Hepta Systems Plugin Signature Trust Install Cache Boundary Readback

## Purpose

This readback makes the post-canonical plugin trust gate explicit for the
`hepta-system@hepta-local` fixture. The prior canonical manifest contract proves
the declared skill, MCP server, app connector, tool schemas, permissions,
activation events, and tool policies are structurally queryable. This layer
separates that manifest readiness from the still-closed install path.

The readback covers signature artifact, trust root, install cache, operator evidence, and operator acceptance boundaries.

## Source

- Source report:
  `scripts/hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-report.sh`
- Source plugin:
  `plugins/hepta-system/.codex-plugin/plugin.json`
- Readback gate:
  `scripts/hepta-systems-plugin-signature-trust-install-cache-boundary-readback-gate.sh`

## Contract

For each canonical hepta-system candidate contribution, this readback projects:

- signature boundary readiness before install
- trust-root boundary readiness before install
- install-cache route projection without materialization
- required operator evidence before any signature or trust acceptance
- required operator acceptance before any install, cache, or activation transition
- explicit non-acceptance receipt projection without persistence

The current fixture intentionally has no signature artifact and no trust root.
That is an operator-visible blocker, not an implicit permission to install.

## Closed Boundary

Closed boundary: no plugin install, cache mutation, install-cache materialization, manifest rewrite, manifest schema write, dynamic activation, permission grant, signature verification, signature acceptance, trust-root acceptance, operator evidence recording, operator acceptance recording, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Expected Readback

- 2 candidate contributions remain in scope.
- 2/2 signature boundaries are projected.
- 2/2 trust boundaries are projected.
- 2/2 install-cache boundaries are projected.
- 2/2 candidates require operator evidence.
- 2/2 candidates require operator acceptance.
- 0 signature artifacts are present.
- 0 trust roots are present.
- 0 install-cache entries are materialized.
- 0 evidence or acceptance records are written.
- 0 plugin install, cache mutation, dynamic activation, tool registration,
  ledger write, approval request, receipt persistence, runtime write, or live
  execution occurs.

## Next Step

The recommended next step is
`hepta_systems_plugin_operator_evidence_acceptance_packet_readback`: make the
operator evidence packet and acceptance checklist queryable without recording
evidence, accepting trust, installing the plugin, mutating the cache, or enabling
live paths.
