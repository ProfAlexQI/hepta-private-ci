# Hepta Systems Plugin Tool Contribution Inventory Preview - 2026-06-21

This note records the local-only bridge from plugin contribution points to
ToolRegistry candidate metadata. It is evidence for reviewer/runtime planning
only. It does not register tools, invoke tools, write a ledger, request approval,
start MCP servers, start app connectors, mutate plugin cache, create local
storage, or enable live execution.

## Current Checkout Reality

The local `hepta-system` plugin fixture now exists and is read only for
candidate planning. The two candidates are read back from the manifest fixture
without registration.

Current report facts:

- `hepta_system_manifest_present=true`
- `candidate_source=manifest_fixture_readback_without_registration`
- `current_fixture_candidate_count=2`
- `planned_candidate_count=2`
- live mutation disabled

## Preview Candidates

The preview maps the tool-relevant loader-bound contribution points into
ToolRegistry-shaped metadata:

| Contribution | Loader output | Candidate source | Side effect | Approval | Auth | Ledger | Guard route |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `mcp_server` | `mcp_servers` | `mcp` | `local_mutation` | `on_use` | false | required | `require_approval_ledger` |
| `app_connector` | `apps` | `connector` | `external_mutation` | `install` | true | required | `require_approval_ledger` |

The non-tool loader-bound entries remain intentionally skipped for ToolRegistry
preview:

- `skill` stays a skill contribution, not a tool registration.
- `hook` stays a hook contribution, not a tool registration.

## Guardrails

The report and gate keep these boundaries true:

- ToolRegistry registration disabled.
- Tool invocation disabled.
- Tool ledger writes disabled.
- Approval requests disabled.
- MCP server and app connector startup disabled.
- Plugin cache, package lock, manifest rewrite, remote sync, and local storage
  mutation disabled.
- Workflow event-log/SQLite mutation disabled.
- Telegram/provider/model/gateway/Native POST external action disabled.
- Public GA and release/live promotion blocked.

## Files

- Preview contract: `codex-rs/tools/src/plugin_contribution_inventory_preview.rs`
- ToolRegistry vocabulary: `codex-rs/tools/src/tool_registry_inventory.rs`
- Report: `scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-contribution-inventory-preview-gate.sh`

## Next Move

The plugin ToolRegistry source-of-truth dry-run, manifest parser fields,
manifest schema preflight adapter, and invocation router preflight binding are
now restored. The next move is to keep tightening the read-only E2E chain,
still without registration, invocation, approval requests, or ledger writes.

## 2026-06-25 Fixture Readback Update

The candidate source is now `manifest_fixture_readback_without_registration`.
The manifest fixture readback confirms the two planned candidates while keeping
inventory registration, tool invocation, ledger writes, approval requests, MCP
startup, app connector startup, and external delivery disabled.
