# Hepta Systems Plugin Lifecycle State Machine - 2026-06-27

This note records the compact Plugin Lifecycle State Machine restored for the
Hepta systems lane. It replaces the stale memory-only lifecycle checkpoint with
a current checkout source of truth and does not install plugins, mutate plugin
cache, register tools, invoke tools, write ledgers, request approvals, or open
live mutation disabled paths.

## Sources

The lifecycle state machine consumes current local sources:

- `codex-rs/core-plugins/src/contribution_point_abi.rs`
- `codex-rs/core-plugins/src/contribution_point_loader_binding.rs`
- `codex-rs/core-plugins/src/lifecycle_phase_summary.rs`
- `codex-rs/core-plugins/src/lifecycle_state_machine.rs`
- `plugins/hepta-system/.codex-plugin/plugin.json`
- `scripts/hepta-systems-plugin-tool-contribution-inventory-preview-report.sh`

The fixture shape is part of the lifecycle read-model. The manifest points to
local declaration files for skills, MCP servers, and app connectors, so the
state machine resolves those paths before counting declarations:

- 1 skill declaration under `./skills`
- 1 MCP server declaration in `./.mcp.json`
- 1 app connector declaration in `./.app.json`
- 2 tool schema declarations
- 2 permission declarations
- 2 activation event declarations
- 2 tool policy declarations

## Phases

The lifecycle read-model has six ready phases:

- `manifest_fixture_discovered`
- `contribution_point_abi_audited`
- `loader_binding_audited`
- `fixture_policy_metadata_audited`
- `tool_preview_contract_audited`
- `live_mutation_blocked`

The first five phases establish manifest, permission, activation, tool-policy,
loader, and tool-preview source-of-truth readiness. The final phase is an
explicit denial phase: registration, invocation, ledger writes, approvals,
plugin cache mutation, local storage creation, provider calls, gateway/auth
mutation, Native POST mutation, package writes, and Public GA promotion remain
disabled.

## Boundary

This lifecycle surface is report-only. It does not:

- install plugins
- mutate plugin cache
- rewrite manifests
- invoke the loader at runtime
- register ToolRegistry entries
- invoke tools
- write tool ledgers
- request or resolve approvals
- create local plugin storage
- mutate workflow event logs
- read credentials
- invoke providers or models
- mutate gateway/auth or Native POST routing
- send channels
- package, release, or promote Public GA

## Next Move

The next reversible slice is Phase 2:
`phase2_promote_tool_registry_to_read_only_dispatch_preflight_without_invocation`.
That should use this lifecycle read-model as the plugin source of truth, then
route plugin tool candidates through ToolRegistry lookup, ledger preview,
approval preflight, and receipt projection without invoking external tools.
