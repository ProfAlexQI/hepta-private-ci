# Hepta Systems Plugin Tool Manifest Preflight Adapter Bridge - 2026-06-21

This note records the local-only bridge from parsed plugin manifest tool
metadata into the ToolRegistry preflight adapter. The bridge accepts
parser-shaped input and keeps registration, invocation, ledgers, approvals,
manifest rewrites, schema writes, plugin cache mutation, and live mutation
disabled.

## Current Checkout Reality

The current checkout still does not include the historical
`plugins/hepta-system/.codex-plugin/plugin.json` fixture. The bridge is ready,
but the current fixture contributes zero parsed tool declarations.

Current report facts:

- `source_registry_dry_run_ready=true`
- `source_manifest_parser_fields_ready=true`
- `parser_input_field_count=7`
- `planned_candidate_count=2`
- `parsed_manifest_declared_candidate_count=0`
- `preflight_adapter_bridge_ready=true`
- `registration_cutover_allowed=false`

## Parser-Shaped Input

The bridge models the parser output as:

- `contribution_candidate_ids`
- `tool_schemas`
- `permissions`
- `activation_events`
- `tool_policies`
- `schema_complete_candidate_ids`
- `policy_complete_candidate_ids`

Unknown parsed candidate ids are treated as unbound declarations by the Rust
adapter and fail closed. Missing schema or policy declarations block planned
candidates before any registration or invocation path.

## Guardrails

- ToolRegistry source-of-truth enablement disabled.
- Registration cutover execution disabled.
- Tool registration disabled.
- Tool invocation disabled.
- Ledger writes disabled.
- Approval requests disabled.
- MCP server and app connector startup disabled.
- Manifest rewrites and manifest schema writes disabled.
- Plugin cache, package lock, remote sync, workflow event log, local storage,
  SQLite, Telegram/provider/model/gateway/Native POST, package, release, and
  Public GA actions disabled.

## Files

- Rust adapter: `codex-rs/tools/src/plugin_tool_manifest_schema_cutover_preflight.rs`
- Report: `scripts/hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-manifest-preflight-adapter-bridge-gate.sh`

## Next Move

Restore the manifest schema cutover preflight report/gate without registration.
It should consume the bridge and prove current missing manifest metadata blocks
both planned candidates while all live mutation remains disabled.

## 2026-06-25 Fixture Readback Update

The bridge now receives two parser-shaped manifest fixture readback candidates
instead of an empty fixture surface. It remains an adapter-only step: no
registration cutover, ToolRegistry registration, tool invocation, ledger write,
approval request, MCP startup, or app connector startup is enabled.
