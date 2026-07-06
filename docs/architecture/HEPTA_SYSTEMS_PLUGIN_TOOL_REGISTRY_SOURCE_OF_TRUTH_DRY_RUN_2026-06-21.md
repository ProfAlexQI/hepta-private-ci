# Hepta Systems Plugin ToolRegistry Source-of-Truth Dry-Run - 2026-06-21

This note records the local-only dry-run that binds plugin tool contribution
preview candidates into one ToolRegistry planning surface. It is evidence for
source-of-truth planning only. It does not register tools, invoke tools, write a
ledger, request approval, start MCP servers, start app connectors, mutate plugin
cache, rewrite manifests, create local storage, or enable live execution.

## Current Checkout Reality

The historical patch assumed a local `hepta-system` plugin fixture. The current
checkout still does not have that fixture, so this recovery keeps the source as
contract-planned preview candidates. The source-of-truth dry-run binds those
candidates to ToolRegistry inventory entries and validates duplicate ids,
unbound candidates, schema metadata, risk metadata, ledger requirements, and
approval guard routes.

Current report facts:

- `hepta_system_manifest_present=false`
- `preview_candidate_count=2`
- `planned_registry_entry_count=2`
- `duplicate_candidate_ids=[]`
- `duplicate_registry_ids=[]`
- `unbound_candidate_ids=[]`
- `all_preview_candidates_bound_to_registry=true`
- `registry_source_of_truth_dry_run_ready=true`
- live mutation disabled

## Planned Source-of-Truth Entries

| Contribution | Candidate id | Registry source | Guard route |
| --- | --- | --- | --- |
| `mcp_server` | `preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp` | `mcp` | `require_approval_ledger` |
| `app_connector` | `preview:connector:hepta-system@hepta-local:hepta_system_local_app` | `connector` | `require_approval_ledger` |

Both entries have input/output schema presence, side-effect metadata, approval
metadata, ledger requirement, and non-executing guard decisions. This surface is
not an enablement switch.

## Guardrails

The report and gate keep these boundaries true:

- Source-of-truth enablement disabled.
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

- Dry-run contract: `codex-rs/tools/src/plugin_tool_registry_source_of_truth_dry_run.rs`
- Preview contract: `codex-rs/tools/src/plugin_contribution_inventory_preview.rs`
- ToolRegistry vocabulary: `codex-rs/tools/src/tool_registry_inventory.rs`
- Report: `scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh`
- Gate: `scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-gate.sh`

## Next Move

Plugin tool manifest parser fields, manifest schema preflight, and invocation
router preflight binding are now restored. The next move is to restore or
replace local manifest fixture declarations without registration, invocation,
approval requests, ledger writes, or live mutation.

## 2026-06-25 Fixture Readback Update

The source-of-truth dry-run now inherits `hepta_system_manifest_present=true`
from the manifest fixture readback. It still binds only the two planned dry-run
candidates and keeps source-of-truth enablement, ToolRegistry registration, tool
invocation, ledger writes, approval requests, MCP startup, and app connector
startup disabled.
