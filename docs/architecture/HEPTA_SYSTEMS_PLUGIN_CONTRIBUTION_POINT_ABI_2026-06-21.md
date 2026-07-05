# Hepta Systems Plugin Contribution Point ABI - 2026-06-21

## Purpose

This note records the recovered local Contribution Point ABI for the Hepta
plugin system. It is a current-checkout reconstruction from the `hepta` agent
session history, not a blind replay of the old patch.

## Current Checkout Boundary

The historical patch expected a local `hepta-system` plugin fixture and
marketplace metadata. The current checkout does not contain those files, so this
recovery slice restores the typed ABI contract, public crate export, and the
read-only loader binding contract. It still does not claim fixture declarations.

The ABI has live mutation disabled. It does not install plugins, mutate plugin
caches, write package locks, start remote sync, register tools, invoke tools,
write ledgers, request approvals, read credentials, call providers/models, send
Telegram/channel messages, mutate gateway/auth state, mutate Native POST, write
workflow event logs, or promote Public GA.

## Contribution Points

The ABI covers eight contribution kinds:

- skill
- MCP server
- tool
- app connector
- hook
- permission
- activation event
- local storage

The current manifest/loader path support is bound to `skills`, `mcpServers`,
`apps`, and `hooks` through the loader binding contract. Tool, permission,
activation event, and local storage remain explicit future bridges. ToolRegistry
is required for contributed tools but is not enabled here.

## Local Contract

- Rust contract: `codex-rs/core-plugins/src/contribution_point_abi.rs`
- Loader binding contract: `codex-rs/core-plugins/src/contribution_point_loader_binding.rs`
- Public export: `codex-rs/core-plugins/src/lib.rs`
- Report: `scripts/hepta-systems-plugin-contribution-point-abi-report.sh`
- Gate: `scripts/hepta-systems-plugin-contribution-point-abi-gate.sh`

The gate validates the report and runs the focused `codex-core-plugins`
`contribution_point_abi` tests.

## Next Move

The plugin ToolRegistry source-of-truth dry-run, manifest parser fields,
manifest schema preflight adapter, and invocation router preflight binding are
now restored. The next move is to restore or replace local manifest fixture
declarations, still without ToolRegistry registration, invocation, approval
requests, ledger writes, or live execution.
