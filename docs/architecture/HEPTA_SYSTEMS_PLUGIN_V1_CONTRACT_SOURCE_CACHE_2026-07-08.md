# Hepta Systems Plugin v1 Contract Source Cache - 2026-07-08

This source-cache keeps the Plugins v1 boundary short and queryable without
adding more current-reality matrix rows. It consumes existing readbacks for the
`hepta-system` fixture and locks the manifest/permission/activation/toolPolicy
schema v1 contract before any install or activation path is opened.

Stable anchor: manifest/permission/activation/toolPolicy schema v1.

Stable closed-boundary anchor: no manifest rewrite, schema write, plugin install, cache mutation, signature acceptance, trust-root acceptance, permission grant, dynamic activation, connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, transport mutation, canary, live, or Public GA.

## Sources

- `scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh`
- `scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-report.sh`
- `scripts/hepta-systems-plugin-tool-manifest-schema-cutover-preflight-report.sh`
- `scripts/hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-report.sh`
- `scripts/hepta-systems-plugin-signature-trust-install-cache-boundary-readback-report.sh`
- `scripts/hepta-systems-plugin-install-cache-noop-preflight-readback-report.sh`
- `scripts/hepta-systems-plugin-install-cache-idempotency-denial-receipt-readback-report.sh`
- `scripts/hepta-systems-plugin-install-cache-rollback-uninstall-noop-readback-report.sh`
- `scripts/hepta-systems-plugin-dynamic-activation-connector-start-boundary-readback-report.sh`
- `codex-rs/core-plugins/src/manifest_v1_validator.rs`

## Contract

- Manifest schema v1: `toolSchemas` is keyed by canonical candidate id and
  each entry carries `inputSchema` and `outputSchema`.
- Permission schema v1: `permissions` is keyed by the same candidate ids and
  keeps network set to `none`; the MCP path is filesystem read-only, and the
  app connector path is a local connector scope.
- Activation schema v1: `activationEvents` is keyed by the same candidate ids
  and remains manual-only.
- Tool policy schema v1: `toolPolicies` requires approval, ledger, and a
  bounded timeout before any registration path.

The schema validator kernel lives in `codex-core-plugins` and is read-only. It
accepts the `hepta-system` fixture contract and rejects missing tool schema,
missing permission, permission drift, non-manual activation, and tool-policy
drift. It reports closed boundary flags for manifest schema write, plugin
install, cache mutation, dynamic activation, ToolRegistry registration, tool
invocation, and live execution.

The migration runner is also dry-run only. It emits the five schema migration
phases for a valid v1 fixture and blocks invalid or empty manifests without
writing migration state, rewriting manifests, mutating plugin cache, installing
plugins, registering tools, invoking tools, or enabling live execution.

The signature/trust verifier is dry-run only. It projects signature artifact,
trust root, operator evidence, and operator acceptance requirements for each
candidate and can block invalid manifests, but it does not verify signatures,
accept signatures, accept trust roots, materialize install cache, install
plugins, mutate cache, register tools, invoke tools, or enable live execution.

The install-cache fixture kernel is dry-run only. It derives deterministic cache
keys, dry-run cache paths, artifact digest labels, idempotency keys,
rollback/uninstall no-op plan ids, and denial receipt ids from a validated
manifest and the signature/trust dry-run boundary. It can block invalid
manifests, but it does not write fixture files, persist denial receipts, write
runtime event logs, materialize install cache, mutate cache, install plugins,
activate connectors, register tools, invoke tools, or enable live execution.

The schema migration plan is visible-only: it names the v1 schema header,
candidate-id canonicalization, permission normalization, manual-only activation,
and approval/ledger/timeout tool policy normalization. It does not rewrite the
manifest and does not write schema state.

The signature/trust plan requires a signature artifact, trust root, signature
verification, operator evidence, and operator acceptance before install-cache
materialization. Current source state intentionally has none accepted.

The install-cache test path is limited to no-op preflight, idempotency denial
receipts, rollback/uninstall no-op planning, and dynamic activation/connector
start denial. It does not materialize cache, mutate cache, install a plugin, or
execute rollback/uninstall.

The sandbox enforcement design records the required rules for network-none,
filesystem read-only, local connector scoping, manual activation, approval and
ledger policy, credential boundary, transport boundary, and persistence
boundary. Enforcement remains designed-not-enforced until operator approval,
trust acceptance, and clean source state exist.

The sandbox enforcement kernel is dry-run only. It tests that manifest v1
enforces network none, MCP filesystem read-only scope, local connector scope,
manual-only activation, approval plus ledger policy, credential boundary,
transport boundary, and runtime persistence boundary. It can block invalid
manifests, but it does not grant permissions, start connectors or MCP servers,
allow network access, read credentials, mutate runtime state, register tools,
invoke tools, or enable live execution.

## Closed Boundary

This source-cache performs no manifest rewrite, schema write, plugin install,
cache mutation, signature acceptance, trust-root acceptance, permission grant,
dynamic activation, connector start, ToolRegistry registration, tool invocation,
ledger write, approval request, receipt persistence, runtime event-log write,
SQLite write, credential read, external network, transport mutation, canary,
live, or Public GA.
