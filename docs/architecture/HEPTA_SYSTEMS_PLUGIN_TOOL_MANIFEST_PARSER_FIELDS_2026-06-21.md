# Hepta Systems Plugin Tool Manifest Parser Fields - 2026-06-21

This note records the local-only manifest parser surface for plugin tool
metadata fields. It restores parser support for `toolSchemas`, `permissions`,
`activationEvents`, and `toolPolicies` without registering tools, invoking
tools, writing ledgers, requesting approvals, starting MCP servers, starting app
connectors, mutating plugin cache, rewriting manifests, or enabling live
execution.

## Current Checkout Reality

The historical patch assumed a local `hepta-system` plugin fixture. The current
checkout still does not have that fixture, so this recovery only claims parser
support and reports zero current fixture declarations.

Current report facts:

- `hepta_system_manifest_present=false`
- `parser_supported_field_count=4`
- `current_fixture_declared_candidate_count=0`
- `current_fixture_schema_complete_count=0`
- `current_fixture_policy_complete_count=0`
- `parser_fields_ready=true`
- `parsed_declarations_feed_preflight=true`
- live mutation disabled

## Parser Fields

The parser surface models four optional manifest fields:

| Field | Meaning |
| --- | --- |
| `toolSchemas` | candidate input/output schema presence |
| `permissions` | candidate permission declaration presence |
| `activationEvents` | candidate activation declaration presence |
| `toolPolicies` | candidate approval, ledger, and timeout declaration presence |

The parser keeps only declaration ids and completeness booleans. It does not
execute schema payloads, read credentials, start tools, or mutate plugin state.

## Guardrails

The report and gate keep these boundaries true:

- Manifest rewrite disabled.
- Manifest schema write disabled.
- ToolRegistry source-of-truth enablement disabled.
- Registration cutover execution disabled.
- Tool registration disabled.
- Tool invocation disabled.
- Ledger writes disabled.
- Approval requests disabled.
- MCP server and app connector startup disabled.
- Plugin cache, package lock, remote sync, workflow event log, local storage,
  SQLite, Telegram/provider/model/gateway/Native POST, package, release, and
  Public GA actions disabled.

## Files

- Parser source: `codex-rs/core-plugins/src/manifest.rs`
- Report: `scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-manifest-parser-fields-gate.sh`

## Next Move

Manifest schema preflight and invocation router preflight binding now consume
parser-shaped declarations and block the current two planned candidates on
missing manifest metadata. The next move is to restore or replace local manifest
fixture declarations while keeping registration, invocation, ledgers, approvals,
package/release/live actions, and Public GA blocked until explicit cutover
approval.

## 2026-06-25 Fixture Readback Update

The parser-fields gate now reads the real
`plugins/hepta-system/.codex-plugin/plugin.json` fixture. The fixture supplies
complete `toolSchemas`, `permissions`, `activationEvents`, and `toolPolicies`
for the two planned candidates, but the parser surface still performs no
registration, invocation, ledger write, approval request, or external action.
