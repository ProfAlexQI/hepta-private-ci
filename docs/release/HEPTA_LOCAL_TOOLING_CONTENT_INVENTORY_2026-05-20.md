# Hepta Local Tooling/Content Inventory

Date: 2026-05-20
Scope: old standalone Hepta local tooling/content ops modules versus current `hepta-codex`
Status: local planning inventory landed; process/filesystem/network/tool execution disabled

## Summary

The channel adapter disabled-status slice left local tooling/content surfaces as
the next safe migration target. This slice exposes those old modules only as
planning entries:

- `/api/hepta-local-tooling-content-inventory`
- source-command equivalent: `/hepta-local-tooling-content-inventory --json`
- validation script: `scripts/hepta-codex-local-tooling-content-inventory.sh`

The route does not spawn processes, read or write filesystem content, fetch
network URLs, invoke tools, invoke providers or models, read credentials, read or
send channel messages, mutate Gateway/native POST state, or perform external
sends.

## Inventory Counts

- old local tooling/content ops files covered: `11`
- local tooling/content surfaces inventoried: `11`
- planner-ready surfaces: `11`
- live process execution enabled: `0`
- filesystem touch enabled: `0`
- network read enabled: `0`
- tool invocation enabled: `0`
- current `hepta-codex` scripts: `11`
- current native gateway source commands: `58`
- Control UI route parity after memory/capability absorption inventory continuation: `58/58`, missing `0`

## Files Covered

- `canvas_ops.rs`
- `device_control_ops.rs`
- `diffs_ops.rs`
- `document_extract_ops.rs`
- `filesystem_ops.rs`
- `local_content_ops.rs`
- `process_execution_ops.rs`
- `search_tools_ops.rs`
- `tools_invoke_ops.rs`
- `web_readability_ops.rs`
- `wiki_tools_ops.rs`

## Boundary

This is a sequencing and safety inventory only. It intentionally keeps old CLI
invocation compatibility unclaimed and keeps all process execution, filesystem
read/write, network read, and tool invocation gates disabled.

## Remaining Blockers

- process execution is not operator-approved
- filesystem read is not operator-approved
- filesystem write is not operator-approved
- network fetch is not operator-approved
- tool invocation is not operator-approved
- old local tooling CLI invocation compatibility is not claimed

## Safe Next Slice

Port remaining external release and hardening scripts as local-only status gates before any live
process, filesystem, network, or tool-invocation smoke.
