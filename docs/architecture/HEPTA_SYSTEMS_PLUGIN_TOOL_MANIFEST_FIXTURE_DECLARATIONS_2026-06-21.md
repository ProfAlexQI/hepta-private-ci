# Hepta Systems Plugin Tool Manifest Fixture Declarations - 2026-06-21

This note records local-only manifest fixture readback for plugin tool metadata,
using `plugins/hepta-system/.codex-plugin/plugin.json` without plugin
installation or runtime execution.

## Current Checkout Reality

The manifest fixture readback supplies complete tool metadata for the two
planned `hepta-system` plugin-tool candidates without registration:

- `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp`
- `preview:connector:hepta-system@hepta-local:hepta_system_local_app`

Current report facts:

- `hepta_system_manifest_present=true`
- `declaration_source=hepta_system_manifest_fixture_readback`
- `current_fixture_declared_candidate_count=2`
- `current_fixture_schema_complete_count=2`
- `current_fixture_policy_complete_count=2`
- `current_fixture_registration_preconditions_satisfied=true`
- `manifest_fixture_declarations_ready=true`

## Boundary

This is not a plugin installation and does not rewrite `plugin.json`. It is a
readback source for local preflight only, so downstream gates can prove the
forward path while keeping execution disabled.

## Guardrails

- Manifest file rewrite disabled.
- Manifest schema write disabled.
- Loader invocation disabled.
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

- Report: `scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-gate.sh`

## Next Move

The manifest fixture declarations are now consumed by manifest schema preflight
and the invocation router preflight binding. The next move is to restore the
tool registry invocation source-of-truth path without execution, while keeping
registration, invocation, ledgers, approvals, package/release/live actions, and
Public GA blocked until explicit cutover.

## 2026-06-25 Fixture Readback Update

`declaration_source` is now `hepta_system_manifest_fixture_readback`. The
manifest fixture readback supplies complete schema and policy metadata for the
two planned candidates without plugin installation, loader invocation,
ToolRegistry registration, tool invocation, ledger writes, approval requests,
MCP startup, app connector startup, external delivery, or release promotion.
