# Hepta Systems Plugin Canonical Manifest Permission Activation Contract Readback

Date: 2026-06-30

## Scope

This readback promotes the `hepta-system` dogfood fixture into a canonical
plugin contract surface without installing, activating, or mutating it.

The contract covers manifest, permission, activation, tool policy, version, signature, and trust boundaries for the two current hepta-system tool candidates:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

## Source Reports

The report consumes these local source reports:

- `scripts/hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-report.sh`
- `scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh`
- `scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh`
- `scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-report.sh`
- `scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh`
- `scripts/hepta-systems-plugin-tool-manifest-schema-cutover-preflight-report.sh`

It also reads `plugins/hepta-system/.codex-plugin/plugin.json` as the local
fixture source of truth. The Rust read model lives in
`codex-rs/hepta-runtime/src/hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback.rs`.

## Contract

The report is ready-blocked when all of these hold:

- manifest identity is `hepta-system`
- manifest version is `0.0.0-fixture`
- 1 skill path, 1 MCP server path, and 1 app connector path are present
- 2 tool schemas are declared with input/output schemas
- 2 permission declarations are present with network disabled
- 2 manual activation events are declared
- 2 tool policies include approval, ledger, and timeout metadata
- the MCP candidate is constrained to read-only filesystem and no network
- the app connector candidate is constrained to its connector binding and no
  network
- signature/trust boundaries are checked, but no signature or trust root is
  accepted

## Closed Boundary

Closed boundary: no plugin install, cache mutation, manifest rewrite, manifest schema write, dynamic activation, permission grant, signature acceptance, trust-root acceptance, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution.

## Next Step

The next migration step is
`hepta_systems_plugin_signature_trust_install_cache_boundary_readback`.
That should deepen the signature/trust/install-cache boundary as a readback
contract before any plugin install or cache mutation is enabled.
