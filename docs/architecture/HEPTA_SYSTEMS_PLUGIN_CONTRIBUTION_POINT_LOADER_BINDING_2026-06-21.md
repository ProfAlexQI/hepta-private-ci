# Hepta Systems Plugin Contribution Point Loader Binding - 2026-06-21

This note records the local-only binding between plugin manifest loader outputs
and the Contribution Point ABI. It does not authorize plugin installation,
plugin cache mutation, package-lock persistence, manifest rewrites, remote sync,
ToolRegistry registration, tool execution, ledger writes, approval requests,
credential reads, provider/model calls, local storage creation, Telegram
delivery, gateway/auth mutation, Native POST mutation, release, or public GA
promotion.

## Current Checkout Reality

The historical patch assumed a local `plugins/hepta-system/.codex-plugin/plugin.json`
fixture. The current checkout does not have that fixture, so this recovery does
not claim fixture declarations. Instead, it restores the read-only contract that
maps the current manifest/loader fields to typed ABI entries.

Current report facts:

- `hepta_system_manifest_present=false`
- `declared_manifest_field_count=0`
- `fixture_declared_bound_entry_count=0`
- `loader_contract_ready=true`
- `binding_ready=true`
- `current_fixture_binding_ready=false`
- live mutation disabled

## Bound Loader Surface

Four contribution kinds are bound to manifest fields and loader outputs:

| Contribution kind | Manifest field | Loader output |
| --- | --- | --- |
| skill | `skills` | `skill_roots` |
| MCP server | `mcpServers` | `mcp_servers` |
| app connector | `apps` | `apps` |
| hook | `hooks` | `hook_sources` |

These are contract bindings only. The gate does not invoke the loader, install a
plugin, mutate caches, rewrite manifests, register tools, write a ledger, or
create local storage.

## Future Bridges

The remaining contribution kinds stay blocked as future bridges:

- tool
- permission
- activation event
- local storage

They require explicit manifest fields, policy metadata, ToolRegistry preview
mapping, or plugin-data-root scoping before runtime binding. None of these
bridges registers tools, creates local storage, writes ledgers, or enables
runtime execution.

## Contract

- 8 ABI entries are known.
- 4 entries are loader-contract bound.
- 3 current fixture declarations are bound from the local manifest fixture.
- 4 future bridges are blocked.
- 0 entries are unbound without a future bridge reason.
- ToolRegistry registration is disabled.
- Runtime execution is disabled.
- Local storage creation is disabled.
- All live paths remain blocked.

## Files

- Rust contract: `codex-rs/core-plugins/src/contribution_point_loader_binding.rs`
- ABI contract: `codex-rs/core-plugins/src/contribution_point_abi.rs`
- Report: `scripts/hepta-systems-plugin-contribution-point-loader-binding-report.sh`
- Gate: `scripts/hepta-systems-plugin-contribution-point-loader-binding-gate.sh`

## Next Move

The plugin ToolRegistry source-of-truth dry-run, manifest parser fields,
manifest schema preflight adapter, and invocation router preflight binding are
now restored. The next move is to restore or replace local manifest fixture
declarations, still without registration, invocation, approval requests, or
ledger writes.

## 2026-06-25 Fixture Readback Update

`plugins/hepta-system/.codex-plugin/plugin.json` now exists as a local read-only
fixture. The loader-binding gate treats `skills`, `mcpServers`, and `apps` as
manifest fixture readback fields, while hooks remain a loader contract field and
all runtime execution, ToolRegistry registration, ledger writes, approval
requests, and local storage creation stay disabled.
